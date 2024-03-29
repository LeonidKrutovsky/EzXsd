pub mod attributes;
pub mod complex_types;
pub mod elements;
pub mod groups;
pub mod simple_types;

pub use elements::all::*;
pub use elements::annotation::*;
pub use elements::any::*;
pub use elements::any_attribute::*;
pub use elements::app_info::*;
pub use elements::attribute::*;
pub use elements::attribute_group::*;
pub use elements::choice::*;
pub use elements::complex_content::*;
pub use elements::complex_type::*;
pub use elements::documentation::*;
pub use elements::element::*;
pub use elements::enumeration::*;
pub use elements::extension::*;
pub use elements::field::*;
pub use elements::fraction_digits::*;
pub use elements::group::*;
pub use elements::import::*;
pub use elements::include::*;
pub use elements::key::*;
pub use elements::key_ref::*;
pub use elements::length::*;
pub use elements::list::*;
pub use elements::max_exclusive::*;
pub use elements::max_inclusive::*;
pub use elements::max_length::*;
pub use elements::min_exclusive::*;
pub use elements::min_inclusive::*;
pub use elements::min_length::*;
pub use elements::notation::*;
pub use elements::pattern::*;
pub use elements::redefine::*;
pub use elements::restriction::*;
pub use elements::schema::*;
pub use elements::selector::*;
pub use elements::sequence::*;
pub use elements::simple_content::*;
pub use elements::simple_type::*;
pub use elements::total_digits::*;
pub use elements::union::*;
pub use elements::unique::*;
pub use elements::white_space::*;
use std::rc::Rc;

pub const XSD_NS_URI: &str = "http://www.w3.org/2001/XMLSchema";

pub struct Namespace {
    pub uri: Rc<String>,
    pub alias: Option<String>,
}
