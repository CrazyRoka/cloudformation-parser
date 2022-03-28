use crate::value::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Output {
    description: Option<String>,
    value: Value,
    export: Option<Export>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct Export {
    name: Value,
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::{Export, Output};

    #[test]
    fn test_deserialize_output() {
        let test_cases = [
            (
                r#"
Description: The URL of the website
Value:
  Fn::Join: ['', ['http://', Fn::GetAtt: [ElasticLoadBalancer, DNSName]]]
            "#,
                Output {
                    description: Some("The URL of the website".to_string()),
                    value: Value::Join {
                        join: (
                            "".to_string(),
                            vec![
                                Value::String("http://".to_string()),
                                Value::GetAtt {
                                    get_att: vec![
                                        "ElasticLoadBalancer".to_string(),
                                        "DNSName".to_string(),
                                    ],
                                },
                            ],
                        ),
                    },
                    export: None,
                },
            ),
            (
                r#"
Description: The ID of the VPC
Value:
    Ref: MyVPC
Export:
    Name:
        Fn::Sub: "${AWS::StackName}-VPCID"
            "#,
                Output {
                    description: Some("The ID of the VPC".to_string()),
                    value: Value::Ref {
                        r#ref: "MyVPC".to_string(),
                    },
                    export: Some(Export {
                        name: Value::Sub {
                            sub: "${AWS::StackName}-VPCID".to_string(),
                        },
                    }),
                },
            ),
        ];

        for (yaml, expected) in test_cases {
            let actual = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
