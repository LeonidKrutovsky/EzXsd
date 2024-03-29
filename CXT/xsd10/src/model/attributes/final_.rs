use xml_utils::*;

use crate::model::simple_types::{DerivationSet, SimpleDerivationSet};

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
#[attribute(name = "final")]
pub struct Final(pub DerivationSet);

#[attribute(name = "final")]
pub struct SimpleFinal(pub SimpleDerivationSet);
