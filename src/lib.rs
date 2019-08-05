extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn ensures(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
