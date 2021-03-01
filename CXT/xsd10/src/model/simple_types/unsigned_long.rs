// xsd:unsignedLong
// The type xsd:unsignedLong represents an integer between 0 and 18446744073709551615. An xsd:unsignedLong is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:nonNegativeInteger
// Minimum Inclusive: 0 (Defined in type xsd:nonNegativeInteger)
// Maximum Inclusive: 18446744073709551615
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct UnsignedLong(pub u64);

impl FromStr for UnsignedLong {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<u64>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<u64> for UnsignedLong {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl ToXml for UnsignedLong {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::unsigned_long::UnsignedLong;

    #[test]
    fn test_parse() {
        assert_eq!("0".parse::<UnsignedLong>().unwrap(), 0);
        assert_eq!(
            "18446744073709551615".parse::<UnsignedLong>().unwrap(),
            18446744073709551615
        );
    }
}
