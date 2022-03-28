use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn test_deserialize_tags() {
        let yaml = r#"
- Key: "keyname1"
  Value: "value1"
- Key: "keyname2"
  Value: "value2"
        "#;
        let expected = vec![
            Tag {
                key: "keyname1".to_string(),
                value: "value1".to_string(),
            },
            Tag {
                key: "keyname2".to_string(),
                value: "value2".to_string(),
            },
        ];

        let actual: Vec<Tag> = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }
}
