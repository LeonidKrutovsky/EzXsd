use crate::model::complex_types::t_binding_operation_message::BindingOperationMessage;
use crate::model::complex_types::t_param::Param;

// wsdl:input
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tParam
// Properties: Local, Qualified
//
// Content
// wsdl:documentation [0..1]       from type wsdl:tDocumented
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
// name	[0..1]	    xsd:NCName
// message	[1..1]	    xsd:QName
//
// Used in
//     Group wsdl:request-response-or-one-way-operation
//     Group wsdl:solicit-response-or-notification-operation
//     Type wsdl:tOperation via reference to wsdl:request-response-or-one-way-operation (Element wsdl:operation)
//     Type wsdl:tOperation via reference to wsdl:solicit-response-or-notification-operation (Element wsdl:operation)
// Sample instance
//     <wsdl:input message="tns:GetLastTradePriceInput"/>
pub type Input<'a> = Param<'a>;

// wsdl:input
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Type: wsdl:tBindingOperationMessage
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]  from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax  from type wsdl:tExtensibleDocumented
// Attributes
// name	[0..1]	xsd:NCName
// Used in
//     Type wsdl:tBindingOperation (Element wsdl:operation)
// Sample instance
//     <wsdl:input message="tns:GetLastTradePriceInput"/>

pub type BindingOperationInput<'a> = BindingOperationMessage<'a>;
