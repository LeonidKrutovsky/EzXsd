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

pub type Float = f32;

#[cfg(test)]
mod test {
    use crate::model::simple_types::float::Float;
    use std::num::ParseFloatError;

    #[test]
    fn test_valid_parse() {
        assert_eq!("-3E2".parse::<Float>().unwrap(), -300.0);
        assert_eq!("4268.22752E11".parse::<Float>().unwrap(), 4268.22752E11);
        assert_eq!("+24.3e-3".parse::<Float>().unwrap(), 0.0243);
        assert_eq!("12".parse::<Float>().unwrap(), 12.0);
        //assert_eq!("-INF".parse::<Float>().unwrap(), f32::neg_infinity());
        assert_eq!("-0".parse::<Float>().unwrap(), 0.0);
        assert_eq!("NaN".parse::<Float>().unwrap().to_string(), "NaN");
    }

    #[test]
    fn test_invalid_parse() {
        assert!("-3E2.4".parse::<Float>().is_err());
        assert!("12E".parse::<Float>().is_err());
        assert!("NAN".parse::<Float>().is_err());
    }
}
