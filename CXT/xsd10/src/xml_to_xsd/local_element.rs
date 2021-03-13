use crate::model::groups::element_model::ElementModel;
use crate::model::LocalElement;
use crate::xml_to_xsd::utils::annotation_first;
use roxmltree::Node;
use std::convert::TryInto;

impl<'a> LocalElement<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self {
            annotation: annotation_first(node)?,
            model: ElementModel::parse(node)?,
            ..Default::default()
        };

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "name" => res.name = Some(attr.try_into()?),
                "ref" => res.ref_ = Some(attr.try_into()?),
                "type" => res.type_ = Some(attr.try_into()?),
                "minOccurs" => res.min_occurs = attr.try_into()?,
                "maxOccurs" => res.max_occurs = attr.try_into()?,
                "default" => res.default = Some(attr.try_into()?),
                "fixed" => res.fixed = Some(attr.try_into()?),
                "nillable" => res.nillable = attr.try_into()?,
                "block" => res.block = Some(attr.try_into()?),
                "form" => res.form = Some(attr.try_into()?),

                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}
