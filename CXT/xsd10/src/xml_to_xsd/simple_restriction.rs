use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::model::SimpleRestriction;
use crate::xml_to_xsd::utils::annotation_first;
use crate::xml_to_xsd::ElementChildren;
use roxmltree::Node;
use std::convert::TryInto;

impl<'a> SimpleRestriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_first(node)?;

        let skip = if res.annotation.is_some() { 1 } else { 0 };
        let mut iter = node.element_children().skip(skip);

        res.model = SimpleRestrictionModel::parse(&mut iter)?;
        res.attr_decls = AttrDecls::parse(iter)?;

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
