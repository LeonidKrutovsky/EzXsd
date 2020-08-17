// http://schemas.xmlsoap.org/wsdl/http/
pub mod http;
//http://schemas.xmlsoap.org/wsdl/mime/
pub mod mime;
//http://schemas.xmlsoap.org/wsdl/soap/
pub mod soap;
// http://schemas.xmlsoap.org/wsdl/
pub mod complex_types;
pub mod elements;
pub mod groups;

pub use elements::binding::*;
pub use elements::definitions::*;
pub use elements::documentation::*;
pub use elements::fault::*;
pub use elements::import::*;
pub use elements::input::*;
pub use elements::message::*;
pub use elements::operation::*;
pub use elements::output::*;
pub use elements::part::*;
pub use elements::port::*;
pub use elements::port_type::*;
pub use elements::service::*;
pub use elements::types::*;

pub type RawElement<'a> = roxmltree::Node<'a, 'a>;
pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
