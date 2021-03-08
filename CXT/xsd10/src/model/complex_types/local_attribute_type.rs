use crate::model::complex_types::local_simple_type::LocalSimpleType;
use crate::model::elements::annotation::Annotation;
use crate::model::RawAttribute;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::ref_::Ref;
use crate::model::attributes::type_::Type;
use crate::model::attributes::use_::Use;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::form::Form;

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
#[derive(Debug)]
pub struct LocalAttributeType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub simple_type: Option<LocalSimpleType<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<Ref>,
    pub type_: Option<Type>,
    pub use_: Option<Use>,
    pub default: Option<Default_>,
    pub fixed: Option<Fixed>,
    pub form: Option<Form>,
}