use std::str::FromStr;

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
#[derive(Debug, PartialEq)]
pub enum ProcessContents {
    Lax,
    Skip,
    Strict,
}

impl ProcessContents {
    pub const NAME: &'static str = "processContents";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        attr.value().parse()
    }
}

impl FromStr for ProcessContents {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "skip" => Self::Skip,
            "lax" => Self::Lax,
            "strict" => Self::Strict,
            _ => {
                return Err(format!(
                    "ProcessContents: Invalid attribute value: {}",
                    s
                ))
            }
        })
    }
}
