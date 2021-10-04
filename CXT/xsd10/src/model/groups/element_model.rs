use crate::model::elements;
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
    SimpleType(elements::LocalSimpleType),
    ComplexType(elements::LocalComplexType),
}

impl ElementModel {
    pub const NAMES: &'static [&'static str] = &[
        elements::LocalSimpleType::NAME,
        elements::LocalComplexType::NAME,
        elements::Unique::NAME,
        elements::Key::NAME,
        elements::KeyRef::NAME,
    ];

    pub fn push(&mut self, node: roxmltree::Node) -> Result<(), String> {
        match node.tag_name().name() {
            elements::LocalSimpleType::NAME => {
                if self.choice.is_some() {
                    return Err(format!(
                        "Two local simple/complex types in ElementModel group. {:?}",
                        node
                    ));
                }
                self.choice = Some(ElementModelChoice::SimpleType(
                    elements::LocalSimpleType::parse(node)?,
                ))
            }
            elements::LocalComplexType::NAME => {
                if self.choice.is_some() {
                    return Err(format!(
                        "Two local simple/complex types in ElementModel group. {:?}",
                        node
                    ));
                }
                self.choice = Some(ElementModelChoice::ComplexType(
                    elements::LocalComplexType::parse(node)?,
                ))
            }
            tag_name if IdentityConstraint::NAMES.contains(&tag_name) => self
                .identity_constraints
                .push(IdentityConstraint::parse(node)?),
            _ => {}
        }
        Ok(())
    }
}
