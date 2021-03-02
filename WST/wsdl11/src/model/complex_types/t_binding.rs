// wsdl:tBinding
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element        [0..*] Namespace: ##other, Process Contents: lax   from type wsdl:tExtensibleDocumented
//     wsdl:operation     [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
//     type	[1..1]	xsd:QName
// Used by
//     Element wsdl:binding
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tBinding

use crate::model::complex_types::t_binding_operation::BindingOperation;
use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawElement;
use xsd10::model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Binding<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub operations: Vec<BindingOperation<'a>>,
    pub name: xsd::NCName,
    pub type_: xsd::QName,
}
