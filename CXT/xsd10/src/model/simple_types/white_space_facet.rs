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
            .map(|x| if x.is_whitespace() {' '} else {x})
            .collect()
}


//After the processing implied by replace, contiguous sequences
// of #x20's are collapsed to a single #x20, and leading and trailing
// #x20's are removed.
pub fn collapse(value: &str) -> String {
     let re = Regex::new(" {2,}").unwrap();
     re.replace_all(replace(value).trim(), " ").into()
}