// name
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:NCName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd NCName
//
// Used in
// Anonymous type of element xsd:notation Anonymous type of element xsd:keyref via derivation of xsd:keybase Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:namedAttributeGroup (Element xsd:attributeGroup)
//  Type xsd:namedGroup (Element xsd:group)
//  Type xsd:topLevelAttributeType (Element xsd:attribute)
//  Type xsd:topLevelComplexType (Element xsd:complexType)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:keybase (Elements xsd:unique , xsd:key)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::NCName;

pub struct Name(NCName);

impl TryFrom<RawAttribute<'_>> for Name {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Name {
    pub const NAME: &'static str = "name";
}