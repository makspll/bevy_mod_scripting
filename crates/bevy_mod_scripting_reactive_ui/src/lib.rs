//! Prototype of reactive `bevy_ui` trees described by scripts, built on signals.
//!
//! Components run once (and again when their script reloads). Properties bound to functions re-run when the
//! signals they read change, and `show` regions re-run to produce changing children.
//! `list(items, function(item) .. end)` is the region for lists: one child per item of the list in a signal,
//! kept by item (items are the keys), so the function only runs for items which were not in the list before.
//!
//! `list_template(items, function(item) .. end)` builds the rows of large uniform lists without running
//! script code per row: the function runs once, `item` stands for each row's item, and the `node`s and `text`s
//! it returns are made per row in Rust. Properties are values computed in Rust (`format`, `when`, `selected`,
//! `path(watch_item(item, Component), ..)` for items which are entities), functions (handlers and bound
//! properties) are shared by all rows and get the row's item after their dependencies.
//!
//! Wherever a function is taken, a list of the function and its dependencies can be given instead (see
//! [`WithDeps`]): `{ f, selected, name }` calls `f(selected, name, ..)`. This is how a function reaches signals and
//! entities in languages whose closures can't capture them when handed to Rust (Rune).
//!
//! `component(name, f)` returns a function making elements of the component: `Counter({ start = 1 })` renders
//! it with `f({ start = 1 })`, `Slot(a, b)` with `f(a, b)`. Arguments are kept as they are, not converted.
//!
//! The API is typed (props are `UiProps`, signals [`Signal<T>`]), so typed languages check UI scripts.
//! Everything is language agnostic: script functions are stored as [`ScriptValue::Function`]s and run
//! in their script's context with [`AppScriptCallbackRunners::run_in_context`].
//!
//! ```lua
//! local ui = import_namespace("reactive_ui")
//! local Counter = ui.component("Counter", function(props)
//!     local n = ui.signal(props.start)
//!     return ui.node({ flex_direction = "Row" }, {
//!         ui.node({ name = "dec", on_click = function() n:set(n:get() - 1) end }),
//!         ui.text({ text = function() return "n=" .. n:get() end }),
//!     })
//! end)
//! function on_script_loaded() ui.render("counter", Counter({ start = 0 })) end
//! ```
//!
//! With the `testing` feature, `ReactiveUiTestPlugin` adds `reactive_ui_test`, for testing UIs from scripts.

mod element;
mod events;
mod expr;
mod props;
mod runtime;

mod style;
#[cfg(feature = "testing")]
mod testing;
#[cfg(feature = "testing")]
pub use testing::{ReactiveUiTestPlugin, Screen};

use std::{collections::VecDeque, marker::PhantomData};

use bevy_mod_scripting_bindings::{
    A, ArgMeta, CallScope, FromScript, GenericTypesAppExt, GetTypeDependencies, Lazy,
    ThroughTypeInfo, TypeOf, TypedThrough, Union, ValueOf, WithDeps,
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_reflect::{GetTypeRegistration, TypePath, TypeRegistry, Typed};
use props::{Element, UiProps, parse_props};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    entity::Entity,
    message::MessageReader,
    reflect::AppTypeRegistry,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::{Commands, Res},
    world::World,
};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    AppScriptCallbackRunners, CurrentScriptAttachment, FunctionCallContext, InteropError,
    NamespaceMembers, ScriptComponentRegistration, ScriptValue, V, WorldAccessGuard,
    WorldExtensions, WorldGuard,
};
use bevy_mod_scripting_core::event::ScriptLifecycleMessage;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_picking::Pickable;
use bevy_reflect::Reflect;
use bevy_ui::{Node, UiSystems, Val};
pub use element::{ComponentDef, UiElement};
use element::{
    ElementData, ElementKind, Func, element_from_script, elements_from_script, new_element,
};
pub use expr::UiValue;
use parking_lot::Mutex;
use runtime::{
    ContextsGuard, CurrentRuntime, Cx, Def, Instance, NodeId, NodeKind, Root, Runtime,
    current_contexts, deps_changed, read, run_rust_binding, runtime, slot_node, take_rust_bindings,
    write,
};
pub use runtime::{ReactiveUi, UiHandlers, UiRoot};
use std::sync::Arc;

/// How many times a flush re-runs work made dirty by the work it ran (i.e. effects setting signals) before leaving it for the next frame.
pub const MAX_FLUSH_ITERATIONS: usize = 16;

/// A handle to a signal of a `T`, computed or watched component, read with `get` (tracked) or `peek` (untracked), set with `set`.
///
/// `T` only documents what the signal holds, signals are [`Signal<A>`] at runtime.
#[derive(Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Signal<T = A>(NodeId, PhantomData<fn() -> T>);

/// A signal taken by a function, documented as a [`Signal<T>`] and converted as the [`Signal`] it is at runtime,
/// i.e. `SignalOf<Vec<A>>` for a signal of a list.
pub struct SignalOf<T>(Signal, PhantomData<fn() -> T>);

impl<T> FromScript for SignalOf<T> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        V::<Signal>::from_script(value, scope).map(|s| SignalOf(*s, PhantomData))
    }

    fn could_convert(value: &ScriptValue) -> bool {
        V::<Signal>::could_convert(value)
    }
}

impl<T> ArgMeta for SignalOf<T> {}

impl<T: TypePath + Send + Sync> TypedThrough for SignalOf<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Val(Signal::<T>::type_info())
    }
}

// reflection doesn't see the parameter of an opaque type, registered it describes `Signal<Vec<A>>` as a signal of a list
impl<T: GetTypeRegistration> GetTypeDependencies for SignalOf<T> {
    type Underlying = Signal;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        registry.register::<Signal>();
        registry.register::<T>();
    }
}

