use crate::model::elements::complex_content::ComplexContent;
use crate::model::elements::simple_content::SimpleContent;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::type_def_particle::TypeDefParticle;

// xsd:complexTypeModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleContent
//      xsd:complexContent
//      Sequence [1..1]
//          Choice [0..1]       from group xsd:typeDefParticle
//              xsd:group
//              xsd:all         An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//              xsd:choice
//              xsd:sequence
//          Choice [0..*]       from group xsd:attrDecls
//              xsd:attribute
//              xsd:attributeGroup
//          xsd:anyAttribute [0..1]
//
// Used in
// Type xsd:complexType
// Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)
#[derive(Debug)]
pub enum ComplexTypeModel {
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    Content(Option<TypeDefParticle>, Box<AttrDecls>),
}

impl Default for ComplexTypeModel {
    fn default() -> Self {
        Self::SimpleContent(SimpleContent::default())
    }
}

impl ComplexTypeModel {
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        match node.tag_name().name() {
            SimpleContent::NAME => Ok(Self::SimpleContent(SimpleContent::parse(node)?)),
            ComplexContent::NAME => Ok(Self::ComplexContent(ComplexContent::parse(node)?)),
            Group::NAME => Ok(Self::Group(Box::new(Group::parse(node)?))),
            AttributeGroup::NAME => Ok(Self::AttributeGroup(Box::new(AttributeGroup::parse(node)?))),
            _ => Err(String::default())
        }
    }
}
