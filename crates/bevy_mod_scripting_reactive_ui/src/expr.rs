//! Values computed in Rust from signals and watched components ([`UiValue`]), so that properties made of them
//! update without running script code.

use std::sync::Arc;

use bevy_ecs::world::World;
use bevy_mod_scripting_bindings::{
    InteropError, NamespaceMembers, ReflectReference, ScriptValue, WorldGuard,
    function::{
        from::{Union, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_reflect::{PartialReflect, Reflect, ReflectRef};
use parking_lot::Mutex;

use crate::{
    Selector, Signal,
    runtime::{GraphNode, NodeId, NodeKind, Runtime, selector_key},
};

/// A value computed in Rust, made with `path`, `format`, `percent`, `lt`, `mul`, `select` and `is`.
///
/// Used as a property, it updates when the signals and components it reads change, without running script code.
#[derive(Clone, Reflect)]
#[reflect(opaque, Clone)]
pub struct UiValue(pub(crate) Arc<Expr>);

impl std::fmt::Debug for UiValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UiValue")
    }
}

pub(crate) enum Expr {
    Const(ScriptValue),
    /// A field of the value of a signal, computed or watched component
    Path {
        source: NodeId,
        path: Vec<String>,
    },
    /// The template split at its `{}`s, and the values filling them
    Format {
        pieces: Vec<String>,
        args: Vec<Expr>,
    },
    Percent(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Select(Box<Expr>, Box<Expr>, Box<Expr>),
    Is {
        selector: NodeId,
        key: String,
    },
}

/// What a script passes where a [`UiValue`] is expected: one, or a plain value.
type ValueArg = Union<V<UiValue>, ScriptValue>;

fn arg(value: ValueArg) -> Expr {
    match value.into_left() {
        Ok(value) => clone_expr(&value.into_inner().0),
        Err(constant) => Expr::Const(constant),
    }
}

fn clone_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Const(v) => Expr::Const(v.clone()),
        Expr::Path { source, path } => Expr::Path {
            source: *source,
            path: path.clone(),
        },
        Expr::Format { pieces, args } => Expr::Format {
            pieces: pieces.clone(),
            args: args.iter().map(clone_expr).collect(),
        },
        Expr::Percent(a, b) => Expr::Percent(Box::new(clone_expr(a)), Box::new(clone_expr(b))),
        Expr::Lt(a, b) => Expr::Lt(Box::new(clone_expr(a)), Box::new(clone_expr(b))),
        Expr::Mul(a, b) => Expr::Mul(Box::new(clone_expr(a)), Box::new(clone_expr(b))),
        Expr::Select(c, a, b) => Expr::Select(
            Box::new(clone_expr(c)),
            Box::new(clone_expr(a)),
            Box::new(clone_expr(b)),
        ),
        Expr::Is { selector, key } => Expr::Is {
            selector: *selector,
            key: key.clone(),
        },
    }
}

fn value(expr: Expr) -> V<UiValue> {
    V(UiValue(Arc::new(expr)))
}

impl Expr {
    /// The signals, computeds, watches and selectors (with the key asked about) this reads.
    pub fn sources(&self, out: &mut Vec<(NodeId, Option<String>)>) {
        match self {
            Expr::Const(_) => {}
            Expr::Path { source, .. } => out.push((*source, None)),
            Expr::Is { selector, key } => out.push((*selector, Some(key.clone()))),
            Expr::Format { args, .. } => args.iter().for_each(|a| a.sources(out)),
            Expr::Percent(a, b) | Expr::Lt(a, b) | Expr::Mul(a, b) => {
                a.sources(out);
                b.sources(out);
            }
            Expr::Select(c, a, b) => {
                c.sources(out);
                a.sources(out);
                b.sources(out);
            }
        }
    }

    /// Whether it reads a computed, which needs script code to bring up to date.
    pub fn reads_computed(&self, rt: &Runtime) -> bool {
        let mut sources = Vec::new();
        self.sources(&mut sources);
        sources.iter().any(|(id, _)| {
            matches!(
                rt.nodes.get(id),
                Some(GraphNode {
                    kind: NodeKind::Computed { .. },
                    ..
                })
            )
        })
    }

