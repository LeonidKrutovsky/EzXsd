use crate::model::complex_types::local_element;
use crate::model::complex_types::narrow_max_min::NarrowMaxMin;
use crate::model::complex_types::top_level_element;
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
pub struct TopLevelElement(pub top_level_element::TopLevelElement);


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
pub struct LocalElement(pub local_element::LocalElement);

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
pub struct Element(pub NarrowMaxMin);
