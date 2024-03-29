use xml_utils::attribute;

use crate::model::simple_types::FormChoice;

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
#[attribute(name = "elementFormDefault")]
pub struct ElementFormDefault(pub FormChoice);
