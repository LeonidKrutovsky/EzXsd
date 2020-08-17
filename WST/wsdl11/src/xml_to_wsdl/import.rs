use crate::model::Import;
use crate::xml_to_wsdl::documentation::documentation_only;
use roxmltree::Node;

impl<'a> Import<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "namespace" => res.namespace = attr.into(),
                "location" => res.location = attr.into(),
                _ => res.attributes.push(attr.clone()),
            }
        }

        res.documentation = documentation_only(node)?;
        Ok(res)
    }
}
