use crate::model::Include;
use crate::xml_to_xsd::utils::annotation_only;
use roxmltree::Node;
use std::convert::TryInto;

impl Include {
    pub fn parse(node: Node<'_, '_>) -> Result<Include, String> {
        let mut res = Include::default();
        res.annotation = annotation_only(node, "include")?;

        for attr in node.attributes() {
            match attr.name() {
                "schemaLocation" => res.schema_location = attr.try_into()?,
                "id" => res.id = Some(attr.try_into()?),
                _ => res.attributes.push(attr.try_into()?),
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
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.schema_location.0.as_ref(), "http://uri");
    }
}
