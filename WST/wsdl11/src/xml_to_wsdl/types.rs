use crate::model::Types;
use crate::xml_to_wsdl::documentation::documentation_first;
use roxmltree::Node;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Types<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        if !node.attributes().is_empty() {
            return Err(format!("Attributes not allowed for wsdl:types! {:?}", node));
        }

        if let Some(doc) = documentation_first(node)? {
            res.documentation = Some(doc);
            res.elements = node.element_children().skip(1).collect();
        } else {
            res.elements = node.element_children().collect();
        }

        Ok(res)
    }
}
