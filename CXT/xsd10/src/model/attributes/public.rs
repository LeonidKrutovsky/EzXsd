// public
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:public
// Properties: Local, Unqualified
//
// Value:
// xsd token
//
// Used in
// Anonymous type of element xsd:notation

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::Public as SimpleTypePublic;

pub struct Public(SimpleTypePublic);

impl TryFrom<RawAttribute<'_>> for Public {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Public {
    pub const NAME: &'static str = "public";
}