//! WIT generation backend for LAD files.  
//!  
//! Converts a LAD (Language Agnostic Definition) file to a WIT (WebAssembly Interface Types)  
//! file that can be used to generate bindings for guest WebAssembly components.  

use ladfile::{
    LadFieldOrVariableKind, LadFile, LadFilePlugin, LadFunctionNamespace, LadTypeId,
    ReflectionPrimitiveKind,
};
use std::{
    error::Error,
    fmt::Write,
    path::{Path, PathBuf},
};

/// Generate a WIT file from a LAD file.  
pub fn generate_wit_from_ladfile(ladfile: &LadFile) -> Result<String, anyhow::Error> {
    let mut out = String::new();

    writeln!(out, "package bms:scripting@0.1.0;")?;
    writeln!(out)?;
    // Get unique types by grouping polymorphic types together
    let polymorphic_types = ladfile.polymorphizied_types(true); // exclude primitives  

    // Generate one interface per unique type (not per generic instance)
    for (poly_key, type_instances) in polymorphic_types {
        // Skip if none of the instances have associated functions
        if !type_instances.iter().any(|&type_id| {
            ladfile
                .types
                .get(type_id)
                .map(|t| !t.associated_functions.is_empty())
                .unwrap_or(false)
        }) {
            continue;
        }

        // Use the first instance to get the type info
        if let Some(&first_type_id) = type_instances.iter().next() {
            if let Some(lad_type) = ladfile.types.get(first_type_id) {
                let interface_name = to_wit_interface_name(&lad_type.identifier);

                writeln!(out, "/// Methods for {}", lad_type.identifier)?;
                writeln!(out, "interface {} {{", interface_name)?;
                writeln!(out, "    use {{reflect-reference}};")?;
                writeln!(out)?;

                // Collect all associated functions from all instances
                let mut all_functions = std::collections::HashSet::new();
                for &type_id in &type_instances {
                    if let Some(t) = ladfile.types.get(type_id) {
                        for func_id in &t.associated_functions {
                            all_functions.insert(func_id);
                        }
                    }
                }

                // Generate functions
                for func_id in &all_functions {
                    if let Some(func) = ladfile.functions.get(*func_id) {
                        if let Some(doc) = &func.documentation {
                            for line in doc.lines() {
                                writeln!(out, "    /// {}", line)?;
                            }
                        }
                        let wit_func = lad_function_to_wit(ladfile, func);
                        writeln!(out, "    {}", wit_func)?;
                    }
                }

                writeln!(out, "}}")?;
                writeln!(out)?;
            }
        }
    }
    // Global functions interface
    let global_functions: Vec<_> = ladfile
        .functions
        .values()
        .filter(|f| matches!(f.namespace, LadFunctionNamespace::Global))
        .collect();

    if !global_functions.is_empty() {
        writeln!(out, "/// Global BMS functions")?;
        writeln!(out, "interface globals {{")?;
        writeln!(
            out,
            "    use types.{{reflect-reference, component-registration, entity}};"
        )?;
        writeln!(out)?;

        for func in &global_functions {
            if let Some(doc) = &func.documentation {
                for line in doc.lines() {
                    writeln!(out, "    /// {}", line)?;
                }
            }
            let wit_func = lad_function_to_wit(ladfile, func);
            writeln!(out, "    {}", wit_func)?;
        }

        writeln!(out, "}}")?;
        writeln!(out)?;
    }

    // World definition
    writeln!(
        out,
        "/// The BMS guest world - what a guest component must implement"
    )?;
    writeln!(out, "world bms-guest {{")?;

    if !global_functions.is_empty() {
        writeln!(out, "    import globals;")?;
    }

    for (_, lad_type) in ladfile.types.iter() {
        if lad_type.associated_functions.is_empty() {
            continue;
        }
        let interface_name = to_wit_interface_name(&lad_type.identifier);
        writeln!(out, "    import {};", interface_name)?;
    }

    writeln!(out)?;
    writeln!(out, "    export on-script-loaded: func();")?;
    writeln!(out, "    export on-script-unloaded: func();")?;
    writeln!(out, "    export on-update: func();")?;
    writeln!(out, "}}")?;

    Ok(out)
}
fn lad_function_to_wit(ladfile: &LadFile, func: &ladfile::LadFunction) -> String {
    let name = to_wit_ident(&func.identifier);

    // Filter out FunctionCallContext args (host-injected)
    let args: Vec<_> = func
        .arguments
        .iter()
        .filter(|a| {
            !matches!(
                a.kind,
                LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::FunctionCallContext)
            )
        })
        .collect();

    let params: Vec<String> = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            // Use to_wit_ident for parameter names to escape reserved keywords
            let param_name = arg
                .name
                .as_ref()
                .map(|n| to_wit_ident(n))
                .unwrap_or_else(|| format!("p{}", i));
            let wit_type = lad_kind_to_wit_type(ladfile, &arg.kind);
            format!("{}: {}", param_name, wit_type)
        })
        .collect();

    let ret = lad_kind_to_wit_type(ladfile, &func.return_type.kind);

    if ret == "()" || ret == "unit" {
        format!("{}: func({});", name, params.join(", "))
    } else {
        format!("{}: func({}) -> {};", name, params.join(", "), ret)
    }
}
fn lad_kind_to_wit_type(ladfile: &LadFile, kind: &LadFieldOrVariableKind) -> String {
    match kind {
        LadFieldOrVariableKind::Primitive(prim) => primitive_to_wit(prim),
        LadFieldOrVariableKind::Ref(id)
        | LadFieldOrVariableKind::Mut(id)
        | LadFieldOrVariableKind::Val(id) => {
            // Check if it's a primitive
            if let Some(prim) = ladfile.primitive_kind(id) {
                return primitive_to_wit(prim);
            }
            // Otherwise use the type identifier
            let ident = ladfile.get_type_identifier(id, Some("reflect-reference"));
            if ident == "ReflectReference" {
                "borrow<reflect-reference>".to_string()
            } else {
                to_wit_ident(&ident)
            }
        }
        LadFieldOrVariableKind::Option(inner) => {
            format!("option<{}>", lad_kind_to_wit_type(ladfile, inner))
        }
        LadFieldOrVariableKind::Vec(inner) => {
            format!("list<{}>", lad_kind_to_wit_type(ladfile, inner))
        }
        LadFieldOrVariableKind::HashMap(k, v) => {
            // WIT doesn't have maps natively; use list of tuples
            format!(
                "list<tuple<{}, {}>>",
                lad_kind_to_wit_type(ladfile, k),
                lad_kind_to_wit_type(ladfile, v)
            )
        }
        LadFieldOrVariableKind::InteropResult(inner) => {
            let inner_type = lad_kind_to_wit_type(ladfile, inner);
            // If inner type is unit, omit it in result
            if inner_type == "()" {
                format!("result<_, string>")
            } else {
                format!("result<{}, string>", inner_type)
            }
        }
        LadFieldOrVariableKind::Tuple(items) => {
            if items.is_empty() {
                "()".to_string()
            } else {
                let types: Vec<_> = items
                    .iter()
                    .map(|i| lad_kind_to_wit_type(ladfile, i))
                    .collect();
                format!("tuple<{}>", types.join(", "))
            }
        }
        LadFieldOrVariableKind::Array(inner, _) => {
            format!("list<{}>", lad_kind_to_wit_type(ladfile, inner))
        }
        LadFieldOrVariableKind::Union(items) => {
            // WIT doesn't have union types; use the first type or string
            if items.is_empty() {
                "string".to_string()
            } else {
                lad_kind_to_wit_type(ladfile, &items[0])
            }
        }
        LadFieldOrVariableKind::Unknown(_) => "string".to_string(),
        _ => "string".to_string(),
    }
}

