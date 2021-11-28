use xml_utils::attribute;

use crate::model::simple_types::FormChoice;

// attributeFormDefault
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:formChoice
// Properties: Local, Unqualified
//
// Value
// Type based on xsd:NMTOKEN

// Valid value	Description
// qualified	Local declarations are qualified (in a namespace)
// unqualified	Local declarations are unqualified (not in a namespace)

// Used in
// Anonymous type of element xsd:schema
#[attribute(name = "attributeFormDefault")]
pub struct AttributeFormDefault(pub FormChoice);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::AttributeFormDefault;
    use super::FormChoice;

    #[test]
    fn test_valid_values() {
        assert_eq!(
            AttributeFormDefault::from_str("qualified").unwrap().0,
            FormChoice::Qualified
        );
        assert_eq!(
            AttributeFormDefault::from_str("unqualified").unwrap().0,
            FormChoice::Unqualified
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(
            AttributeFormDefault::from_str("qualified").unwrap().text(),
            " attributeFormDefault=\"qualified\""
        );
        assert_eq!(
            AttributeFormDefault::from_str("unqualified")
                .unwrap()
                .text(),
            " attributeFormDefault=\"unqualified\""
        );
    }

    #[test]
    fn to_text() {
        let value = AttributeFormDefault(FormChoice::Unqualified);
        assert_eq!(value.text(), " attributeFormDefault=\"unqualified\"");

        let value = AttributeFormDefault(FormChoice::Qualified);
        assert_eq!(value.text(), " attributeFormDefault=\"qualified\"");
    }
}