impl<T> Signal<T> {
    fn new(id: NodeId) -> Self {
        Signal(id, PhantomData)
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Signal<T> {}

/// Tells whether a key is the value of a signal of a `T`, made by `selector`.
///
/// `selector:selected(key)` only re-runs its reader when `key` becomes or stops being selected,
/// so a list of items each checking whether they are selected only re-runs two of them when the selection changes.
#[derive(Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Selector<T = A>(NodeId, PhantomData<fn() -> T>);

impl<T> Selector<T> {
    fn new(id: NodeId) -> Self {
        Selector(id, PhantomData)
    }
}

impl<T> Clone for Selector<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Selector<T> {}

/// The systems updating reactive UI trees, in [`PostUpdate`] before the UI layout.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReactiveUiSystems;

/// Adds the `reactive_ui` script namespace and the systems keeping mounted trees up to date.
#[derive(Default)]
pub struct ReactiveUiPlugin;

impl Plugin for ReactiveUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReactiveUi>()
            .register_type::<UiElement>()
            .register_type::<Signal>()
            .register_type::<Selector>()
            .register_type::<UiValue>()
            .register_type::<element::UiContext>()
            .register_type::<expr::ItemWatch>()
            .register_generic_type::<Signal<A>>(
                "reactive_ui",
                &["T"],
                "A signal of a `T`: read with `get` (tracked) or `peek` (untracked), set with `set`",
            )
            .register_generic_type::<Selector<A>>(
                "reactive_ui",
                &["T"],
                "Tells whether a `T` is the value of a signal, see `selector`",
            )
            .register_generic_type::<element::UiContext<A>>(
                "reactive_ui",
                &["T"],
                "A context of a `T`, see `create_context`",
            )
            .add_systems(
                PostUpdate,
                (forget_detached_scripts, flush_reactive_ui)
                    .chain()
                    .in_set(ReactiveUiSystems)
                    .before(UiSystems::Layout),
            );
        events::add_observers(app);
        app.world_mut()
            .get_resource_or_init::<AppScriptCallbackRunners>()
            .add_unload_hook(Arc::new(dispose_unloading_script));
        register_ui_functions(app.world_mut());
        register_selector_functions(app.world_mut());
        register_signal_functions(app.world_mut());
        expr::register_value_functions(app.world_mut());
    }

    fn finish(&self, app: &mut App) {
        let registry = app.world().resource::<AppTypeRegistry>().clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_mod_scripting_reactive_ui"],
            "reactive_ui",
        );
    }
}

fn current_attachment(ctx: &FunctionCallContext) -> Result<ScriptAttachment, InteropError> {
    ctx.world()?
        .current_attachment()
        .0
        .ok_or_else(|| InteropError::str("reactive ui functions can only be used from a script"))
}

/// Defines a component, returning a function which makes an element of it: called with a props table,
/// or with the arguments of its render function, which renders the component with what it was called with.
fn define(
    ctx: FunctionCallContext,
    name: String,
    render: ValueOf<A, Func>,
) -> Result<ValueOf<A, Func>, InteropError> {
    let attachment = current_attachment(&ctx)?;
    let runtime = runtime(&ctx)?;
    let mut rt = runtime.lock();
    let def = Def {
        render: render.into_inner(),
        #[cfg(feature = "testing")]
        name: name.clone(),
    };
    let id = if let Some(existing) = rt
        .def_index
        .get(&(attachment.clone(), name.clone()))
        .copied()
    {
        // redefined, i.e. on reload: re-render every instance with the new code
        rt.defs[existing.0 as usize] = def;
        let instances = rt
            .nodes
            .iter()
            .filter(|(_, n)| matches!(&n.kind, NodeKind::Instance(i) if i.def == existing))
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        for instance in instances {
            rt.mark_dirty(instance);
        }
        existing
    } else {
        let id = ComponentDef(rt.defs.len() as u32);
        rt.defs.push(def);
        rt.def_index.insert((attachment, name), id);
        id
    };
    Ok(ValueOf::new(Func::from(
        move |ctx: FunctionCallContext, args: VecDeque<ScriptValue>| {
            component_element(&ctx, id, args.into()).unwrap_or_else(ScriptValue::Error)
        },
    )))
}

/// An element of the component, called with `args`. A props table can hold the `key` of the element.
fn component_element(
    ctx: &FunctionCallContext,
    def: ComponentDef,
    args: Vec<ScriptValue>,
) -> Result<ScriptValue, InteropError> {
    let key = match args.as_slice() {
        [ScriptValue::Map(map)] => map
            .get("key")
            .map(|key| element::key_of(key, &ctx.world()?))
            .transpose()?,
        _ => None,
    };
    Ok(new_element(
        &*runtime(ctx)?,
        ElementData {
            kind: ElementKind::Component(def, args),
            key,
            props: Vec::new(),
            children: Vec::new(),
        },
    ))
}

/// A region element, `props` are the properties of the node holding its children (and its `key`).
fn region(
    ctx: &FunctionCallContext,
    kind: ElementKind,
    props: Option<Lazy<UiProps>>,
) -> Result<Element, InteropError> {
    let (key, props) = parse_props(props.map_or(ScriptValue::Unit, |p| p.0), &ctx.world()?)?;
    Ok(Element::new(new_element(
        &*runtime(ctx)?,
        ElementData {
            kind,
            key,
            props,
            children: Vec::new(),
        },
    )))
}

/// Builds and mounts UI: `import_namespace("reactive_ui")`.
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "ui_functions",
    namespace = "reactive_ui"
)]
impl NamespaceMembers {
    /// A container, one `bevy_ui` entity with its children. Layout properties are the fields of `bevy_ui`'s `Node`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "row", flex_direction = "Row", column_gap = 8 }, { ui.text("a"), ui.text("b") })
    /// )
    /// assert(#s:children(s:get_by_name("row")) == 2)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "row", flex_direction: "Row", column_gap: 8 }, [ui.text("a"), ui.text("b")])
    /// );
    /// assert!(s.children(s.get_by_name("row")).len() == 2);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties.
    /// * `children`: Its children.
    /// Returns:
    /// * `element`: The element.
    fn node(
        ctx: FunctionCallContext,
        props: Option<Lazy<UiProps>>,
        children: Option<props::Children>,
    ) -> Result<Element, InteropError> {
        let world = ctx.world()?;
        let runtime = runtime(&ctx)?;
        let (key, props) = parse_props(props.map_or(ScriptValue::Unit, |p| p.0), &world)?;
        let children = match children {
            Some(children) => elements_from_script(&children.0, &runtime)?,
            None => Vec::new(),
        };
        Ok(Element::new(new_element(
            &runtime,
            ElementData {
                kind: ElementKind::Node,
                key,
                props,
                children,
            },
        )))
    }

    /// Text, `text("hello")` is `text({ text = "hello" })`. Its `text` can be bound to a function or a value
    /// computed in Rust, to show what changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "title", text = "Inventory", font_size = 20 }))
    /// s:expect_style(s:get_by_name("title"), { text = "Inventory", font_size = 20 })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "title", text: "Inventory", font_size: 20 }));
    /// s.expect_style(s.get_by_name("title"), #{ text: "Inventory", font_size: 20 });
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties, or its text.
    /// Returns:
    /// * `element`: The element.
    fn text(
        ctx: FunctionCallContext,
        props: Lazy<Union<String, UiProps>>,
    ) -> Result<Element, InteropError> {
        let (key, props) = parse_props(props.0, &ctx.world()?)?;
        Ok(Element::new(new_element(
            &*runtime(&ctx)?,
            ElementData {
                kind: ElementKind::Text,
                key,
                props,
                children: Vec::new(),
            },
        )))
    }

