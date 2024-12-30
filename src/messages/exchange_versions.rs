use log::{debug, error};
use serde::{Deserialize, Serialize};
use treexml;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExchangeVersions {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub release: Option<i64>,
    pub name: Option<String>,
}

impl From<&treexml::Element> for ExchangeVersions {
    fn from(node: &treexml::Element) -> Self {
        if node.name != "exchange_versions" {
            error!("Root node is not 'exchange_versions':\n{node}");
        } else {
            debug!("Parsing ExchangeVersions:\n{node}");
        }
        let mut e = Self::default();
        e.major = node
            .find_child(|tag| tag.name == "major")
            .and_then(|tag| tag.text.clone()?.parse().ok());
        e.minor = node
            .find_child(|tag| tag.name == "minor")
            .and_then(|tag| tag.text.clone()?.parse().ok());
        e.release = node
            .find_child(|tag| tag.name == "release")
            .and_then(|tag| tag.text.clone()?.parse().ok());
        e.name = node
            .find_child(|tag| tag.name == "name")
            .and_then(|tag| tag.text.clone()?.parse().ok());
        e
    }
}

impl From<&ExchangeVersions> for treexml::Element {
    fn from(e: &ExchangeVersions) -> Self {
        let mut content_node = Self::new("exchange_versions");

        if e.major.is_some() {
            let mut node = Self::new("major");
            node.text = e.major.map(|v| format!("{v}"));
            content_node.children.push(node);
        }

        if e.minor.is_some() {
            let mut node = Self::new("minor");
            node.text = e.minor.map(|v| format!("{v}"));
            content_node.children.push(node);
        }

        if e.release.is_some() {
            let mut node = Self::new("release");
            node.text = e.release.map(|v| format!("{v}"));
            content_node.children.push(node);
        }

        if e.name.is_some() {
            let mut node = Self::new("name");
            node.text = e.name.clone();
            content_node.children.push(node);
        }

        content_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colog;

    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn setup() {
        INIT.call_once(|| {
            let mut clog = colog::default_builder();
            clog.filter(None, log::LevelFilter::Info);
            clog.init();
        });
    }

    #[test]
    fn test_serialization_version_exchange() {
        setup();

        let version = ExchangeVersions {
            major: Some(7),
            minor: Some(16),
            release: Some(16),
            name: Some("BOINC".to_string()),
        };
        let xml = serde_yml::to_string(&version).unwrap();
        assert_eq!(xml, "major: 7\nminor: 16\nrelease: 16\nname: BOINC\n");
    }

    #[test]
    fn test_parse_version_exchange() {
        setup();

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
        print!("\n\n{node}\n\n");
        assert_eq!(version.major, Some(7));
        assert_eq!(version.minor, Some(16));
        assert_eq!(version.release, Some(16));
        assert_eq!(version.name, Some("BOINC".to_string()));

        let xml = treexml::Element::from(&version);
        assert_eq!(node, xml);
    }

    #[test]
    fn test_parse_version_exchang_no_name() {
        setup();

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
