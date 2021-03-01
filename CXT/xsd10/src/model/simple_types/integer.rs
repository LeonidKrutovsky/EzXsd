use crate::model::ToXml;
use num_bigint::{BigInt, ToBigInt};
use std::fmt;
use std::str::FromStr;

// xsd:integer
// The type xsd:integer represents an arbitrarily large integer, from which twelve other built-in integer types are derived (directly or indirectly). An xsd:integer is a sequence of digits, optionally preceded by a + or - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:decimal
// Fraction Digits: 0
// Pattern: [\-+]?[0-9]+
// White Space: collapse (Defined in type xsd:decimal)

// Examples
// Valid values     Comment
// 122
// 00122	          leading zeros are permitted
// 0
// -3
// +3
// Invalid values	Comment
// 3.	              an integer must not contain a decimal point
// 3.0	              an integer must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used

// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:decimal
//         restricted by xsd:integer
//             restricted by xsd:nonPositiveInteger
//                 restricted by xsd:negativeInteger
//             restricted by xsd:long
//                 restricted by xsd:int
//                     restricted by xsd:short
//                         restricted by xsd:byte
//             restricted by xsd:nonNegativeInteger
//                 restricted by xsd:unsignedLong
//                     restricted by xsd:unsignedInt
//                         restricted by xsd:unsignedShort
//                             restricted by xsd:unsignedByte
//                 restricted by xsd:positiveInteger

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Integer(pub BigInt);

impl Integer {
    pub fn from_bigint(bigint: BigInt) -> Self {
        Integer(bigint)
    }
}

impl ToBigInt for Integer {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.0.clone())
    }
}

impl FromStr for Integer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Integer(BigInt::from_str(s).map_err(|e| e.to_string())?))
    }
}

impl ToXml for Integer {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_parse_test() {
        assert_eq!(
            Integer::from_str("12678967543233"),
            Ok(Integer(BigInt::from_str("12678967543233").unwrap()))
        );

        assert_eq!(
            Integer::from_str("+100000"),
            Ok(Integer(100000.to_bigint().unwrap()))
        );

        assert_eq!(Integer::from_str("0"), Ok(Integer(0.to_bigint().unwrap())));

        assert_eq!(
            Integer::from_str("-1"),
            Ok(Integer(-1.to_bigint().unwrap()))
        );

        // Invalid values.
        assert!(Integer::from_str("A").is_err());
        assert!(Integer::from_str("--1").is_err());
        assert!(Integer::from_str("++1").is_err());
        assert!(Integer::from_str("-+1").is_err());
    }

    #[test]
    fn integer_display_test() {
        assert_eq!(
            Integer(BigInt::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(Integer(100000.to_bigint().unwrap()).to_string(), "100000");
        assert_eq!(Integer(0.to_bigint().unwrap()).to_string(), "0");
        assert_eq!(Integer(-1.to_bigint().unwrap()).to_string(), "-1");
    }
}
