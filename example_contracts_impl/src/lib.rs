extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Item, Expr, Stmt, parse_macro_input};
use quote::quote_spanned;
use syn::spanned::Spanned;

fn add_print_statement(kind: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_string = attr.to_string();
    let attr = parse_macro_input!(attr as Expr);
    let mut item = parse_macro_input!(item as Item);

    if let Item::Fn(ref mut fn_item) = item {
        let stmt_tokens: TokenStream = quote_spanned!{attr.span()=>
            println!("[rust_contracts] {}: {}", #kind, #attr_string);
        }.into();
        let stmt: Stmt = parse_macro_input!(stmt_tokens as Stmt);
        fn_item.block.stmts.insert(0, stmt);
    }

    item.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    add_print_statement("requires", attr, item)
}

#[proc_macro_attribute]
pub fn ensures(attr: TokenStream, item: TokenStream) -> TokenStream {
    add_print_statement("ensures", attr, item)
}
