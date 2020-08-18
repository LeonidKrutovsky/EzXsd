// wsdl:definitions
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tDefinitions
// Properties: Global, Qualified
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
// Identity constraints
//     Type	    Name	    Selector	    Field(s)
//     key	    message	w   sdl:message	    @name
//     key	    portType	wsdl:portType	@name
//     key	    binding	    wsdl:binding	@name
//     key	    service	    wsdl:service	@name
//     key	    import	    wsdl:import	    @namespace
use crate::model::complex_types::t_definitions;

pub type Definitions<'a> = t_definitions::Definitions<'a>;



// Sample instance
//       <wsdl:definitions name="StockQuote" targetNamespace="http://example.com/stockquote.wsdl">
//           <wsdl:types>
//               <xsd:schema targetNamespace="http://example.com/stockquote.xsd">
//                   <xsd:element name="TradePriceRequest">
//                       <xsd:complexType>
//                           <xsd:all>
//                               <xsd:element name="tickerSymbol" type="string"/>
//                           </xsd:all>
//                       </xsd:complexType>
//                   </xsd:element>
//                   <xsd:element name="TradePrice">
//                       <xsd:complexType>
//                           <xsd:all>
//                               <wsdl:element name="price" type="float"/>
//                           </xsd:all>
//                       </xsd:complexType>
//                   </xsd:element>
//               </xsd:schema>
//           </wsdl:types>
//           <wsdl:message name="GetLastTradePriceInput">
//               <wsdl:part name="body" element="xsd1:TradePriceRequest"/>
//           </wsdl:message>
//           <wsdl:message name="GetLastTradePriceOutput">
//               <wsdl:part name="body" element="xsd1:TradePrice"/>
//           </wsdl:message>
//           <wsdl:portType name="StockQuotePortType">
//               <wsdl:operation name="GetLastTradePrice">
//                   <wsdl:input message="tns:GetLastTradePriceInput"/>
//                   <wsdl:output message="tns:GetLastTradePriceOutput"/>
//               </wsdl:operation>
//           </wsdl:portType>
//           <wsdl:binding name="StockQuoteSoapBinding" type="tns:StockQuotePortType">
//               <soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
//               <wsdl:operation name="GetLastTradePrice">
//                   <soap:operation soapAction="http://example.com/GetLastTradePrice"/>
//                   <wsdl:input>
//                       <soap:body use="literal"/>
//                   </wsdl:input>
//                   <wsdl:output>
//                       <soap:body use="literal"/>
//                   </wsdl:output>
//               </wsdl:operation>
//           </wsdl:binding>
//           <wsdl:service name="StockQuoteService">
//               <wsdl:documentation>My first service</wsdl:documentation>
//               <wsdl:port name="StockQuotePort" binding="tns:StockQuoteBinding">
//                   <soap:address location="http://example.com/stockquote"/>
//               </wsdl:port>
//           </wsdl:service>
//       </wsdl:definitions>
