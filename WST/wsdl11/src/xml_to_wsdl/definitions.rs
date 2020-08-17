use crate::model::elements::ElementType;
use crate::model::groups::any_top_level_optional_element::AnyTopLevelOptionalElement;
use crate::model::{Binding, Definitions, PortType};
use crate::model::{Import, Message, Types};
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::model::simple_types::{AnyUri, NCName};

impl<'a> Definitions<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "targetNamespace" => res.target_namespace = Some(AnyUri::from(attr)),
                "name" => res.name = Some(NCName::from(attr)),
                x => return Err(format!("Invalid attribute: {}", x)),
            }
        }

        res.content = node
            .children()
            .filter(|n| n.is_element())
            .map(parse_content)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }
}

fn parse_content<'a>(node: Node<'a, '_>) -> Result<AnyTopLevelOptionalElement<'a>, String> {
    match node.wsdl_type()? {
        ElementType::Import => Ok(AnyTopLevelOptionalElement::Import(Import::parse(node)?)),
        ElementType::Types => Ok(AnyTopLevelOptionalElement::Types(Types::parse(node)?)),
        ElementType::Message => Ok(AnyTopLevelOptionalElement::Message(Message::parse(node)?)),
        ElementType::PortType => Ok(AnyTopLevelOptionalElement::PortType(PortType::parse(node)?)),
        ElementType::Binding => Ok(AnyTopLevelOptionalElement::Binding(Binding::parse(node)?)),
        ElementType::Service => unimplemented!(),
        x => return Err(format!("Invalid child element: {:?}", x)),
    }
}
