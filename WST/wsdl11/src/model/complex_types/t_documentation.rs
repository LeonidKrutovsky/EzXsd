// wsdl:tDocumentation
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd

// Content
// Any text (mixed) content, intermingled with:
// Any element [0..*] Namespace: ##any, Process Contents: lax

// Attributes
// None

// Used by
// Element wsdl:documentation

use crate::model::RawElement;

#[derive(Default, Debug)]
pub struct Documentation<'a> {
    pub text: Option<&'a str>,
    pub elements: Vec<RawElement<'a>>,
}