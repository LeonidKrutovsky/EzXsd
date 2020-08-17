use crate::model::complex_types::t_part::Part;
use crate::model::elements::ElementType;
use crate::model::{Documentation, Message};
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Message<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "name" => res.name = attr.into(),
                _ => return Err(format!("Invalid attribute. {:?}", node)),
            }
        }

        for ch in node.element_children() {
            match ch.wsdl_type()? {
                ElementType::Documentation => res.documentation = Some(Documentation::parse(ch)?),
                ElementType::Part => res.part = Part::parse(ch)?,
                _ => res.elements.push(ch),
            }
        }
        Ok(res)
    }
}
