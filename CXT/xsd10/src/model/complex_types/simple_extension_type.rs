use crate::model::elements::annotation::Annotation;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::RawAttribute;
use crate::model::attributes::id::Id;
use crate::model::attributes::base::Base;

// xsd:simpleExtensionType
// attrs only
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..*]       from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
// Used by
// Element xsd:extension
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:simpleExtensionType
#[derive(Debug, Default)]
pub struct SimpleExtensionType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attr_decls: AttrDecls<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Option<Id>,
    pub base: Base,
}
