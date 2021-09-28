use crate::model::elements;
use crate::model::groups::facets::Facets;

// xsd:simpleRestrictionModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:simpleType [0..1]
//      Choice [0..*]       from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:simpleRestrictionType (Element xsd:restriction)
#[derive(Default, Debug)]
pub struct SimpleRestrictionModel {
    pub simple_type: Option<elements::LocalSimpleType>,
    pub facets: Vec<Facets>,
}

impl SimpleRestrictionModel {
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        let mut result = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.tag_name().name() {
                elements::LocalSimpleType::NAME => result.simple_type = Some(elements::LocalSimpleType::parse(node)?),
                tag_name if Facets::NAMES.contains(&tag_name) => {
                    result.facets.push(Facets::parse(ch)?)
                }
                _ => {}
            }
        }
        Ok(result)
    }
}
