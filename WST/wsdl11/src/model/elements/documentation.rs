use crate::model::complex_types::t_documentation;

// wsdl:documentation
// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tDocumentation
// Properties: Local, Qualified
//
// Content
//     Any text (mixed) content, intermingled with:
//     Any element [0..*] Namespace: ##any, Process Contents: lax
// Attributes
//     None
//
// Used in
//     Type wsdl:tBinding via extension of wsdl:tDocumented (Element wsdl:binding)
//     Type wsdl:tBindingOperation via extension of wsdl:tDocumented (Element wsdl:operation)
//     Type wsdl:tBindingOperationFault via extension of wsdl:tDocumented (Element wsdl:fault)
//     Type wsdl:tBindingOperationMessage via extension of wsdl:tDocumented (Elements wsdl:input, wsdl:output)
//     Type wsdl:tDefinitions via extension of wsdl:tDocumented (Element wsdl:definitions)
//     Type wsdl:tDocumented
//     Type wsdl:tExtensibleAttributesDocumented via extension of wsdl:tDocumented
//     Type wsdl:tExtensibleDocumented via extension of wsdl:tDocumented
//     Type wsdl:tFault via extension of wsdl:tDocumented (Element wsdl:fault)
//     Type wsdl:tImport via extension of wsdl:tDocumented (Element wsdl:import)
//     Type wsdl:tMessage via extension of wsdl:tDocumented (Element wsdl:message)
//     Type wsdl:tOperation via extension of wsdl:tDocumented (Element wsdl:operation)
//     Type wsdl:tParam via extension of wsdl:tDocumented (Elements wsdl:input, wsdl:output)
//     Type wsdl:tPart via extension of wsdl:tDocumented (Element wsdl:part)
//     Type wsdl:tPort via extension of wsdl:tDocumented (Element wsdl:port)
//     Type wsdl:tPortType via extension of wsdl:tDocumented (Element wsdl:portType)
//     Type wsdl:tService via extension of wsdl:tDocumented (Element wsdl:service)
//     Type wsdl:tTypes via extension of wsdl:tDocumented (Element wsdl:types)
// Sample instance
//     <wsdl:documentation>My first service</wsdl:documentation>
pub type Documentation<'a> = t_documentation::Documentation<'a>;
