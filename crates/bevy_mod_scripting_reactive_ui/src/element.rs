//! UI descriptions built by scripts ([`UiElement`]) and the properties they set on `bevy_ui` entities.

use std::{marker::PhantomData, sync::Arc};

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_bindings::{
    A, DynamicScriptFunction, InteropError, ScriptValue, WorldGuard,
};
use bevy_reflect::Reflect;

use parking_lot::Mutex;

use crate::{expr::Expr, runtime::Runtime};

pub(crate) use crate::style::{Prop, UiParts, apply_prop};

/// A script function stored on the Rust side.
pub type Func = DynamicScriptFunction;

/// A component definition, made with `component`, keyed by the defining script and its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(opaque, Clone)]
pub struct ComponentDef(pub(crate) u32);

/// A description of a piece of UI, made by `node`, `text`, a component, `list` or `show`.
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

#[derive(Clone)]
pub(crate) struct ElementData {
    pub kind: ElementKind,
    pub key: Option<String>,
    pub props: Vec<(Prop, PropValue)>,
    pub children: Vec<UiElement>,
}

#[derive(Clone)]
pub(crate) enum ElementKind {
    Node,
    Text,
    /// An instance of a component, with what it was called with
    Component(ComponentDef, Vec<ScriptValue>),
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

/// A context of a `T` made with `create_context`: a value components read with `use_context`, the default
/// unless an element above them provides another with `provide`.
#[derive(Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct UiContext<T = A>(pub(crate) u32, PhantomData<fn() -> T>);

impl<T> UiContext<T> {
    pub(crate) fn new(id: u32) -> Self {
        UiContext(id, PhantomData)
    }
}

impl<T> Clone for UiContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UiContext<T> {}

impl<T> PartialEq for UiContext<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for UiContext<T> {}

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

pub(crate) fn key_of(value: &ScriptValue, world: &WorldGuard) -> Result<String, InteropError> {
    // tagged by type so `1` and `"1"` differ, numbers by value so `1` and `1.0` don't (as in Lua)
    Ok(match value {
        ScriptValue::String(s) => format!("s:{s}"),
        ScriptValue::Integer(i) => format!("n:{i}"),
        ScriptValue::Float(f) if f.fract() == 0.0 && f.abs() < i64::MAX as f64 => {
            format!("n:{}", *f as i64)
        }
        ScriptValue::Float(f) => format!("n:{f}"),
        ScriptValue::Bool(b) => format!("b:{b}"),
        ScriptValue::Reference(r) => match r.downcast::<Entity>(world.clone()) {
            Ok(entity) => format!("e:{}", entity.to_bits()),
            Err(_) => format!("r:{r:?}"),
        },
        other => format!("v:{other:?}"),
    })
}

pub(crate) fn script_values_equal(a: &ScriptValue, b: &ScriptValue) -> bool {
    values_equal(a, b, false)
}

/// Whether a component was called with the same arguments, functions compare as equal: a closure made again
/// by a region re-running doesn't make its component render again.
pub(crate) fn props_equal(a: &[ScriptValue], b: &[ScriptValue]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| values_equal(a, b, true))
}

fn values_equal(a: &ScriptValue, b: &ScriptValue, functions_equal: bool) -> bool {
    match (a, b) {
        (ScriptValue::Unit, ScriptValue::Unit) => true,
        (ScriptValue::Bool(a), ScriptValue::Bool(b)) => a == b,
        (ScriptValue::Integer(a), ScriptValue::Integer(b)) => a == b,
        (ScriptValue::Float(a), ScriptValue::Float(b)) => a == b,
        (ScriptValue::Integer(a), ScriptValue::Float(b))
        | (ScriptValue::Float(b), ScriptValue::Integer(a)) => (*a as f64) == *b,
        (ScriptValue::String(a), ScriptValue::String(b)) => a == b,
        (ScriptValue::Reference(a), ScriptValue::Reference(b)) => a == b,
        (
            ScriptValue::Function(_) | ScriptValue::FunctionMut(_),
            ScriptValue::Function(_) | ScriptValue::FunctionMut(_),
        ) => functions_equal,
        (ScriptValue::List(a), ScriptValue::List(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b)
                    .all(|(a, b)| values_equal(a, b, functions_equal))
        }
        (ScriptValue::Map(a), ScriptValue::Map(b)) => {
            a.len() == b.len()
                && a.iter().all(|(k, v)| {
                    b.get(k)
                        .is_some_and(|w| values_equal(v, w, functions_equal))
                })
        }
        _ => false,
    }
}

