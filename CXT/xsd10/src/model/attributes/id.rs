// id
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:ID
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd ID
//
// Used in
// Anonymous type of element xsd:annotation
// Anonymous type of element xsd:redefine
// Anonymous type of element xsd:schema
// Type xsd:annotated
// Anonymous type of element xsd:all via derivation of xsd:annotated
// Anonymous type of element xsd:any via derivation of xsd:annotated
// Anonymous type of element xsd:complexContent via derivation of xsd:annotated
// Anonymous type of element xsd:field via derivation of xsd:annotated
// Anonymous type of element xsd:import via derivation of xsd:annotated
// Anonymous type of element xsd:include via derivation of xsd:annotated
// Anonymous type of element xsd:keyref via derivation of xsd:annotated
// Anonymous type of element xsd:list via derivation of xsd:annotated
// Anonymous type of element xsd:notation via derivation of xsd:annotated
// Anonymous type of element xsd:pattern via derivation of xsd:annotated
// Anonymous type of element xsd:restriction via derivation of xsd:annotated
// Anonymous type of element xsd:selector via derivation of xsd:annotated
// Anonymous type of element xsd:simpleContent via derivation of xsd:annotated
// Anonymous type of element xsd:totalDigits via derivation of xsd:annotated
// Anonymous type of element xsd:union via derivation of xsd:annotated
// Anonymous type of element xsd:whiteSpace via derivation of xsd:annotated
// Type xsd:complexType via derivation of xsd:annotated
// Type xsd:simpleType via derivation of xsd:annotated
// Type xsd:allType via derivation of xsd:annotated (Element xsd:all)
// Type xsd:attributeGroupRef via derivation of xsd:annotated (Element xsd:attributeGroup)
// Type xsd:complexRestrictionType via derivation of xsd:annotated (Element xsd:restriction)
// Type xsd:extensionType via derivation of xsd:annotated (Element xsd:extension)
// Type xsd:localAttributeType via derivation of xsd:annotated (Element xsd:attribute)
// Type xsd:localComplexType via derivation of xsd:annotated (Element xsd:complexType)
// Type xsd:localElement via derivation of xsd:annotated (Element xsd:element)
// Type xsd:localSimpleType via derivation of xsd:annotated (Element xsd:simpleType)
// Type xsd:namedAttributeGroup via derivation of xsd:annotated (Element xsd:attributeGroup)
// Type xsd:namedGroup via derivation of xsd:annotated (Element xsd:group)
// Type xsd:namedGroupRef via derivation of xsd:annotated (Element xsd:group)
// Type xsd:narrowMaxMin via derivation of xsd:annotated (Element xsd:element)
// Type xsd:noFixedFacet via derivation of xsd:annotated (Element xsd:enumeration)
// Type xsd:simpleExtensionType via derivation of xsd:annotated (Element xsd:extension)
// Type xsd:simpleRestrictionType via derivation of xsd:annotated (Element xsd:restriction)
// Type xsd:topLevelAttributeType via derivation of xsd:annotated (Element xsd:attribute)
// Type xsd:topLevelComplexType via derivation of xsd:annotated (Element xsd:complexType)
// Type xsd:topLevelElement via derivation of xsd:annotated (Element xsd:element)
// Type xsd:topLevelSimpleType via derivation of xsd:annotated (Element xsd:simpleType)
// Type xsd:wildcard via derivation of xsd:annotated (Element xsd:anyAttribute)
// Type xsd:explicitGroup via derivation of xsd:annotated (Elements xsd:choice , xsd:sequence)
// Type xsd:keybase via derivation of xsd:annotated (Elements xsd:unique , xsd:key)
// Type xsd:simpleExplicitGroup via derivation of xsd:annotated (Elements xsd:choice , xsd:sequence)
// Type xsd:facet via derivation of xsd:annotated (Elements xsd:minExclusive , xsd:minInclusive , xsd:maxExclusive , xsd:maxInclusive)
// Type xsd:numFacet via derivation of xsd:annotated (Elements xsd:fractionDigits , xsd:length , xsd:minLength , xsd:maxLength)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::id::Id as SimpleTypeId;

pub struct Id(SimpleTypeId);

impl TryFrom<RawAttribute<'_>> for Id {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Id {
    pub const NAME: &'static str = "id";
}