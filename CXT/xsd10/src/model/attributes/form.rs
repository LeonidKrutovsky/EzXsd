use crate::model::simple_types::FormChoice;

use xml_utils::*;

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
#[attribute(name = "form")]
pub struct Form(pub FormChoice);
