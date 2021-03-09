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
use xml_utils::*;


#[attribute(name = "final")]
pub struct Final(DerivationSet);
