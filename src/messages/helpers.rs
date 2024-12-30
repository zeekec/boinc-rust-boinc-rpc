use log::warn;

use treexml::Element;

pub fn parse_node<T: std::str::FromStr>(name: &str, node: &Element) -> Option<T> {
    let children: Vec<&Element> = node.filter_children(|tag| tag.name == name).collect();
    if children.len() > 1 {
        warn!(
            "Expected 1 child with name '{name}', found {0}:\n{node}",
            children.len()
        );
    }
    children
        .last()
        .and_then(|tag| tag.text.clone()?.parse::<T>().ok())
}

pub fn add_element<T: std::fmt::Display>(parent: &mut Element, name: &str, value: &Option<T>) {
    if let Some(v) = value {
        let mut node = Element::new(name);
        node.text = Some(format!("{v}"));
        parent.children.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate testing_logger;
    use log::Level;

    #[test]
    fn test_parse_node() {
        testing_logger::setup();
        let mut node = Element::new("root");
        let mut child = Element::new("child");
        child.text = Some("42".to_string());
        node.children.push(child);

        let result: Option<i32> = parse_node("child", &node);
        assert_eq!(result, Some(42));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
    }

    #[test]
    fn test_parse_node_extra_children() {
        testing_logger::setup();
        let mut node = Element::new("root");
        let mut child = Element::new("child");
        child.text = Some("42".to_string());
        node.children.push(child);
        let mut child = Element::new("child");
        child.text = Some("43".to_string());
        node.children.push(child);

        let result: Option<i32> = parse_node("child", &node);
        assert_eq!(result, Some(43));

        testing_logger::validate(|captured_logs| {
            let warnings = captured_logs
                .iter()
                .filter(|c| c.level == Level::Warn)
                .collect::<Vec<&testing_logger::CapturedLog>>();
            assert_eq!(warnings.len(), 1);
            let expected_warning: &'static str = r#"Expected 1 child with name 'child', found 2:
<root>
  <child>42</child>
  <child>43</child>
</root>"#;
            assert_eq!(warnings[0].body, expected_warning);
        });
    }
}
