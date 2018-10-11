#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate doubter_impl;
#[doc(hidden)]
pub use doubter_impl::*;

proc_macro_item_decl! {
    /// Insert a code for testing doctest.
    doubter! => doubter_impl
}
