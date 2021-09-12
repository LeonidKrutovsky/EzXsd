// mixed
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd boolean
//
// Used in
// Anonymous type of element xsd:complexContent Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)
//

use crate::model::simple_types::Boolean;
use xml_utils::*;

#[attribute(name = "mixed")]
pub struct Mixed(Boolean);
