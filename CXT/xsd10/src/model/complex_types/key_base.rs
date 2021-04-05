use crate::model::elements::annotation::Annotation;
use crate::model::elements::field::Field;
use crate::model::elements::selector::Selector;
use crate::model::attributes::name::Name;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

// xsd:keybase
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      xsd:selector [1..1]
//      xsd:field [1..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used by
// Element xsd:key
// Element xsd:unique
// Element xsd:keyref via derived anonymous type
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:keybase
// extended by Anonymous type of element xsd:keyref
#[derive(Debug, Default)]
pub struct KeyBase {
    pub annotation: Option<Annotation>,
    pub selector: Selector,
    pub fields: Vec<Field>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
}
