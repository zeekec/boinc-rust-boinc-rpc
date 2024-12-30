use crate::messages::helpers::{add_element, parse_node};
use log::{trace, warn};
use serde::{Deserialize, Serialize};
use serde_json;
use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use treexml;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let json_string = serde_json::to_string(self).unwrap();

        write!(f, "{json_string}")
    }
}

impl From<&treexml::Element> for ExchangeVersions {
    fn from(node: &treexml::Element) -> Self {
        if node.name != "exchange_versions" {
            warn!("Root node is not 'exchange_versions':\n{node}");
        } else {
            trace!("Parsing ExchangeVersions:\n{node}");
        }
        let mut e = Self::default();

        e.major = parse_node("major", node);
        e.minor = parse_node("minor", node);
        e.release = parse_node("release", node);
        e.name = parse_node("name", node);

        e
    }
}

impl From<&ExchangeVersions> for treexml::Element {
    fn from(e: &ExchangeVersions) -> Self {
        trace!("Creating XML from ExchangeVersions:\n{e}");

        let mut content_node = Self::new("exchange_versions");

        add_element(&mut content_node, "major", &e.major);
        add_element(&mut content_node, "minor", &e.minor);
        add_element(&mut content_node, "release", &e.release);
        add_element(&mut content_node, "name", &e.name);

        content_node
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    extern crate testing_logger;

    #[test]
    fn test_serialization() {
        testing_logger::setup();

        let version = ExchangeVersions {
            name: Some("BOINC".to_string()),
            ..Default::default()
        };
        let xml = serde_json::to_string(&version).unwrap();
        assert_eq!(
            xml,
            "{\"major\":8,\"minor\":1,\"release\":0,\"name\":\"BOINC\"}"
        );
    }

    #[test]
    fn test_parse() {
        testing_logger::setup();

        let xml = r#"<?xml version="1.0"?>
        <exchange_versions>
            <major>7</major>
            <minor>16</minor>
            <release>16</release>
            <name>BOINC</name>
        </exchange_versions>"#;
        let node = treexml::Document::parse(xml.as_bytes())
            .unwrap()
            .root
            .unwrap();

        let version: ExchangeVersions = ExchangeVersions::from(&node);

        assert_eq!(version.major, Some(7));
        assert_eq!(version.minor, Some(16));
        assert_eq!(version.release, Some(16));
        assert_eq!(version.name, Some("BOINC".to_string()));
    }

    #[test]
    fn test_unparse() {
        let xml = r#"<?xml version="1.0"?>
        <exchange_versions>
            <major>7</major>
            <minor>16</minor>
            <release>16</release>
            <name>BOINC</name>
        </exchange_versions>"#;
        let node = treexml::Document::parse(xml.as_bytes())
            .unwrap()
            .root
            .unwrap();

        let version = ExchangeVersions::new(Some(7), Some(16), Some(16), Some("BOINC".to_string()));

        let xml = treexml::Element::from(&version);

        assert_eq!(node, xml);
    }

    #[test]
    fn test_parse_no_name() {
        testing_logger::setup();

        let xml = r#"<?xml version="1.0"?>
        <exchange_versions>
            <major>7</major>
            <minor>16</minor>
            <release>16</release>
        </exchange_versions>"#;
        let node = treexml::Document::parse(xml.as_bytes())
            .unwrap()
            .root
            .unwrap();
        let version: ExchangeVersions = ExchangeVersions::from(&node);
        print!("\n\n{node}\n\n");
        assert_eq!(version.major, Some(7));
        assert_eq!(version.minor, Some(16));
        assert_eq!(version.release, Some(16));
        assert_eq!(version.name, None);

        let xml = treexml::Element::from(&version);
        assert_eq!(node, xml);
    }
}
