// use
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd NMTOKEN
//     Valid value
//          prohibited
//          optional
//          required
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//
use std::convert::TryFrom;
use crate::model::RawAttribute;

pub enum Use{
    Optional,
    Prohibited,
    Required
}

impl TryFrom<RawAttribute<'_>> for Use {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(match attr.value() {
            "prohibited" => Self::Prohibited,
            "optional" => Self::Optional,
            "required" => Self::Required,
            _ => return Err(format!("Use: invalid attribute value: {}", attr.value()))
        })
    }
}

impl Use {
    pub const NAME: &'static str = "use";
}