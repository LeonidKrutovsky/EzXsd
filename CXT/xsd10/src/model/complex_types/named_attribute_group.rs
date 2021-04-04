use crate::model::elements::annotation::Annotation;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::attributes::name::Name;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

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
#[derive(Debug)]
pub struct NamedAttributeGroup {
    pub annotation: Option<Annotation>,
    pub content: AttrDecls,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
}
