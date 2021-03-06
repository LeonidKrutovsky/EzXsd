// version
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:token
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd token
//
// Used in
// Anonymous type of element xsd:schema

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::Token;

pub struct Version(Token);

impl TryFrom<RawAttribute<'_>> for Version {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Version {
    pub const NAME: &'static str = "version";
}