fn primitive_to_wit(prim: &ReflectionPrimitiveKind) -> String {
    match prim {
        ReflectionPrimitiveKind::Bool => "bool".to_string(),
        ReflectionPrimitiveKind::I8 => "s8".to_string(),
        ReflectionPrimitiveKind::I16 => "s16".to_string(),
        ReflectionPrimitiveKind::I32 => "s32".to_string(),
        ReflectionPrimitiveKind::I64 | ReflectionPrimitiveKind::Isize => "s64".to_string(),
        ReflectionPrimitiveKind::I128 => "s64".to_string(), // WIT doesn't have s128
        ReflectionPrimitiveKind::U8 => "u8".to_string(),
        ReflectionPrimitiveKind::U16 => "u16".to_string(),
        ReflectionPrimitiveKind::U32 => "u32".to_string(),
        ReflectionPrimitiveKind::U64 | ReflectionPrimitiveKind::Usize => "u64".to_string(),
        ReflectionPrimitiveKind::U128 => "u64".to_string(), // WIT doesn't have u128
        ReflectionPrimitiveKind::F32 => "f32".to_string(),
        ReflectionPrimitiveKind::F64 => "f64".to_string(),
        ReflectionPrimitiveKind::Char => "char".to_string(),
        ReflectionPrimitiveKind::Str
        | ReflectionPrimitiveKind::String
        | ReflectionPrimitiveKind::OsString
        | ReflectionPrimitiveKind::PathBuf => "string".to_string(),
        ReflectionPrimitiveKind::FunctionCallContext => "()".to_string(), // filtered out
        ReflectionPrimitiveKind::DynamicFunction | ReflectionPrimitiveKind::DynamicFunctionMut => {
            "string".to_string()
        }
        ReflectionPrimitiveKind::ReflectReference => "borrow<reflect-reference>".to_string(),
        ReflectionPrimitiveKind::ScriptValue => "string".to_string(), // dynamic, use string
        ReflectionPrimitiveKind::External(_) => "string".to_string(),
    }
}
/// List of WIT reserved keywords that need to be escaped  
const WIT_RESERVED_KEYWORDS: &[&str] = &[
    // Primitive types
    "bool",
    "s8",
    "s16",
    "s32",
    "s64",
    "u8",
    "u16",
    "u32",
    "u64",
    "f32",
    "f64",
    "char",
    "string",
    // WIT keywords
    "func",
    "interface",
    "world",
    "type",
    "resource",
    "record",
    "variant",
    "enum",
    "flags",
    "tuple",
    "list",
    "option",
    "result",
    "borrow",
    "own",
    "use",
    "export",
    "import",
    "package",
    "as",
    // Control flow
    "if",
    "else",
    "match",
    "for",
    "while",
    "loop",
    "return",
    // Field/parameter keywords
    "from",
    "to",
    // Other
    "true",
    "false",
    "null",
    "undefined",
    "with",
];

