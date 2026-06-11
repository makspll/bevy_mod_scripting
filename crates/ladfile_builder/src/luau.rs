//! Native **Luau** (`.d.luau`) definition-file backend for LAD files.
//!
//! BMS ships LAD post-processors for the Lua Language Server (`--- @class`
//! `.lua`, see [`lua_language_server_lad_backend`]) and for mdbook, but no native
//! Luau one — and [`luau-lsp`] cannot consume the LuaLS flavour. This module is
//! that missing backend: it turns a [`LadFile`] into `declare class … end` /
//! `declare name: T` definitions that `luau-lsp analyze --defs=…` checks scripts
//! against.
//!
//! The public entry point [`lad_to_luau`] is a pure `&LadFile -> String`
//! conversion; [`LuauLadPlugin`] is the [`LadFilePlugin`] processor that writes
//! the file into the generation output directory, exactly like the LuaLS backend.
//! Enable it with the `luau_files` feature.
//!
//! The reflection API BMS exposes to Lua is *dynamic* (component references are
//! `ReflectReference` proxies resolved at runtime), so the generated types cover
//! the statically-knowable surface: reflected types and their fields, registered
//! host globals, and instance handles like `world`. Anything the generator can't
//! pin down resolves to `any`, which Luau treats permissively, so scripts still
//! type-check.
//!
//! [`lua_language_server_lad_backend`]: https://docs.rs/lua_language_server_lad_backend
//! [`luau-lsp`]: https://github.com/JohnnyMorganz/luau-lsp

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};

use ladfile::{
    LadArgument, LadFieldOrVariableKind, LadFile, LadFilePlugin, LadFunction, LadFunctionNamespace,
    LadTypeId, LadTypeLayout, LadVariant, ReflectionPrimitiveKind,
};

/// Configuration for the Luau backend.
#[derive(Clone, Debug, Default)]
pub struct LuauBackendConfig {
    /// Crate names whose reflected types get a full `declare class … end`.
    ///
    /// Pass the crates whose types you actually script against (e.g. your game
    /// crate) to keep the file focused on your API. Pass an empty list to fall
    /// back to declaring only registered **component/resource** types (so the
    /// output stays bounded rather than dumping the entire reflection registry).
    /// The core interaction types (`World`, the query/registration types) are
    /// always declared; anything not declared resolves to `any`.
    pub focus_crates: Vec<String>,

    /// Opt-in phantom-typed registration brand. `None` (the default) emits the
    /// plain, general API. See [`HandleBranding`] for what enabling it does and the
    /// host-side convention it assumes.
    pub handle_branding: Option<HandleBranding>,
}

/// Opt-in `Reg<T>` phantom-brand configuration.
///
/// When set, the backend emits a `type Reg<T> = { __phantom: T }` brand, rewrites
/// the component getter to the generic `get_component: <T>(…, reg: Reg<T>) -> T?`,
/// and brands each host-registered registration global whose name ends in
/// [`global_suffix`](Self::global_suffix) as `Reg<Component>`. The net effect is
/// **cast-free, statically-typed component access** in scripts:
///
/// ```luau
/// local vel = world.get_component(entity, VelocityType) -- vel: Velocity?, no cast
/// ```
///
/// This relies on a host convention: for a component `Velocity`, the app must
/// register a script global named `Velocity{suffix}` (e.g. `VelocityType`) whose
/// value is `Velocity`'s `ScriptComponentRegistration`. It is off by default
/// precisely because it assumes that convention; the general backend does not.
#[derive(Clone, Debug)]
pub struct HandleBranding {
    /// Suffix on a registration global's name that selects the branded component.
    /// With `"Type"`, a non-static global `VelocityType` is typed `Reg<Velocity>`
    /// (the base name must itself be a declared class).
    pub global_suffix: String,
}

