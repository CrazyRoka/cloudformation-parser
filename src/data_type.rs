use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum DataType {
    String,
    Number,
    #[serde(rename = "List<Number>")]
    NumberList,
    CommaDelimitedList,
    #[serde(rename = "AWS::EC2::KeyPair::KeyName")]
    KeyName,
}

#[cfg(test)]
mod tests {
    use super::DataType;

    #[test]
    fn test_deserialize_data_type_string() {
        let yaml = "String";
        let expected = DataType::String;

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_data_type_number() {
        let yaml = "Number";
        let expected = DataType::Number;

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_data_type_number_list() {
        let yaml = "List<Number>";
        let expected = DataType::NumberList;

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_data_type_comma_delimited_list() {
        let yaml = "CommaDelimitedList";
        let expected = DataType::CommaDelimitedList;

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_data_type_keyname() {
        let yaml = "AWS::EC2::KeyPair::KeyName";
        let expected = DataType::KeyName;

        let actual = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(expected, actual);
    }
}
