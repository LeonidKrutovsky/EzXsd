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
#[derive(Debug, PartialEq)]
pub enum Use {
    Optional,
    Prohibited,
    Required,
}

impl Use {
    pub const NAME: &'static str = "use";

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        Ok(match attr.value() {
            "prohibited" => Self::Prohibited,
            "optional" => Self::Optional,
            "required" => Self::Required,
            _ => return Err(format!("Use: invalid attribute value: {}", attr.value())),
        })
    }

    pub fn text(&self) -> String {
        let value = match self {
            Use::Optional => "optional",
            Use::Prohibited => "prohibited",
            Use::Required => "required"
        };
        format!("{}={}", Self::NAME, value)
    }
}
