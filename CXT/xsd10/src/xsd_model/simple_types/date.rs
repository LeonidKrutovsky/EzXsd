use crate::xsd_model::simple_types::parse_timezone;
use chrono::{format::strftime::StrftimeItems, FixedOffset, NaiveDate};
use std::fmt;
use std::str::FromStr;

// xsd:date
// The type xsd:date represents a Gregorian calendar date in the format CCYY-MM-DD where CC represents the century, YY the year, MM the month and DD the day. No left truncation is allowed for any part of the date. To represent years later than 9999, additional digits can be added to the left of the year value, but extra leading zeros are not permitted. To represent years before 0001, a preceding minus sign ("-") is allowed. The year 0000 is not a valid year in the Gregorian calendar.
//
// An optional time zone expression may be added at the end. The letter Z is used to indicate Coordinated Universal Time (UTC). All other time zones are represented by their difference from Coordinated Universal Time in the format +hh:mm, or -hh:mm. These values may range from -14:00 to 14:00. For example, US Eastern Standard Time, which is five hours behind UTC, is represented as -05:00. If no time zone value is present, it is considered unknown; it is not assumed to be UTC.
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
// Valid values	    Comment
// 2004-04-12         April 12, 2004
// -0045-01-01        January 1, 45 BC
// 12004-04-12        April 12, 12004
// 2004-04-12-05:00	  April 12, 2004, US Eastern Standard Time, which is 5 hours behind Coordinated Universal Time (UTC)
// 2004-04-12Z	      April 12, 2004, Coordinated Universal Time (UTC)
// Invalid values   Comment
// 99-04-12           left truncation of the century is not allowed
// 2004-4-2           month and day must be two digits each
// 2004/04/02         slashes are not valid separators
// 04-12-2004         the value must be in CCYY-MM-DD order
// 2004-04-31         the date must be a valid date (April has 30 days)
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:date

#[derive(PartialEq, Debug)]
pub struct Date {
    pub value: NaiveDate,
    pub timezone: Option<FixedOffset>,
}

impl Date {
    pub fn from_chrono_naive_date(date: NaiveDate) -> Self {
        Date {
            value: date,
            timezone: None,
        }
    }

    pub fn to_chrono_naive_date(&self) -> NaiveDate {
        self.value
    }
}

impl Default for Date {
    fn default() -> Date {
        Self {
            value: NaiveDate::from_ymd(1, 1, 1),
            timezone: None,
        }
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_naive_date(s: &str) -> Result<NaiveDate, String> {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|e| e.to_string())
        }

        if s.ends_with('Z') {
            return Ok(Date {
                value: parse_naive_date(&s[..s.len() - 1])?,
                timezone: Some(FixedOffset::east(0)),
            });
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let date_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Date {
                value: parse_naive_date(date_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        if s.matches('-').count() == 3 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[2].0;
            let date_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Date {
                value: parse_naive_date(date_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        Ok(Date {
            value: parse_naive_date(s)?,
            timezone: None,
        })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%Y-%m-%d");
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
    fn date_parse_test() {
        // No timezone.
        assert_eq!(
            Date::from_str("2020-02-02"),
            Ok(Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            Date::from_str("2020-02-02Z"),
            Ok(Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::east(0))
            })
        );

        // Positive offset.
        assert_eq!(
            Date::from_str("2020-02-02+06:30"),
            Ok(Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            })
        );

        // Negative offset.
        assert_eq!(
            Date::from_str("2020-02-02-06:30"),
            Ok(Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            })
        );
    }

    #[test]
    fn date_display_test() {
        // No timezone.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: None
            }
            .to_string(),
            "2020-02-02"
        );

        // Timezone +00:00.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::east(0))
            }
            .to_string(),
            "2020-02-02+00:00"
        );

        // Positive offset.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "2020-02-02+06:30"
        );

        // Negative offset.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd(2020, 2, 2),
                timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60))
            }
            .to_string(),
            "2020-02-02-06:30"
        );
    }
}
