// abstract
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:boolean.

// Used in
// Type xsd:topLevelComplexType (Element xsd:complexType)
// Type xsd:topLevelElement (Element xsd:element)

use crate::model::simple_types::Boolean;

pub type Abstract = Boolean;

impl Abstract {
    const NAME: &'static str = "abstract";
}
