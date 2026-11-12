//! The reactive graph (signals, computeds, effects, watches), component instances, and how element trees are
//! mounted onto and diffed against `bevy_ui` entities.
//!
//! Script functions run while no lock is held: the runtime is locked briefly for each graph operation only.

use std::{
    any::TypeId,
    cell::RefCell,
    collections::{BTreeSet, VecDeque},
    sync::Arc,
};

use bevy_ecs::{
    change_detection::Tick,
    component::{Component, ComponentId},
    entity::Entity,
    hierarchy::{ChildOf, Children},
    resource::Resource,
    world::World,
};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    FunctionCallContext, InteropError, ReflectReference, ScriptValue, WorldGuard,
};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;
use bevy_ui::{Interaction, Node, widget::Text};
use parking_lot::Mutex;

use crate::element::{
    ComponentDef, ElementKind, Func, Prop, PropValue, UiElement, apply_prop, element_from_script,
    elements_from_script, script_values_equal,
};

/// Identifies a node of the reactive graph. Ids are never reused.
pub type NodeId = u32;

/// The shared reactive UI state of a world.
#[derive(Resource, Clone, Default)]
pub struct ReactiveUi(pub(crate) Arc<Mutex<Runtime>>);

/// Marks the entity a tree is mounted under, see `mount`.
#[derive(Component, Debug)]
pub struct UiRoot {
    /// The script which mounted the tree
    pub attachment: ScriptAttachment,
    /// The key it was mounted with
    pub key: String,
}

/// The event handlers of a mounted element.
#[derive(Component)]
pub(crate) struct UiHandlers {
    pub attachment: ScriptAttachment,
    pub click: Option<Func>,
    pub hover: Option<Func>,
}

pub(crate) enum NodeKind {
    Signal {
        value: ScriptValue,
    },
    Watch {
        entity: Entity,
        component: ComponentId,
        type_id: TypeId,
        last_changed: Option<Tick>,
    },
    Computed {
        f: Func,
        value: Option<ScriptValue>,
    },
    Effect {
        f: Func,
        cleanup: Option<Func>,
    },
    Binding {
        entity: Entity,
        prop: Prop,
        f: Func,
    },
    Region {
        container: Entity,
        f: Func,
        children: Vec<Mounted>,
    },
    Instance(Instance),
}

pub(crate) struct Instance {
    pub def: ComponentDef,
    pub props: ScriptValue,
    /// Nodes created by `signal`, `computed`, `effect` and `watch` while rendering, in call order
    pub slots: Vec<NodeId>,
    pub rendered: Option<Mounted>,
    pub parent: Entity,
}

pub(crate) struct GraphNode {
    pub kind: NodeKind,
    pub attachment: ScriptAttachment,
    /// What this node read the last time it ran
    pub deps: Vec<NodeId>,
    /// What read this node
    pub subs: Vec<NodeId>,
    pub dirty: bool,
}

pub(crate) struct Def {
    pub render: Func,
}

pub(crate) struct Root {
    pub container: Entity,
    pub instance: NodeId,
}

pub(crate) struct QueuedEvent {
    pub attachment: ScriptAttachment,
    pub handler: Func,
    pub args: Vec<ScriptValue>,
}

/// What is mounted for an element.
pub(crate) enum Mounted {
    Host {
        entity: Entity,
        element: UiElement,
        children: Vec<Mounted>,
        bindings: Vec<(Prop, NodeId)>,
    },
    Component {
        instance: NodeId,
        key: Option<String>,
    },
    Dynamic {
        region: NodeId,
        entity: Entity,
    },
}

impl Mounted {
    fn key(&self) -> Option<&str> {
        match self {
            Mounted::Host { element, .. } => element.0.key.as_deref(),
            Mounted::Component { key, .. } => key.as_deref(),
            Mounted::Dynamic { .. } => None,
        }
    }
}

#[derive(Default)]
pub(crate) struct Runtime {
    next_id: NodeId,
    pub nodes: HashMap<NodeId, GraphNode>,
    pub defs: Vec<Def>,
    pub def_index: HashMap<(ScriptAttachment, String), ComponentDef>,
    pub roots: HashMap<(ScriptAttachment, String), Root>,
    /// Dirty instances, bindings and regions, run in creation order so parents render before their children
    pub dirty: BTreeSet<NodeId>,
    pub dirty_effects: BTreeSet<NodeId>,
    pub events: Vec<QueuedEvent>,
    pub watches: Vec<NodeId>,
}

