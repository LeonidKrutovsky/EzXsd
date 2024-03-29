use crate::model::attributes;
use crate::model::elements;

use xml_utils::complex_type;

// xsd:attributeGroupRef
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// ref	            [1..1]	xsd:QName
//
// Used by
// Element xsd:attributeGroup
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:attributeGroupRef
#[complex_type()]
pub struct AttributeGroupRef {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub ref_: attributes::Ref,
}
