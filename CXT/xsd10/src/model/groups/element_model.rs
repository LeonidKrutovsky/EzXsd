use crate::model::elements::complex_type::LocalComplexType;
use crate::model::elements::simple_type::LocalSimpleType;
use crate::model::groups::identity_constraint::IdentityConstraint;

// xsd:elementModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      Choice [0..1]
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]        from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Used in
// Type xsd:localElement (Element xsd:element)
// Type xsd:narrowMaxMin (Element xsd:element)
// Type xsd:topLevelElement (Element xsd:element)
#[derive(Debug, Default)]
pub struct ElementModel {
    pub choice: Option<ElementModelChoice>,
    pub identity_constraints: Vec<IdentityConstraint>,
}

#[derive(Debug)]
pub enum ElementModelChoice {
    SimpleType(LocalSimpleType),
    ComplexType(LocalComplexType),
}

impl ElementModel {
    pub fn parse(node: &roxmltree::Node) -> Result<Self, String> {
        let mut choice: Option<ElementModelChoice> = None;
        let mut identity_constraints: Vec<IdentityConstraint> = vec![];
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.tag_name().name() {
                LocalSimpleType::NAME => {
                    choice = Some(ElementModelChoice::SimpleType(LocalSimpleType::parse(ch)?))
                }
                LocalComplexType::NAME => {
                    choice = Some(ElementModelChoice::ComplexType(LocalComplexType::parse(
                        ch,
                    )?))
                }
                tag_name if IdentityConstraint::NAMES.contains(&tag_name) => {
                    identity_constraints.push(IdentityConstraint::parse(ch)?)
                }
                _ => {}
            }
        }
        Ok(Self {
            choice,
            identity_constraints,
        })
    }
}
