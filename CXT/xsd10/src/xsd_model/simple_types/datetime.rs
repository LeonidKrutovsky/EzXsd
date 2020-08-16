use chrono::{format::ParseError, DateTime as CDateTime, FixedOffset};
use std::fmt;
use std::str::FromStr;

// xsd:dateTime
// The type xsd:dateTime represents a specific date and time in the format CCYY-MM-DDThh:mm:ss.sss, which is a concatenation of the date and time forms, separated by a literal letter "T". All of the same rules that apply to the date and time types are applicable to xsd:dateTime as well.
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
// Valid values              Comment
// 2004-04-12T13:20:00         1:20 pm on April 12, 2004
// 2004-04-12T13:20:15.5       1:20 pm and 15.5 seconds on April 12, 2004
// 2004-04-12T13:20:00-05:00   1:20 pm on April 12, 2004, US Eastern Standard Time
// 2004-04-12T13:20:00Z	1:20   pm on April 12, 2004, Coordinated Universal Time (UTC)
// Invalid values            Comment
// 2004-04-12T13:00            seconds must be specified
// 2004-04-1213:20:00          the letter T is required
// 99-04-12T13:00              the century must not be left truncated
// 2004-04-12                  the time is required
//                             an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:dateTime

#[derive(PartialEq, PartialOrd, Debug)]
pub struct DateTime {
    pub value: CDateTime<FixedOffset>,
}

impl DateTime {
    pub fn from_chrono_datetime(datetime: CDateTime<FixedOffset>) -> Self {
        DateTime { value: datetime }
    }

    pub fn to_chrono_datetime(&self) -> CDateTime<FixedOffset> {
        self.value
    }
}

impl Default for DateTime {
    fn default() -> DateTime {
        Self {
            value: CDateTime::parse_from_rfc3339("0001-01-01T00:00:00Z").unwrap(),
        }
    }
}

impl FromStr for DateTime {
    type Err = ParseError;

    // Note:
    // `parse_from_rfc3339` parses an RFC 3339 and ISO 8601 date and time string.
    // XSD follows ISO 8601, which allows no time zone at the end of literal.
    // Since RFC 3339 does not allow such behavior, the function tries to add
    // 'Z' (which equals "+00:00") in case there is no timezone provided.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tz_provided = s.ends_with('Z') || s.contains('+') || s.matches('-').count() == 3;
        let s_with_timezone = if tz_provided {
            s.to_string()
        } else {
            format!("{}Z", s)
        };
        match CDateTime::parse_from_rfc3339(&s_with_timezone) {
            Ok(cdt) => Ok(DateTime { value: cdt }),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn datetime_parse_test() {
        // No timezone.
        let offset = FixedOffset::east(0);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00"),
            Ok(DateTime { value: dt })
        );
        // Timezone "Z".
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00Z"),
            Ok(DateTime { value: dt })
        );

        // Positive offset.
        let offset = FixedOffset::east(6 * 3600 + 30 * 60);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00+06:30"),
            Ok(DateTime { value: dt })
        );

        // Negative offset.
        let offset = FixedOffset::west(6 * 3600 + 30 * 60);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00-06:30"),
            Ok(DateTime { value: dt })
        );
    }

    #[test]
    fn datetime_display_test() {
        // Timezone +00:00.
        let offset = FixedOffset::east(0);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime { value: dt }.to_string(),
            "2020-03-07T04:40:00+00:00"
        );

        // Positive offset.
        let offset = FixedOffset::east(6 * 3600 + 30 * 60);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime { value: dt }.to_string(),
            "2020-03-07T04:40:00+06:30"
        );

        // Negative offset.
        let offset = FixedOffset::west(6 * 3600 + 30 * 60);
        let dt_utc = NaiveDate::from_ymd(2020, 3, 7).and_hms(4, 40, 0) - offset;
        let dt = CDateTime::<FixedOffset>::from_utc(dt_utc, offset);
        assert_eq!(
            DateTime { value: dt }.to_string(),
            "2020-03-07T04:40:00-06:30"
        );
    }
}
