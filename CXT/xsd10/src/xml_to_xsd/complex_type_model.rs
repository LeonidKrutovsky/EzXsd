use crate::model::elements::ElementType;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::complex_type_model::ComplexTypeModel;
use crate::model::groups::type_def_particle::TypeDefParticle;
use crate::model::{ComplexContent, SimpleContent};
use crate::xml_to_xsd::{ElementChildren, XsdNode};
use roxmltree::Node;

impl<'a> ComplexTypeModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut iter = node.element_children().peekable();
        let mut first_child = iter.peek();
        if let Some(val) = first_child {
            if val.xsd_type()? == ElementType::Annotation {
                iter.next();
                first_child = iter.peek();
            }
        }

        let first_child = match first_child {
            Some(x) => x,
            None => {
                return Ok(ComplexTypeModel::Content(
                    None,
                    Box::new(AttrDecls::default()),
                ))
            }
        };

        let type_def_particle;

        match first_child.xsd_type()? {
            ElementType::SimpleContent => {
                return Ok(Self::SimpleContent(SimpleContent::parse(*first_child)?))
            }
            ElementType::ComplexContent => {
                return Ok(Self::ComplexContent(ComplexContent::parse(*first_child)?))
            }
            x => type_def_particle = TypeDefParticle::parse(*first_child, x)?,
        };

        let skip = if type_def_particle.is_some() { 1 } else { 0 };

        let res = ComplexTypeModel::Content(
            type_def_particle,
            Box::new(AttrDecls::parse(iter.skip(skip))?),
        );
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::complex_types::local_attribute_type::UseType;
    use crate::model::groups::complex_type_model::ComplexTypeModel;
    use crate::model::groups::type_def_particle::TypeDefParticle;
    use crate::model::{ComplexContentChoice, MaxOccurs, SimpleContentChoice};
    use num_bigint::ToBigUint;

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
        if let ComplexTypeModel::SimpleContent(sc) = ComplexTypeModel::parse(root).unwrap() {
            assert!(sc.annotation.is_some());
            assert_eq!(
                sc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .unwrap(),
                "Text"
            );

            if let SimpleContentChoice::Extension(ext) = sc.content {
                assert_eq!(ext.base.name.as_ref(), "string");
                assert_eq!(ext.base.prefix.unwrap().as_ref(), "xs");
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
        if let ComplexTypeModel::ComplexContent(cc) = ComplexTypeModel::parse(root).unwrap() {
            assert!(cc.annotation.is_some());
            assert_eq!(
                cc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .unwrap(),
                "Text"
            );

            if let ComplexContentChoice::Restriction(restr) = cc.content {
                assert_eq!(restr.base.name.as_ref(), "anyType");
                assert_eq!(restr.base.prefix.unwrap().as_ref(), "xs");
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
        if let ComplexTypeModel::Content(type_def, attr_decls) =
            ComplexTypeModel::parse(root).unwrap()
        {
            if let TypeDefParticle::Choice(ch) = type_def.unwrap() {
                assert_eq!(ch.min_occurs.0, 2_i32.to_biguint().unwrap());
                if let MaxOccurs::Bounded(x) = ch.max_occurs {
                    assert_eq!(x.0, 5_i32.to_biguint().unwrap());
                } else {
                    panic!()
                }
            } else {
                panic!()
            }

            assert_eq!(attr_decls.attributes.len(), 3);
            assert_eq!(attr_decls.any_attribute.unwrap().process_contents, "lax");
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
        if let ComplexTypeModel::ComplexContent(cc) = ComplexTypeModel::parse(root).unwrap() {
            assert!(cc.annotation.is_some());
            assert_eq!(
                cc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .unwrap(),
                "Text"
            );

            if let ComplexContentChoice::Restriction(restr) = cc.content {
                assert_eq!(restr.base.name.as_ref(), "anyType");
                assert_eq!(restr.base.prefix.unwrap().as_ref(), "xs");
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
        if let ComplexTypeModel::Content(type_def, attr_decls) =
            ComplexTypeModel::parse(root).unwrap()
        {
            assert!(type_def.is_none());
            assert!(attr_decls.any_attribute.is_none());
            assert_eq!(attr_decls.attribute_groups.len(), 0);
            assert_eq!(attr_decls.attributes.len(), 1);
            assert_eq!(
                attr_decls.attributes[0].name.as_ref().unwrap().as_ref(),
                "token"
            );
            assert_eq!(attr_decls.attributes[0].use_, UseType::Required);
        } else {
            panic!("Test failed!");
        }
    }
}
