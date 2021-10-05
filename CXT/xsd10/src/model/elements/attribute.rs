use crate::model::{attributes, elements};
use xml_utils::element;

// xsd:attribute
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localAttributeType
// Properties: Local, Qualified
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
#[element(name = "attribute")]
pub struct LocalAttribute {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Option<elements::LocalSimpleType>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub use_: Option<attributes::Use>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub form: Option<attributes::Form>,
}

// xsd:attribute
// See http://www.w3.org/TR/xmlschema-1/#element-attribute.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelAttributeType
// Properties: Global, Qualified
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop

#[element(name = "attribute")]
pub struct TopLevelAttribute {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Option<elements::LocalSimpleType>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub type_: Option<attributes::Type>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
}


#[cfg(test)]
mod test {
    use super::TopLevelAttribute;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"
            <attribute name="NCName">
               <simpleType>
                  <restriction>
                     <minExclusive value="any text content"/>
                  </restriction>
               </simpleType>
            </attribute>
            "#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: TopLevelAttribute = TopLevelAttribute::parse(root).unwrap();
        assert_eq!(res.name.0.as_ref(), "NCName");
    }
}