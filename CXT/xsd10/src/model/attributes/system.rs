// system
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:notation

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::AnyUri;

pub struct System(AnyUri);

impl TryFrom<RawAttribute<'_>> for System {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl System {
    pub const NAME: &'static str = "system";
}