//! Parses tokens into untyped AST.

use crate::ast::untyped as ast;
use proc_macro2::TokenStream;
use std::convert::TryInto;
use syn::Result;

mod expr_parser;
mod preparser;

pub fn parse_assertion(tokens: TokenStream) -> Result<ast::Assertion> {
    let assertion = preparser::preparse_assertion(tokens)?;
    assertion.try_into()
}
