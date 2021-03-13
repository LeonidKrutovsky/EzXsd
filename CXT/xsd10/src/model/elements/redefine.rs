use crate::model::elements::annotation::Annotation;
use crate::model::groups::redefinable::Redefinable;
use crate::model::attributes::schema_location::SchemaLocation;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

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
//              from group xsd:redefinable
//      xsd:simpleType
//      xsd:complexType
//      xsd:group
//      xsd:attributeGroup
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// schemaLocation	[1..1]	xsd:anyURI
// id	            [0..1]	xsd:ID
//
// Used in
// Anonymous type of element xsd:schema
#[derive(Debug)]
pub struct Redefine<'a> {
    pub annotations: Vec<Annotation<'a>>,
    pub content: Vec<Redefinable<'a>>,
    pub attributes: AnyAttributes,
    pub schema_location: SchemaLocation,
    pub id: Option<Id>,
}
