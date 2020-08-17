// wsdl:tService
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax        from type wsdl:tExtensibleDocumented
//     wsdl:port [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
//
// Used by
//     Element wsdl:service
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tService

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::complex_types::t_port::Port;
use crate::model::RawElement;
use xsd10::xsd_model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Service<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub port: Port<'a>,
    pub name: xsd::NCName<'a>,
}
