// minOccurs
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:nonNegativeInteger
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd nonNegativeInteger
//
// Used in
// Attribute group xsd:occurs
// Anonymous type of element xsd:any via reference to xsd:occurs
// Type xsd:localElement via reference to xsd:occurs (Element xsd:element)
//  Type xsd:namedGroupRef via reference to xsd:occurs (Element xsd:group)
//  Type xsd:narrowMaxMin via reference to xsd:occurs (Element xsd:element)
//  Type xsd:explicitGroup via reference to xsd:occurs (Elements xsd:choice , xsd:sequence)

use crate::model::simple_types::NonNegativeInteger;
use std::convert::TryFrom;
use crate::model::RawAttribute;

#[derive(Debug)]
pub struct MinOccurs(pub NonNegativeInteger);

impl TryFrom<&RawAttribute<'_>> for MinOccurs {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Default for MinOccurs {
    fn default() -> Self {
        Self("1".parse().unwrap())
    }
}

impl MinOccurs {
    pub const NAME: &'static str = "minOccurs";
}

// minOccurs
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd nonNegativeInteger
//   Valid value
//   0 1
//
// Used in
// Type xsd:allType (Element xsd:all)
//  Type xsd:narrowMaxMin (Element xsd:element)

#[derive(Debug)]
pub enum MinOccursBool{
    Zero,
    One
}

impl TryFrom<RawAttribute<'_>> for MinOccursBool {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(match attr.value() {
            "0" => Self::Zero,
            "1" => Self::One,
            _ => return Err(format!("MinOccurs: Invalid attribute value: {}", attr.value()))
        })
    }
}

impl MinOccursBool {
    pub const NAME: &'static str = "minOccurs";
}