//! Prototype of reactive `bevy_ui` trees described by scripts, built on signals.
//!
//! Components run once (and again when their script reloads). Properties bound to functions re-run when the
//! signals they read change, and `show` regions re-run to produce changing children.
//! `each(items, function(item) .. end)` is the region for lists: one child per item of the list in a signal,
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

mod element;
mod events;
mod expr;
mod props;
mod runtime;

mod style;

use std::{collections::VecDeque, marker::PhantomData};

use bevy_mod_scripting_bindings::{
    A, ArgMeta, CallScope, FromScript, GenericTypesAppExt, GetTypeDependencies, ThroughTypeInfo,
    TypeOf, TypedThrough, Union, ValueOf, WithDeps,
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_reflect::{GetTypeRegistration, TypePath, TypeRegistry, Typed};
use element::{Prop, PropValue};
use props::{As, Element, UiProps};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    entity::Entity,
    hierarchy::Children,
    message::MessageReader,
    name::Name,
    reflect::AppTypeRegistry,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::{Commands, Res},
    world::World,
};
use bevy_mod_scripting_bindings::{
    AppScriptCallbackRunners, CurrentScriptAttachment, FunctionCallContext, InteropError,
    NamespaceMembers, ScriptComponentRegistration, ScriptValue, V, WorldAccessGuard,
    WorldExtensions, WorldGuard,
};
use bevy_mod_scripting_core::event::ScriptLifecycleMessage;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_picking::Pickable;
use bevy_reflect::Reflect;
use bevy_ui::{BackgroundColor, Node, UiSystems, Val, widget::Text};
pub use element::{ComponentDef, UiElement};
use element::{
    ElementData, ElementKind, Func, element_from_script, elements_from_script, new_element,
};
pub use expr::UiValue;
use parking_lot::Mutex;
use runtime::{
    CurrentRuntime, Cx, Def, Instance, NodeId, NodeKind, Root, Runtime, deps_changed, read,
    run_rust_binding, runtime, slot_node, take_rust_bindings, write,
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

/// A region element, `props` are the properties of the node holding its children.
fn region(
    ctx: &FunctionCallContext,
    kind: ElementKind,
    props: Option<UiProps>,
) -> Result<Element, InteropError> {
    let props = match props {
        Some(props) => props.into_props(&ctx.world()?)?.1,
        None => Vec::new(),
    };
    Ok(Element::new(new_element(
        &*runtime(ctx)?,
        ElementData {
            kind,
            key: None,
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
    /// A container, one `bevy_ui` entity with its children.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties.
    /// * `children`: Its children.
    /// Returns:
    /// * `element`: The element.
    fn node(
        ctx: FunctionCallContext,
        props: Option<UiProps>,
        children: Option<props::Children>,
    ) -> Result<Element, InteropError> {
        let world = ctx.world()?;
        let runtime = runtime(&ctx)?;
        let (key, props) = match props {
            Some(props) => props.into_props(&world)?,
            None => (None, Vec::new()),
        };
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

    /// Text, `text("hello")` is `text({ text = "hello" })`.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties, or its text.
    /// Returns:
    /// * `element`: The element.
    fn text(
        ctx: FunctionCallContext,
        props: Union<String, UiProps>,
    ) -> Result<Element, InteropError> {
        let (key, props) = match props.into_left() {
            Ok(text) => (
                None,
                vec![(
                    Prop::Text,
                    PropValue::Static(ScriptValue::String(text.into())),
                )],
            ),
            Err(props) => props.into_props(&ctx.world()?)?,
        };
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
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `props`: Its properties.
    /// Returns:
    /// * `element`: The element.
    fn image(ctx: FunctionCallContext, props: UiProps) -> Result<Element, InteropError> {
        let (key, props) = props.into_props(&ctx.world()?)?;
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
    /// Call it from `on_script_loaded`.
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
        let existing = runtime
            .lock()
            .roots
            .get(&(attachment.clone(), key.clone()))
            .map(|r| (r.container, r.instance));
        if let Some((container, instance)) = existing {
            let mut rt = runtime.lock();
            if let Some(i) = rt.instance_mut(instance) {
                i.def = def;
                i.props = props;
            }
            rt.mark_dirty(instance);
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
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `key`: The key.
    /// * `element`: The element.
    /// Returns:
    /// * `element`: The element with the key.
    fn keyed(
        ctx: FunctionCallContext,
        key: As<props::Key>,
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
            |k| matches!(k, NodeKind::Signal { .. }),
            move || NodeKind::Signal { value: initial },
        );
        Ok(V(Signal::<A>::new(id)))
    }

    /// A value derived from others, recomputed when what `f` read changed. Its readers only update when the value did.
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
        let id = slot_node(
            &*runtime(&ctx)?,
            attachment,
            move |k| match k {
                NodeKind::Computed { f, value } => {
                    *f = new_f;
                    *value = None;
                    true
                }
                _ => false,
            },
            move || NodeKind::Computed { f, value: None },
        );
        Ok(V(Signal::<A>::new(id)))
    }

    /// Runs `f` now and whenever what it read changes. `f` can return a function to clean up before the next run.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `f`: The effect.
    fn effect(ctx: FunctionCallContext, f: WithDeps<fn()>) -> Result<(), InteropError> {
        let f = f.into_inner();
        let attachment = current_attachment(&ctx)?;
        let runtime = runtime(&ctx)?;
        let new_f = f.clone();
        let mut created = false;
        let id = slot_node(
            &runtime,
            attachment,
            move |k| match k {
                NodeKind::Effect { f, .. } => {
                    *f = new_f;
                    true
                }
                _ => false,
            },
            || {
                created = true;
                NodeKind::Effect { f, cleanup: None }
            },
        );
        if created {
            runtime.lock().mark_dirty(id);
        }
        Ok(())
    }

    /// A signal over a component of an entity, updated when the component changes, whoever changed it.
    /// `get` returns the component, or nil when it is gone.
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
            |k| matches!(k, NodeKind::Watch { entity: e, component: c, .. } if *e == entity && *c == component_id),
            move || NodeKind::Watch {
                entity,
                component: component_id,
                type_id,
                last_changed: now,
            },
        );
        Ok(V(Signal::<A>::new(id)))
    }

    /// The entity with the `Name` below `root`, breadth first.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `root`: Where to search from, i.e. what `render` returned.
    /// * `name`: The name.
    /// Returns:
    /// * `entity`: The entity, or nil.
    fn find(
        ctx: FunctionCallContext,
        root: V<Entity>,
        name: String,
    ) -> Result<Option<V<Entity>>, InteropError> {
        ctx.world()?
            .with_world_access(|w| {
                let mut queue = VecDeque::from([*root]);
                while let Some(e) = queue.pop_front() {
                    if w.get::<Name>(e).is_some_and(|n| n.as_str() == name) {
                        return Some(V(e));
                    }
                    if let Some(children) = w.get::<Children>(e) {
                        queue.extend(children.iter());
                    }
                }
                None
            })
            .map_err(InteropError::from)
    }

    /// What an element shows: its text, colours and layout, for tests.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `entity`: The entity of the element.
    /// Returns:
    /// * `description`: What it shows.
    fn describe(
        ctx: FunctionCallContext,
        entity: V<Entity>,
    ) -> Result<props::Description, InteropError> {
        ctx.world()?
            .with_world_access(|w| {
                let entity = *entity;
                // the rest is described as Rust prints it, which is enough to check what was set
                props::Description {
                    text: w.get::<Text>(entity).map(|text| text.0.clone()),
                    name: w.get::<Name>(entity).map(|name| name.as_str().to_owned()),
                    background_color: w
                        .get::<BackgroundColor>(entity)
                        .map(|bg| bg.0.to_srgba().to_hex()),
                    layout: w.get::<Node>(entity).map(|node| format!("{node:?}")),
                    font_size: w
                        .get::<bevy_text::TextFont>(entity)
                        .map(|font| format!("{:?}", font.font_size)),
                    color: w
                        .get::<bevy_text::TextColor>(entity)
                        .map(|color| color.0.to_srgba().to_hex()),
                    border_color: w
                        .get::<bevy_ui::BorderColor>(entity)
                        .map(|border| border.top.to_srgba().to_hex()),
                    image: w.get::<bevy_ui::widget::ImageNode>(entity).is_some(),
                    children: w.get::<Children>(entity).map_or(0, |c| c.len()) as i64,
                }
            })
            .map_err(InteropError::from)
    }

    /// Does what a pointer event on the element would, for tests and for driving a UI without a pointer.
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `entity`: The entity of the element.
    /// * `event`: The event: `"click"`, `"press"`, `"release"`, `"pointer_enter"`, `"pointer_leave"`, `"hover"`, `"drag"` or `"scroll"`.
    fn interact(
        ctx: FunctionCallContext,
        entity: V<Entity>,
        event: String,
    ) -> Result<(), InteropError> {
        let Some((_, event)) = style::UiEvent::ALL.iter().find(|(name, _)| *name == event) else {
            let names = style::UiEvent::ALL.map(|(name, _)| name).join(", ");
            return Err(InteropError::string(format!(
                "unknown event `{event}`, use one of: {names}"
            )));
        };
        let runtime = runtime(&ctx)?;
        ctx.world()?
            .with_world_access(|w| events::simulate(w, &runtime, *entity, *event))?;
        Ok(())
    }

    /// A value provided to a subtree with `provide` and read with `use_context`.
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

    /// The value of a context provided nearest above the component rendering, or its default.
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

    /// A node providing the value of a context to what is below it.
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
    /// which is the signal's value.
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
        runtime::selector(&*runtime(&ctx)?, attachment, source.into_inner().0)
            .map(|id| V(Selector::<A>::new(id)))
    }

    /// One child per item of the list in the signal, made by `f(item)`. Children are kept by item, `f` only runs for
    /// items which were not in the list before.
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
        props: Option<UiProps>,
    ) -> Result<Element, InteropError> {
        let kind = ElementKind::Each(items.0.0, f.into_inner(), false);
        region(&ctx, kind, props)
    }

    /// As `list`, but `f` runs once with a value standing for each item, and the rows are built from what it returned
    /// without running script code. Handlers get the row's item first.
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
        f: WithDeps<fn(As<V<UiValue>>) -> Element>,
        props: Option<UiProps>,
    ) -> Result<Element, InteropError> {
        let kind = ElementKind::Each(items.0.0, f.into_inner(), true);
        region(&ctx, kind, props)
    }

    /// A region showing what `f` returns, an element or nothing. Re-runs when what `f` read changes.
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
        props: Option<UiProps>,
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
        runtime::is_selected(&*runtime(&ctx)?, selector.into_inner().0, &key)
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "signal_functions"
)]
impl Signal {
    /// The value, recording the read: what is reading it re-runs when it changes.
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

    /// The value, without recording the read.
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

    /// Changes the value, re-running what read it.
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
        write(&*runtime(&ctx)?, signal.into_inner().0, value.into_inner())
    }
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
        let mut poll = false;
        for _ in 0..MAX_FLUSH_ITERATIONS {
            match flush_pass(&runtime, &guard, &runners, poll) {
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

/// What a pass of [`flush_reactive_ui`] did.
enum Pass {
    Idle,
    Ran {
        /// Whether event handlers or effects ran
        side_effects: bool,
    },
}

/// One pass of [`flush_reactive_ui`], `poll` checks the watched components first.
fn flush_pass(
    runtime: &Arc<Mutex<Runtime>>,
    guard: &WorldGuard,
    runners: &AppScriptCallbackRunners,
    poll: bool,
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
                runners.report_error(
                    guard.clone(),
                    Some(&attachment),
                    "running reactive ui".into(),
                    e,
                );
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
        let result = runners.run_in_context(guard.clone(), &attachment, &mut |world, language| {
            let cx = Cx {
                runtime,
                world,
                attachment: &attachment,
                language,
            };
            run_script_work(&cx, runners);
            Ok(())
        });
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
            runners.report_error(
                guard.clone(),
                Some(&attachment),
                "running reactive ui".into(),
                e,
            );
        }
    }
    Pass::Ran { side_effects }
}

/// Runs the queued handlers, then the dirty nodes, then the dirty effects of one script, inside its context.
fn run_script_work(cx: &Cx, runners: &AppScriptCallbackRunners) {
    let report = |e: InteropError| {
        runners.report_error(
            cx.world.clone(),
            Some(cx.attachment),
            "running reactive ui".into(),
            e,
        )
    };
    let events = {
        let mut rt = cx.runtime.lock();
        let (mine, others) = std::mem::take(&mut rt.events)
            .into_iter()
            .partition::<Vec<_>, _>(|e| e.attachment == *cx.attachment);
        rt.events = others;
        mine
    };
    for event in events {
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
