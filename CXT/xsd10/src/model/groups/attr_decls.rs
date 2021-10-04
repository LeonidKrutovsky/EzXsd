use crate::model::elements::any_attribute::AnyAttribute;
use crate::model::elements::attribute::LocalAttribute;
use crate::model::elements::attribute_group::AttributeGroupRef;

// xsd:attrDecls
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
// Type xsd:namedAttributeGroup (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType (Element xsd:extension)
// Type xsd:simpleRestrictionType (Element xsd:restriction)
#[derive(Debug, Default)]
pub struct AttrDecls {
    pub attributes: Vec<LocalAttribute>,
    pub attribute_groups: Vec<AttributeGroupRef>,
    pub any_attribute: Option<AnyAttribute>,
}

impl AttrDecls {
    pub const NAMES: &'static [&'static str] = &[
                LocalAttribute::NAME, AttributeGroupRef::NAME, AnyAttribute::NAME
            ];

    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.any_attribute.is_none() && self.attribute_groups.is_empty()
    }

    pub fn push(&mut self, node: roxmltree::Node<'_, '_>) -> Result<(), String> {
        match node.tag_name().name() {
            LocalAttribute::NAME => self.attributes.push(LocalAttribute::parse(node)?),
            AttributeGroupRef::NAME => self.attribute_groups.push(AttributeGroupRef::parse(node)?),
            AnyAttribute::NAME => self.any_attribute = Some(AnyAttribute::parse(node)?),
            _ => Err(format!("Unexpected node in AttrDecls group: {:?}", node))?
        }
        Ok(())
    }
}
