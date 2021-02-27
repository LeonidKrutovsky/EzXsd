// xsd:string
// The type xsd:string represents a character string that may contain
// any Unicode character allowed by XML. Certain characters, namely the
// "less than" symbol (<) and the ampersand (&), must be escaped
// (using the entities &lt; and &amp;, respectively) when used in strings
// in XML instances.
//
// The xsd:string type has a whiteSpace facet of preserve, which means that
// all whitespace characters (spaces, tabs, carriage returns, and line feeds)
// are preserved by the processor. This is in contrast to two types derived from
// it: normalizedString, and token.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: preserve

// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:string
//      restricted by xsd:normalizedString
//        restricted by xsd:token
//          restricted by xsd:language
//          restricted by xsd:NMTOKEN
//            used in list xsd:NMTOKENS
//          restricted by xsd:Name
//            restricted by xsd:NCName
//              restricted by xsd:ID
//              restricted by xsd:IDREF
//                used in list xsd:IDREFS
//              restricted by xsd:ENTITY
//                used in list xsd:ENTITIES

use crate::model::ToXml;
use std::borrow::{Borrow, Cow};

pub type String_<'a> = Cow<'a, str>;

impl<'a> ToXml for String_<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let mut result = String::new();
        let mut start = 0;

        self.chars().enumerate().for_each(|x| match x {
            (end, '<') => {
                result.push_str(unsafe { self.get_unchecked(start..end) });
                result.push_str("&lt;");
                start = end + 1;
            }
            (end, '&') => {
                result.push_str(unsafe { self.get_unchecked(start..end) });
                result.push_str("&amp;");
                start = end + 1;
            }
            _ => {}
        });

        if start != self.len() {
            result.push_str(unsafe { self.get_unchecked(start..) });
        }

        Ok(result)
    }

    fn raw(&self) -> &str {
        self.borrow()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::String_ as Str;
    use crate::model::ToXml;

    #[test]
    fn test_valid_string() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Str::from(left).to_xml().unwrap(), right);
        }
        let two_lines_str = r"
This
is on two lines.
        ";

        eq("This is a string!", "This is a string!");
        eq("Édition française.", "Édition française.");
        eq("12.5", "12.5");
        eq("", "");
        eq("   3 spaces.   ", "   3 spaces.   ");
        eq(two_lines_str, two_lines_str);
        eq("3 < 4", "3 &lt; 4");
        eq("AT&T", "AT&amp;T");
    }
}
