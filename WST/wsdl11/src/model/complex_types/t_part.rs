// wsdl:tPart
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  wsdl:documentation [0..1]       from type wsdl:tDocumented

// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
// name	[1..1]	        xsd:NCName
// element	[0..1]	        xsd:QName
// type	[0..1]	        xsd:QName

// Used by
//     Element wsdl:part
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleAttributesDocumented
//             wsdl:tPart

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawAttribute;
use xsd10::xsd_model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Part<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub name: xsd::NCName<'a>,
    pub element: Option<xsd::QName<'a>>,
    pub type_: Option<xsd::QName<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
}
