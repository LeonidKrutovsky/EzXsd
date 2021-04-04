use crate::model::elements::ElementType;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::{AnyAttribute, LocalAttribute};
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;

impl AttrDecls {
    pub fn parse(iter: impl Iterator<Item = Node>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in iter {
            match ch.xsd_type()? {
                ElementType::Attribute => res.attributes.push(LocalAttribute::parse(ch)?),
                ElementType::AttributeGroup => unimplemented!("AttributeGroupRef::parse"), //res.attribute_groups.push(AttributeGroupRef::parse(ch)?), //TODO:  AttributeGroupRef::parse
                ElementType::AnyAttribute => res.any_attribute = Some(AnyAttribute::parse(ch)?),
                _ => {
                    return Err(
                        format!("Invalid nod for xsd::attrDecls group: {:?}***", ch,)
                            + ch.document().input_text()[ch.range().start..ch.range().end].as_ref(),
                    )
                }
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::groups::attr_decls::AttrDecls;
    use crate::xml_to_xsd::ElementChildren;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<root xmlns:xs="http://www.w3.org/2001/XMLSchema">
                   <xs:attribute name="Attr1" type="xs:boolean" />
                   <xs:attribute name="Attr2" type="xs:boolean" />
                   <xs:attribute name="Attr3" type="xs:boolean" />
                   <xs:anyAttribute processContents="lax"/>
                   </root>"#,
        )
        .unwrap();

        let root = doc.root_element();
        let mut iter = root.element_children();

        let res = AttrDecls::parse(&mut iter).unwrap();
        assert_eq!(res.attribute_groups.len(), 0);
        assert_eq!(res.attributes.len(), 3);
        assert!(res.any_attribute.is_some());
    }
}
