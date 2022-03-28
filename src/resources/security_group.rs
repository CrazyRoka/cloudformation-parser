use crate::{tag::Tag, value::Value};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityGroup {
    group_description: String,
    group_name: Option<Value>,
    security_group_egress: Option<Vec<Egress>>,
    security_group_ingress: Option<Vec<Ingress>>,
    tags: Option<Vec<Tag>>,
    vpc_id: Option<Value>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct Egress {
    cidr_ip: Option<Value>,
    cidr_ipv6: Option<Value>,
    description: Option<String>,
    destination_prefix_list_id: Option<Value>,
    destination_security_group_id: Option<Value>,
    from_port: Option<Value>,
    ip_protocol: Value,
    to_port: Option<Value>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct Ingress {
    cidr_ip: Option<Value>,
    cidr_ipv6: Option<Value>,
    description: Option<String>,
    from_port: Option<Value>,
    ip_protocol: Value,
    source_prefix_list_id: Option<Value>,
    source_security_group_id: Option<Value>,
    source_security_group_name: Option<Value>,
    source_security_group_owner_id: Option<Value>,
    to_port: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::{Egress, Ingress, SecurityGroup};
    use crate::{tag::Tag, value::Value};

    #[test]
    fn test_deserialize_security_group() {
        let yaml = r#"
        GroupDescription: Allow http to client host
        VpcId:
           Ref: myVPC
        SecurityGroupIngress:
        - IpProtocol: tcp
          FromPort: 80
          ToPort: 80
          CidrIp: 0.0.0.0/0
        SecurityGroupEgress:
        - IpProtocol: tcp
          FromPort: 80
          ToPort: 80
          CidrIp: 0.0.0.0/0
        Tags:
        - Key: Name
          Value: mySecurityGroup
        "#;
        let expected = SecurityGroup {
            group_description: "Allow http to client host".to_string(),
            group_name: None,
            security_group_egress: Some(vec![Egress {
                cidr_ip: Some(Value::String("0.0.0.0/0".to_string())),
                cidr_ipv6: None,
                description: None,
                destination_prefix_list_id: None,
                destination_security_group_id: None,
                from_port: Some(Value::Number(80)),
                ip_protocol: Value::String("tcp".to_string()),
                to_port: Some(Value::Number(80)),
            }]),
            security_group_ingress: Some(vec![Ingress {
                ip_protocol: Value::String("tcp".to_string()),
                cidr_ip: Some(Value::String("0.0.0.0/0".to_string())),
                cidr_ipv6: None,
                description: None,
                from_port: Some(Value::Number(80)),
                source_prefix_list_id: None,
                source_security_group_id: None,
                source_security_group_name: None,
                source_security_group_owner_id: None,
                to_port: Some(Value::Number(80)),
            }]),
            tags: Some(vec![Tag {
                key: "Name".to_string(),
                value: "mySecurityGroup".to_string(),
            }]),
            vpc_id: Some(Value::Ref {
                r#ref: "myVPC".to_string(),
            }),
        };

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_egress() {
        let yaml = r#"
  IpProtocol: tcp
  FromPort: 0
  ToPort: 65535
  Description: Allow any outbound traffic
  DestinationSecurityGroupId:
    Fn::GetAtt:
    - TargetSG
    - GroupId
  GroupId:
    Fn::GetAtt:
    - SourceSG
    - GroupId
  CidrIp: 0.0.0.0/0 
  CidrIpv6: ::/0
        "#;
        let expected = Egress {
            cidr_ip: Some(Value::String("0.0.0.0/0".to_string())),
            cidr_ipv6: Some(Value::String("::/0".to_string())),
            description: Some("Allow any outbound traffic".to_string()),
            destination_prefix_list_id: None,
            destination_security_group_id: Some(Value::GetAtt {
                get_att: vec!["TargetSG".to_string(), "GroupId".to_string()],
            }),
            from_port: Some(Value::Number(0)),
            to_port: Some(Value::Number(65535)),
            ip_protocol: Value::String("tcp".to_string()),
        };

        let actual = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_ingress() {
        let yaml = r#"
- IpProtocol: tcp
  FromPort: 22
  ToPort: 22
  CidrIp:
    Ref: 'SSHLocation'
  CidrIpv6: ::/0
  Description: Allow HTTP
- IpProtocol: tcp
  FromPort: 80
  ToPort: 80
  SourceSecurityGroupId:
    Ref: SecurityGroupBastion
  SourceSecurityGroupOwnerId:
    Fn::GetAtt: [ElasticLoadBalancer, SourceSecurityGroup.OwnerAlias]
  SourceSecurityGroupName:
    Fn::GetAtt: [ElasticLoadBalancer, SourceSecurityGroup.GroupName]
        "#;
        let expected = vec![
            Ingress {
                ip_protocol: Value::String("tcp".to_string()),
                cidr_ip: Some(Value::Ref {
                    r#ref: "SSHLocation".to_string(),
                }),
                cidr_ipv6: Some(Value::String("::/0".to_string())),
                description: Some("Allow HTTP".to_string()),
                from_port: Some(Value::Number(22)),
                to_port: Some(Value::Number(22)),
                source_prefix_list_id: None,
                source_security_group_id: None,
                source_security_group_name: None,
                source_security_group_owner_id: None,
            },
            Ingress {
                ip_protocol: Value::String("tcp".to_string()),
                cidr_ip: None,
                cidr_ipv6: None,
                description: None,
                from_port: Some(Value::Number(80)),
                to_port: Some(Value::Number(80)),
                source_prefix_list_id: None,
                source_security_group_id: Some(Value::Ref {
                    r#ref: "SecurityGroupBastion".to_string(),
                }),
                source_security_group_name: Some(Value::GetAtt {
                    get_att: vec![
                        "ElasticLoadBalancer".to_string(),
                        "SourceSecurityGroup.GroupName".to_string(),
                    ],
                }),
                source_security_group_owner_id: Some(Value::GetAtt {
                    get_att: vec![
                        "ElasticLoadBalancer".to_string(),
                        "SourceSecurityGroup.OwnerAlias".to_string(),
                    ],
                }),
            },
        ];

        let actual: Vec<Ingress> = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(expected, actual);
    }
}
