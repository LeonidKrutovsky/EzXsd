use crate::model::complex_types::t_binding_operation_fault;
use crate::model::complex_types::t_fault;

// wsdl:fault
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Other elements with the same name: wsdl:fault
// Type: wsdl:tFault
// Properties: Local, Qualified
//
// Content
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
// Attributes
//     Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
//     name	        [1..1]	xsd:NCName
//     message	        [1..1]	xsd:QName
// Used in
//     Group wsdl:request-response-or-one-way-operation
//     Group wsdl:solicit-response-or-notification-operation
//     Type wsdl:tOperation via reference to wsdl:request-response-or-one-way-operation (Element wsdl:operation)
//     Type wsdl:tOperation via reference to wsdl:solicit-response-or-notification-operation (Element wsdl:operation)
// Sample instance
//     <wsdl:fault name="NCName" message="QName">
//     <wsdl:documentation>Any text, intermingled with:
//     <!--any element-->
//     </wsdl:documentation>
//     </wsdl:fault>

pub type Fault<'a> = t_fault::Fault<'a>;

// wsdl:fault
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Other elements with the same name: wsdl:fault
// Type: wsdl:tBindingOperationFault
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
// Attributes
//     name	[1..1]	xsd:NCName
// Used in
//     Type wsdl:tBindingOperation (Element wsdl:operation)
// Sample instance
//     <wsdl:fault name="NCName">
//     <wsdl:documentation>Any text, intermingled with:
//     <!--any element-->
//     </wsdl:documentation>
//     <!--any element-->
//     </wsdl:fault>

pub type BindingOperationFault<'a> = t_binding_operation_fault::BindingOperationFault<'a>;