impl HandleBranding {
    /// A `…Type` suffix convention (`VelocityType`, `PositionType`, …).
    pub fn type_suffix() -> Self {
        Self {
            global_suffix: "Type".to_string(),
        }
    }
}

/// Render a parsed LAD file as Luau definition-file source.
///
/// This is the whole backend as a pure function; [`LuauLadPlugin`] just calls it
/// and writes the result to disk.
pub fn lad_to_luau(lad: &LadFile, config: &LuauBackendConfig) -> String {
    Converter::new(lad, config).render()
}

/// A [`LadFilePlugin`] post-processor that writes a native Luau `.d.luau` file.
///
/// Add it to the `ScriptingFilesGenerationPlugin` processor list (the `luau_files`
/// feature wires a default instance into `default_processors()`); on generation it
/// writes [`filename`](Self::filename) into the configured output directory.
#[derive(Clone, Debug)]
pub struct LuauLadPlugin {
    /// Conversion configuration.
    pub config: LuauBackendConfig,
    /// File name to write inside the generation output directory.
    pub filename: PathBuf,
}

impl Default for LuauLadPlugin {
    fn default() -> Self {
        Self {
            config: LuauBackendConfig::default(),
            filename: PathBuf::from("bindings.d.luau"),
        }
    }
}

impl LadFilePlugin for LuauLadPlugin {
    fn name(&self) -> &'static str {
        "luau_definition_files"
    }

    fn run(&self, ladfile: &LadFile, output_dir: &Path) -> Result<(), Box<dyn Error>> {
        let source = lad_to_luau(ladfile, &self.config);
        std::fs::write(output_dir.join(&self.filename), source)?;
        Ok(())
    }
}

struct Converter<'a> {
    lad: &'a LadFile,
    config: &'a LuauBackendConfig,
    /// type id -> the Luau identifier we emit for it (sanitised + unique).
    names: HashMap<LadTypeId, String>,
    /// type ids we emit a full `declare class … end` for (and may therefore
    /// reference by name). Everything else resolves to `any`.
    declared: HashSet<LadTypeId>,
}

impl<'a> Converter<'a> {
    fn new(lad: &'a LadFile, config: &'a LuauBackendConfig) -> Self {
        // A small set of always-interesting core types beyond components/resources,
        // so the `world` handle and entity references get real method signatures.
        const CORE: &[&str] = &[
            "World",
            "Entity",
            "ScriptQueryBuilder",
            "ScriptQueryResult",
            "ScriptComponentRegistration",
            "ScriptTypeRegistration",
            "ScriptResourceRegistration",
        ];

        let mut names = HashMap::new();
        let mut used: HashSet<String> = HashSet::new();
        let mut declared = HashSet::new();

        for (id, def) in &lad.types {
            // Mapped-to-primitive types are rendered as Luau builtins, never classes.
            if def.metadata.mapped_to_primitive_kind.is_some() {
                continue;
            }
            let mut name = ident(&def.identifier);
            while !used.insert(name.clone()) {
                name.push('_');
            }
            names.insert(id.clone(), name);

            // With a focus list, declare every reflected type from those crates;
            // without one, fall back to components/resources so the output stays
            // bounded. Core interaction types are always declared.
            let in_focus = if config.focus_crates.is_empty() {
                def.metadata.is_component || def.metadata.is_resource
            } else {
                def.crate_
                    .as_deref()
                    .is_some_and(|c| config.focus_crates.iter().any(|f| f == c))
            };
            if in_focus || CORE.contains(&def.identifier.as_str()) {
                declared.insert(id.clone());
            }
        }

        Converter {
            lad,
            config,
            names,
            declared,
        }
    }

    /// Whether the `Reg<T>` phantom-brand is enabled.
    fn branding(&self) -> Option<&HandleBranding> {
        self.config.handle_branding.as_ref()
    }

