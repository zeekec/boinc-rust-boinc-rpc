use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "exchange_versions")]
pub struct ExchangeVersions {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub release: Option<i64>,
    pub name: Option<String>,
}

impl ExchangeVersions {
    #[must_use]
    pub const fn new(
        major: Option<i64>,
        minor: Option<i64>,
        release: Option<i64>,
        name: Option<String>,
    ) -> Self {
        Self {
            major,
            minor,
            release,
            name,
        }
    }
}

impl Default for ExchangeVersions {
    fn default() -> Self {
        Self {
            major: Some(8),
            minor: Some(1),
            release: Some(0),
            name: None,
        }
    }
}

impl Display for ExchangeVersions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use quick_xml::se::Serializer;
        use serde::Serialize;

        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 4);

        self.serialize(ser).unwrap();

        write!(f, "{buffer}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate testing_logger;

    #[test]
    fn test_format() {
        testing_logger::setup();

        let version = ExchangeVersions {
            name: Some("BOINC".to_string()),
            ..Default::default()
        };
        let xml = format!("{version}");
        assert_eq!(
            xml,
            "<exchange_versions>\n    <major>8</major>\n    <minor>1</minor>\n    <release>0</release>\n    <name>BOINC</name>\n</exchange_versions>"
        );
    }

    #[test]
    fn test_unparse() {
        let expected = r#"<exchange_versions><major>7</major><minor>16</minor><release>16</release><name>BOINC</name></exchange_versions>"#;

        let version = ExchangeVersions::new(Some(7), Some(16), Some(16), Some("BOINC".to_string()));

        let result = quick_xml::se::to_string(&version).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse() {
        testing_logger::setup();

        let expected = ExchangeVersions {
            major: Some(7),
            minor: Some(16),
            release: Some(16),
            name: Some("BOINC".to_string()),
        };

        let xml = r#"<?xml version="1.0"?>
        <exchange_versions>
            <major>7</major>
            <minor>16</minor>
            <release>16</release>
            <name>BOINC</name>
        </exchange_versions>"#;

        let result: ExchangeVersions = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_no_name() {
        testing_logger::setup();

        let expected = ExchangeVersions::new(Some(7), Some(16), Some(16), None);

        let xml = r#"<?xml version="1.0"?>
        <exchange_versions>
            <major>7</major>
            <minor>16</minor>
            <release>16</release>
        </exchange_versions>"#;

        let result: ExchangeVersions = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(expected, result);
    }
}
