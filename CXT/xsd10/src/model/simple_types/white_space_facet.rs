use regex::Regex;

//No normalization is done, the value is not changed
// (this is the behavior required by [XML 1.0 (Second Edition)]
// for element content)
// fn preserve(value: &str) - empty func

//All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return)
// are replaced with #x20 (space)
pub fn replace(value: &str) -> String {
    value
        .chars()
        .map(|x| if x.is_whitespace() { ' ' } else { x })
        .collect()
}

pub fn is_replaced<T: AsRef<str>>(value: T) -> bool {
    value.as_ref().chars().any(|c| c.is_whitespace() && c != ' ')
}

pub fn assert_replaced<T: AsRef<str>>(value: T, name: T) -> Result<(), String> {
    if is_replaced(value.as_ref()) {
        Ok(())
    } else {
        Err(format!("Invalid value for {}. White space must be replaced: {}", name.as_ref(), value.as_ref()))
    }
}

//After the processing implied by replace, contiguous sequences
// of #x20's are collapsed to a single #x20, and leading and trailing
// #x20's are removed.
pub fn collapse(value: &str) -> String {
    let re = Regex::new(" {2,}").unwrap();
    re.replace_all(replace(value.trim()).as_str(), " ").into()
}

pub fn is_collapsed<T: AsRef<str>>(value: T) -> bool {
    let value = value.as_ref();
    !is_replaced(value) || value.starts_with(' ') || value.ends_with(' ') || value.contains("  ")
}

pub fn assert_collapsed<T: AsRef<str>>(value: T, name: T) -> Result<(), String> {
    if is_collapsed(value.as_ref()) {
        Ok(())
    } else {
        Err(format!("Invalid value for {}. White spaces must be collapsed: {}", name.as_ref(), value.as_ref()))
    }
}
