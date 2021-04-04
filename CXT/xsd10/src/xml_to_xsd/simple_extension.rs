use crate::model::groups::attr_decls::AttrDecls;
use crate::model::SimpleExtension;
use crate::xml_to_xsd::utils::annotation_first;
use crate::xml_to_xsd::ElementChildren;
use roxmltree::Node;
use std::convert::TryInto;

impl SimpleExtension {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut iter = node.element_children();

        res.annotation = annotation_first(node)?;
        res.attr_decls = AttrDecls::parse(&mut iter)?;

        let mut base = None;

        for attr in node.attributes() {
            match attr.name() {
                "base" => base = Some(attr.try_into()?),
                "id" => res.id = Some(attr.try_into()?),
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        res.base = base.ok_or_else(|| {
            format!(
                "Attribute 'base' required for xsd:simpleExtension element: {:?}",
                node
            )
        })?;
        Ok(res)
    }
}
