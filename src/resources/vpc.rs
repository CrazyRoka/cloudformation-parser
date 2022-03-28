use crate::{tag::Tag, value::Value};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Vpc {
    cidr_block: String,
    enable_dns_hostnames: Option<Value>,
    enable_dns_support: Option<Value>,
    instance_tenancy: Option<InstanceTenancy>,
    ipv4_ipam_pool_id: Option<String>,
    ipv4_netmask_length: Option<Value>,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum InstanceTenancy {
    Default,
    Dedicated,
    Host,
}

#[cfg(test)]
mod tests {
    use crate::{resources::vpc::InstanceTenancy, tag::Tag, value::Value};

    use super::Vpc;

    #[test]
    fn test_deserialize_vpc() {
        let json = r#"
{
    "CidrBlock" : "10.0.0.0/16",
    "EnableDnsSupport" : "true",
    "EnableDnsHostnames" : "true",
    "InstanceTenancy": "dedicated",
    "Ipv4NetmaskLength": "28",
    "Tags" : [ 
        {"Key" : "stack", "Value" : "production"} 
    ]
}"#;
        let expected = Vpc {
            cidr_block: "10.0.0.0/16".to_string(),
            enable_dns_support: Some(Value::String("true".to_string())),
            enable_dns_hostnames: Some(Value::String("true".to_string())),
            tags: Some(vec![Tag {
                key: "stack".to_string(),
                value: "production".to_string(),
            }]),
            instance_tenancy: Some(InstanceTenancy::Dedicated),
            ipv4_ipam_pool_id: None,
            ipv4_netmask_length: Some(Value::String("28".to_string())),
        };

        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
