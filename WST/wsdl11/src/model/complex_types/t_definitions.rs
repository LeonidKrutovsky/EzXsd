// wsdl:tDefinitions
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     Choice [0..*]   from group wsdl:anyTopLevelOptionalElement
//         wsdl:import
//         wsdl:types
//         wsdl:message
//         wsdl:portType
//         wsdl:binding
//         wsdl:service
// Attributes
//     targetNamespace	[0..1]	xsd:anyURI
//     name	[0..1]	xsd:NCName
// Used by
//     Element wsdl:definitions
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tDefinitions

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::{Binding, Import, Message, PortType, RawElement, Service, Types};
use std::collections::HashMap;
use xsd10::model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Definitions<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub content: DefinitionsContent<'a>,
    pub target_namespace: Option<xsd::AnyUri<'a>>,
    pub name: Option<xsd::NCName<'a>>,
}

// Helper for identity constraints
#[derive(Default, Debug)]
pub struct DefinitionsContent<'a> {
    pub imports: HashMap<&'a str, Import<'a>>,
    pub types: Vec<Types<'a>>,
    pub messages: HashMap<&'a str, Message<'a>>,
    pub port_types: HashMap<&'a str, PortType<'a>>,
    pub bindings: HashMap<&'a str, Binding<'a>>,
    pub services: HashMap<&'a str, Service<'a>>,
}

impl<'a> DefinitionsContent<'a> {
    pub fn add_import(&mut self, imp: Import<'a>) -> Result<(), String> {
        if let Some(val) = self.imports.insert(&imp.namespace.0, imp) {
            Err(format!("Duplicate import namespace: {}", val.namespace.0))
        } else {
            Ok(())
        }
    }

    pub fn add_types(&mut self, ty: Types<'a>) -> Result<(), String> {
        self.types.push(ty);
        Ok(())
    }

    pub fn add_message(&mut self, mes: Message<'a>) -> Result<(), String> {
        if let Some(val) = self.messages.insert(mes.name.0, mes) {
            Err(format!("Duplicate message name: {}", val.name.0))
        } else {
            Ok(())
        }
    }

    pub fn add_port_type(&mut self, pt: PortType<'a>) -> Result<(), String> {
        if let Some(val) = self.port_types.insert(pt.name.0, pt) {
            Err(format!("Duplicate port type name: {}", val.name.0))
        } else {
            Ok(())
        }
    }

    pub fn add_binding(&mut self, bin: Binding<'a>) -> Result<(), String> {
        if let Some(val) = self.bindings.insert(bin.name.0, bin) {
            Err(format!("Duplicate binding name: {}", val.name.0))
        } else {
            Ok(())
        }
    }

    pub fn add_service(&mut self, svc: Service<'a>) -> Result<(), String> {
        if let Some(val) = self.services.insert(svc.name.0, svc) {
            Err(format!("Duplicate service name: {}", val.name.0))
        } else {
            Ok(())
        }
    }
}
