use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:selector
// See http://www.w3.org/TR/xmlschema-1/#element-selector.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]       from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// xpath	        [1..1]	Anonymous
//
// Used in
// Anonymous type of element xsd:keyref via extension of xsd:keybase
// Type xsd:keybase (Elements xsd:unique, xsd:key)
#[element(name = "selector")]
pub struct Selector {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub xpath: attributes::XPath,
}

fn test() -> &'static str{
    attributes::xpath::NAME
}
