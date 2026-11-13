//! UI descriptions built by scripts ([`UiElement`]) and the properties they set on `bevy_ui` entities.

use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_bindings::{DynamicScriptFunction, InteropError, ScriptValue, WorldGuard};
use bevy_reflect::Reflect;

use parking_lot::Mutex;

use crate::{
    expr::{Expr, UiValue},
    runtime::Runtime,
};

pub(crate) use crate::style::{Prop, UiParts, apply_prop};

/// A script function stored on the Rust side.
pub type Func = DynamicScriptFunction;

/// A component definition, made with `component`, keyed by the defining script and its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(opaque, Clone)]
pub struct ComponentDef(pub(crate) u32);

/// A description of a piece of UI, made by `node`, `text`, `element` or `show`.
///
/// Elements are immutable and cheap to clone, parents share their children.
#[derive(Clone, Reflect)]
#[reflect(opaque, Clone)]
pub struct UiElement(pub(crate) Arc<ElementData>);

impl std::fmt::Debug for UiElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiElement")
            .field("key", &self.0.key)
            .field("children", &self.0.children.len())
            .finish()
    }
}

pub(crate) struct ElementData {
    pub kind: ElementKind,
    pub key: Option<String>,
    pub props: Vec<(Prop, PropValue)>,
    pub children: Vec<UiElement>,
}

pub(crate) enum ElementKind {
    Node,
    Text,
    /// An instance of a component, with the props it is given
    Component(ComponentDef, ScriptValue),
    /// A region whose children are produced by a function, re-run when the signals it reads change
    Dynamic(Func),
    /// A region with one child per item of the list read from a signal, made by calling the function
    /// with the item. Children are kept by item, the function only runs for new items.
    ///
    /// With the flag set the function is a row template: it runs once for the whole list, given a value standing
    /// for the item, and each row is made from the element it returned without running script code.
    Each(crate::runtime::NodeId, Func, bool),
    /// A node providing the value of a context to the components below it, see `provide`
    Provide(UiContext, ScriptValue),
}

/// A context made with `create_context`: a value components read with `use_context`, the default
/// unless an element above them provides another with `provide`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(opaque, Clone)]
pub struct UiContext(pub(crate) u32);

