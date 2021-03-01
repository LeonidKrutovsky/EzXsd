// xsd:short
// The type xsd:short represents an integer between -32768 and 32767.
// An xsd:short is a sequence of digits, optionally preceded by a + or - sign.
// Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:int
// Minimum Inclusive: -32768
// Maximum Inclusive: 32767
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Short(pub i16);

impl FromStr for Short {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<i16>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<i16> for Short {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

impl ToXml for Short {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::short::Short;

    #[test]
    fn test_parse() {
        assert_eq!("-32768".parse::<Short>().unwrap(), -32768);
        assert_eq!("32767".parse::<Short>().unwrap(), 32767);
    }
}
