use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::complex_type;

// xsd:simpleRestrictionType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Sequence [0..1]                 from group xsd:simpleRestrictionModel
//          xsd:simpleType [0..1]
//          Choice [0..*]               from group xsd:facets
//              xsd:minExclusive
//              xsd:minInclusive
//              xsd:maxExclusive
//              xsd:maxInclusive
//              xsd:totalDigits
//              xsd:fractionDigits
//              xsd:length
//              xsd:minLength
//              xsd:maxLength
//              xsd:enumeration
//              xsd:whiteSpace
//              xsd:pattern
//      Choice [0..*]                   from group xsd:attrDecls
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
// Element xsd:restriction
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleRestrictionType
#[complex_type()]
pub struct SimpleRestrictionType {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::SimpleRestrictionModel,
    pub attr_decls: groups::AttrDecls,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
    pub attributes: Vec<attributes::RawAttribute>,
}
