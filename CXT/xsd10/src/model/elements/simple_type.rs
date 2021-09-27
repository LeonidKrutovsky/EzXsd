use xml_utils::element;
use crate::model::{elements, groups, attributes};

// xsd:simpleType
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:localSimpleType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:list
// Anonymous type of element xsd:union
// Group xsd:elementModel
// Group xsd:simpleRestrictionModel
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Type xsd:localAttributeType (Element xsd:attribute)
// Type xsd:topLevelAttributeType (Element xsd:attribute)
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
#[element(name = "simpleType")]
pub struct LocalSimpleType {
    pub annotation: Option<elements::Annotation>,
    pub content_choice: groups::SimpleDerivation,
    pub id: Option<attributes::Id>,
    pub attributes: Vec<attributes::RawAttribute>,
}

// xsd:simpleType
// See http://www.w3.org/TR/xmlschema-2/#element-simpleType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:topLevelSimpleType
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
#[element(name = "simpleType")]
pub struct TopLevelSimpleType {
    pub annotation: Option<elements::Annotation>,
    pub content_choice: groups::SimpleDerivation,
    pub id: Option<attributes::Id>,
    pub final_: Option<attributes::SimpleFinal>,
    pub name: attributes::Name,
    pub attributes: Vec<attributes::RawAttribute>,
}
