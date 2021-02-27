use crate::model::attributes::attribute_form_default::AttributeFormDefault;
use crate::model::RawAttribute;
use std::convert::TryFrom;

impl TryFrom<RawAttribute<'_>> for AttributeFormDefault {
    type Error = String;

    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        match value.value() {
            "qualified" => Ok(Self::Qualified),
            "unqualified" => Ok(Self::Unqualified),
            _ => Err(format!("Invalid attribute {:?}", value)),
        }
    }
}
