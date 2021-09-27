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
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        let mut attr_decls = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.tag_name().name() {
                LocalAttribute::NAME => attr_decls.attributes.push(LocalAttribute::parse(node)?),
                AttributeGroupRef::NAME => attr_decls
                    .attribute_groups
                    .push(AttributeGroupRef::parse(node)?),
                AnyAttribute::NAME => attr_decls.any_attribute = Some(AnyAttribute::parse(node)?),
                _ => {}
            }
        }
        Ok(attr_decls)
    }
}
