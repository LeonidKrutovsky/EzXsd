use crate::model::simple_types::FullDerivationSet;
use xml_utils::attribute;

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
#[attribute(name = "finalDefault")]
pub struct FinalDefault(pub FullDerivationSet);
