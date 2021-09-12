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

use crate::model::simple_types::{BlockSet, DerivationSet, DerivationSubset};

use xml_utils::*;

#[attribute(name = "block")]
pub struct DerivationBlock(pub DerivationSet);

impl DerivationBlock {
    pub fn is_all(&self) -> bool {
        match self.0 {
            DerivationSet::All => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self.0 {
            DerivationSet::List(_) => true,
            _ => false,
        }
    }

    pub fn get_list(&self) -> Option<&[DerivationSubset]> {
        match self.0 {
            DerivationSet::List(ref value) => Some(value.0.as_slice()),
            _ => None,
        }
    }
}

// block
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Other attributes with the same name: block
// Type: xsd:blockSet
// Properties: Local, Unqualified
//
// Value
//  Union of:
//      Type based on xsd:token
//          Valid value
//              #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//                  extension
//                  restriction
//                  substitution
// Used in
// Type xsd:localElement (Element xsd:element)
// Type xsd:topLevelElement (Element xsd:element)
// Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)

#[attribute(name = "block")]
pub struct Block(pub BlockSet);
