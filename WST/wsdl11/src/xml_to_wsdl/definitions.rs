use crate::model::Definitions;
use roxmltree::Node;
use xsd10::model::simple_types::{AnyUri, NCName};
use crate::xml_to_wsdl::WsdlNode;
use crate::model::elements::ElementType;

impl<'a> Definitions<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Definitions::default();

        for attr in node.attributes() {
            match attr.name() {
                "targetNamespace" => res.target_namespace = Some(AnyUri::from(attr)),
                "name" => res.name = Some(NCName::from(attr)),
                x => {return Err(format!("Invalid attribute: {}", x))}
            }
        }

        use ElementType::*;
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.wsdl_type()? {
                Import => unimplemented!(),
                Types => unimplemented!(),
                Message => unimplemented!(),
                PortType => unimplemented!(),
                Binding => unimplemented!(),
                Service => unimplemented!(),
                x => {return Err(format!("Invalid child element: {:?}", x))}

            }
        }

        Ok(res)
    }
}