use chrono::FixedOffset;
use regex::Regex;

//No normalization is done, the value is not changed
// (this is the behavior required by [XML 1.0 (Second Edition)]
// for element content)
// fn preserve(value: &str) - empty func

//All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return)
// are replaced with #x20 (space)
#[allow(dead_code)]
pub fn replace(value: &str) -> String {
    value
        .chars()
        .map(|x| if x.is_whitespace() { ' ' } else { x })
        .collect()
}

pub fn is_replaced<T: AsRef<str>>(value: T) -> bool {
    !value
        .as_ref()
        .chars()
        .any(|c| c.is_whitespace() && c != ' ')
}

pub fn assert_replaced<T: AsRef<str>>(value: T, name: T) -> Result<(), String> {
    if is_replaced(value.as_ref()) {
        Ok(())
    } else {
        Err(format!(
            "Invalid value for {}. White space must be replaced: {}",
            name.as_ref(),
            value.as_ref()
        ))
    }
}

//After the processing implied by replace, contiguous sequences
// of #x20's are collapsed to a single #x20, and leading and trailing
// #x20's are removed.
#[allow(dead_code)]
pub fn collapse(value: &str) -> String {
    let re = Regex::new(" {2,}").unwrap();
    re.replace_all(replace(value.trim()).as_str(), " ").into()
}

pub fn is_collapsed<T: AsRef<str>>(value: T) -> bool {
    let value = value.as_ref();

    let res = !is_replaced(value)
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.contains("  ");

    !res
}

pub fn assert_collapsed<T: AsRef<str>>(value: T, name: T) -> Result<(), String> {
    if is_collapsed(value.as_ref()) {
        Ok(())
    } else {
        Err(format!(
            "Invalid value for {}. White spaces must be collapsed: {}",
            name.as_ref(),
            value.as_ref()
        ))
    }
}

// Parses ISO 8601 timezone.
pub fn parse_timezone(s: &str) -> Result<FixedOffset, String> {
    if s == "Z" {
        return Ok(FixedOffset::east(0));
    }

    let tokens: Vec<&str> = s[1..].split(':').collect();
    if tokens.len() != 2 || tokens[0].len() != 2 || tokens[1].len() != 2 {
        return Err("bad timezone format".to_string());
    }
    if !tokens.iter().all(|t| t.chars().all(|c| c.is_digit(10))) {
        return Err("bad timezone format".to_string());
    }

    let hours = tokens[0].parse::<i32>().unwrap();
    let minutes = tokens[1].parse::<i32>().unwrap();

    if hours > 14 || (hours == 14 && minutes != 0) || minutes >= 60 {
        return Err("bad timezone format: out of range".to_string());
    }

    let offset_secs = 60 * (60 * hours + minutes);
    match s.chars().next().unwrap() {
        '+' => Ok(FixedOffset::east(offset_secs)),
        '-' => Ok(FixedOffset::west(offset_secs)),
        _ => Err("bad timezone format: timezone should start with '+' or '-'".to_string()),
    }
}