    /// A node showing the image asset at the path given as its `image`.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.image({ name = "icon", image = "icons/sword.png", width = 16 }))
    /// s:expect_style(s:get_by_name("icon"), { image = "icons/sword.png", width = 16 })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.image(#{ name: "icon", image: "icons/sword.png", width: 16 }));
    /// s.expect_style(s.get_by_name("icon"), #{ image: "icons/sword.png", width: 16 });
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties.
    /// Returns:
    /// * `element`: The element.
    fn image(ctx: FunctionCallContext, props: Lazy<UiProps>) -> Result<Element, InteropError> {
        let (key, props) = parse_props(props.0, &ctx.world()?)?;
        Ok(Element::new(new_element(
            &*runtime(&ctx)?,
            ElementData {
                kind: ElementKind::Node,
                key,
                props,
                children: Vec::new(),
            },
        )))
    }

    /// Defines a component, returning a function of the same type which makes elements of it: called with a props
    /// table, or with any arguments, which the component renders with. Defining the name again (a reload) swaps
    /// the code and re-renders its instances.
    ///
    /// The function runs **once** per instance: state goes in signals, and what changes is bound to functions.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// ---@param props { label: string }
    /// local Badge = ui.component("Badge", function(props)
    ///     return ui.text({ name = "badge", text = "[" .. props.label .. "]" })
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(Badge({ label = "new" }))
    /// assert(s:text(s:get_by_name("badge")) == "[new]")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn badge(props) {
    ///     import_namespace("reactive_ui").text(#{ name: "badge", text: `[${props.label}]` })
    /// }
    ///
    /// pub fn main() {
    ///     // components take any number of arguments, Rune calls them with `call`
    ///     let Badge = import_namespace("reactive_ui").component("Badge", badge);
    ///     let s = import_namespace("reactive_ui_test").render(Badge.call(#{ label: "new" }));
    ///     assert!(s.text(s.get_by_name("badge")) == "[new]");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `name`: The name of the component, unique in the script.
    /// * `render`: The function rendering an instance, run once per instance.
    /// Returns:
    /// * `component`: The function making elements of the component.
    fn component(
        ctx: FunctionCallContext,
        name: String,
        render: ValueOf<A, Func>,
    ) -> Result<ValueOf<A, Func>, InteropError> {
        define(ctx, name, render)
    }

