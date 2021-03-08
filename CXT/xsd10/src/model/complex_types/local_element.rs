use crate::model::elements::annotation::Annotation;
use crate::model::groups::element_model::ElementModel;
use crate::model::simple_types::block_set::BlockSet;
use crate::model::simple_types::form_choice::FormChoice;
use crate::model::simple_types::ncname::NCName;
use crate::model::simple_types::qname::QName;
use crate::model::simple_types::Id;
use crate::model::{RawAttribute};
use crate::model::attributes::max_occurs::MaxOccurs;
use crate::model::attributes::min_occurs::MinOccurs;

// xsd:localElement
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
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	        from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                from type xsd:annotated
// name	            [0..1]	xsd:NCName
// ref	            [0..1]	xsd:QName
// type	            [0..1]	xsd:QName
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1". from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1". from group xsd:occurs
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
// nillable	        [0..1]	xsd:boolean		                                        Default value is "false".
// block	        [0..1]	xsd:blockSet
// form	            [0..1]	xsd:formChoice
//
// Used by
// Element xsd:element
// Element xsd:element via derived type xsd:narrowMaxMin
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localElement
//                  restricted by xsd:narrowMaxMin
#[derive(Debug, Default)]
pub struct LocalElement<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id,
    pub name: Option<NCName>,
    pub ref_: Option<QName>,
    pub type_: Option<QName>,
    pub min_occurs: MinOccurs,
    pub max_occurs: MaxOccurs,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
    pub nillable: bool,
    pub block: Option<BlockSet>,
    pub form: Option<FormChoice>,
}