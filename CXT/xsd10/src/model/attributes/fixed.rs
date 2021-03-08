// fixed
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
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::String_;

#[derive(Debug)]
pub struct Fixed(String_);

impl TryFrom<&RawAttribute<'_>> for Fixed {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Fixed {
    pub const NAME: &'static str = "fixed";
}