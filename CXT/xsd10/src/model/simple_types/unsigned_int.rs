// xsd:unsignedInt
// The type xsd:unsignedInt represents an integer between 0 and 4294967295. An xsd:unsignedInt is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:unsignedLong
// Minimum Inclusive: 0 (Defined in type xsd:nonNegativeInteger)
// Maximum Inclusive: 4294967295
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct UnsignedInt(pub u32);

impl FromStr for UnsignedInt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<u32>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<u32> for UnsignedInt {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl_display!(UnsignedInt);

#[cfg(test)]
mod test {
    use crate::model::simple_types::unsigned_int::UnsignedInt;

    #[test]
    fn test_parse() {
        assert_eq!("0".parse::<UnsignedInt>().unwrap(), 0);
        assert_eq!("4294967295".parse::<UnsignedInt>().unwrap(), 4294967295);
    }
}
