// schemaLocation
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:import Anonymous type of element xsd:include Anonymous type of element xsd:redefine

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::AnyUri;

pub struct SchemaLocation(AnyUri);

impl TryFrom<RawAttribute<'_>> for SchemaLocation {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl SchemaLocation {
    pub const NAME: &'static str = "schemaLocation";
}