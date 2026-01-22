use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn str_to_char(input: TokenStream) -> TokenStream {
    let str_input = parse_macro_input!(input as LitStr);
    let str = str_input.value();

    let chars = str.chars().collect::<Box<[char]>>();

    let chars_litterals = 
        chars.into_iter().map(|c| Literal::character(c)).collect::<Box<[_]>>();

    let output = quote! {
        [#(#chars_litterals),*]
    };

    TokenStream::from(output)
}