use proc_macro2::TokenStream;

pub trait Plugin {
    fn requires(&self, _attr: TokenStream, _item: TokenStream) -> TokenStream;
    fn ensures(&self, _attr: TokenStream, _item: TokenStream) -> TokenStream;
}

/// Declare a plugin type and its constructor.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _rust_contracts_plugin_constructor() -> *mut ($crate::Plugin + Sync) {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;
            let plugin = constructor();
            let boxed: Box<$crate::Plugin + Sync> = Box::new(plugin);
            Box::into_raw(boxed)
        }
    };
}

#[macro_export]
macro_rules! get_plugin {
    () => {
        {
            extern "C" {
                pub fn _rust_contracts_plugin_constructor() -> *mut ($crate::Plugin + Sync);
            }
            let boxed_raw = unsafe { _rust_contracts_plugin_constructor() };
            let plugin: Box<Plugin + Sync> = unsafe { Box::from_raw(boxed_raw) };
            plugin
        }
    };
}
