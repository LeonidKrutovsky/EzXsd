use crate::model::attributes;
use crate::model::elements;
use xml_utils::complex_type;

// xsd:namedGroup
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]     from type xsd:annotated
//      Choice [1..1]
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements. This declaration is for an "all" group that is a child of xsd:group; its type disallows minOccurs and maxOccurs
//          xsd:choice
//          xsd:sequence
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used by
// Element xsd:group
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:namedGroup
#[complex_type()]
pub struct NamedGroup {
    pub annotation: Option<elements::Annotation>,
    pub content_choice: ContentChoice,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
}

#[derive(Debug)]
pub enum ContentChoice {
    All(elements::All),
    Choice(elements::SimpleChoice),
    Sequence(elements::SimpleSequence),
}

impl Default for ContentChoice {
    fn default() -> Self {
        unimplemented!()
    }
}
