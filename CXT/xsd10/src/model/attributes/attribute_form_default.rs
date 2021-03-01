// attributeFormDefault
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:formChoice
// Properties: Local, Unqualified
//
// Value
// Type based on xsd:NMTOKEN

// Valid value	Description
// qualified	Local declarations are qualified (in a namespace)
// unqualified	Local declarations are unqualified (not in a namespace)

// Used in
// Anonymous type of element xsd:schema

use crate::model::RawAttribute;
use std::convert::TryFrom;

pub enum AttributeFormDefault {
    Qualified,
    Unqualified,
}

impl AttributeFormDefault {
    const NAME: &'static str = "attributeFormDefault";
}

impl TryFrom<RawAttribute<'_>> for AttributeFormDefault {
    type Error = String;

    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        match value.value() {
            "qualified" => Ok(Self::Qualified),
            "unqualified" => Ok(Self::Unqualified),
            _ => Err(format!("Invalid attribute {:?}", value)),
        }
    }
}
