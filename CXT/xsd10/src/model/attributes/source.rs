// source
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:appinfo Anonymous type of element xsd:documentation

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::AnyUri;

pub struct Source(AnyUri);

impl TryFrom<RawAttribute<'_>> for Source {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Source {
    pub const NAME: &'static str = "source";
}