use crate::model::{RawElement};
use crate::model::attributes::source::Source;
use crate::model::simple_types::Language;
use crate::model::attributes::AnyAttributes;

// xsd:documentation
// See http://www.w3.org/TR/xmlschema-1/#element-documentation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Any text (mixed) content, intermingled with:
//  Any element     [0..*]  Namespace: ##any, Process Contents: lax
//
// Attributes
// source	        [0..1]	    xsd:anyURI
// xml:lang	        [0..1]	    Anonymous
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax
//
// Used in
// Anonymous type of element xsd:annotation
#[derive(Default, Debug)]
pub struct Documentation<'a> {
    pub text: Option<&'a str>,
    pub elements: Vec<RawElement<'a>>,
    pub source: Option<Source>,
    pub lang: Option<Language>,
    pub attributes: AnyAttributes,
}
