use crate::model::{attributes, elements, groups};
use xml_utils::element;

// xsd:element
// See http://www.w3.org/TR/xmlschema-1/#element-element.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelElement
// Properties: Global, Qualified
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
#[element(name = "element")]
pub struct TopLevelElement {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub type_: Option<attributes::Type>,
    pub substitution_group: Option<attributes::SubstitutionGroup>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub abstract_: attributes::Abstract,
    pub final_: Option<attributes::Final>,
    pub block: Option<attributes::Block>,
}

// xsd:element
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localElement
// Properties: Local, Qualified
//
// Used in
// Group xsd:nestedParticle
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[element(name = "element")]
pub struct LocalElement {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub min_occurs: attributes::MinOccurs,
    pub max_occurs: attributes::MaxOccurs,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub block: Option<attributes::Block>,
    pub form: Option<attributes::Form>,
}

// xsd:element
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:element, xsd:element
// Type: xsd:narrowMaxMin
// Properties: Local, Qualified
//
// Used in
// Group xsd:allModel
// Anonymous type of element xsd:all via reference to xsd:allModel
// Type xsd:allType via reference to xsd:allModel (Element xsd:all)
#[element(name = "element")]
pub struct Element {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub block: Option<attributes::Block>,
    pub form: Option<attributes::Form>,
    pub min_occurs: attributes::MinOccursBool,
    pub max_occurs: attributes::MaxOccursBool,
    pub attributes: Vec<attributes::RawAttribute>,
}