    /// Shows a component under a new root entity filling the window. Called again with the key, it updates it.
    /// Call it from `on_script_loaded`. It is mounted in the next UI update, before `bevy_ui` lays out.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local Hello = ui.component("Hello", function() return ui.text({ name = "hello", text = "hello" }) end)
    /// local root = ui.render("hello", Hello())
    /// -- tests see it at once
    /// local s = import_namespace("reactive_ui_test").screen(root)
    /// assert(s:query_by_name("hello") ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn hello() {
    ///     import_namespace("reactive_ui").text(#{ name: "hello", text: "hello" })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let root = ui.render("hello", ui.component("Hello", hello).call());
    ///     // tests see it at once
    ///     let s = import_namespace("reactive_ui_test").screen(root);
    ///     assert!(s.query_by_name("hello") != ());
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `key`: Identifies the root among the script's roots.
    /// * `element`: An element of a component, i.e. `Counter({ start = 1 })`.
    /// Returns:
    /// * `root`: The root entity.
    fn render(
        ctx: FunctionCallContext,
        key: String,
        element: Element,
    ) -> Result<V<Entity>, InteropError> {
        let attachment = current_attachment(&ctx)?;
        let runtime = runtime(&ctx)?;
        let element = element_from_script(&element.0, &runtime)?;
        let Some(ElementKind::Component(def, props)) = element.map(|e| e.0.kind.clone()) else {
            return Err(InteropError::str(
                "`render` renders a component, i.e. `render(\"key\", Counter({ start = 1 }))`",
            ));
        };
        // the root renders in a later update, when the elements of this one are gone
        if props
            .iter()
            .any(|p| runtime.lock().elements.holds_element(p))
        {
            return Err(InteropError::str(
                "`render` can't be given elements in its props, they are only valid in the update which built them: \
                 build them inside the component, or pass a component to call",
            ));
        }
        let existing = runtime
            .lock()
            .roots
            .get(&(attachment.clone(), key.clone()))
            .map(|r| (r.container, r.instance));
        if let Some((container, instance)) = existing {
            let same = runtime
                .lock()
                .instance_mut(instance)
                .is_some_and(|i| i.def == def);
            if same {
                let mut rt = runtime.lock();
                if let Some(i) = rt.instance_mut(instance) {
                    i.props = props;
                }
                rt.mark_dirty(instance);
                return Ok(V(container));
            }
            // another component: the old one is removed with its state (running its cleanups), the new one starts fresh
            let cx = Cx {
                runtime: &runtime,
                world: ctx.world()?,
                attachment: &attachment,
                language: ctx.language(),
            };
            cx.dispose_instance(instance, true)?;
            let mut rt = runtime.lock();
            let instance = rt.create(
                NodeKind::Instance(Instance {
                    def,
                    props,
                    slots: Vec::new(),
                    rendered: None,
                    parent: container,
                    contexts: None,
                }),
                attachment.clone(),
            );
            rt.mark_dirty(instance);
            rt.roots.insert(
                (attachment, key),
                Root {
                    container,
                    instance,
                },
            );
            return Ok(V(container));
        }
        let root = UiRoot {
            attachment: attachment.clone(),
            key: key.clone(),
        };
        let container = ctx.world()?.with_world_mut_access(|w| {
            // fills the window, so sizes in percent in the script are of the window (percents
            // on a root node are of its content, see bevy_ui's implicit viewport node). Clicks
            // which hit no element go through
            let node = Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..Default::default()
            };
            w.spawn((node, Pickable::IGNORE, root)).id()
        })?;
        let mut rt = runtime.lock();
        let instance = rt.create(
            NodeKind::Instance(Instance {
                def,
                props,
                slots: Vec::new(),
                rendered: None,
                parent: container,
                contexts: None,
            }),
            attachment.clone(),
        );
        rt.mark_dirty(instance);
        rt.roots.insert(
            (attachment, key),
            Root {
                container,
                instance,
            },
        );
        Ok(V(container))
    }

    /// The element with a key, which identifies it among its siblings, i.e. a component called with its arguments.
    /// When a region runs again, children are kept or replaced by key, keeping the state of those kept.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local Slot = ui.component("Slot", function(name) return ui.text({ name = name, text = name }) end)
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, { ui.keyed("sword", Slot("sword")) }))
    /// assert(s:query_by_name("sword") ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn slot(name) {
    ///     import_namespace("reactive_ui").text(#{ name: name, text: name })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let Slot = ui.component("Slot", slot);
    ///     let s = import_namespace("reactive_ui_test").render(ui.node(#{}, [ui.keyed("sword", Slot.call("sword"))]));
    ///     assert!(s.query_by_name("sword") != ());
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `key`: The key.
    /// * `element`: The element.
    /// Returns:
    /// * `element`: The element with the key.
    fn keyed(
        ctx: FunctionCallContext,
        key: Lazy<props::Key>,
        element: Element,
    ) -> Result<Element, InteropError> {
        let runtime = runtime(&ctx)?;
        let Some(element) = element_from_script(&element.0, &runtime)? else {
            return Err(InteropError::str("`keyed` takes an element"));
        };
        let mut data = (*element.0).clone();
        data.key = Some(element::key_of(&key.0, &ctx.world()?)?);
        Ok(Element::new(new_element(&runtime, data)))
    }

    /// State: `s:get()` reads it and records the read, `s:peek()` reads it without recording, `s:set(value)` changes it.
    /// What read it runs again when it changes. In a component it belongs to the instance, outside of one it is
    /// plain script state any component can read.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local count = ui.signal(0)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.text({ name = "n", text = function() return "n=" .. count:get() end })
    /// )
    /// s:act(function() count:set(5) end)
    /// assert(s:text(s:get_by_name("n")) == "n=5")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let count = ui.signal(0);
    /// // Rune closures can't capture signals: functions are given them as dependencies, `[f, a, b]` calls `f(a, b)`
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "n", text: [|count| `n=${count.get()}`, count] }));
    /// s.act([|count| count.set(5), count]);
    /// assert!(s.text(s.get_by_name("n")) == "n=5");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `initial`: Its value, when the component first renders.
    /// Returns:
    /// * `signal`: The signal.
    fn signal(
        ctx: FunctionCallContext,
        initial: ValueOf<A, ScriptValue>,
    ) -> Result<V<Signal<A>>, InteropError> {
        let attachment = current_attachment(&ctx)?;
        let initial = initial.into_inner();
        let id = slot_node(
            &*runtime(&ctx)?,
            attachment,
            "signal",
            |k| matches!(k, NodeKind::Signal { .. }),
            move || NodeKind::Signal {
                value: initial,
                key: None,
            },
        )?;
        Ok(V(Signal::<A>::new(id)))
    }

    /// A value derived from others, recomputed when what `f` read changed. Its readers only update when the value did.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local items = ui.signal({ "sword", "shield" })
    /// local count = ui.computed(function() return #items:get() end)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.text({ name = "count", text = function() return count:get() .. " items" end })
    /// )
    /// s:act(function() items:set({ "sword" }) end)
    /// assert(s:text(s:get_by_name("count")) == "1 items")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let items = ui.signal(["sword", "shield"]);
    /// let count = ui.computed([|items| items.get().len(), items]);
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "count", text: [|count| `${count.get()} items`, count] }));
    /// s.act([|items| items.set(["sword"]), items]);
    /// assert!(s.text(s.get_by_name("count")) == "1 items");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `f`: Computes the value.
    /// Returns:
    /// * `signal`: The signal of the value.
    fn computed(
        ctx: FunctionCallContext,
        f: WithDeps<fn() -> ValueOf<A, ScriptValue>>,
    ) -> Result<V<Signal<A>>, InteropError> {
        let f = f.into_inner();
        let attachment = current_attachment(&ctx)?;
        let new_f = f.clone();
        let contexts = current_contexts();
        let new_contexts = contexts.clone();
        let id = slot_node(
            &*runtime(&ctx)?,
            attachment,
            "computed",
            move |k| match k {
                NodeKind::Computed { f, value, contexts } => {
                    *f = new_f;
                    *value = None;
                    *contexts = new_contexts;
                    true
                }
                _ => false,
            },
            move || NodeKind::Computed {
                f,
                value: None,
                contexts,
            },
        )?;
        Ok(V(Signal::<A>::new(id)))
    }

    /// Runs `f` once the UI update it was made in has mounted the tree, then whenever what it read changes. `f` can
    /// return a function to clean up before the next run and when its component is removed or its script unloaded.
    /// For what isn't a property: writing to the world, logging, starting sounds.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local selected = ui.signal("sword")
    /// ---@type string[]
    /// local log = {}
    /// local Logger = ui.component("Logger", function()
    ///     ui.effect(function() log[#log + 1] = "selected " .. selected:get() end)
    ///     return ui.node()
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(Logger())
    /// s:act(function() selected:set("shield") end)
    /// assert(#log == 2 and log[2] == "selected shield")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn logger(selected, runs) {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.effect([|selected, runs| {
    ///         selected.get();
    ///         runs.set(runs.peek() + 1);
    ///     }, selected, runs]);
    ///     ui.node(#{}, [])
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let selected = ui.signal("sword");
    ///     let runs = ui.signal(0);
    ///     let s = import_namespace("reactive_ui_test").render(ui.component("Logger", logger).call(selected, runs));
    ///     s.act([|selected| selected.set("shield"), selected]);
    ///     assert!(runs.peek() == 2);
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `f`: The effect.
    fn effect(ctx: FunctionCallContext, f: WithDeps<fn()>) -> Result<(), InteropError> {
        let f = f.into_inner();
        let attachment = current_attachment(&ctx)?;
        let runtime = runtime(&ctx)?;
        let new_f = f.clone();
        let contexts = current_contexts();
        let new_contexts = contexts.clone();
        let mut created = false;
        let id = slot_node(
            &runtime,
            attachment,
            "effect",
            move |k| match k {
                NodeKind::Effect { f, contexts, .. } => {
                    *f = new_f;
                    *contexts = new_contexts;
                    true
                }
                _ => false,
            },
            || {
                created = true;
                NodeKind::Effect {
                    f,
                    cleanup: None,
                    contexts,
                }
            },
        )?;
        if created {
            runtime.lock().mark_dirty(id);
        }
        Ok(())
    }

    /// A signal over a component of an entity, updated when the component changes, whoever changed it.
    /// `get` returns the component, or nil when it is gone. Components are checked by their change tick once per
    /// update, so Rust systems changing it update the UI too.
    ///
    /// ```multi-lang-example,lua
    /// local bms = import_namespace("bms")
    /// local ui = import_namespace("reactive_ui")
    /// ---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
    /// local Health = bms.world.register_new_component("script.docs", "Health", true)
    /// local player = bms.world.spawn()
    /// bms.world.insert_component(player, Health, bms.DynamicComponent.new({ data = 100 }))
    ///
    /// local Bar = ui.component("Bar", function()
    ///     local health = ui.watch(player, Health)
    ///     return ui.text({ name = "hp", text = ui.format("hp: {}", { ui.path(health, "data") }) })
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(Bar())
    /// s:act(function() assert(bms.world.get_component(player, Health)).data = 50 end)
    /// assert(s:text(s:get_by_name("hp")) == "hp: 50")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn bar(player) {
    ///     let ui = import_namespace("reactive_ui");
    ///     let health = ui.watch(player, state()["Health"]);
    ///     ui.text(#{ name: "hp", text: ui.format("hp: {}", [ui.path(health, "data")]) })
    /// }
    ///
    /// pub fn main() {
    ///     let bms = import_namespace("bms");
    ///     let ui = import_namespace("reactive_ui");
    ///     let Health = bms.world.register_new_component("script.docs", "Health", true);
    ///     state()["Health"] = Health;
    ///     let player = bms.world.spawn();
    ///     bms.world.insert_component(player, Health, bms.DynamicComponent.new(#{ data: 100 }));
    ///
    ///     let s = import_namespace("reactive_ui_test").render(ui.component("Bar", bar).call(player));
    ///     s.act([|player| {
    ///         let bms = import_namespace("bms");
    ///         bms.world.get_component(player, state()["Health"]).data = 50;
    ///     }, player]);
    ///     assert!(s.text(s.get_by_name("hp")) == "hp: 50");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `entity`: The entity.
    /// * `component`: The component type.
    /// Returns:
    /// * `signal`: The signal of the component.
    fn watch(
        ctx: FunctionCallContext,
        entity: V<Entity>,
        component: TypeOf<V<ScriptComponentRegistration>, A>,
    ) -> Result<V<Signal<A>>, InteropError> {
        let attachment = current_attachment(&ctx)?;
        let entity = *entity;
        let component_id = component.component_id();
        let type_id = component.type_registration().type_id();
        let now = ctx.world()?.with_world_access(|w| {
            w.get_entity(entity)
                .ok()
                .and_then(|e| e.get_change_ticks_by_id(component_id))
                .map(|t| t.changed)
        })?;
        let id = slot_node(
            &*runtime(&ctx)?,
            attachment,
            "watch",
            |k| matches!(k, NodeKind::Watch { entity: e, component: c, .. } if *e == entity && *c == component_id),
            move || NodeKind::Watch {
                entity,
                component: component_id,
                type_id,
                last_changed: now,
            },
        )?;
        Ok(V(Signal::<A>::new(id)))
    }

    /// A value provided to a subtree with `provide` and read with `use_context`, without passing it through the
    /// props of every component in between. Create it once, outside of components.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local Theme = ui.create_context({ accent = "#ffcc00" })
    /// local Title = ui.component("Title", function()
    ///     return ui.text({ name = "title", text = "hi", color = ui.use_context(Theme).accent })
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(ui.provide(Theme, { accent = "#00ccff" }, { Title() }))
    /// s:expect_style(s:get_by_name("title"), { color = "#00ccff" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn title() {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: "title", text: "hi", color: ui.use_context(state()["Theme"]).accent })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     state()["Theme"] = ui.create_context(#{ accent: "#ffcc00" });
    ///     let Title = ui.component("Title", title);
    ///     let s = import_namespace("reactive_ui_test").render(ui.provide(state()["Theme"], #{ accent: "#00ccff" }, [Title.call()]));
    ///     s.expect_style(s.get_by_name("title"), #{ color: "#00ccff" });
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `default`: The value where none is provided.
    /// Returns:
    /// * `context`: The context.
    fn create_context(
        ctx: FunctionCallContext,
        default: ValueOf<A, ScriptValue>,
    ) -> Result<V<element::UiContext<A>>, InteropError> {
        let runtime = runtime(&ctx)?;
        let mut rt = runtime.lock();
        rt.contexts.push(default.into_inner());
        Ok(V(element::UiContext::<A>::new(
            rt.contexts.len() as u32 - 1,
        )))
    }

    /// The value of a context provided nearest above the component rendering, or its default. Read it when the
    /// component renders, and provide a signal for a value which changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local Labels = ui.create_context({ greeting = "hello" })
    /// local Greeting = ui.component("Greeting", function()
    ///     return ui.text({ name = "greeting", text = ui.use_context(Labels).greeting })
    /// end)
    /// -- nothing provided: the default
    /// local s = import_namespace("reactive_ui_test").render(Greeting())
    /// assert(s:text(s:get_by_name("greeting")) == "hello")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn greeting() {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: "greeting", text: ui.use_context(state()["Labels"]).greeting })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     state()["Labels"] = ui.create_context(#{ greeting: "hello" });
    ///     // nothing provided: the default
    ///     let s = import_namespace("reactive_ui_test").render(ui.component("Greeting", greeting).call());
    ///     assert!(s.text(s.get_by_name("greeting")) == "hello");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `context`: The context.
    /// Returns:
    /// * `value`: The value.
    fn use_context(
        ctx: FunctionCallContext,
        context: V<element::UiContext<A>>,
    ) -> Result<ValueOf<A, ScriptValue>, InteropError> {
        if let Some(value) = runtime::provided(context.0) {
            return Ok(ValueOf::new(value));
        }
        let runtime = runtime(&ctx)?;
        let default = runtime.lock().contexts.get(context.0.0 as usize).cloned();
        default
            .map(ValueOf::new)
            .ok_or_else(|| InteropError::str("unknown context"))
    }

    /// A node providing the value of a context to what is below it, read with `use_context`. Regions mounted later
    /// below it see it too.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local User = ui.create_context("guest")
    /// local Name = ui.component("Name", function()
    ///     return ui.text({ name = "user", text = ui.use_context(User) })
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(ui.provide(User, "ada", { Name() }))
    /// assert(s:text(s:get_by_name("user")) == "ada")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn name() {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: "user", text: ui.use_context(state()["User"]) })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     state()["User"] = ui.create_context("guest");
    ///     let Name = ui.component("Name", name);
    ///     let s = import_namespace("reactive_ui_test").render(ui.provide(state()["User"], "ada", [Name.call()]));
    ///     assert!(s.text(s.get_by_name("user")) == "ada");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `context`: The context.
    /// * `value`: The value provided.
    /// * `children`: Its children.
    /// Returns:
    /// * `element`: The element.
    fn provide(
        ctx: FunctionCallContext,
        context: V<element::UiContext<A>>,
        value: ValueOf<A, ScriptValue>,
        children: Option<props::Children>,
    ) -> Result<Element, InteropError> {
        let runtime = runtime(&ctx)?;
        let children = match children {
            Some(children) => elements_from_script(&children.0, &runtime)?,
            None => Vec::new(),
        };
        Ok(Element::new(new_element(
            &runtime,
            ElementData {
                kind: ElementKind::Provide(context.0, value.into_inner()),
                key: None,
                props: Vec::new(),
                children,
            },
        )))
    }

    /// For "which one is selected": `selector:selected(key)` only wakes the readers of the key which was and the key
    /// which is the signal's value. A list of a hundred rows highlighting the selected one updates two rows, not all.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// ---@type reactive_ui.Signal<string>
    /// local selected = ui.signal("a")
    /// local is_selected = ui.selector(selected)
    /// local rows = {}
    /// for _, key in ipairs({ "a", "b", "c" }) do
    ///     rows[#rows + 1] = ui.text({ name = key, text = function() return is_selected:selected(key) and "*" or "" end })
    /// end
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, rows))
    /// s:updates()
    /// s:act(function() selected:set("b") end)
    /// assert(s:text(s:get_by_name("b")) == "*" and s:updates().bindings == 2, "only a and b ran again")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let selected = ui.signal("a");
    /// let is_selected = ui.selector(selected);
    /// let rows = [];
    /// for key in ["a", "b", "c"] {
    ///     rows.push(ui.text(#{ name: key, text: ui.when(ui.selected(is_selected, key), "*", "") }));
    /// }
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{}, rows));
    /// s.updates();
    /// s.act([|selected| selected.set("b"), selected]);
    /// assert!(s.text(s.get_by_name("b")) == "*" && s.updates().bindings == 2, "only a and b ran again");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `source`: The signal of the selected key.
    /// Returns:
    /// * `selector`: The selector.
    fn selector(
        ctx: FunctionCallContext,
        source: V<Signal<A>>,
    ) -> Result<V<Selector<A>>, InteropError> {
        let attachment = current_attachment(&ctx)?;
        runtime::selector(
            &*runtime(&ctx)?,
            &ctx.world()?,
            attachment,
            source.into_inner().0,
        )
        .map(|id| V(Selector::<A>::new(id)))
    }

    /// One child per item of the list in the signal, made by `f(item)`. Children are kept by item, `f` only runs for
    /// items which were not in the list before. The last argument is the layout of the node holding them.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local items = ui.signal({ "sword", "shield" })
    /// local s = import_namespace("reactive_ui_test").render(ui.list(items, function(item)
    ///     return ui.text({ name = item, text = item })
    /// end, { flex_direction = "Column" }))
    /// s:act(function() items:set({ "shield", "potion" }) end)
    /// assert(s:query_by_name("sword") == nil and s:query_by_name("potion") ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let items = ui.signal(["sword", "shield"]);
    /// let s = import_namespace("reactive_ui_test").render(ui.list(items, |item| {
    ///     import_namespace("reactive_ui").text(#{ name: item, text: item })
    /// }, #{ flex_direction: "Column" }));
    /// s.act([|items| items.set(["shield", "potion"]), items]);
    /// assert!(s.query_by_name("sword") == () && s.query_by_name("potion") != ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `items`: The signal of the list.
    /// * `f`: Makes the child of an item.
    /// * `props`: The properties of the node holding the children, i.e. its layout.
    /// Returns:
    /// * `element`: The element.
    fn list(
        ctx: FunctionCallContext,
        items: SignalOf<Vec<A>>,
        f: WithDeps<fn(ValueOf<A, ScriptValue>) -> Element>,
        props: Option<Lazy<UiProps>>,
    ) -> Result<Element, InteropError> {
        let kind = ElementKind::Each(items.0.0, f.into_inner(), false);
        region(&ctx, kind, props)
    }

    /// As `list`, but `f` runs once with a value standing for each item, and the rows are built from what it returned
    /// without running script code. Properties are plain values or computed in Rust, with `item` wherever a value is
    /// taken. Handlers get the row's item first. For long, uniform lists.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local items = ui.signal({ "sword", "shield" })
    /// local s = import_namespace("reactive_ui_test").render(ui.list_template(items, function(item)
    ///     return ui.text({ name = ui.format("row_{}", { item }), text = ui.format("- {}", { item }) })
    /// end))
    /// assert(s:text(s:get_by_name("row_shield")) == "- shield")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let items = ui.signal(["sword", "shield"]);
    /// let s = import_namespace("reactive_ui_test").render(ui.list_template(items, |item| {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: ui.format("row_{}", [item]), text: ui.format("- {}", [item]) })
    /// }));
    /// assert!(s.text(s.get_by_name("row_shield")) == "- shield");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `items`: The signal of the list.
    /// * `f`: Makes the row template.
    /// * `props`: The properties of the node holding the rows, i.e. its layout.
    /// Returns:
    /// * `element`: The element.
    fn list_template(
        ctx: FunctionCallContext,
        items: SignalOf<Vec<A>>,
        f: WithDeps<fn(Lazy<V<UiValue>>) -> Element>,
        props: Option<Lazy<UiProps>>,
    ) -> Result<Element, InteropError> {
        let kind = ElementKind::Each(items.0.0, f.into_inner(), true);
        region(&ctx, kind, props)
    }

    /// A region showing what `f` returns, an element or nothing. Re-runs when what `f` read changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local open = ui.signal(false)
    /// local s = import_namespace("reactive_ui_test").render(ui.show(function()
    ///     if open:get() then return ui.text({ name = "menu", text = "menu" }) end
    /// end))
    /// assert(s:query_by_name("menu") == nil)
    /// s:act(function() open:set(true) end)
    /// assert(s:query_by_name("menu") ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let open = ui.signal(false);
    /// let s = import_namespace("reactive_ui_test").render(ui.show([|open| {
    ///     if open.get() { import_namespace("reactive_ui").text(#{ name: "menu", text: "menu" }) } else { () }
    /// }, open]));
    /// assert!(s.query_by_name("menu") == ());
    /// s.act([|open| open.set(true), open]);
    /// assert!(s.query_by_name("menu") != ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `f`: Makes the content.
    /// * `props`: The properties of the node holding the content.
    /// Returns:
    /// * `element`: The element.
    fn show(
        ctx: FunctionCallContext,
        f: WithDeps<fn() -> Option<Element>>,
        props: Option<Lazy<UiProps>>,
    ) -> Result<Element, InteropError> {
        region(&ctx, ElementKind::Dynamic(f.into_inner()), props)
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "selector_functions"
)]
impl Selector {
    /// Whether `key` is the value of the signal. Only wakes its reader when that changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local selected = ui.signal("a")
    /// local is_selected = ui.selector(selected)
    /// assert(is_selected:selected("a") and not is_selected:selected("b"))
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let is_selected = ui.selector(ui.signal("a"));
    /// assert!(is_selected.selected("a") && !is_selected.selected("b"));
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `selector`: The selector.
    /// * `key`: The key.
    /// Returns:
    /// * `selected`: Whether it is selected.
    fn selected(
        ctx: FunctionCallContext,
        selector: V<Selector<A>>,
        key: ValueOf<A, ScriptValue>,
    ) -> Result<bool, InteropError> {
        runtime::is_selected(
            &*runtime(&ctx)?,
            &ctx.world()?,
            selector.into_inner().0,
            &key,
        )
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "signal_functions"
)]
impl Signal {
    /// The value, recording the read: what is reading it (a bound property, a computed, an effect, a region) re-runs
    /// when it changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local name = ui.signal("sword")
    /// local upper = ui.computed(function() return string.upper(name:get()) end)
    /// name:set("shield")
    /// assert(upper:get() == "SHIELD")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let name = ui.signal("sword");
    /// let upper = ui.computed([|name| name.get().to_uppercase(), name]);
    /// name.set("shield");
    /// assert!(upper.get() == "SHIELD");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `signal`: The signal.
    /// Returns:
    /// * `value`: The value.
    fn get(
        ctx: FunctionCallContext,
        signal: V<Signal<A>>,
    ) -> Result<ValueOf<A, ScriptValue>, InteropError> {
        read(&*runtime(&ctx)?, &ctx, signal.into_inner().0, true).map(ValueOf::new)
    }

