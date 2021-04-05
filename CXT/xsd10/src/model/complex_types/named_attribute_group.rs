use crate::model::groups;
use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:namedAttributeGroup
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//
//      Choice [0..*]    from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used by
// Element xsd:attributeGroup
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:namedAttributeGroup
#[complex_type()]
pub struct NamedAttributeGroup {
    pub annotation: Option<elements::Annotation>,
    pub content: groups::AttrDecls,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
}
