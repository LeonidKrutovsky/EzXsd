use crate::model::Any;
use crate::xml_to_xsd::utils::annotation_only;
use roxmltree::Node;
use std::convert::TryInto;

impl<'a> Any<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "any")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "namespace" => res.namespace = attr.try_into()?,
                "processContents" => res.process_contents = attr.try_into()?,
                "minOccurs" => res.min_occurs = attr.try_into()?,
                "maxOccurs" => res.max_occurs = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::elements::any::Any;
    use crate::model::simple_types::namespace_list::NamespaceList;
    use crate::model::attributes::process_contents::ProcessContents;
    use crate::model::attributes::max_occurs::MaxOccurs;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"<any a='a' b='b' namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded" c='c'/>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Any::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.namespace.0, NamespaceList::Any);
        assert_eq!(res.process_contents, ProcessContents::Lax);
        assert_eq!(res.min_occurs.0, "0".parse().unwrap());
        assert_eq!(res.max_occurs, MaxOccurs::Unbounded);
    }
}
