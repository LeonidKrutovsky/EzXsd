use crate::model::elements::ElementType;
use crate::model::{Annotation, LocalAttribute, LocalSimpleType, TopLevelAttribute};
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;
use crate::model::attributes::use_::Use;
use crate::model::attributes::id::Id;
use std::convert::TryInto;
use crate::model::attributes::name::Name;
use crate::model::attributes::ref_::Ref;
use crate::model::attributes::type_::Type;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::form::Form;

impl<'a> TopLevelAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut simple_type = None;
        let mut attributes = vec![];
        let mut id = None;
        let mut name: Option<Name> = None;
        let mut type_ = None;
        let mut default = None;
        let mut fixed = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child type of xsd:topLevelAttribute: {:?}",
                        node
                    ))
                }
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                "name" => name = Some(attr.try_into()?),
                "type" => type_ = Some(attr.try_into()?),
                "default" => default = Some(attr.try_into()?),
                "fixed" => fixed = Some(attr.try_into()?),
                _ => attributes.push(attr.clone()),
            };
        }

        let name =
            name.ok_or_else(|| format!("Name required for xsd:topLevelAttribute: {:?}", node))?;

        Ok(Self {
            annotation,
            simple_type,
            attributes,
            id,
            name,
            type_,
            default,
            fixed,
        })
    }
}

impl<'a> LocalAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut simple_type = None;
        let mut attributes = vec![];
        let mut id = None;
        let mut name: Option<Name> = None;
        let mut ref_: Option<Ref> = None;
        let mut type_: Option<Type> = None;
        let mut use_ = Some(Use::Optional);
        let mut default = None;
        let mut fixed = None;
        let mut form = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child type of xsd:localAttribute: {:?}",
                        node
                    ))
                }
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                Id::NAME => id = Some(attr.try_into()?),
                Name::NAME => name = Some(attr.try_into()?),
                Ref::NAME => ref_ = Some(attr.try_into()?),
                Type::NAME => type_ = Some(attr.try_into()?),
                Use::NAME => use_ = Some(attr.try_into()?),
                Default_::NAME => default = Some(attr.try_into()?),
                Fixed::NAME => fixed = Some(attr.try_into()?),
                Form::NAME => form = Some(attr.try_into()?),
                _ => attributes.push(attr.clone()),
            };
        }

        Ok(Self {
            annotation,
            simple_type,
            attributes,
            id,
            name,
            ref_,
            type_,
            use_,
            default,
            fixed,
            form,
        })
    }
}
