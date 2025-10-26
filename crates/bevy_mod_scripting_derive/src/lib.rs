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
/// - `bms_bindings_path`: If set the path to override bms imports, normally only used internally
/// - `unregistered`: If set, will use `new_unregistered` instead of `new` for the namespace builder
/// - `core`: If set, marks the type as `core` using the `MarkAsCore` type data
/// - `significant`: If set, marks the type as `significant` using the `MarkAsSignificant` type data
/// - `use_dummy_registry`: If true will register into the [`bevy_mod_scripting_bindings::function::DummyScriptFunctionRegistry`] instead of the full one. This is useful for documenting functions without actually making them available, if you're exposing them another way.
///
/// It is encouraged to place `significant` markers on your own types, for the purposes of documentation generation.
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

/// Derive macro for generating `Debug` implementations which include type information.
/// Uses the `DebugWithTypeInfoBuilder` trait from `bevy_mod_scripting_display` to
/// generate the debug output.
///
/// Works the same way as the standard `Debug` derive macro, but delegates to
/// `WithTypeInfo` adapters for fields, entries and keys/values.
#[proc_macro_derive(DebugWithTypeInfo, attributes(debug_with_type_info))]
pub fn debug_with_type_info_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::debug_with_type_info(input)
}
