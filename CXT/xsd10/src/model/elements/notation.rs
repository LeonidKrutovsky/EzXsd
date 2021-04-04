use crate::model::elements::annotation::Annotation;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::public::Public;
use crate::model::attributes::system::System;
use crate::model::attributes::AnyAttributes;

// xsd:notation
// See http://www.w3.org/TR/xmlschema-1/#element-notation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]   from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
// public	        [0..1]	xsd:public
// system	        [0..1]	xsd:anyURI
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
#[derive(Debug, Default)]
pub struct Notation {
    pub annotation: Option<Annotation>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
    pub public: Option<Public>,
    pub system: Option<System>,
}
