// xsd:long
// The type xsd:long represents an integer between
// -9223372036854775808 and 9223372036854775807.
// An xsd:long is a sequence of digits, optionally preceded by a + or - sign.
// Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:integer
// Minimum Inclusive: -9223372036854775808
// Maximum Inclusive: 9223372036854775807
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Long(pub i64);

impl FromStr for Long {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<i64>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<i64> for Long {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl ToXml for Long {
    fn to_xml(&self) -> Result<String, String> {
        Ok(self.0.to_string())
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::long::Long;

    #[test]
    fn test_parse() {
        assert_eq!(
            "-9223372036854775808".parse::<Long>().unwrap(),
            -9223372036854775808
        );
        assert_eq!(
            "9223372036854775807".parse::<Long>().unwrap(),
            9223372036854775807
        );
    }
}
