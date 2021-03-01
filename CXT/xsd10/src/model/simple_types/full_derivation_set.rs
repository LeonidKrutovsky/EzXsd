use crate::model::ToXml;
use std::str::FromStr;

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
    List(Vec<FullDerivationSubSet>),
}

#[derive(Debug, PartialEq)]
pub enum FullDerivationSubSet {
    Extension,
    Restriction,
    List,
    Union,
}

impl FullDerivationSubSet {
    pub fn parse(s: &str) -> Result<Self, String> {
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

impl FullDerivationSet {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = if s == "#all" {
            Self::All
        } else {
            let res: Result<Vec<_>, String> = s
                .split(' ')
                .map(|v| FullDerivationSubSet::parse(v))
                .collect();
            Self::List(res?)
        };
        Ok(res)
    }
}

impl ToXml for FullDerivationSubSet {
    fn to_xml(&self) -> Result<String, String> {
        let res = match self {
            Self::Extension => "extension",
            Self::Restriction => "restriction",
            Self::List => "list",
            Self::Union => "union",
        };
        Ok(res.into())
    }
}

impl FromStr for FullDerivationSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s == "#all" {
            Self::All
        } else {
            let res: Result<Vec<_>, String> = s
                .split(' ')
                .map(|v| FullDerivationSubSet::parse(v))
                .collect();
            Self::List(res?)
        };
        Ok(res)
    }
}

impl ToXml for FullDerivationSet {
    fn to_xml(&self) -> Result<String, String> {
        let res = match self {
            Self::All => "#all".to_string(),
            Self::List(x) => x
                .iter()
                .map(|x| x.to_xml())
                .collect::<Result<Vec<String>, String>>()?
                .into_iter()
                .fold(String::new(), |a, b| format!("{} {}", a, b)),
        };
        Ok(res)
    }
}
