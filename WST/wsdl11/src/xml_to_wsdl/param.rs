use crate::model::complex_types::t_param::Param;
use roxmltree::Node;
use crate::xml_to_wsdl::documentation::documentation_only;

impl<'a> Param<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut message = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = Some(attr.into()),
                "message" => message = attr.into(),
                _ => res.attributes.push(attr.clone()),
            }
        }

        res.message = message
            .ok_or_else(|| format!("Message attribute required: {:?}", node))?
            .into();
        res.documentation = documentation_only(node)?;

        Ok(res)
    }
}