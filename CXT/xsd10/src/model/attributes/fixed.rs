// fixed
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
//

use crate::model::simple_types::String_;

use xml_utils::*;

#[attribute(name = "fixed")]
pub struct Fixed(pub String_);
