use roxmltree::Node;

use crate::model::complex_types::facet::Facet;
use crate::model::complex_types::no_fixed_facet::NoFixedFacet;
use crate::model::complex_types::num_facet::NumFacet;
use crate::model::elements::ElementType;
use crate::model::groups::facets::Facets;
use crate::model::{Pattern, TotalDigits, WhiteSpace};
use crate::xml_to_xsd::utils::annotation_only;
use std::convert::TryInto;
use crate::model::attributes::fixed::FixedBool;
use crate::model::attributes::AnyAttributes;

impl Facet {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "facet")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "value" => res.value = attr.try_into()?,
                "fixed" => res.fixed = attr.try_into()?,

                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}

impl TotalDigits {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "totalDigits")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "value" => res.value = attr.try_into()?,
                "fixed" => res.fixed = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}

impl NumFacet {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "numFacet")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.value().parse()?),
                "value" => res.value = attr.value().parse()?,
                "fixed" => res.fixed = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}

impl NoFixedFacet {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "noFixedFacet")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "value" => res.value = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}

impl WhiteSpace {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let annotation = annotation_only(node, "whiteSpace")?;

        let mut id = None;
        let mut value = None;
        let mut fixed= FixedBool::default() ;
        let mut attributes = AnyAttributes::default();
        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                "value" => value = Some(attr.try_into()?),
                "fixed" => fixed = attr.try_into()?,
                _ => attributes.push(attr.try_into()?),
            };
        }

        let value =
            value.ok_or_else(|| "value attribute required for xsd:whiteSpace".to_string())?;

        Ok(Self {
            annotation,
            id,
            value,
            fixed,
            attributes,
        })
    }
}

impl Pattern {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "whiteSpace")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                "value" => res.value = attr.try_into()?,
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}

impl Facets {
    pub fn parse(node: Node<'_, '_>, element_type: ElementType) -> Result<Option<Self>, String> {
        Ok(Some(match element_type {
            ElementType::MinExclusive => Self::MinExclusive(Facet::parse(node)?),
            ElementType::MinInclusive => Self::MinInclusive(Facet::parse(node)?),
            ElementType::MaxExclusive => Self::MaxExclusive(Facet::parse(node)?),
            ElementType::MaxInclusive => Self::MaxInclusive(Facet::parse(node)?),
            ElementType::TotalDigits => Self::TotalDigits(TotalDigits::parse(node)?),
            ElementType::FractionDigits => Self::FractionDigits(NumFacet::parse(node)?),
            ElementType::Length => Self::Length(NumFacet::parse(node)?),
            ElementType::MinLength => Self::MinLength(NumFacet::parse(node)?),
            ElementType::MaxLength => Self::MaxLength(NumFacet::parse(node)?),
            ElementType::Enumeration => Self::Enumeration(NoFixedFacet::parse(node)?),
            ElementType::WhiteSpace => Self::WhiteSpace(WhiteSpace::parse(node)?),
            ElementType::Pattern => Self::Pattern(Pattern::parse(node)?),
            _ => return Ok(None),
        }))
    }
}
