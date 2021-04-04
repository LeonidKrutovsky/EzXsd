use crate::model::complex_types::named_group_ref::NamedGroupRef;
use crate::model::elements::any::Any;
use crate::model::elements::choice::Choice;
use crate::model::elements::element::LocalElement;
use crate::model::elements::sequence::Sequence;

// xsd:nestedParticle
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:element
//      xsd:group
//      xsd:choice
//      xsd:sequence
//      xsd:any
//
// Used in
// Type xsd:explicitGroup (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup (Elements xsd:choice, xsd:sequence)
#[derive(Debug)]
pub enum NestedParticle {
    Element(Box<LocalElement>),
    Group(NamedGroupRef),
    Choice(Choice),
    Sequence(Sequence),
    Any(Any),
}