    /// The value, without recording the read: for values used once, i.e. in a handler updating a counter.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local clicks = ui.signal(0)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "button", on_click = function() clicks:set(clicks:peek() + 1) end })
    /// )
    /// s:click(s:get_by_name("button"))
    /// assert(clicks:peek() == 1)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let clicks = ui.signal(0);
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "button", on_click: [|clicks, _event| clicks.set(clicks.peek() + 1), clicks] }, [])
    /// );
    /// s.click(s.get_by_name("button"));
    /// assert!(clicks.peek() == 1);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `signal`: The signal.
    /// Returns:
    /// * `value`: The value.
    fn peek(
        ctx: FunctionCallContext,
        signal: V<Signal<A>>,
    ) -> Result<ValueOf<A, ScriptValue>, InteropError> {
        read(&*runtime(&ctx)?, &ctx, signal.into_inner().0, false).map(ValueOf::new)
    }

    /// Changes the value. What read it runs again in the next UI update, once however many times it was set, and not
    /// at all when the value is the same.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hp = ui.signal(100)
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "hp", text = function() return "hp " .. hp:get() end }))
    /// s:act(function() hp:set(80) end)
    /// assert(s:text(s:get_by_name("hp")) == "hp 80")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let hp = ui.signal(100);
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "hp", text: ui.format("hp {}", [ui.path(hp, "")]) }));
    /// s.act([|hp| hp.set(80), hp]);
    /// assert!(s.text(s.get_by_name("hp")) == "hp 80");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `signal`: The signal.
    /// * `value`: The new value.
    fn set(
        ctx: FunctionCallContext,
        signal: V<Signal<A>>,
        value: ValueOf<A, ScriptValue>,
    ) -> Result<(), InteropError> {
        write(
            &*runtime(&ctx)?,
            &ctx.world()?,
            signal.into_inner().0,
            value.into_inner(),
        )
    }
}

