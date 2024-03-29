use crate::model::attributes;
use crate::model::elements;
use xml_utils::complex_type;

// xsd:numFacet
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		from type xsd:annotated
// fixed	        [0..1]	xsd:boolean		        Default value is "false". from type xsd:facet
// value	        [1..1]	xsd:nonNegativeInteger
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:fractionDigits
// Element xsd:length
// Element xsd:maxLength
// Element xsd:minLength
// Element xsd:totalDigits via derived anonymous type
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:facet
//                  xsd:numFacet
//                      restricted by Anonymous type of element xsd:totalDigits
#[complex_type()]
pub struct NumFacet {
    pub annotation: Option<elements::Annotation>,
    pub id: Option<attributes::Id>,
    pub fixed: attributes::FixedBool,
    pub value: attributes::NonNegativeValue,
    pub attributes: Vec<attributes::RawAttribute>,
}
