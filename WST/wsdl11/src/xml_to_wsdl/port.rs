use crate::model::elements::ElementType;
use crate::model::{Documentation, Port};
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::model::simple_types::{NCName, QName};
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Port<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;
        let mut binding = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(attr.value().parse()?),
                "binding" => binding = Some(QName::from(attr.value())),
                _ => return Err(format!("Invalid attribute. {:?}", node)),
            }
        }

        res.name = name.ok_or_else(|| format!("Name attribute required: {:?}", node))?;

        res.binding = binding.ok_or_else(|| format!("Binding attribute required: {:?}", node))?;

        for ch in node.element_children() {
            match ch.wsdl_type() {
                Ok(ElementType::Documentation) => {
                    res.documentation = Some(Documentation::parse(ch)?)
                }
                _ => res.elements.push(ch),
            }
        }

        Ok(res)
    }
}
