use crate::model::simple_types;
use xml_utils::attribute;

#[attribute(name = "lang")]
pub struct Language(pub simple_types::Language);
