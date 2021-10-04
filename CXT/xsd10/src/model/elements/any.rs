use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:any
// See http://www.w3.org/TR/xmlschema-1/#element-any.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	            from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                    from type xsd:annotated
// namespace	    [0..1]	xsd:namespaceList	    Default value is "##any".                               from type xsd:wildcard
// processContents	[0..1]	Anonymous		        Default value is "strict".                              from type xsd:wildcard
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1".   from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1".   from group xsd:occurs
//
// Used in
// Group xsd:nestedParticle
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[element(name = "any")]
pub struct Any {
    pub annotation: Option<elements::Annotation>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    #[default]
    pub namespace: attributes::Namespace,
    pub process_contents: attributes::ProcessContents,
    #[default]
    pub min_occurs: attributes::MinOccurs,
    #[default]
    pub max_occurs: attributes::MaxOccurs,
}

#[cfg(test)]
mod test {
    use crate::model::elements::any::Any;
    use crate::model::simple_types::namespace_list::NamespaceList;
    use crate::model::attributes::process_contents::ProcessContents;
    use crate::model::attributes::max_occurs::MaxOccurs;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"<any a='a' b='b' namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded" c='c'/>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res: Any = Any::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.namespace.0, NamespaceList::Any);
        assert_eq!(res.process_contents, ProcessContents::Lax);
        assert_eq!(res.min_occurs.0, "0".parse().unwrap());
        assert_eq!(res.max_occurs, MaxOccurs::Unbounded);
    }
}
