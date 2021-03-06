// elementFormDefault
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:formChoice
// Properties: Local, Unqualified
//
// Value
//   Type based on xsd:NMTOKEN
//     Valid value	    Description
//       qualified	    Local declarations are qualified (in a namespace)
//       unqualified	Local declarations are unqualified (not in a namespace)
//
// Used in
// Anonymous type of element xsd:schema

use crate::model::simple_types::FormChoice;

use std::convert::TryFrom;
use crate::model::RawAttribute;

pub struct ElementFormDefault(FormChoice);

impl TryFrom<RawAttribute<'_>> for ElementFormDefault {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl ElementFormDefault {
    pub const NAME: &'static str = "elementFormDefault";
}

