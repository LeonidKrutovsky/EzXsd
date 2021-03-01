// xsd:normalizedString
// The type xsd:normalizedString represents a character string that may contain any Unicode character allowed by XML. Certain characters, namely the "less than" symbol (<) and the ampersand (&), must be escaped (using the entities &lt; and &amp;, respectively) when used in strings in XML instances.
//
// The xsd:normalizedString type has a whiteSpace facet of replace, which means that the processor replaces each carriage return, line feed, and tab by a single space. This processing is equivalent to the processing of CDATA attribute values in XML 1.0. There is no collapsing of multiple consecutive spaces into a single space.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:string
// White Space: replace
// Examples
// Valid values	Comment
// This is a string!
// Édition française.
// 12.5
// an empty string is valid
// PB&amp;J	when parsed, it will become "PB&J"
//    Separated by 3 spaces.	when parsed, the spaces will be retained
// This
// is on two lines.	when parsed, the line break will be replaced by a space
// Invalid values	Comment
// AT&T	ampersand must be escaped
// 3 < 4	the "less than" symbol must be escaped
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

use crate::model::simple_types::white_space_facet::replace;
use crate::model::simple_types::String_;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct NormalizedString<'a>(String_<'a>);

impl<'a, T> From<T> for NormalizedString<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: String_::from(value.into()),
        }
    }
}

impl<'a> ToXml for NormalizedString<'a> {
    fn to_xml(&self) -> Result<String, String> {
        Ok(replace(self.0.to_xml()?.as_str()))
    }
}

impl<'a> NormalizedString<'a> {
    pub fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::normalized_string::NormalizedString as Str;
    use crate::model::ToXml;

    #[test]
    fn test_valid_normalized_string() {
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
        eq(two_lines_str, " This is on two lines. ");
        eq("3 < 4", "3 &lt; 4");
        eq("AT&T", "AT&amp;T");
        eq("AT&T", "AT&amp;T");
        eq("AT&T", "AT&amp;T");
        eq("AT\nT", "AT T");
        eq("AT\rT", "AT T");
        eq("Tab char=	=", "Tab char= =");
    }
}
