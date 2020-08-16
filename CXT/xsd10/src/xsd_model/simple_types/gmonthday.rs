use crate::xsd_model::simple_types::gday::GDay;
use crate::xsd_model::simple_types::gmonth::GMonth;
use chrono::FixedOffset;
use std::fmt;

use crate::xsd_model::simple_types::parse_timezone;
use std::str::FromStr;

// xsd:gMonthDay
// The type xsd:gMonthDay represents a specific day that recurs every year. The letter g signifies "Gregorian." xsd:gMonthDay can be used to say, for example, that your birthday is on the 14th of April every year. The format of xsd:gMonthDay is --MM-DD.
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
// --04-12            April 12
// --04-12Z           April 12, Coordinated Universal Time (UTC)
// Invalid values   Comment
// 04-12              the leading hyphens are required
// --04-31            it must be a valid day of the year (April has 30 days)
// --4-6              the month and day must be 2 digits each
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gMonthDay

#[derive(PartialEq, Debug)]
pub struct GMonthDay {
    pub month: i32,
    pub day: i32,
    pub timezone: Option<FixedOffset>,
}

impl GMonthDay {
    pub fn new(month: i32, day: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if month < 1 || month > 12 {
            return Err("Month value within GMonthDay should lie between 1 and 12".to_string());
        }

        if day < 1 || day > 31 {
            return Err("Day value within GMonthDay should lie between 1 and 31".to_string());
        }

        const MONTH_MAX_LEN: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if day > MONTH_MAX_LEN[month as usize - 1] {
            return Err("Day value within GMonthDay is to big for specified month".to_string());
        }

        Ok(GMonthDay {
            month,
            day,
            timezone,
        })
    }

    pub fn gmonth(self) -> GMonth {
        GMonth {
            value: self.month,
            timezone: self.timezone,
        }
    }

    pub fn gday(self) -> GDay {
        GDay {
            value: self.day,
            timezone: self.timezone,
        }
    }
}

impl Default for GMonthDay {
    fn default() -> GMonthDay {
        Self {
            month: 1,
            day: 1,
            timezone: None,
        }
    }
}

impl FromStr for GMonthDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> Result<(i32, i32), String> {
            if s.len() != 7 || &s[0..2] != "--" || &s[4..5] != "-" {
                return Err("bad gMonthDay format".to_string());
            }

            let month_token = &s[2..4];
            if !month_token.chars().all(|c| c.is_digit(10)) {
                return Err("bad month format within gMonthDay".to_string());
            }
            let month = month_token.parse::<i32>().map_err(|e| e.to_string())?;

            let day_token = &s[5..7];
            if !day_token.chars().all(|c| c.is_digit(10)) {
                return Err("bad day format within gMonthDay".to_string());
            }
            let day = day_token.parse::<i32>().map_err(|e| e.to_string())?;

            Ok((month, day))
        }

        if s.ends_with('Z') {
            let (month, day) = parse_value(&s[..s.len() - 1])?;
            return GMonthDay::new(month, day, Some(FixedOffset::east(0)));
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad gMonthDay format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            let (month, day) = parse_value(value_token)?;
            return GMonthDay::new(month, day, Some(parse_timezone(tz_token)?));
        }

        if s.matches('-').count() == 4 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[3].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            let (month, day) = parse_value(value_token)?;
            return GMonthDay::new(month, day, Some(parse_timezone(tz_token)?));
        }

        let (month, day) = parse_value(s)?;
        GMonthDay::new(month, day, None)
    }
}

impl fmt::Display for GMonthDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "--{:02}-{:02}{}", self.month, self.day, tz),
            None => write!(f, "--{:02}-{:02}", self.month, self.day),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmonthday_parse_test() {
        // No timezone.
        assert_eq!(
            GMonthDay::from_str("--12-20"),
            Ok(GMonthDay {
                month: 12,
                day: 20,
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            GMonthDay::from_str("--12-20Z"),
            Ok(GMonthDay {
                month: 12,
                day: 20,
                timezone: Some(FixedOffset::east(0))
            })
        );

        // Positive offset.
        assert_eq!(
            GMonthDay::from_str("--12-20+06:30"),
            Ok(GMonthDay {
                month: 12,
                day: 20,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            })
        );

        // Negative offset.
        assert_eq!(
            GMonthDay::from_str("--12-20-06:30"),
            Ok(GMonthDay {
                month: 12,
                day: 20,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            })
        );

        // Invalid values.
        assert!(GMonthDay::from_str("-01-30-").is_err());
        assert!(GMonthDay::from_str("--01-35").is_err());
        assert!(GMonthDay::from_str("--1-5").is_err());
        assert!(GMonthDay::from_str("01-15").is_err());
        assert!(GMonthDay::from_str("01---").is_err());
        assert!(GMonthDay::from_str("AA-AA").is_err());
        assert!(GMonthDay::from_str("++-++").is_err());
        assert!(GMonthDay::from_str("+1-01").is_err());
        assert!(GMonthDay::from_str("01-+1").is_err());
        // Specific month length breach.
        assert!(GMonthDay::from_str("--02-30").is_err());
        assert!(GMonthDay::from_str("--04-31").is_err());
    }

    #[test]
    fn gmonthday_display_test() {
        // No timezone.
        assert_eq!(
            GMonthDay {
                month: 3,
                day: 2,
                timezone: None
            }
            .to_string(),
            "--03-02"
        );

        // Timezone +00:00.
        assert_eq!(
            GMonthDay {
                month: 3,
                day: 2,
                timezone: Some(FixedOffset::east(0))
            }
            .to_string(),
            "--03-02+00:00"
        );

        // Positive offset.
        assert_eq!(
            GMonthDay {
                month: 3,
                day: 2,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "--03-02+06:30"
        );

        // Negative offset.
        assert_eq!(
            GMonthDay {
                month: 3,
                day: 2,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "--03-02-06:30"
        );
    }
}
