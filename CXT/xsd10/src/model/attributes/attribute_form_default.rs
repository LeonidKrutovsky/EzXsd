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
use crate::model::simple_types::FormChoice;

pub struct AttributeFormDefault(FormChoice);

impl AttributeFormDefault {
    const NAME: &'static str = "attributeFormDefault";
}

impl TryFrom<RawAttribute<'_>> for AttributeFormDefault {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}
