use crate::model::complex_types::t_import;

// wsdl:import
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tImport
// Properties: Local, Qualified
//
// Content
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//
// Attributes
//     Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
//     namespace	    [1..1]	xsd:anyURI
//     location	    [1..1]	xsd:anyURI
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//     <wsdl:import namespace="http://example.com/stockquote/schemas"
//     location="http://example.com/stockquote/stockquote.xsd"/>
pub type Import<'a> = t_import::Import<'a>;
