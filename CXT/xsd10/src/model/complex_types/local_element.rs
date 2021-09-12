use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::complex_type;

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
#[complex_type()]
pub struct LocalElement {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub min_occurs: attributes::MinOccurs,
    pub max_occurs: attributes::MaxOccurs,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub block: Option<attributes::Block>,
    pub form: Option<attributes::Form>,
}
