use crate::model::elements::RawElement;
use crate::model::attributes::source::Source;
use crate::model::attributes::AnyAttributes;

// xsd:appinfo
// See http://www.w3.org/TR/xmlschema-1/#element-appinfo.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// Any text (mixed) content, intermingled with:
// Any element [0..*] Namespace: ##any, Process Contents: lax
//
// Attributes
// source	        [0..1]	xsd:anyURI
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used in
// Anonymous type of element xsd:annotation
#[derive(Debug, Default)]
pub struct AppInfo {
    pub text: Option<String>,
    pub elements: Vec<RawElement>,
    pub source: Option<Source>,
    pub attributes: AnyAttributes,
}
