use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Mapping {
    #[serde(flatten)]
    entries: HashMap<String, MappingEntry>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum MappingEntry {
    String(String),
    List(Vec<String>),
    Mapping(Mapping),
}

#[cfg(test)]
mod test {
    use super::Mapping;
    use crate::mapping::MappingEntry;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_mappings() {
        let yaml = r#"
Name: Test
NameList:
  - First
  - Second
NameMap:
  first: First
  second:
    - A
    - B
    - C
  third:
    A: B
    B: C
    C: A
        "#;

        let expected = Mapping {
            entries: HashMap::from([
                ("Name".to_string(), MappingEntry::String("Test".to_string())),
                (
                    "NameList".to_string(),
                    MappingEntry::List(vec!["First".to_string(), "Second".to_string()]),
                ),
                (
                    "NameMap".to_string(),
                    MappingEntry::Mapping(Mapping {
                        entries: HashMap::from([
                            (
                                "first".to_string(),
                                MappingEntry::String("First".to_string()),
                            ),
                            (
                                "second".to_string(),
                                MappingEntry::List(vec![
                                    "A".to_string(),
                                    "B".to_string(),
                                    "C".to_string(),
                                ]),
                            ),
                            (
                                "third".to_string(),
                                MappingEntry::Mapping(Mapping {
                                    entries: HashMap::from([
                                        ("A".to_string(), MappingEntry::String("B".to_string())),
                                        ("B".to_string(), MappingEntry::String("C".to_string())),
                                        ("C".to_string(), MappingEntry::String("A".to_string())),
                                    ]),
                                }),
                            ),
                        ]),
                    }),
                ),
            ]),
        };
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(expected, mapping);
    }
}
