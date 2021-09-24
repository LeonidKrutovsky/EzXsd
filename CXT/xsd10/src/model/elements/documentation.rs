use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

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
#[element(name = "documentation")]
pub struct Documentation {
    pub text: Option<String>,
    pub elements: Vec<elements::RawElement>,
    pub source: Option<attributes::Source>,
    pub lang: Option<attributes::Language>,
    pub attributes: Vec<attributes::RawAttribute>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Documentation::NAME, "documentation");
    }
}