/// Removes the component instances a script rendered while it is being unloaded, in its context, so the cleanups of
/// their effects run. What is left (roots, script state) is forgotten by `forget_detached_scripts`.
fn dispose_unloading_script(
    world: WorldGuard,
    attachment: &ScriptAttachment,
    language: Language,
) -> Result<(), InteropError> {
    let Some(runtime) =
        world.with_world_access(|w| w.get_resource::<ReactiveUi>().map(|r| r.0.clone()))?
    else {
        return Ok(());
    };
    let _current = CurrentRuntime::set(runtime.clone());
    let instances = runtime
        .lock()
        .roots
        .iter()
        .filter(|((a, _), _)| a == attachment)
        .map(|(_, root)| root.instance)
        .collect::<Vec<_>>();
    let cx = Cx {
        runtime: &runtime,
        world,
        attachment,
        language,
    };
    let mut first_error = None;
    for instance in instances {
        if let Err(e) = cx.dispose_instance(instance, true) {
            first_error.get_or_insert(e);
        }
    }
    first_error.map_or(Ok(()), Err)
}

/// Forgets the trees and state of scripts which were unloaded.
fn forget_detached_scripts(
    mut messages: MessageReader<ScriptLifecycleMessage>,
    ui: Res<ReactiveUi>,
    mut commands: Commands,
) {
    for message in messages.read() {
        if let ScriptLifecycleMessage::Detached(attachment) = message {
            for container in ui.0.lock().forget_attachment(attachment) {
                commands.entity(container).try_despawn();
            }
        }
    }
}

