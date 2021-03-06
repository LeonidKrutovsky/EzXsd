// blockDefault
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:blockSet
// Properties: Local, Unqualified
//
// Value:
// Union of:
//     Type based on xsd:token
//       Valid value
//         #all
//     List of:
//       Type based on xsd:NMTOKEN
//         Valid value
//           extension
//           restriction
//           substitution
//
// Used in
// Anonymous type of element xsd:schema

use crate::model::simple_types::BlockSet;
use std::convert::TryFrom;
use crate::model::RawAttribute;

pub struct BlockDefault(BlockSet);

impl TryFrom<RawAttribute<'_>> for BlockDefault {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl BlockDefault {
    pub const NAME: &'static str = "blockDefault";
}
