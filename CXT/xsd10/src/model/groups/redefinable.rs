use crate::model::elements::attribute_group::AttributeGroup;
use crate::model::elements::complex_type::TopLevelComplexType;
use crate::model::elements::group::Group;
use crate::model::elements::simple_type::TopLevelSimpleType;

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
