use crate::model::Import;
use crate::xml_to_xsd::utils::annotation_only;
use roxmltree::Node;

impl<'a> Import<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Import<'a>, String> {
        let mut res = Import::default();
        res.annotation = annotation_only(node, "import")?;

        for attr in node.attributes() {
            match attr.name() {
                "schemaLocation" => res.schema_location = Some(attr.value().parse()?),
                "id" => res.id = Some(attr.value().parse()?),
                "namespace" => res.namespace = Some(attr.value().parse()?),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::Import;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<import id="ID" namespace="xsd" schemaLocation="http://uri" b='a'>
                    <annotation>
                        <appInfo>Some appinfo</appInfo>
                        <documentation>Some doc2</documentation>
                    </annotation>
            </import>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Import::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().documentations.len(), 1);
        assert_eq!(res.annotation.unwrap().app_infos.len(), 1);
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.unwrap().raw(), "ID");
        assert_eq!(res.schema_location.unwrap().raw(), "http://uri");
        assert_eq!(res.namespace.unwrap().raw(), "xsd");
    }
}
