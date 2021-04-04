use crate::model::elements::annotation::Annotation;
use crate::model::attributes::id::Id;
use crate::model::attributes::fixed::FixedBool;
use crate::model::attributes::value::Value;
use crate::model::attributes::AnyAttributes;

// xsd:facet
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
// xsd:annotation [0..1]       from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType
// fixed	        [0..1]	xsd:boolean		Default value is "false".
//
// Used by
// Element xsd:maxExclusive
// Element xsd:maxInclusive
// Element xsd:minExclusive
// Element xsd:minInclusive
// Element xsd:enumeration via derived type xsd:noFixedFacet
// Element xsd:fractionDigits via derived type xsd:numFacet
// Element xsd:length via derived type xsd:numFacet
// Element xsd:maxLength via derived type xsd:numFacet
// Element xsd:minLength via derived type xsd:numFacet
// Element xsd:pattern via derived anonymous type
// Element xsd:totalDigits via derived anonymous type
// Element xsd:whiteSpace via derived anonymous type
//
// Type inheritance chain
// xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:facet
//                  restricted by xsd:noFixedFacet
//                      restricted by Anonymous type of element xsd:pattern
//                  restricted by xsd:numFacet
//                      restricted by Anonymous type of element xsd:totalDigits
//                  restricted by Anonymous type of element xsd:whiteSpace
#[derive(Default, Debug)]
pub struct Facet {
    pub annotation: Option<Annotation>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub value: Value,
    pub fixed: FixedBool,
}
