use crate::model::groups::complex_type_model::ComplexTypeModel;
use crate::model::LocalComplexType;
use crate::xml_to_xsd::utils::annotation_first;
use roxmltree::Node;
use crate::model::attributes::mixed::Mixed;
use std::convert::TryInto;

impl<'a> LocalComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self {
            annotation: annotation_first(node)?,
            model: ComplexTypeModel::parse(node)?,
            id: None,
            mixed: Mixed::default(),
            attributes: vec![],
        };

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "mixed" => res.mixed = attr.try_into()?,
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}
