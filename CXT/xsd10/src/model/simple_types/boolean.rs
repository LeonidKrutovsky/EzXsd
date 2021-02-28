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
// Invalid values	Comment
// TRUE	            values are case sensitive
// T	            the word "true" must be spelled out
//                  an empty value is not valid, unless xsi:nil is used
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:boolean

use std::convert::TryFrom;
use std::str::FromStr;
use crate::model::ToXml;

pub struct Boolean(pub bool);

impl FromStr for Boolean {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" || s == "false" {
            Ok(Self(false))
        } else if s == "1" || s == "true" {
            Ok(Self(true))
        } else {
            Err(format!("Invalid value for boolean: {}", s))
        }
    }
}

impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl ToXml for Boolean {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::Boolean;
    use std::str::FromStr;

    #[test]
    fn test_valid_values() {
        assert!(!Boolean::from_str("0").unwrap().0);
        assert!(!Boolean::from_str("0").unwrap().0);
        assert!(Boolean::from_str("1").unwrap().0);
        assert!(Boolean::from_str("true").unwrap().0);
    }

    #[test]
    fn test_invalid_values() {
        assert_eq!(
            Boolean::from_str("2").err().unwrap(),
            "Invalid value for boolean: 2".to_string()
        );
        assert!(Boolean::from_str("True").is_err());
        assert!(Boolean::from_str("FALSE").is_err());
        assert!(Boolean::from_str("").is_err());
    }
}
