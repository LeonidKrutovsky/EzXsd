use crate::model::{attributes, elements};
use xml_utils::element;

// xsd:anyAttribute
// See http://www.w3.org/TR/xmlschema-1/#element-anyAttribute.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:wildcard
// Properties: Global, Qualified
//
// Used in
// Group xsd:attrDecls
// Group xsd:complexTypeModel via reference to xsd:attrDecls
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:namedAttributeGroup via reference to xsd:attrDecls (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:simpleRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
#[element(name = "anyAttribute")]
pub struct AnyAttribute {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub namespace: attributes::Namespace,
    pub process_contents: attributes::ProcessContents,
}
