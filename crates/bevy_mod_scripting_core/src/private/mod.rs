//! This module contains private functions and modules that are used internally by the crate.

macro_rules! export_all_in_modules {
    ( $( $module:ident ),* $(,)? ) => {
        $(
            pub mod $module;
        )*
        pub use {
            $( $module::*, )*
        };
    };
}


pub(crate) use export_all_in_modules;
