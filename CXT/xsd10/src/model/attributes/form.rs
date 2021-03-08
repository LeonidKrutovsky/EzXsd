// form
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:formChoice
// Properties: Local, Unqualified
//
// Value
//   Type based on xsd:NMTOKEN
//     Valid value	  Description
//     qualified	  Local declarations are qualified (in a namespace)
//     unqualified	  Local declarations are unqualified (not in a namespace)
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::FormChoice;

#[derive(Debug)]
pub struct Form(FormChoice);

impl TryFrom<&RawAttribute<'_>> for Form {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Form {
    pub const NAME: &'static str = "form";
}