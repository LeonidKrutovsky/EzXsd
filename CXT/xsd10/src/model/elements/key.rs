use crate::model::{attributes, elements};
use xml_utils::element;

// See http://www.w3.org/TR/xmlschema-1/#element-key.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:keybase
// Properties: Global, Qualified
//
// Used in
// Group xsd:identityConstraint
// Group xsd:elementModel via reference to xsd:identityConstraint
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
#[element(name = "key")]
pub struct Key {
    pub annotation: Option<elements::Annotation>,
    pub selector: elements::Selector,
    pub fields: Vec<elements::Field>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
}
