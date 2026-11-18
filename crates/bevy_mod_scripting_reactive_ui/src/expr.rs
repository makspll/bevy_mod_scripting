//! Values computed in Rust from signals and watched components ([`UiValue`]), so that properties made of them
//! update without running script code.

use std::{any::TypeId, sync::Arc};

use bevy_ecs::{component::ComponentId, world::World};
use bevy_mod_scripting_bindings::{
    FunctionCallContext, InteropError, Lazy, NamespaceMembers, ReflectReference,
    ScriptComponentRegistration, ScriptValue, Union, V, WorldGuard,
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_reflect::{PartialReflect, Reflect, ReflectRef};
use parking_lot::Mutex;

use crate::{
    Selector, Signal,
    props::Constant,
    runtime::{GraphNode, NodeId, NodeKind, Runtime, selector_key},
};

/// A value computed in Rust, made with `path`, `format`, `percent`, `lt`, `mul`, `when` and `selected`.
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
    /// In a row template: the row's item, see [`Expr::bind`]
    Item,
    /// In a row template: a field of a component of the row's item, which is an entity
    ItemPath {
        component: ComponentId,
        type_id: TypeId,
        path: Vec<String>,
    },
    /// In a row template: [`Expr::Is`] with a key read from the row once, when it is built
    IsKey {
        selector: NodeId,
        key: Box<Expr>,
    },
}

/// A component of the item of each row of a template, made with `watch_item` and read with `path`.
#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct ItemWatch {
    component: ComponentId,
    type_id: TypeId,
}

/// What a script passes where a [`UiValue`] is expected: one, or a plain value.
type ValueArg = Union<V<UiValue>, Lazy<Constant>>;

fn arg(value: ValueArg) -> Expr {
    match value.into_left() {
        Ok(value) => clone_expr(&value.into_inner().0),
        Err(constant) => Expr::Const(constant.0),
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
        Expr::Item => Expr::Item,
        Expr::ItemPath {
            component,
            type_id,
            path,
        } => Expr::ItemPath {
            component: *component,
            type_id: *type_id,
            path: path.clone(),
        },
        Expr::IsKey { selector, key } => Expr::IsKey {
            selector: *selector,
            key: Box::new(clone_expr(key)),
        },
    }
}

/// The value standing for the item of each row, given to the function of a row template.
pub(crate) fn item_placeholder() -> ScriptValue {
    ScriptValue::Reference(ReflectReference::new_allocated(UiValue(Arc::new(
        Expr::Item,
    ))))
}

fn value(expr: Expr) -> V<UiValue> {
    V(UiValue(Arc::new(expr)))
}

