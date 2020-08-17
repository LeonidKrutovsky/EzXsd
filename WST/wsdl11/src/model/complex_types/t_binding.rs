// wsdl:tBinding
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax   from type wsdl:tExtensibleDocumented
//     wsdl:operation [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
//     type	[1..1]	xsd:QName
// Used by
//     Element wsdl:binding
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tBinding

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::complex_types::t_part::Part;
use crate::model::RawElement;
use xsd10::xsd_model::simple_types as xsd;
use crate::model::complex_types::t_operation::Operation;

#[derive(Default, Debug)]
pub struct Binding<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub operation: Vec<Operation<'a>>,
    pub name: xsd::NCName<'a>,
    pub type_: xsd::QName<'a>,
}
