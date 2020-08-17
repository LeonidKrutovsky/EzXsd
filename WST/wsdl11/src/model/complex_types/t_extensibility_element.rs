// wsdl:tExtensibilityElement
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Properties: Abstract
//
// Content
//     Empty content
// Attributes
//     wsdl:required	[0..1]	xsd:boolean
// Used by
//     Element http:address via derived type http:addressType
//     Element http:binding via derived type http:bindingType
//     Element http:operation via derived type http:operationType
//     Element mime:content via derived type mime:contentType
//     Element mime:mimeXml via derived type mime:tMimeXml
//     Element mime:multipartRelated via derived type mime:multipartRelatedType
//     Element soap:address via derived type soap:tAddress
//     Element soap:binding via derived type soap:tBinding
//     Element soap:body via derived type soap:tBody
//     Element soap:fault via derived type soap:tFault
//     Element soap:header via derived type soap:tHeader
//     Element soap:operation via derived type soap:tOperation
// Type inheritance chain
//     wsdl:tExtensibilityElement
//         extended by http:addressType
//         extended by http:bindingType
//         extended by http:operationType
//         extended by mime:contentType
//         extended by mime:multipartRelatedType
//         extended by mime:tMimeXml
//         extended by soap:tBinding
//         extended by soap:tOperation
//         extended by soap:tBody
//             restricted by soap:tFaultRes
//                 extended by soap:tFault
//         extended by soap:tHeader
//         extended by soap:tAddress

pub trait ExtensibilityElement {
    fn required(&self) -> Option<bool>;
}
