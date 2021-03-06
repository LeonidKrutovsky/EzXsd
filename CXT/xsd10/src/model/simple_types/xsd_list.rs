use std::str::FromStr;

#[derive(Debug)]
pub struct NotEmptyXsdList<I>(pub Vec<I>);

impl<I: FromStr> FromStr for NotEmptyXsdList<I> {
    type Err = I::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = XsdList::from_str(s)?.0;

        Ok(Self(res))

    }
}

// impl<I: Into<String>, T: Into<String>> From<T> for NotEmptyXsdList<I> {
//     fn from(s: T) -> Self {
//         Self (
//             s.into()
//             .split_whitespace()
//             .map(|v| v.into())
//             .collect(),
//         )
//     }
// }

impl<I: AsRef<str>> NotEmptyXsdList<I> {
    pub fn text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.as_ref())
            .fold(String::new(), |a, b| format!("{} {}", a, b))

    }
}

#[derive(Debug, Default, PartialEq)]
pub struct XsdList<I>(pub Vec<I>);

impl<I: FromStr> FromStr for XsdList<I> {
    type Err = I::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .split_whitespace()
            .map(|v| I::from_str(v))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(res))
    }
}

// impl<I: Into<String>, T: Into<String>> From<T> for XsdList<I> {
//     fn from(s: T) -> Self {
//         Self (
//             s.into()
//             .split_whitespace()
//             .map(|v| v.into())
//             .collect(),
//         )
//     }
// }

impl<I: AsRef<str>> XsdList<I> {
    pub fn text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.as_ref())
            .fold(String::new(), |a, b| format!("{} {}", a, b))

    }
}
