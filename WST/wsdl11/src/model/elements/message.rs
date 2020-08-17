use crate::model::complex_types::t_message;

// wsdl:message
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tMessage
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     wsdl:part [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
// Identity constraints
//     Type	    Name	Selector	Field(s)
//     unique	part	wsdl:part	@name
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//     <wsdl:message name="GetLastTradePriceInput">
//     <wsdl:part name="body" element="xsd1:TradePriceRequest"/>
//     </wsdl:message>
pub type Message<'a> = t_message::Message<'a>;
