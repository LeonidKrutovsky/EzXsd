use num_bigint::{BigInt, ToBigInt};
use std::fmt;

use crate::model::ToXml;
use std::str::FromStr;

// xsd:nonPositiveInteger
// The type xsd:nonPositiveInteger represents an arbitrarily large non-positive integer. An xsd:nonPositiveInteger is a sequence of digits, optionally preceded by a - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:integer
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
// Maximum Inclusive: 0
//
// Examples
// Valid values     Comment
// -3
// 0
// -00122             leading zeros are permitted
// Invalid values   Comment
// 122                value cannot be positive
// +3                 value cannot be positive
// 3.0                value must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//     xsd:anySimpleType
//         restricted by xsd:decimal
//             restricted by xsd:integer
//                 restricted by xsd:nonPositiveInteger
//                     restricted by xsd:negativeInteger

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct NonPositiveInteger(pub BigInt);

impl NonPositiveInteger {
    pub fn from_bigint(bigint: BigInt) -> Self {
        NonPositiveInteger(bigint)
    }
}

impl ToBigInt for NonPositiveInteger {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.0.clone())
    }
}

impl FromStr for NonPositiveInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigInt::from_str(s).map_err(|e| e.to_string())?;
        if value > 0.to_bigint().unwrap() {
            Err("Bad value for NonPositiveInteger".to_string())
        } else {
            Ok(NonPositiveInteger(value))
        }
    }
}

impl ToXml for NonPositiveInteger {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}

impl fmt::Display for NonPositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_positive_integer_parse_test() {
        assert_eq!(
            NonPositiveInteger::from_str("-12678967543233"),
            Ok(NonPositiveInteger(
                BigInt::from_str("-12678967543233").unwrap()
            ))
        );

        assert_eq!(
            NonPositiveInteger::from_str("-100000"),
            Ok(NonPositiveInteger(-100000.to_bigint().unwrap()))
        );

        assert_eq!(
            NonPositiveInteger::from_str("-1"),
            Ok(NonPositiveInteger(-1.to_bigint().unwrap()))
        );

        assert_eq!(
            NonPositiveInteger::from_str("0"),
            Ok(NonPositiveInteger(0.to_bigint().unwrap()))
        );

        // Invalid values.
        assert!(NonPositiveInteger::from_str("1").is_err());
        assert!(NonPositiveInteger::from_str("+1234").is_err());
        assert!(NonPositiveInteger::from_str("A").is_err());
        assert!(NonPositiveInteger::from_str("--1").is_err());
        assert!(NonPositiveInteger::from_str("++1").is_err());
        assert!(NonPositiveInteger::from_str("-+1").is_err());
    }

    #[test]
    fn non_positive_integer_display_test() {
        assert_eq!(
            NonPositiveInteger(BigInt::from_str("-12678967543233").unwrap()).to_string(),
            "-12678967543233"
        );

        assert_eq!(
            NonPositiveInteger(-100000.to_bigint().unwrap()).to_string(),
            "-100000"
        );

        assert_eq!(NonPositiveInteger(0.to_bigint().unwrap()).to_string(), "0");

        assert_eq!(
            NonPositiveInteger(-1.to_bigint().unwrap()).to_string(),
            "-1"
        );
    }
}
