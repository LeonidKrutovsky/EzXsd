// final
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:derivationSet
// Properties: Local, Unqualified
//
// Value
//   Union of:
//     Type based on xsd:token
//       Valid value
//         #all
//   List of:
//     Type based on xsd:NMTOKEN
//       Valid value	Description
//       extension	    Extension is disallowed
//       restriction	Restriction is disallowed
//
// Used in
// Type xsd:topLevelComplexType (Element xsd:complexType)
//  Type xsd:topLevelElement (Element xsd:element)
//

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::DerivationSet;

pub struct Final(DerivationSet);

impl TryFrom<RawAttribute<'_>> for Final {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Final {
    pub const NAME: &'static str = "final";
}