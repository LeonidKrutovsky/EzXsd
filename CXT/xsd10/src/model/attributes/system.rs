use crate::model::simple_types::AnyUri;
use xml_utils::*;

// system
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:notation

#[attribute(name = "system")]
pub struct System(pub AnyUri);
