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

#[derive(Debug, PartialEq)]
pub enum Use{
    Optional,
    Prohibited,
    Required
}

impl TryFrom<&roxmltree::Attribute<'_>> for Use {
    type Error = String;

    fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
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

impl Default for Use {
    fn default() -> Self {
        Self::Optional
    }
}