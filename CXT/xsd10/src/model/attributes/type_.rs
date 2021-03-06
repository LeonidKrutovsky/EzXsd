// type
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
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
use crate::model::simple_types::QName;

pub struct Type(QName);

impl TryFrom<RawAttribute<'_>> for Type {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Type {
    pub const NAME: &'static str = "type";
}