use crate::model::{BindingOperationFault, Fault};
use crate::xml_to_wsdl::documentation::{documentation_first, documentation_only};
use roxmltree::Node;
use std::any::Any;
use xsd10::model::simple_types::NCName;
use xsd10::model::simple_types::QName;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Fault<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = attr.into(),
                "message" => res.message = attr.into(),
                x => res.attributes.push(attr.clone()),
            }
        }

        res.documentation = documentation_only(node)?;

        Ok(res)
    }
}

impl<'a> BindingOperationFault<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = attr.into(),
                x => return Err(format!("Invalid Attribute: {:?}", attr)),
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
