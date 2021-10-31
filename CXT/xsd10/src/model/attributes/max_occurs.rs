use crate::model::simple_types::NonNegativeInteger;
use std::convert::TryFrom;

// maxOccurs
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Union of
//  xsd nonNegativeInteger
//  Type based on xsd NMTOKEN
//    Valid value
//    unbounded
//
// Used in
// Attribute group xsd:occurs
// Anonymous type of element xsd:any via reference to xsd:occurs
// Type xsd:localElement via reference to xsd:occurs (Element xsd:element)
//  Type xsd:namedGroupRef via reference to xsd:occurs (Element xsd:group)
//  Type xsd:narrowMaxMin via reference to xsd:occurs (Element xsd:element)
//  Type xsd:explicitGroup via reference to xsd:occurs (Elements xsd:choice , xsd:sequence)
#[derive(Debug, PartialEq)]
pub enum MaxOccurs {
    Bounded(NonNegativeInteger),
    Unbounded,
}

impl Default for MaxOccurs {
    fn default() -> Self {
        Self::Bounded("1".parse().unwrap())
    }
}

impl MaxOccurs {
    pub const NAME: &'static str = "maxOccurs";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(match attr.value() {
            "unbounded" => Self::Unbounded,
            _ => Self::Bounded(attr.value().parse()?),
        })
    }

    pub fn text(&self) -> String {
        match self {
            MaxOccurs::Bounded(v) => format!("{}=\"{}\"", Self::NAME, v),
            MaxOccurs::Unbounded => format!("{}=\"unbounded\"", Self::NAME),
        }
    }
}

// maxOccurs
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd nonNegativeInteger
//    Valid value
//    0 1
//
// Used in
// Type xsd:narrowMaxMin (Element xsd:element)

#[derive(Debug, PartialEq)]
pub enum MaxOccursBool {
    Zero,
    One,
}

impl Default for MaxOccursBool {
    fn default() -> Self {
        Self::One
    }
}

impl TryFrom<&roxmltree::Attribute<'_>> for MaxOccursBool {
    type Error = String;

    fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
        Ok(match attr.value() {
            "0" => Self::Zero,
            "1" => Self::One,
            _ => {
                return Err(format!(
                    "MaxOccurs: Invalid attribute value: {}",
                    attr.value()
                ))
            }
        })
    }
}

impl MaxOccursBool {
    pub const NAME: &'static str = "maxOccurs";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(match attr.value() {
            "0" => Self::Zero,
            "1" => Self::One,
            _ => {
                return Err(format!(
                    "MaxOccurs: Invalid attribute value: {}",
                    attr.value()
                ))
            }
        })
    }

    pub fn text(&self) -> String {
        let value = match self {
            Self::Zero => "0",
            Self::One => "1",
        };
        format!("{}={}", Self::NAME, value)
    }
}

// maxOccurs
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd nonNegativeInteger
//    Valid value
//    1
//
// Used in
// Type xsd:allType (Element xsd:all)
//

#[derive(Debug, PartialEq)]
pub struct MaxOccursOne(u8);

impl MaxOccursOne {
    pub const NAME: &'static str = "maxOccurs";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(match attr.value() {
            "1" => Self(1),
            _ => {
                return Err(format!(
                    "MaxOccurs: Invalid attribute value: {}",
                    attr.value()
                ))
            }
        })
    }
}

impl Default for MaxOccursOne {
    fn default() -> Self {
        Self(1)
    }
}
