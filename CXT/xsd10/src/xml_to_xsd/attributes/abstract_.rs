use crate::model::attributes::abstract_::Abstract;
use crate::model::simple_types::Boolean;
use crate::model::RawAttribute;
use std::convert::TryFrom;

impl TryFrom<RawAttribute<'_>> for Abstract {
    type Error = String;

    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        Boolean::try_from(value.value())
    }
}
