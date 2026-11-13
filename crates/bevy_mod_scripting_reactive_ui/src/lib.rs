//! Prototype of reactive `bevy_ui` trees described by scripts, built on signals.
//!
//! Components run once (and again when their script reloads). Properties bound to functions re-run when the
//! signals they read change, and `show` regions re-run to produce changing children.
//! `each(items, function(item) .. end)` is the region for lists: one child per item of the list in a signal,
//! kept by item (items are the keys), so the function only runs for items which were not in the list before.
//!
//! `list_template(items, function(item) .. end, deps)` builds the rows of large uniform lists without running
//! script code per row: the function runs once, `item` stands for each row's item, and the `node`s and `text`s
//! it returns are made per row in Rust. Properties are values computed in Rust (`format`, `when`, `selected`,
//! `path(watch_item(item, Component), ..)` for items which are entities), functions (handlers and bound
//! properties) are shared by all rows and get the row's item after their dependencies.
//!
//! Functions can take dependencies: `computed(f, deps)`, `effect(f, deps)`, `show(f, deps)` and
//! `list(items, f, deps)` call `f` with the values of the list `deps` before its own arguments. A handler or
//! bound property is given with its dependencies as a list starting with the function,
//! `on_click = { f, selected, name }`. This is how a function reaches signals and entities in languages whose
//! closures can't capture them when handed to Rust (Rune), and it is optional where they can (Lua).
//!
//! `element(Def, f, deps, key)` renders a component with `f(deps..)` instead of a props table, which is converted
//! to Rust and back for every instance.
//!
//! Everything is language agnostic: script functions are stored as [`ScriptValue::Function`]s and run
//! in their script's context with [`AppScriptCallbackRunners::run_in_context`].
//!
//! ```lua
//! local ui = import_namespace("reactive_ui")
//! local Counter = ui.component("Counter", function(props)
//!     local n = ui.signal(props.start)
//!     return ui.node({ flex_direction = "row" }, {
//!         ui.node({ name = "dec", on_click = function() n:set(n:get() - 1) end }),
//!         ui.text({ text = function() return "n=" .. n:get() end }),
//!     })
//! end)
//! function on_script_loaded() ui.render("counter", Counter, { start = 0 }) end
//! ```

// `Interaction` is deprecated for picking's `Hovered` and `Pressed`, it keeps the prototype independent of picking
#![allow(deprecated)]

mod element;
mod expr;
mod runtime;
mod style;

use std::collections::VecDeque;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    change_detection::{DetectChanges, Ref},
    entity::Entity,
    hierarchy::Children,
    message::MessageReader,
    name::Name,
    query::{Changed, QueryState},
    reflect::AppTypeRegistry,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::{Commands, Local, Res},
    world::World,
};
use bevy_mod_scripting_bindings::{
    AppScriptCallbackRunners, CurrentScriptAttachment, FunctionCallContext, InteropError,
    NamespaceMembers, ScriptComponentRegistration, ScriptValue, WorldAccessGuard, WorldExtensions,
    WorldGuard,
    function::{from::V, namespace::NamespaceBuilder},
};
use bevy_mod_scripting_core::event::ScriptLifecycleMessage;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_reflect::Reflect;
use bevy_ui::{BackgroundColor, Interaction, Node, UiSystems, widget::Text};
pub use element::{ComponentDef, UiElement};
use element::{ElementData, ElementKind, Func, elements_from_script, new_element, parse_props};
pub use expr::UiValue;
use parking_lot::Mutex;
use runtime::{
    CurrentRuntime, Cx, Def, Instance, NodeId, NodeKind, QueuedEvent, Root, Runtime, deps_changed,
    read, run_rust_binding, runtime, slot_node, take_rust_bindings, write,
};
pub use runtime::{ReactiveUi, UiHandlers, UiRoot};
use std::sync::Arc;

/// How many times a flush re-runs work made dirty by the work it ran (i.e. effects setting signals) before leaving it for the next frame.
pub const MAX_FLUSH_ITERATIONS: usize = 16;

/// A handle to a signal, computed or watched component, read with `get` (tracked) or `peek` (untracked), set with `set`.
#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Signal(NodeId);

