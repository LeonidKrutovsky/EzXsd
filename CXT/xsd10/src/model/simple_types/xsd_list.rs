use crate::model::Parse;

#[derive(Debug)]
pub struct NotEmptyXsdList<I>(pub Vec<I>);

impl<I> Parse for NotEmptyXsdList<I>
where
    I: Parse,
{
    fn parse(value: &str) -> Result<Self, String> {
        let res = value
                .split_whitespace()
                .map(|v| I::parse(v))
                .collect::<Result<Vec<_>, _>>()?;

        if res.is_empty() {
            Err(format!("There must be at least one Item in the list."))
        } else {
            Ok(Self(res))
        }
    }

    fn create(value: String) -> Self {
        Self {
            0: value
                .split_whitespace()
                .map(|v| I::create(v.to_string()))
                .collect(),
        }
    }

    fn text(&self) -> Result<String, String> {
        let result = self
            .0
            .iter()
            .map(|x| x.text())
            .collect::<Result<Vec<String>, String>>()?
            .into_iter()
            .fold(String::new(), |a, b| format!("{} {}", a, b));

        if result.is_empty() {
            Err(format!("There must be at least one Item in the list."))
        } else {
            Ok(result)
        }
    }
}

#[derive(Debug, Default)]
pub struct XsdList<I>(pub Vec<I>);

impl<I> Parse for XsdList<I>
where
    I: Parse,
{
    fn parse(value: &str) -> Result<Self, String> {
        Ok(Self {
            0: value
                .split_whitespace()
                .map(|v| I::parse(v))
                .collect::<Result<_, _>>()?,
        })
    }

    fn create(value: String) -> Self {
        Self {
            0: value
                .split_whitespace()
                .map(|v| I::create(v.to_string()))
                .collect(),
        }
    }

    fn text(&self) -> Result<String, String> {
        let result = self
            .0
            .iter()
            .map(|x| x.text())
            .collect::<Result<Vec<String>, String>>()?
            .into_iter()
            .fold(String::new(), |a, b| format!("{} {}", a, b));
            Ok(result)
    }
}


