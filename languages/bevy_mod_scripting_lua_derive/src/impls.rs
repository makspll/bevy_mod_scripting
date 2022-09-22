use bevy_mod_scripting_common::{
    derive_data::{EnumData, ProxyFlag, StructData},
    newtype,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn impl_struct(struct_data: StructData) -> TokenStream {
    let proxy_name = format_ident!("Lua{}", struct_data.meta.base_type_name);
    let base_type_name = struct_data.meta.base_type_name;

    // make proxy definition and basic impls
    let mut definition;

    if struct_data.meta.proxy_flags.contains(&ProxyFlag::Clone) {
        definition = quote! {
            bevy_script_api::make_script_wrapper!(#base_type_name as #proxy_name with Clone);
        }
    } else {
        definition = quote! {
            bevy_script_api::make_script_wrapper!(#base_type_name as #proxy_name);
        }
    };

    if struct_data.meta.proxy_flags.contains(&ProxyFlag::Clone) {
        definition = quote! {
            #definition
            impl std::fmt::Debug for #proxy_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))                    }
            }
        }
    }

    definition
}

pub fn impl_enum(enum_data: EnumData) -> TokenStream {
    todo!()
}
