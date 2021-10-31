use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::element;

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      xsd:simpleType [0..1]        from group xsd:simpleRestrictionModel
//      Choice [0..*]                from group xsd:facets
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
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// base	            [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
#[element(name = "restriction")]
pub struct Restriction {
    pub annotation: Option<elements::Annotation>,
    #[sequence_group]
    pub model: groups::SimpleRestrictionModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: Option<attributes::Base>, // base attribute and simpleType child are mutually exclusive, but one or other is required
}

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
#[element(name = "restriction")]
pub struct SimpleRestriction {
    pub annotation: Option<elements::Annotation>,
    #[sequence_group]
    pub model: groups::SimpleRestrictionModel,
    #[sequence_group]
    pub attr_decls: groups::AttrDecls,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
    pub attributes: Vec<attributes::RawAttribute>,
}

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:complexRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:complexContent
#[element(name = "restriction")]
pub struct ComplexRestriction {
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
    use crate::model::attributes::ProcessContents;
    use crate::model::groups::TypeDefParticle;

    #[test]
    fn test_parse_restriction() {
        let doc = roxmltree::Document::parse(
            r#"<restriction xmlns:xsd="http://www.w3.org/2001/XMLSchema" id="ID" base="xsd:Type1" a='b' b='a'>
                    <xsd:annotation>
						<xsd:documentation>Text</xsd:documentation>
					</xsd:annotation>

                    <xsd:minExclusive value="2"/>
                    <xsd:minInclusive value="1"/>
                    <xsd:maxExclusive value="6"/>
                    <xsd:maxInclusive value="5"/>

                    <xsd:totalDigits value="1"/>
                    <xsd:fractionDigits value="1"/>
                    <xsd:length value="1"/>
                    <xsd:minLength value="1"/>

                    <xsd:maxLength value="1"/>
                    <xsd:enumeration value="4"/>
                    <xsd:whiteSpace value="collapse"/>
                    <xsd:pattern value="[2-5]"/>
            </restriction>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: Restriction = Restriction::parse(root).unwrap();
        assert!(res.annotation.is_some());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.base.as_ref().unwrap().0.name(), "Type1");
        assert_eq!(res.base.as_ref().unwrap().0.prefix().unwrap(), "xsd");
        let model = &res.model;
        assert_eq!(model.facets.len(), 12);
    }

    #[test]
    fn test_empty() {
        let doc = roxmltree::Document::parse(
            r#"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema" a='a' b='b' id="ID">
                    </xs:restriction>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res: ComplexRestriction = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.base.0.name(), "BarType");
        assert_eq!(res.base.0.prefix(), Some("tns"));
        assert!(res.type_def_particle.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
        assert_eq!(res.attr_decls.attributes.len(), 0);
    }

    #[test]
    fn test_full() {
        let doc = roxmltree::Document::parse(
            r###"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
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
                    </xs:restriction>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res: ComplexRestriction = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_some());
        if let TypeDefParticle::Sequence(val) = res.type_def_particle.unwrap() {
            assert!(val.annotation.is_some());
        } else {
            panic!()
        }

        let attr = &res.attr_decls;
        assert_eq!(attr.attributes.len(), 3);
        assert_eq!(
            attr.any_attribute.as_ref().unwrap().process_contents,
            ProcessContents::Lax
        );
    }

    #[test]
    fn test_only_attr_decls() {
        let doc = roxmltree::Document::parse(
            r###"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                        <xs:attribute name="Attr2" type="xs:anyURI"/>
                        <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                        <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:restriction>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res: ComplexRestriction = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert!(res.type_def_particle.is_none());
        let attr = &res.attr_decls;
        assert_eq!(attr.attributes.len(), 3);
        assert_eq!(
            attr.any_attribute.as_ref().unwrap().process_contents,
            ProcessContents::Lax
        );
    }
}