/// Tells whether a key is the value of a signal, made by `selector`.
///
/// `selector:selected(key)` only re-runs its reader when `key` becomes or stops being selected,
/// so a list of items each checking whether they are selected only re-runs two of them when the selection changes.
#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Selector(NodeId);

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
            .register_type::<ComponentDef>()
            .register_type::<Signal>()
            .register_type::<Selector>()
            .register_type::<UiValue>()
            .register_type::<expr::ItemWatch>()
            .add_systems(
                PostUpdate,
                (forget_detached_scripts, flush_reactive_ui)
                    .chain()
                    .in_set(ReactiveUiSystems)
                    .before(UiSystems::Layout),
            );
        register_functions(app.world_mut());
        expr::register(app.world_mut());
    }

    fn finish(&self, app: &mut App) {
        // for the `image` property, where the app loads assets
        if app.world().contains_resource::<bevy_asset::AssetServer>() {
            bevy_asset::AssetApp::init_asset::<bevy_image::Image>(app);
        }
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

fn define(
    ctx: FunctionCallContext,
    name: String,
    render: Func,
) -> Result<V<ComponentDef>, InteropError> {
    let attachment = current_attachment(&ctx)?;
    let runtime = runtime(&ctx)?;
    let mut rt = runtime.lock();
    let def = Def { render };
    if let Some(existing) = rt
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
        return Ok(V(existing));
    }
    let id = ComponentDef(rt.defs.len() as u32);
    rt.defs.push(def);
    rt.def_index.insert((attachment, name), id);
    Ok(V(id))
}

