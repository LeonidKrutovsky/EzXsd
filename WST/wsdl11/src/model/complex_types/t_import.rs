// wsdl:tImport
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//
// wsdl:documentation [0..1]       from type wsdl:tDocumented
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	    from type wsdl:tExtensibleAttributesDocumented
// namespace	[1..1]	    xsd:anyURI
// location	[1..1]	    xsd:anyURI
// Used by
//     Element wsdl:import
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleAttributesDocumented
//             wsdl:tImport

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::RawAttribute;
use xsd10::xsd_model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Import<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub namespace: xsd::AnyUri<'a>,
    pub location: xsd::AnyUri<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
}