/// The elements built during the current flush. Scripts get plain integer handles to them, so building and passing
/// elements around needs no allocation for scripts, reflection or access claims.
///
/// Elements are only built while components and regions run, and what is mounted keeps its own copy,
/// so the arena is emptied at the start of every flush.
///
/// Generations start at 1, so a handle is never below `1 << 32`: numbers scripts use by mistake are not handles.
pub(crate) struct ElementArena {
    items: Vec<UiElement>,
    generation: u32,
}

impl Default for ElementArena {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            generation: 1,
        }
    }
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
        if generation == 0 || generation > GENERATION_MASK {
            return Err(InteropError::string(format!(
                "expected a ui element, got the number {handle}"
            )));
        }
        if generation != self.generation {
            return Err(InteropError::str(
                "this ui element was built in an earlier update. Elements are only valid in the update which built \
                 them: build them where they are returned, don't keep them in signals or pass them to `render`",
            ));
        }
        self.items
            .get(index)
            .cloned()
            .ok_or_else(|| InteropError::string(format!("{handle} is not a ui element")))
    }

    /// Whether the value is, or holds, a handle to an element of this update.
    pub fn holds_element(&self, value: &ScriptValue) -> bool {
        match value {
            ScriptValue::Integer(handle) => {
                (handle >> INDEX_BITS) as u32 == self.generation
                    && ((handle & ((1 << INDEX_BITS) - 1)) as usize) < self.items.len()
            }
            ScriptValue::List(list) => list.iter().any(|v| self.holds_element(v)),
            ScriptValue::Map(map) => map.values().any(|v| self.holds_element(v)),
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.items.clear();
        // skips 0 when wrapping, see `ElementArena`
        self.generation = (self.generation % GENERATION_MASK) + 1;
    }
}

/// Reads an element handle out of a script value, `nil` and `false` are no element.
pub(crate) fn element_from_script(
    value: &ScriptValue,
    runtime: &Mutex<Runtime>,
) -> Result<Option<UiElement>, InteropError> {
    match value {
        ScriptValue::Unit | ScriptValue::Bool(false) => Ok(None),
        ScriptValue::Integer(handle) => runtime.lock().elements.get(*handle).map(Some),
        other => Err(InteropError::string(format!(
            "expected a ui element, got: {other:?}"
        ))),
    }
}

/// Reads the elements returned by a dynamic region or passed as children: one element, a list of them, or nothing.
/// `nil` and `false` in a list are skipped (`cond and X()`). A list with holes reaches Rust as a table with integer
/// keys, which is read in the order of its keys.
pub(crate) fn elements_from_script(
    value: &ScriptValue,
    runtime: &Mutex<Runtime>,
) -> Result<Vec<UiElement>, InteropError> {
    let read = |items: &mut dyn Iterator<Item = &ScriptValue>| {
        let rt = runtime.lock();
        items
            .filter_map(|v| match v {
                ScriptValue::Unit | ScriptValue::Bool(false) => None,
                ScriptValue::Integer(handle) => Some(rt.elements.get(*handle)),
                other => Some(Err(InteropError::string(format!(
                    "expected a ui element, got: {other:?}"
                )))),
            })
            .collect()
    };
    match value {
        ScriptValue::List(list) => read(&mut list.iter()),
        ScriptValue::Map(map) if map.is_empty() => Ok(Vec::new()),
        ScriptValue::Map(map) => {
            let mut indexed = map
                .iter()
                .map(|(k, v)| k.parse::<i64>().map(|k| (k, v)))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| {
                    InteropError::string(format!("expected a list of ui elements, got: {value:?}"))
                })?;
            indexed.sort_by_key(|(k, _)| *k);
            read(&mut indexed.into_iter().map(|(_, v)| v))
        }
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
