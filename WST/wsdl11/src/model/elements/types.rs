use crate::model::complex_types::t_types;

// wsdl:types
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tTypes
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
// Attributes
//     None
//
// Used in
//     Group wsdl:anyTopLevelOptionalElement
//     Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
// Sample instance
//  <wsdl:types>
//     <xsd:schema targetNamespace="http://example.com/stockquote.xsd">
//        <xsd:element name="TradePriceRequest">
//           <xsd:complexType>
//              <xsd:all>
//                 <xsd:element name="tickerSymbol" type="string"/>
//              </xsd:all>
//           </xsd:complexType>
//        </xsd:element>
//        <xsd:element name="TradePrice">
//           <xsd:complexType>
//              <xsd:all>
//                 <wsdl:element name="price" type="float"/>
//              </xsd:all>
//           </xsd:complexType>
//        </xsd:element>
//     </xsd:schema>
//  </wsdl:types>
pub type Types<'a> = t_types::Types<'a>;