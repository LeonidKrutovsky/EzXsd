use crate::model::complex_types::local_simple_type::LocalSimpleType;
use crate::model::elements::annotation::Annotation;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::type_::Type;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::AnyAttributes;

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
#[derive(Debug)]
pub struct TopLevelAttributeType {
    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
    pub type_: Option<Type>,
    pub default: Option<Default_>,
    pub fixed: Option<Fixed>,
}
