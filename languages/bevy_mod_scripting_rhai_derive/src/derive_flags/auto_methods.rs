use std::iter::once;

use bevy_mod_scripting_common::{
    arg::SimpleType, derive_flag::DeriveFlag, newtype::Newtype, utils::EmptyToken,
};
use proc_macro2::Span;
use quote::{format_ident, quote_spanned};
use syn::{parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, LitInt, Token};

use crate::rhai_method::RhaiMethod;