    /// Evaluates it, computeds it reads must be up to date.
    pub fn eval(
        &self,
        runtime: &Mutex<Runtime>,
        world: &WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        Ok(match self {
            Expr::Const(v) => v.clone(),
            Expr::Path { source, path } => read_path(runtime, world, *source, path)?,
            Expr::Format { pieces, args } => {
                let mut out = String::new();
                for (i, piece) in pieces.iter().enumerate() {
                    out.push_str(piece);
                    if let Some(arg) = args.get(i) {
                        out.push_str(&text(&arg.eval(runtime, world)?));
                    }
                }
                ScriptValue::String(out.into())
            }
            Expr::Percent(a, b) => {
                let (a, b) = (
                    number(&a.eval(runtime, world)?)?,
                    number(&b.eval(runtime, world)?)?,
                );
                ScriptValue::String(
                    format!("{}%", if b == 0.0 { 0.0 } else { a / b * 100.0 }).into(),
                )
            }
            Expr::Lt(a, b) => ScriptValue::Bool(
                number(&a.eval(runtime, world)?)? < number(&b.eval(runtime, world)?)?,
            ),
            Expr::Mul(a, b) => ScriptValue::Float(
                number(&a.eval(runtime, world)?)? * number(&b.eval(runtime, world)?)?,
            ),
            Expr::Select(c, a, b) => {
                if truthy(&c.eval(runtime, world)?) {
                    a.eval(runtime, world)?
                } else {
                    b.eval(runtime, world)?
                }
            }
            Expr::Is { selector, key } => match runtime.lock().nodes.get(selector) {
                Some(GraphNode {
                    kind: NodeKind::Selector { current, .. },
                    ..
                }) => ScriptValue::Bool(current.as_deref() == Some(key.as_str())),
                _ => return Err(InteropError::str("this selector no longer exists")),
            },
        })
    }
}

fn truthy(value: &ScriptValue) -> bool {
    !matches!(value, ScriptValue::Unit | ScriptValue::Bool(false))
}

fn number(value: &ScriptValue) -> Result<f64, InteropError> {
    match value {
        ScriptValue::Integer(i) => Ok(*i as f64),
        ScriptValue::Float(f) => Ok(*f),
        other => Err(InteropError::string(format!(
            "expected a number, got: {other:?}"
        ))),
    }
}

fn text(value: &ScriptValue) -> String {
    match value {
        ScriptValue::String(s) => s.to_string(),
        ScriptValue::Integer(i) => i.to_string(),
        ScriptValue::Float(f) => f.to_string(),
        ScriptValue::Bool(b) => b.to_string(),
        ScriptValue::Unit => String::new(),
        other => format!("{other:?}"),
    }
}

/// Walks `path` into a script value, by map key or list position (from 0).
fn walk_script_value(
    mut value: &ScriptValue,
    path: &[String],
) -> Result<ScriptValue, InteropError> {
    for segment in path {
        value = match value {
            ScriptValue::Map(map) => map.get(segment.as_str()),
            ScriptValue::List(list) => segment.parse::<usize>().ok().and_then(|i| list.get(i)),
            _ => None,
        }
        .ok_or_else(|| InteropError::string(format!("no `{segment}` in {value:?}")))?;
    }
    Ok(value.clone())
}

/// Walks `path` into a reflected value: struct fields by name, tuple fields and list items by position (from 0).
/// Continues into script values (i.e. the data of script components) by map key.
fn walk_reflect(value: &dyn PartialReflect, path: &[String]) -> Result<ScriptValue, InteropError> {
    let mut value = value;
    for (i, segment) in path.iter().enumerate() {
        if let Some(script_value) = value.try_downcast_ref::<ScriptValue>() {
            return walk_script_value(script_value, &path[i..]);
        }
        let index = segment.parse::<usize>().ok();
        value = match value.reflect_ref() {
            ReflectRef::Struct(s) => s.field(segment),
            ReflectRef::TupleStruct(s) => index.and_then(|i| s.field(i)),
            ReflectRef::Tuple(s) => index.and_then(|i| s.field(i)),
            ReflectRef::List(l) => index.and_then(|i| l.get(i)),
            _ => None,
        }
        .ok_or_else(|| InteropError::string(format!("no field `{segment}` in {value:?}")))?;
    }
    leaf(value)
}

fn leaf(value: &dyn PartialReflect) -> Result<ScriptValue, InteropError> {
    macro_rules! numbers {
        ($($int:ty),*; $($float:ty),*) => {
            $(if let Some(v) = value.try_downcast_ref::<$int>() { return Ok(ScriptValue::Integer(*v as i64)); })*
            $(if let Some(v) = value.try_downcast_ref::<$float>() { return Ok(ScriptValue::Float(*v as f64)); })*
        };
    }
    numbers!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize; f32, f64);
    if let Some(v) = value.try_downcast_ref::<bool>() {
        return Ok(ScriptValue::Bool(*v));
    }
    if let Some(v) = value.try_downcast_ref::<String>() {
        return Ok(ScriptValue::String(v.clone().into()));
    }
    if let Some(v) = value.try_downcast_ref::<ScriptValue>() {
        return Ok(v.clone());
    }
    Err(InteropError::string(format!(
        "paths must lead to a number, boolean or string, got: {value:?}"
    )))
}

