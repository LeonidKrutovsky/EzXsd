use xml_utils::*;

use crate::model::simple_types::BlockSet;

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
#[attribute(name = "blockDefault")]
pub struct BlockDefault(pub BlockSet);