/// Runs queued event handlers, re-runs dirty components, bindings and regions, then dirty effects.
///
/// Work is grouped by script so each script's context is entered once per pass. Work made dirty by a pass
/// (i.e. by handlers or effects setting signals) runs in another pass, up to [`MAX_FLUSH_ITERATIONS`].
pub fn flush_reactive_ui(world: &mut World) {
    let Some(runtime) = world.get_resource::<ReactiveUi>().map(|r| r.0.clone()) else {
        return;
    };
    {
        let mut rt = runtime.lock();
        rt.poll_watches(world);
        if rt.events.is_empty() && rt.dirty.is_empty() && rt.dirty_effects.is_empty() {
            return;
        }
        // elements built in earlier flushes are mounted or gone
        rt.elements.reset();
    }
    let cache = WorldGuard::setup_cache(world, CurrentScriptAttachment(None));
    WorldAccessGuard::with_static_guard(world, cache, |guard| {
        let _current = CurrentRuntime::set(runtime.clone());
        let runners = guard.script_callback_runners();
        let report = |attachment: Option<&ScriptAttachment>, e| {
            runners.report_error(guard.clone(), attachment, "running reactive ui".into(), e)
        };
        let mut poll = false;
        for _ in 0..MAX_FLUSH_ITERATIONS {
            match flush_pass(&runtime, &guard, &runners, poll, None, &report) {
                Pass::Idle => return,
                // handlers and effects may have changed watched components
                Pass::Ran { side_effects } => poll = side_effects,
            }
        }
        let rt = runtime.lock();
        if !rt.events.is_empty() || !rt.dirty.is_empty() || !rt.dirty_effects.is_empty() {
            bevy_log::warn!(
                "reactive ui work was still dirty after {MAX_FLUSH_ITERATIONS} passes, it runs next frame. Do effects keep setting signals they read?"
            );
        }
    });
}

