use std::fmt;
use std::str::FromStr;

use num_bigint::{BigUint, ToBigUint};

// xsd:positiveInteger
// The type xsd:positiveInteger represents an arbitrarily large positive integer. An xsd:positiveInteger is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
//  Based on xsd:nonNegativeInteger
//  Minimum Inclusive: 1
//  Fraction Digits: 0 (Defined in type xsd:integer)
//  Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
//  White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	    Comment
// 122
// +3
// 00122	        leading zeros are permitted
// Invalid values	Comment
// 0	            0 is not considered positive
// -3	            value cannot be negative
// 3.0	            value must not contain a decimal point
//                  an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
// restricted by xsd:decimal
// restricted by xsd:integer
// restricted by xsd:nonNegativeInteger
// restricted by xsd:positiveInteger

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct PositiveInteger(pub BigUint);

impl PositiveInteger {
    pub fn from_biguint(bigint: BigUint) -> Self {
        PositiveInteger(bigint)
    }
}

impl ToBigUint for PositiveInteger {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.0.clone())
    }
}

impl FromStr for PositiveInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigUint::from_str(s).map_err(|e| e.to_string())?;
        if value <= 0.to_biguint().unwrap() {
            Err("Bad value for PositiveInteger".to_string())
        } else {
            Ok(PositiveInteger(value))
        }
    }
}

impl fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_integer_parse_test() {
        assert_eq!(
            PositiveInteger::from_str("12678967543233"),
            Ok(PositiveInteger(
                BigUint::from_str("12678967543233").unwrap()
            ))
        );

        assert_eq!(
            PositiveInteger::from_str("+100000"),
            Ok(PositiveInteger(100000.to_biguint().unwrap()))
        );

        // Invalid values.
        assert!(PositiveInteger::from_str("0").is_err());
        assert!(PositiveInteger::from_str("+0").is_err());
        assert!(PositiveInteger::from_str("-0").is_err());
        assert!(PositiveInteger::from_str("-1").is_err());
        assert!(PositiveInteger::from_str("-1234").is_err());
        assert!(PositiveInteger::from_str("A").is_err());
        assert!(PositiveInteger::from_str("--1").is_err());
        assert!(PositiveInteger::from_str("++1").is_err());
        assert!(PositiveInteger::from_str("-+1").is_err());
    }

    #[test]
    fn positive_integer_display_test() {
        assert_eq!(
            PositiveInteger(BigUint::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(
            PositiveInteger(100000.to_biguint().unwrap()).to_string(),
            "100000"
        );
    }
}
