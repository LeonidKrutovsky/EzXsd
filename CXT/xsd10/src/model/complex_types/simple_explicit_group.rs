use crate::model::elements::annotation::Annotation;
use crate::model::groups::nested_particle::NestedParticle;
use crate::model::simple_types::Id;
use crate::model::RawAttribute;

// xsd:simpleExplicitGroup
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [0..*]           from group xsd:nestedParticle
//          xsd:element
//          xsd:group
//          xsd:choice
//          xsd:sequence
//          xsd:any
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
//
// Used by
// Element xsd:choice
// Element xsd:sequence
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleExplicitGroup
#[derive(Debug, Default)]
pub struct SimpleExplicitGroup<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub nested_particle: Vec<NestedParticle<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
}
