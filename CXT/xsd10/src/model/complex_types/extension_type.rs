use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::complex_type;

// xsd:extensionType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..1]               from group xsd:typeDefParticle
//          xsd:group
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//          xsd:choice
//          xsd:sequence
//      Choice [0..*]       from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
//
// Used by
// Element xsd:extension
#[complex_type()]
pub struct ExtensionType {
    pub annotation: Option<elements::Annotation>,
    pub type_def_particle: Option<groups::TypeDefParticle>,
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}
