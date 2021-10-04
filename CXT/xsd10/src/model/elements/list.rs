use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// itemType	        [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
#[element(name = "list")]
pub struct List {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Option<elements::LocalSimpleType>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub item_type: Option<attributes::ItemType>,
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<list id="ID" a='b' b='a'>
                <simpleType id="STN">
                    <list itemType="ListOfType" />
                </simpleType>
            </list>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: List = List::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert!(res.item_type.is_none());
        assert_eq!(res.simple_type.unwrap().id.unwrap().0.as_ref(), "STN");
    }
}

