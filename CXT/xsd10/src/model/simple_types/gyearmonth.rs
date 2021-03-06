use std::fmt;
use std::str::FromStr;

use chrono::FixedOffset;

use crate::model::simple_types::gmonth::GMonth;
use crate::model::simple_types::gyear::GYear;
use crate::model::simple_types::utils::parse_timezone;

// xsd:gYearMonth
// The type xsd:gYearMonth represents a specific month of a specific year. The letter g signifies "Gregorian." The format of xsd:gYearMonth is CCYY-MM. No left truncation is allowed on either part. To represents years later than 9999, additional digits can be added to the left of the year value. To represent years before 0001, a preceding minus sign ("-") is permitted.
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
// 2004-04            April 2004
// 2004-04-05:00      April 2004, US Eastern Standard Time
// Invalid values   Comment
// 99-04              the century must not be truncated
// 2004               the month is required
// 2004-4             the month must be two digits
// 2004-13            the month must be a valid month
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gYearMonth

#[derive(PartialEq, Debug)]
pub struct GYearMonth {
    pub year: i32,
    pub month: i32,
    pub timezone: Option<FixedOffset>,
}

impl GYearMonth {
    pub fn new(year: i32, month: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if year == 0 {
            return Err("bad gYear format: year 0 occurred".to_string());
        }

        if month < 1 || month > 12 {
            return Err("Month value within GYearMonth should lie between 1 and 12".to_string());
        }

        Ok(GYearMonth {
            year,
            month,
            timezone,
        })
    }

    pub fn gyear(self) -> GYear {
        GYear {
            value: self.year,
            timezone: self.timezone,
        }
    }

    pub fn gmonth(self) -> GMonth {
        GMonth {
            value: self.month,
            timezone: self.timezone,
        }
    }
}

impl Default for GYearMonth {
    fn default() -> GYearMonth {
        Self {
            year: 1,
            month: 1,
            timezone: None,
        }
    }
}

impl FromStr for GYearMonth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('-') {
            let result = parse_str_positive(&s[1..]);
            if let Ok(mut gyearmonth) = result {
                gyearmonth.year *= -1;
                return Ok(gyearmonth);
            } else {
                return result;
            }
        }
        parse_str_positive(s)
    }
}

fn parse_str_positive(s: &str) -> Result<GYearMonth, String> {
    fn parse_value(s: &str) -> Result<(i32, i32), String> {
        if s.matches('-').count() != 1 {
            return Err("bad gYearMonth format".to_string());
        }

        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[0].0;
        let year_token = &s[..idx];
        let month_token = &s[idx + 1..];
        if year_token.len() < 4 || month_token.len() != 2 {
            return Err("bad gYearMonth format".to_string());
        }

        if !year_token.chars().all(|c| c.is_digit(10)) {
            return Err("bad year format within gYearMonth".to_string());
        }
        let year = year_token.parse::<i32>().map_err(|e| e.to_string())?;

        if !month_token.chars().all(|c| c.is_digit(10)) {
            return Err("bad month format within gYearMonth".to_string());
        }
        let month = month_token.parse::<i32>().map_err(|e| e.to_string())?;

        Ok((year, month))
    }

    if s.ends_with('Z') {
        let (year, month) = parse_value(&s[..s.len() - 1])?;
        return GYearMonth::new(year, month, Some(FixedOffset::east(0)));
    }

    if s.contains('+') {
        if s.matches('+').count() > 1 {
            return Err("bad gMonthDay format".to_string());
        }

        let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        let (year, month) = parse_value(value_token)?;
        return GYearMonth::new(year, month, Some(parse_timezone(tz_token)?));
    }

    if s.matches('-').count() == 2 {
        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[1].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        let (year, month) = parse_value(value_token)?;
        return GYearMonth::new(year, month, Some(parse_timezone(tz_token)?));
    }

    let (year, month) = parse_value(s)?;
    GYearMonth::new(year, month, None)
}

impl fmt::Display for GYearMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.year > 0 {
            match self.timezone {
                Some(tz) => write!(f, "{:04}-{:02}{}", self.year, self.month, tz),
                None => write!(f, "{:04}-{:02}", self.year, self.month),
            }
        } else {
            match self.timezone {
                Some(tz) => write!(f, "-{:04}-{:02}{}", -self.year, self.month, tz),
                None => write!(f, "-{:04}-{:02}", -self.year, self.month),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gyearmonth_parse_test() {
        // No timezone.
        assert_eq!(
            GYearMonth::from_str("2020-03"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: None,
            })
        );

        // Timezone "Z".
        assert_eq!(
            GYearMonth::from_str("2020-03Z"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::east(0)),
            })
        );

        // Positive offset.
        assert_eq!(
            GYearMonth::from_str("2020-03+06:30"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)),
            })
        );

        // Negative offset.
        assert_eq!(
            GYearMonth::from_str("2020-03-06:30"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Negative year.
        assert_eq!(
            GYearMonth::from_str("-0020-03-06:30"),
            Ok(GYearMonth {
                year: -20,
                month: 3,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Negative year with five digits.
        assert_eq!(
            GYearMonth::from_str("-20000-03-06:30"),
            Ok(GYearMonth {
                year: -20000,
                month: 3,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Invalid values.
        assert!(GYearMonth::from_str("01-03").is_err());
        assert!(GYearMonth::from_str("2000-1").is_err());
        assert!(GYearMonth::from_str("2000-13").is_err());
        assert!(GYearMonth::from_str("2000-00").is_err());
        assert!(GYearMonth::from_str("0000-03").is_err());
        assert!(GYearMonth::from_str("2000-+3").is_err());
        assert!(GYearMonth::from_str("-200-03").is_err());
        assert!(GYearMonth::from_str("+200-03").is_err());
        assert!(GYearMonth::from_str("++++-++").is_err());
    }

    #[test]
    fn gyearmonth_display_test() {
        // No timezone.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: None,
            }
            .to_string(),
            "0987-06"
        );

        // Timezone +00:00.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::east(0)),
            }
            .to_string(),
            "0987-06+00:00"
        );

        // Positive offset.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)),
            }
            .to_string(),
            "0987-06+06:30"
        );

        // Negative offset.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
            .to_string(),
            "0987-06-06:30"
        );

        // Negative year.
        assert_eq!(
            GYearMonth {
                year: -987,
                month: 6,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
            .to_string(),
            "-0987-06-06:30"
        );

        // Negative year with five digits.
        assert_eq!(
            GYearMonth {
                year: -98765,
                month: 6,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
            .to_string(),
            "-98765-06-06:30"
        );
    }
}
