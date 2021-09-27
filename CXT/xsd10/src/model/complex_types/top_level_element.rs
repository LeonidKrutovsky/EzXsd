use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::complex_type;

// xsd:topLevelElement
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]               from type xsd:annotated
//      Choice [0..1]                       from group xsd:elementModel
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]                       from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Attributes
// Any attribute	    [0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	                [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	                [1..1]	xsd:NCName
// type	                [0..1]	xsd:QName
// substitutionGroup	[0..1]	xsd:QName
// default	            [0..1]	xsd:string
// fixed	            [0..1]	xsd:string
// nillable	            [0..1]	xsd:boolean		    Default value is "false".
// abstract	            [0..1]	xsd:boolean		    Default value is "false".
// final	            [0..1]	xsd:derivationSet
// block	            [0..1]	xsd:blockSet
//
// Used by
// Element xsd:element
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelElement
#[complex_type()]
pub struct TopLevelElement {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub type_: Option<attributes::Type>,
    pub substitution_group: Option<attributes::SubstitutionGroup>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub abstract_: attributes::Abstract,
    pub final_: Option<attributes::Final>,
    pub block: Option<attributes::Block>,
}
