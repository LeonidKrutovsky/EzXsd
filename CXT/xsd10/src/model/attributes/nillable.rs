// nillable
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd boolean
//
// Used in
// Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
//

use crate::model::simple_types::Boolean;

use xml_utils::*;

#[attribute(name = "nillable")]
#[derive(Default)]
pub struct Nillable(Boolean);
