use crate::model::complex_types::t_binding;

// wsdl:binding
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tBinding
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     wsdl:operation [0..*]
// Attributes
//     name	[1..1]	xsd:NCName
//     type	[1..1]	xsd:QName
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//   <wsdl:binding name="StockQuoteSoapBinding" type="tns:StockQuotePortType">
//      <soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
//      <wsdl:operation name="GetLastTradePrice">
//         <soap:operation soapAction="http://example.com/GetLastTradePrice"/>
//         <wsdl:input>
//            <soap:body use="literal"/>
//         </wsdl:input>
//         <wsdl:output>
//            <soap:body use="literal"/>
//         </wsdl:output>
//      </wsdl:operation>
//   </wsdl:binding>
pub type Binding<'a> = t_binding::Binding<'a>;