use crate::model::attributes;
use crate::model::attributes::RawAttribute;
use crate::model::elements;
use xml_utils::complex_type;

// xsd:keybase
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      xsd:selector [1..1]
//      xsd:field [1..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used by
// Element xsd:key
// Element xsd:unique
// Element xsd:keyref via derived anonymous type
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:keybase
// extended by Anonymous type of element xsd:keyref
#[complex_type()]
pub struct KeyBase {
    pub annotation: Option<elements::Annotation>,
    pub selector: elements::Selector,
    pub fields: Vec<elements::Field>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
}

impl KeyBase {
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        let mut annotation: Option<elements::Annotation> = None;
        let mut selector: Option<elements::Selector> = None;
        let mut fields: Vec<elements::Field> = vec![];
        let mut attributes: Vec<attributes::RawAttribute> = vec![];
        let mut id: Option<attributes::Id> = None;
        let mut name: Option<attributes::Name> = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.tag_name().name() {
                elements::Annotation::NAME => annotation = Some(elements::Annotation::parse(ch)?),
                elements::Selector::NAME => selector = Some(elements::Selector::parse(ch)?),
                elements::Field::NAME => fields.push(elements::Field::parse(ch)?),
                _ => Err(format!("err"))?,
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                attributes::Id::NAME => id = Some(attributes::Id::parse(attr)?),
                attributes::Name::NAME => name = Some(attributes::Name::parse(attr)?),
                _ => attributes.push(RawAttribute::parse(attr)?),
            }
        }

        Ok(Self {
            annotation,
            selector: selector.ok_or_else(|| format!(""))?,
            fields,
            attributes,
            id,
            name: name.ok_or_else(|| format!(""))?,
        })
    }
}
