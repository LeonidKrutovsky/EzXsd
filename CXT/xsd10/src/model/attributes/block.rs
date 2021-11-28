use xml_utils::*;

use crate::model::simple_types::{BlockSet, DerivationSet, DerivationSubset};

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

#[cfg(test)]
mod tests {
    use crate::model::attributes::{Block, DerivationBlock};
    use crate::model::simple_types::{BlockSet, DerivationSet};

    #[test]
    fn deriviation_block_to_text() {
        let deriviation_block = DerivationBlock(DerivationSet::All);
        assert_eq!(deriviation_block.text(), " block=\"#all\"");

        let deriviation_block = DerivationBlock(DerivationSet::List("restriction extension".parse().unwrap()));
        assert_eq!(deriviation_block.text(), " block=\"restriction extension\"");
    }

        #[test]
    fn block_to_text() {
        let block = Block(BlockSet::All);
        assert_eq!(block.text(), " block=\"#all\"");

        let block = Block(BlockSet::List("restriction extension substitution".parse().unwrap()));
        assert_eq!(block.text(), " block=\"restriction extension substitution\"");
    }
}