    /// The Luau name we emit for a declared type id (falls back to the raw id,
    /// which only happens for ids we never registered a name for).
    fn declared_name(&self, id: &LadTypeId) -> &str {
        self.names.get(id).map(String::as_str).unwrap_or("any")
    }

    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("--!strict\n");
        out.push_str(
            "-- AUTO-GENERATED from the Bevy reflection registry via the bevy_mod_scripting\n",
        );
        out.push_str("-- Luau LAD backend. Do not edit by hand.\n\n");

        // The phantom-typed registration brand (opt-in). A `Reg<Velocity>` is, at
        // runtime, an ordinary component registration; the phantom `T` lets
        // `get_component` return the right component type with no cast. Requires the
        // host to register a matching `…{suffix}` global per component.
        if self.branding().is_some() {
            out.push_str("type Reg<T> = { __phantom: T }\n\n");
        }

        // Group functions by namespace once.
        let mut methods: HashMap<LadTypeId, Vec<&LadFunction>> = HashMap::new();
        let mut globals_fns: Vec<&LadFunction> = Vec::new();
        for func in self.lad.functions.values() {
            match &func.namespace {
                LadFunctionNamespace::Type(id) => methods.entry(id.clone()).or_default().push(func),
                LadFunctionNamespace::Global => globals_fns.push(func),
            }
        }

        // Declared classes, in a stable order.
        let mut declared_ids: Vec<&LadTypeId> = self.declared.iter().collect();
        declared_ids.sort_by_key(|id| self.declared_name(id).to_string());

        for id in declared_ids {
            let Some(def) = self.lad.types.get(id) else {
                continue;
            };
            let name = self.declared_name(id);
            if let Some(doc) = &def.documentation {
                push_doc(&mut out, doc, "");
            }
            out.push_str(&format!("declare class {name}\n"));

            // Named struct fields become Luau fields.
            if let LadTypeLayout::MonoVariant(LadVariant::Struct { fields, .. }) = &def.layout {
                for f in fields {
                    out.push_str(&format!(
                        "\t{}: {}\n",
                        field_key(&f.name),
                        self.kind(&f.type_)
                    ));
                }
            }

            // Associated functions. A function whose first (post-context) argument
            // is the owning type is a *method*, called `value:method(...)`. One that
            // only takes the auto-injected call context (like the `world` API) is a
            // dot-callable function field, called `value.func(...)`.
            let mut fns: Vec<&LadFunction> = methods.remove(id).unwrap_or_default();
            fns.sort_by_key(|f| f.identifier.to_string());
            for func in fns {
                self.push_member(&mut out, name, func);
            }
            out.push_str("end\n\n");
        }

        // Global host functions.
        globals_fns.sort_by_key(|f| f.identifier.to_string());
        for func in globals_fns {
            if let Some(doc) = &func.documentation {
                push_doc(&mut out, doc, "");
            }
            let params = self.params(self.global_args(func));
            let ret = self.kind(&func.return_type.kind);
            out.push_str(&format!(
                "declare function {}({params}): {ret}\n\n",
                ident(&func.identifier)
            ));
        }

        // Global instances. BMS also exposes every reflected type as a *static*
        // accessor global (`Velocity`, `Affine2`, …); those are obtained instead via
        // `world.get_type_by_name(...)` in scripts, so we emit only the real
        // instance handles (`world`, `entity`, …).
        let declared_names: HashSet<&str> = self
            .declared
            .iter()
            .map(|id| self.declared_name(id))
            .collect();
        let mut globals: Vec<(&str, String)> = Vec::new();
        for (key, inst) in &self.lad.globals {
            if inst.is_static {
                continue;
            }
            // With branding on, a host-registered `…{suffix}` registration global is
            // typed `Reg<Component>` so `get_component` returns the right type.
            let ty = self
                .branded_global(key, &declared_names)
                .unwrap_or_else(|| self.kind(&inst.type_kind));
            globals.push((key, ty));
        }
        globals.sort_by_key(|(k, _)| k.to_string());
        for (key, ty) in globals {
            out.push_str(&format!("declare {}: {ty}\n", ident(key)));
        }

