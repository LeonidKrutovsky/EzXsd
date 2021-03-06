use std::str::FromStr;

use crate::model::simple_types::xsd_list::XsdList;

// xsd:derivationSet
// #all or (possibly empty) subset of {extension, restriction}
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
//              Valid value	    Description
//              extension	    Extension is disallowed
//              restriction	    Restriction is disallowed
//
// Used by
// Attribute block
// Attribute final
#[derive(Debug)]
pub enum DerivationSet {
    All,
    List(XsdList<DerivationSubset>),
}

impl FromStr for DerivationSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "#all" => Self::All,
            _ => {
                Self::List(s.parse()?)
            }
        })
    }
}

#[derive(Debug)]
pub enum DerivationSubset {
    Extension,
    Restriction,
}

impl FromStr for DerivationSubset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extension" => Ok(Self::Extension),
            "restriction" => Ok(Self::Restriction),
            _ => Err(format!("Invalid value for xsd:derivationSet type: {}", s)),
        }
    }
}
