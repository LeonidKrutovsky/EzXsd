use crate::model::elements;
use crate::model::groups::facets::Facets;

// xsd:simpleRestrictionModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:simpleType [0..1]
//      Choice [0..*]       from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:simpleRestrictionType (Element xsd:restriction)
#[derive(Default, Debug)]
pub struct SimpleRestrictionModel {
    pub simple_type: Option<elements::LocalSimpleType>,
    pub facets: Vec<Facets>,
}

impl SimpleRestrictionModel {
    pub const NAMES: &'static [&'static str] = &[
        elements::LocalSimpleType::NAME,

        elements::MinExclusive::NAME,
        elements::MinInclusive::NAME,
        elements::MaxExclusive::NAME,
        elements::MaxInclusive::NAME,
        elements::TotalDigits::NAME,
        elements::FractionDigits::NAME,
        elements::Length::NAME,
        elements::MinLength::NAME,
        elements::MaxLength::NAME,
        elements::Enumeration::NAME,
        elements::WhiteSpace::NAME,
        elements::Pattern::NAME,
    ];

    pub fn push(&mut self, node: roxmltree::Node<'_, '_>) -> Result<(), String> {
        match node.tag_name().name() {
            elements::LocalSimpleType::NAME => {
                self.simple_type = Some(elements::LocalSimpleType::parse(node)?)
            }
            tag_name if Facets::NAMES.contains(&tag_name) => {
                self.facets.push(Facets::parse(node)?)
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::model::groups::facets::Facets;
    use crate::model::groups::simple_restriction_model::SimpleRestrictionModel;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<root id="ID" a='b' b='a'>
                <simpleType id="STN">
                    <list itemType="ListOfType" />
                </simpleType>
                <minInclusive value="1"/>
                <maxInclusive value="5"/>
                <pattern value="[0-9]"/>
                </root>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let mut res = SimpleRestrictionModel::default();
        for ch in root.children().filter(|e| e.is_element()) {
            match ch.tag_name().name() {
                tn if SimpleRestrictionModel::NAMES.contains(&tn) => {
                    assert!(res.push(ch).is_ok());
                }
                &_ => {}
            }
        }

        println!("{:?}", res);

        assert_eq!(res.simple_type.unwrap().id.unwrap().0.as_ref(), "STN");
        assert_eq!(res.facets.len(), 3);
        if let Facets::MinInclusive(val) = &res.facets[0] {
            assert_eq!(val.value.0, "1")
        } else {
            panic!()
        }

        if let Facets::MaxInclusive(val) = &res.facets[1] {
            assert_eq!(val.value.0, "5")
        } else {
            panic!()
        }

        if let Facets::Pattern(val) = &res.facets[2] {
            assert_eq!(val.value.0.as_ref(), "[0-9]")
        } else {
            panic!()
        }
    }
}
