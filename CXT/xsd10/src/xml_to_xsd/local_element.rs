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
                "id" => res.id = Some(attr.value().parse()?),
                "name" => res.name = Some(attr.value().parse()?),
                "ref" => res.ref_ = Some(attr.value().parse()?),
                "type" => res.type_ = Some(attr.value().parse()?),
                "minOccurs" => res.min_occurs = attr.try_into()?,
                "maxOccurs" => res.max_occurs = attr.try_into()?,
                "default" => res.default = Some(attr.value()),
                "fixed" => res.fixed = Some(attr.value()),
                "nillable" => {
                    res.nillable = attr.value().parse().map_err(|_| {
                        format!("Invalid 'nillable' attribute value: {}", attr.value())
                    })?
                }
                "block" => res.block = Some(attr.value().parse()?),
                "form" => res.form = Some(attr.value().parse()?),

                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}
