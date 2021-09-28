use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:union
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Type: Anonymous
//
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      from type xsd:annotated
//          xsd:annotation [0..1]
//          xsd:simpleType [0..*]
//  Attributes
//      Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
//      id	            [0..1]	xsd:ID		                                            from type xsd:annotated
//      memberTypes	    [0..1]	Anonymous

// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)

// Sample instance
// <xsd:union>
//    <xsd:simpleType>
//       <xsd:restriction>
//          <xsd:simpleType>...
//          </xsd:simpleType>
//          <xsd:minExclusive value="any text content">...
//          </xsd:minExclusive>
//       </xsd:restriction>
//    </xsd:simpleType>
// </xsd:union>

#[element(name = "union")]
pub struct Union {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Vec<elements::LocalSimpleType>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub member_types: attributes::MemberTypes,
}

#[cfg(test)]
mod test {
    use crate::model::Union;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<union id="ID" memberTypes="Type1 xs:Type2" a='b' b='a'>
                <simpleType>
                    <list itemType="ListOfType" />
                </simpleType>
                <simpleType>
                    <list itemType="ListOfType1" />
                </simpleType>
                <simpleType>
                    <list itemType="ListOfType2" />
                </simpleType>
            </union>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Union::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.member_types.0.0.len(), 2);
        assert_eq!(res.member_types.0.0[0].name.as_ref(), "Type1");
        assert_eq!(res.member_types.0.0[1].name.as_ref(), "Type2");
        assert_eq!(res.member_types.0.0[1].prefix(), Some("xs"));
        assert_eq!(res.simple_type.len(), 3);
    }
}
