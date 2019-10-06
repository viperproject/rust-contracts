extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Item, Expr, Stmt, parse_macro_input};
use quote::quote_spanned;
use quote::quote;
use syn::spanned::Spanned;
use rust_contracts_parser::parse_assertion;

fn add_print_statement(kind: &str, spec: TokenStream, item: TokenStream) -> TokenStream {
    let spec = match parse_assertion(spec.into()) {
        Ok(assertion) => assertion,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };
    let spec_string = format!("{}", spec);
    let mut item = parse_macro_input!(item as Item);

    if let Item::Fn(ref mut fn_item) = item {
        // TODO: Switch to quote_spanned.
        let stmt_tokens: TokenStream = quote!{
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
