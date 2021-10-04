use crate::model::{attributes, elements, groups};
use xml_utils::element;

// See http://www.w3.org/TR/xmlschema-1/#element-group.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:namedGroup
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
#[element(name = "group")]
pub struct Group {
    pub annotation: Option<elements::Annotation>,
    pub content_choice: groups::ContentChoice,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
}

// xsd:group
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:group
// Type: xsd:namedGroupRef
// Properties: Local, Qualified
//
// Used in
// Group xsd:nestedParticle
// Group xsd:typeDefParticle
// Group xsd:complexTypeModel via reference to xsd:typeDefParticle
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:typeDefParticle (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:typeDefParticle (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[element(name = "group")]
pub struct GroupRef {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub ref_: attributes::Ref,
    #[default]
    pub min_occurs: attributes::MinOccurs,
    #[default]
    pub max_occurs: attributes::MaxOccurs,
}
