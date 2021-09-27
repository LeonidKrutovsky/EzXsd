use crate::model::elements;

use xml_utils::group;

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
#[group()]
pub enum Facets {
    MinExclusive(elements::MinExclusive),
    MinInclusive(elements::MinInclusive),
    MaxExclusive(elements::MaxExclusive),
    MaxInclusive(elements::MaxInclusive),
    TotalDigits(elements::TotalDigits),
    FractionDigits(elements::FractionDigits),
    Length(elements::Length),
    MinLength(elements::MinLength),
    MaxLength(elements::MaxLength),
    Enumeration(elements::Enumeration),
    WhiteSpace(elements::WhiteSpace),
    Pattern(elements::Pattern),
}
