use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:import
// See http://www.w3.org/TR/xmlschema-1/#element-import.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Type: Anonymous
//
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]      from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// namespace	    [0..1]	xsd:anyURI
// schemaLocation	[0..1]	xsd:anyURI
//
// Used in
// Anonymous type of element xsd:schema
#[element(name = "import")]
pub struct Import {
    pub annotation: Option<elements::Annotation>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub namespace: Option<attributes::NamespaceUri>,
    pub schema_location: Option<attributes::SchemaLocation>,
}
