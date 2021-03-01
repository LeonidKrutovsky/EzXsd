pub mod any_simple_type;
pub mod any_uri;
pub mod base64binary;
pub mod block_set;
pub mod boolean;
pub mod byte;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod derivation_set;
pub mod deriviation_control;
pub mod double;
pub mod duration;
pub mod entities;
pub mod entity;
pub mod float;
pub mod form_choice;
pub mod full_derivation_set;
pub mod gday;
pub mod gmonth;
pub mod gmonthday;
pub mod gyear;
pub mod gyearmonth;
pub mod hex_binary;
pub mod id;
pub mod idref;
pub mod idrefs;
pub mod integer;
pub mod language;
pub mod name;
pub mod ncname;
pub mod negative_integer;
pub mod nmtoken;
pub mod nmtokens;
pub mod non_negative_integer;
pub mod non_positive_integer;
pub mod normalized_string;
pub mod positive_integer;
pub mod public;
pub mod qname;
pub mod simple_derivation_set;
pub mod string;
pub mod time;
pub mod token;
mod white_space_facet;
pub mod long;
pub mod int;
pub mod short;
pub mod unsigned_long;
pub mod unsigned_int;
pub mod unsigned_short;
pub mod unsigned_byte;

pub use any_uri::AnyUri;
pub use base64binary::Base64Binary;
pub use block_set::*;
pub use boolean::*;
pub use byte::*;
pub use date::*;
pub use datetime::*;
pub use decimal::*;
pub use derivation_set::*;
pub use deriviation_control::*;
pub use duration::*;
pub use form_choice::*;
pub use full_derivation_set::*;
pub use gday::*;
pub use gmonth::*;
pub use gmonthday::*;
pub use gyear::*;
pub use gyearmonth::*;
pub use id::*;
pub use integer::*;
pub use language::*;
pub use ncname::*;
pub use negative_integer::*;
pub use non_negative_integer::*;
pub use non_positive_integer::*;
pub use positive_integer::*;
pub use public::*;
pub use qname::*;
pub use simple_derivation_set::*;
pub use string::*;
pub use time::*;
pub use token::*;

use chrono::FixedOffset;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

#[derive(Debug, PartialEq)]
pub enum SimpleType {
    AnyURI,
    Base64Binary,
    Boolean,
    Byte,
    Date,
    DateTime,
    Decimal,
    DerivationControl,
    Double,
    Duration,
    ENTITIES,
    ENTITY,
    Float,
    GDay,
    GMonth,
    GMonthDay,
    GYear,
    GYearMonth,
    HexBinary,
    ID,
    IDREF,
    IDREFS,
    Int,
    Integer,
    Language,
    Long,
    Name,
    NCName,
    NegativeInteger,
    NMTOKEN,
    NMTOKENS,
    NonNegativeInteger,
    NonPositiveInteger,
    NormalizedString,
    NOTATION,
    PositiveInteger,
    QName,
    Short,
    SimpleDerivationSet,
    String,
    Time,
    Token,
    UnsignedByte,
    UnsignedInt,
    UnsignedLong,
    UnsignedShort,
}

pub fn xsd_simple_type(name: &str) -> Result<SimpleType, String> {
    use SimpleType::*;
    Ok(match name {
        "anyURI" => AnyURI,
        "base64Binary" => Base64Binary,
        "boolean" => Boolean,
        "byte" => Byte,
        "date" => Date,
        "dateTime" => DateTime,
        "decimal" => Decimal,
        "derivationControl" => DerivationControl,
        "double" => Double,
        "duration" => Duration,
        "ENTITIES" => ENTITIES,
        "ENTITY" => ENTITY,
        "float" => Float,
        "gDay" => GDay,
        "gMonth" => GMonth,
        "gMonthDay" => GMonthDay,
        "gYear" => GYear,
        "gYearMonth" => GYearMonth,
        "hexBinary" => HexBinary,
        "ID" => ID,
        "IDREF" => IDREF,
        "IDREFS" => IDREFS,
        "int" => Int,
        "integer" => Integer,
        "language" => Language,
        "long" => Long,
        "name" => Name,
        "NCName" => NCName,
        "negativeInteger" => NegativeInteger,
        "NMTOKEN" => NMTOKEN,
        "NMTOKENS" => NMTOKENS,
        "nonNegativeInteger" => NonNegativeInteger,
        "nonPositiveInteger" => NonPositiveInteger,
        "normalizedString" => NormalizedString,
        "NOTATION" => NOTATION,
        "positiveInteger" => PositiveInteger,
        "QName" => QName,
        "short" => Short,
        "simpleDerivationSet" => SimpleDerivationSet,
        "string" => String,
        "time" => Time,
        "token" => Token,
        "unsignedByte" => UnsignedByte,
        "unsignedInt" => UnsignedInt,
        "unsignedLong" => UnsignedLong,
        "unsignedShort" => UnsignedShort,
        _ => return Err(format!("Invalid xs:simpleType name: {}", name)),
    })
}

// Parses ISO 8601 timezone.
pub fn parse_timezone(s: &str) -> Result<FixedOffset, String> {
    if s == "Z" {
        return Ok(FixedOffset::east(0));
    }

    let tokens: Vec<&str> = s[1..].split(':').collect();
    if tokens.len() != 2 || tokens[0].len() != 2 || tokens[1].len() != 2 {
        return Err("bad timezone format".to_string());
    }
    if !tokens.iter().all(|t| t.chars().all(|c| c.is_digit(10))) {
        return Err("bad timezone format".to_string());
    }

    let hours = tokens[0].parse::<i32>().unwrap();
    let minutes = tokens[1].parse::<i32>().unwrap();

    if hours > 14 || (hours == 14 && minutes != 0) || minutes >= 60 {
        return Err("bad timezone format: out of range".to_string());
    }

    let offset_secs = 60 * (60 * hours + minutes);
    match s.chars().next().unwrap() {
        '+' => Ok(FixedOffset::east(offset_secs)),
        '-' => Ok(FixedOffset::west(offset_secs)),
        _ => Err("bad timezone format: timezone should start with '+' or '-'".to_string()),
    }
}
