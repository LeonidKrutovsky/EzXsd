// xsd:QName
// The type xsd:QName represents an XML namespace-qualified name. A xsd:QName value consists of a prefix and a local part, separated by a colon, both of which are NCName values. The prefix and colon are optional, but if they are not present, it is assumed that either the name is namespace-qualified by other means (e.g., by a default namespace declaration), or the name is not in a namespace.
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values	        Comment
// pre:myElement	        valid assuming the prefix "pre" is mapped to a namespace in scope
// myElement	            prefix and colon are optional
// Invalid values	    Comment
// :myElement	            a QName must not start with a colon
// pre:3rdElement	        the local part must not start with a number; it must be a valid NCName
//                          an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:QName

use crate::model::simple_types::white_space_facet::collapse;
use crate::model::ToXml;
use core::fmt;
use regex::Regex;
use std::borrow::{Borrow, Cow};

#[derive(Default, Debug, PartialEq)]
pub struct QName<'a>(Cow<'a, str>);

impl<'a, T> From<T> for QName<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self { 0: value.into() }
    }
}

impl<'a> QName<'a> {
    pub fn prefix(&self) -> Option<&str> {
        if let Some(index) = self.0.find(':') {
            Some(&self.0[0..index])
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        if let Some(index) = self.0.find(':') {
            &self.0[index + 1..]
        } else {
            self.0.borrow()
        }
    }
}

impl<'a> ToXml for QName<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let raw = self.raw();
        if raw.starts_with(':') {
            Err(format!("A QName must not start with a colon: {}", raw))
        } else if self.name().chars().next().unwrap().is_numeric() {
            Err(format!(
                "The local part must not start with a number; it must be a valid NCName: {}",
                raw
            ))
        } else {
            Ok(collapse(raw))
        }
    }

    fn raw(&self) -> &str {
        self.0.raw()
    }
}

impl fmt::Display for QName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = self.prefix() {
            write!(f, "{}:{}", prefix, self.name())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::qname::QName;
    use crate::model::ToXml;

    #[test]
    fn test_valid_qname() {
        fn eq(left: &str, right: &str) {
            assert_eq!(QName::from(left).to_xml().unwrap(), right);
        }

        eq("pre:myElement", "pre:myElement");
        eq("myElement", "myElement");
        eq("  myElement  ", "myElement");
    }

    #[test]
    fn test_invalid_qname() {
        assert_eq!(
            QName::from(":myElement").to_xml(),
            Err("A QName must not start with a colon: :myElement".to_string())
        );

        assert_eq!(
        QName::from("pre:3rdElement").to_xml(),
        Err("The local part must not start with a number; it must be a valid NCName: pre:3rdElement".to_string())
        );
    }
}
