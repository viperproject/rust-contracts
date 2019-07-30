#[macro_use]
extern crate rust_contracts_common;

use rust_contracts_common::Plugin;
use quote::ToTokens;

use syn::{Item, Expr, Stmt};
use quote::quote_spanned;
use syn::spanned::Spanned;
use proc_macro2::TokenStream;

struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        ExamplePlugin
    }
}

macro_rules! parse_macro_input2 {
    ($tokenstream:ident as $ty:ty) => {
        match syn::parse2::<$ty>($tokenstream) {
            syn::export::Ok(data) => data,
            syn::export::Err(err) => {
                return TokenStream::from(err.to_compile_error());
            }
        };
    };
}

fn add_print_statement(kind: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_string = attr.to_string();
    let attr = parse_macro_input2!(attr as Expr);
    let mut item = parse_macro_input2!(item as Item);

    if let Item::Fn(ref mut fn_item) = item {
        let stmt_tokens: TokenStream = quote_spanned!{attr.span()=>
            println!("[rust_contracts] {}: {}", #kind, #attr_string);
        };
        let stmt: Stmt = parse_macro_input2!(stmt_tokens as Stmt);
        fn_item.block.stmts.insert(0, stmt);
    }

    item.into_token_stream()
}

impl Plugin for ExamplePlugin {
    fn requires(&self, attr: TokenStream, item: TokenStream) -> TokenStream {
        add_print_statement("requires", attr, item)
    }

    fn ensures(&self, attr: TokenStream, item: TokenStream) -> TokenStream {
        add_print_statement("ensures", attr, item)
    }
}

declare_plugin!(ExamplePlugin, ExamplePlugin::new);