impl Runtime {
    pub fn create(&mut self, kind: NodeKind, attachment: ScriptAttachment) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        if matches!(kind, NodeKind::Watch { .. }) {
            self.watches.push(id);
        }
        self.nodes.insert(
            id,
            GraphNode {
                kind,
                attachment,
                deps: Vec::new(),
                subs: Vec::new(),
                dirty: false,
            },
        );
        id
    }

    pub fn instance_mut(&mut self, id: NodeId) -> Option<&mut Instance> {
        match self.nodes.get_mut(&id).map(|n| &mut n.kind) {
            Some(NodeKind::Instance(instance)) => Some(instance),
            _ => None,
        }
    }

    /// Records that `observer` read `source`.
    pub fn link(&mut self, observer: NodeId, source: NodeId) {
        if observer == source || !self.nodes.contains_key(&observer) {
            return;
        }
        let Some(node) = self.nodes.get_mut(&source) else {
            return;
        };
        if !node.subs.contains(&observer) {
            node.subs.push(observer);
        }
        if let Some(observer) = self.nodes.get_mut(&observer)
            && !observer.deps.contains(&source)
        {
            observer.deps.push(source);
        }
    }

    /// Forgets what the node read, before it runs again.
    pub fn clear_deps(&mut self, id: NodeId) {
        let deps = match self.nodes.get_mut(&id) {
            Some(node) => std::mem::take(&mut node.deps),
            None => return,
        };
        for dep in deps {
            if let Some(dep) = self.nodes.get_mut(&dep) {
                dep.subs.retain(|s| *s != id);
            }
        }
    }

    /// Marks everything which read `id` as dirty, through computeds.
    pub fn mark_subs_dirty(&mut self, id: NodeId) {
        let mut stack = match self.nodes.get(&id) {
            Some(node) => node.subs.clone(),
            None => return,
        };
        while let Some(sub) = stack.pop() {
            self.mark_dirty_one(sub, &mut stack);
        }
    }

    pub fn mark_dirty(&mut self, id: NodeId) {
        let mut stack = Vec::new();
        self.mark_dirty_one(id, &mut stack);
        while let Some(sub) = stack.pop() {
            self.mark_dirty_one(sub, &mut stack);
        }
    }

    fn mark_dirty_one(&mut self, id: NodeId, stack: &mut Vec<NodeId>) {
        let Some(node) = self.nodes.get_mut(&id) else {
            return;
        };
        match &mut node.kind {
            NodeKind::Computed { value, .. } => {
                if value.take().is_some() || !node.dirty {
                    node.dirty = true;
                    stack.extend(node.subs.iter().copied());
                }
            }
            NodeKind::Effect { .. } => {
                node.dirty = true;
                self.dirty_effects.insert(id);
            }
            NodeKind::Signal { .. } | NodeKind::Watch { .. } => {}
            _ => {
                node.dirty = true;
                self.dirty.insert(id);
            }
        }
    }

    /// Removes a node from the graph. Does not run effect cleanups or unmount anything.
    pub fn remove(&mut self, id: NodeId) -> Option<GraphNode> {
        self.clear_deps(id);
        let node = self.nodes.remove(&id)?;
        for sub in &node.subs {
            if let Some(sub) = self.nodes.get_mut(sub) {
                sub.deps.retain(|d| *d != id);
            }
        }
        self.dirty.remove(&id);
        self.dirty_effects.remove(&id);
        if matches!(node.kind, NodeKind::Watch { .. }) {
            self.watches.retain(|w| *w != id);
        }
        Some(node)
    }

    /// Marks the readers of watched components which changed since they were last looked at.
    pub fn poll_watches(&mut self, world: &World) {
        let mut changed = Vec::new();
        for id in &self.watches {
            let Some(GraphNode {
                kind:
                    NodeKind::Watch {
                        entity,
                        component,
                        last_changed,
                        ..
                    },
                ..
            }) = self.nodes.get_mut(id)
            else {
                continue;
            };
            let now = world
                .get_entity(*entity)
                .ok()
                .and_then(|e| e.get_change_ticks_by_id(*component))
                .map(|ticks| ticks.changed);
            if now != *last_changed {
                *last_changed = now;
                changed.push(*id);
            }
        }
        for id in changed {
            self.mark_subs_dirty(id);
        }
    }

    /// Forgets everything owned by the given script, i.e. once it is unloaded, returning the root entities to despawn.
    pub fn forget_attachment(&mut self, attachment: &ScriptAttachment) -> Vec<Entity> {
        let roots = self
            .roots
            .keys()
            .filter(|(a, _)| a == attachment)
            .cloned()
            .collect::<Vec<_>>();
        let containers = roots
            .into_iter()
            .filter_map(|key| self.roots.remove(&key))
            .map(|root| root.container)
            .collect();
        let owned = self
            .nodes
            .iter()
            .filter(|(_, n)| n.attachment == *attachment)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        for id in owned {
            self.remove(id);
        }
        self.events.retain(|e| e.attachment != *attachment);
        self.def_index.retain(|(a, _), _| a != attachment);
        containers
    }
}

