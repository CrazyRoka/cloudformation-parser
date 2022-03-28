use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(i64),
    #[serde(rename_all = "PascalCase")]
    Ref {
        r#ref: String,
    },
    GetAtt {
        #[serde(rename = "Fn::GetAtt")]
        get_att: Vec<String>,
    },
    Join {
        #[serde(rename = "Fn::Join")]
        join: (String, Vec<Value>),
    },
    Sub {
        #[serde(rename = "Fn::Sub")]
        sub: String,
    },
}

#[cfg(test)]
mod test {
    use super::Value;

    #[test]
    fn test_deserialize_value_string() {
        let yaml = "some value";
        let expected = Value::String(yaml.to_string());

        let actual: Value = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_value_number() {
        let test_cases = [("-5", -5), ("0", 0), ("23123", 23123)];
        for test_case in test_cases {
            let yaml = test_case.0;
            let expected = Value::Number(test_case.1);

            let actual = serde_yaml::from_str(yaml).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_deserialize_value_ref() {
        let yaml = "Ref: 'SSHLocation'";
        let expected = Value::Ref {
            r#ref: "SSHLocation".to_string(),
        };

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_value_get_att() {
        let yaml = "Fn::GetAtt: [ElasticLoadBalancer, SourceSecurityGroup.OwnerAlias]";
        let expected = Value::GetAtt {
            get_att: vec![
                "ElasticLoadBalancer".to_string(),
                "SourceSecurityGroup.OwnerAlias".to_string(),
            ],
        };

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_value_join() {
        let yaml = "Fn::Join: ['', ['http://', Fn::GetAtt: [ElasticLoadBalancer, DNSName]]]";
        let expected = Value::Join {
            join: (
                "".to_string(),
                vec![
                    Value::String("http://".to_string()),
                    Value::GetAtt {
                        get_att: vec!["ElasticLoadBalancer".to_string(), "DNSName".to_string()],
                    },
                ],
            ),
        };

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_value_sub() {
        let yaml = "Fn::Sub: '${AWS::StackName}-VPCID'";
        let expected = Value::Sub {
            sub: "${AWS::StackName}-VPCID".to_string(),
        };

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }
}
