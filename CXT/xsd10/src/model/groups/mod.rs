pub mod attr_decls;
pub mod complex_type_model;
pub mod element_model;
pub mod facets;
pub mod identity_constraint;
pub mod nested_particle;
pub mod redefinable;
pub mod schema_top;
pub mod simple_derivation;
pub mod simple_restriction_model;
pub mod type_def_particle;

pub use attr_decls::AttrDecls;
pub use complex_type_model::ComplexTypeModel;
pub use element_model::ElementModel;
pub use nested_particle::NestedParticle;
pub use redefinable::Redefinable;
pub use schema_top::SchemaTop;
pub use simple_derivation::SimpleDerivation;
pub use simple_restriction_model::SimpleRestrictionModel;
pub use type_def_particle::TypeDefParticle;

use crate::model::elements;

#[derive(Debug)]
pub enum ComplexContentChoice {
    Restriction(Box<elements::ComplexRestriction>),
    Extension(Box<elements::Extension>),
}

impl ComplexContentChoice {
    pub const NAMES: &'static [&'static str] = &[
        elements::ComplexRestriction::NAME,
        elements::Extension::NAME,
    ];

    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        match node.tag_name().name() {
            elements::ComplexRestriction::NAME => Ok(Self::Restriction(Box::new(
                elements::ComplexRestriction::parse(node)?,
            ))),

            elements::Extension::NAME => {
                Ok(Self::Extension(Box::new(elements::Extension::parse(node)?)))
            }

            _ => Err(format!("Unexpected node: {:#?}", node)),
        }
    }
}

#[derive(Debug)]
pub enum SimpleContentChoice {
    Restriction(Box<elements::SimpleRestriction>),
    Extension(Box<elements::SimpleExtension>),
}

#[derive(Debug)]
pub enum ContentChoice {
    All(elements::All),
    Choice(elements::SimpleChoice),
    Sequence(elements::SimpleSequence),
}

impl ContentChoice {
    pub const NAMES: &'static [&'static str] = &[
        elements::All::NAME,
        elements::SimpleChoice::NAME,
        elements::SimpleSequence::NAME,
    ];

    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        match node.tag_name().name() {
            elements::All::NAME => Ok(Self::All(elements::All::parse(node)?)),
            elements::SimpleChoice::NAME => Ok(Self::Choice(elements::SimpleChoice::parse(node)?)),
            elements::SimpleSequence::NAME => {
                Ok(Self::Sequence(elements::SimpleSequence::parse(node)?))
            }

            _ => Err(format!("Unexpected node: {:#?}", node)),
        }
    }
}
