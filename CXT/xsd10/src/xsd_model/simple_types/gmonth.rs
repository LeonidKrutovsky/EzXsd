use chrono::FixedOffset;
use std::fmt;

use std::str::FromStr;
use crate::xsd_model::simple_types::parse_timezone;


// xsd:gMonth
// The type xsd:gMonth represents a specific month that recurs every year. The letter g signifies "Gregorian." xsd:gMonth can be used to indicate, for example, that fiscal year-end processing occurs in September of every year. To represent a duration of months, use the duration type instead. The format of xsd:gMonth is --MM.
//
// An optional time zone expression may be added at the end of the value. The letter Z is used to indicate Coordinated Universal Time (UTC). All other time zones are represented by their difference from Coordinated Universal Time in the format +hh:mm, or -hh:mm. These values may range from -14:00 to 14:00. For example, US Eastern Standard Time, which is five hours behind UTC, is represented as -05:00. If no time zone value is present, it is considered unknown; it is not assumed to be UTC.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
//
// Examples
// Valid values     Comment
// --04	April
// --04-05:00         April, US Eastern Standard Time
// Invalid values   Comment
// 2004-04            the year must not be specified; use gYearMonth instead
// 04                 the leading hyphens are required
// --4                the month must be 2 digits
// --13               the month must be a valid month
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gMonth

#[derive(PartialEq, Debug)]
pub struct GMonth {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GMonth {
    pub fn new(month: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if month < 1 || month > 12 {
            return Err("GMonth value should lie between 1 and 12".to_string());
        }
        Ok(GMonth {
            value: month,
            timezone,
        })
    }
}

impl Default for GMonth {
    fn default() -> GMonth {
        Self {
            value: 1,
            timezone: None,
        }
    }
}

impl FromStr for GMonth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> Result<i32, String> {
            if s.len() != 4 || &s[0..2] != "--" {
                return Err("bad gMonth format".to_string());
            }
            let token = &s[2..4];
            if !token.chars().all(|c| c.is_digit(10)) {
                return Err("bad gMonth format".to_string());
            }
            token.parse::<i32>().map_err(|e| e.to_string())
        }

        if s.ends_with('Z') {
            return GMonth::new(parse_value(&s[..s.len() - 1])?, Some(FixedOffset::east(0)));
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad gMonth format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GMonth::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        if s.matches('-').count() == 3 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[2].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GMonth::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        GMonth::new(parse_value(s)?, None)
    }
}

impl fmt::Display for GMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "--{:02}{}", self.value, tz),
            None => write!(f, "--{:02}", self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmonth_parse_test() {
        // No timezone.
        assert_eq!(
            GMonth::from_str("--12"),
            Ok(GMonth {
                value: 12,
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            GMonth::from_str("--12Z"),
            Ok(GMonth {
                value: 12,
                timezone: Some(FixedOffset::east(0))
            })
        );

        // Positive offset.
        assert_eq!(
            GMonth::from_str("--12+06:30"),
            Ok(GMonth {
                value: 12,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            })
        );

        // Negative offset.
        assert_eq!(
            GMonth::from_str("--12-06:30"),
            Ok(GMonth {
                value: 12,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            })
        );

        // Invalid values.
        assert!(GMonth::from_str("-10-").is_err());
        assert!(GMonth::from_str("--15").is_err());
        assert!(GMonth::from_str("--5").is_err());
        assert!(GMonth::from_str("11").is_err());
        assert!(GMonth::from_str("----11").is_err());
        assert!(GMonth::from_str("----1").is_err());
        assert!(GMonth::from_str("--+1").is_err());
    }

    #[test]
    fn gmonth_display_test() {
        // No timezone.
        assert_eq!(
            GMonth {
                value: 3,
                timezone: None
            }
                .to_string(),
            "--03"
        );

        // Timezone +00:00.
        assert_eq!(
            GMonth {
                value: 3,
                timezone: Some(FixedOffset::east(0))
            }
                .to_string(),
            "--03+00:00"
        );

        // Positive offset.
        assert_eq!(
            GMonth {
                value: 3,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            }
                .to_string(),
            "--03+06:30"
        );

        // Negative offset.
        assert_eq!(
            GMonth {
                value: 3,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            }
                .to_string(),
            "--03-06:30"
        );
    }
}