/// Convert a Rust identifier to a WIT-compatible identifier, escaping reserved keywords  
fn to_wit_ident(name: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;

    for (i, c) in name.chars().enumerate() {
        if c == '_' || c == ':' || c == ' ' {
            if !result.is_empty() && !result.ends_with('-') {
                result.push('-');
            }
            prev_upper = false;
        } else if c.is_uppercase() {
            if i > 0 && !prev_upper && !result.is_empty() && !result.ends_with('-') {
                result.push('-');
            }
            #[allow(clippy::unwrap_used)]
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }

    result = result.trim_end_matches('-').to_string();

    // Escape reserved keywords by appending an underscore
    if WIT_RESERVED_KEYWORDS.contains(&result.as_str()) {
        // TODO: Just make it work for now
        result.push('a');
    }

    result
}

fn to_wit_interface_name(name: &str) -> String {
    to_wit_ident(name)
}
/// A plugin which generates WIT interface definition files from LAD files  
#[derive(Clone)]
pub struct WITLadBackendPlugin {
    /// The filename of the generated WIT definition file  
    pub filename: PathBuf,
}

impl Default for WITLadBackendPlugin {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("bindings.wit"),
        }
    }
}

// /// List of WIT reserved keywords that need to be escaped
// const WIT_RESERVED_KEYWORDS: &[&str] = &[
//     // Primitive types
//     "bool",
//     "s8",
//     "s16",
//     "s32",
//     "s64",
//     "u8",
//     "u16",
//     "u32",
//     "u64",
//     "f32",
//     "f64",
//     "char",
//     "string",
//     // WIT keywords
//     "func",
//     "interface",
//     "world",
//     "type",
//     "resource",
//     "record",
//     "variant",
//     "enum",
//     "flags",
//     "tuple",
//     "list",
//     "option",
//     "result",
//     "borrow",
//     "own",
//     "use",
//     "export",
//     "import",
//     "package",
//     "as",
//     // Control flow
//     "if",
//     "else",
//     "match",
//     "for",
//     "while",
//     "loop",
//     "return",
//     // Other
//     "true",
//     "false",
//     "null",
//     "undefined",
// ];

/// Convert a Rust identifier to a WIT-compatible identifier, escaping reserved keywords  
// fn to_wit_ident(name: &str) -> String {
//     let mut result = String::new();
//     let mut prev_upper = false;

//     for (i, c) in name.chars().enumerate() {
//         if c == '_' || c == ':' || c == ' ' {
//             if !result.is_empty() && !result.ends_with('-') {
//                 result.push('-');
//             }
//             prev_upper = false;
//         } else if c.is_uppercase() {
//             if i > 0 && !prev_upper && !result.is_empty() && !result.ends_with('-') {
//                 result.push('-');
//             }
//             result.push(c.to_lowercase().next().unwrap());
//             prev_upper = true;
//         } else {
//             result.push(c);
//             prev_upper = false;
//         }
//     }

//     result = result.trim_end_matches('-').to_string();

//     // Escape reserved keywords by appending an underscore
//     if WIT_RESERVED_KEYWORDS.contains(&result.as_str()) {
//         result.push('_');
//     }

//     result
// }
impl LadFilePlugin for WITLadBackendPlugin {
    fn run(&self, ladfile: &ladfile::LadFile, path: &Path) -> Result<(), Box<dyn Error>> {
        let wit_content = generate_wit_from_ladfile(ladfile)
            .map_err(|e| e.into_boxed_dyn_error() as Box<dyn Error>)?;

        let output_path = path.join(&self.filename);
        std::fs::write(&output_path, wit_content)?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "WIT interface definition file generator"
    }
}
