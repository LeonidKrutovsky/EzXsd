// block
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Other attributes with the same name: block
// Type: xsd:derivationSet
// Properties: Local, Unqualified
//
// Value
// Union of:
//  Type based on xsd:token
//      Valid value
//      #all
//  List of:
//      Type based on xsd:NMTOKEN
//          Valid value	Description
//          extension	Extension is disallowed
//          restriction	Restriction is disallowed

// Used in
// Type xsd:topLevelComplexType (Element xsd:complexType)

use crate::model::simple_types::DerivationSet;

pub type Block = DerivationSet;

impl Block {
        const NAME: &'static str = "abstract";
}