/// Runs the work queued so far now, instead of in `PostUpdate`, from a script function: the calling script's
/// work runs in place, other scripts' in their contexts. Returns the first error the work hit.
#[cfg(feature = "testing")]
pub(crate) fn settle(ctx: &FunctionCallContext) -> Result<(), InteropError> {
    let attachment = current_attachment(ctx)?;
    let language = ctx.language();
    let runtime = runtime(ctx)?;
    let guard = ctx.world()?;
    let runners = guard.script_callback_runners();
    let _current = CurrentRuntime::set(runtime.clone());
    let errors = std::cell::RefCell::new(Vec::new());
    let report = |_: Option<&ScriptAttachment>, e| errors.borrow_mut().push(e);
    let mut poll = true;
    for _ in 0..MAX_FLUSH_ITERATIONS {
        let pass = flush_pass(
            &runtime,
            &guard,
            &runners,
            poll,
            Some((&attachment, &language)),
            &report,
        );
        match pass {
            Pass::Idle => return errors.into_inner().into_iter().next().map_or(Ok(()), Err),
            Pass::Ran { side_effects } => poll = side_effects,
        }
    }
    Err(InteropError::string(format!(
        "the ui was still changing after {MAX_FLUSH_ITERATIONS} passes, do effects keep setting signals they read?"
    )))
}

/// What a pass of [`flush_reactive_ui`] did.
enum Pass {
    Idle,
    Ran {
        /// Whether event handlers or effects ran
        side_effects: bool,
    },
}

/// What reports the errors of a flush, with the script they come from.
type Report<'a> = dyn Fn(Option<&ScriptAttachment>, InteropError) + 'a;

/// One pass of [`flush_reactive_ui`], `poll` checks the watched components first.
///
/// The work of the `current` script, the one calling, runs in place: it is already in its context.
fn flush_pass(
    runtime: &Arc<Mutex<Runtime>>,
    guard: &WorldGuard,
    runners: &AppScriptCallbackRunners,
    poll: bool,
    current: Option<(&ScriptAttachment, &Language)>,
    report: &Report,
) -> Pass {
    let (attachments, side_effects) = {
        let mut rt = runtime.lock();
        if poll && let Err(e) = guard.with_world_access(|w| rt.poll_watches(w)) {
            bevy_log::error!("reactive ui could not check watched components: {e:?}");
        }
        // values computed in Rust don't need their script's context
        let rust_bindings = take_rust_bindings(&mut rt);
        drop(rt);
        for (id, attachment) in rust_bindings {
            if let Err(e) = run_rust_binding(runtime, guard, id) {
                report(Some(&attachment), e);
            }
        }
        let rt = runtime.lock();
        let side_effects = !rt.events.is_empty() || !rt.dirty_effects.is_empty();
        let mut attachments: Vec<ScriptAttachment> = Vec::new();
        let dirty = rt
            .dirty
            .iter()
            .chain(&rt.dirty_effects)
            .copied()
            .collect::<Vec<_>>();
        let pending = rt.events.iter().map(|e| e.attachment.clone()).chain(
            dirty
                .iter()
                .filter_map(|id| rt.nodes.get(id).map(|n| n.attachment.clone())),
        );
        for attachment in pending {
            if !attachments.contains(&attachment) {
                attachments.push(attachment);
            }
        }
        (attachments, side_effects)
    };
    if attachments.is_empty() {
        return Pass::Idle;
    }
    for attachment in attachments {
        let mut run = |world: WorldGuard<'_>, language: Language| {
            let cx = Cx {
                runtime,
                world,
                attachment: &attachment,
                language,
            };
            run_script_work(&cx, report);
            Ok(())
        };
        let result = match current {
            Some((current, language)) if *current == attachment => {
                run(guard.clone(), language.clone())
            }
            _ => runners.run_in_context(guard.clone(), &attachment, &mut run),
        };
        if let Err(e) = result {
            // the script is gone or busy, drop its work rather than retrying forever
            let mut rt = runtime.lock();
            rt.events.retain(|e| e.attachment != attachment);
            let nodes = rt
                .dirty
                .iter()
                .chain(&rt.dirty_effects)
                .copied()
                .filter(|id| rt.nodes.get(id).is_some_and(|n| n.attachment == attachment))
                .collect::<Vec<_>>();
            for id in nodes {
                rt.dirty.remove(&id);
                rt.dirty_effects.remove(&id);
            }
            drop(rt);
            report(Some(&attachment), e);
        }
    }
    Pass::Ran { side_effects }
}

/// Runs the queued handlers, then the dirty nodes, then the dirty effects of one script, inside its context.
fn run_script_work(cx: &Cx, report: &Report) {
    let report = |e: InteropError| report(Some(cx.attachment), e);
    let events = {
        let mut rt = cx.runtime.lock();
        let (mine, others) = std::mem::take(&mut rt.events)
            .into_iter()
            .partition::<Vec<_>, _>(|e| e.attachment == *cx.attachment);
        rt.events = others;
        mine
    };
    for event in events {
        let _contexts = ContextsGuard::set(event.contexts);
        if let Err(e) = event
            .handler
            .call(event.args, FunctionCallContext::new(cx.language.clone()))
        {
            report(e);
        }
    }
    for effects in [false, true] {
        let ids = {
            let rt = cx.runtime.lock();
            let set = if effects {
                &rt.dirty_effects
            } else {
                &rt.dirty
            };
            set.iter()
                .copied()
                .filter(|id| {
                    rt.nodes
                        .get(id)
                        .is_some_and(|n| n.attachment == *cx.attachment)
                })
                .collect::<Vec<_>>()
        };
        for id in ids {
            type Run = fn(&Cx, NodeId) -> Result<(), InteropError>;
            let (run, certain): (Run, bool) = {
                let mut rt = cx.runtime.lock();
                rt.dirty.remove(&id);
                rt.dirty_effects.remove(&id);
                // a parent may have re-rendered it already in this pass
                match rt.nodes.get(&id) {
                    Some(node) if node.dirty || node.check => (
                        match &node.kind {
                            NodeKind::Instance(_) => |cx, id| cx.render_instance(id),
                            NodeKind::Binding { .. } => |cx, id| cx.run_binding(id),
                            NodeKind::Region { .. } => |cx, id| cx.run_region(id),
                            NodeKind::Effect { .. } => |cx, id| cx.run_effect(id),
                            _ => continue,
                        },
                        node.dirty,
                    ),
                    _ => continue,
                }
            };
            if !certain {
                // only a computed it read may have changed: skip it if none did
                match deps_changed(cx.runtime, &cx.call_context(), id) {
                    Ok(true) => {}
                    Ok(false) => {
                        if let Some(node) = cx.runtime.lock().nodes.get_mut(&id) {
                            node.check = false;
                        }
                        continue;
                    }
                    Err(e) => {
                        report(e);
                        continue;
                    }
                }
            }
            let result = run(cx, id);
            if let Err(e) = result {
                report(e);
            }
        }
    }
}
