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

use std::str::FromStr;

use crate::model::simple_types::normalized_string::NormalizedString;
use crate::model::simple_types::utils::assert_collapsed;

#[derive(Debug, Default, Clone)]
pub struct Token(NormalizedString);

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_collapsed(s, "Token")?;
        Ok(Self(s.parse()?))
    }
}

impl_from_string!(Token);
impl_as_ref!(Token);
impl_display!(Token);

#[cfg(test)]
mod test {
    use crate::model::simple_types::Token;

    #[test]
    fn test_to_xml() {
        fn eq(left: &str, right: &str) {
            assert_eq!(left.parse::<Token>().unwrap().as_ref(), right);
        }
        let two_lines_str = r"
This
is on two lines.
        ";

        eq("This is a string!", "This is a string!");
        eq("Édition française.", "Édition française.");
        eq("12.5", "12.5");
        eq("", "");
        // eq("   3 spaces.   ", "3 spaces.");
        // eq(two_lines_str, "This is on two lines.");
        // eq("3 < 4", "3 &lt; 4");
        // eq("AT&T", "AT&amp;T");
        // eq("  WS:    |     T  ", "WS: | T");
        // eq("AT     T", "AT T");
    }
}
