use std::fmt;
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

impl<I: fmt::Display> fmt::Display for XsdList<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 1 {
            write!(f, "{}", self.0[0])
        } else {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .fold(String::new(), |a, b| {
                        if a.is_empty(){
                            format!("{}", b)
                        } else {
                            format!("{} {}", a, b)
                        }
                    })
            )
        }
    }
}

impl<I: AsRef<str>> XsdList<I> {
    pub fn text(&self) -> String {
        self.0
            .iter()
            .map(|x| x.as_ref())
            .fold(String::new(), |a, b| format!("{} {}", a, b))
    }
}
