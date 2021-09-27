use crate::model::elements::ElementType;
use crate::model::groups::simple_derivation::SimpleDerivation;
use crate::model::{Annotation, Restriction};
use crate::model::{List, LocalSimpleType, TopLevelSimpleType, Union};
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;
use crate::model::attributes::name::Name;
use std::convert::TryInto;
use crate::model::attributes::final_::SimpleFinal;

impl LocalSimpleType {
    pub fn parse(node: Node<'_, '_>) -> Result<LocalSimpleType, String> {
        let mut annotation = None;
        let mut id = None;
        let mut attributes= vec![];
        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                _ => attributes.push(attr.try_into()?),
            }
        }

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                x => {
                    return Ok(Self {
                        annotation,
                        content_choice: SimpleDerivation::parse(ch, x)?,
                        id,
                        attributes,
                    })
                }
            };
        }

        Err("".to_string())
    }
}

impl TopLevelSimpleType {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content_choice = None;
        let mut id = None;
        let mut final_: Option<SimpleFinal> = None;
        let mut name: Option<Name> = None;
        let mut attributes: Vec<attributes::RawAttribute> = vec![];

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                x => content_choice = Some(SimpleDerivation::parse(ch, x)?),
            };
        }

        let content_choice = content_choice.ok_or_else(|| {
            format!(
                "Content required for xsd:TopLevelSimpleType type: {:?}",
                node
            )
        })?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                "final" => final_ = Some(attr.try_into()?),
                "name" => name = Some(attr.try_into()?),
                _ => attributes.push(attr.try_into()?),
            }
        }

        let name = name.ok_or_else(|| {
            format!(
                "Content required for xsd:TopLevelSimpleType type: {:?}",
                node
            )
        })?;

        Ok(Self {
            annotation,
            content_choice,
            id,
            final_,
            name,
            attributes,
        })
    }
}

impl SimpleDerivation {
    pub fn parse(
        node: Node<'_, '_>,
        element_type: ElementType,
    ) -> Result<SimpleDerivation, String> {
        let res = match element_type {
            ElementType::Union => Self::Union(Box::new(Union::parse(node)?)),
            ElementType::Restriction => Self::Restriction(Box::new(Restriction::parse(node)?)),
            ElementType::List => Self::List(Box::new(List::parse(node)?)),
            _ => return Err(format!("Invalid simple derivation content: {:?}", node)),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod test {

    use crate::model::groups::simple_derivation::SimpleDerivation;
    use crate::model::TopLevelSimpleType;

    #[test]
    fn test_top_level_simple_type_parse() {
        let doc = roxmltree::Document::parse(
            r##"<simpleType id="ID" name="Type1" final="#all" a='b' b='a'>
                        <list itemType="itemType" />
                    </simpleType>"##,
        )
        .unwrap();
        let root = doc.root_element();
        let res = TopLevelSimpleType::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.name.0.as_ref(), "Type1");
        match &res.content_choice {
            SimpleDerivation::List(x) => {
                assert_eq!(x.item_type.as_ref().unwrap().0.name(), "itemType")
            }
            _ => unreachable!("test failed!"),
        }
    }
}
