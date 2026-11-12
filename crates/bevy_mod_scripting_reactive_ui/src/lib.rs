//! Prototype of reactive `bevy_ui` trees described by scripts, in two flavours sharing one runtime:
//!
//! * `signals`: components run once, properties bound to functions re-run when the signals they read change,
//!   and `dynamic` regions re-run to produce changing children.
//! * `hybrid`: components re-render when the state they read changes, and their output is diffed against the previous render.
//!
//! Everything is language agnostic: script functions are stored as [`ScriptValue::Function`]s and run
//! in their script's context with [`AppScriptCallbackRunners::run_in_context`].
//!
//! ```lua
//! local ui = import_namespace("reactive_ui")
//! local Counter = ui.define_hybrid("Counter", function(props)
//!     local n = ui.signal(props.start)
//!     return ui.node({ direction = "row" }, {
//!         ui.node({ name = "dec", on_click = function() n:set(n:get() - 1) end }),
//!         ui.text("n=" .. n:get()),
//!     })
//! end)
//! function on_script_loaded() ui.mount("counter", Counter, { start = 0 }) end
//! ```

// `Interaction` is deprecated for picking's `Hovered` and `Pressed`, it keeps the prototype independent of picking
#![allow(deprecated)]

mod element;
mod runtime;

use std::collections::VecDeque;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    change_detection::{DetectChanges, Ref},
    entity::Entity,
    hierarchy::Children,
    message::MessageReader,
    name::Name,
    query::Changed,
    reflect::AppTypeRegistry,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::{Commands, Res},
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
use parking_lot::Mutex;
use runtime::{
    CurrentRuntime, Cx, Def, Instance, NodeId, NodeKind, QueuedEvent, Root, Runtime, UiHandlers,
    read, runtime, slot_node, write,
};
pub use runtime::{ReactiveUi, UiRoot};
use std::sync::Arc;

/// How many times a flush re-runs work made dirty by the work it ran (i.e. effects setting signals) before leaving it for the next frame.
pub const MAX_FLUSH_ITERATIONS: usize = 16;

/// A handle to a signal, computed or watched component, read with `get` (tracked) or `peek` (untracked), set with `set`.
#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Signal(NodeId);

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
            .add_systems(
                PostUpdate,
                (forget_detached_scripts, flush_reactive_ui)
                    .chain()
                    .in_set(ReactiveUiSystems)
                    .before(UiSystems::Layout),
            );
        register_functions(app.world_mut());
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

fn define(
    ctx: FunctionCallContext,
    name: String,
    render: Func,
    tracked: bool,
) -> Result<V<ComponentDef>, InteropError> {
    let attachment = current_attachment(&ctx)?;
    let runtime = runtime(&ctx)?;
    let mut rt = runtime.lock();
    let def = Def { render, tracked };
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
            let (key, props) = parse_props(props, &world)?;
            let children = match children {
                Some(children) => elements_from_script(&children, &world)?,
                None => Vec::new(),
            };
            Ok::<_, InteropError>(new_element(ElementData {
                kind: ElementKind::Node,
                key,
                props,
                children,
            }))
        },
    )
    .register("text", |ctx: FunctionCallContext, props: ScriptValue| {
        let (key, props) = parse_props(Some(props), &ctx.world()?)?;
        Ok::<_, InteropError>(new_element(ElementData {
            kind: ElementKind::Text,
            key,
            props,
            children: Vec::new(),
        }))
    })
    .register(
        "el",
        |ctx: FunctionCallContext, def: V<ComponentDef>, props: Option<ScriptValue>| {
            let props = props.unwrap_or(ScriptValue::Unit);
            let key = match &props {
                ScriptValue::Map(map) => map
                    .get("key")
                    .map(|k| element::key_of(k, &ctx.world()?))
                    .transpose()?,
                _ => None,
            };
            Ok::<_, InteropError>(new_element(ElementData {
                kind: ElementKind::Component(*def, props),
                key,
                props: Vec::new(),
                children: Vec::new(),
            }))
        },
    )
    .register(
        "mount",
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
    .register("computed", |ctx: FunctionCallContext, f: Func| {
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
    .register("effect", |ctx: FunctionCallContext, f: Func| {
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
                    map.insert("bg".to_owned(), ScriptValue::String(bg.0.to_srgba().to_hex().into()));
                }
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
    );
    #[cfg(feature = "signals")]
    ui.register(
        "define_signals",
        |ctx: FunctionCallContext, name: String, render: Func| define(ctx, name, render, false),
    )
    .register("dynamic", |f: Func| {
        new_element(ElementData {
            kind: ElementKind::Dynamic(f),
            key: None,
            props: Vec::new(),
            children: Vec::new(),
        })
    });
    #[cfg(feature = "hybrid")]
    ui.register(
        "define_hybrid",
        |ctx: FunctionCallContext, name: String, render: Func| define(ctx, name, render, true),
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

/// Queues the handlers of elements whose [`Interaction`] changed.
fn queue_interactions(world: &mut World, runtime: &Mutex<Runtime>) {
    let mut query = world.query_filtered::<(Ref<Interaction>, &UiHandlers), Changed<Interaction>>();
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
pub fn flush_reactive_ui(world: &mut World) {
    let Some(runtime) = world.get_resource::<ReactiveUi>().map(|r| r.0.clone()) else {
        return;
    };
    queue_interactions(world, &runtime);
    let cache = WorldGuard::setup_cache(world, CurrentScriptAttachment(None));
    WorldAccessGuard::with_static_guard(world, cache, |guard| {
        let _current = CurrentRuntime::set(runtime.clone());
        let runners = guard.script_callback_runners();
        for _ in 0..MAX_FLUSH_ITERATIONS {
            if !flush_pass(&runtime, &guard, &runners) {
                return;
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

/// One pass of [`flush_reactive_ui`], returns whether there was work to do.
fn flush_pass(
    runtime: &Arc<Mutex<Runtime>>,
    guard: &WorldGuard,
    runners: &AppScriptCallbackRunners,
) -> bool {
    let attachments = {
        let mut rt = runtime.lock();
        if let Err(e) = guard.with_world_access(|w| rt.poll_watches(w)) {
            bevy_log::error!("reactive ui could not check watched components: {e:?}");
        }
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
        attachments
    };
    if attachments.is_empty() {
        return false;
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
    true
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
            let run: Run = {
                let mut rt = cx.runtime.lock();
                rt.dirty.remove(&id);
                rt.dirty_effects.remove(&id);
                // a parent may have re-rendered it already in this pass
                match rt.nodes.get(&id) {
                    Some(node) if node.dirty => match &node.kind {
                        NodeKind::Instance(_) => |cx, id| cx.render_instance(id),
                        NodeKind::Binding { .. } => |cx, id| cx.run_binding(id),
                        NodeKind::Region { .. } => |cx, id| cx.run_region(id),
                        NodeKind::Effect { .. } => |cx, id| cx.run_effect(id),
                        _ => continue,
                    },
                    _ => continue,
                }
            };
            let result = run(cx, id);
            if let Err(e) = result {
                report(e);
            }
        }
    }
}
