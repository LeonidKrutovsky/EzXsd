// wsdl:tDocumented
// This type is extended by  component types to allow them to be documented

// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/

// Schema document: wsdl11.xsd
// Content
// wsdl:documentation [0..1]
// Attributes
// None

// Used by
//      Element wsdl:binding via derived type wsdl:tBinding
//      Element wsdl:definitions via derived type wsdl:tDefinitions
//      Element wsdl:fault via derived type wsdl:tBindingOperationFault
//      Element wsdl:fault via derived type wsdl:tFault
//      Element wsdl:import via derived type wsdl:tImport
//      Element wsdl:input via derived type wsdl:tBindingOperationMessage
//      Element wsdl:input via derived type wsdl:tParam
//      Element wsdl:message via derived type wsdl:tMessage
//      Element wsdl:operation via derived type wsdl:tBindingOperation
//      Element wsdl:operation via derived type wsdl:tOperation
//      Element wsdl:output via derived type wsdl:tBindingOperationMessage
//      Element wsdl:output via derived type wsdl:tParam
//      Element wsdl:part via derived type wsdl:tPart
//      Element wsdl:port via derived type wsdl:tPort
//      Element wsdl:portType via derived type wsdl:tPortType
//      Element wsdl:service via derived type wsdl:tService
//      Element wsdl:types via derived type wsdl:tTypes

// Type inheritance chain
//  wsdl:tDocumented
//      extended by wsdl:tExtensibleAttributesDocumented
//          extended by wsdl:tImport
//          extended by wsdl:tPart
//          extended by wsdl:tPortType
//          extended by wsdl:tParam
//          extended by wsdl:tFault
//      extended by wsdl:tExtensibleDocumented
//          extended by wsdl:tDefinitions
//          extended by wsdl:tTypes
//          extended by wsdl:tMessage
//          extended by wsdl:tOperation
//          extended by wsdl:tBinding
//          extended by wsdl:tBindingOperationMessage
//          extended by wsdl:tBindingOperationFault
//          extended by wsdl:tBindingOperation
//          extended by wsdl:tService
//          extended by wsdl:tPort

use crate::model::complex_types::t_documentation::Documentation;

pub trait Documented<'a> {
    fn documentation(&self) -> &Documentation<'a>;
}