use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

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
#[complex_type()]
pub struct AllType {
    pub annotation: Option<elements::Annotation>,
    pub elements: Vec<elements::Element>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub min_occurs: Option<attributes::MinOccursBool>,
    pub max_occurs: Option<attributes::MaxOccursOne>,
}
