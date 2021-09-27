use crate::model::complex_types::local_complex_type;
use crate::model::{attributes, elements, groups};
use xml_utils::element;

// See http://www.w3.org/TR/xmlschema-1/#element-complexType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelComplexType
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
#[element(name = "complexType")]
pub struct TopLevelComplexType {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ComplexTypeModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub abstract_: attributes::Abstract,
    pub final_: Option<attributes::Final>,
    pub block: Option<attributes::DerivationBlock>,
    pub mixed: attributes::Mixed,
}

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localComplexType
// Properties: Local, Qualified
//
// Used in
// Group xsd:elementModel
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
#[element(name = "complexType")]
pub struct LocalComplexType(pub local_complex_type::LocalComplexType);