/// What the script function currently running is a part of.
#[derive(Clone, Copy)]
pub(crate) struct Frame {
    /// The node recording what is read, if any
    pub observer: Option<NodeId>,
    /// The component instance whose slots `signal`, `effect` etc. use, and the next slot
    pub instance: Option<(NodeId, usize)>,
}

thread_local! {
    static CURRENT_RUNTIME: RefCell<Option<Arc<Mutex<Runtime>>>> = const { RefCell::new(None) };
    static FRAMES: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
}

/// Makes the runtime available to script functions without a world lookup while flushing.
pub(crate) struct CurrentRuntime(Option<Arc<Mutex<Runtime>>>);

impl CurrentRuntime {
    pub fn set(runtime: Arc<Mutex<Runtime>>) -> Self {
        Self(CURRENT_RUNTIME.with(|c| c.borrow_mut().replace(runtime)))
    }
}

impl Drop for CurrentRuntime {
    fn drop(&mut self) {
        let previous = self.0.take();
        CURRENT_RUNTIME.with(|c| *c.borrow_mut() = previous);
    }
}

/// The runtime of the world script functions run in.
pub(crate) fn runtime(ctx: &FunctionCallContext) -> Result<Arc<Mutex<Runtime>>, InteropError> {
    if let Some(runtime) = CURRENT_RUNTIME.with(|c| c.borrow().clone()) {
        return Ok(runtime);
    }
    ctx.world()?
        .with_world_access(|w| w.get_resource::<ReactiveUi>().map(|r| r.0.clone()))?
        .ok_or_else(|| InteropError::str("the reactive ui plugin is not installed"))
}

pub(crate) fn current_frame() -> Option<Frame> {
    FRAMES.with(|f| f.borrow().last().copied())
}

/// Takes the next slot of the instance being rendered, if any.
pub(crate) fn next_slot() -> Option<(NodeId, usize)> {
    FRAMES.with(|f| {
        let mut frames = f.borrow_mut();
        let frame = frames.last_mut()?;
        let (instance, index) = frame.instance.as_mut()?;
        let slot = (*instance, *index);
        *index += 1;
        Some(slot)
    })
}

struct PopFrame;

impl Drop for PopFrame {
    fn drop(&mut self) {
        FRAMES.with(|f| f.borrow_mut().pop());
    }
}

pub(crate) fn with_frame<R>(frame: Frame, f: impl FnOnce() -> R) -> R {
    FRAMES.with(|f| f.borrow_mut().push(frame));
    let _pop = PopFrame;
    f()
}

/// Finds or creates the node of the next slot of the instance being rendered.
///
/// `reuse` decides whether an existing node can be kept, and may update it (i.e. with the new function).
pub(crate) fn slot_node(
    runtime: &Mutex<Runtime>,
    attachment: ScriptAttachment,
    reuse: impl FnOnce(&mut NodeKind) -> bool,
    create: impl FnOnce() -> NodeKind,
) -> NodeId {
    let mut rt = runtime.lock();
    let Some((instance, index)) = next_slot() else {
        return rt.create(create(), attachment);
    };
    let existing = rt
        .instance_mut(instance)
        .and_then(|i| i.slots.get(index).copied());
    if let Some(id) = existing
        && let Some(node) = rt.nodes.get_mut(&id)
        && reuse(&mut node.kind)
    {
        return id;
    }
    let id = rt.create(create(), attachment);
    if let Some(old) = existing {
        rt.remove(old);
    }
    if let Some(instance) = rt.instance_mut(instance) {
        if index < instance.slots.len() {
            instance.slots[index] = id;
        } else {
            instance.slots.push(id);
        }
    }
    id
}

