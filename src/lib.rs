extern crate proc_macro;
#[cfg(use_plugin)]
#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;
#[cfg(use_plugin)]
use rust_contracts_common::{get_plugin, Plugin};
#[cfg(use_plugin)]
use proc_macro2::TokenStream as TokenStream2;

#[cfg(use_plugin)]
lazy_static! {
    static ref PLUGIN: Box<Plugin + Sync> = get_plugin!();
}

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(use_plugin)]
    { PLUGIN.requires(TokenStream2::from(_attr), TokenStream2::from(item)).into() }
    #[cfg(not(use_plugin))]
    { item }
}

#[proc_macro_attribute]
pub fn ensures(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(use_plugin)]
    { PLUGIN.ensures(TokenStream2::from(_attr), TokenStream2::from(item)).into() }
    #[cfg(not(use_plugin))]
    { item }
}
