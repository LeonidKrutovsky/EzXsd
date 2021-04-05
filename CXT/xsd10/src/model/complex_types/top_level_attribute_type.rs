use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:topLevelAttributeType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
// type	            [0..1]	xsd:QName
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
//
// Used by
// Element xsd:attribute
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelAttributeType
#[complex_type()]
pub struct TopLevelAttributeType {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Option<elements::TopLevelSimpleType>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub type_: Option<attributes::Type>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
}
