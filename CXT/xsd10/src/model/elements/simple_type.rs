use crate::model::{attributes, elements, groups};
use xml_utils::element;

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

#[cfg(test)]
mod test {

    use crate::model::groups::simple_derivation::SimpleDerivation;
    use crate::model::TopLevelSimpleType;

    #[test]
    fn test_top_level_simple_type_parse() {
        let doc = roxmltree::Document::parse(
            r##"<simpleType id="ID" name="Type1" final="#all" a='b' b='a'>
                        <list itemType="itemType" />
                    </simpleType>"##,
        )
        .unwrap();
        let root = doc.root_element();
        let res: TopLevelSimpleType = TopLevelSimpleType::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.name.0.as_ref(), "Type1");
        match &res.content_choice {
            SimpleDerivation::List(x) => {
                assert_eq!(x.item_type.as_ref().unwrap().0.name(), "itemType")
            }
            _ => unreachable!("test failed!"),
        }
    }
}

