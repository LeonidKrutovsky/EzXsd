// wsdl:tExtensibleAttributesDocumented
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Properties: Abstract
//
// Content
// wsdl:documentation [0..1]       from type wsdl:tDocumented
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
// Used by
//     Element wsdl:fault via derived type wsdl:tFault
//     Element wsdl:import via derived type wsdl:tImport
//     Element wsdl:input via derived type wsdl:tParam
//     Element wsdl:output via derived type wsdl:tParam
//     Element wsdl:part via derived type wsdl:tPart
//     Element wsdl:portType via derived type wsdl:tPortType
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleAttributesDocumented
//             extended by wsdl:tImport
//             extended by wsdl:tPart
//             extended by wsdl:tPortType
//             extended by wsdl:tParam
//             extended by wsdl:tFault

use crate::model::complex_types::t_documented::Documented;

pub trait ExtensibleAttributesDocumented<'a>: Documented<'a> {
    fn required(&self) -> Option<bool>;
}
