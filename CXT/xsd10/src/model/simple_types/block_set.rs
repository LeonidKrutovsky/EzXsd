use std::str::FromStr;

use crate::model::simple_types::xsd_list::XsdList;
use std::fmt;

// xsd:blockSet
// #all or (possibly empty) subset of {substitution, extension, restriction}
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//              extension
//              restriction
//              substitution
//
// Used by
// Attribute block
// Attribute blockDefault
#[derive(Debug, PartialEq)]
pub enum BlockSet {
    All,
    List(XsdList<BlockSetChoice>),
}

impl FromStr for BlockSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s == "#all" {
            Self::All
        } else {
            Self::List(s.parse()?)
        };
        Ok(res)
    }
}

impl Default for BlockSet {
    fn default() -> Self {
        Self::List(XsdList(vec![]))
    }
}

impl fmt::Display for BlockSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockSet::All => write!(f, "{}", "#all"),
            BlockSet::List(x) => write!(f, "{}", x)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockSetChoice {
    Extension,
    Restriction,
    Substitution,
}

impl FromStr for BlockSetChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "substitution" => Self::Substitution,
            _ => return Err(format!("Invalid value for BlockSet: {}", s)),
        };
        Ok(res)
    }
}

impl fmt::Display for BlockSetChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockSetChoice::Extension => write!(f, "{}", "extension"),
            BlockSetChoice::Restriction => write!(f, "{}", "restriction"),
            BlockSetChoice::Substitution => write!(f, "{}", "substitution"),
        }
    }
}
