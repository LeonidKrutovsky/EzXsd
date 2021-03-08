use crate::model::AnyAttribute;
use crate::xml_to_xsd::utils::annotation_only;
use roxmltree::Node;
use std::convert::TryInto;

impl<'a> AnyAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "any")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "namespace" => res.namespace = attr.try_into()?,
                "processContents" => res.process_contents = attr.try_into()?,
                _ => res.attributes.push(attr.clone()),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::AnyAttribute;
    use crate::model::simple_types::namespace_list::NamespaceList;
    use crate::model::attributes::process_contents::ProcessContents;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"<anyAttribute a='a' b='b' namespace="##any" processContents="lax" c='c'/>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = AnyAttribute::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.namespace.0, NamespaceList::Any);
        assert_eq!(res.process_contents, ProcessContents::Lax);
    }
}
