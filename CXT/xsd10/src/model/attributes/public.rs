use crate::model::simple_types::Public as SimpleTypePublic;
use xml_utils::*;

// public
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:public
// Properties: Local, Unqualified
//
// Value:
// xsd token
//
// Used in
// Anonymous type of element xsd:notation
#[attribute(name = "public")]
pub struct Public(pub SimpleTypePublic);
