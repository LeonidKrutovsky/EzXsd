use crate::model::Documentation;
use roxmltree::Node;
use xsd10::xml_to_xsd::ElementChildren;
use crate::xml_to_wsdl::WsdlNode;
use crate::model::elements::ElementType;

impl<'a> Documentation<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        if !node.attributes().is_empty() {
            return Err(format!("Attributes not allowed"));
        }

        Ok(Self{
            text: node.text(),
            elements: node.children().filter(|n| n.is_element()).collect()
        })
    }
}

pub fn documentation_only<'a>(node: Node<'a, '_>) -> Result<Option<Documentation<'a>>, String> {
    if node.element_children().count() > 1 {
        return Err(format!("Only one child allowed!"));
    }
    if let Some(n) = node.first_element_child() {
        if n.wsdl_type()? == ElementType::Documentation {
            Ok(Some(Documentation::parse(n)?))
        } else {
            Err(format!("Only wsdl:documentation allowed! {:?}", n))
        }
    } else {
        Ok(None)
    }
}

pub fn documentation_first<'a>(node: Node<'a, '_>) -> Result<Option<Documentation<'a>>, String> {
    if let Some(n) = node.first_element_child() {
        if n.wsdl_type()? == ElementType::Documentation {
            Ok(Some(Documentation::parse(n)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}