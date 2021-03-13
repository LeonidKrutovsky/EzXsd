use std::str::FromStr;

use crate::model::simple_types::xsd_list::XsdList;

// xsd:simpleDerivationSet
//    #all or (possibly empty) subset of {restriction, union, list}
//
//    A utility type, not for public use
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//                  list
//                  union
//                  restriction
//
// Used by
// Attribute final
#[derive(Debug, PartialEq)]
pub enum SimpleDerivationSet {
    All,
    List(XsdList<SimpleDerivationSubset>),
}

impl Default for SimpleDerivationSet{
    fn default() -> Self {
        Self::List(XsdList(vec![]))
    }
}

impl FromStr for SimpleDerivationSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "#all" => Self::All,
            _ => Self::List(s.parse()?),
        };
        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
pub enum SimpleDerivationSubset {
    List,
    Union,
    Restriction,
}

impl FromStr for SimpleDerivationSubset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "list" => Self::List,
            "union" => Self::Union,
            "restriction" => Self::Restriction,
            _ => return Err(format!("Invalid value for SimpleDerivationSubset: {}", s)),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::xsd_list::XsdList;

    use super::SimpleDerivationSet;
    use super::SimpleDerivationSubset::*;
    use std::str::FromStr;

    #[test]
    fn test() {
        assert_eq!(
            SimpleDerivationSet::from_str("#all").unwrap(),
            SimpleDerivationSet::All
        );
        assert_eq!(
            SimpleDerivationSet::from_str("list union restriction").unwrap(),
            SimpleDerivationSet::List(XsdList(vec![List, Union, Restriction]))
        );
        assert!(SimpleDerivationSet::from_str("val").is_err());
    }
}
