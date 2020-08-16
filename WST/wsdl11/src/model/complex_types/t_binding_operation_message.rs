// wsdl:tBindingOperationMessage
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax  from type wsdl:tExtensibleDocumented
// Attributes
// name	[0..1]	xsd:NCName
// Used by
//     Element wsdl:input
//     Element wsdl:output
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tBindingOperationMessage

use xsd10::xsd_model::simple_types as xsd;
use crate::model::{RawAttribute, RawElement};
use crate::model::complex_types::t_documentation::Documentation;

#[derive(Default, Debug)]
pub struct BindingOperationMessage<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub name: Option<xsd::NCName<'a>>,
}