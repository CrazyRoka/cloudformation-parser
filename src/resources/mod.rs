use self::{ec2::Ec2, security_group::SecurityGroup, vpc::Vpc};
use serde::Deserialize;

mod ec2;
mod security_group;
mod vpc;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Type")]
pub enum Resource {
    #[serde(rename = "AWS::EC2::Instance")]
    Ec2(ResourceContainer<Ec2>),
    #[serde(rename = "AWS::EC2::VPC")]
    Vpc(ResourceContainer<Vpc>),
    #[serde(rename = "AWS::SNS::Topic")]
    Topic,
    #[serde(rename = "AWS::AutoScaling::AutoScalingGroup")]
    AutoScalingGroup,
    #[serde(rename = "AWS::AutoScaling::LaunchConfiguration")]
    LaunchConfiguration,
    #[serde(rename = "AWS::AutoScaling::ScalingPolicy")]
    ScalingPolicy,
    #[serde(rename = "AWS::CloudWatch::Alarm")]
    Alarm,
    #[serde(rename = "AWS::ElasticLoadBalancing::LoadBalancer")]
    LoadBalancer,
    #[serde(rename = "AWS::EC2::SecurityGroup")]
    SecurityGroup(ResourceContainer<SecurityGroup>),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceContainer<T> {
    properties: T,
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::{ec2::Ec2, Resource, ResourceContainer};

    #[test]
    fn test_deserialize_resource() {
        let json = r#"
{
    "Type": "AWS::EC2::Instance",
    "Properties": {
        "KeyName": "myKey"
    }
}
        "#;
        let expected = Resource::Ec2(ResourceContainer {
            properties: Ec2 {
                key_name: Some(Value::String("myKey".to_string())),
                security_groups: None,
                image_id: None,
            },
        });

        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
