use crate::model::elements::ElementType;
use crate::model::Annotation;
use crate::model::LocalSimpleType;
use crate::model::Union;
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;
use std::convert::TryInto;

impl Union {
    pub fn parse(node: Node<'_, '_>) -> Result<Union, String> {
        let mut res = Union::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => res.simple_type.push(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child node for xsd:union element: {:?}",
                        node
                    ))
                }
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "memberTypes" => res.member_types = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }
        Ok(res)
    }
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
