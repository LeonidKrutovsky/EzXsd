use crate::model::elements::annotation::Annotation;
use crate::model::attributes::id::Id;
use crate::model::attributes::schema_location::SchemaLocation;
use crate::model::attributes::AnyAttributes;

// xsd:include
// See http://www.w3.org/TR/xmlschema-1/#element-include.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// schemaLocation	[1..1]	xsd:anyURI
//
// Used in
// Anonymous type of element xsd:schema
#[derive(Debug, Default)]
pub struct Include {
    pub annotation: Option<Annotation>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub schema_location: SchemaLocation,
}
