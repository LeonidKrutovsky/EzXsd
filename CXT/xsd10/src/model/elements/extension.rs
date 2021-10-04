use crate::model::{attributes, elements, groups};
use xml_utils::element;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExtensionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
#[element(name = "extension")]
pub struct SimpleExtension {
    pub annotation: Option<elements::Annotation>,
    #[sequence_group]
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:extensionType
// Properties: Local, Qualified
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	[0..1]	xsd:ID		from type xsd:annotated
// base	[1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:complexContent
#[element(name = "extension")]
pub struct Extension {
    pub annotation: Option<elements::Annotation>,
    pub type_def_particle: Option<groups::TypeDefParticle>,
    #[sequence_group]
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::groups::TypeDefParticle;
    use crate::model::attributes::ProcessContents;

    #[test]
    fn test_empty() {
        let doc = roxmltree::Document::parse(
            r#"<xs:extension base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema" a='a' b='b' id="ID">
                    </xs:extension>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res:Extension = Extension::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.base.0.name(), "BarType");
        assert_eq!(res.base.0.prefix(), Some("tns"));
        assert!(res.type_def_particle.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.attr_decls.attributes.len(), 0);
    }

    #[test]
    fn test() {
        let doc = roxmltree::Document::parse(
            r###"<xs:extension base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:annotation>
							<xs:documentation>Text</xs:documentation>
						</xs:annotation>
                        <xs:sequence>
                            <xs:annotation>
                                <xs:documentation>Text</xs:documentation>
                            </xs:annotation>
                        </xs:sequence>
                        <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                        <xs:attribute name="Attr2" type="xs:anyURI"/>
                        <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                        <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:extension>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res:Extension = Extension::parse(root).unwrap();
        assert!(res.annotation.is_some());
        if let TypeDefParticle::Sequence(val) = res.type_def_particle.unwrap() {
            assert!(val.annotation.is_some());
        } else {
            panic!()
        }

        let attr = &res.attr_decls;
        assert_eq!(attr.attributes.len(), 3);
        assert_eq!(attr.any_attribute.as_ref().unwrap().process_contents, ProcessContents::Lax);
    }
}

