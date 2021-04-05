use crate::model::elements;
use crate::model::attributes;
use xml_utils::element;


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
#[element(name = "notation")]
pub struct Notation {
    pub annotation: Option<elements::Annotation>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub public: Option<attributes::Public>,
    pub system: Option<attributes::System>,
}
