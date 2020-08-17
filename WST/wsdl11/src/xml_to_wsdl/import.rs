use crate::model::complex_types::t_documentation::Documentation;
use crate::model::elements::ElementType;
use crate::model::Import;
use crate::xml_to_wsdl::documentation::documentation_only;
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Import<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "namespace" => res.namespace = AnyUri::from(attr),
                "location" => res.name = AnyUri::from(attr),
                x => res.attributes.push(attr.clone()),
            }
        }

        res.documentation = documentation_only(node)?;
        Ok(res)
    }
}
