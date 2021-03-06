use std::str::FromStr;

use regex::bytes::Regex;

use crate::model::simple_types::Token;

// xsd:language
// The type xsd:language represents a natural language identifier, generally used to indicate the
// language of a document or a part of a document. Before creating a new attribute of
// type xsd:language, consider using the xml:lang attribute that is intended to indicate
// the natural language of the element and its content.
// Values of the xsd:language type conform to RFC 3066, Tags for the Identification of Languages.
// The three most common formats are:
// For ISO-recognized languages, the format is a two- or three-letter, (usually lowercase)
// language code that conforms to ISO 639, optionally followed by a hyphen and a two-letter,
// (usually uppercase) country code that conforms to ISO 3166. For example, en or en-US.
// For languages registered by the Internet Assigned Numbers Authority (IANA), the format is
// i-langname, where langname is the registered name. For example, i-navajo.
// For unofficial languages, the format is x-langname, where langname is a name of up to eight
// characters agreed upon by the two parties sharing the document. For example, x-Newspeak.
// Any of these three formats may have additional parts, each preceded by a hyphen, which
// identify additional countries or dialects. Schema processors will not verify that values
// of the xsd:language type conform to the above rules. They will simply validate based on the
// pattern specified for this type.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
// White Space: collapse (Defined in type xsd:token)
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language

#[derive(Debug)]
pub struct Language(Token);

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*").unwrap();
        }
        if RE.is_match(s.as_bytes()) {
            Err(format!("Invalid value for Language: {}", s))
        } else {
            Ok(Self(s.parse()?))
        }
    }
}

impl_from_string!(Language);
impl_as_ref!(Language);


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::Language;

    #[test]
    fn test_valid_values() {
        fn is_ok(s: &str) {
            assert!(Language::from_str(s).is_ok());
        }
        is_ok("en");
        is_ok("en-GB");
        is_ok("en-US");
        is_ok("fr");
        is_ok("de");
        is_ok("es");
        is_ok("ja");
        is_ok("i-navajo");
        is_ok("x-Newspeak");
        is_ok("any-value-with-short-parts");  //although a schema processor will consider this value valid, it does not follow RFC 3066 guidelines
        is_ok("");
    }

        #[test]
    fn test_invalid_values() {
        fn is_err(s: &str) {
            assert!(Language::from_str(s).is_err());
        }
        is_err("longerThan8");
        is_err("");
    }
}
