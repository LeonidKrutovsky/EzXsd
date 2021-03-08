// default
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:string
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd string
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelAttributeType (Element xsd:attribute)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)

use crate::model::simple_types::String_;
use std::convert::TryFrom;
use crate::model::RawAttribute;

pub struct Default(String_);

impl TryFrom<RawAttribute<'_>> for Default {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Default {
    pub const NAME: &'static str = "default";
}



