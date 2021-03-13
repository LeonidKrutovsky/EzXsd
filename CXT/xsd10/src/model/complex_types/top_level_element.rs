use crate::model::elements::annotation::Annotation;
use crate::model::groups::element_model::ElementModel;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::type_::Type;
use crate::model::attributes::substitution_group::SubstitutionGroup;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::nillable::Nillable;
use crate::model::attributes::abstract_::Abstract;
use crate::model::attributes::final_::Final;
use crate::model::attributes::block::Block;
use crate::model::attributes::AnyAttributes;

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
#[derive(Debug)]
pub struct TopLevelElement<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
    pub type_: Option<Type>,
    pub substitution_group: Option<SubstitutionGroup>,
    pub default: Option<Default_>,
    pub fixed: Option<Fixed>,
    pub nillable: Nillable,
    pub abstract_: Abstract,
    pub final_: Option<Final>,
    pub block: Option<Block>,
}
