use crate::model::simple_types::any_uri::AnyUri;
use crate::model::simple_types::language::Language;
use crate::model::{RawAttribute, RawElement};

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
    pub source: Option<AnyUri>,
    pub lang: Option<Language>,
    pub attributes: Vec<RawAttribute<'a>>,
}
