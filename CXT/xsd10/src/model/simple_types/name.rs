// xsd:Name
// The type xsd:Name represents an XML name, which can be used as an element-type
// name or attribute name, among other things. Specifically, this means that values
// must start with a letter, underscore(_), or colon (:), and may contain only letters,
// digits, underscores (_), colons (:), hyphens (-), and periods (.).
// Colons should only be used to separate namespace prefixes from local names.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: \i\c*
// White Space: collapse (Defined in type xsd:token)

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                          restricted by xsd:IDREF
//                              used in list xsd:IDREFS
//                          restricted by xsd:ENTITY
//                              used in list xsd:ENTITIES

use std::str::FromStr;

use crate::model::simple_types::Token;

#[derive(Debug, Default, Clone)]
pub struct Name(Token);

impl FromStr for Name {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(format!(
                "An empty value is not valid, unless xsi:nil is used"
            ))
        } else if value.starts_with('-') {
            Err(format!("A Name must not start with a hyphen: {}", value))
        } else if value.chars().nth(0).unwrap().is_digit(10) {
            Err(format!("A Name must not start with a number: {}", value))
        } else {
            Ok(Self(value.parse()?))
        }
    }
}

impl_from_string!(Name);
impl_as_ref!(Name);
impl_display!(Name);

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::name::Name;

    #[test]
    fn test_valid_name() {
        fn is_ok(s: &str) {
            assert!(Name::from_str(s).is_ok());
        }

        is_ok("myElement");
        is_ok("_my.Element");
        is_ok("my-element");
        is_ok("pre:myelement3");
    }

    #[test]
    fn test_invalid_name() {
        assert_eq!(
            Name::from_str("-myelement").err().unwrap(),
            "A Name must not start with a hyphen: -myelement"
        );

        assert_eq!(
            Name::from_str("3myelement").err().unwrap(),
            "A Name must not start with a number: 3myelement"
        );

        assert_eq!(
            Name::from_str("").err().unwrap(),
            "An empty value is not valid, unless xsi:nil is used"
        );
    }
}
