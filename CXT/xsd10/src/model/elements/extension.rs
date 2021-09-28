use xml_utils::element;
use crate::model::{attributes, elements, groups};

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExtensionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
#[element(name = "extension")]
pub struct SimpleExtension {
    pub annotation: Option<elements::Annotation>,
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:extensionType
// Properties: Local, Qualified
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	[0..1]	xsd:ID		from type xsd:annotated
// base	[1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:complexContent
#[element(name = "extension")]
pub struct Extension {
    pub annotation: Option<elements::Annotation>,
    pub type_def_particle: Option<groups::TypeDefParticle>,
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}

