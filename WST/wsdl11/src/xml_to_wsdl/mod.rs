use crate::model::elements::{wsdl_element_type, ElementType};

pub mod binding;
pub mod binding_operation_message;
pub mod definitions;
pub mod documentation;
pub mod fault;
pub mod import;
pub mod message;
pub mod operation;
pub mod param;
pub mod port;
pub mod part;
pub mod port_type;
pub mod types;
pub mod service;

pub const WSDL_NS_URI: &str = "http://schemas.xmlsoap.org/wsdl/";
pub const SOAP_NS_URI: &str = "http://schemas.xmlsoap.org/wsdl/soap12/";

pub trait WsdlNode {
    fn wsdl_type(&self) -> Result<ElementType, String>;
}

impl WsdlNode for roxmltree::Node<'_, '_> {
    fn wsdl_type(&self) -> Result<ElementType, String> {
        if let Some(uri) = self.tag_name().namespace() {
            if uri != WSDL_NS_URI {
                return Err(format!(
                    "Invalid prefix for wsdl element: {:?}",
                    self.tag_name()
                ));
            }
        }
        wsdl_element_type(self.tag_name().name())
    }
}
