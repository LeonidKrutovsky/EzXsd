use crate::model::complex_types::{t_operation, t_binding_operation};

// wsdl:operation
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Other elements with the same name: wsdl:operation
// Type: wsdl:tOperation
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//         Choice [1..1]       from group wsdl:request-response-or-one-way-operation
//             Sequence [1..1]
//                 wsdl:input [1..1]
//                 Sequence [0..1]
//                     wsdl:output [1..1]
//                     wsdl:fault [0..*]
//
//             Sequence [1..1]     from group wsdl:solicit-response-or-notification-operation
//                 wsdl:output [1..1]
//                 Sequence [0..1]
//                     wsdl:input [1..1]
//                     wsdl:fault [0..*]
// Attributes
// name	        [1..1]	xsd:NCName
// parameterOrder	[0..1]	xsd:NMTOKENS
// Used in
//     Type wsdl:tPortType (Element wsdl:portType)
// Sample instance
//     <wsdl:operation name="GetLastTradePrice">
//         <wsdl:input message="tns:GetLastTradePriceInput"/>
//         <wsdl:output message="tns:GetLastTradePriceOutput"/>
//     </wsdl:operation>
pub type Operation<'a> = t_operation::Operation<'a>;


// wsdl:operation
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Other elements with the same name: wsdl:operation
// Type: wsdl:tBindingOperation
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]       from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax        from type wsdl:tExtensibleDocumented
//     wsdl:input [0..1]
//     wsdl:output [0..1]
//     wsdl:fault [0..*]
// Attributes
// name	[1..1]	xsd:NCName
// Used in
//     Type wsdl:tBinding (Element wsdl:binding)
// Sample instance
//     <wsdl:operation name="GetLastTradePrice">
//         <wsdl:input message="tns:GetLastTradePriceInput"/>
//         <wsdl:output message="tns:GetLastTradePriceOutput"/>
//     </wsdl:operation>
pub type BindingOperation<'a> = t_binding_operation::BindingOperation<'a>;