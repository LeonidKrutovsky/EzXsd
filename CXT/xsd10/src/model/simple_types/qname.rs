// xsd:QName
// The type xsd:QName represents an XML namespace-qualified name.
// A xsd:QName value consists of a prefix and a local part, separated by a colon,
// both of which are NCName values. The prefix and colon are optional,
// but if they are not present, it is assumed that either the name is namespace-qualified
// by other means (e.g., by a default namespace declaration), or the name is not in a namespace.
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

use core::fmt;
use std::str::FromStr;

use crate::model::simple_types::NCName;

#[derive(Default, Debug)]
pub struct QName {
    pub prefix: Option<NCName>,
    pub name: NCName,
}

impl QName {
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_ref().map(|v| v.as_ref())
    }
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl FromStr for QName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(':') {
            return Err(format!("A QName must not start with a colon: {}", s));
        }
        if let Some(index) = s.find(':') {
            Ok(Self {
                prefix: Some(s[0..index].parse()?),
                name: s[index + 1..].parse()?,
            })
        } else {
            Ok(Self {
                prefix: None,
                name: s.parse()?,
            })
        }
    }
}

impl fmt::Display for QName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref prefix) = self.prefix {
            write!(f, "{}:{}", prefix, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::qname::QName;

    #[test]
    fn test_valid_qname() {
        fn eq(left: &str, right: &str) {
            assert_eq!(QName::from_str(left).unwrap().to_string(), right);
        }

        eq("pre:myElement", "pre:myElement");
        eq("myElement", "myElement");
    }

    #[test]
    fn test_invalid_qname() {
        assert_eq!(
            QName::from_str(":myElement").err().unwrap(),
            "A QName must not start with a colon: :myElement"
        );

        assert_eq!(
            QName::from_str("pre:3rdElement").err().unwrap(),
            "A Name must not start with a number: 3rdElement"
        );
    }
}
