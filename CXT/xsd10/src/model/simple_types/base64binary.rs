// xsd:base64Binary
// The type xsd:base64Binary represents binary data as a sequence of binary octets.
// It uses base64 encoding, as described in RFC 2045.The following rules apply to
// xsd:base64Binary values:
// The following characters are allowed: the letters A to Z (upper and lower case),
// digits 0 through 9, the plus sign ("+"), the slash ("/"), the equals sign ("=")
// and XML whitespace characters.
// XML whitespace characters may appear anywhere in the value.
// The number of non-whitespace characters must be divisible by 4.
// Equals signs may only appear at the end of the value, and there may be zero,
// one or two of them. If there are two equals signs, they must be preceded by one
// of the following characters: AQgw. If there is only one equals sign, it must be preceded
// by one of the following characters: AEIMQUYcgkosw048. In either case, there may
// be whitespace in between the necessary characters and the equals sign(s).
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:base64Binary

use crate::model::simple_types::any_simple_type::AnySimpleType;
use crate::model::simple_types::white_space_facet::collapse;
use crate::model::ToXml;
use std::borrow::{Borrow, Cow};

#[derive(Debug, PartialEq)]
pub struct Base64Binary<'a>(AnySimpleType<'a>);

//TODO: add base64 crate and validate strings

impl<'a, T> From<T> for Base64Binary<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self { 0: value.into() }
    }
}

impl<'a> ToXml for Base64Binary<'a> {
    fn to_xml(&self) -> Result<String, String> {
        Ok(collapse(self.0.borrow()))
    }

    fn raw(&self) -> &str {
        self.0.borrow()
    }
}
