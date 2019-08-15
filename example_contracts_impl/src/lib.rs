extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Item, Expr, Stmt, parse_macro_input};
use quote::quote_spanned;
use syn::spanned::Spanned;

fn add_print_statement(kind: &str, spec: TokenStream, item: TokenStream) -> TokenStream {
    let spec_string = spec.to_string();
    let spec = parse_macro_input!(spec as Expr);
    let mut item = parse_macro_input!(item as Item);

    if let Item::Fn(ref mut fn_item) = item {
        let stmt_tokens: TokenStream = quote_spanned!{spec.span()=>
            println!("[rust_contracts] {}: {}", #kind, #spec_string);
        }.into();
        let stmt: Stmt = parse_macro_input!(stmt_tokens as Stmt);
        fn_item.block.stmts.insert(0, stmt);
    }

    item.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn requires(spec: TokenStream, item: TokenStream) -> TokenStream {
    add_print_statement("requires", spec, item)
}

#[proc_macro_attribute]
pub fn ensures(spec: TokenStream, item: TokenStream) -> TokenStream {
    add_print_statement("ensures", spec, item)
}

#[proc_macro]
pub fn invariant(spec: TokenStream) -> TokenStream {
    let spec_string = spec.to_string();
    let spec = parse_macro_input!(spec as Expr);
    let stmt_tokens: TokenStream = quote_spanned!{spec.span()=>
        println!("[rust_contracts] invariant: {}", #spec_string);
    }.into();
    stmt_tokens
}