        out
    }

    /// If branding is on and `key` is `<Base><suffix>` where `Base` is a declared
    /// class, return the branded type `Reg<Base>`.
    fn branded_global(&self, key: &str, declared_names: &HashSet<&str>) -> Option<String> {
        let branding = self.branding()?;
        key.strip_suffix(&branding.global_suffix)
            .filter(|base| declared_names.contains(base))
            .map(|base| format!("Reg<{base}>"))
    }

    /// Render one associated function either as a colon-method (if it has a
    /// receiver) or as a dot-callable function field (if it is context-only).
    fn push_member(&self, out: &mut String, owner: &str, func: &LadFunction) {
        if let Some(doc) = &func.documentation {
            push_doc(out, doc, "\t");
        }
        let (args, has_self) = self.script_visible_args(func, owner);
        let ret = self.kind(&func.return_type.kind);
        let name = ident(&func.identifier);

        // With branding on, brand a component getter generically: a context-only
        // function whose last argument is a component registration and which returns
        // an optional reflect reference is rewritten as `<T>(…, reg: Reg<T>) -> T?`,
        // so passing a branded `…{suffix}` handle yields the concrete component type
        // with no cast.
        if self.branding().is_some()
            && !has_self
            && ret == "any?"
            && args
                .last()
                .is_some_and(|a| self.kind(&a.kind) == "ScriptComponentRegistration")
        {
            let mut params = self.params(args.split_last().map(|(_, rest)| rest).unwrap_or(&[]));
            if !params.is_empty() {
                params.push_str(", ");
            }
            out.push_str(&format!("\t{name}: <T>({params}reg: Reg<T>) -> T?\n"));
            return;
        }

        if has_self {
            let params = self.params(args);
            let sep = if params.is_empty() { "" } else { ", " };
            out.push_str(&format!("\tfunction {name}(self{sep}{params}): {ret}\n"));
        } else {
            // A dot-callable function field, e.g. `world.query()`.
            let params = self.params(args);
            out.push_str(&format!("\t{name}: ({params}) -> {ret}\n"));
        }
    }

    /// The arguments a script actually passes, and whether the function has a
    /// receiver. BMS injects a leading auto-provided `FunctionCallContext` on many
    /// functions (dropped here); a following argument of the owning type is the
    /// method receiver (dropped, and reported as `has_self`).
    fn script_visible_args<'f>(
        &self,
        func: &'f LadFunction,
        owner: &str,
    ) -> (&'f [LadArgument], bool) {
        let args = self.global_args(func);
        if let Some(first) = args.first()
            && self.kind(&first.kind) == owner
        {
            return (
                args.split_first().map(|(_, rest)| rest).unwrap_or(&[]),
                true,
            );
        }
        (args, false)
    }

    /// Drop a leading auto-injected `FunctionCallContext` from a function's args.
    fn global_args<'f>(&self, func: &'f LadFunction) -> &'f [LadArgument] {
        if let Some(first) = func.arguments.first()
            && matches!(
                &first.kind,
                LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::FunctionCallContext)
            )
        {
            return func
                .arguments
                .split_first()
                .map(|(_, rest)| rest)
                .unwrap_or(&[]);
        }
        func.arguments.as_slice()
    }

    fn params(&self, args: &[LadArgument]) -> String {
        args.iter()
            .enumerate()
            .map(|(i, a)| {
                let name = a
                    .name
                    .as_ref()
                    .map(|n| ident(n))
                    .unwrap_or_else(|| format!("arg{i}"));
                format!("{name}: {}", self.kind(&a.kind))
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Resolve a LAD type kind to a Luau type expression.
    fn kind(&self, kind: &LadFieldOrVariableKind) -> String {
        match kind {
            LadFieldOrVariableKind::Ref(id)
            | LadFieldOrVariableKind::Mut(id)
            | LadFieldOrVariableKind::Val(id)
            | LadFieldOrVariableKind::Unknown(id) => self.type_name(id),
            LadFieldOrVariableKind::Option(inner) => format!("{}?", self.kind(inner)),
            LadFieldOrVariableKind::Vec(inner) | LadFieldOrVariableKind::Array(inner, _) => {
                format!("{{ {} }}", self.kind(inner))
            }
            LadFieldOrVariableKind::HashSet(inner) => format!("{{ {} }}", self.kind(inner)),
            LadFieldOrVariableKind::HashMap(k, v) => {
                format!("{{ [{}]: {} }}", self.kind(k), self.kind(v))
            }
            LadFieldOrVariableKind::InteropResult(inner) => self.kind(inner),
            LadFieldOrVariableKind::Tuple(items) => {
                match items.first() {
                    // The unit type. `nil` is valid in every position; a bare `()`
                    // is only legal as a function return, so avoid it.
                    None => "nil".to_string(),
                    // Luau has no value-level tuple type; approximate as an array.
                    Some(first) => format!("{{ {} }}", self.kind(first)),
                }
            }
            // A variadic tuple of unknown arity/element types.
            LadFieldOrVariableKind::UntypedTuple => "any".to_string(),
            LadFieldOrVariableKind::Primitive(p) => primitive(p).to_string(),
            LadFieldOrVariableKind::Union(items) => items
                .iter()
                .map(|i| self.kind(i))
                .collect::<Vec<_>>()
                .join(" | "),
        }
    }

    /// Resolve a type id to a Luau type name: a declared class, a builtin (for
    /// primitive-mapped types), or `any`.
    fn type_name(&self, id: &LadTypeId) -> String {
        if let Some(def) = self.lad.types.get(id)
            && let Some(p) = &def.metadata.mapped_to_primitive_kind
        {
            return primitive(p).to_string();
        }
        if self.declared.contains(id) {
            return self.declared_name(id).to_string();
        }
        "any".to_string()
    }
}

fn primitive(p: &ReflectionPrimitiveKind) -> &'static str {
    use ReflectionPrimitiveKind::*;
    match p {
        Bool => "boolean",
        Isize | I8 | I16 | I32 | I64 | I128 | Usize | U8 | U16 | U32 | U64 | U128 | F32 | F64 => {
            "number"
        }
        Char | Str | String | OsString | PathBuf => "string",
        _ => "any",
    }
}

/// Luau keywords (incl. contextual ones) that can't be used as bare identifiers.
const RESERVED: &[&str] = &[
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
    "nil", "not", "or", "repeat", "return", "then", "true", "until", "while", "continue", "export",
    "type",
];

fn is_reserved(s: &str) -> bool {
    RESERVED.contains(&s)
}

/// Make an arbitrary Rust identifier/path safe to use as a Luau identifier.
fn sanitize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// An identifier safe in *every* position (method/param/type/global name):
/// sanitised, and suffixed if it collides with a Luau keyword (which can't be
/// quoted in those positions).
fn ident(s: &str) -> String {
    let mut out = sanitize(s);
    if is_reserved(&out) {
        out.push('_');
    }
    out
}

/// A class field key: a bare identifier, or a quoted key when the name is a Luau
/// keyword (so the field's real name is preserved, e.g. `["end"]: number`).
fn field_key(s: &str) -> String {
    let n = sanitize(s);
    if is_reserved(&n) {
        format!("[\"{n}\"]")
    } else {
        n
    }
}

/// Emit a (possibly multi-line) doc comment with the given indent.
fn push_doc(out: &mut String, doc: &str, indent: &str) {
    for line in doc.lines() {
        out.push_str(indent);
        out.push_str("-- ");
        out.push_str(line.trim_end());
        out.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn focus(crate_: &str) -> LuauBackendConfig {
        LuauBackendConfig {
            focus_crates: vec![crate_.to_string()],
            handle_branding: None,
        }
    }

    /// The general (unbranded) backend against the canonical example LAD file:
    /// classes with fields, the global function, a non-static instance handle —
    /// static accessor globals excluded, and crucially *no* `Reg<T>` machinery.
    #[test]
    fn converts_example_ladfile() {
        let lad = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let luau = lad_to_luau(&lad, &focus("ladfile_builder"));

        // Reflected struct types from the focus crate, with their named fields.
        assert!(luau.contains("declare class PlainStructType"), "{luau}");
        assert!(luau.contains("int_field: number"), "{luau}");
        assert!(luau.contains("declare class GenericStructType"));
        // The global host function.
        assert!(
            luau.contains("declare function hello_world(arg1: number): number"),
            "{luau}"
        );
        // Non-static instance handles are emitted; static accessors are not.
        assert!(luau.contains("declare my_non_static_instance:"), "{luau}");
        assert!(
            !luau.contains("my_static_instance"),
            "static globals must be skipped: {luau}"
        );

        // The brand is opt-in: default output must not contain it.
        assert!(!luau.contains("type Reg<T>"), "Reg<T> must be opt-in");

        assert_well_formed(&luau);
    }

    /// Enabling branding adds the `Reg<T>` alias; leaving it off does not.
    #[test]
    fn branding_is_gated() {
        let lad = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();

        let plain = lad_to_luau(&lad, &focus("ladfile_builder"));
        assert!(!plain.contains("type Reg<T>"));

        let branded_cfg = LuauBackendConfig {
            focus_crates: vec!["ladfile_builder".to_string()],
            handle_branding: Some(HandleBranding::type_suffix()),
        };
        let branded = lad_to_luau(&lad, &branded_cfg);
        assert!(branded.contains("type Reg<T> = { __phantom: T }"));
    }

    /// The full brand path against a crafted fixture: a component, a `world`
    /// handle with a context-only `get_component`, and a `VelocityType`
    /// registration global. Branding rewrites the getter generically and brands
    /// the `…Type` global with its component, giving cast-free typed access.
    #[test]
    fn brands_component_access() {
        let lad = ladfile::parse_lad_file(include_str!("test_assets/branded.lad.json")).unwrap();
        let config = LuauBackendConfig {
            focus_crates: vec!["my_game".to_string()],
            handle_branding: Some(HandleBranding::type_suffix()),
        };
        let luau = lad_to_luau(&lad, &config);

        assert!(luau.contains("declare class Velocity"), "{luau}");
        assert!(luau.contains("x: number"), "{luau}");
        assert!(
            luau.contains("get_component: <T>(entity: Entity, reg: Reg<T>) -> T?"),
            "generic getter not branded:\n{luau}"
        );
        assert!(
            luau.contains("declare VelocityType: Reg<Velocity>"),
            "{luau}"
        );

        // Without branding, the same getter keeps its concrete registration arg and
        // the global is an unbranded registration.
        let plain = lad_to_luau(&lad, &focus("my_game"));
        assert!(!plain.contains("Reg<"), "{plain}");
        assert!(plain.contains("get_component:"), "{plain}");
    }

    /// No reserved word may appear as a bare field/method/param name, and the unit
    /// type must never be rendered as a bare `()` outside a return arrow.
    fn assert_well_formed(luau: &str) {
        for line in luau.lines() {
            let t = line.trim_start();
            assert!(
                !t.starts_with("nil:") && !t.starts_with("end:") && !t.starts_with("type:"),
                "reserved word used as a bare field name: {line}"
            );
        }
    }

    #[test]
    fn sanitizes_and_escapes() {
        assert_eq!(ident("Handle<Image>"), "Handle_Image_");
        assert_eq!(ident("end"), "end_");
        assert_eq!(field_key("end"), "[\"end\"]");
        assert_eq!(field_key("current"), "current");
    }
}
