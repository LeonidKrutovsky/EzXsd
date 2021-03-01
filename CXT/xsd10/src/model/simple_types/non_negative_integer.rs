use num_bigint::{BigUint, ToBigUint};
use std::fmt;

use crate::model::ToXml;
use std::str::FromStr;

// xsd:nonNegativeInteger
// The type xsd:nonNegativeInteger represents an arbitrarily large non-negative integer. An xsd:nonNegativeInteger is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:integer
// Minimum Inclusive: 0
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	      Comment
// +3
// 122
// 0
// 00122	            leading zeros are permitted
// Invalid values	 Comment
// -3	                value cannot be negative
// 3.0	                value must not contain a decimal point
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//  restricted by xsd:decimal
//      restricted by xsd:integer
//          restricted by xsd:nonNegativeInteger
//              restricted by xsd:unsignedLong
//                  restricted by xsd:unsignedInt
//                      restricted by xsd:unsignedShort
//                          restricted by xsd:unsignedByte
//          restricted by xsd:positiveInteger

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct NonNegativeInteger(pub BigUint);

impl NonNegativeInteger {
    pub fn from_biguint(bigint: BigUint) -> Self {
        NonNegativeInteger(bigint)
    }
}

impl ToBigUint for NonNegativeInteger {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.0.clone())
    }
}

impl FromStr for NonNegativeInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigUint::from_str(s).map_err(|e| e.to_string())?;
        if value < 0.to_biguint().unwrap() {
            Err("Bad value for NonNegativeInteger".to_string())
        } else {
            Ok(NonNegativeInteger(value))
        }
    }
}

impl ToXml for NonNegativeInteger {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}

impl fmt::Display for NonNegativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_negative_integer_parse_test() {
        assert_eq!(
            NonNegativeInteger::from_str("12678967543233"),
            Ok(NonNegativeInteger(
                BigUint::from_str("12678967543233").unwrap()
            ))
        );

        assert_eq!(
            NonNegativeInteger::from_str("+100000"),
            Ok(NonNegativeInteger(100000.to_biguint().unwrap()))
        );

        assert_eq!(
            NonNegativeInteger::from_str("0"),
            Ok(NonNegativeInteger(0.to_biguint().unwrap()))
        );

        // Invalid values.
        assert!(NonNegativeInteger::from_str("-1").is_err());
        assert!(NonNegativeInteger::from_str("-1234").is_err());
        assert!(NonNegativeInteger::from_str("A").is_err());
        assert!(NonNegativeInteger::from_str("--1").is_err());
        assert!(NonNegativeInteger::from_str("++1").is_err());
        assert!(NonNegativeInteger::from_str("-+1").is_err());
    }

    #[test]
    fn non_negative_integer_display_test() {
        assert_eq!(
            NonNegativeInteger(BigUint::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(
            NonNegativeInteger(100000.to_biguint().unwrap()).to_string(),
            "100000"
        );

        assert_eq!(NonNegativeInteger(0.to_biguint().unwrap()).to_string(), "0");
    }
}
