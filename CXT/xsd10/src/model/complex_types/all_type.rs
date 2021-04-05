use crate::model::elements::annotation::Annotation;
use crate::model::elements::element::Element;
use crate::model::attributes::min_occurs::MinOccursBool;
use crate::model::attributes::max_occurs::MaxOccursOne;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

// xsd:allType
// An "all" group that allows elements to appear in any order.
// Unlike other group types, does not allow other groups as children, only elements.
// See http://www.w3.org/TR/xmlschema-1/#element-all.
//
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..*]       from group xsd:allModel
//          xsd:element
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		from type xsd:annotated
// minOccurs	    [0..1]	Anonymous		Default value is "1".
// maxOccurs	    [0..1]	Anonymous		Default value is "1".
//
// Used by
// Element xsd:all
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:allType
#[derive(Debug, Default)]
pub struct AllType {
    pub annotation: Option<Annotation>,
    pub elements: Vec<Element>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub min_occurs: Option<MinOccursBool>,
    pub max_occurs: Option<MaxOccursOne>,
}
