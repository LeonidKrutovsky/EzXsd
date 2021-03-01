// xsd:hexBinary
// The xsd:hexBinary type represents binary data as a sequence of binary octets. It uses hexadecimal encoding, where each binary octet is a two-character hexadecimal number. Lowercase and uppercase letters A through F are permitted. For example, 0FB8 and 0fb8 are two equal xsd:hexBinary representations consisting of two octets.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse

// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:hexBinary

use crate::model::simple_types::white_space_facet::collapse;
use crate::model::ToXml;
use std::str::FromStr;

pub struct HexBinary(Vec<u8>);

impl FromStr for HexBinary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = hex::decode(s).map_err(|er| er.to_string())?;
        Ok(Self(decoded))
    }
}

impl HexBinary {
    pub fn decoded(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn encoded(&self) -> String {
        hex::encode(self.0.as_slice())
    }
}

impl ToXml for HexBinary {
    fn to_xml(&self) -> Result<String, String> {
        Ok(collapse(self.encoded().as_str()))
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::hex_binary::HexBinary;
    use std::str::FromStr;

    #[test]
    fn test_hex_binary() {
        let first = HexBinary::from_str("0FB8").unwrap();
        let second = HexBinary::from_str("0fb8").unwrap();
        let empty = HexBinary::from_str("").unwrap();
        assert_eq!(first.decoded(), vec![15, 184]);
        assert_eq!(second.decoded(), vec![15, 184]);
        assert_eq!(empty.decoded(), vec![]);

        assert_eq!(first.encoded(), "0fb8");
        assert_eq!(second.encoded(), "0fb8");
        assert_eq!(empty.encoded(), "");

        assert!(HexBinary::from_str("FB8").is_err());
    }
}
