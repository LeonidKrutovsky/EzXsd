// xsd:token
// The type xsd:token represents a character string that may contain any Unicode character allowed by XML.
// Certain characters, namely the "less than" symbol (<) and the ampersand (&), must be escaped
// (using the entities &lt; and &amp;, respectively) when used in strings in XML instances.
// The name xsd:token may be slightly confusing because it implies that there may be only one token
// with no whitespace. In fact, there can be whitespace in a token value. The xsd:token type has a
// whiteSpace facet of collapse, which means that the processor replaces each carriage return,
// line feed, and tab by a single space. After this replacement, each group of consecutive spaces
// is collapsed into one space character, and all leading and trailing spaces are removed.
// This processing is equivalent to the processing of non-CDATA attribute values in XML 1.0.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
//  Based on xsd:normalizedString
//  White Space: collapse
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                          restricted by xsd:IDREF
//                              used in list xsd:IDREFS
//                          restricted by xsd:ENTITY
//                              used in list xsd:ENTITIES

use crate::model::simple_types::normalized_string::NormalizedString;
use crate::model::simple_types::white_space_facet::{collapse, is_collapsed};
use crate::model::Parse;

#[derive(Debug, Default, Clone)]
pub struct Token(NormalizedString);

impl Parse for Token {
    fn parse(value: &str) -> Result<Self, String> {
        if is_collapsed(value) {
            Ok(Self(NormalizedString::parse(value)?))
        } else {
            Err(format!(
                "Invalid value for Token. White spaces must be collapsed: {}",
                value
            ))
        }
    }

    fn create(value: String) -> Self {
        Self(NormalizedString::create(value))
    }

    fn text(&self) -> Result<String, String> {
        Ok(collapse(self.0.text()?.as_str()))
    }
}

impl_from_str!(Token);
impl_from_string!(Token);

impl Token {
    pub fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::Token;
    use crate::model::Parse;

    #[test]
    fn test_to_xml() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Token::create(left.to_string()).text().unwrap(), right);
        }
        let two_lines_str = r"
This
is on two lines.
        ";

        eq("This is a string!", "This is a string!");
        eq("Édition française.", "Édition française.");
        eq("12.5", "12.5");
        eq("", "");
        eq("   3 spaces.   ", "3 spaces.");
        eq(two_lines_str, "This is on two lines.");
        eq("3 < 4", "3 &lt; 4");
        eq("AT&T", "AT&amp;T");
        eq("  WS:    |     T  ", "WS: | T");
        eq("AT     T", "AT T");
    }
}