/// Reads a node, recording the read for the running observer.
pub(crate) fn read(
    runtime: &Mutex<Runtime>,
    ctx: &FunctionCallContext,
    id: NodeId,
    track: bool,
) -> Result<ScriptValue, InteropError> {
    let observer = if track {
        current_frame().and_then(|f| f.observer)
    } else {
        None
    };
    let mut rt = runtime.lock();
    if let Some(observer) = observer {
        rt.link(observer, id);
    }
    let node = rt
        .nodes
        .get_mut(&id)
        .ok_or_else(|| InteropError::str("this signal no longer exists"))?;
    match &node.kind {
        NodeKind::Signal { value } => Ok(value.clone()),
        NodeKind::Computed {
            value: Some(value), ..
        } => Ok(value.clone()),
        NodeKind::Computed { f, value: None } => {
            let f = f.clone();
            node.dirty = false;
            rt.clear_deps(id);
            drop(rt);
            let value = with_frame(
                Frame {
                    observer: Some(id),
                    instance: None,
                },
                || f.call(vec![], ctx.clone()),
            )?;
            if let Some(GraphNode {
                kind: NodeKind::Computed { value: slot, .. },
                ..
            }) = runtime.lock().nodes.get_mut(&id)
            {
                *slot = Some(value.clone());
            }
            Ok(value)
        }
        NodeKind::Watch {
            entity,
            component,
            type_id,
            ..
        } => {
            let (entity, component, type_id) = (*entity, *component, *type_id);
            drop(rt);
            let present = ctx.world()?.with_world_access(|w| {
                w.get_entity(entity).is_ok_and(|e| e.contains_id(component))
            })?;
            if !present {
                return Ok(ScriptValue::Unit);
            }
            // Safety: the type id is the type of the component, both come from its registration
            Ok(ScriptValue::Reference(unsafe {
                ReflectReference::new_component_ref_by_id(entity, component, type_id)
            }))
        }
        _ => Err(InteropError::str("this handle can not be read")),
    }
}

/// Sets a signal, marking what read it as dirty.
pub(crate) fn write(
    runtime: &Mutex<Runtime>,
    id: NodeId,
    value: ScriptValue,
) -> Result<(), InteropError> {
    let mut rt = runtime.lock();
    match rt.nodes.get_mut(&id).map(|n| &mut n.kind) {
        Some(NodeKind::Signal { value: current }) => {
            if script_values_equal(current, &value) {
                return Ok(());
            }
            *current = value;
        }
        Some(_) => return Err(InteropError::str("only signals can be set")),
        None => return Err(InteropError::str("this signal no longer exists")),
    }
    rt.mark_subs_dirty(id);
    Ok(())
}

/// Everything needed to run script functions and touch the world while flushing one script's work.
pub(crate) struct Cx<'a, 'w> {
    pub runtime: &'a Arc<Mutex<Runtime>>,
    pub world: WorldGuard<'w>,
    pub attachment: &'a ScriptAttachment,
    pub language: Language,
}

impl Cx<'_, '_> {
    fn call(&self, f: &Func, args: Vec<ScriptValue>) -> Result<ScriptValue, InteropError> {
        f.call(args, FunctionCallContext::new(self.language.clone()))
    }

    fn world_mut<O>(&self, f: impl FnOnce(&mut World) -> O) -> Result<O, InteropError> {
        Ok(self.world.with_world_mut_access(f)?)
    }

    fn spawn(&self, kind: &ElementKind, parent: Entity) -> Result<Entity, InteropError> {
        self.world_mut(|w| match kind {
            ElementKind::Text => w
                .spawn((Node::default(), Text::default(), ChildOf(parent)))
                .id(),
            _ => w.spawn((Node::default(), ChildOf(parent))).id(),
        })
    }

    fn apply(&self, entity: Entity, prop: Prop, value: &ScriptValue) -> Result<(), InteropError> {
        self.world_mut(|w| match w.get_entity_mut(entity) {
            Ok(mut e) => apply_prop(&mut e, prop, value),
            Err(_) => Ok(()),
        })?
    }

