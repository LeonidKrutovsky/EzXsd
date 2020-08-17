// wsdl:tTypes
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax        from type wsdl:tExtensibleDocumented
// Attributes
//     None
//
// Used by
//     Element wsdl:types
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tTypes

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawElement;

#[derive(Default, Debug)]
pub struct Types<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
}