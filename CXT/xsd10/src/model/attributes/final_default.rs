// finalDefault
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:fullDerivationSet
// Properties: Local, Unqualified
//
// Value
//   Union of:
//     Type based on xsd:token
//       Valid value
//         #all
//     List of:
//       Type based on xsd:NMTOKEN
//         Valid value	    Description
//         extension	    Extension is disallowed
//         restriction	    Restriction is disallowed
//         list	        Derivation by list is disallowed
//         union	        Derivation by union is disallowed
//
// Used in
// Anonymous type of element xsd:schema

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::FullDerivationSet;

pub struct FinalDefault(FullDerivationSet);

impl TryFrom<RawAttribute<'_>> for FinalDefault {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl FinalDefault {
    pub const NAME: &'static str = "finalDefault";
}