// xsd:boolean
// The type xsd:boolean represents logical yes/no values.
// The valid values for xsd:boolean are true, false, 0, and 1.
// Values that are capitalized (e.g. TRUE) or abbreviated (e.g. T) are not valid.

// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema

// Schema Document: datatypes.xsd

// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values	    Comment
// true
// false
// 0	            false
// 1	            true

use std::convert::TryFrom;

// Invalid values	Comment
// TRUE	            values are case sensitive
// T	            the word "true" must be spelled out
//                  an empty value is not valid, unless xsi:nil is used
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:boolean
pub struct Boolean(pub bool);

impl TryFrom<&str> for Boolean {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "0" || s == "false" {
            Ok(Self(false))
        } else if s == "1" || s == "true" {
            Ok(Self(true))
        } else {
            Err(format!("Invalid value for boolean: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    use std::convert::TryFrom;

    #[test]
    fn test_valid_values() {
        assert!(!Boolean::try_from("0").unwrap().0);
        assert!(!Boolean::try_from("0").unwrap().0);
        assert!(Boolean::try_from("1").unwrap().0);
        assert!(Boolean::try_from("true").unwrap().0);
    }

    #[test]
    fn test_invalid_values() {
        assert_eq!(
            Boolean::try_from("2").err().unwrap(),
            "Invalid value for boolean: 2".to_string()
        );
        assert!(Boolean::try_from("True").is_err());
        assert!(Boolean::try_from("FALSE").is_err());
        assert!(Boolean::try_from("").is_err());
    }
}
