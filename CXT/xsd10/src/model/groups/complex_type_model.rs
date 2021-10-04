use crate::model::elements;
use crate::model::elements::complex_content::ComplexContent;
use crate::model::elements::simple_content::SimpleContent;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::type_def_particle::TypeDefParticle;

// xsd:complexTypeModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleContent
//      xsd:complexContent
//      Sequence [1..1]
//          Choice [0..1]       from group xsd:typeDefParticle
//              xsd:group
//              xsd:all         An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//              xsd:choice
//              xsd:sequence
//          Choice [0..*]       from group xsd:attrDecls
//              xsd:attribute
//              xsd:attributeGroup
//          xsd:anyAttribute [0..1]
//
// Used in
// Type xsd:complexType
// Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)
#[derive(Debug)]
pub enum ComplexTypeModel {
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    Content(Option<TypeDefParticle>, Box<AttrDecls>),
}

impl Default for ComplexTypeModel {
    fn default() -> Self {
        Self::Content(None, Box::new(AttrDecls::default()))
    }
}

impl ComplexTypeModel {
    pub const NAMES: &'static [&'static str] = &[
        elements::SimpleContent::NAME,
        elements::ComplexContent::NAME,
        elements::Group::NAME,
        elements::AllType::NAME,
        elements::Choice::NAME,
        elements::Sequence::NAME,
        elements::LocalAttribute::NAME,
        elements::AttributeGroupRef::NAME,
        elements::AnyAttribute::NAME,
    ];

    pub fn push(&mut self, node: roxmltree::Node<'_, '_>) -> Result<(), String> {
        match self {
            ComplexTypeModel::SimpleContent(_) | ComplexTypeModel::ComplexContent(_) => Err(
                format!("Unexpected node in ComplexTypeModel group: {:?}", node),
            )?,
            ComplexTypeModel::Content(tdp, ad) => {
                if tdp.is_some() || !ad.is_empty() {
                    match node.tag_name().name() {
                        SimpleContent::NAME | ComplexContent::NAME => Err(format!(
                            "Unexpected node in ComplexTypeModel group: {:?}",
                            node
                        ))?,
                        tag_name if TypeDefParticle::NAMES.contains(&tag_name) => {
                            *tdp = Some(TypeDefParticle::parse(node)?);
                        }
                        tag_name if AttrDecls::NAMES.contains(&tag_name) => {
                            ad.push(node)?;
                        }
                        _ => {}
                    }
                } else {
                    match node.tag_name().name() {
                        SimpleContent::NAME => {
                            *self = Self::SimpleContent(SimpleContent::parse(node)?)
                        }
                        ComplexContent::NAME => {
                            *self = Self::ComplexContent(ComplexContent::parse(node)?)
                        }
                        tag_name if TypeDefParticle::NAMES.contains(&tag_name) => {
                            *tdp = Some(TypeDefParticle::parse(node)?);
                        }
                        tag_name if AttrDecls::NAMES.contains(&tag_name) => {
                            ad.push(node)?;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::attributes::{MaxOccurs, ProcessContents, Use};
    use crate::model::groups::{ComplexContentChoice, SimpleContentChoice};

    #[test]
    fn test_parse_simple_content() {
        let doc = roxmltree::Document::parse(
            r#"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:simpleContent>
                            <xs:annotation>
								<xs:documentation>Text</xs:documentation>
							</xs:annotation>
	                        <xs:extension base="xs:string" a='a' b='b'>
        	                    <xs:attribute ref="xml:lang" use="required"/>
        	                </xs:extension>
        	            </xs:simpleContent>
                    </xs:complexType>
                 "#,
        )
        .unwrap();

        let root = doc.root_element();
        let mut res = ComplexTypeModel::default();
        assert!(res.push(root.first_element_child().unwrap()).is_ok());

        if let ComplexTypeModel::SimpleContent(sc) = res {
            assert!(sc.annotation.is_some());
            assert_eq!(
                sc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .as_ref()
                    .unwrap(),
                "Text"
            );

            if let SimpleContentChoice::Extension(ext) = sc.content {
                assert_eq!(ext.base.0.name(), "string");
                assert_eq!(ext.base.0.prefix().unwrap(), "xs");
                assert_eq!(ext.attributes.len(), 2);
                assert_eq!(ext.attr_decls.attributes.len(), 1);
            } else {
                panic!("Test failed!");
            }
        } else {
            panic!("Test failed!");
        }
    }

    #[test]
    fn test_parse_complex_content() {
        let doc = roxmltree::Document::parse(
            r###"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                            <xs:complexContent>
                                <xs:annotation>
                                    <xs:documentation>Text</xs:documentation>
                                </xs:annotation>
                                <xs:restriction base="xs:anyType" a='a'>
                                    <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                                    <xs:attribute name="Attr2" type="xs:anyURI"/>
                                    <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                                </xs:restriction>
                            </xs:complexContent>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
        let mut res = ComplexTypeModel::default();
        assert!(res.push(root.first_element_child().unwrap()).is_ok());

        if let ComplexTypeModel::ComplexContent(cc) = res {
            assert!(cc.annotation.is_some());
            assert_eq!(
                cc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .as_ref()
                    .unwrap(),
                "Text"
            );

            if let ComplexContentChoice::Restriction(restr) = cc.content {
                assert_eq!(restr.base.0.name.as_ref(), "anyType");
                assert_eq!(restr.base.0.prefix.unwrap().as_ref(), "xs");
                assert_eq!(restr.attributes.len(), 1);
                assert_eq!(restr.attr_decls.attributes.len(), 3);
            } else {
                panic!("Test failed!");
            }
        } else {
            panic!("Test failed!");
        }
    }

    #[test]
    fn test_parse_content() {
        let doc = roxmltree::Document::parse(
            r###"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                         <xs:choice minOccurs="2" maxOccurs="5"/>
                         <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                         <xs:attribute name="Attr2" type="xs:anyURI"/>
                         <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                         <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
        let mut res = ComplexTypeModel::default();
        for ch in root.children().filter(|n| n.is_element()) {
            assert!(res.push(ch).is_ok());
        }

        if let ComplexTypeModel::Content(type_def, attr_decls) = res {
            if let TypeDefParticle::Choice(ch) = type_def.unwrap() {
                assert_eq!(ch.min_occurs.0, "2".parse().unwrap());
                if let MaxOccurs::Bounded(x) = ch.max_occurs {
                    assert_eq!(x.0, "5".parse().unwrap());
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
            assert_eq!(attr_decls.attributes.len(), 3);
            assert_eq!(
                attr_decls.any_attribute.unwrap().process_contents,
                ProcessContents::Lax
            );
        } else {
            panic!()
        }
    }

    #[test]
    fn test_skip_annotation() {
        let doc = roxmltree::Document::parse(
            r###"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                            <xs:annotation>
                               <xs:documentation>Text</xs:documentation>
                            </xs:annotation>
                            <xs:complexContent>
                                <xs:annotation>
                                    <xs:documentation>Text</xs:documentation>
                                </xs:annotation>
                                <xs:restriction base="xs:anyType" a='a'>
                                    <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                                    <xs:attribute name="Attr2" type="xs:anyURI"/>
                                    <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                                </xs:restriction>
                            </xs:complexContent>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
        let mut res = ComplexTypeModel::default();
        for ch in root.children().filter(|n| n.is_element()) {
            assert!(res.push(ch).is_ok());
        }

        if let ComplexTypeModel::ComplexContent(cc) = res {
            assert!(cc.annotation.is_some());
            assert_eq!(
                cc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .as_ref()
                    .unwrap(),
                "Text"
            );

            if let ComplexContentChoice::Restriction(restr) = cc.content {
                assert_eq!(restr.base.0.name.as_ref(), "anyType");
                assert_eq!(restr.base.0.prefix.unwrap().as_ref(), "xs");
                assert_eq!(restr.attributes.len(), 1);
                assert_eq!(restr.attr_decls.attributes.len(), 3);
            } else {
                panic!("Test failed!");
            }
        } else {
            panic!("Test failed!");
        }
    }

    #[test]
    fn test_skip_annotation2() {
        let doc = roxmltree::Document::parse(
            r###"<xs:complexType name="DeviceEntity" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:annotation>
                            <xs:documentation>Base class for physical entities like inputs and outputs.</xs:documentation>
                        </xs:annotation>
                        <xs:attribute name="token" type="tt:ReferenceToken" use="required">
                            <xs:annotation>
                                <xs:documentation>Unique identifier referencing the physical entity.</xs:documentation>
                            </xs:annotation>
                        </xs:attribute>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
        let mut res = ComplexTypeModel::default();
        for ch in root.children().filter(|n| n.is_element()) {
            assert!(res.push(ch).is_ok());
        }

        if let ComplexTypeModel::Content(type_def, attr_decls) = res {
            assert!(type_def.is_none());
            assert!(attr_decls.any_attribute.is_none());
            assert_eq!(attr_decls.attribute_groups.len(), 0);
            assert_eq!(attr_decls.attributes.len(), 1);
            assert_eq!(
                attr_decls.attributes[0].name.as_ref().unwrap().0.as_ref(),
                "token"
            );
            assert_eq!(attr_decls.attributes[0].use_, Some(Use::Required));
        } else {
            panic!("Test failed!");
        }
    }
}
