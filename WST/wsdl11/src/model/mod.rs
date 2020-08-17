pub mod complex_types;
pub mod elements;
pub mod groups;

pub type RawElement<'a> = roxmltree::Node<'a, 'a>;
pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
