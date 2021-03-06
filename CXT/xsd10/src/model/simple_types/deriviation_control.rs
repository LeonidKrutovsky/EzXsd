// xsd:derivationControl
//    A utility type, not for public use
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema

// Schema document: datatypes.xsd

// Content
// Type based on xsd:NMTOKEN
// Valid value
// substitution
// extension
// restriction
// list
// union

// Type inheritance chain
//  xsd:anySimpleType
//      xsd:string
//          xsd:normalizedString
//              xsd:token
//                  xsd:NMTOKEN
//                      xsd:derivationControl
//                          restricted by within xsd:simpleDerivationSet

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum DeriviationControl {
    Substitution,
    Extension,
    Restriction,
    List,
    Union,
}

impl FromStr for DeriviationControl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "substitution" => Self::Substitution,
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "list" => Self::List,
            "union" => Self::Union,
            _ => return Err(format!("Invalid value for DeriviationControl: {}", s)),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::DeriviationControl;

    #[test]
    fn test_parse() {
        assert_eq!(
            DeriviationControl::from_str("substitution").unwrap(),
            DeriviationControl::Substitution
        );
        assert_eq!(
            DeriviationControl::from_str("extension").unwrap(),
            DeriviationControl::Extension
        );
        assert_eq!(
            DeriviationControl::from_str("restriction").unwrap(),
            DeriviationControl::Restriction
        );
        assert_eq!(
            DeriviationControl::from_str("list").unwrap(),
            DeriviationControl::List
        );
        assert_eq!(
            DeriviationControl::from_str("union").unwrap(),
            DeriviationControl::Union
        );
        assert!(DeriviationControl::from_str("").is_err());
    }
}