impl Expr {
    /// The signals, computeds, watches and selectors (with the key asked about) this reads.
    pub fn sources(&self, out: &mut Vec<(NodeId, Option<String>)>) {
        match self {
            // what stands for a row's item reads nothing until it is bound to a row
            Expr::Const(_) | Expr::Item | Expr::ItemPath { .. } | Expr::IsKey { .. } => {}
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
            Expr::Item | Expr::ItemPath { .. } | Expr::IsKey { .. } => {
                return Err(InteropError::str(
                    "the item of a row can only be used inside the function given to `list_template`",
                ));
            }
        })
    }

    /// The value for one row of a template: what stands for the row's item is replaced by the item.
    ///
    /// `watch` gives the node watching a component of the item, `eval` evaluates a bound value.
    pub fn bind(
        &self,
        item: &ScriptValue,
        watch: &mut dyn FnMut(ComponentId, TypeId) -> Result<NodeId, InteropError>,
        eval: &dyn Fn(&Expr) -> Result<ScriptValue, InteropError>,
        world: &WorldGuard,
    ) -> Result<Expr, InteropError> {
        let mut bind = |expr: &Expr| expr.bind(item, watch, eval, world).map(Box::new);
        Ok(match self {
            Expr::Item => Expr::Const(item.clone()),
            Expr::ItemPath {
                component,
                type_id,
                path,
            } => Expr::Path {
                source: watch(*component, *type_id)?,
                path: path.clone(),
            },
            Expr::IsKey { selector, key } => {
                let key = eval(&*bind(key)?)?;
                Expr::Is {
                    selector: *selector,
                    key: selector_key(&key, world)?.unwrap_or_default(),
                }
            }
            Expr::Format { pieces, args } => Expr::Format {
                pieces: pieces.clone(),
                args: args
                    .iter()
                    .map(|a| a.bind(item, watch, eval, world))
                    .collect::<Result<_, _>>()?,
            },
            Expr::Percent(a, b) => Expr::Percent(bind(a)?, bind(b)?),
            Expr::Lt(a, b) => Expr::Lt(bind(a)?, bind(b)?),
            Expr::Mul(a, b) => Expr::Mul(bind(a)?, bind(b)?),
            Expr::Select(c, a, b) => Expr::Select(bind(c)?, bind(a)?, bind(b)?),
            Expr::Const(_) | Expr::Path { .. } | Expr::Is { .. } => clone_expr(self),
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
        Some(NodeKind::Signal { value, .. }) => return walk_script_value(value, path),
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

/// The helpers making [`UiValue`]s in the `reactive_ui` namespace.
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "value_functions",
    namespace = "reactive_ui"
)]
impl NamespaceMembers {
    /// A field of the value of a signal, computed or watched component, i.e. `path(health, "data.current")`.
    /// Struct fields are named, tuple fields and list items are numbered from 0, an empty path is the value itself.
    ///
    /// Values computed in Rust update without running script code: for what changes every frame or exists hundreds
    /// of times.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local player = ui.signal({ name = "ada", hp = 100 })
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "name", text = ui.path(player, "name") }))
    /// s:act(function() player:set({ name = "grace", hp = 100 }) end)
    /// assert(s:text(s:get_by_name("name")) == "grace")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let player = ui.signal(#{ name: "ada", hp: 100 });
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "name", text: ui.path(player, "name") }));
    /// s.act([|player| player.set(#{ name: "grace", hp: 100 }), player]);
    /// assert!(s.text(s.get_by_name("name")) == "grace");
    /// ```
    ///
    /// Arguments:
    /// * `source`: The signal, or in a row template the component of the row's item.
    /// * `path`: The path of the field.
    /// Returns:
    /// * `value`: The field.
    fn path(source: Union<V<Signal>, V<ItemWatch>>, path: String) -> V<UiValue> {
        let path = path
            .split('.')
            .filter(|segment| !segment.is_empty())
            .map(str::to_owned)
            .collect();
        value(match source.into_left() {
            Ok(source) => Expr::Path {
                source: source.into_inner().0,
                path,
            },
            Err(watch) => Expr::ItemPath {
                component: watch.component,
                type_id: watch.type_id,
                path,
            },
        })
    }

    /// In the function given to `list_template`: the component of each row's item, which must be an entity.
    /// Read its fields with `path`, i.e. `path(watch_item(item, Item), "data.count")`.
    ///
    /// ```multi-lang-example,lua
    /// local bms = import_namespace("bms")
    /// local ui = import_namespace("reactive_ui")
    /// ---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
    /// local Item = bms.world.register_new_component("script.docs", "Item", true)
    /// local sword = bms.world.spawn()
    /// bms.world.insert_component(sword, Item, bms.DynamicComponent.new({ data = { count = 3 } }))
    /// local items = ui.signal({ sword })
    /// local s = import_namespace("reactive_ui_test").render(ui.list_template(items, function(item)
    ///     return ui.text({ name = "count", text = ui.format("x{}", { ui.path(ui.watch_item(item, Item), "data.count") }) })
    /// end))
    /// assert(s:text(s:get_by_name("count")) == "x3")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let bms = import_namespace("bms");
    /// let ui = import_namespace("reactive_ui");
    /// let Item = bms.world.register_new_component("script.docs", "Item", true);
    /// state()["Item"] = Item;
    /// let sword = bms.world.spawn();
    /// bms.world.insert_component(sword, Item, bms.DynamicComponent.new(#{ data: #{ count: 3 } }));
    /// let items = ui.signal([sword]);
    /// let s = import_namespace("reactive_ui_test").render(ui.list_template(items, |item| {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: "count", text: ui.format("x{}", [ui.path(ui.watch_item(item, state()["Item"]), "data.count")]) })
    /// }));
    /// assert!(s.text(s.get_by_name("count")) == "x3");
    /// ```
    ///
    /// Arguments:
    /// * `item`: The item given to the function.
    /// * `component`: The component type.
    /// Returns:
    /// * `watch`: The component of each row's item.
    fn watch_item(
        item: V<UiValue>,
        component: V<ScriptComponentRegistration>,
    ) -> Result<V<ItemWatch>, InteropError> {
        if !matches!(&*item.0.0, Expr::Item) {
            return Err(InteropError::str(
                "`watch_item` takes the item given to the function of `list_template`",
            ));
        }
        Ok(V(ItemWatch {
            component: component.component_id(),
            type_id: component.type_registration().type_id(),
        }))
    }

    /// Text made from a template, each `{}` is replaced by the next value, i.e. `format("{} / {}", { current, max })`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hp = ui.signal(80)
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "hp", text = ui.format("{} / {}", { ui.path(hp, ""), 100 }) }))
    /// assert(s:text(s:get_by_name("hp")) == "80 / 100")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let hp = ui.signal(80);
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "hp", text: ui.format("{} / {}", [ui.path(hp, ""), 100]) }));
    /// assert!(s.text(s.get_by_name("hp")) == "80 / 100");
    /// ```
    ///
    /// Arguments:
    /// * `template`: The template.
    /// * `args`: The values.
    /// Returns:
    /// * `value`: The text.
    fn format(template: String, args: Vec<ValueArg>) -> V<UiValue> {
        value(Expr::Format {
            pieces: template.split("{}").map(str::to_owned).collect(),
            args: args.into_iter().map(arg).collect(),
        })
    }

    /// `part` as a percentage of `whole`, as a size, i.e. `width = percent(current, max)`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hp = ui.signal(25)
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "bar", width = ui.percent(ui.path(hp, ""), 100) }))
    /// s:expect_style(s:get_by_name("bar"), { width = "25%" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let hp = ui.signal(25);
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "bar", width: ui.percent(ui.path(hp, ""), 100) }, []));
    /// s.expect_style(s.get_by_name("bar"), #{ width: "25%" });
    /// ```
    ///
    /// Arguments:
    /// * `part`: The part.
    /// * `whole`: The whole.
    /// Returns:
    /// * `value`: The size.
    fn percent(part: ValueArg, whole: ValueArg) -> V<UiValue> {
        value(Expr::Percent(Box::new(arg(part)), Box::new(arg(whole))))
    }

    /// Whether `a` is less than `b`, for `when`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hp = ui.signal(20)
    /// local low = ui.lt(ui.path(hp, ""), 30)
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "hp", text = ui.when(low, "LOW", "ok") }))
    /// assert(s:text(s:get_by_name("hp")) == "LOW")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let hp = ui.signal(20);
    /// let low = ui.lt(ui.path(hp, ""), 30);
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "hp", text: ui.when(low, "LOW", "ok") }));
    /// assert!(s.text(s.get_by_name("hp")) == "LOW");
    /// ```
    ///
    /// Arguments:
    /// * `a`: The left side.
    /// * `b`: The right side.
    /// Returns:
    /// * `value`: Whether it is less.
    fn lt(a: ValueArg, b: ValueArg) -> V<UiValue> {
        value(Expr::Lt(Box::new(arg(a)), Box::new(arg(b))))
    }

    /// `a` times `b`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local half = ui.signal(25)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "bar", width = ui.percent(ui.mul(ui.path(half, ""), 2), 100) })
    /// )
    /// s:expect_style(s:get_by_name("bar"), { width = "50%" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let half = ui.signal(25);
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "bar", width: ui.percent(ui.mul(ui.path(half, ""), 2), 100) }, [])
    /// );
    /// s.expect_style(s.get_by_name("bar"), #{ width: "50%" });
    /// ```
    ///
    /// Arguments:
    /// * `a`: The left side.
    /// * `b`: The right side.
    /// Returns:
    /// * `value`: The product.
    fn mul(a: ValueArg, b: ValueArg) -> V<UiValue> {
        value(Expr::Mul(Box::new(arg(a)), Box::new(arg(b))))
    }

    /// `if_true` if `condition` is true, otherwise `if_false`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local on = ui.signal(true)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "light", background_color = ui.when(ui.path(on, ""), "#ffcc00", "#333333") })
    /// )
    /// s:act(function() on:set(false) end)
    /// s:expect_style(s:get_by_name("light"), { background_color = "#333333" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let on = ui.signal(true);
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "light", background_color: ui.when(ui.path(on, ""), "#ffcc00", "#333333") }, [])
    /// );
    /// s.act([|on| on.set(false), on]);
    /// s.expect_style(s.get_by_name("light"), #{ background_color: "#333333" });
    /// ```
    ///
    /// Arguments:
    /// * `condition`: The condition.
    /// * `if_true`: The value if it is true.
    /// * `if_false`: The value otherwise.
    /// Returns:
    /// * `value`: One of the two.
    fn when(condition: ValueArg, if_true: ValueArg, if_false: ValueArg) -> V<UiValue> {
        value(Expr::Select(
            Box::new(arg(condition)),
            Box::new(arg(if_true)),
            Box::new(arg(if_false)),
        ))
    }

    /// Whether `key` is selected, i.e. `background_color = when(selected(selector, name), "#ffcc00", "#333333")`.
    /// In a row template the key can be a value of the row, it is read once when the row is built.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local items = ui.signal({ "sword", "shield" })
    /// local is_selected = ui.selector(ui.signal("shield"))
    /// local s = import_namespace("reactive_ui_test").render(ui.list_template(items, function(item)
    ///     return ui.node({
    ///         name = ui.format("row_{}", { item }),
    ///         background_color = ui.when(ui.selected(is_selected, item), "#ffcc00", "#333333"),
    ///     })
    /// end))
    /// s:expect_style(s:get_by_name("row_shield"), { background_color = "#ffcc00" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let items = ui.signal(["sword", "shield"]);
    /// let is_selected = ui.selector(ui.signal("shield"));
    /// let s = import_namespace("reactive_ui_test").render(ui.list_template(items, [|is_selected, item| {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.node(#{
    ///         name: ui.format("row_{}", [item]),
    ///         background_color: ui.when(ui.selected(is_selected, item), "#ffcc00", "#333333"),
    ///     }, [])
    /// }, is_selected]));
    /// s.expect_style(s.get_by_name("row_shield"), #{ background_color: "#ffcc00" });
    /// ```
    ///
    /// Arguments:
    /// * `selector`: The selector.
    /// * `key`: The key.
    /// Returns:
    /// * `value`: Whether it is selected.
    fn selected(
        ctx: FunctionCallContext,
        selector: V<Selector>,
        key: ValueArg,
    ) -> Result<V<UiValue>, InteropError> {
        let selector = selector.into_inner().0;
        let key = match key.into_left() {
            // a value of the row in a template, read when the row is built
            Ok(key) => {
                return Ok(value(Expr::IsKey {
                    selector,
                    key: Box::new(clone_expr(&key.0.0)),
                }));
            }
            Err(key) => key.0,
        };
        let key = selector_key(&key, &ctx.world()?)?.unwrap_or_default();
        Ok(value(Expr::Is { selector, key }))
    }
}
