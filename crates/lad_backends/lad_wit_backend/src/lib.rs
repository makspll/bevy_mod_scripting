//! WIT generation backend for LAD files.  
//!  
//! Converts a LAD (Language Agnostic Definition) file to a WIT (WebAssembly Interface Types)  
//! file that can be used to generate bindings for guest WebAssembly components.  

use bevy_mod_scripting_wasmtime::{
    to_canonical_abi_func_name, to_canonical_abi_func_name_without_reg, to_wit_ident,
};
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
    writeln!(out, "interface types {{")?;

    // Get unique types by grouping polymorphic types together
    let polymorphic_types = ladfile.polymorphizied_types(true); // exclude primitives  

    // Generate one resource per unique type (not per generic instance)
    for (poly_key, type_instances) in polymorphic_types {
        // exclude generic types
        if poly_key.arity != 0 || type_instances.len() != 1 {
            continue;
        }

        let instance = match type_instances
            .iter()
            .next()
            .and_then(|instance| ladfile.types.get(*instance))
        {
            Some(instance) => instance,
            None => continue,
        };
        let wit_type_ident = to_wit_ident(&instance.identifier);
        writeln!(out, "resource {wit_type_ident} {{")?;

        for assoc_fn in &instance.associated_functions {
            if let Some(lad_fn) = ladfile.functions.get(assoc_fn) {
                lad_to_wit_function(ladfile, &mut out, lad_fn)?;
            }
        }
        writeln!(out, "}}")?;
    }
    writeln!(out, "}}")?;

    Ok(out)
}

fn lad_to_wit_function(
    ladfile: &LadFile,
    out: &mut String,
    lad_fn: &ladfile::LadFunction,
) -> Result<(), anyhow::Error> {
    let ident = lad_fn.identifier_with_overload();
    let wit_ident = to_wit_ident(&ident);
    let is_method = lad_fn.is_method();
    let static_ = if is_method { "" } else { "static" };
    let params: Vec<_> = lad_fn
        .arguments
        .iter()
        .filter(|a| {
            !matches!(
                a.kind,
                LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::FunctionCallContext)
            )
        })
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
    let ret = lad_kind_to_wit_type(ladfile, &lad_fn.return_type.kind);
    Ok(if ret == "()" || ret == "unit" {
        writeln!(out, "{}: func({});", wit_ident, params.join(", "))?;
    } else {
        writeln!(
            out,
            "{}: func({}) -> {};",
            wit_ident,
            params.join(", "),
            ret
        )?;
    })
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
            // if ident == "ReflectReference" {
            //     "borrow<reflect-reference>".to_string()
            // } else {
            to_wit_ident(&ident)
            // }
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
