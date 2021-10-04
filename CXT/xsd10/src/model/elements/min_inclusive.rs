use crate::model::{attributes, elements};
use xml_utils::element;

// xsd:minInclusive
// See http://www.w3.org/TR/xmlschema-2/#element-minInclusive.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:facet
// Properties: Global, Qualified
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
#[element(name = "minInclusive")]
pub struct MinInclusive {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub value: attributes::Value,
    #[default]
    pub fixed: attributes::FixedBool,
}
