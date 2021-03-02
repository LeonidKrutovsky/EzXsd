use crate::model::complex_types::t_param::Param;
use crate::xml_to_wsdl::documentation::documentation_only;
use roxmltree::Node;
use xsd10::model::simple_types::QName;

impl<'a> Param<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut message = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = Some(attr.value().parse()?),
                "message" => message = Some(attr.value().parse::<QName>()?),
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
