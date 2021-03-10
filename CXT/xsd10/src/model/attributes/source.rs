// source
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:appinfo Anonymous type of element xsd:documentation

use crate::model::simple_types::AnyUri;
use xml_utils::*;

#[attribute(name = "source")]
pub struct Source(AnyUri);
