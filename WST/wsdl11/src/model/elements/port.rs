use crate::model::complex_types::t_port;

// wsdl:port
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tPort
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
// Attributes
//     name	[1..1]	xsd:NCName
//     binding	[1..1]	xsd:QName
// Used in
//     Type wsdl:tService (Element wsdl:service)
// Sample instance
//     <wsdl:port name="StockQuotePort"
//         binding="tns:StockQuoteBinding">
//         <soap:address location="http://example.com/stockquote"/>
//     </wsdl:port>
pub type Port<'a> = t_port::Port<'a>;