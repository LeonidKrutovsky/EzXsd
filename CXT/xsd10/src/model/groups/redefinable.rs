use crate::model::elements::attribute_group::AttributeGroup;
use crate::model::elements::complex_type::TopLevelComplexType;
use crate::model::elements::group::Group;
use crate::model::elements::simple_type::TopLevelSimpleType;
use roxmltree::Node;

// xsd:redefinable
// This group is for the elements which can self-redefine.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleType
//      xsd:complexType
//      xsd:group
//      xsd:attributeGroup
//
// Used in
// Anonymous type of element xsd:redefine
// Group xsd:schemaTop
#[derive(Debug)]
pub enum Redefinable {
    SimpleType(Box<TopLevelSimpleType>),
    ComplexType(Box<TopLevelComplexType>),
    Group(Box<Group>),
    AttributeGroup(Box<AttributeGroup>),
}

impl Default for Redefinable {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Redefinable {
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        match node.tag_name().name() {
            TopLevelSimpleType::NAME => Ok(Self::SimpleType(Box::new(TopLevelSimpleType::parse(node)?))),
            TopLevelComplexType::NAME => Ok(Self::ComplexType(Box::new(TopLevelComplexType::parse(node)?))),
            Group::NAME => Ok(Self::Group(Box::new(Group::parse(node)?))),
            AttributeGroup::NAME => Ok(Self::AttributeGroup(Box::new(AttributeGroup::parse(node)?))),
            _ => Err(String::default())
        }

    }
}
