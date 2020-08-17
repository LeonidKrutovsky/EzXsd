use crate::model::complex_types::t_part;

// wsdl:part
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tPart
// Properties: Local, Qualified
//
// Content
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
// Attributes
//     Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
//     name	[1..1]	xsd:NCName
//     element	[0..1]	xsd:QName
//     type	[0..1]	xsd:QName
// Used in
//     Type wsdl:tMessage (Element wsdl:message)
// Sample instance
//     <wsdl:part name="body"
//     element="xsd1:TradePriceRequest"/>
pub type Part<'a> = t_part::Part<'a>;
