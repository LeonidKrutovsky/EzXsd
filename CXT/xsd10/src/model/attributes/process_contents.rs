// processContents
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd NMTOKEN
// Valid value
//      skip
//      lax
//      strict
//
// Used in
// Anonymous type of element xsd:any via derivation of xsd:wildcard
// Type xsd:wildcard (Element xsd:anyAttribute)
//
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum ProcessContents {
    Lax,
    Skip,
    Strict,
}

impl TryFrom<&roxmltree::Attribute<'_>> for ProcessContents {
    type Error = String;

    fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
        Ok(match attr.value() {
            "skip" => Self::Skip,
            "lax" => Self::Lax,
            "strict" => Self::Strict,
            _ => {
                return Err(format!(
                    "ProcessContents: Invalid attribute value: {}",
                    attr.value()
                ))
            }
        })
    }
}

impl ProcessContents {
    pub const NAME: &'static str = "processContents";
}

impl Default for ProcessContents {
    fn default() -> Self {
        Self::Strict
    }
}
