use std::fmt;
use std::str::FromStr;

use crate::model::simple_types::xsd_list::XsdList;

// xsd:fullDerivationSet
// #all or (possibly empty) subset of {extension, restriction, list, union}
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
//              Valid value	Description
//              extension	    Extension is disallowed
//              restriction	    Restriction is disallowed
//              list	        Derivation by list is disallowed
//              union	        Derivation by union is disallowed
//
// Used by
// Attribute finalDefault
#[derive(Debug, PartialEq)]
pub enum FullDerivationSet {
    All,
    List(XsdList<FullDerivationSubSet>),
}

impl FromStr for FullDerivationSet {
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

impl fmt::Display for FullDerivationSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FullDerivationSet::All => write!(f, "{}", "#all"),
            FullDerivationSet::List(x) => {
                let res =
                    x.0.iter()
                        .fold(String::new(), |a, b| format!("{} {}", a, b));
                write!(f, "{}", res)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FullDerivationSubSet {
    Extension,
    Restriction,
    List,
    Union,
}

impl FromStr for FullDerivationSubSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "list" => Self::List,
            "union" => Self::Union,
            _ => return Err(format!("Invalid value for FullDerivationSet: {}", s)),
        };
        Ok(res)
    }
}

impl fmt::Display for FullDerivationSubSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FullDerivationSubSet::Extension => write!(f, "{}", "extension"),
            FullDerivationSubSet::Restriction => write!(f, "{}", "restriction"),
            FullDerivationSubSet::List => write!(f, "{}", "list"),
            FullDerivationSubSet::Union => write!(f, "{}", "union"),
        }
    }
}
