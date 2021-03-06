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

use std::fmt;
use std::str::FromStr;

use crate::model::simple_types::AnyUri;
use crate::model::simple_types::xsd_list::XsdList;

#[derive(Debug, PartialEq)]
pub enum NamespaceList {
    Any,
    Other,
    ListOf(XsdList<TargetOrLocal>),
}

impl Default for NamespaceList {
    fn default() -> Self {
        Self::Any
    }
}

impl FromStr for NamespaceList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "##any" => Ok(Self::Any),
            "##other" => Ok(Self::Other),
            x => {
                Ok(Self::ListOf(x.parse()?))
            }
        }
    }
}

impl fmt::Display for NamespaceList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamespaceList::Any => write!(f, "{}", "##any"),
            NamespaceList::Other => write!(f, "{}", "##other"),
            NamespaceList::ListOf(x) => {
                let res = x.0
                    .iter()
                    .fold(String::new(), |a, b| format!("{} {}", a, b));
                write!(f, "{}", res)
            }
            }
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
        match s {
            "##targetNamespace" => Ok(Self::TargetNamespace),
            "##local" => Ok(Self::Local),
            x => Ok(Self::Uri(x.parse()?)),
        }
    }
}

impl fmt::Display for TargetOrLocal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetOrLocal::TargetNamespace => write!(f, "{}", "##targetNamespace"),
            TargetOrLocal::Local => write!(f, "{}", "##local"),
            TargetOrLocal::Uri(uri) => write!(f, "{}", uri.as_ref()),
        }
    }
}