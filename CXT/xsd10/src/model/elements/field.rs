use crate::model::elements::annotation::Annotation;
use crate::model::attributes::id::Id;
use crate::model::attributes::xpath::FieldXPath;
use crate::model::attributes::AnyAttributes;

// xsd:field
// See http://www.w3.org/TR/xmlschema-1/#element-field.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]       from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// xpath	        [1..1]	Anonymous
//
// Used in
// Anonymous type of element xsd:keyref via extension of xsd:keybase
// Type xsd:keybase (Elements xsd:unique, xsd:key)
#[derive(Debug, Default)]
pub struct Field {
    pub annotation: Option<Annotation>,
    pub attributes: AnyAttributes,
    pub id:  Option<Id>,
    pub xpath: FieldXPath,
}
