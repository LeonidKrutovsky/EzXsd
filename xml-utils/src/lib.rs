extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn attribute(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    println!("{:?}", _metadata);
    println!("{:?}", input);
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
