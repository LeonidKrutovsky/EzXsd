// wsdl:tPortType
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     wsdl:operation [0..*]
// Attributes
//     Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
//     name	[1..1]	xsd:NCName
// Used by
//     Element wsdl:portType
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleAttributesDocumented
//             wsdl:tPortType

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawAttribute;
use xsd10::xsd_model::simple_types as xsd;
use crate::model::complex_types::t_operation::Operation;

#[derive(Default, Debug)]
pub struct PortType<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub operation: Operation<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub name: xsd::NCName<'a>,
}