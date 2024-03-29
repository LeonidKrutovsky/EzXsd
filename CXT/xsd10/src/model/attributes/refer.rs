use crate::model::simple_types::QName;

use xml_utils::*;

// refer
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Anonymous type of element xsd:keyref
#[attribute(name = "refer")]
pub struct Refer(pub QName);
