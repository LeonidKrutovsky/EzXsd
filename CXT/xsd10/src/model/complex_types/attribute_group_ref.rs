use crate::model::elements::annotation::Annotation;
use crate::model::simple_types::qname::QName;
use crate::model::simple_types::Id;
use crate::model::RawAttribute;

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
#[derive(Debug)]
pub struct AttributeGroupRef<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id,
    pub name: QName,
}
