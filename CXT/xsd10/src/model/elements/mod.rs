use crate::model::simple_types::QName;
use crate::model::attributes::AnyAttributes;
use roxmltree::Node;
use std::convert::{TryFrom, TryInto};

pub mod all;
pub mod annotation;
pub mod any;
pub mod any_attribute;
pub mod app_info;
pub mod attribute;
pub mod attribute_group;
pub mod choice;
pub mod complex_content;
pub mod complex_type;
pub mod documentation;
pub mod element;
pub mod enumeration;
pub mod extension;
pub mod field;
pub mod fraction_digits;
pub mod group;
pub mod import;
pub mod include;
pub mod key;
pub mod key_ref;
pub mod length;
pub mod list;
pub mod max_exclusive;
pub mod max_inclusive;
pub mod max_length;
pub mod min_exclusive;
pub mod min_inclusive;
pub mod min_length;
pub mod notation;
pub mod pattern;
pub mod redefine;
pub mod restriction;
pub mod schema;
pub mod selector;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod total_digits;
pub mod union;
pub mod unique;
pub mod white_space;

#[derive(Debug, PartialEq)]
pub enum ElementType {
    All,
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute,
    AttributeGroup,
    Choice,
    ComplexContent,
    ComplexType,
    Documentation,
    Element,
    Enumeration,
    Extension,
    Field,
    FractionDigits,
    Group,
    Import,
    Include,
    Key,
    KeyRef,
    Length,
    List,
    MaxExclusive,
    MaxInclusive,
    MaxLength,
    MinExclusive,
    MinInclusive,
    MinLength,
    Notation,
    Pattern,
    Redefine,
    Restriction,
    Schema,
    Selector,
    Sequence,
    SimpleContent,
    SimpleType,
    TotalDigits,
    Union,
    Unique,
    WhiteSpace,

    Unknown(String),
}

pub fn xsd_element_type(name: &str) -> Result<ElementType, String> {
    use ElementType::*;
    let element = match name {
        "all" => All,
        annotation::Annotation::TOKEN => Annotation,
        "any" => Any,
        "anyAttribute" => AnyAttribute,
        "appInfo" => AppInfo,
        "attribute" => Attribute,
        "attributeGroup" => AttributeGroup,
        "choice" => Choice,
        "complexContent" => ComplexContent,
        "complexType" => ComplexType,
        "documentation" => Documentation,
        "element" => Element,
        "enumeration" => Enumeration,
        "extension" => Extension,
        "field" => Field,
        "fractionDigits" => FractionDigits,
        "group" => Group,
        "import" => Import,
        "include" => Include,
        "key" => Key,
        "keyRef" => KeyRef,
        "length" => Length,
        "list" => List,
        "maxExclusive" => MaxExclusive,
        "maxInclusive" => MaxInclusive,
        "maxLength" => MaxLength,
        "minExclusive" => MinExclusive,
        "minInclusive" => MinInclusive,
        "minLength" => MinLength,
        "notation" => Notation,
        "pattern" => Pattern,
        "redefine" => Redefine,
        "restriction" => Restriction,
        "schema" => Schema,
        "selector" => Selector,
        "sequence" => Sequence,
        "simpleContent" => SimpleContent,
        "simpleType" => SimpleType,
        "totalDigits" => TotalDigits,
        "union" => Union,
        "unique" => Unique,
        "whiteSpace" => WhiteSpace,
        _ => return Err(format!("Invalid xsd element name: {}", name)),
    };
    Ok(element)
}

#[derive(Debug, Default)]
pub struct RawElement {
    name: QName,
    attributes: AnyAttributes,
    text: Option<String>,
}

impl TryFrom<roxmltree::Node<'_, '_>> for RawElement {
    type Error = String;

    fn try_from(value: Node<'_, '_>) -> Result<Self, Self::Error> {
        let name = value.tag_name().name().parse()?;
        let prefix = if let Some(p) = value.tag_name().namespace() {
            Some(p.parse()?)
        } else {
            None
        };
        Ok(
            Self {
            name: QName{
                prefix,
                name,
            },
            attributes: AnyAttributes(value.attributes().iter().map(|a| a.try_into()).collect::<Result<Vec<_>, _>>()?),
            text: value.text().map(String::from)
        }
        )
    }
}

#[derive(Debug, Default)]
pub struct AnyElements(pub Vec<RawElement>);

impl AnyElements {
    pub fn push(&mut self, elem: RawElement) {
        self.0.push(elem);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TryFrom<roxmltree::Children<'_, '_>> for AnyElements{
    type Error = String;

    fn try_from(value: roxmltree::Children<'_, '_>) -> Result<Self, Self::Error> {
        Ok(
            AnyElements(
                value
                    .map(|n| n.try_into())
                    .collect::<Result<Vec<_>, _>>()?
            )
        )
    }
}
