use bevy_mod_scripting_derive::DebugWithTypeInfo;

/// A primitive type kind in the LAD file format.
///
/// The docstrings on variants corresponding to Reflect types, are used to generate documentation for these primitives.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum ReflectionPrimitiveKind {
    Bool,
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Char,
    Str,
    String,
    OsString,
    PathBuf,
    FunctionCallContext,
    DynamicFunction,
    ScriptValue,
    DynamicFunctionMut,
    ReflectReference,
    /// A primitive defined outside of BMS, useful for custom implementations of FromScript and IntoScript.
    /// Downstream processors like mdbook plugins won't know how to treat these.
    External(String),
}

impl std::fmt::Display for ReflectionPrimitiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectionPrimitiveKind::Bool => f.write_str("Bool"),
            ReflectionPrimitiveKind::Isize => f.write_str("Isize"),
            ReflectionPrimitiveKind::I8 => f.write_str("I8"),
            ReflectionPrimitiveKind::I16 => f.write_str("I16"),
            ReflectionPrimitiveKind::I32 => f.write_str("I32"),
            ReflectionPrimitiveKind::I64 => f.write_str("I64"),
            ReflectionPrimitiveKind::I128 => f.write_str("I128"),
            ReflectionPrimitiveKind::Usize => f.write_str("Usize"),
            ReflectionPrimitiveKind::U8 => f.write_str("U8"),
            ReflectionPrimitiveKind::U16 => f.write_str("U16"),
            ReflectionPrimitiveKind::U32 => f.write_str("U32"),
            ReflectionPrimitiveKind::U64 => f.write_str("U64"),
            ReflectionPrimitiveKind::U128 => f.write_str("U128"),
            ReflectionPrimitiveKind::F32 => f.write_str("F32"),
            ReflectionPrimitiveKind::F64 => f.write_str("F64"),
            ReflectionPrimitiveKind::Char => f.write_str("Char"),
            ReflectionPrimitiveKind::Str => f.write_str("Str"),
            ReflectionPrimitiveKind::String => f.write_str("String"),
            ReflectionPrimitiveKind::OsString => f.write_str("OsString"),
            ReflectionPrimitiveKind::PathBuf => f.write_str("PathBuf"),
            ReflectionPrimitiveKind::FunctionCallContext => f.write_str("FunctionCallContext"),
            ReflectionPrimitiveKind::DynamicFunction => f.write_str("DynamicFunction"),
            ReflectionPrimitiveKind::ScriptValue => f.write_str("ScriptValue"),
            ReflectionPrimitiveKind::DynamicFunctionMut => f.write_str("DynamicFunctionMut"),
            ReflectionPrimitiveKind::ReflectReference => f.write_str("ReflectReference"),
            ReflectionPrimitiveKind::External(e) => f.write_str(e.as_str()),
        }
    }
}
