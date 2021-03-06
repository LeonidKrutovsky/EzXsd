// ref
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Type xsd:attributeGroupRef (Element xsd:attributeGroup)
//  Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:namedGroupRef (Element xsd:group)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//
use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::QName;

pub struct Ref(QName);

impl TryFrom<RawAttribute<'_>> for Ref {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Ref {
    pub const NAME: &'static str = "ref";
}