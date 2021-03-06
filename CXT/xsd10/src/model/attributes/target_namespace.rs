// targetNamespace
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:schema
use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::AnyUri;

pub struct TargetNamespace(AnyUri);

impl TryFrom<RawAttribute<'_>> for TargetNamespace {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl TargetNamespace {
    pub const NAME: &'static str = "targetNamespace";
}