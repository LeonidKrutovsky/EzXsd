// xsd:hexBinary
// The xsd:hexBinary type represents binary data as a sequence of binary octets. It uses hexadecimal encoding, where each binary octet is a two-character hexadecimal number. Lowercase and uppercase letters A through F are permitted. For example, 0FB8 and 0fb8 are two equal xsd:hexBinary representations consisting of two octets.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse

// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:hexBinary


use crate::model::simple_types::any_simple_type::AnySimpleType;
use std::borrow::Cow;
use crate::model::ToXml;
use crate::model::simple_types::white_space_facet::collapse;

pub struct HexBinary<'a>(AnySimpleType<'a>);

impl<'a, T> From<T> for HexBinary<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: value.into(),
        }
    }
}

impl<'a> ToXml for HexBinary<'a> {
    fn to_xml(&self) -> Result<String, String> {
        Ok(collapse(self.0.to_xml()?.as_str()))
    }

    fn raw(&self) -> &str {
        self.0.raw()
    }
}