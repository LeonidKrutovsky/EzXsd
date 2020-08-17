use crate::model::elements::{wsdl_element_type, ElementType};

pub mod definitions;
pub mod documentation;
pub mod fault;
pub mod import;
pub mod input;

pub const WSDL_NS_URI: &str = "http://schemas.xmlsoap.org/wsdl/";

pub trait WsdlNode {
    fn wsdl_type(&self) -> Result<ElementType, String>;
}

impl WsdlNode for roxmltree::Node<'_, '_> {
    fn wsdl_type(&self) -> Result<ElementType, String> {
        if let Some(uri) = self.tag_name().namespace() {
            if uri != WSDL_NS_URI {
                return Err(format!(
                    "Invalid prefix for xsd element: {:?}",
                    self.tag_name()
                ));
            }
        }
        wsdl_element_type(self.tag_name().name())
    }
}
