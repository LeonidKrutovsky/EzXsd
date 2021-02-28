use crate::model::attributes::abstract_::Abstract;
use crate::model::simple_types::Boolean;
use crate::model::RawAttribute;
use std::convert::TryFrom;
use std::str::FromStr;

impl TryFrom<RawAttribute<'_>> for Abstract {
    type Error = String;

    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        Boolean::from_str(value.value())
    }
}
