// xsd:NMTOKENS
// The type xsd:NMTOKENS represents a list of NMTOKEN values separated by whitespace.
// There must be at least one NMTOKEN in the list.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:NMTOKEN
// Minimum Length: 1
// White Space: collapse

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS

use crate::model::simple_types::nmtoken::NmToken;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug)]
pub struct NmTokens<'a>(Vec<NmToken<'a>>);

impl<'a, T> From<T> for NmTokens<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: value
                .into()
                .split_whitespace()
                .map(|v| NmToken::from(v.to_string()))
                .collect(),
        }
    }
}

impl<'a> ToXml for NmTokens<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let result = self
            .0
            .iter()
            .map(|x| x.to_xml())
            .collect::<Result<Vec<String>, String>>()?
            .into_iter()
            .fold(String::new(), |a, b| format!("{} {}", a, b));

        if result.is_empty() {
            Err(format!("There must be at least one NMTOKEN in the list."))
        } else {
            Ok(result)
        }
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}
