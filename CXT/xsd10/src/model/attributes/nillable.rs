// nillable
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd boolean
//
// Used in
// Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::Boolean;

pub struct Nillable(Boolean);

impl TryFrom<RawAttribute<'_>> for Nillable {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Nillable {
    pub const NAME: &'static str = "nillable";
}