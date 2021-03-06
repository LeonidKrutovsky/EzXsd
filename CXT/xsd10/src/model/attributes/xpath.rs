// xpath
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd token
// Pattern: (\.//)?(((child::)?((\i\c*:)?(\i\c*|\*)))|\.)(/(((child::)?((\i\c*:)?(\i\c*|\*)))|\.))*(\|(\.//)?(((child::)?((\i\c*:)?(\i\c*|\*)))|\.)(/(((child::)?((\i\c*:)?(\i\c*|\*)))|\.))*)*
//
// Used in
// Anonymous type of element xsd:selector

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::Token;

pub struct XPath(Token);

impl TryFrom<RawAttribute<'_>> for XPath {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl XPath {
    pub const NAME: &'static str = "xpath";
}



// xpath
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd token
// Pattern: (\.//)?((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)/)*((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)|((attribute::|@)((\i\c*:)?(\i\c*|\*))))(\|(\.//)?((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)/)*((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)|((attribute::|@)((\i\c*:)?(\i\c*|\*)))))*
//
// Used in
// Anonymous type of element xsd:field
