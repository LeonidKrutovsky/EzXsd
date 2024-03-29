use xml_utils::*;

use crate::model::simple_types::String_;

// default
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:string
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd string
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelAttributeType (Element xsd:attribute)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
#[attribute(name = "default")]
pub struct Default_(pub String_);
