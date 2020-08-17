// wsdl:tExtensibleDocumented
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Properties: Abstract
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax
//
// Attributes
//     None
//
// Used by
//     Element wsdl:binding via derived type wsdl:tBinding
//     Element wsdl:definitions via derived type wsdl:tDefinitions
//     Element wsdl:fault via derived type wsdl:tBindingOperationFault
//     Element wsdl:input via derived type wsdl:tBindingOperationMessage
//     Element wsdl:message via derived type wsdl:tMessage
//     Element wsdl:operation via derived type wsdl:tBindingOperation
//     Element wsdl:operation via derived type wsdl:tOperation
//     Element wsdl:output via derived type wsdl:tBindingOperationMessage
//     Element wsdl:port via derived type wsdl:tPort
//     Element wsdl:service via derived type wsdl:tService
//     Element wsdl:types via derived type wsdl:tTypes
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             extended by wsdl:tDefinitions
//             extended by wsdl:tTypes
//             extended by wsdl:tMessage
//             extended by wsdl:tOperation
//             extended by wsdl:tBinding
//             extended by wsdl:tBindingOperationMessage
//             extended by wsdl:tBindingOperationFault
//             extended by wsdl:tBindingOperation
//             extended by wsdl:tService
//             extended by wsdl:tPort

use crate::model::complex_types::t_documented::Documented;
use crate::model::RawElement;

pub trait ExtensibleDocumented<'a> : Documented<'a> {
    fn elements(&self) -> Vec<RawElement<'a>>;
}