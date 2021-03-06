use std::fmt;
use std::str::FromStr;

use chrono::FixedOffset;

use crate::model::simple_types::parse_timezone;

// xsd:gYear
// The type xsd:gYear represents a specific calendar year. The letter g signifies "Gregorian." The format of xsd:gYear is CCYY. No left truncation is allowed. To represent years later than 9999, additional digits can be added to the left of the year value. To represent years before 0001, a preceding minus sign ("-") is allowed.
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
// Examples
// Valid values     Comment
// 2004	2004
// 2004-05:00         2004, US Eastern Standard Time
// 12004              the year 12004
// 0922               the year 922
// -0045              45 BC
// Invalid values   Comment
// 99                  the century must not be truncated
// 922                 no left truncation is allowed; leading zeros should be added if necessary
//                     an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gYear

#[derive(PartialEq, Debug)]
pub struct GYear {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GYear {
    pub fn new(year: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if year == 0 {
            return Err("bad gYear format: year 0 occurred".to_string());
        }
        Ok(GYear {
            value: year,
            timezone,
        })
    }
}

impl Default for GYear {
    fn default() -> GYear {
        Self {
            value: 1,
            timezone: None,
        }
    }
}

impl FromStr for GYear {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('-') {
            let result = parse_str_positive(&s[1..]);
            if let Ok(mut gyear) = result {
                gyear.value *= -1;
                return Ok(gyear);
            } else {
                return result;
            }
        }
        parse_str_positive(s)
    }
}

fn parse_str_positive(s: &str) -> Result<GYear, String> {
    fn parse_value(s: &str) -> Result<i32, String> {
        if s.len() < 4 {
            return Err("bad gYear format: to short".to_string());
        }
        if !s.chars().all(|c| c.is_digit(10)) {
            return Err("bad gYear format".to_string());
        }
        s.parse::<i32>().map_err(|e| e.to_string())
    }

    if s.ends_with('Z') {
        return GYear::new(parse_value(&s[..s.len() - 1])?, Some(FixedOffset::east(0)));
    }

    if s.contains('+') {
        if s.matches('+').count() > 1 {
            return Err("bad gYear format".to_string());
        }

        let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        return GYear::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
    }

    if s.contains('-') {
        if s.matches('-').count() > 1 {
            return Err("bad gYear format".to_string());
        }

        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        return GYear::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
    }

    GYear::new(parse_value(s)?, None)
}

impl fmt::Display for GYear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value > 0 {
            match self.timezone {
                Some(tz) => write!(f, "{:04}{}", self.value, tz),
                None => write!(f, "{:04}", self.value),
            }
        } else {
            match self.timezone {
                Some(tz) => write!(f, "-{:04}{}", -self.value, tz),
                None => write!(f, "-{:04}", -self.value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gyear_parse_test() {
        // No timezone.
        assert_eq!(
            GYear::from_str("2020"),
            Ok(GYear {
                value: 2020,
                timezone: None,
            })
        );

        // Timezone "Z".
        assert_eq!(
            GYear::from_str("2020Z"),
            Ok(GYear {
                value: 2020,
                timezone: Some(FixedOffset::east(0)),
            })
        );

        // Positive offset.
        assert_eq!(
            GYear::from_str("2020+06:30"),
            Ok(GYear {
                value: 2020,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)),
            })
        );

        // Negative offset.
        assert_eq!(
            GYear::from_str("2020-06:30"),
            Ok(GYear {
                value: 2020,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Negative year.
        assert_eq!(
            GYear::from_str("-0020-06:30"),
            Ok(GYear {
                value: -20,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Negative year with five digits.
        assert_eq!(
            GYear::from_str("-20000-06:30"),
            Ok(GYear {
                value: -20000,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            })
        );

        // Invalid values.
        assert!(GYear::from_str("01").is_err());
        assert!(GYear::from_str("2001-12").is_err());
        assert!(GYear::from_str("0000").is_err());
        assert!(GYear::from_str("+123").is_err());
    }

    #[test]
    fn gyear_display_test() {
        // No timezone.
        assert_eq!(
            GYear {
                value: 987,
                timezone: None,
            }
                .to_string(),
            "0987"
        );

        // Timezone +00:00.
        assert_eq!(
            GYear {
                value: 987,
                timezone: Some(FixedOffset::east(0)),
            }
                .to_string(),
            "0987+00:00"
        );

        // Positive offset.
        assert_eq!(
            GYear {
                value: 987,
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)),
            }
                .to_string(),
            "0987+06:30"
        );

        // Negative offset.
        assert_eq!(
            GYear {
                value: 987,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
                .to_string(),
            "0987-06:30"
        );

        // Negative year.
        assert_eq!(
            GYear {
                value: -987,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
                .to_string(),
            "-0987-06:30"
        );

        // Negative year with five digits.
        assert_eq!(
            GYear {
                value: -98765,
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)),
            }
                .to_string(),
            "-98765-06:30"
        );
    }
}
