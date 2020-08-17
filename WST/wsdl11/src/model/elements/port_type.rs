use crate::model::complex_types::t_port_type;

// wsdl:portType
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tPortType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     wsdl:operation [0..*]
// Attributes
//     Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
//     name	[1..1]	xsd:NCName
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//     <wsdl:portType name="StockQuotePortType">
//      <wsdl:operation name="GetLastTradePrice">
//          <wsdl:input message="tns:GetLastTradePriceInput"/>
//          <wsdl:output message="tns:GetLastTradePriceOutput"/>
//      </wsdl:operation>
//     </wsdl:portType>
pub type PortType<'a> = t_port_type::PortType<'a>;