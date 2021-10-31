use xml_utils::attribute;

// abstract
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:boolean.

// Used in
// Type xsd:topLevelComplexType (Element xsd:complexType)
// Type xsd:topLevelElement (Element xsd:element)
#[attribute(name = "abstract")]
pub struct Abstract(pub bool);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Abstract;

    #[test]
    fn test_valid_values() {
        assert!(!Abstract::from_str("0").unwrap().0);
        assert!(!Abstract::from_str("0").unwrap().0);
        assert!(Abstract::from_str("1").unwrap().0);
        assert!(Abstract::from_str("true").unwrap().0);
    }

    #[test]
    fn test_invalid_values() {
        assert_eq!(
            Abstract::from_str("2").err().unwrap(),
            "Attribute <abstract> Error! Invalid value for boolean: 2".to_string()
        );
        assert!(Abstract::from_str("True").is_err());
        assert!(Abstract::from_str("FALSE").is_err());
        assert!(Abstract::from_str("").is_err());
    }

    #[test]
    fn test_to_text() {
        assert_eq!(Abstract::from_str("0").unwrap().text(), " abstract=false");
        assert_eq!(Abstract::from_str("1").unwrap().text(), " abstract=true");
    }
}
