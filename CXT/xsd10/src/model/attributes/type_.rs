// type
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelAttributeType (Element xsd:attribute)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//

use crate::model::simple_types::QName;
use xml_utils::*;

#[attribute(name = "type")]
pub struct Type(pub(crate) QName);
