use crate::{data_type::DataType, value::Value};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(rename = "Type")]
    data_type: DataType,
    description: Option<String>,
    max_length: Option<Value>,
    min_length: Option<Value>,
    max_value: Option<Value>,
    min_value: Option<Value>,
    default: Option<Value>,
    allowed_pattern: Option<String>,
    allowed_values: Option<Vec<Value>>,
    constraint_description: Option<String>,
    no_echo: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::Parameter;
    use crate::{data_type::DataType, value::Value};

    #[test]
    fn test_deserialize_parameters() {
        let test_cases = [
            (
                r#"
Description: The EC2 Key Pair to allow SSH access to the instances
Type: AWS::EC2::KeyPair::KeyName
ConstraintDescription: must be the name of an existing EC2 KeyPair.
        "#,
                Parameter {
                    description: Some(
                        "The EC2 Key Pair to allow SSH access to the instances".to_string(),
                    ),
                    data_type: DataType::KeyName,
                    constraint_description: Some(
                        "must be the name of an existing EC2 KeyPair.".to_string(),
                    ),
                    max_length: None,
                    min_length: None,
                    max_value: None,
                    min_value: None,
                    default: None,
                    allowed_pattern: None,
                    allowed_values: None,
                    no_echo: None,
                },
            ),
            (
                r#"
Description: The IP address range that can be used to SSH to the EC2 instances
Type: String
MinLength: 9
MaxLength: 18
Default: 0.0.0.0/0
AllowedPattern: (\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d{1,2})
ConstraintDescription: must be a valid IP CIDR range of the form x.x.x.x/x.
        "#,
                Parameter {
                    description: Some(
                        "The IP address range that can be used to SSH to the EC2 instances"
                            .to_string(),
                    ),
                    data_type: DataType::String,
                    constraint_description: Some(
                        "must be a valid IP CIDR range of the form x.x.x.x/x.".to_string(),
                    ),
                    max_length: Some(Value::Number(18)),
                    min_length: Some(Value::Number(9)),
                    max_value: None,
                    min_value: None,
                    default: Some(Value::String("0.0.0.0/0".to_string())),
                    allowed_pattern: Some(
                        "(\\d{1,3})\\.(\\d{1,3})\\.(\\d{1,3})\\.(\\d{1,3})/(\\d{1,2})".to_string(),
                    ),
                    allowed_values: None,
                    no_echo: None,
                },
            ),
            (
                r#"
Description: WebServer EC2 instance type
Type: String
Default: t2.small
AllowedValues: [t1.micro, t2.nano, t2.micro, t2.small]
ConstraintDescription: must be a valid EC2 instance type.
            "#,
                Parameter {
                    description: Some("WebServer EC2 instance type".to_string()),
                    data_type: DataType::String,
                    constraint_description: Some("must be a valid EC2 instance type.".to_string()),
                    max_length: None,
                    min_length: None,
                    max_value: None,
                    min_value: None,
                    default: Some(Value::String("t2.small".to_string())),
                    allowed_pattern: None,
                    allowed_values: Some(vec![
                        Value::String("t1.micro".to_string()),
                        Value::String("t2.nano".to_string()),
                        Value::String("t2.micro".to_string()),
                        Value::String("t2.small".to_string()),
                    ]),
                    no_echo: None,
                },
            ),
            (
                r#"
DBPort: 
Default: 3306
Description: TCP/IP port for the database
Type: Number
MinValue: 1150
MaxValue: 65535
NoEcho: true
            "#,
                Parameter {
                    description: Some("TCP/IP port for the database".to_string()),
                    data_type: DataType::Number,
                    constraint_description: None,
                    max_length: None,
                    min_length: None,
                    max_value: Some(Value::Number(65535)),
                    min_value: Some(Value::Number(1150)),
                    default: Some(Value::Number(3306)),
                    allowed_pattern: None,
                    allowed_values: None,
                    no_echo: Some(true),
                },
            ),
        ];

        for (yaml, expected) in test_cases {
            let actual = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_deserialize_json() {
        let json = r#"
{
    "Default" : "3306",
    "Description" : "TCP/IP port for the database",
    "Type" : "Number",
    "MinValue" : "1150",
    "MaxValue" : "65535"
}"#;
        let expected = Parameter {
            default: Some(Value::String("3306".to_string())),
            description: Some("TCP/IP port for the database".to_string()),
            data_type: DataType::Number,
            min_value: Some(Value::String("1150".to_string())),
            max_value: Some(Value::String("65535".to_string())),
            max_length: None,
            min_length: None,
            allowed_pattern: None,
            allowed_values: None,
            constraint_description: None,
            no_echo: None,
        };

        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
