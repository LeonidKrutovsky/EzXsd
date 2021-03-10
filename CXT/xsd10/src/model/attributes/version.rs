// version
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:token
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd token
//
// Used in
// Anonymous type of element xsd:schema

use crate::model::simple_types::Token;
use xml_utils::*;

#[attribute(name = "version")]
pub struct Version(Token);