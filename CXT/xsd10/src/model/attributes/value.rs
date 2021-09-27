use crate::model::simple_types::{AnySimpleType, NonNegativeInteger, PositiveInteger, String_};
use xml_utils::*;

// value
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
// Type: xsd:anySimpleType
// Properties: Local, Unqualified
//
// Value
//  A value of type xsd:anySimpleType.
// Used in
// Type xsd:noFixedFacet via derivation of xsd:facet (Element xsd:enumeration)
// Type xsd:facet (Elements xsd:minExclusive, xsd:minInclusive, xsd:maxExclusive, xsd:maxInclusive)
#[derive(Default, Debug)]
pub struct Value(pub AnySimpleType);

impl Value {
    pub const NAME: &'static str = "value";

    pub fn parse(s: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(Self(s.value().to_string()))
    }
}

// value
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
// Type: xsd:nonNegativeInteger
//
// Properties: Local, Unqualified
//
// Value
//  A value of type xsd:nonNegativeInteger.
// Used in
// Type xsd:numFacet (Elements xsd:fractionDigits, xsd:length, xsd:minLength, xsd:maxLength)

#[attribute(name = "value")]
pub struct NonNegativeValue(pub NonNegativeInteger);

// value
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
// Type: xsd:positiveInteger
//
// Properties: Local, Unqualified
//
// Value
//  A value of type xsd:positiveInteger.
// Used in
// Anonymous type of element xsd:totalDigits

#[attribute(name = "value")]
pub struct PositiveValue(pub PositiveInteger);

// value
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value
// Type based on xsd:NMTOKEN
//  Valid value
//      preserve
//      replace
//      collapse
// Used in
// Anonymous type of element xsd:whiteSpace
#[derive(Debug, PartialEq)]
pub enum WhiteSpaceValue {
    Preserve,
    Replace,
    Collapse,
}

impl WhiteSpaceValue {
    pub const NAME: &'static str = "value";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(match attr.value() {
            "collapse" => Self::Collapse,
            "preserve" => Self::Preserve,
            "replace" => Self::Replace,
            _ => {
                return Err(format!(
                "Invalid xsd:whiteSpace:value type: {}. Anonymous (collapse | preserve | replace)",
                attr.value()
            ))
            }
        })
    }
}

// value
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
// Type: xsd:string
// Properties: Local, Unqualified
//
// Value
//  A value of type xsd:string.
// Used in
// Anonymous type of element xsd:pattern
#[attribute(name = "value")]
pub struct PatternValue(pub String_);
