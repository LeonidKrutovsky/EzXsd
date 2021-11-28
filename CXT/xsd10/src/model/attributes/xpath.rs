use crate::model::simple_types::Token;
use xml_utils::*;

// xpath
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd token
// Pattern: (\.//)?(((child::)?((\i\c*:)?(\i\c*|\*)))|\.)(/(((child::)?((\i\c*:)?(\i\c*|\*)))|\.))*(\|(\.//)?(((child::)?((\i\c*:)?(\i\c*|\*)))|\.)(/(((child::)?((\i\c*:)?(\i\c*|\*)))|\.))*)*
//
// Used in
// Anonymous type of element xsd:selector
#[attribute(name = "xpath")]
pub struct XPath(pub Token);

// xpath
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value:
// Type based on xsd token
// Pattern: (\.//)?((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)/)*((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)|((attribute::|@)((\i\c*:)?(\i\c*|\*))))(\|(\.//)?((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)/)*((((child::)?((\i\c*:)?(\i\c*|\*)))|\.)|((attribute::|@)((\i\c*:)?(\i\c*|\*)))))*
//
// Used in
// Anonymous type of element xsd:field
#[attribute(name = "xpath")]
pub struct FieldXPath(pub Token);


#[cfg(test)]
mod test {
    use crate::model::attributes::{FieldXPath, XPath};

    #[test]
    fn xpath_to_text() {
        let xpath: XPath = "someString".parse().unwrap();
        assert_eq!(xpath.text(), " xpath=\"someString\"")
    }

    #[test]
    fn fieldxpath_to_text() {
        let xpath: FieldXPath = "someString".parse().unwrap();
        assert_eq!(xpath.text(), " xpath=\"someString\"")
    }
}