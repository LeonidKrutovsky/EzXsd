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

use crate::model::Parse;

#[derive(Debug, Default, Clone)]
pub struct String_(String);

impl Parse for String_ {
    fn parse(value: &str) -> Result<Self, String> {
        for i in value.chars().enumerate() {
            match i {
                (_, '<') => {
                    return Err(format!("Symbol '<' must be escaped: {}", value));
                }
                (start, '&') => {
                    if unsafe {
                        value.get_unchecked(start..start + 4) != "&lt;"
                            && value.get_unchecked(start..start + 5) != "&amp;"
                    } {
                        return Err(format!("Symbol '&' must be escaped: {}", value));
                    }
                }
                _ => {}
            }
        }

        Ok(Self(value.to_string()))
    }

    fn create(value: String) -> Self {
        Self(value)
    }

    fn text(&self) -> Result<String, String> {
        let mut result = String::new();
        let mut start = 0;

        self.0.chars().enumerate().for_each(|x| match x {
            (end, '<') => {
                result.push_str(unsafe { self.0.get_unchecked(start..end) });
                result.push_str("&lt;");
                start = end + 1;
            }
            (end, '&') => {
                result.push_str(unsafe { self.0.get_unchecked(start..end) });
                result.push_str("&amp;");
                start = end + 1;
            }
            _ => {}
        });

        if start != self.0.len() {
            result.push_str(unsafe { self.0.get_unchecked(start..) });
        }

        Ok(result)
    }
}

impl String_ {
    pub fn raw(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::String_;
    use crate::model::Parse;

    #[test]
    fn test_xml_conversation() {
        fn eq(left: &str, right: &str) {
            assert_eq!(String_::create(left.to_string()).text().unwrap(), right);
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

    #[test]
    fn test_parsing() {
        fn eq(left: &str, right: &str) {
            assert_eq!(String_::parse(left).unwrap().raw(), right);
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
        eq("3 &lt; 4", "3 &lt; 4");
        eq("AT&amp;T", "AT&amp;T");

        assert!(String_::parse("3 < 4").is_err());
        assert!(String_::parse("AT&T").is_err());
        assert_eq!(
            String_::parse("&").err().unwrap(),
            "Symbol '&' must be escaped: &"
        );
    }
}
