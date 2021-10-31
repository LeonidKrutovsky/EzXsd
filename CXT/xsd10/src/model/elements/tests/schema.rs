#[cfg(test)]
mod test {
    use crate::model::elements::tests::parse_document;
    use crate::model::simple_types::form_choice::FormChoice;
    use crate::model::Schema;
    use roxmltree::{Document, Node};

    fn print(node: Node<'_, '_>, level: usize) {
        let indent = " ".repeat(level);
        println!(
            "{}<{}>  {}",
            indent,
            node.tag_name().name(),
            node.tag_name().namespace().unwrap_or("")
        );
        // for ns in node.namespaces() {
        //     println!("{}NS = {:?}", indent, ns);
        // }

        for ch in node.children().filter(|n| n.is_element()) {
            print(ch, level + 4);
        }
    }

    const TEXT: &str = include_str!("fixtures/schema.xsd");
    #[test]
    fn test_parse_document() {
        let doc = Document::parse(TEXT).unwrap();
        let root = doc.root_element();
        print(root, 0);
        let schema = parse_document(&doc).unwrap();
        assert_eq!(
            schema.target_namespace.as_ref().unwrap().0.as_ref(),
            "http://www.onvif.org/ver10/schema"
        );

        test_attributes(&schema);
        test_includes(&schema);
        test_imports(&schema);
    }

    fn test_attributes(schema: &Schema) {
        assert_eq!(
            schema.target_namespace.as_ref().unwrap().0.as_ref(),
            "http://www.onvif.org/ver10/schema"
        );
        assert_eq!(schema.element_form_default.0, FormChoice::Qualified);
        assert_eq!(schema.version.as_ref().unwrap().0.as_ref(), "1.0");
        assert_eq!(schema.id.as_ref().unwrap().0.as_ref(), "ID");
        assert_eq!(schema.attributes.len(), 3);
        assert_eq!(schema.attributes[2].value(), "C");
    }

    fn test_includes(schema: &Schema) {
        assert_eq!(schema.includes.len(), 4);
        assert_eq!(schema.includes[0].schema_location.0.as_ref(), "common1.xsd");
        assert_eq!(schema.includes[1].schema_location.0.as_ref(), "common2.xsd");
        assert_eq!(schema.includes[2].schema_location.0.as_ref(), "common3.xsd");
        assert_eq!(schema.includes[3].schema_location.0.as_ref(), "common4.xsd");
    }

    fn test_imports(schema: &Schema) {
        assert_eq!(schema.imports.len(), 4);
        assert_eq!(
            schema.imports[1].namespace.as_ref().unwrap().0.as_ref(),
            "http://www.w3.org/2003/05/soap-envelope"
        );
        assert_eq!(
            schema.imports[3].namespace.as_ref().unwrap().0.as_ref(),
            "http://www.w3.org/2004/08/xop/include"
        );
        assert_eq!(
            schema.imports[0]
                .schema_location
                .as_ref()
                .unwrap()
                .0
                .as_ref(),
            "http://www.w3.org/2005/05/xmlmime"
        );
        assert_eq!(
            schema.imports[2]
                .schema_location
                .as_ref()
                .unwrap()
                .0
                .as_ref(),
            "http://docs.oasis-open.org/wsn/b-2.xsd"
        );
    }
}
