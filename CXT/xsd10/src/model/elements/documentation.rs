use crate::model::elements;
use crate::model::attributes;

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

use xml_utils::element;
use std::convert::{TryFrom, TryInto};
use roxmltree::Node;

#[element(name = "documentation")]
pub struct Documentation {
    pub text: Option<String>,
    pub elements: elements::AnyElements,
    pub source: Option<attributes::Source>,
    pub lang: Option<attributes::Language>,
    pub attributes: attributes::AnyAttributes,
}

impl TryFrom<roxmltree::Node<'_, '_>> for Documentation {
    type Error = String;

    fn try_from(node: Node<'_, '_>) -> Result<Self, Self::Error> {
        let text = node.text().map(|s| s.to_string());
        let elements =node.children().try_into()?;
        let mut source: Option<attributes::Source> = None;
        let mut lang: Option<attributes::Language> = None;
        let mut attributes = attributes::AnyAttributes::default();

        for attribute in node.attributes() {
            match attribute.name() {
                attributes::Source::NAME => source = Some(attribute.try_into()?),
                attributes::Language::NAME => lang = Some(attribute.try_into()?),
                _ => attributes.push(attribute.try_into()?),
            };
        }

        Ok(Self {
            text,
            elements,
            source,
            lang,
            attributes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Documentation::NAME, "documentation");
    }
}