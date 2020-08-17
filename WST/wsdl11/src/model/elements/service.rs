use crate::model::complex_types::t_service;

// wsdl:service
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tService
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     wsdl:port [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
// Identity constraints
//     Type	Name	Selector	Field(s)
//     unique	port	wsdl:port	@name
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//     <wsdl:service name="StockQuoteService">
//         <wsdl:documentation>My first service</wsdl:documentation>
//         <wsdl:port name="StockQuotePort" binding="tns:StockQuoteBinding">
//             <soap:address location="http://example.com/stockquote"/>
//         </wsdl:port>
//     </wsdl:service>
pub type Service<'a> = t_service::Service<'a>;
