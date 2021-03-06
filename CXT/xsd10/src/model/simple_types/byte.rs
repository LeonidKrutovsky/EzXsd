// xsd:byte
// The type xsd:byte represents an integer between -128 and 127. An xsd:byte is a sequence of digits, optionally preceded by a + or - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:short
// Minimum Inclusive: -128
// Maximum Inclusive: 127
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Byte(pub i8);

impl FromStr for Byte {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<i8>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<i8> for Byte {
    fn eq(&self, other: &i8) -> bool {
        self.0 == *other
    }
}


#[cfg(test)]
mod tests {
    use super::Byte;

    #[test]
    fn test_valid_values() {
        assert_eq!("+3".parse::<Byte>().unwrap(), 3);
        assert_eq!("122".parse::<Byte>().unwrap(), 122);
        assert_eq!("0".parse::<Byte>().unwrap(), 0);
        assert_eq!("-123".parse::<Byte>().unwrap(), -123);
    }

    #[test]
    fn test_invalid_values() {
        assert!("130".parse::<Byte>().is_err());
        assert!("3.0".parse::<Byte>().is_err());
        assert!("".parse::<Byte>().is_err());
    }
}
