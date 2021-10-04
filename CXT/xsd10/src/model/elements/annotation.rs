use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:annotation
// See http://www.w3.org/TR/xmlschema-1/#element-annotation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Choice [0..*]
//      xsd:appinfo
//      xsd:documentation
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID
//
// Used in
// Anonymous type of element xsd:pattern
// Anonymous type of element xsd:redefine
// Anonymous type of element xsd:schema
// Anonymous type of element xsd:totalDigits
// Anonymous type of element xsd:whiteSpace
// Type xsd:annotated
// Anonymous type of element xsd:all via extension of xsd:annotated
// Anonymous type of element xsd:any via extension of xsd:annotated
// Anonymous type of element xsd:complexContent via extension of xsd:annotated
// Anonymous type of element xsd:field via extension of xsd:annotated
// Anonymous type of element xsd:import via extension of xsd:annotated
// Anonymous type of element xsd:include via extension of xsd:annotated
// Anonymous type of element xsd:keyref via extension of xsd:annotated
// Anonymous type of element xsd:list via extension of xsd:annotated
// Anonymous type of element xsd:notation via extension of xsd:annotated
// Anonymous type of element xsd:restriction via extension of xsd:annotated
// Anonymous type of element xsd:selector via extension of xsd:annotated
// Anonymous type of element xsd:simpleContent via extension of xsd:annotated
// Anonymous type of element xsd:union via extension of xsd:annotated
// Type xsd:complexType via extension of xsd:annotated
// Type xsd:localSimpleType (Element xsd:simpleType)
// Type xsd:narrowMaxMin (Element xsd:element)
// Type xsd:noFixedFacet (Element xsd:enumeration)
// Type xsd:simpleType via extension of xsd:annotated
// Type xsd:topLevelSimpleType (Element xsd:simpleType)
// Type xsd:allType via extension of xsd:annotated (Element xsd:all)
// Type xsd:attributeGroupRef via extension of xsd:annotated (Element xsd:attributeGroup)
// Type xsd:complexRestrictionType via extension of xsd:annotated (Element xsd:restriction)
// Type xsd:extensionType via extension of xsd:annotated (Element xsd:extension)
// Type xsd:localAttributeType via extension of xsd:annotated (Element xsd:attribute)
// Type xsd:localComplexType via extension of xsd:annotated (Element xsd:complexType)
// Type xsd:localElement via extension of xsd:annotated (Element xsd:element)
// Type xsd:namedAttributeGroup via extension of xsd:annotated (Element xsd:attributeGroup)
// Type xsd:namedGroup via extension of xsd:annotated (Element xsd:group)
// Type xsd:namedGroupRef via extension of xsd:annotated (Element xsd:group)
// Type xsd:simpleExtensionType via extension of xsd:annotated (Element xsd:extension)
// Type xsd:simpleRestrictionType via extension of xsd:annotated (Element xsd:restriction)
// Type xsd:topLevelAttributeType via extension of xsd:annotated (Element xsd:attribute)
// Type xsd:topLevelComplexType via extension of xsd:annotated (Element xsd:complexType)
// Type xsd:topLevelElement via extension of xsd:annotated (Element xsd:element)
// Type xsd:wildcard via extension of xsd:annotated (Element xsd:anyAttribute)
// Type xsd:explicitGroup via extension of xsd:annotated (Elements xsd:choice, xsd:sequence)
// Type xsd:keybase via extension of xsd:annotated (Elements xsd:unique, xsd:key)
// Type xsd:simpleExplicitGroup via extension of xsd:annotated (Elements xsd:choice, xsd:sequence)
// Type xsd:numFacet (Elements xsd:fractionDigits, xsd:length, xsd:minLength, xsd:maxLength)
// Type xsd:facet via extension of xsd:annotated (Elements xsd:minExclusive, xsd:minInclusive, xsd:maxExclusive, xsd:maxInclusive)
#[element(name = "annotation")]
pub struct Annotation {
    pub app_infos: Vec<elements::AppInfo>,
    pub documentations: Vec<elements::Documentation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
}

#[cfg(test)]
mod test {
    use crate::model::elements::annotation::Annotation;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<annotation id="ID" lang="us" a='a' b='a'>
            <appinfo>Some appinfo</appinfo>
            <appinfo>Some appinfo2</appinfo>
            <documentation>Some doc</documentation>
            <documentation>Some doc2</documentation>
            </annotation>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Annotation::parse(root).unwrap();
        assert_eq!(res.app_infos.len(), 2);
        assert_eq!(res.documentations.len(), 2);
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.id.unwrap().0.as_ref(), "ID");
    }
}

