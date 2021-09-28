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
use xml_utils::group;
#[group()]
pub enum ComplexContentChoice {
    Restriction(Box<elements::ComplexRestriction>),
    Extension(Box<elements::Extension>),
}

#[group()]
pub enum SimpleContentChoice {
    Restriction(Box<elements::SimpleRestriction>),
    Extension(Box<elements::SimpleExtension>),
}

#[group()]
pub enum ContentChoice {
    All(elements::All),
    Choice(elements::SimpleChoice),
    Sequence(elements::SimpleSequence),
}