fn register_functions(world: &mut World) {
    let mut ui = NamespaceBuilder::<NamespaceMembers>::in_namespace(world, "reactive_ui");
    ui.register(
        "node",
        |ctx: FunctionCallContext, props: Option<ScriptValue>, children: Option<ScriptValue>| {
            let world = ctx.world()?;
            let runtime = runtime(&ctx)?;
            let (key, props) = parse_props(props, &world)?;
            let children = match children {
                Some(children) => elements_from_script(&children, &runtime)?,
                None => Vec::new(),
            };
            Ok::<_, InteropError>(new_element(
                &runtime,
                ElementData {
                    kind: ElementKind::Node,
                    key,
                    props,
                    children,
                },
            ))
        },
    )
    .register("text", |ctx: FunctionCallContext, props: ScriptValue| {
        let (key, props) = parse_props(Some(props), &ctx.world()?)?;
        Ok::<_, InteropError>(new_element(&*runtime(&ctx)?, ElementData {
            kind: ElementKind::Text,
            key,
            props,
            children: Vec::new(),
        }))
    })
    .register("image", |ctx: FunctionCallContext, props: ScriptValue| {
        // a node showing the image asset at the path given as its `image`
        let (key, props) = parse_props(Some(props), &ctx.world()?)?;
        Ok::<_, InteropError>(new_element(&*runtime(&ctx)?, ElementData {
            kind: ElementKind::Node,
            key,
            props,
            children: Vec::new(),
        }))
    })
    .register(
        "element",
        |ctx: FunctionCallContext,
         def: V<ComponentDef>,
         props: Option<ScriptValue>,
         deps: Option<ScriptValue>,
         key: Option<ScriptValue>| {
            // `props` is a table given to the component's function. Or a function rendering the component,
            // called with the dependencies: they are passed as they are, not as a table converted both ways
            let props = match props.unwrap_or(ScriptValue::Unit) {
                ScriptValue::Function(f) => ScriptValue::Function(element::with_deps(f, deps)?),
                props => props,
            };
            let key = match (&key, &props) {
                (Some(key), _) => Some(element::key_of(key, &ctx.world()?)?),
                (None, ScriptValue::Map(map)) => map
                    .get("key")
                    .map(|k| element::key_of(k, &ctx.world()?))
                    .transpose()?,
                _ => None,
            };
            Ok::<_, InteropError>(new_element(&*runtime(&ctx)?, ElementData {
                kind: ElementKind::Component(*def, props),
                key,
                props: Vec::new(),
                children: Vec::new(),
            }))
        },
    )
    .register(
        "render",
        |ctx: FunctionCallContext, key: String, def: V<ComponentDef>, props: Option<ScriptValue>| {
            let attachment = current_attachment(&ctx)?;
            let runtime = runtime(&ctx)?;
            let props = props.unwrap_or(ScriptValue::Unit);
            let existing = runtime
                .lock()
                .roots
                .get(&(attachment.clone(), key.clone()))
                .map(|r| (r.container, r.instance));
            if let Some((container, instance)) = existing {
                let mut rt = runtime.lock();
                if let Some(i) = rt.instance_mut(instance) {
                    i.def = *def;
                    i.props = props;
                }
                rt.mark_dirty(instance);
                return Ok::<_, InteropError>(V(container));
            }
            let root = UiRoot {
                attachment: attachment.clone(),
                key: key.clone(),
            };
            let container = ctx
                .world()?
                .with_world_mut_access(|w| w.spawn((Node::default(), root)).id())?;
            let mut rt = runtime.lock();
            let instance = rt.create(
                NodeKind::Instance(Instance {
                    def: *def,
                    props,
                    slots: Vec::new(),
                    rendered: None,
                    parent: container,
                }),
                attachment.clone(),
            );
            rt.mark_dirty(instance);
            rt.roots.insert((attachment, key), Root { container, instance });
            Ok(V(container))
        },
    )
    .register("signal", |ctx: FunctionCallContext, initial: ScriptValue| {
        let attachment = current_attachment(&ctx)?;
        let id = slot_node(
            &*runtime(&ctx)?,
            attachment,
            |k| matches!(k, NodeKind::Signal { .. }),
            move || NodeKind::Signal { value: initial },
        );
        Ok::<_, InteropError>(V(Signal(id)))
    })
    .register("computed", |ctx: FunctionCallContext, f: Func, deps: Option<ScriptValue>| {
        let f = element::with_deps(f, deps)?;
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
        Ok::<_, InteropError>(V(Signal(id)))
    })
    .register("effect", |ctx: FunctionCallContext, f: Func, deps: Option<ScriptValue>| {
        let f = element::with_deps(f, deps)?;
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
        Ok::<_, InteropError>(())
    })
    .register(
        "watch",
        |ctx: FunctionCallContext, entity: V<Entity>, component: V<ScriptComponentRegistration>| {
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
                |k| {
                    matches!(k, NodeKind::Watch { entity: e, component: c, .. } if *e == entity && *c == component_id)
                },
                move || NodeKind::Watch {
                    entity,
                    component: component_id,
                    type_id,
                    last_changed: now,
                },
            );
            Ok::<_, InteropError>(V(Signal(id)))
        },
    )
    .register(
        "find",
        |ctx: FunctionCallContext, root: V<Entity>, name: String| {
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
        },
    )
    .register("describe", |ctx: FunctionCallContext, entity: V<Entity>| {
        ctx.world()?
            .with_world_access(|w| {
                let entity = *entity;
                let mut map = std::collections::HashMap::new();
                if let Some(text) = w.get::<Text>(entity) {
                    map.insert("text".to_owned(), ScriptValue::String(text.0.clone().into()));
                }
                if let Some(name) = w.get::<Name>(entity) {
                    map.insert("name".to_owned(), ScriptValue::String(name.as_str().to_owned().into()));
                }
                if let Some(bg) = w.get::<BackgroundColor>(entity) {
                    map.insert("background_color".to_owned(), ScriptValue::String(bg.0.to_srgba().to_hex().into()));
                }
                // the rest is described as Rust prints it, which is enough to check what was set
                if let Some(node) = w.get::<Node>(entity) {
                    map.insert("layout".to_owned(), ScriptValue::String(format!("{node:?}").into()));
                }
                if let Some(font) = w.get::<bevy_text::TextFont>(entity) {
                    map.insert("font_size".to_owned(), ScriptValue::String(format!("{:?}", font.font_size).into()));
                }
                if let Some(color) = w.get::<bevy_text::TextColor>(entity) {
                    map.insert("color".to_owned(), ScriptValue::String(color.0.to_srgba().to_hex().into()));
                }
                if let Some(border) = w.get::<bevy_ui::BorderColor>(entity) {
                    map.insert("border_color".to_owned(), ScriptValue::String(border.top.to_srgba().to_hex().into()));
                }
                map.insert(
                    "image".to_owned(),
                    ScriptValue::Bool(w.get::<bevy_ui::widget::ImageNode>(entity).is_some()),
                );
                let children = w.get::<Children>(entity).map_or(0, |c| c.len());
                map.insert("children".to_owned(), ScriptValue::Integer(children as i64));
                ScriptValue::Map(map.into_iter().collect())
            })
            .map_err(InteropError::from)
    })
    .register(
        "interact",
        |ctx: FunctionCallContext, entity: V<Entity>, state: String| {
            let interaction = match state.as_str() {
                "press" => Interaction::Pressed,
                "hover" => Interaction::Hovered,
                "none" => Interaction::None,
                other => {
                    return Err(InteropError::string(format!(
                        "unknown interaction `{other}`, use \"press\", \"hover\" or \"none\""
                    )));
                }
            };
            ctx.world()?.with_world_mut_access(|w| {
                if let Ok(mut e) = w.get_entity_mut(*entity) {
                    e.insert(interaction);
                }
            })?;
            Ok(())
        },
    )
    .register("component", define)
    .register("selector", |ctx: FunctionCallContext, source: V<Signal>| {
        let attachment = current_attachment(&ctx)?;
        runtime::selector(&*runtime(&ctx)?, attachment, source.into_inner().0)
            .map(|id| V(Selector(id)))
    })
    .register(
        "list",
        |ctx: FunctionCallContext, items: V<Signal>, f: Func, deps: Option<ScriptValue>| {
            let f = element::with_deps(f, deps)?;
            Ok::<_, InteropError>(new_element(
                &*runtime(&ctx)?,
                ElementData {
                    kind: ElementKind::Each(items.into_inner().0, f, false),
                    key: None,
                    props: Vec::new(),
                    children: Vec::new(),
                },
            ))
        },
    )
    .register(
        "list_template",
        |ctx: FunctionCallContext, items: V<Signal>, f: Func, deps: Option<ScriptValue>| {
            let f = element::with_deps(f, deps)?;
            Ok::<_, InteropError>(new_element(
                &*runtime(&ctx)?,
                ElementData {
                    kind: ElementKind::Each(items.into_inner().0, f, true),
                    key: None,
                    props: Vec::new(),
                    children: Vec::new(),
                },
            ))
        },
    )
    .register("show", |ctx: FunctionCallContext, f: Func, deps: Option<ScriptValue>| {
        let f = element::with_deps(f, deps)?;
        Ok::<_, InteropError>(new_element(
            &*runtime(&ctx)?,
            ElementData {
                kind: ElementKind::Dynamic(f),
                key: None,
                props: Vec::new(),
                children: Vec::new(),
            },
        ))
    });

    NamespaceBuilder::<Selector>::new(world).register(
        "selected",
        |ctx: FunctionCallContext, s: V<Selector>, key: ScriptValue| {
            runtime::is_selected(&*runtime(&ctx)?, s.into_inner().0, &key)
        },
    );
    NamespaceBuilder::<Signal>::new(world)
        .register("get", |ctx: FunctionCallContext, s: V<Signal>| {
            read(&*runtime(&ctx)?, &ctx, s.into_inner().0, true)
        })
        .register("peek", |ctx: FunctionCallContext, s: V<Signal>| {
            read(&*runtime(&ctx)?, &ctx, s.into_inner().0, false)
        })
        .register(
            "set",
            |ctx: FunctionCallContext, s: V<Signal>, value: ScriptValue| {
                write(&*runtime(&ctx)?, s.into_inner().0, value)
            },
        );
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

