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
use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum NamespaceList<'a> {
    Any,
    Other,
    ListOf(Vec<TargetOrLocal<'a>>),
}

impl<'a> Default for NamespaceList<'a> {
    fn default() -> Self {
        Self::Any
    }
}

impl<'a> FromStr for NamespaceList<'a> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
}

#[derive(Debug, PartialEq)]
pub enum TargetOrLocal<'a> {
    TargetNamespace,
    Local,
    Uri(AnyUri<'a>),
}

impl<'a> FromStr for TargetOrLocal<'a> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "##targetNamespace" => Ok(Self::TargetNamespace),
            "##local" => Ok(Self::Local),
            x => Ok(Self::Uri(x.parse()?)),
        }
    }
}

impl<'a> ToXml for TargetOrLocal<'a> {
    fn to_xml(&self) -> Result<String, String> {
        Ok(match self {
            TargetOrLocal::TargetNamespace => "##targetNamespace".to_string(),
            TargetOrLocal::Local => "##local".to_string(),
            TargetOrLocal::Uri(uri) => uri.to_xml()?,
        })
    }
}

impl<'a> ToXml for NamespaceList<'a> {
    fn to_xml(&self) -> Result<String, String> {
        Ok(match self {
            NamespaceList::Any => "##any".to_string(),
            NamespaceList::Other => "##other".to_string(),
            NamespaceList::ListOf(x) => x
                .iter()
                .map(|v| v.to_xml())
                .collect::<Result<Vec<String>, String>>()?
                .into_iter()
                .fold(String::new(), |a, b| format!("{} {}", a, b)),
        })
    }
}
