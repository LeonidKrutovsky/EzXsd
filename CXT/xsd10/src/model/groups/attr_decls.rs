use crate::model::elements::any_attribute::AnyAttribute;
use crate::model::elements::attribute::LocalAttribute;
use crate::model::elements::attribute_group::AttributeGroupRef;

// xsd:attrDecls
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
// Type xsd:namedAttributeGroup (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType (Element xsd:extension)
// Type xsd:simpleRestrictionType (Element xsd:restriction)
#[derive(Debug, Default)]
pub struct AttrDecls<'a> {
    pub attributes: Vec<LocalAttribute<'a>>,
    pub attribute_groups: Vec<AttributeGroupRef<'a>>,
    pub any_attribute: Option<AnyAttribute<'a>>,
}
