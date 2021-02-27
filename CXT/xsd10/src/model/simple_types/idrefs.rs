// xsd:IDREFS
// The type xsd:IDREFS represents a list of IDREF values separated by whitespace.
// There must be at least one IDREF in the list. Each of the IDREF values must
// match an ID contained in the same XML document.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:IDREF
// Minimum Length: 1
// White Space: collapse
// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:string
//      restricted by xsd:normalizedString
//        restricted by xsd:token
//          restricted by xsd:Name
//            restricted by xsd:NCName
//              restricted by xsd:IDREF
//                used in list xsd:IDREFS

use crate::model::simple_types::idref::IdRef;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug)]
pub struct IdRefs<'a>(Vec<IdRef<'a>>);

impl<'a, T> From<T> for IdRefs<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: value
                .into()
                .split_whitespace()
                .map(|v| IdRef::from(v.to_string()))
                .collect(),
        }
    }
}

impl<'a> ToXml for IdRefs<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let result = self
            .0
            .iter()
            .map(|x| x.to_xml())
            .collect::<Result<Vec<String>, String>>()?
            .into_iter()
            .fold(String::new(), |a, b| format!("{} {}", a, b));

        if result.is_empty() {
            Err(format!("There must be at least one IDREF in the list"))
        } else {
            Ok(result)
        }
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}
