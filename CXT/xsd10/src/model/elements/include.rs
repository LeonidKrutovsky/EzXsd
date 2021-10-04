use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:include
// See http://www.w3.org/TR/xmlschema-1/#element-include.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// schemaLocation	[1..1]	xsd:anyURI
//
// Used in
// Anonymous type of element xsd:schema
#[element(name = "include")]
pub struct Include {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub schema_location: attributes::SchemaLocation,
}

#[cfg(test)]
mod test {
    use super::Include;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<include id="ID" schemaLocation="http://uri" b='a'>
                    <annotation>
                        <appinfo>Some appinfo</appinfo>
                        <documentation>Some doc2</documentation>
                    </annotation>
            </include>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: Include = Include::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().documentations.len(), 1);
        assert_eq!(res.annotation.unwrap().app_infos.len(), 1);
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.schema_location.0.as_ref(), "http://uri");
    }
}

