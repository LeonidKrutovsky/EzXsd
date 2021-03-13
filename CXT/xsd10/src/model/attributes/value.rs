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

use xml_utils::*;
use crate::model::simple_types::{AnySimpleType, NonNegativeInteger, PositiveInteger, String_};
use std::str::FromStr;
use std::convert::TryFrom;
use crate::model::RawAttribute;

#[derive(Default, Debug)]
pub struct Value(pub AnySimpleType);

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Value {
    pub const NAME: &'static str = "value";
}

impl TryFrom<&RawAttribute<'_>> for Value {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        attr.value().parse()
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
pub enum WhiteSpaceValue{
    Preserve,
    Replace,
    Collapse,
}

impl FromStr for WhiteSpaceValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "collapse" => Self::Collapse,
            "preserve" => Self::Preserve,
            "replace" => Self::Replace,
            _ => {
                return Err(format!(
                "Invalid xsd:whiteSpace:value type: {}. Anonymous (collapse | preserve | replace)",
                s
            ))
            }
        })
    }
}

impl WhiteSpaceValue {
    pub const NAME: &'static str = "value";
}

impl TryFrom<&RawAttribute<'_>> for WhiteSpaceValue {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        attr.value().parse()
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