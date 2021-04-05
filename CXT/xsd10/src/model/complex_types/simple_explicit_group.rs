use crate::model::groups;
use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

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
#[complex_type()]
pub struct SimpleExplicitGroup {
    pub annotation: Option<elements::Annotation>,
    pub nested_particle: Vec<groups::NestedParticle>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
}
