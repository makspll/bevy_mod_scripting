//! Derive macros for BMS

mod derive;

#[proc_macro_derive(TypedThrough)]
/// Default implementation for the `TypedThrough` trait
pub fn typed_through(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::typed_through(input.into()).into()
}

#[proc_macro_derive(IntoScript)]
/// Default implementation for the `IntoScript` trait
pub fn into_script(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::into_script(input.into()).into()
}

/// Derive macro for generating script bindings from an impl block.
///
/// Generates a registration function with visibility determined by the highest visibility in the impl block.
///
/// Does not support generics.
///
/// Arguments:
/// - `name`: the name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn. Defaults to `functions`
/// - `remote`: If true the original impl block will be ignored, and only the function registrations will be generated
/// - `bms_core_path`: If set the path to override bms imports, normally only used internally
/// - `unregistered`: If set, will use `new_unregistered` instead of `new` for the namespace builder
#[proc_macro_attribute]
pub fn script_bindings(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive::script_bindings(args, input)
}

/// Derive macro for generating script globals from an impl block.
///
/// Generates a registration function with visibility determined by the highest visibility in the impl block.
///
/// Does not support generics.
///
/// Arguments:
/// - `name`: the name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn. Defaults to `globals`
/// - `bms_core_path`: If set the path to override bms imports, normally only used internally
#[proc_macro_attribute]
pub fn script_globals(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive::script_globals(args, input)
}

/// Derive macro for generating `GetTypeDependencies` implementations.
#[proc_macro_derive(GetTypeDependencies, attributes(get_type_dependencies))]
pub fn get_type_dependencies(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::get_type_dependencies(input.into()).into()
}

/// Proc macro equivalent of `GetTypeDependencies` which does not generate a type, purely the impl.
/// Useful for generating implementations against remote types.
#[proc_macro]
pub fn impl_get_type_dependencies(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::get_type_dependencies(input.into()).into()
}
