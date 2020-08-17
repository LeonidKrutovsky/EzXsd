use chrono::{format::strftime::StrftimeItems, FixedOffset, NaiveTime};
use std::fmt;

use std::str::FromStr;

use crate::model::simple_types::parse_timezone;

// xsd:time
// The type xsd:time represents a time of day in the format hh:mm:ss.sss where hh represents the hour, mm the minutes, and ss.sss the seconds. An unlimited number of additional digits can be used to increase the precision of fractional seconds if desired. The time is based on a 24-hour time period, so hours should be represented as 00 through 24. Either of the values 00:00:00 or 24:00:00 can be used to represent midnight.
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
// Valid values       Comment
// 13:20:00	1:20 PM
// 13:20:30.5555        1:20 PM and 30.5555 seconds
// 13:20:00-05:00       1:20 PM, US Eastern Standard Time
// 13:20:00Z            1:20 PM, Coordinated Universal Time (UTC)
// 00:00:00             midnight
// 24:00:00             midnight
// Invalid values     Comment
// 5:20:00              hours, minutes, and seconds must be two digits each
// 13:20                seconds must be specified, even if it is 00
// 13:20.5:00           values for hours and minutes must be integers
// 13:65:00             the value must be a valid time of day
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:time

#[derive(PartialEq, Debug)]
pub struct Time {
    pub value: NaiveTime,
    pub timezone: Option<FixedOffset>,
}

impl Time {
    pub fn from_chrono_naive_time(time: NaiveTime) -> Self {
        Time {
            value: time,
            timezone: None,
        }
    }

    pub fn to_chrono_naive_time(&self) -> NaiveTime {
        self.value
    }
}

impl Default for Time {
    fn default() -> Time {
        Self {
            value: NaiveTime::from_hms(0, 0, 0),
            timezone: None,
        }
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_naive_time(s: &str) -> Result<NaiveTime, String> {
            NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(|e| e.to_string())
        }

        if s.ends_with('Z') {
            return Ok(Time {
                value: parse_naive_time(&s[..s.len() - 1])?,
                timezone: Some(FixedOffset::east(0)),
            });
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let time_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Time {
                value: parse_naive_time(time_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        if s.contains('-') {
            if s.matches('-').count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[0].0;
            let time_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Time {
                value: parse_naive_time(time_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        Ok(Time {
            value: parse_naive_time(s)?,
            timezone: None,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%H:%M:%S");
        match self.timezone {
            Some(tz) => write!(f, "{}{}", self.value.format_with_items(fmt.clone()), tz),
            None => write!(f, "{}", self.value.format_with_items(fmt.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_parse_test() {
        // No timezone.
        assert_eq!(
            Time::from_str("04:40:00"),
            Ok(Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            Time::from_str("04:40:00Z"),
            Ok(Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::east(0))
            })
        );

        // Positive offset.
        assert_eq!(
            Time::from_str("04:40:00+06:30"),
            Ok(Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            })
        );

        // Negative offset.
        assert_eq!(
            Time::from_str("04:40:00-06:30"),
            Ok(Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            })
        );
    }

    #[test]
    fn time_display_test() {
        // No timezone.
        assert_eq!(
            Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: None
            }
            .to_string(),
            "04:40:00"
        );

        // Timezone +00:00.
        assert_eq!(
            Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::east(0))
            }
            .to_string(),
            "04:40:00+00:00"
        );

        // Positive offset.
        assert_eq!(
            Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "04:40:00+06:30"
        );

        // Negative offset.
        assert_eq!(
            Time {
                value: NaiveTime::from_hms(4, 40, 0),
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "04:40:00-06:30"
        );
    }
}
