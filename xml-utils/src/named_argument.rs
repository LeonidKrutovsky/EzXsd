use syn::{LitStr, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};


#[derive(Debug)]
pub struct NamedArgument {
    pub name: Ident,
    pub eq_token: Token![=],
    pub value: LitStr
}

impl Parse for NamedArgument {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}