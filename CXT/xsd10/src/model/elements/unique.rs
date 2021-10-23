use crate::model::{attributes, elements};
use xml_utils::element;
use std::ops::{Deref, DerefMut};
use crate::model::complex_types::key_base::KeyBase;
use crate::model::attributes::RawAttribute;

// xsd:unique
// See http://www.w3.org/TR/xmlschema-1/#element-unique.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:keybase
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      from type xsd:annotated
//          xsd:annotation [0..1]
//          xsd:selector [1..1]
//          xsd:field [1..*]
//  Attributes
//      Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
//      id	            [0..1]	xsd:ID		                                            from type xsd:annotated
//      name	        [1..1]	xsd:NCName
//
// Used in
// Group xsd:identityConstraint
// Group xsd:elementModel via reference to xsd:identityConstraint
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
//
// Sample instance
// <xsd:unique name="NCName">
//    <xsd:selector xpath="token">
//    </xsd:selector>
//    <xsd:field xpath="token">
//    </xsd:field>
// </xsd:unique>
#[derive(Debug)]
pub struct Unique(KeyBase);

impl Deref for Unique {
    type Target = KeyBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Unique {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Unique {
    pub const NAME: &'static str = "unique";
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        Ok(Self{0: KeyBase::parse(node)?})
    }
}

#[cfg(test)]
mod test {
    use crate::model::Unique;

    #[test]
    fn test_parse() {
         let doc = roxmltree::Document::parse(
            r#"
<unique name="NCName" attr1="" attr2 = "">
   <selector xpath="token">
   </selector>
   <field xpath="token">
   </field>
</unique>
"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Unique::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
    }
}
