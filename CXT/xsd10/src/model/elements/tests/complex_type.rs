#[cfg(test)]
mod test {
    use crate::model::elements::tests::parse_document;
    use crate::model::groups::nested_particle::NestedParticle;
    use crate::model::groups::schema_top::SchemaTop;
    use crate::model::groups::type_def_particle::TypeDefParticle;
    use crate::model::groups::ComplexTypeModel;
    use crate::model::{Annotation, TopLevelComplexType};
    use roxmltree::Document;
    use std::rc::Rc;

    const TEXT: &str = include_str!("fixtures/complex_types.xsd");
    #[test]
    pub fn test_top_level_complex_type() {
        let doc = Document::parse(TEXT).unwrap();
        let schema = parse_document(&doc).unwrap();
        let mut iter = schema.content.iter();

        test_1(next_complex_type(&mut iter));
    }

    fn next_complex_type<'a>(
        iter: &mut impl Iterator<Item = &'a (SchemaTop, Vec<Annotation>)>,
    ) -> &'a Rc<TopLevelComplexType> {
        match &iter.next().unwrap().0 {
            SchemaTop::ComplexType(ct) => ct,
            _ => panic!("Test failed!"),
        }
    }

    fn test_1(ct: &Rc<TopLevelComplexType>) {
        assert_eq!(ct.name.0.as_ref(), "IntRange");
        assert_eq!(
            ct.annotation.as_ref().unwrap().documentations[0]
                .text
                .as_ref()
                .unwrap(),
            "Doc Text"
        );

        assert_eq!(ct.attributes.len(), 1);
        assert_eq!(ct.attributes[0].value(), "Whatever!");

        let type_def_particle =
            if let ComplexTypeModel::Content(type_def_particle, _attr_decls) = &ct.model {
                type_def_particle
            } else {
                unreachable!()
            };
        if let TypeDefParticle::Sequence(seq) = type_def_particle.as_ref().unwrap() {
            assert_eq!(seq.nested_particle.len(), 2);
            if let NestedParticle::Element(el) = &seq.nested_particle[0] {
                assert_eq!(el.name.as_ref().unwrap().0.as_ref(), "Min");
                assert_eq!(el.type_.as_ref().unwrap().0.name(), "int");
                assert_eq!(el.type_.as_ref().unwrap().0.prefix(), Some("xs"));
            } else {
                panic!()
            }

            if let NestedParticle::Element(el) = &seq.nested_particle[1] {
                assert_eq!(el.name.as_ref().unwrap().0.as_ref(), "Max");
                assert_eq!(el.type_.as_ref().unwrap().0.name(), "int");
                assert_eq!(el.type_.as_ref().unwrap().0.prefix(), Some("xs"));
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}
