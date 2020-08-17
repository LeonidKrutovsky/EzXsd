use crate::model::complex_types::t_binding_operation_message::BindingOperationMessage;
use crate::xml_to_wsdl::documentation::documentation_first;
use roxmltree::Node;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> BindingOperationMessage<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = Some(attr.into()),
                _ => return Err(format!("Invalid Attribute: {:?}", attr)),
            }
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
