// xsd:unsignedShort
// The type xsd:unsignedShort represents an integer between 0 and 65535. An xsd:unsignedShort is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:unsignedInt
// Minimum Inclusive: 0 (Defined in type xsd:nonNegativeInteger)
// Maximum Inclusive: 65535
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct UnsignedShort(pub u16);

impl FromStr for UnsignedShort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<u16>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<u16> for UnsignedShort {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl ToXml for UnsignedShort {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::unsigned_short::UnsignedShort;

    #[test]
    fn test_parse() {
        assert_eq!("0".parse::<UnsignedShort>().unwrap(), 0);
        assert_eq!("65535".parse::<UnsignedShort>().unwrap(), 65535);
    }
}
