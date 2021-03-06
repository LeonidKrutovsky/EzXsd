// xsd:unsignedByte
// The type xsd:unsignedByte represents an integer between 0 and 255. An xsd:unsignedByte is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:unsignedShort
// Minimum Inclusive: 0 (Defined in type xsd:nonNegativeInteger)
// Maximum Inclusive: 255
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct UnsignedByte(pub u8);

impl FromStr for UnsignedByte {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<u8>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<u8> for UnsignedByte {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl_display!(UnsignedByte);

#[cfg(test)]
mod test {
    use crate::model::simple_types::unsigned_byte::UnsignedByte;

    #[test]
    fn test_parse() {
        assert_eq!("0".parse::<UnsignedByte>().unwrap(), 0);
        assert_eq!("255".parse::<UnsignedByte>().unwrap(), 255);
    }
}
