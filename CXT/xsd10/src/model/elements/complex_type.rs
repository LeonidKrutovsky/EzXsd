use crate::model::complex_types::local_complex_type;
use crate::model::complex_types::top_level_complex_type;
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
pub struct TopLevelComplexType(pub top_level_complex_type::TopLevelComplexType);

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
