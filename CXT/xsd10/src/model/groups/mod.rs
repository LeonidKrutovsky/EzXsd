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

pub use redefinable::Redefinable;
pub use simple_restriction_model::SimpleRestrictionModel;

use crate::model::elements;

#[derive(Debug)]
pub enum ComplexContentChoice {
    Restriction(Box<elements::ComplexRestriction>),
    Extension(Box<elements::Extension>),
}

impl Default for ComplexContentChoice {
    fn default() -> Self {
        unimplemented!()
    }
}


#[derive(Debug)]
pub enum SimpleContentChoice {
    Restriction(Box<elements::SimpleRestriction>),
    Extension(Box<elements::SimpleExtension>),
}

impl Default for SimpleContentChoice {
    fn default() -> Self {
        unimplemented!()
    }
}