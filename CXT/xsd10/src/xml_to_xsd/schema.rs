use crate::model::elements::ElementType;
use crate::model::groups::schema_top::SchemaTop;
use crate::model::simple_types::any_uri::AnyUri;
use crate::model::simple_types::block_set::BlockSet;
use crate::model::simple_types::form_choice::FormChoice;
use crate::model::simple_types::full_derivation_set::FullDerivationSet;
use crate::model::simple_types::id::Id;
use crate::model::simple_types::language::Language;
use crate::model::simple_types::token::Token;
use crate::model::Annotation;
use crate::model::Import;
use crate::model::Include;
use crate::model::Schema;
use crate::xml_to_xsd::{ElementChildren, XsdNode};
use roxmltree::{Document, Node};

pub fn parse_document<'a>(doc: &'a Document) -> Result<Schema<'a>, String> {
    let schema_node = doc.root_element();
    Schema::parse(schema_node)
}

impl<'a> Schema<'a> {
    pub fn parse(schema_node: Node<'a, '_>) -> Result<Self, String> {
        let mut schema = Schema::default();

        for attr in schema_node.attributes() {
            match attr.name() {
                "targetNamespace" => schema.target_namespace = Some(AnyUri(attr.value())),
                "version" => schema.version = Some(Token(attr.value())),
                "finalDefault" => {
                    schema.final_default = Some(FullDerivationSet::parse(attr.value())?)
                }
                "blockDefault" => schema.block_default = Some(BlockSet::parse(attr.value())?),
                "attributeFormDefault" => {
                    schema.attribute_form_default = FormChoice::parse(attr.value())?
                }
                "elementFormDefault" => {
                    schema.element_form_default = FormChoice::parse(attr.value())?
                }
                "id" => schema.id = Some(Id(attr.value())),
                "lang" => schema.lang = Some(Language(attr.value())),
                _ => schema.attributes.push(attr.clone()),
            };
        }

        for ch in schema_node.element_children() {
            match ch.xsd_type()? {
                ElementType::Include => schema.includes.push(Include::parse(ch)?),
                ElementType::Import => schema.imports.push(Import::parse(ch)?),
                ElementType::Redefine => unimplemented!("Not present in ONVIF"),
                ElementType::Annotation => {
                    if let Some(val) = schema.content.last_mut() {
                        val.1.push(Annotation::parse(ch)?)
                    } else {
                        schema.annotations.push(Annotation::parse(ch)?)
                    }
                }
                x => schema.content.push((SchemaTop::parse(ch, x)?, vec![])),
            }
        }

        Ok(schema)
    }
}
