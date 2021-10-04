use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:appinfo
// See http://www.w3.org/TR/xmlschema-1/#element-appinfo.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// Any text (mixed) content, intermingled with:
// Any element [0..*] Namespace: ##any, Process Contents: lax
//
// Attributes
// source	        [0..1]	xsd:anyURI
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used in
// Anonymous type of element xsd:annotation
#[element(name = "appinfo")]
pub struct AppInfo {
    pub text: Option<String>,
    pub elements: Vec<elements::RawElement>,
    pub source: Option<attributes::Source>,
    pub attributes: Vec<attributes::RawAttribute>,
}

#[cfg(test)]
mod test {
    use super::AppInfo;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<appInfo source="http://ya.com" lang="us" a='a' b='a'>
            A string
            <el>Some element</el>
            </appInfo>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: AppInfo = AppInfo::parse(root).unwrap();
        assert_eq!(res.text.unwrap().trim(), "A string");
        assert_eq!(res.source.unwrap().0.as_ref(), "http://ya.com");
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.elements.len(), 1);
        assert_eq!(res.elements[0].text.as_ref().unwrap(), "Some element");
    }
}
