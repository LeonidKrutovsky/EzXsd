use crate::model::elements::enumeration::Enumeration;
use crate::model::elements::fraction_digits::FractionDigits;
use crate::model::elements::length::Length;
use crate::model::elements::max_exclusive::MaxExclusive;
use crate::model::elements::max_inclusive::MaxInclusive;
use crate::model::elements::max_length::MaxLength;
use crate::model::elements::min_exclusive::MinExclusive;
use crate::model::elements::min_inclusive::MinInclusive;
use crate::model::elements::min_length::MinLength;
use crate::model::elements::pattern::Pattern;
use crate::model::elements::total_digits::TotalDigits;
use crate::model::elements::white_space::WhiteSpace;

// xsd:facets
//        We should use a substitution group for facets, but
//        that's ruled out because it would allow users to
//        add their own, which we're not ready for yet.
//
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Choice [1..1]
//      xsd:minExclusive
//      xsd:minInclusive
//      xsd:maxExclusive
//      xsd:maxInclusive
//      xsd:totalDigits
//      xsd:fractionDigits
//      xsd:length
//      xsd:minLength
//      xsd:maxLength
//      xsd:enumeration
//      xsd:whiteSpace
//      xsd:pattern
//
// Used in
// Group xsd:simpleRestrictionModel
#[derive(Debug)]
pub enum Facets {
    MinExclusive(MinExclusive),
    MinInclusive(MinInclusive),
    MaxExclusive(MaxExclusive),
    MaxInclusive(MaxInclusive),
    TotalDigits(TotalDigits),
    FractionDigits(FractionDigits),
    Length(Length),
    MinLength(MinLength),
    MaxLength(MaxLength),
    Enumeration(Enumeration),
    WhiteSpace(WhiteSpace),
    Pattern(Pattern),
}