    fn set_handlers(
        &self,
        entity: Entity,
        props: &[(Prop, PropValue)],
    ) -> Result<(), InteropError> {
        let mut click = None;
        let mut hover = None;
        for (prop, value) in props {
            if let PropValue::Handler(f) = value {
                match prop {
                    Prop::OnClick => click = Some(f.clone()),
                    Prop::OnHover => hover = Some(f.clone()),
                    _ => {}
                }
            }
        }
        let attachment = self.attachment.clone();
        self.world_mut(|w| {
            let Ok(mut e) = w.get_entity_mut(entity) else {
                return;
            };
            if click.is_none() && hover.is_none() {
                e.remove::<UiHandlers>();
            } else {
                e.insert((
                    UiHandlers {
                        attachment,
                        click,
                        hover,
                    },
                    Interaction::default(),
                ));
            }
        })
    }

    fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        self.world_mut(|w| {
            if let Ok(e) = w.get_entity_mut(entity) {
                e.despawn();
            }
        })
    }

    /// The entity an element is mounted as.
    fn entity_of(&self, mounted: &Mounted) -> Option<Entity> {
        match mounted {
            Mounted::Host { entity, .. } | Mounted::Dynamic { entity, .. } => Some(*entity),
            Mounted::Component { instance, .. } => {
                // taken out so that nested components can be looked at without holding the lock
                let rendered = self
                    .runtime
                    .lock()
                    .instance_mut(*instance)?
                    .rendered
                    .take()?;
                let entity = self.entity_of(&rendered);
                if let Some(instance) = self.runtime.lock().instance_mut(*instance) {
                    instance.rendered = Some(rendered);
                }
                entity
            }
        }
    }

    /// Puts the parent's children in the given order, if they are not already.
    fn order_children(&self, parent: Entity, order: &[Entity]) -> Result<(), InteropError> {
        self.world_mut(|w| {
            let current = w.get::<Children>(parent).map(|c| c.to_vec());
            if current.as_deref() != Some(order)
                && let Ok(mut parent) = w.get_entity_mut(parent)
            {
                parent.replace_children(order);
            }
        })
    }

    /// Mounts an element as the last child of `parent`.
    pub fn mount(&self, element: &UiElement, parent: Entity) -> Result<Mounted, InteropError> {
        let data = &element.0;
        match &data.kind {
            ElementKind::Node | ElementKind::Text => {
                let entity = self.spawn(&data.kind, parent)?;
                let mut bindings = Vec::new();
                for (prop, value) in &data.props {
                    match value {
                        PropValue::Static(v) => self.apply(entity, *prop, v)?,
                        PropValue::Bound(f) => {
                            bindings.push((*prop, self.create_binding(entity, *prop, f.clone())?))
                        }
                        PropValue::Handler(_) => {}
                    }
                }
                if data
                    .props
                    .iter()
                    .any(|(_, v)| matches!(v, PropValue::Handler(_)))
                {
                    self.set_handlers(entity, &data.props)?;
                }
                let children = data
                    .children
                    .iter()
                    .map(|child| self.mount(child, entity))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Mounted::Host {
                    entity,
                    element: element.clone(),
                    children,
                    bindings,
                })
            }
            ElementKind::Component(def, props) => {
                let instance = self.runtime.lock().create(
                    NodeKind::Instance(Instance {
                        def: *def,
                        props: props.clone(),
                        slots: Vec::new(),
                        rendered: None,
                        parent,
                    }),
                    self.attachment.clone(),
                );
                self.render_instance(instance)?;
                Ok(Mounted::Component {
                    instance,
                    key: data.key.clone(),
                })
            }
            ElementKind::Dynamic(f) => {
                let entity = self.spawn(&ElementKind::Node, parent)?;
                let region = self.runtime.lock().create(
                    NodeKind::Region {
                        container: entity,
                        f: f.clone(),
                        children: Vec::new(),
                    },
                    self.attachment.clone(),
                );
                self.run_region(region)?;
                Ok(Mounted::Dynamic { region, entity })
            }
        }
    }

    fn create_binding(&self, entity: Entity, prop: Prop, f: Func) -> Result<NodeId, InteropError> {
        let id = self.runtime.lock().create(
            NodeKind::Binding { entity, prop, f },
            self.attachment.clone(),
        );
        self.run_binding(id)?;
        Ok(id)
    }

    pub fn run_binding(&self, id: NodeId) -> Result<(), InteropError> {
        let (entity, prop, f) = {
            let mut rt = self.runtime.lock();
            rt.clear_deps(id);
            match rt.nodes.get_mut(&id) {
                Some(GraphNode {
                    kind: NodeKind::Binding { entity, prop, f },
                    dirty,
                    ..
                }) => {
                    *dirty = false;
                    (*entity, *prop, f.clone())
                }
                _ => return Ok(()),
            }
        };
        let value = with_frame(
            Frame {
                observer: Some(id),
                instance: None,
            },
            || self.call(&f, vec![]),
        )?;
        self.apply(entity, prop, &value)
    }

    pub fn run_region(&self, id: NodeId) -> Result<(), InteropError> {
        let (container, f, old) = {
            let mut rt = self.runtime.lock();
            rt.clear_deps(id);
            match rt.nodes.get_mut(&id) {
                Some(GraphNode {
                    kind:
                        NodeKind::Region {
                            container,
                            f,
                            children,
                        },
                    dirty,
                    ..
                }) => {
                    *dirty = false;
                    (*container, f.clone(), std::mem::take(children))
                }
                _ => return Ok(()),
            }
        };
        let result = with_frame(
            Frame {
                observer: Some(id),
                instance: None,
            },
            || self.call(&f, vec![]),
        )
        .and_then(|v| elements_from_script(&v, &self.world));
        let elements = match result {
            Ok(elements) => elements,
            Err(e) => {
                self.put_region_children(id, old);
                return Err(e);
            }
        };
        let children = self.reconcile(old, &elements, container)?;
        self.put_region_children(id, children);
        Ok(())
    }

    fn put_region_children(&self, id: NodeId, new: Vec<Mounted>) {
        if let Some(GraphNode {
            kind: NodeKind::Region { children, .. },
            ..
        }) = self.runtime.lock().nodes.get_mut(&id)
        {
            *children = new;
        }
    }

    pub fn run_effect(&self, id: NodeId) -> Result<(), InteropError> {
        let (f, cleanup) = {
            let mut rt = self.runtime.lock();
            rt.clear_deps(id);
            match rt.nodes.get_mut(&id) {
                Some(GraphNode {
                    kind: NodeKind::Effect { f, cleanup },
                    dirty,
                    ..
                }) => {
                    *dirty = false;
                    (f.clone(), cleanup.take())
                }
                _ => return Ok(()),
            }
        };
        if let Some(cleanup) = cleanup {
            self.call(&cleanup, vec![])?;
        }
        let result = with_frame(
            Frame {
                observer: Some(id),
                instance: None,
            },
            || self.call(&f, vec![]),
        )?;
        if let ScriptValue::Function(new_cleanup) = result
            && let Some(GraphNode {
                kind: NodeKind::Effect { cleanup, .. },
                ..
            }) = self.runtime.lock().nodes.get_mut(&id)
        {
            *cleanup = Some(new_cleanup);
        }
        Ok(())
    }

    /// Runs the component's render function and mounts or diffs its output.
    pub fn render_instance(&self, id: NodeId) -> Result<(), InteropError> {
        let (render, props, old, parent) = {
            let mut rt = self.runtime.lock();
            let def = match rt.nodes.get(&id) {
                Some(GraphNode {
                    kind: NodeKind::Instance(instance),
                    ..
                }) => instance.def,
                _ => return Ok(()),
            };
            let Some(Def { render }) = rt.defs.get(def.0 as usize) else {
                return Err(InteropError::str("unknown component definition"));
            };
            let render = render.clone();
            let Some(node) = rt.nodes.get_mut(&id) else {
                return Ok(());
            };
            node.dirty = false;
            let NodeKind::Instance(instance) = &mut node.kind else {
                return Ok(());
            };
            (
                render,
                instance.props.clone(),
                instance.rendered.take(),
                instance.parent,
            )
        };
        self.runtime.lock().dirty.remove(&id);
        let result = with_frame(
            Frame {
                observer: None,
                instance: Some((id, 0)),
            },
            || self.call(&render, vec![props]),
        )
        .and_then(|v| element_from_script(&v, &self.world));
        let element = match result {
            Ok(Some(element)) => element,
            Ok(None) => crate::element::empty_node(),
            Err(e) => {
                // keep showing the previous render
                if let Some(instance) = self.runtime.lock().instance_mut(id) {
                    instance.rendered = old;
                }
                return Err(e);
            }
        };
        let rendered = match old {
            Some(old) => self.patch(old, &element, parent)?,
            None => self.mount(&element, parent)?,
        };
        if let Some(instance) = self.runtime.lock().instance_mut(id) {
            instance.rendered = Some(rendered);
        }
        Ok(())
    }

    fn same_kind(&self, mounted: &Mounted, element: &UiElement) -> bool {
        match (mounted, &element.0.kind) {
            (Mounted::Host { element: old, .. }, kind) => old.0.kind.same_kind(kind),
            (Mounted::Component { instance, .. }, ElementKind::Component(def, _)) => self
                .runtime
                .lock()
                .instance_mut(*instance)
                .is_some_and(|i| i.def == *def),
            (Mounted::Dynamic { .. }, ElementKind::Dynamic(_)) => true,
            _ => false,
        }
    }

    /// Updates what is mounted to match the element, keeping entities and component state where possible.
    pub fn patch(
        &self,
        old: Mounted,
        element: &UiElement,
        parent: Entity,
    ) -> Result<Mounted, InteropError> {
        if !self.same_kind(&old, element) {
            let old_entity = self.entity_of(&old);
            let new = self.mount(element, parent)?;
            if let (Some(old_entity), Some(new_entity)) = (old_entity, self.entity_of(&new)) {
                self.swap_child(parent, old_entity, new_entity)?;
            }
            self.unmount(old, true)?;
            return Ok(new);
        }
        match old {
            Mounted::Host {
                entity,
                element: old_element,
                children,
                mut bindings,
            } => {
                let new = &element.0;
                for (prop, old_value) in &old_element.0.props {
                    if !new.props.iter().any(|(p, _)| p == prop) {
                        if let PropValue::Bound(_) = old_value {
                            self.drop_binding(&mut bindings, *prop);
                        }
                        if !matches!(old_value, PropValue::Handler(_)) {
                            self.apply(entity, *prop, &ScriptValue::Unit)?;
                        }
                    }
                }
                for (prop, value) in &new.props {
                    let old_value = old_element
                        .0
                        .props
                        .iter()
                        .find(|(p, _)| p == prop)
                        .map(|(_, v)| v);
                    match value {
                        PropValue::Static(v) => {
                            if let Some(PropValue::Static(old)) = old_value
                                && script_values_equal(old, v)
                            {
                                continue;
                            }
                            self.drop_binding(&mut bindings, *prop);
                            self.apply(entity, *prop, v)?;
                        }
                        PropValue::Bound(f) => {
                            match bindings.iter().find(|(p, _)| p == prop).map(|(_, id)| *id) {
                                Some(id) => {
                                    if let Some(GraphNode {
                                        kind: NodeKind::Binding { f: old_f, .. },
                                        ..
                                    }) = self.runtime.lock().nodes.get_mut(&id)
                                    {
                                        *old_f = f.clone();
                                    }
                                    self.run_binding(id)?;
                                }
                                None => bindings
                                    .push((*prop, self.create_binding(entity, *prop, f.clone())?)),
                            }
                        }
                        PropValue::Handler(_) => {}
                    }
                }
                let had_handlers = old_element
                    .0
                    .props
                    .iter()
                    .any(|(_, v)| matches!(v, PropValue::Handler(_)));
                if had_handlers
                    || new
                        .props
                        .iter()
                        .any(|(_, v)| matches!(v, PropValue::Handler(_)))
                {
                    self.set_handlers(entity, &new.props)?;
                }
                let children = self.reconcile(children, &new.children, entity)?;
                Ok(Mounted::Host {
                    entity,
                    element: element.clone(),
                    children,
                    bindings,
                })
            }
            Mounted::Component { instance, .. } => {
                let ElementKind::Component(_, props) = &element.0.kind else {
                    unreachable!("checked by same_kind")
                };
                // components run once, new props are seen when they run again (i.e. on reload)
                let rerender = {
                    let mut rt = self.runtime.lock();
                    let dirty = rt.nodes.get(&instance).is_some_and(|n| n.dirty);
                    if let Some(i) = rt.instance_mut(instance) {
                        i.props = props.clone();
                    }
                    dirty
                };
                if rerender {
                    self.render_instance(instance)?;
                }
                Ok(Mounted::Component {
                    instance,
                    key: element.0.key.clone(),
                })
            }
            Mounted::Dynamic { region, entity } => {
                let ElementKind::Dynamic(f) = &element.0.kind else {
                    unreachable!("checked by same_kind")
                };
                if let Some(GraphNode {
                    kind: NodeKind::Region { f: old_f, .. },
                    ..
                }) = self.runtime.lock().nodes.get_mut(&region)
                {
                    *old_f = f.clone();
                }
                self.run_region(region)?;
                Ok(Mounted::Dynamic { region, entity })
            }
        }
    }

    fn drop_binding(&self, bindings: &mut Vec<(Prop, NodeId)>, prop: Prop) {
        if let Some(index) = bindings.iter().position(|(p, _)| *p == prop) {
            let (_, id) = bindings.remove(index);
            self.runtime.lock().remove(id);
        }
    }

    /// Moves `new` (the last child) into the place of `old`.
    fn swap_child(&self, parent: Entity, old: Entity, new: Entity) -> Result<(), InteropError> {
        let order = self.world_mut(|w| {
            w.get::<Children>(parent).map(|children| {
                children
                    .iter()
                    .copied()
                    .filter(|c| *c != new)
                    .map(|c| if c == old { new } else { c })
                    .collect::<Vec<_>>()
            })
        })?;
        match order {
            Some(order) => self.order_children(parent, &order),
            None => Ok(()),
        }
    }

    /// Matches children by key, or by position among the unkeyed ones, patching matches and mounting the rest.
    pub fn reconcile(
        &self,
        old: Vec<Mounted>,
        new: &[UiElement],
        parent: Entity,
    ) -> Result<Vec<Mounted>, InteropError> {
        let mut keyed: HashMap<String, Mounted> = HashMap::default();
        let mut unkeyed = VecDeque::new();
        for mounted in old {
            match mounted.key() {
                Some(key) => {
                    keyed.insert(key.to_owned(), mounted);
                }
                None => unkeyed.push_back(mounted),
            }
        }
        let mut result = Vec::with_capacity(new.len());
        for element in new {
            let previous = match &element.0.key {
                Some(key) => keyed.remove(key),
                None => unkeyed.pop_front(),
            };
            result.push(match previous {
                Some(previous) => self.patch(previous, element, parent)?,
                None => self.mount(element, parent)?,
            });
        }
        for leftover in keyed.into_values().chain(unkeyed) {
            self.unmount(leftover, true)?;
        }
        let order = result
            .iter()
            .filter_map(|m| self.entity_of(m))
            .collect::<Vec<_>>();
        self.order_children(parent, &order)?;
        Ok(result)
    }

    /// Removes what is mounted, running effect cleanups, and despawns its entity if `despawn` is set.
    ///
    /// Descendants are not despawned one by one, despawning the top entity despawns them.
    pub fn unmount(&self, mounted: Mounted, despawn: bool) -> Result<(), InteropError> {
        match mounted {
            Mounted::Host {
                entity,
                children,
                bindings,
                ..
            } => {
                {
                    let mut rt = self.runtime.lock();
                    for (_, id) in bindings {
                        rt.remove(id);
                    }
                }
                for child in children {
                    self.unmount(child, false)?;
                }
                if despawn {
                    self.despawn(entity)?;
                }
            }
            Mounted::Component { instance, .. } => self.dispose_instance(instance, despawn)?,
            Mounted::Dynamic { region, entity } => {
                let node = self.runtime.lock().remove(region);
                if let Some(GraphNode {
                    kind: NodeKind::Region { children, .. },
                    ..
                }) = node
                {
                    for child in children {
                        self.unmount(child, false)?;
                    }
                }
                if despawn {
                    self.despawn(entity)?;
                }
            }
        }
        Ok(())
    }

    pub fn dispose_instance(&self, instance: NodeId, despawn: bool) -> Result<(), InteropError> {
        let node = self.runtime.lock().remove(instance);
        let Some(GraphNode {
            kind: NodeKind::Instance(instance),
            ..
        }) = node
        else {
            return Ok(());
        };
        if let Some(rendered) = instance.rendered {
            self.unmount(rendered, despawn)?;
        }
        for slot in instance.slots {
            let node = self.runtime.lock().remove(slot);
            if let Some(GraphNode {
                kind:
                    NodeKind::Effect {
                        cleanup: Some(cleanup),
                        ..
                    },
                ..
            }) = node
            {
                self.call(&cleanup, vec![])?;
            }
        }
        Ok(())
    }
}
