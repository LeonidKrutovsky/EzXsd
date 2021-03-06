// xsd:NMTOKEN
// The type xsd:NMTOKEN represents a single string token. xsd:NMTOKEN values
// may consist of letters, digits, periods (.), hyphens (-), underscores (_), and colons (:).
// They may start with any of these characters. xsd:NMTOKEN has a whiteSpace facet
// value of collapse, so any leading or trailing whitespace will be removed.
// However, no whitespace may appear within the value itself.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: \c+
// White Space: collapse (Defined in type xsd:token)

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS

use std::str::FromStr;

use crate::model::simple_types::Token;

#[derive(Debug)]
pub struct NmToken(Token);

impl_from_string!(NmToken);
impl_as_ref!(NmToken);

impl FromStr for NmToken {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(format!(
                "An empty value is not valid NMTOKEN, unless xsi:nil is used"
            ))
        } else if value.contains(" ") {
            Err(format!("NMTOKEN must not contain a space: {}", value))
        } else {
            Ok(Self(value.parse()?))
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::nmtoken::NmToken;

    #[test]
    fn test_valid_string() {
        fn is_ok(s: &str) {
            assert!(NmToken::from_str(s).is_ok());
        }
        is_ok("12.5");
        is_ok("xsi:nil");
        is_ok("3&lt;4")
    }

    #[test]
    fn test_err_string() {
        assert_eq!(
            NmToken::from_str("name with spaces").err().unwrap(),
            "NMTOKEN must not contain a space: name with spaces"
        );

        assert_eq!(
            NmToken::from_str("").err().unwrap(),
            "An empty value is not valid NMTOKEN, unless xsi:nil is used"
        );
    }
}
