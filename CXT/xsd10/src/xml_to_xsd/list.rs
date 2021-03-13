use crate::model::elements::ElementType;
use crate::model::Annotation;
use crate::model::List;
use crate::model::LocalSimpleType;
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;

impl<'a> List<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<List<'a>, String> {
        let mut res = List::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => res.simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child node for xsd:list element: {:?}",
                        node
                    ))
                }
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.value().parse()?),
                "itemType" => res.item_type = Some(attr.value().parse()?),
                _ => res.attributes.push(attr.clone()),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::List;
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
        let res = List::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert!(res.item_type.is_none());
        assert_eq!(res.simple_type.unwrap().id.unwrap().0.as_ref(), "STN");
    }
}
