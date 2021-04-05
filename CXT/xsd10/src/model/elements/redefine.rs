use crate::model::elements;
use crate::model::attributes;
use crate::model::groups;
use xml_utils::element;

// xsd:redefine
// See http://www.w3.org/TR/xmlschema-1/#element-redefine.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Choice [0..*]
//      xsd:annotation
//         from group xsd:redefinable
//              xsd:simpleType
//              xsd:complexType
//              xsd:group
//              xsd:attributeGroup
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// schemaLocation	[1..1]	xsd:anyURI
// id	            [0..1]	xsd:ID
//
// Used in
// Anonymous type of element xsd:schema
#[element(name = "redefine")]
pub struct Redefine {
    pub annotations: Vec<elements::Annotation>,
    pub content: Vec<groups::Redefinable>,
    pub attributes: attributes::AnyAttributes,
    pub schema_location: attributes::SchemaLocation,
    pub id: Option<attributes::Id>,
}
