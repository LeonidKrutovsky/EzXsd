// wsdl:tDefinitions
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     Choice [0..*]   from group wsdl:anyTopLevelOptionalElement
//         wsdl:import
//         wsdl:types
//         wsdl:message
//         wsdl:portType
//         wsdl:binding
//         wsdl:service
// Attributes
//     targetNamespace	[0..1]	xsd:anyURI
//     name	[0..1]	xsd:NCName
// Used by
//     Element wsdl:definitions
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tDefinitions

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::complex_types::t_part::Part;
use crate::model::RawElement;
use xsd10::xsd_model::simple_types as xsd;
use crate::model::groups::any_top_level_optional_element::AnyTopLevelOptionalElement;

#[derive(Default, Debug)]
pub struct Definitions<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub content: Vec<AnyTopLevelOptionalElement<'a>>,
    pub target_namespace: xsd::AnyUri<'a>,
    pub name: Option<xsd::NCName<'a>>,
}
