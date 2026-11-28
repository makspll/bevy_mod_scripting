use proc_macro2::TokenStream;
use quote::quote;

/// Stub: the overlay expansion is not implemented yet.
pub fn unsafe_c_overlay(_input: TokenStream) -> TokenStream {
    quote! { ::core::compile_error!("unsafe_c_overlay! is not implemented yet"); }
}
