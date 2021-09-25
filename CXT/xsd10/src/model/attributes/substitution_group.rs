use crate::model::simple_types::QName;
use xml_utils::*;

// substitutionGroup
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Type xsd:topLevelElement (Element xsd:element)
//
#[attribute(name = "substitutionGroup")]
pub struct SubstitutionGroup(pub QName);
