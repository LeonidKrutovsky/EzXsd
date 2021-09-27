use crate::model::complex_types::explicit_group::ExplicitGroup;
use crate::model::complex_types::simple_explicit_group::SimpleExplicitGroup;
use xml_utils::element;
use crate::model::{elements, groups, attributes};

// xsd:sequence
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExplicitGroup
// Properties: Local, Qualified
//
// Used in
// Type xsd:namedGroup (Element xsd:group)
#[element(name = "sequence")]
pub struct SimpleSequence {
    pub annotation: Option<elements::Annotation>,
    pub nested_particle: Vec<groups::NestedParticle>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
}


// xsd:sequence
// See http://www.w3.org/TR/xmlschema-1/#element-sequence.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:sequence
// Type: xsd:explicitGroup
// Properties: Global, Qualified
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
#[element(name = "sequence")]
pub struct Sequence(pub ExplicitGroup);
