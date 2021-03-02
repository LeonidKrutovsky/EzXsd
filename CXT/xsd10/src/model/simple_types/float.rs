#![allow(illegal_floating_point_literal_pattern)]
// xsd:float
// The type xsd:float represents an IEEE single-precision 32-bit floating-point number. The format of xsd:float values is a mantissa (a number which conforms to the type decimal) followed, optionally, by the character "E" or "e" followed by an exponent. The exponent must be an integer. For example, 3E2 represents 3 times 10 to the 2nd power, or 300. The exponent must be an integer.
//
// In addition, the following values are valid: INF (infinity), -INF (negative infinity), and NaN (Not a Number). INF is considered to be greater than all other values, while -INF is less than all other values. The value NaN cannot be compared to any other values, although it equals itself.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse

use crate::model::Parse;

#[derive(Debug, PartialEq)]
pub struct Float(pub f32);

impl Parse for Float {
    fn parse(s: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        match s {
            "-INF" => Ok(Self(f32::NEG_INFINITY)),
            "INF" => Ok(Self(f32::INFINITY)),
            _ => Ok(Self(s.parse::<f32>().map_err(|e| e.to_string())?)),
        }
    }

    fn create(s: String) -> Self
    where
        Self: Sized,
    {
        Self(s.parse::<f32>().unwrap_or(f32::NAN))
    }

    fn text(&self) -> Result<String, String> {
        Ok(match self.0 {
            f32::NEG_INFINITY => "-INF".into(),
            f32::INFINITY => "INF".into(),
            _ => self.0.to_string(),
        })
    }
}

impl_from_str!(Float);

impl PartialEq<f32> for Float {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::float::Float;
    use crate::model::Parse;
    use std::str::FromStr;

    #[test]
    fn test_valid_parse() {
        fn eq(left: &str, right: f32) {
            assert_eq!(Float::from_str(left).unwrap(), right)
        }

        eq("-3E2", -300.0);
        eq("4268.22752E11", 4268.22752E11);
        eq("+24.3e-3", 0.0243);
        eq("12", 12.0);
        eq("-INF", f32::NEG_INFINITY);
        eq("INF", f32::INFINITY);
        eq("-0", 0.0);
        assert!(Float::from_str("NaN").unwrap().0.is_nan());
    }

    #[test]
    fn test_invalid_parse() {
        assert!(Float::from_str("-3E2.4").is_err());
        assert!(Float::from_str("12E").is_err());
        assert!(Float::from_str("NAN").is_err());
    }

    #[test]
    fn test_to_xml() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Float::from_str(left).unwrap().text().unwrap(), right);
        }
        eq("-3E2", "-300");
        eq("4268.22752E11", "426822740000000");
        eq("+24.3e-3", "0.0243");
        eq("12", "12");
        eq("-INF", "-INF");
        eq("INF", "INF");
        eq("-0", "0");
        eq("NaN", "NaN");
    }
}
