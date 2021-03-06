// refer
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Anonymous type of element xsd:keyref

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::QName;

pub struct Refer(QName);

impl TryFrom<RawAttribute<'_>> for Refer {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Refer {
    pub const NAME: &'static str = "refer";
}