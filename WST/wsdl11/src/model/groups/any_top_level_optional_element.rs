// wsdl:anyTopLevelOptionalElement
// Any top level optional element allowed to appear more then once - any child of definitions element except wsdl:types. Any extensibility element is allowed in any place.
//
// Group information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Choice [1..1]
//     wsdl:import
//     wsdl:types
//     wsdl:message
//     wsdl:portType
//     wsdl:binding
//     wsdl:service
// Used in
//     Type wsdl:tDefinitions (Element wsdl:definitions)

use crate::model::elements::binding::Binding;
use crate::model::elements::import::Import;
use crate::model::elements::message::Message;
use crate::model::elements::port_type::PortType;
use crate::model::elements::service::Service;
use crate::model::elements::types::Types;

#[derive(Debug)]
pub enum AnyTopLevelOptionalElement<'a> {
    Import(Import<'a>),
    Types(Types<'a>),
    Message(Message<'a>),
    PortType(PortType<'a>),
    Binding(Binding<'a>),
    Service(Service<'a>),
}
