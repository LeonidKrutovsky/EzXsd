use crate::model::elements::attribute_group::AttributeGroup;
use crate::model::elements::complex_type::TopLevelComplexType;
use crate::model::elements::group::Group;
use crate::model::elements::simple_type::TopLevelSimpleType;
use xml_utils::group;

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
#[group()]
pub enum Redefinable {
    SimpleType(TopLevelSimpleType),
    ComplexType(TopLevelComplexType),
    Group(Group),
    AttributeGroup(AttributeGroup),
}
