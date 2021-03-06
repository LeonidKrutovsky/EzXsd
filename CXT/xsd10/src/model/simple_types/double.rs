#![allow(illegal_floating_point_literal_pattern)]
// xsd:double
// The type xsd:double represents an IEEE double-precision 64-bit floating-point number.
// The format of xsd:double values is a mantissa (a number which conforms to the type decimal)
// followed, optionally, by the character "E" or "e" followed by an exponent.
// The exponent must be an integer. For example, 3E2 represents 3 times 10 to the 2nd power,
// or 300. The exponent must be an integer.
//
// In addition, the following values are valid: INF (infinity), -INF (negative infinity),
// and NaN (Not a Number). INF is considered to be greater than all other values,
// while -INF is less than all other values. The value NaN cannot be compared to
// any other values, although it equals itself.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Double(pub f64);

impl FromStr for Double {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-INF" => Ok(Double(f64::NEG_INFINITY)),
            "INF" => Ok(Double(f64::INFINITY)),
            _ => Ok(Double(s.parse::<f64>().map_err(|e| e.to_string())?)),
        }
    }
}

impl From<f64> for Double {
    fn from(v: f64) -> Self {
        Self(v)
    }
}

impl PartialEq<f64> for Double {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = if self.0.is_infinite() {
            self.0.to_string().to_uppercase()
        } else {
            self.0.to_string()
        };
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::double::Double;

    #[test]
    fn test_valid_parse() {
        fn eq(left: &str, right: f64) {
            assert_eq!(Double::from_str(left).unwrap(), right)
        }

        eq("-3E2", -300.0);
        eq("4268.22752E11", 4268.22752E11);
        eq("+24.3e-3", 0.0243);
        eq("12", 12.0);
        eq("-INF", f64::NEG_INFINITY);
        eq("INF", f64::INFINITY);
        eq("-0", 0.0);
        assert!(Double::from_str("NaN").unwrap().0.is_nan());
    }

    #[test]
    fn test_invalid_parse() {
        assert!(Double::from_str("-3E2.4").is_err());
        assert!(Double::from_str("12E").is_err());
        assert!(Double::from_str("NAN").is_err());
    }

    #[test]
    fn test_to_xml() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Double::from_str(left).unwrap().to_string(), right);
        }
        eq("-3E2", "-300");
        eq("4268.22752E11", "426822752000000");
        eq("+24.3e-3", "0.0243");
        eq("12", "12");
        eq("-INF", "-INF");
        eq("INF", "INF");
        eq("-0", "0");
        eq("NaN", "NaN");
    }
}
