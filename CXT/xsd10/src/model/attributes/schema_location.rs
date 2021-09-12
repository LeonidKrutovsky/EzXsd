// schemaLocation
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:import Anonymous type of element xsd:include Anonymous type of element xsd:redefine

use crate::model::simple_types::AnyUri;
use xml_utils::*;

#[attribute(name = "schemaLocation")]
pub struct SchemaLocation(pub AnyUri);
