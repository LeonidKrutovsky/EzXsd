// mixed
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd boolean
//
// Used in
// Anonymous type of element xsd:complexContent Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::Boolean;

#[derive(Default, Debug)]
pub struct Mixed(Boolean);

impl TryFrom<&RawAttribute<'_>> for Mixed {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Mixed {
    pub const NAME: &'static str = "mixed";
}