/// The query finding elements whose [`Interaction`] changed, kept between flushes.
type InteractionQuery =
    QueryState<(Ref<'static, Interaction>, &'static UiHandlers), Changed<Interaction>>;

/// Queues the handlers of elements whose [`Interaction`] changed.
fn queue_interactions(world: &mut World, query: &mut InteractionQuery, runtime: &Mutex<Runtime>) {
    let mut rt = runtime.lock();
    for (interaction, handlers) in query.iter(world) {
        // newly mounted elements start with no interaction
        if interaction.is_added() {
            continue;
        }
        let (handler, args) = match *interaction {
            Interaction::Pressed => (&handlers.click, vec![]),
            Interaction::Hovered => (&handlers.hover, vec![ScriptValue::Bool(true)]),
            Interaction::None => (&handlers.hover, vec![ScriptValue::Bool(false)]),
        };
        if let Some(handler) = handler {
            rt.events.push(QueuedEvent {
                attachment: handlers.attachment.clone(),
                handler: handler.clone(),
                args,
            });
        }
    }
}

/// Runs queued event handlers, re-runs dirty components, bindings and regions, then dirty effects.
///
/// Work is grouped by script so each script's context is entered once per pass. Work made dirty by a pass
/// (i.e. by handlers or effects setting signals) runs in another pass, up to [`MAX_FLUSH_ITERATIONS`].
pub fn flush_reactive_ui(world: &mut World, mut interactions: Local<Option<InteractionQuery>>) {
    let Some(runtime) = world.get_resource::<ReactiveUi>().map(|r| r.0.clone()) else {
        return;
    };
    let interactions = interactions.get_or_insert_with(|| world.query_filtered());
    queue_interactions(world, interactions, &runtime);
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
