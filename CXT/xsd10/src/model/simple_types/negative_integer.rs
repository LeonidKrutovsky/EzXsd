use num_bigint::{BigInt, ToBigInt};
use std::fmt;

use std::str::FromStr;
use crate::model::ToXml;

// xsd:negativeInteger
// The type xsd:negativeInteger represents an arbitrarily large negative integer. An xsd:negativeInteger is a sequence of digits, preceded by a - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:nonPositiveInteger
// Maximum Inclusive: -1
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	    Comment
// -3
// -00122	          leading zeros are permitted
// Invalid values	Comment
// 0	              0 is not considered negative
// 122	              value cannot be positive
// +3	              value cannot be positive
// 3.0	              value must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:decimal
//         restricted by xsd:integer
//             restricted by xsd:nonPositiveInteger
//                 restricted by xsd:negativeInteger

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct NegativeInteger(pub BigInt);

impl NegativeInteger {
    pub fn from_bigint(bigint: BigInt) -> Self {
        NegativeInteger(bigint)
    }
}

impl ToBigInt for NegativeInteger {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.0.clone())
    }
}

impl FromStr for NegativeInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigInt::from_str(s).map_err(|e| e.to_string())?;
        if value >= 0.to_bigint().unwrap() {
            Err("Bad value for NegativeInteger".to_string())
        } else {
            Ok(NegativeInteger(value))
        }
    }
}

impl ToXml for NegativeInteger {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}

impl fmt::Display for NegativeInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_integer_parse_test() {
        assert_eq!(
            NegativeInteger::from_str("-12678967543233"),
            Ok(NegativeInteger(
                BigInt::from_str("-12678967543233").unwrap()
            ))
        );

        assert_eq!(
            NegativeInteger::from_str("-100000"),
            Ok(NegativeInteger(-100000.to_bigint().unwrap()))
        );

        assert_eq!(
            NegativeInteger::from_str("-1"),
            Ok(NegativeInteger(-1.to_bigint().unwrap()))
        );

        // Invalid values.
        assert!(NegativeInteger::from_str("0").is_err());
        assert!(NegativeInteger::from_str("+0").is_err());
        assert!(NegativeInteger::from_str("-0").is_err());
        assert!(NegativeInteger::from_str("1").is_err());
        assert!(NegativeInteger::from_str("+1234").is_err());
        assert!(NegativeInteger::from_str("A").is_err());
        assert!(NegativeInteger::from_str("--1").is_err());
        assert!(NegativeInteger::from_str("++1").is_err());
        assert!(NegativeInteger::from_str("-+1").is_err());
    }

    #[test]
    fn negative_integer_display_test() {
        assert_eq!(
            NegativeInteger(BigInt::from_str("-12678967543233").unwrap()).to_string(),
            "-12678967543233"
        );

        assert_eq!(
            NegativeInteger(-100000.to_bigint().unwrap()).to_string(),
            "-100000"
        );

        assert_eq!(NegativeInteger(-1.to_bigint().unwrap()).to_string(), "-1");
    }
}
