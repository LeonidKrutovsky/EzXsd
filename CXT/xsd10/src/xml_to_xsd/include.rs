use crate::model::Include;
use crate::xml_to_xsd::utils::annotation_only;
use roxmltree::Node;

impl<'a> Include<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Include<'a>, String> {
        let mut res = Include::default();
        res.annotation = annotation_only(node, "include")?;

        for attr in node.attributes() {
            match attr.name() {
                "schemaLocation" => res.schema_location = attr.value().parse()?,
                "id" => res.id = Some(attr.value().parse()?),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::elements::include::Include;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<include id="ID" schemaLocation="http://uri" b='a'>
                    <annotation>
                        <appInfo>Some appinfo</appInfo>
                        <documentation>Some doc2</documentation>
                    </annotation>
            </include>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Include::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().documentations.len(), 1);
        assert_eq!(res.annotation.unwrap().app_infos.len(), 1);
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.unwrap().raw(), "ID");
        assert_eq!(res.schema_location.raw(), "http://uri");
    }
}
