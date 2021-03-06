// xsd:int
// The type xsd:int represents an integer between -2147483648 and 2147483647. An xsd:int is a sequence of digits, optionally preceded by a + or - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:long
// Minimum Inclusive: -2147483648
// Maximum Inclusive: 2147483647
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Int(pub i32);

impl FromStr for Int {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<i32>().map_err(|e| e.to_string())?))
    }
}

impl PartialEq<i32> for Int {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::int::Int;

    #[test]
    fn test_parse() {
        assert_eq!("-2147483648".parse::<Int>().unwrap(), -2147483648);
        assert_eq!("2147483647".parse::<Int>().unwrap(), 2147483647);
    }
}