fn read_path(
    runtime: &Mutex<Runtime>,
    world: &WorldGuard,
    source: NodeId,
    path: &[String],
) -> Result<ScriptValue, InteropError> {
    let watched = match runtime.lock().nodes.get(&source).map(|n| &n.kind) {
        Some(NodeKind::Signal { value }) => return walk_script_value(value, path),
        Some(NodeKind::Computed { value, .. }) => {
            return walk_script_value(value.as_ref().unwrap_or(&ScriptValue::Unit), path);
        }
        Some(NodeKind::Watch {
            entity,
            component,
            type_id,
            ..
        }) => (*entity, *component, *type_id),
        _ => {
            return Err(InteropError::str(
                "paths can only read signals, computeds and watches",
            ));
        }
    };
    let (entity, component, type_id) = watched;
    let present = world.with_world_access(|w: &World| {
        w.get_entity(entity).is_ok_and(|e| e.contains_id(component))
    })?;
    if !present {
        return Ok(ScriptValue::Unit);
    }
    // Safety: the type id is the type of the component, both come from its registration
    let reference =
        unsafe { ReflectReference::new_component_ref_by_id(entity, component, type_id) };
    reference.with_reflect(world.clone(), |r| walk_reflect(r, path))?
}

/// Registers the helpers making [`UiValue`]s in the `reactive_ui` namespace.
pub(crate) fn register(world: &mut World) {
    NamespaceBuilder::<NamespaceMembers>::in_namespace(world, "reactive_ui")
        .register_documented(
            "path",
            |source: V<Signal>, path: String| {
                value(Expr::Path {
                    source: source.into_inner().0,
                    path: path.split('.').map(str::to_owned).collect(),
                })
            },
            "A field of the value of a signal, computed or watched component, i.e. `path(health, \"data.current\")`.\n\
             Struct fields are named, tuple fields and list items are numbered from 0.",
            &["source", "path"],
        )
        .register_documented(
            "format",
            |template: String, args: Vec<ValueArg>| {
                value(Expr::Format {
                    pieces: template.split("{}").map(str::to_owned).collect(),
                    args: args.into_iter().map(arg).collect(),
                })
            },
            "Text made from a template, each `{}` is replaced by the next value, i.e. `format(\"{} / {}\", { current, max })`.",
            &["template", "args"],
        )
        .register_documented(
            "percent",
            |part: ValueArg, whole: ValueArg| {
                value(Expr::Percent(Box::new(arg(part)), Box::new(arg(whole))))
            },
            "`part` as a percentage of `whole`, as a size, i.e. `width = percent(current, max)`.",
            &["part", "whole"],
        )
        .register_documented(
            "lt",
            |a: ValueArg, b: ValueArg| value(Expr::Lt(Box::new(arg(a)), Box::new(arg(b)))),
            "Whether `a` is less than `b`.",
            &["a", "b"],
        )
        .register_documented(
            "mul",
            |a: ValueArg, b: ValueArg| value(Expr::Mul(Box::new(arg(a)), Box::new(arg(b)))),
            "`a` times `b`.",
            &["a", "b"],
        )
        .register_documented(
            "select",
            |condition: ValueArg, if_true: ValueArg, if_false: ValueArg| {
                value(Expr::Select(
                    Box::new(arg(condition)),
                    Box::new(arg(if_true)),
                    Box::new(arg(if_false)),
                ))
            },
            "`if_true` if `condition` is true, otherwise `if_false`.",
            &["condition", "if_true", "if_false"],
        )
        .register_documented(
            "is",
            |selector: V<Selector>, key: ScriptValue| {
                let key = selector_key(&key).ok_or_else(|| {
                    InteropError::str("selector keys must be strings, numbers or booleans")
                })?;
                Ok::<_, InteropError>(value(Expr::Is {
                    selector: selector.into_inner().0,
                    key,
                }))
            },
            "Whether `key` is selected, i.e. `bg = select(is(selected, name), \"#ffcc00\", \"#333333\")`.",
            &["selector", "key"],
        );
}
