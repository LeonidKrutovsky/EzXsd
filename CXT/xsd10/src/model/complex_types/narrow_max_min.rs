use crate::model::elements::annotation::Annotation;
use crate::model::groups::element_model::ElementModel;
use crate::model::{RawAttribute};
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::ref_::Ref;
use crate::model::attributes::type_::Type;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::nillable::Nillable;
use crate::model::attributes::block::Block;
use crate::model::attributes::form::Form;
use crate::model::attributes::min_occurs::MinOccurs;
use crate::model::attributes::max_occurs::MaxOccurs;

// xsd:narrowMaxMin
// restricted max/min
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [0..1]    from group xsd:elementModel
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]    from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Attributes
// id	            [0..1]	xsd:ID		                                                from type xsd:annotated
// name	            [0..1]	xsd:NCName		                                            from type xsd:localElement
// ref	            [0..1]	xsd:QName		                                            from type xsd:localElement
// type	            [0..1]	xsd:QName		                                            from type xsd:localElement
// default	        [0..1]	xsd:string		                                            from type xsd:localElement
// fixed	        [0..1]	xsd:string		                                            from type xsd:localElement
// nillable	        [0..1]	xsd:boolean		Default value is "false".                   from type xsd:localElement
// block	        [0..1]	xsd:blockSet		                                        from type xsd:localElement
// form	            [0..1]	xsd:formChoice		                                        from type xsd:localElement
// minOccurs	    [0..1]	Anonymous		Default value is "1".
// maxOccurs	    [0..1]	Anonymous		Default value is "1".
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:element
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localElement
//                  xsd:narrowMaxMin
#[derive(Debug)]
pub struct NarrowMaxMin<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<Ref>,
    pub type_: Option<Type>,
    pub default: Option<Default_>,
    pub fixed: Option<Fixed>,
    pub nillable: Nillable,
    pub block: Option<Block>,
    pub form: Option<Form>,
    pub min_occurs: MinOccurs, //Anonymous in doc, probably mistake
    pub max_occurs: MaxOccurs,
    pub attributes: Vec<RawAttribute<'a>>,
}
