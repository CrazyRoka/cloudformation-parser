use serde::Deserialize;

use crate::value::Value;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Ec2 {
    pub security_groups: Option<Vec<Value>>,
    pub key_name: Option<Value>,
    pub image_id: Option<Value>,
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::Ec2;

    #[test]
    fn test_deserialize_ec2() {
        let json = r#"
{
    "SecurityGroups": [
        {
            "Ref": "InstanceSecurityGroup"
        },
        "MyExistingSecurityGroup"
    ],
    "KeyName": "mykey",
    "ImageId": "ami-7a11e213"
}
        "#;
        let expected = Ec2 {
            image_id: Some(Value::String("ami-7a11e213".to_string())),
            key_name: Some(Value::String("mykey".to_string())),
            security_groups: Some(vec![
                Value::Ref {
                    r#ref: "InstanceSecurityGroup".to_string(),
                },
                Value::String("MyExistingSecurityGroup".to_string()),
            ]),
        };

        let actual = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }
}
