// wsdl:tBindingOperation
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     wsdl:input  [0..1]
//     wsdl:output [0..1]
//     wsdl:fault  [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
//
// Used by
//     Element wsdl:operation
//
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tBindingOperation

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawElement;
use xsd10::xsd_model::simple_types as xsd;
use crate::model::elements::input::BindingOperationInput;
use crate::model::elements::output::BindingOperationOutput;
use crate::model::elements::fault::BindingOperationFault;

#[derive(Default, Debug)]
pub struct BindingOperation<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub input: Option<BindingOperationInput<'a>>,
    pub output: Option<BindingOperationOutput<'a>>,
    pub faults: Vec<BindingOperationFault<'a>>,
    pub name: xsd::NCName<'a>,
}