impl ElementKind {
    pub fn same_kind(&self, other: &ElementKind) -> bool {
        match (self, other) {
            (ElementKind::Node, ElementKind::Node)
            | (ElementKind::Text, ElementKind::Text)
            | (ElementKind::Dynamic(_), ElementKind::Dynamic(_))
            | (ElementKind::Each(..), ElementKind::Each(..)) => true,
            (ElementKind::Provide(a, _), ElementKind::Provide(b, _)) => a == b,
            (ElementKind::Component(a, _), ElementKind::Component(b, _)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub(crate) enum PropValue {
    Static(ScriptValue),
    /// Bound to a function, re-run when the signals it reads change
    Bound(Func),
    /// Bound to a value computed in Rust
    Expr(Arc<Expr>),
    Handler(Func),
}

/// The function called with the given dependencies before its own arguments.
///
/// Dependencies are how a function gets values it can't capture: closures of some languages (Rune) can only
/// capture numbers and strings when they are handed to Rust, not signals or entities.
pub(crate) fn with_deps(f: Func, deps: Option<ScriptValue>) -> Result<Func, InteropError> {
    let deps = match deps {
        None | Some(ScriptValue::Unit) => return Ok(f),
        Some(ScriptValue::List(deps)) => deps,
        Some(ScriptValue::Map(map)) if map.is_empty() => return Ok(f),
        Some(other) => {
            return Err(InteropError::string(format!(
                "dependencies must be a list of values, got: {other:?}"
            )));
        }
    };
    if deps.is_empty() {
        return Ok(f);
    }
    Ok(Func::from(
        move |context, mut args: std::collections::VecDeque<ScriptValue>| {
            for dep in deps.iter().rev() {
                args.push_front(dep.clone());
            }
            match f.call(args, context) {
                Ok(value) => value,
                Err(error) => ScriptValue::Error(error),
            }
        },
    ))
}

/// Splits a props map into the key and the known properties, sorted so that diffs walk them in the same order.
pub(crate) fn parse_props(
    props: Option<ScriptValue>,
    world: &WorldGuard,
) -> Result<(Option<String>, Vec<(Prop, PropValue)>), InteropError> {
    let map = match props {
        None | Some(ScriptValue::Unit) => return Ok((None, Vec::new())),
        Some(ScriptValue::String(text)) => {
            return Ok((
                None,
                vec![(Prop::Text, PropValue::Static(ScriptValue::String(text)))],
            ));
        }
        Some(ScriptValue::Map(map)) => map,
        Some(ScriptValue::List(list)) if list.is_empty() => return Ok((None, Vec::new())),
        Some(other) => {
            return Err(InteropError::string(format!(
                "expected a table of properties, got: {other:?}"
            )));
        }
    };
    let mut key = None;
    let mut out = Vec::with_capacity(map.len());
    for (name, value) in map {
        if name == "key" {
            key = Some(key_of(&value, world)?);
            continue;
        }
        let prop = Prop::parse(&name)
            .ok_or_else(|| InteropError::string(format!("unknown ui property `{name}`")))?;
        let value = match value {
            ScriptValue::Function(f) if prop.is_handler() => PropValue::Handler(f),
            ScriptValue::Function(f) => PropValue::Bound(f),
            // a function with its dependencies: `{ f, a, b }` is `f` called with `a` and `b` first
            ScriptValue::List(mut list)
                if matches!(list.front(), Some(ScriptValue::Function(_))) =>
            {
                let Some(ScriptValue::Function(f)) = list.pop_front() else {
                    unreachable!("checked by the guard")
                };
                let f = with_deps(f, Some(ScriptValue::List(list)))?;
                if prop.is_handler() {
                    PropValue::Handler(f)
                } else {
                    PropValue::Bound(f)
                }
            }
            v if prop.is_handler() => {
                return Err(InteropError::string(format!(
                    "`{name}` must be a function, got: {v:?}"
                )));
            }
            ScriptValue::Reference(r) => match r.downcast::<UiValue>(world.clone()) {
                Ok(value) => PropValue::Expr(value.0),
                Err(_) => PropValue::Static(ScriptValue::Reference(r)),
            },
            v => PropValue::Static(v),
        };
        out.push((prop, value));
    }
    out.sort_by_key(|(p, _)| *p);
    Ok((key, out))
}

/// A canonical key for reconciling children: entities compare by value, not by reference.
pub(crate) fn key_of(value: &ScriptValue, world: &WorldGuard) -> Result<String, InteropError> {
    Ok(match value {
        ScriptValue::String(s) => s.to_string(),
        ScriptValue::Integer(i) => i.to_string(),
        ScriptValue::Float(f) => f.to_string(),
        ScriptValue::Bool(b) => b.to_string(),
        ScriptValue::Reference(r) => match r.downcast::<Entity>(world.clone()) {
            Ok(entity) => format!("e{}", entity.to_bits()),
            Err(_) => format!("{r:?}"),
        },
        other => format!("{other:?}"),
    })
}

/// Equality of plain values, references compare by identity.
pub(crate) fn script_values_equal(a: &ScriptValue, b: &ScriptValue) -> bool {
    match (a, b) {
        (ScriptValue::Unit, ScriptValue::Unit) => true,
        (ScriptValue::Bool(a), ScriptValue::Bool(b)) => a == b,
        (ScriptValue::Integer(a), ScriptValue::Integer(b)) => a == b,
        (ScriptValue::Float(a), ScriptValue::Float(b)) => a == b,
        (ScriptValue::Integer(a), ScriptValue::Float(b))
        | (ScriptValue::Float(b), ScriptValue::Integer(a)) => (*a as f64) == *b,
        (ScriptValue::String(a), ScriptValue::String(b)) => a == b,
        (ScriptValue::Reference(a), ScriptValue::Reference(b)) => a == b,
        (ScriptValue::List(a), ScriptValue::List(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| script_values_equal(a, b))
        }
        (ScriptValue::Map(a), ScriptValue::Map(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|w| script_values_equal(v, w)))
        }
        _ => false,
    }
}

/// The elements built during the current flush. Scripts get plain integer handles to them, so building and passing
/// elements around needs no allocation for scripts, reflection or access claims.
///
/// Elements are only built while components and regions run, and what is mounted keeps its own copy,
/// so the arena is emptied at the start of every flush.
#[derive(Default)]
pub(crate) struct ElementArena {
    items: Vec<UiElement>,
    generation: u32,
}

/// Bits of a handle holding the index, the generation is above them. 53 bits in all, so handles stay exact
/// in languages storing numbers as doubles.
const INDEX_BITS: u32 = 32;
const GENERATION_MASK: u32 = (1 << 21) - 1;

impl ElementArena {
    pub fn push(&mut self, element: UiElement) -> i64 {
        self.items.push(element);
        ((self.generation as i64) << INDEX_BITS) | (self.items.len() as i64 - 1)
    }

    pub fn get(&self, handle: i64) -> Result<UiElement, InteropError> {
        let generation = (handle >> INDEX_BITS) as u32;
        let index = (handle & ((1 << INDEX_BITS) - 1)) as usize;
        if generation != self.generation {
            return Err(InteropError::str(
                "this ui element was built in an earlier update, elements are only valid in the update which built them",
            ));
        }
        self.items
            .get(index)
            .cloned()
            .ok_or_else(|| InteropError::string(format!("{handle} is not a ui element")))
    }

    pub fn reset(&mut self) {
        self.items.clear();
        self.generation = (self.generation + 1) & GENERATION_MASK;
    }
}

/// Reads an element handle out of a script value.
pub(crate) fn element_from_script(
    value: &ScriptValue,
    runtime: &Mutex<Runtime>,
) -> Result<Option<UiElement>, InteropError> {
    match value {
        ScriptValue::Unit => Ok(None),
        ScriptValue::Integer(handle) => runtime.lock().elements.get(*handle).map(Some),
        other => Err(InteropError::string(format!(
            "expected a ui element, got: {other:?}"
        ))),
    }
}

/// Reads the elements returned by a dynamic region or passed as children: one element, a list of them, or nil.
pub(crate) fn elements_from_script(
    value: &ScriptValue,
    runtime: &Mutex<Runtime>,
) -> Result<Vec<UiElement>, InteropError> {
    match value {
        ScriptValue::List(list) => {
            let rt = runtime.lock();
            list.iter()
                .filter_map(|v| match v {
                    ScriptValue::Unit => None,
                    ScriptValue::Integer(handle) => Some(rt.elements.get(*handle)),
                    other => Some(Err(InteropError::string(format!(
                        "expected a ui element, got: {other:?}"
                    )))),
                })
                .collect()
        }
        ScriptValue::Map(map) if map.is_empty() => Ok(Vec::new()),
        v => Ok(element_from_script(v, runtime)?.into_iter().collect()),
    }
}

/// What a component renders when its render function returns nothing.
pub(crate) fn empty_node() -> UiElement {
    UiElement(Arc::new(ElementData {
        kind: ElementKind::Node,
        key: None,
        props: Vec::new(),
        children: Vec::new(),
    }))
}

/// Puts a new element in the arena, returning its handle for scripts.
pub(crate) fn new_element(runtime: &Mutex<Runtime>, data: ElementData) -> ScriptValue {
    ScriptValue::Integer(runtime.lock().elements.push(UiElement(Arc::new(data))))
}
