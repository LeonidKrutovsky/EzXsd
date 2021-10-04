use crate::model::{attributes, elements};
use xml_utils::element;

// /xsd:maxLength
// See http://www.w3.org/TR/xmlschema-2/#element-maxLength.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:numFacet
// Properties: Global, Qualified
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
#[element(name = "maxLength")]
pub struct MaxLength {
    pub annotation: Option<elements::Annotation>,
    pub id: Option<attributes::Id>,
    #[default]
    pub fixed: attributes::FixedBool,
    #[default]
    pub value: attributes::NonNegativeValue,
    pub attributes: Vec<attributes::RawAttribute>,
}
