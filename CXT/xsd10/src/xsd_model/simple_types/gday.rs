use chrono::FixedOffset;
use std::fmt;

use crate::xsd_model::simple_types::parse_timezone;
use std::str::FromStr;

// xsd:gDay
// The type xsd:gDay represents a day that recurs every month. The letter g signifies "Gregorian." xsd:gDay can be used to say, for example, that checks are paid on the 5th of each month. To represent a duration of days, use the duration type instead. The format of gDay is ---DD.
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
// ---02              the 2nd of the month
// Invalid values   Comment
// 02                 the leading hyphens are required
// ---2               the day must be 2 digits
// ---32              the day must be a valid day of the month; no month has 32 days
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gDay

#[derive(PartialEq, Debug)]
pub struct GDay {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GDay {
    pub fn new(day: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if day < 1 || day > 31 {
            return Err("gDay value should lie between 1 and 31".to_string());
        }
        Ok(GDay {
            value: day,
            timezone,
        })
    }
}

impl Default for GDay {
    fn default() -> GDay {
        Self {
            value: 1,
            timezone: None,
        }
    }
}

impl FromStr for GDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> Result<i32, String> {
            if s.len() != 5 || &s[0..3] != "---" {
                return Err("bad gDay format".to_string());
            }
            let token = &s[3..5];
            if !token.chars().all(|c| c.is_digit(10)) {
                return Err("bad gDay format".to_string());
            }
            token.parse::<i32>().map_err(|e| e.to_string())
        }

        if s.ends_with('Z') {
            return GDay::new(parse_value(&s[..s.len() - 1])?, Some(FixedOffset::east(0)));
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad gDay format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GDay::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        if s.matches('-').count() == 4 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[3].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GDay::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        GDay::new(parse_value(s)?, None)
    }
}

impl fmt::Display for GDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "---{:02}{}", self.value, tz),
            None => write!(f, "---{:02}", self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gday_parse_test() {
        // No timezone.
        assert_eq!(
            GDay::from_str("---25"),
            Ok(GDay {
                value: 25,
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            GDay::from_str("---25Z"),
            Ok(GDay {
                value: 25,
                timezone: Some(FixedOffset::east(0))
            })
        );

        // Positive offset.
        assert_eq!(
            GDay::from_str("---25+06:30"),
            Ok(GDay {
                value: 25,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            })
        );

        // Negative offset.
        assert_eq!(
            GDay::from_str("---25-06:30"),
            Ok(GDay {
                value: 25,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            })
        );

        // Invalid values.
        assert!(GDay::from_str("--30-").is_err());
        assert!(GDay::from_str("---35").is_err());
        assert!(GDay::from_str("---5").is_err());
        assert!(GDay::from_str("15").is_err());
        assert!(GDay::from_str("----15").is_err());
        assert!(GDay::from_str("----5").is_err());
        assert!(GDay::from_str("---+5").is_err());
    }

    #[test]
    fn gday_display_test() {
        // No timezone.
        assert_eq!(
            GDay {
                value: 3,
                timezone: None
            }
            .to_string(),
            "---03"
        );

        // Timezone +00:00.
        assert_eq!(
            GDay {
                value: 3,
                timezone: Some(FixedOffset::east(0))
            }
            .to_string(),
            "---03+00:00"
        );

        // Positive offset.
        assert_eq!(
            GDay {
                value: 3,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "---03+06:30"
        );

        // Negative offset.
        assert_eq!(
            GDay {
                value: 3,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "---03-06:30"
        );
    }
}
