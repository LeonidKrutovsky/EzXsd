use crate::model::Part;
use crate::xml_to_wsdl::documentation::documentation_only;
use roxmltree::Node;
use xsd10::model::simple_types::NCName;

impl<'a> Part<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(attr.value().parse()?),
                "element" => res.element = Some(attr.value().parse()?),
                "type" => res.type_ = Some(attr.value().parse()?),
                _ => res.attributes.push(attr.clone()),
            }
        }

        res.name = name.ok_or_else(|| format!("Name attribute required: {:?}", node))?;
        res.documentation = documentation_only(node)?;

        Ok(res)
    }
}
