// xsd:namespaceList
// simple type for the value of the 'namespace' attr of 'any' and 'anyAttribute'
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
// Union of:
//  Type based on xsd:token
//      Valid value	Description
//      ##any	any non-conflicting replacement at all
//      ##other	any non-conflicting replacement from a namespace other than target namespace
//  List of:
//      Union of:
//          xsd:anyURI
//          Type based on xsd:token
//              Valid value	Description
//              ##targetNamespace	any non-conflicting replacement from the target namespace
//              ##local	any unqualified (in no namespace) non-conflicting replacement
// Used by
// Attribute namespace

use crate::model::simple_types::AnyUri;
use crate::model::{Parse};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum NamespaceList {
    Any,
    Other,
    ListOf(Vec<TargetOrLocal>),
}

impl Default for NamespaceList {
    fn default() -> Self {
        Self::Any
    }
}

impl FromStr for NamespaceList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum TargetOrLocal {
    TargetNamespace,
    Local,
    Uri(AnyUri),
}

impl FromStr for TargetOrLocal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Parse for TargetOrLocal {
    fn parse(value: &str) -> Result<Self, String> where Self: Sized {
        match value {
            "##targetNamespace" => Ok(Self::TargetNamespace),
            "##local" => Ok(Self::Local),
            x => Ok(x.parse()?),
        }
    }

    fn create(value: String) -> Self where Self: Sized {
        match value.as_str() {
            "##targetNamespace" => Self::TargetNamespace,
            "##local" => Self::Local,
            x => Self::Uri(x.to_string().into()),
        }
    }

    fn text(&self) -> Result<String, String> {
        Ok(match self {
            TargetOrLocal::TargetNamespace => "##targetNamespace".to_string(),
            TargetOrLocal::Local => "##local".to_string(),
            TargetOrLocal::Uri(uri) => uri.text()?,
        })
    }
}

impl Parse for NamespaceList {
    fn parse(s: &str) -> Result<Self, String> where Self: Sized {
        match s {
            "##any" => Ok(Self::Any),
            "##other" => Ok(Self::Other),
            x => {
                let res: Result<Vec<_>, _> =
                    x.split(' ').map(|v| v.parse::<TargetOrLocal>()).collect();
                Ok(Self::ListOf(res?))
            }
        }
    }

    fn create(s: String) -> Self where Self: Sized {
        match s.as_str() {
            "##any" => Self::Any,
            "##other" => Self::Other,
            x => {
                let res = x.split(' ').map(|v| TargetOrLocal::create(v.to_string())).collect();
                Self::ListOf(res)
            }
        }
    }

    fn text(&self) -> Result<String, String> {
        Ok(match self {
            NamespaceList::Any => "##any".to_string(),
            NamespaceList::Other => "##other".to_string(),
            NamespaceList::ListOf(x) => x
                .iter()
                .map(|v| v.text())
                .collect::<Result<Vec<String>, String>>()?
                .into_iter()
                .fold(String::new(), |a, b| format!("{} {}", a, b)),
        })
    }
}
