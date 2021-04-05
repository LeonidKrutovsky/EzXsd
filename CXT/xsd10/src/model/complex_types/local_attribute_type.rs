use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:localAttributeType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		from type xsd:annotated
// name	            [0..1]	xsd:NCName
// ref	            [0..1]	xsd:QName
// type	            [0..1]	xsd:QName
// use	            [0..1]	Anonymous		Default value is "optional".
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
// form	            [0..1]	xsd:formChoice
//
// Used by
// Element xsd:attribute
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localAttributeType
#[complex_type()]
pub struct LocalAttributeType {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Option<elements::LocalSimpleType>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub use_: Option<attributes::Use>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub form: Option<attributes::Form>,
}