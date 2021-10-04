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
    #[sequence_group]
    pub model: groups::ComplexTypeModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    #[default]
    pub abstract_: attributes::Abstract,
    pub final_: Option<attributes::Final>,
    pub block: Option<attributes::DerivationBlock>,
    #[default]
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
pub struct LocalComplexType {
    pub annotation: Option<elements::Annotation>,
    #[sequence_group]
    pub model: groups::ComplexTypeModel,
    pub id: Option<attributes::Id>,
    #[default]
    pub mixed: attributes::Mixed,
    pub attributes: Vec<attributes::RawAttribute>,
}

#[cfg(test)]
mod test {
    use crate::model::groups::ComplexTypeModel;
    use crate::model::TopLevelComplexType;

    #[test]
    fn test_parse_top_level_complex_type() {
        let xsd = r###"
        <xs:schema
            xmlns:tt="http://www.onvif.org/ver10/schema"
            xmlns:xs="http://www.w3.org/2001/XMLSchema"
            elementFormDefault="qualified"
            version="19.12">
                <xs:complexType name="IntRectangle">
                    <xs:annotation>
                        <xs:documentation>Rectangle defined by lower left corner position and size. Units are pixel.</xs:documentation>
                    </xs:annotation>
                    <xs:attribute name="x" type="xs:int" use="required"/>
                    <xs:attribute name="y" type="xs:int" use="required"/>
                    <xs:attribute name="width" type="xs:int" use="required"/>
                    <xs:attribute name="height" type="xs:int" use="required"/>
                </xs:complexType>
        </xs:schema>
        "###;
        let doc = roxmltree::Document::parse(xsd).unwrap();
        let root = doc.root_element();
        let tlct = root.first_element_child().unwrap();

        let res = TopLevelComplexType::parse(tlct).unwrap();
        assert_eq!(res.name.0.as_ref(), "IntRectangle");
        assert_eq!(res.annotation.unwrap().documentations.len(), 1);
        if let ComplexTypeModel::Content(_, ad) = &res.model {
            assert_eq!(ad.attributes[0].name.as_ref().unwrap().0.as_ref(), "x");
            assert_eq!(ad.attributes[1].name.as_ref().unwrap().0.as_ref(), "y");
            assert_eq!(ad.attributes[2].type_.as_ref().unwrap().0.name(), "int");
        } else {
            panic!("test_parse_top_level_complex_type failed!");
        }
    }
}
