use crate::model::elements::attribute::TopLevelAttribute;
use crate::model::elements::attribute_group::AttributeGroup;
use crate::model::elements::complex_type::TopLevelComplexType;
use crate::model::elements::element::TopLevelElement;
use crate::model::elements::group::Group;
use crate::model::elements::notation::Notation;
use crate::model::elements::simple_type::TopLevelSimpleType;
use crate::model::simple_types::ncname::NCName;
use std::rc::Rc;

// xsd:schemaTop
// This group is for the elements which occur freely at the top level of schemas. All of their types are based on the "annotated" type by extension.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      from group xsd:redefinable
//          xsd:simpleType
//          xsd:complexType
//          xsd:group
//          xsd:attributeGroup
//      xsd:element
//      xsd:attribute
//      xsd:notation
//
// Used in
// Anonymous type of element xsd:schema
#[derive(Debug)]
pub enum SchemaTop {
    SimpleType(Rc<TopLevelSimpleType>),
    ComplexType(Rc<TopLevelComplexType>),
    Group(Rc<Group>),
    AttributeGroup(Rc<AttributeGroup>),
    Element(Rc<TopLevelElement>),
    Attribute(Rc<TopLevelAttribute>),
    Notation(Rc<Notation>),
}

impl SchemaTop {
    pub fn name(&self) -> NCName {
        match self {
            SchemaTop::SimpleType(val) => val.name.0.clone(),
            SchemaTop::ComplexType(val) => val.0.name.0.clone(),
            SchemaTop::Group(val) => val.0.name.0.clone(),
            SchemaTop::AttributeGroup(val) => val.0.name.0.clone(),
            SchemaTop::Element(val) => val.0.name.0.clone(),
            SchemaTop::Attribute(val) => val.0.name.0.clone(),
            SchemaTop::Notation(val) => val.name.0.clone(),
        }
    }
}
