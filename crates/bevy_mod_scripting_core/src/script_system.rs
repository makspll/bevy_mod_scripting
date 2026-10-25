//! everything to do with dynamically added script systems

use crate::{
    IntoScriptPluginParams,
    callbacks::ScriptCallbacks,
    error::ScriptError,
    event::{CallbackLabel, ScriptErrorMessage},
    extractors::get_all_access_ids,
    handler::ScriptingHandler,
    script::ScriptContexts,
};

use ::{
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        query::{AccessConflicts, FilteredAccessSet, QueryBuilder, QueryState},
        reflect::AppTypeRegistry,
        schedule::SystemSet,
        system::{System, SystemParamValidationError},
        world::{World, unsafe_world_cell::UnsafeWorldCell},
    },
    bevy_reflect::Reflect,
};
use bevy_ecs::{
    archetype::ArchetypeGeneration,
    change_detection::{CheckChangeTicks, Tick},
    message::Messages,
    resource::Resource,
    schedule::{
        InternedScheduleLabel, InternedSystemSet, IntoScheduleConfigs, Schedule,
        ScheduleCleanupPolicy, ScheduleConfigs, ScheduleError, Schedules,
    },
    system::{
        ReadOnlySystem, RunSystemError, ScheduleSystem, SystemAccess, SystemIn, SystemStateFlags,
    },
    world::{DeferredWorld, WorldId},
};
use bevy_log::{debug, warn, warn_once};
use bevy_mod_scripting_bindings::{
    AppScheduleRegistry, AppScriptCallbackRunners, AppScriptComponentRegistry,
    AppScriptFunctionRegistry, AppScriptTypeCatalogue, CurrentScriptAttachment, InteropError,
    IntoScript, QueryAccess, ReflectReference, ScriptComponentRegistration, ScriptQueryBuilder,
    ScriptResourceRegistration, V, WorldExtensions,
};
use bevy_mod_scripting_display::DisplayProxy;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::{AccessByteSet, ArchetypeFilter, WorldAccessGuard, WorldGuard};
use bevy_platform::collections::HashMap;
use bevy_reflect::TypeRegistryArc;
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};
use bevy_time::{Time, Timer, TimerMode};
use bevy_utils::prelude::DebugName;
use core::time::Duration;
use parking_lot::Mutex;
use std::{
    any::TypeId,
    borrow::Cow,
    collections::HashSet,
    hash::Hash,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
/// A system set containing a single script system, identified by its slot in [`ScriptSystems`].
///
/// System sets are interned, which leaks every distinct set value for the lifetime of the program,
/// so slots are recycled once their system is removed, bounding the leak by the peak number of live script systems.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ScriptSystemSet(u32);

impl std::fmt::Debug for ScriptSystemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScriptSystem({})", self.0)
    }
}

#[profiling::all_functions]
impl ScriptSystemSet {
    /// The slot of the set in [`ScriptSystems`]
    pub fn slot(&self) -> u32 {
        self.0
    }
}

#[profiling::all_functions]
impl SystemSet for ScriptSystemSet {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(*self)
    }
}

#[derive(Clone)]
enum ScriptSystemParamDescriptor {
    Res {
        resource: ScriptResourceRegistration,
        write: bool,
    },
    EntityQuery(ScriptQueryBuilder),
}

/// A builder for systems living in scripts
#[derive(Reflect, Clone)]
#[reflect(opaque)]
pub struct ScriptSystemBuilder {
    pub(crate) name: CallbackLabel,
    pub(crate) attachment: ScriptAttachment,
    before: Vec<ReflectSystem>,
    after: Vec<ReflectSystem>,
    before_script_systems: Vec<(CallbackLabel, ScriptAttachment)>,
    after_script_systems: Vec<(CallbackLabel, ScriptAttachment)>,
    before_sets: Vec<InternedSystemSet>,
    after_sets: Vec<InternedSystemSet>,
    in_sets: Vec<InternedSystemSet>,
    run_conditions: Vec<ScriptRunCondition>,
    system_params: Vec<ScriptSystemParamDescriptor>,
    is_exclusive: bool,
}

#[profiling::all_functions]
impl ScriptSystemBuilder {
    /// Creates a new script system builder
    pub fn new(name: CallbackLabel, attachment: ScriptAttachment) -> Self {
        Self {
            before: vec![],
            after: vec![],
            before_script_systems: vec![],
            after_script_systems: vec![],
            before_sets: vec![],
            after_sets: vec![],
            in_sets: vec![],
            run_conditions: vec![],
            name,
            attachment,
            system_params: vec![],
            is_exclusive: false,
        }
    }

    /// Runs the system before the script system of the given script, which must have been added already
    pub fn before_script_system(
        &mut self,
        label: CallbackLabel,
        attachment: ScriptAttachment,
    ) -> &mut Self {
        self.before_script_systems.push((label, attachment));
        self
    }

    /// Runs the system after the script system of the given script, which must have been added already
    pub fn after_script_system(
        &mut self,
        label: CallbackLabel,
        attachment: ScriptAttachment,
    ) -> &mut Self {
        self.after_script_systems.push((label, attachment));
        self
    }

    /// Runs the system before the given set
    pub fn before_set(&mut self, set: impl SystemSet) -> &mut Self {
        self.before_sets.push(set.intern());
        self
    }

    /// Runs the system after the given set
    pub fn after_set(&mut self, set: impl SystemSet) -> &mut Self {
        self.after_sets.push(set.intern());
        self
    }

    /// Puts the system in the given set
    pub fn in_set(&mut self, set: impl SystemSet) -> &mut Self {
        self.in_sets.push(set.intern());
        self
    }

    /// Only runs the system if the condition holds, all conditions must hold for the system to run
    pub fn run_if(&mut self, condition: ScriptRunCondition) -> &mut Self {
        self.run_conditions.push(condition);
        self
    }

    /// Adds a component access to the system
    pub fn query(&mut self, query: ScriptQueryBuilder) -> &mut Self {
        self.system_params
            .push(ScriptSystemParamDescriptor::EntityQuery(query));
        self
    }

    /// Adds a resource access to the system, the resource can be read and written
    pub fn resource(&mut self, resource: ScriptResourceRegistration) -> &mut Self {
        self.system_params.push(ScriptSystemParamDescriptor::Res {
            resource,
            write: true,
        });
        self
    }

    /// Adds a read only resource access to the system, systems reading a resource can run in parallel with each other
    pub fn resource_read(&mut self, resource: ScriptResourceRegistration) -> &mut Self {
        self.system_params.push(ScriptSystemParamDescriptor::Res {
            resource,
            write: false,
        });
        self
    }

    /// Sets the system to be exclusive, i.e. it will be able to access everything but cannot be parallelized.
    pub fn exclusive(&mut self, exclusive: bool) -> &mut Self {
        self.is_exclusive = exclusive;
        self
    }

    /// Adds a system to run before the script system
    pub fn before_system(&mut self, system: ReflectSystem) -> &mut Self {
        self.before.push(system);
        self
    }

    /// Adds a system to run after the script system
    pub fn after_system(&mut self, system: ReflectSystem) -> &mut Self {
        self.after.push(system);
        self
    }

    /// Builds the system and inserts it into the given schedule.
    ///
    /// Replaces the system with the same attachment and callback label if one exists, see [`ScriptSystems`].
    /// If the schedule is running, the system is added once it finishes, and the returned system has no node id.
    #[allow(deprecated)]
    pub fn build<P: IntoScriptPluginParams>(
        self,
        world: WorldGuard,
        schedule: &ReflectSchedule,
    ) -> Result<ReflectSystem, InteropError> {
        world.with_world_mut(|world| {
            let systems = world.get_resource_or_init::<ScriptSystems>().clone();
            let label = *schedule.label();
            let alive = Arc::new(AtomicBool::new(true));
            let mut changes = Vec::with_capacity(2);

            let mut ordering = ScriptSystemOrdering {
                before: self
                    .before
                    .iter()
                    .flat_map(|system| system.default_system_sets().iter().copied())
                    .chain(self.before_sets.iter().copied())
                    .collect(),
                after: self
                    .after
                    .iter()
                    .flat_map(|system| system.default_system_sets().iter().copied())
                    .chain(self.after_sets.iter().copied())
                    .collect(),
                in_sets: self.in_sets.clone(),
            };

            let set = {
                let mut inner = systems.0.lock();
                for (script_systems, sets) in [
                    (&self.before_script_systems, &mut ordering.before),
                    (&self.after_script_systems, &mut ordering.after),
                ] {
                    for (label, attachment) in script_systems {
                        let entry = inner
                            .systems
                            .get(&(attachment.clone(), label.clone()))
                            .ok_or_else(|| {
                                InteropError::string(format!(
                                    "Script system `{label}` of script {attachment} does not exist, script systems can only be ordered against systems added before them"
                                ))
                            })?;
                        sets.push(entry.set.intern());
                    }
                }
                let key = (self.attachment.clone(), self.name.clone());
                let set = match inner.systems.get(&key) {
                    Some(old) => {
                        // a dead one is a tombstone of a removed system, see `remove_systems_of`
                        if old.alive.swap(false, Ordering::Relaxed) {
                            warn!(
                                "Script system `{}` of script {} already exists, replacing it",
                                self.name, self.attachment
                            );
                        }
                        // keep the set, so systems ordered relative to the old system stay ordered relative to the new one
                        changes.push(PendingScheduleChange::new(
                            old.schedule,
                            ScheduleChange::Remove {
                                set: old.set,
                                free_slot: false,
                            },
                        ));
                        old.set
                    }
                    None => inner.allocate_set(),
                };
                inner.systems.insert(
                    key,
                    ScriptSystemEntry {
                        set,
                        schedule: label,
                        alive: alive.clone(),
                        orders_against: ordering
                            .before
                            .iter()
                            .chain(&ordering.after)
                            .copied()
                            .collect(),
                    },
                );
                set
            };

            let unscheduled = ReflectSystem::from_unscheduled_system(
                &DynamicScriptSystem::<P>::new(self.clone(), set, alive.clone(), systems.clone()),
            );
            changes.push(PendingScheduleChange::new(
                label,
                ScheduleChange::Add {
                    builder: Box::new(self.clone()),
                    set,
                    alive,
                    ordering,
                    // schedule configs are not `Send`, so the system is created once it is added
                    into_configs: script_system_configs::<P>,
                },
            ));

            if !systems.apply_or_queue(world, changes) {
                debug!(
                    "Schedule `{}` is running, script system `{}` will be added once it finishes",
                    schedule.identifier(),
                    self.name
                );
                return Ok(unscheduled);
            }

            // initialize the schedule to find the node id of the new system
            let Some(found) = with_schedule(world, label, |world, schedule| {
                schedule.initialize(world).map_err(InteropError::external)?;
                let set = set.intern();
                Ok::<_, InteropError>(
                    schedule
                        .systems()
                        .map_err(InteropError::external)?
                        .find(|(_, system)| system.default_system_sets().contains(&set))
                        .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id)),
                )
            }) else {
                return Ok(unscheduled);
            };
            found?.ok_or_else(|| InteropError::invariant("After adding the system, it was not found in the schedule, could not return a reference to it"))
        })?
    }
}

/// A run condition of a script system, evaluated in Rust without calling into the script, see [`ScriptSystemBuilder::run_if`].
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub enum ScriptRunCondition {
    /// The resource exists
    ResourceExists(ScriptResourceRegistration),
    /// The resource was changed since the system last ran, false if it doesn't exist
    ResourceChanged(ScriptResourceRegistration),
    /// The resource was added since the system last ran
    ResourceAdded(ScriptResourceRegistration),
    /// Any entity has the component
    AnyWithComponent(ScriptComponentRegistration),
    /// The given time passed since the condition last held, measured with [`Time`]
    OnTimer(Duration),
    /// Only the first time the condition is evaluated
    RunOnce,
    /// The condition doesn't hold
    Not(Box<ScriptRunCondition>),
    /// Both conditions hold, the second is not evaluated if the first doesn't hold
    And(Box<ScriptRunCondition>, Box<ScriptRunCondition>),
    /// Either condition holds, the second is not evaluated if the first holds
    Or(Box<ScriptRunCondition>, Box<ScriptRunCondition>),
}

/// A [`ScriptRunCondition`] ready to be evaluated, with its state
enum CompiledRunCondition {
    ResourceExists(ComponentId),
    ResourceChanged(ComponentId),
    ResourceAdded(ComponentId),
    AnyWithComponent(Box<QueryState<Entity>>),
    OnTimer(Timer, ComponentId),
    RunOnce(bool),
    Not(Box<CompiledRunCondition>),
    And(Box<CompiledRunCondition>, Box<CompiledRunCondition>),
    Or(Box<CompiledRunCondition>, Box<CompiledRunCondition>),
}

impl CompiledRunCondition {
    /// Compiles the condition, declaring the accesses needed to evaluate it
    fn compile(
        condition: &ScriptRunCondition,
        world: &mut World,
        access: &mut FilteredAccessSet,
    ) -> Self {
        match condition {
            ScriptRunCondition::ResourceExists(resource) => {
                access.add_resource_read(resource.resource_id);
                Self::ResourceExists(resource.resource_id)
            }
            ScriptRunCondition::ResourceChanged(resource) => {
                access.add_resource_read(resource.resource_id);
                Self::ResourceChanged(resource.resource_id)
            }
            ScriptRunCondition::ResourceAdded(resource) => {
                access.add_resource_read(resource.resource_id);
                Self::ResourceAdded(resource.resource_id)
            }
            ScriptRunCondition::AnyWithComponent(component) => {
                let query = QueryBuilder::<Entity>::new(world)
                    .with_id(component.component_id)
                    .build();
                access.add(query.component_access().clone());
                Self::AnyWithComponent(Box::new(query))
            }
            ScriptRunCondition::OnTimer(duration) => {
                // resources are components, registering keeps the id valid if `Time` is inserted later
                let time = world.register_component::<Time>();
                access.add_resource_read(time);
                Self::OnTimer(Timer::new(*duration, TimerMode::Repeating), time)
            }
            ScriptRunCondition::RunOnce => Self::RunOnce(false),
            ScriptRunCondition::Not(condition) => {
                Self::Not(Box::new(Self::compile(condition, world, access)))
            }
            ScriptRunCondition::And(a, b) => Self::And(
                Box::new(Self::compile(a, world, access)),
                Box::new(Self::compile(b, world, access)),
            ),
            ScriptRunCondition::Or(a, b) => Self::Or(
                Box::new(Self::compile(a, world, access)),
                Box::new(Self::compile(b, world, access)),
            ),
        }
    }

    /// # Safety
    /// The world must allow the accesses declared in [`Self::compile`]
    unsafe fn evaluate(&mut self, world: UnsafeWorldCell, last_run: Tick, this_run: Tick) -> bool {
        // Safety: the resources are declared as read
        let resource_ticks = |id: ComponentId| {
            let entity = unsafe { world.resource_entities() }.get(id)?;
            unsafe { world.get_entity(entity).ok()?.get_change_ticks_by_id(id) }
        };
        match self {
            Self::ResourceExists(id) => unsafe { world.resource_entities() }.get(*id).is_some(),
            Self::ResourceChanged(id) => {
                resource_ticks(*id).is_some_and(|ticks| ticks.is_changed(last_run, this_run))
            }
            Self::ResourceAdded(id) => {
                resource_ticks(*id).is_some_and(|ticks| ticks.is_added(last_run, this_run))
            }
            // Safety: the query's access is declared
            Self::AnyWithComponent(query) => {
                unsafe { query.iter_unchecked(world) }.next().is_some()
            }
            Self::OnTimer(timer, time) => {
                // Safety: `Time` is declared as read
                let Some(time) = (unsafe { world.get_resource_by_id(*time) }) else {
                    return false;
                };
                // Safety: the id is the id of `Time`
                timer.tick(unsafe { time.deref::<Time>() }.delta());
                timer.just_finished()
            }
            Self::RunOnce(has_run) => !std::mem::replace(has_run, true),
            Self::Not(condition) => !unsafe { condition.evaluate(world, last_run, this_run) },
            Self::And(a, b) => unsafe {
                a.evaluate(world, last_run, this_run) && b.evaluate(world, last_run, this_run)
            },
            Self::Or(a, b) => unsafe {
                a.evaluate(world, last_run, this_run) || b.evaluate(world, last_run, this_run)
            },
        }
    }
}

/// Evaluates the run conditions of a script system, all must hold for the system to run
struct ScriptConditionSystem {
    conditions: Vec<ScriptRunCondition>,
    compiled: Vec<CompiledRunCondition>,
    last_run: Tick,
}

impl ScriptConditionSystem {
    fn new(conditions: Vec<ScriptRunCondition>) -> Self {
        Self {
            conditions,
            compiled: Vec::new(),
            last_run: Tick::default(),
        }
    }
}

impl System for ScriptConditionSystem {
    type In = ();
    type Out = bool;

    fn name(&self) -> DebugName {
        "script_run_conditions".into()
    }

    fn flags(&self) -> SystemStateFlags {
        SystemStateFlags::empty()
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: SystemIn<'_, Self>,
        world: UnsafeWorldCell,
    ) -> Result<Self::Out, RunSystemError> {
        let this_run = world.increment_change_tick();
        let last_run = self.last_run;
        // every condition is evaluated, so stateful ones (timers) advance even if an earlier one doesn't hold
        let mut holds = true;
        for condition in &mut self.compiled {
            // Safety: the accesses are declared in `initialize`
            holds &= unsafe { condition.evaluate(world, last_run, this_run) };
        }
        self.last_run = this_run;
        Ok(holds)
    }

    fn apply_deferred(&mut self, _world: &mut World) {}

    fn queue_deferred(&mut self, _world: DeferredWorld) {}

    fn initialize(&mut self, world: &mut World) -> SystemAccess {
        let mut access = FilteredAccessSet::new();
        self.compiled = self
            .conditions
            .iter()
            .map(|condition| CompiledRunCondition::compile(condition, world, &mut access))
            .collect();
        SystemAccess::Shared(access)
    }

    fn check_change_tick(&mut self, change_tick: CheckChangeTicks) {
        self.last_run.check_tick(change_tick);
    }

    fn get_last_run(&self) -> Tick {
        self.last_run
    }

    fn set_last_run(&mut self, last_run: Tick) {
        self.last_run = last_run;
    }
}

// Safety: conditions only read the world
unsafe impl ReadOnlySystem for ScriptConditionSystem {}

pub use bevy_mod_scripting_bindings::{
    ScriptNamedSet, ScriptSystemSetAppExt, ScriptSystemSetRegistry,
};

/// Keeps track of the systems added by scripts.
///
/// A script system is identified by its attachment and callback label: adding a system with the same identity replaces
/// the old one, and the systems of an attachment are removed when its script unloads or reloads.
///
/// Schedules cannot be changed while they run (they are taken out of [`Schedules`]), i.e. `PreUpdate`, where scripts
/// are loaded. Changes to running schedules (and later changes to the same schedule) are queued and applied by
/// [`apply_script_system_changes`] in `First` and `Last`.
/// Removed systems stop running right away, even if their removal from the schedule is queued.
#[derive(Resource, Clone, Default)]
pub struct ScriptSystems(Arc<Mutex<ScriptSystemsInner>>);

#[derive(Default)]
struct ScriptSystemsInner {
    systems: HashMap<(ScriptAttachment, CallbackLabel), ScriptSystemEntry>,
    free_slots: Vec<u32>,
    next_slot: u32,
    /// changes to schedules which were running when requested, in order of request
    pending: Vec<PendingScheduleChange>,
    /// errors of script systems, which cannot send messages themselves as they might not have access to them
    errors: Vec<ScriptError>,
}

struct ScriptSystemEntry {
    set: ScriptSystemSet,
    schedule: InternedScheduleLabel,
    alive: Arc<AtomicBool>,
    /// the sets this system is ordered against, a removed set must not be reused while systems are ordered against it
    orders_against: Vec<InternedSystemSet>,
}

/// Script system sets are never removed from schedules, only their systems.
///
/// Bevy 0.20 does not forget removed sets (`SystemSets::remove` keeps the set in its id map), adding a system to a
/// set which was removed panics. Empty sets are reused instead, when their slot is recycled.
/// Breakages are allowed, i.e. the systems ordered before and after a removed system are not ordered against
/// each other, as that ordering only existed because of the removed system.
const SCRIPT_SYSTEM_CLEANUP_POLICY: ScheduleCleanupPolicy =
    ScheduleCleanupPolicy::RemoveSystemsOnlyAllowBreakages;

struct PendingScheduleChange {
    schedule: InternedScheduleLabel,
    change: ScheduleChange,
    attempts: u8,
}

/// The sets a script system is ordered against and is part of, resolved when the system is built
struct ScriptSystemOrdering {
    before: Vec<InternedSystemSet>,
    after: Vec<InternedSystemSet>,
    in_sets: Vec<InternedSystemSet>,
}

type IntoScriptSystemConfigs = fn(
    ScriptSystemBuilder,
    ScriptSystemSet,
    Arc<AtomicBool>,
    ScriptSystems,
    ScriptSystemOrdering,
) -> ScheduleConfigs<ScheduleSystem>;

/// Creates the system with its ordering and run conditions
fn script_system_configs<P: IntoScriptPluginParams>(
    builder: ScriptSystemBuilder,
    set: ScriptSystemSet,
    alive: Arc<AtomicBool>,
    systems: ScriptSystems,
    ordering: ScriptSystemOrdering,
) -> ScheduleConfigs<ScheduleSystem> {
    let conditions = builder.run_conditions.clone();
    let mut config = DynamicScriptSystem::<P>::new(builder, set, alive, systems).into_configs();
    for other in ordering.before {
        config = config.before(other);
    }
    for other in ordering.after {
        config = config.after(other);
    }
    for other in ordering.in_sets {
        config = config.in_set(other);
    }
    if !conditions.is_empty() {
        config = config.run_if(ScriptConditionSystem::new(conditions));
    }
    config
}

enum ScheduleChange {
    Add {
        builder: Box<ScriptSystemBuilder>,
        set: ScriptSystemSet,
        alive: Arc<AtomicBool>,
        ordering: ScriptSystemOrdering,
        into_configs: IntoScriptSystemConfigs,
    },
    /// Removes the systems in the set, and frees its slot afterwards if `free_slot` is set
    Remove {
        set: ScriptSystemSet,
        free_slot: bool,
    },
}

/// How many times a queued change is retried before giving up, schedules which don't exist at all would never succeed.
const MAX_SCHEDULE_CHANGE_ATTEMPTS: u8 = 8;

impl PendingScheduleChange {
    fn new(schedule: InternedScheduleLabel, change: ScheduleChange) -> Self {
        Self {
            schedule,
            change,
            attempts: 0,
        }
    }
}

impl ScriptSystemsInner {
    fn allocate_set(&mut self) -> ScriptSystemSet {
        ScriptSystemSet(self.free_slots.pop().unwrap_or_else(|| {
            self.next_slot += 1;
            self.next_slot - 1
        }))
    }
}

#[profiling::all_functions]
impl ScriptSystems {
    /// Applies the changes in order, queueing changes to running schedules and changes behind queued ones.
    ///
    /// Returns true if the last change was applied.
    fn apply_or_queue(&self, world: &mut World, changes: Vec<PendingScheduleChange>) -> bool {
        let mut last_applied = true;
        for change in changes {
            // Keep the order of changes to a schedule across calls, the loop only keeps it within one call.
            // A change to a schedule which is back in the world must wait for the ones queued while it was running:
            //   Update:     a script adds S to Update, which is running -> Add(S) is queued
            //   PostUpdate: the script unloads -> Remove(S) to Update, which is back in the world
            //   applied right away, Remove(S) finds no set and frees the slot, then `Last` applies the queued Add(S):
            //   a dead system stays in the schedule, in the set of whichever system reuses the slot
            let queued_before = self
                .0
                .lock()
                .pending
                .iter()
                .any(|pending| pending.schedule == change.schedule);
            let not_applied = if queued_before {
                Some(change)
            } else {
                self.try_apply(world, change)
            };
            last_applied = not_applied.is_none();
            self.0.lock().pending.extend(not_applied);
        }
        last_applied
    }

    /// Applies the change, returns it back if its schedule is not in the world, i.e. because it is running.
    fn try_apply(
        &self,
        world: &mut World,
        change: PendingScheduleChange,
    ) -> Option<PendingScheduleChange> {
        let PendingScheduleChange {
            schedule: label,
            change,
            attempts,
        } = change;
        let mut change = Some(change);
        let result = with_schedule(world, label, |world, schedule| match change.take() {
            Some(ScheduleChange::Add {
                builder,
                set,
                alive,
                ordering,
                into_configs,
            }) => {
                schedule.add_systems(into_configs(*builder, set, alive, self.clone(), ordering));
                Ok(())
            }
            Some(ScheduleChange::Remove { set, free_slot }) => {
                let result = match schedule.remove_systems_in_set(
                    set,
                    world,
                    SCRIPT_SYSTEM_CLEANUP_POLICY,
                ) {
                    Ok(_) | Err(ScheduleError::SetNotFound) => Ok(()),
                    Err(err) => Err(InteropError::external(err)),
                };
                if free_slot && result.is_ok() {
                    let mut inner = self.0.lock();
                    // systems ordered against the set would be ordered against whatever reuses it
                    let interned = set.intern();
                    if !inner
                        .systems
                        .values()
                        .any(|entry| entry.orders_against.contains(&interned))
                    {
                        inner.free_slots.push(set.0);
                    }
                }
                result
            }
            None => Ok(()),
        });

        match (result, change) {
            (Some(Ok(())), _) => None,
            (Some(Err(err)), _) => {
                self.0.lock().errors.push(
                    ScriptError::from(err)
                        .with_context(format!("changing script systems in schedule: {label:?}")),
                );
                None
            }
            (None, Some(change)) if attempts < MAX_SCHEDULE_CHANGE_ATTEMPTS => {
                Some(PendingScheduleChange {
                    schedule: label,
                    change,
                    attempts: attempts + 1,
                })
            }
            (None, _) => {
                self.0.lock().errors.push(
                    ScriptError::from(InteropError::string(format!(
                        "schedule {label:?} was not found in the world after {MAX_SCHEDULE_CHANGE_ATTEMPTS} attempts"
                    )))
                    .with_context("changing script systems"),
                );
                None
            }
        }
    }

    /// Marks the systems of the attachment as removed, and removes them from their schedules.
    ///
    /// A removed system other systems are ordered against stays as a tombstone keeping its set, so a reload re-adding
    /// it is ordered as before. Tombstones nothing is ordered against anymore are freed.
    pub fn remove_systems_of(&self, world: &mut World, attachment: &ScriptAttachment) {
        let changes = {
            let mut inner = self.0.lock();
            let mut removed = HashSet::new();
            for ((owner, _), entry) in inner.systems.iter_mut() {
                if owner == attachment && entry.alive.swap(false, Ordering::Relaxed) {
                    // a tombstone holds no other sets
                    entry.orders_against.clear();
                    removed.insert(entry.set);
                }
            }
            let ordered_against = inner
                .systems
                .values()
                .flat_map(|entry| entry.orders_against.iter().copied())
                .collect::<HashSet<_>>();
            let mut changes = Vec::new();
            inner.systems.retain(|_, entry| {
                if entry.alive.load(Ordering::Relaxed) {
                    return true;
                }
                let free_slot = !ordered_against.contains(&entry.set.intern());
                if free_slot || removed.contains(&entry.set) {
                    changes.push(PendingScheduleChange::new(
                        entry.schedule,
                        ScheduleChange::Remove {
                            set: entry.set,
                            free_slot,
                        },
                    ));
                }
                !free_slot
            });
            changes
        };
        if !changes.is_empty() {
            self.apply_or_queue(world, changes);
        }
    }

    fn report(&self, error: ScriptError) {
        self.0.lock().errors.push(error);
    }
}

/// Removes the script systems of the given attachment from their schedules, see [`ScriptSystems`].
pub fn remove_script_systems(world: &mut World, attachment: &ScriptAttachment) {
    if let Some(systems) = world.get_resource::<ScriptSystems>().cloned() {
        systems.remove_systems_of(world, attachment);
    }
}

/// Applies queued changes to script systems, and sends the errors of script systems as [`ScriptErrorMessage`]s.
pub fn apply_script_system_changes(world: &mut World) {
    let Some(systems) = world.get_resource::<ScriptSystems>().cloned() else {
        return;
    };
    let pending = std::mem::take(&mut systems.0.lock().pending);
    let mut still_pending = Vec::new();
    for change in pending {
        // keep the order of changes to the same schedule
        if still_pending
            .iter()
            .any(|p: &PendingScheduleChange| p.schedule == change.schedule)
        {
            still_pending.push(change);
            continue;
        }
        still_pending.extend(systems.try_apply(world, change));
    }
    let errors = {
        let mut inner = systems.0.lock();
        // changes requested while applying go after the ones which are still pending
        still_pending.append(&mut inner.pending);
        inner.pending = still_pending;
        std::mem::take(&mut inner.errors)
    };
    if !errors.is_empty()
        && let Some(mut messages) = world.get_resource_mut::<Messages<ScriptErrorMessage>>()
    {
        messages.write_batch(errors.into_iter().map(|error| ScriptErrorMessage { error }));
    }
}

/// Takes the schedule out of the world, runs the function on it and puts it back.
///
/// Returns `None` if the schedule is not in the world, i.e. because it is running.
fn with_schedule<O>(
    world: &mut World,
    label: InternedScheduleLabel,
    f: impl FnOnce(&mut World, &mut Schedule) -> O,
) -> Option<O> {
    let mut schedule = world.get_resource_mut::<Schedules>()?.remove(label)?;
    let result = f(world, &mut schedule);
    world.get_resource_or_init::<Schedules>().insert(schedule);
    Some(result)
}

/// TODO: inline world guard into the system state, we should be able to re-use it
struct ScriptSystemState<P: IntoScriptPluginParams> {
    type_registry: TypeRegistryArc,
    function_registry: AppScriptFunctionRegistry,
    schedule_registry: AppScheduleRegistry,
    component_registry: AppScriptComponentRegistry,
    callback_runners: AppScriptCallbackRunners,
    type_catalogue: AppScriptTypeCatalogue,
    subset: AccessByteSet,
    callback_label: CallbackLabel,
    system_params: Vec<ScriptSystemParam>,
    script_contexts: ScriptContexts<P>,
    script_callbacks: ScriptCallbacks<P>,
    /// set if the system parameters are invalid, the system then never runs, the errors are reported once on initialization
    invalid: bool,
    /// the world the system was initialized in, its queries cannot be used in other worlds
    world_id: WorldId,
    /// restricts component accesses to the archetypes matched by the queries, see [`ArchetypeFilter`].
    /// Rebuilt whenever new archetypes appear, as the queries might match them.
    archetype_filter: Option<Arc<ArchetypeFilter>>,
    /// the archetype generation the filter was built for
    archetype_generation: ArchetypeGeneration,
}

/// Equivalent of [`bevy_ecs::system::SystemParam`] but for dynamic systems, these are the kinds of things
/// that scripts can ask for access to and get passed in through dynamic script systems.
pub enum ScriptSystemParam {
    /// An exclusive resource access
    Res {
        /// The component ID of the resource
        component_id: ComponentId,
        /// The type ID of the resource
        type_id: TypeId,
    },
    /// A query which returns entities
    /// Boxed to reduce stack size
    EntityQuery {
        /// The internal state of the query
        query: Box<QueryState<Entity, ()>>,
        /// The query, which builds the results for the matched entities
        builder: Box<ScriptQueryBuilder>,
    },
}

/// A system specified, created, and added by a script
pub struct DynamicScriptSystem<P: IntoScriptPluginParams> {
    name: Cow<'static, str>,
    exclusive: bool,
    pub(crate) last_run: Tick,
    target_attachment: ScriptAttachment,
    system_param_descriptors: Vec<ScriptSystemParamDescriptor>,
    state: Option<ScriptSystemState<P>>,
    /// the set containing just this system, used for ordering against it and removing it
    set: ScriptSystemSet,
    /// cleared when the system is replaced or its script is unloaded, the system then stops running
    alive: Arc<AtomicBool>,
    systems: ScriptSystems,
    _marker: std::marker::PhantomData<fn() -> P>,
}

impl<P: IntoScriptPluginParams> DynamicScriptSystem<P> {
    fn new(
        builder: ScriptSystemBuilder,
        set: ScriptSystemSet,
        alive: Arc<AtomicBool>,
        systems: ScriptSystems,
    ) -> Self {
        Self {
            name: builder.name.to_string().into(),
            exclusive: builder.is_exclusive,
            system_param_descriptors: builder.system_params,
            last_run: Default::default(),
            target_attachment: builder.attachment,
            state: None,
            set,
            alive,
            systems,
            _marker: Default::default(),
        }
    }

    fn report(&self, error: InteropError) {
        self.systems.report(
            ScriptError::from(error)
                .with_script(self.target_attachment.script().display())
                .with_context(format!("script system: {}", self.name))
                .with_language(P::LANGUAGE),
        );
    }
}

/// Builds the [`ArchetypeFilter`] restricting each component declared by a query to the archetypes the queries
/// declaring it matched. Must be called after the queries were updated with the world's archetypes.
fn build_archetype_filter(params: &[ScriptSystemParam]) -> ArchetypeFilter {
    let mut filter = ArchetypeFilter::default();
    for param in params {
        if let ScriptSystemParam::EntityQuery { query, builder } = param {
            // only components the query reads or writes, components used just as filters (`with`) are not accessible.
            // Writes are restricted to the archetypes of queries writing the component, as another query of the
            // system may only read it on other archetypes
            for (component_id, write) in builder.accessed_components() {
                filter.restrict(component_id);
                for archetype_id in query.matched_archetypes() {
                    filter.allow(component_id, archetype_id, write);
                }
            }
        }
    }
    filter
}

// #[profiling::all_functions]
impl<P: IntoScriptPluginParams> System for DynamicScriptSystem<P> {
    type In = ();

    type Out = ();

    fn name(&self) -> DebugName {
        self.name.clone().into()
    }

    fn flags(&self) -> SystemStateFlags {
        if self.exclusive {
            SystemStateFlags::NON_SEND
        } else {
            SystemStateFlags::empty()
        }
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: SystemIn<'_, Self>,
        world: UnsafeWorldCell,
    ) -> Result<Self::Out, RunSystemError> {
        // replaced or unloaded, the system might still be in the schedule until the removal is applied
        if !self.alive.load(Ordering::Relaxed) {
            return Ok(());
        }

        let this_run = world.increment_change_tick();
        let last_run = self.last_run;

        let Some(state) = &mut self.state else {
            return Err(RunSystemError::Skipped(
                SystemParamValidationError::skipped::<()>(
                    "Script system was run before it was initialized",
                ),
            ));
        };

        // reported on initialization
        if state.invalid {
            return Ok(());
        }

        if world.id() != state.world_id {
            return Err(RunSystemError::Skipped(
                SystemParamValidationError::skipped::<()>(
                    "Script system was run in a different world than it was initialized in",
                ),
            ));
        }

        let script_context = {
            // the lock must not be held while the callback runs, which might load or unload scripts
            let contexts = state.script_contexts.read();
            match contexts.get_context(&self.target_attachment) {
                Some(context) => context.as_loaded().cloned(),
                None => {
                    warn_once!(
                        "Dynamic script system `{}` could not find script for attachment: {}. It will not run until it's loaded.",
                        self.name,
                        self.target_attachment
                    );
                    None
                }
            }
        };
        let Some(script_context) = script_context else {
            return Ok(());
        };

        let mut payload = Vec::with_capacity(state.system_params.len());
        let cache = WorldAccessGuard::setup_cache_raw(
            CurrentScriptAttachment(Some(self.target_attachment.clone())),
            state.function_registry.clone(),
            state.schedule_registry.clone(),
            state.component_registry.clone(),
            state.callback_runners.clone(),
            state.type_catalogue.clone(),
        );
        let guard = if self.exclusive {
            // safety: we are an exclusive system, therefore the cell allows us to do this
            let world = unsafe { world.world_mut() };
            WorldAccessGuard::new_exclusive(world, cache)
        } else {
            // the queries match newly created archetypes when updated, the filter has to include them
            if state.archetype_filter.is_none()
                || world.archetypes().generation() != state.archetype_generation
            {
                for param in &mut state.system_params {
                    if let ScriptSystemParam::EntityQuery { query, .. } = param {
                        query.update_archetypes_unsafe_world_cell(world);
                    }
                }
                state.archetype_filter =
                    Some(Arc::new(build_archetype_filter(&state.system_params)));
                state.archetype_generation = world.archetypes().generation();
            }
            // Safety: the subset and archetype filter are exactly the accesses declared to bevy in `initialize`
            unsafe {
                WorldAccessGuard::new_non_exclusive(
                    world,
                    state.subset.clone(),
                    state.archetype_filter.clone(),
                    state.type_registry.clone(),
                    cache,
                )
            }
        };

        // TODO: cache references which don't change once we have benchmarks
        for param in &mut state.system_params {
            match param {
                ScriptSystemParam::Res {
                    component_id,
                    type_id,
                } => {
                    let res_ref = ReflectReference::new_resource_ref_by_id(*component_id, *type_id);
                    payload.push(res_ref.into_script_inline_error(guard.clone()));
                }
                ScriptSystemParam::EntityQuery { query, builder } => {
                    // Safety: the query's access is declared in `initialize`
                    let results = unsafe { query.iter_unchecked(world) }
                        .filter_map(|entity| {
                            let entity = world.get_entity(entity).ok()?;
                            // Safety: the components with change filters are declared as read
                            unsafe { builder.result_for(entity, last_run, this_run) }.map(V)
                        })
                        .collect::<Vec<_>>();

                    payload.push(results.into_script_inline_error(guard.clone()))
                }
            }
        }

        let result = P::handle(
            payload,
            &self.target_attachment,
            &state.callback_label,
            &script_context,
            state.script_callbacks.clone(),
            guard.clone(),
        );
        if let Err(err) = result {
            self.report(err);
        }
        // change filters are relative to the last time the callback ran
        self.last_run = this_run;

        Ok(())
    }

    fn initialize(&mut self, world: &mut World) -> SystemAccess {
        // we need to register all the:
        // - resources, simple just need the component ID's
        // - queries, more difficult the queries need to be built, and archetype access registered on top of component access

        // start with resources
        let mut subset = HashSet::<ComponentId>::new();
        // the accesses in `subset` which can be written, the others are read only
        let mut written = HashSet::<ComponentId>::new();
        let mut system_params = Vec::with_capacity(self.system_param_descriptors.len());
        let mut component_access_set = FilteredAccessSet::new();
        let mut initialization_errors = Vec::new();
        for param in &self.system_param_descriptors {
            match param {
                ScriptSystemParamDescriptor::Res {
                    resource: res,
                    write,
                } => {
                    let component_id = res.resource_id;
                    let type_id = res.type_registration().type_id();

                    let system_param = ScriptSystemParam::Res {
                        component_id,
                        type_id,
                    };
                    system_params.push(system_param);

                    // like `Res`/`ResMut`, an access matching nothing would be compatible with any other access
                    if *write {
                        component_access_set.add_resource_write(component_id);
                        written.insert(component_id);
                    } else {
                        component_access_set.add_resource_read(component_id);
                    }
                    if subset.contains(&component_id) {
                        initialization_errors.push(format!(
                            "Duplicate resource access in system: {component_id:?}."
                        ));
                    }
                    subset.insert(component_id);
                }
                ScriptSystemParamDescriptor::EntityQuery(builder) => {
                    let query = builder.as_query_state::<Entity>(world, QueryAccess::Declared);

                    // queries may access the same components if bevy considers them disjoint, i.e. `With<A>` vs `Without<A>`
                    let conflicts =
                        component_access_set.get_conflicts_single(query.component_access());
                    if !conflicts.is_empty() {
                        let components = match conflicts {
                            AccessConflicts::All => "all components".to_owned(),
                            AccessConflicts::Individual(ids) => ids
                                .iter()
                                .map(|id| {
                                    world.components().get_name(id).map_or_else(
                                        || format!("{id:?}"),
                                        |name| name.shortname().to_string(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(", "),
                        };
                        initialization_errors.push(format!(
                            "Query parameter #{} conflicts with an earlier parameter of the system on: {components}. Use `with`/`without` to make them disjoint.",
                            system_params.len() + 1
                        ));
                    }

                    // The access carries the query filters, so bevy runs this system in parallel with systems accessing
                    // the same components on other archetypes (i.e. `With<A>` vs `Without<A>`).
                    // The guard only allows component accesses per component id, so it is narrowed to the archetypes
                    // matched by this query in `run_unsafe`, see `ArchetypeFilter`.
                    component_access_set.add(query.component_access().clone());

                    let accesses = get_all_access_ids(query.component_access().access());
                    written.extend(
                        accesses
                            .iter()
                            .filter(|(_, write)| *write)
                            .map(|(id, _)| *id),
                    );
                    subset.extend(accesses.into_iter().map(|(a, _)| a));

                    system_params.push(ScriptSystemParam::EntityQuery {
                        query: query.into(),
                        builder: Box::new(builder.clone()),
                    });
                }
            }
        }

        // the guard must not allow writes which are declared to bevy as reads
        let (read_write, read_only): (Vec<_>, Vec<_>) = subset
            .iter()
            .map(|c| (c.index(), written.contains(c)))
            .partition(|(_, write)| *write);
        let final_subset = AccessByteSet::from_allowed_lists(
            &read_write.into_iter().map(|(c, _)| c).collect::<Vec<_>>(),
            &read_only.into_iter().map(|(c, _)| c).collect::<Vec<_>>(),
        );

        self.state = Some(ScriptSystemState {
            type_registry: world.get_resource_or_init::<AppTypeRegistry>().clone().0,
            function_registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .clone(),
            schedule_registry: world.get_resource_or_init::<AppScheduleRegistry>().clone(),
            component_registry: world
                .get_resource_or_init::<AppScriptComponentRegistry>()
                .clone(),
            callback_runners: world
                .get_resource_or_init::<AppScriptCallbackRunners>()
                .clone(),
            type_catalogue: world
                .get_resource_or_init::<AppScriptTypeCatalogue>()
                .clone(),
            subset: final_subset,
            callback_label: self.name.to_string().into(),
            system_params,
            script_contexts: world.get_resource_or_init::<ScriptContexts<P>>().clone(),
            script_callbacks: world.get_resource_or_init::<ScriptCallbacks<P>>().clone(),
            invalid: !initialization_errors.is_empty(),
            world_id: world.id(),
            archetype_filter: None,
            archetype_generation: ArchetypeGeneration::initial(),
        });

        for error in initialization_errors {
            self.report(InteropError::string(format!(
                "{error} The system will not run."
            )));
        }

        if self.exclusive {
            SystemAccess::Exclusive
        } else {
            SystemAccess::Shared(component_access_set)
        }
    }

    fn check_change_tick(&mut self, change_tick: CheckChangeTicks) {
        self.last_run.check_tick(change_tick);
    }

    fn get_last_run(&self) -> Tick {
        self.last_run
    }

    fn set_last_run(&mut self, last_run: Tick) {
        self.last_run = last_run;
    }

    fn apply_deferred(&mut self, _world: &mut World) {}

    fn queue_deferred(&mut self, _world: DeferredWorld) {}

    fn default_system_sets(&self) -> Vec<InternedSystemSet> {
        vec![self.set.intern()]
    }

    fn system_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// A trait for managing script systems in schedules dynamically
pub trait ManageScriptSystems {
    /// Temporarilly removes the given schedule from the world, and calls the given function on it, then re-inserts it.
    ///
    /// Useful for initializing schedules, or modifying systems
    fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
        &self,
        label: &ReflectSchedule,
        f: F,
    ) -> Result<O, InteropError>;

    /// Retrieves all the systems in a schedule
    fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError>;

    /// Creates a system from a system builder and inserts it into the given schedule
    fn add_system<P: IntoScriptPluginParams>(
        &self,
        schedule: &ReflectSchedule,
        builder: ScriptSystemBuilder,
    ) -> Result<ReflectSystem, InteropError>;
}

impl ManageScriptSystems for WorldGuard<'_> {
    /// Temporarilly removes the given schedule from the world, and calls the given function on it, then re-inserts it.
    ///
    /// Useful for initializing schedules, or modifying systems
    fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
        &self,
        label: &ReflectSchedule,
        f: F,
    ) -> Result<O, InteropError> {
        self.with_world_mut(|world| {
            let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
                InteropError::unsupported_operation(
                    None,
                    None,
                    "accessing schedules in a world with no schedules",
                )
            })?;

            let mut removed_schedule = schedules
                .remove(*label.label())
                .ok_or_else(|| InteropError::missing_schedule(label.identifier()))?;

            let result = f(world, &mut removed_schedule);

            // put the schedule back even if `Schedules` was removed in the meantime, so it isn't lost
            let matches = removed_schedule.label() == *label.label();
            world
                .get_resource_or_init::<Schedules>()
                .insert(removed_schedule);
            if !matches {
                return Err(InteropError::invariant(
                    "removed schedule label doesn't match the original",
                ));
            }

            Ok(result)
        })?
    }

    /// Retrieves all the systems in a schedule
    fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError> {
        self.with_resource(|schedules: &Schedules| {
            let schedule = schedules
                .get(*schedule.label())
                .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;

            let systems = schedule.systems().map_err(InteropError::external)?;

            Ok(systems
                .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id))
                .collect())
        })?
    }

    /// Creates a system from a system builder and inserts it into the given schedule
    fn add_system<P: IntoScriptPluginParams>(
        &self,
        schedule: &ReflectSchedule,
        builder: ScriptSystemBuilder,
    ) -> Result<ReflectSystem, InteropError> {
        debug!(
            "Adding script system '{}' for script '{}' to schedule '{}'",
            builder.name,
            builder.attachment,
            schedule.identifier()
        );

        builder.build::<P>(self.clone(), schedule)
    }
}

#[cfg(test)]
mod test {
    use ::{
        bevy_app::{App, MainScheduleOrder, Plugin, Update},
        bevy_asset::{AssetPlugin, Handle},
        bevy_diagnostic::DiagnosticsPlugin,
        bevy_ecs::{
            entity::Entity,
            schedule::{ScheduleLabel, Schedules},
        },
    };
    use bevy_mod_scripting_bindings::{
        ScriptComponentRegistration, ScriptTypeRegistration, ScriptValue,
    };
    use test_utils::make_test_plugin;

    use crate::{
        BMSScriptingInfrastructurePlugin,
        config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    };

    use super::*;

    make_test_plugin!(crate);

    fn test_system_rust(_world: &mut World) {}

    #[test]
    fn test_script_system_with_existing_system_dependency_can_execute() {
        let mut app = App::new();
        #[derive(ScheduleLabel, Clone, Debug, Hash, PartialEq, Eq)]
        struct TestSchedule;

        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
            BMSScriptingInfrastructurePlugin::default(),
        ));
        app.init_schedule(TestSchedule);
        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_after(Update, TestSchedule);
        app.add_systems(TestSchedule, test_system_rust);

        // run the app once
        app.finish();
        app.cleanup();
        app.update();

        // find existing rust system
        let test_system = app
            .world_mut()
            .resource_scope::<Schedules, _>(|_, schedules| {
                let (node_id, system) = schedules
                    .get(TestSchedule)
                    .unwrap()
                    .systems()
                    .unwrap()
                    .find(|(_, system)| system.name().contains("test_system_rust"))
                    .unwrap();

                ReflectSystem::from_system(system.as_ref(), node_id)
            });

        // now dynamically add script system via builder, without a matching script
        let mut builder = ScriptSystemBuilder::new(
            "test".into(),
            ScriptAttachment::StaticScript(Handle::default()),
        );
        builder.before_system(test_system);
        let world_mut = app.world_mut();
        let cache = WorldAccessGuard::setup_cache(world_mut, CurrentScriptAttachment::empty());
        let _ = builder
            .build::<TestPlugin>(
                WorldAccessGuard::new_exclusive(world_mut, cache),
                &ReflectSchedule::from_label(TestSchedule),
            )
            .unwrap();

        // now re-run app, expect no panicks
        app.update();
    }

    #[derive(ScheduleLabel, Clone, Debug, Hash, PartialEq, Eq)]
    struct SystemsTestSchedule;

    fn systems_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
            BMSScriptingInfrastructurePlugin::default(),
        ));
        app.init_schedule(SystemsTestSchedule);
        app.finish();
        app.cleanup();
        app
    }

    fn attachment() -> ScriptAttachment {
        ScriptAttachment::StaticScript(Handle::default())
    }

    fn add(app: &mut App, label: &str) -> ReflectSystem {
        build(
            app,
            ScriptSystemBuilder::new(label.to_owned().into(), attachment()),
        )
    }

    fn build(app: &mut App, builder: ScriptSystemBuilder) -> ReflectSystem {
        let world = app.world_mut();
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        builder
            .build::<TestPlugin>(
                WorldAccessGuard::new_exclusive(world, cache),
                &ReflectSchedule::from_label(SystemsTestSchedule),
            )
            .unwrap()
    }

    fn count(app: &mut App, label: &str) -> usize {
        // `systems` lists the systems of the last schedule build
        with_schedule(
            app.world_mut(),
            SystemsTestSchedule.intern(),
            |world, schedule| {
                schedule.initialize(world).unwrap();
                schedule
                    .systems()
                    .unwrap()
                    .filter(|(_, system)| system.name().to_string() == label)
                    .count()
            },
        )
        .unwrap()
    }

    /// takes the schedule out of the world, as bevy does while running it
    fn while_running(app: &mut App, f: impl FnOnce(&mut App)) {
        let schedule = app
            .world_mut()
            .resource_mut::<Schedules>()
            .remove(SystemsTestSchedule)
            .unwrap();
        f(app);
        app.world_mut().resource_mut::<Schedules>().insert(schedule);
    }

    #[test]
    fn adding_a_system_again_replaces_it() {
        let mut app = systems_test_app();
        let first = add(&mut app, "test");
        let second = add(&mut app, "test");
        assert_eq!(count(&mut app, "test"), 1);
        assert_eq!(first.default_system_sets(), second.default_system_sets());
        // sets are interned by value, so each slot is its own set
        assert_ne!(ScriptSystemSet(0).intern(), ScriptSystemSet(1).intern());
    }

    #[test]
    fn systems_are_removed_with_their_script_and_slots_are_recycled() {
        let mut app = systems_test_app();
        let first = add(&mut app, "test");
        remove_script_systems(app.world_mut(), &attachment());
        assert_eq!(count(&mut app, "test"), 0);

        let second = add(&mut app, "other");
        assert_eq!(first.default_system_sets(), second.default_system_sets());

        // a system of another script is ordered against keeps its set when its script reloads
        let mut dependent = ScriptSystemBuilder::new(
            "dependent".to_owned().into(),
            ScriptAttachment::EntityScript(Entity::PLACEHOLDER, Handle::default()),
        );
        dependent.after_system(second.clone());
        build(&mut app, dependent);
        remove_script_systems(app.world_mut(), &attachment());
        let reloaded = add(&mut app, "other");
        assert_eq!(second.default_system_sets(), reloaded.default_system_sets());
        let schedules = app.world().resource::<Schedules>();
        let graph = bevy_system_reflection::schedule_to_reflect_graph(
            schedules.get(SystemsTestSchedule).unwrap(),
        );
        assert!(graph.system_runs_before("other", "dependent").unwrap());
    }

    #[test]
    fn changes_to_running_schedules_are_applied_later() {
        let mut app = systems_test_app();
        while_running(&mut app, |app| {
            add(app, "test");
        });
        assert_eq!(count(&mut app, "test"), 0);
        apply_script_system_changes(app.world_mut());
        assert_eq!(count(&mut app, "test"), 1);

        while_running(&mut app, |app| {
            remove_script_systems(app.world_mut(), &attachment());
        });
        // stops running right away, but stays in the schedule until the change is applied
        assert_eq!(count(&mut app, "test"), 1);
        assert!(
            app.world()
                .resource::<ScriptSystems>()
                .0
                .lock()
                .systems
                .is_empty()
        );
        apply_script_system_changes(app.world_mut());
        assert_eq!(count(&mut app, "test"), 0);
    }

    #[test]
    fn script_system_errors_are_sent_as_messages() {
        let mut app = systems_test_app();
        app.world()
            .resource::<ScriptSystems>()
            .report(ScriptError::from(InteropError::string("oops".to_owned())));
        apply_script_system_changes(app.world_mut());
        assert_eq!(
            app.world().resource::<Messages<ScriptErrorMessage>>().len(),
            1
        );
    }

    #[derive(bevy_ecs::resource::Resource, Reflect, Default)]
    struct AccessTestResource;

    #[derive(bevy_ecs::component::Component, Reflect)]
    struct AccessTestComponent;

    fn access_of(world: &mut World, f: impl FnOnce(&mut ScriptSystemBuilder)) -> FilteredAccessSet {
        let mut builder = ScriptSystemBuilder::new("test".to_owned().into(), attachment());
        f(&mut builder);
        let mut system = DynamicScriptSystem::<TestPlugin>::new(
            builder,
            ScriptSystemSet(0),
            Arc::new(AtomicBool::new(true)),
            ScriptSystems::default(),
        );
        match system.initialize(world) {
            SystemAccess::Shared(access) => access,
            _ => panic!("expected a non-exclusive system"),
        }
    }

    #[test]
    fn read_only_parameters_are_declared_as_reads() {
        let mut world = World::new();
        let resource = ScriptResourceRegistration::new(
            ScriptTypeRegistration::new(Arc::new(bevy_reflect::TypeRegistration::of::<
                AccessTestResource,
            >())),
            world.init_resource::<AccessTestResource>(),
        );
        let component = ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(bevy_reflect::TypeRegistration::of::<
                AccessTestComponent,
            >())),
            world.register_component::<AccessTestComponent>(),
        );
        let reader = |builder: &mut ScriptSystemBuilder| {
            let mut query = ScriptQueryBuilder::new();
            query.component_read(component.clone());
            builder.resource_read(resource.clone()).query(query);
        };
        let resource_writer = |builder: &mut ScriptSystemBuilder| {
            builder.resource(resource.clone());
        };
        let component_writer = |builder: &mut ScriptSystemBuilder| {
            let mut query = ScriptQueryBuilder::new();
            query.component(component.clone());
            builder.query(query);
        };

        let a = access_of(&mut world, reader);
        let b = access_of(&mut world, reader);
        assert!(a.is_compatible(&b), "readers should run in parallel");
        assert!(!a.is_compatible(&access_of(&mut world, resource_writer)));
        assert!(!a.is_compatible(&access_of(&mut world, component_writer)));
    }

    #[test]
    fn run_conditions_track_changes_and_time() {
        let mut world = World::new();
        world.init_resource::<AccessTestResource>();
        world.insert_resource(Time::<()>::default());
        let resource = ScriptResourceRegistration::new(
            ScriptTypeRegistration::new(Arc::new(bevy_reflect::TypeRegistration::of::<
                AccessTestResource,
            >())),
            world.register_component::<AccessTestResource>(),
        );

        let mut changed =
            ScriptConditionSystem::new(vec![ScriptRunCondition::ResourceChanged(resource.clone())]);
        changed.initialize(&mut world);
        assert!(
            changed.run((), &mut world).unwrap(),
            "added counts as changed"
        );
        assert!(!changed.run((), &mut world).unwrap());
        bevy_ecs::change_detection::DetectChangesMut::set_changed(
            &mut world.resource_mut::<AccessTestResource>(),
        );
        assert!(changed.run((), &mut world).unwrap());

        let mut timer =
            ScriptConditionSystem::new(vec![ScriptRunCondition::OnTimer(Duration::from_secs(1))]);
        timer.initialize(&mut world);
        assert!(!timer.run((), &mut world).unwrap());
        world
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(1500));
        assert!(timer.run((), &mut world).unwrap());
    }

    #[derive(bevy_ecs::schedule::SystemSet, Clone, Hash, PartialEq, Eq, Debug)]
    struct RustTestSet;

    fn rust_system_in_set() {}

    #[test]
    fn script_systems_order_against_registered_sets() {
        let mut app = systems_test_app();
        app.add_systems(SystemsTestSchedule, rust_system_in_set.in_set(RustTestSet));

        let world = app.world_mut();
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        let mut builder = ScriptSystemBuilder::new("test".to_owned().into(), attachment());
        builder.after_set(RustTestSet);
        builder
            .build::<TestPlugin>(
                WorldAccessGuard::new_exclusive(world, cache),
                &ReflectSchedule::from_label(SystemsTestSchedule),
            )
            .unwrap();

        let schedules = app.world().resource::<Schedules>();
        let graph = bevy_system_reflection::schedule_to_reflect_graph(
            schedules.get(SystemsTestSchedule).unwrap(),
        );
        assert!(
            graph
                .system_runs_before("rust_system_in_set", "test")
                .unwrap()
        );
    }

    #[derive(bevy_ecs::component::Component, Reflect)]
    struct OtherTestComponent;

    #[test]
    fn query_change_filters_and_optional_components() {
        let mut world = World::new();
        let component = |_: &mut World, registration, id| {
            ScriptComponentRegistration::new(
                ScriptTypeRegistration::new(Arc::new(registration)),
                id,
            )
        };
        let id = world.register_component::<AccessTestComponent>();
        let test_component = component(
            &mut world,
            bevy_reflect::TypeRegistration::of::<AccessTestComponent>(),
            id,
        );
        let id = world.register_component::<OtherTestComponent>();
        let other_component = component(
            &mut world,
            bevy_reflect::TypeRegistration::of::<OtherTestComponent>(),
            id,
        );

        let unchanged = world.spawn(AccessTestComponent).id();
        let changed = world.spawn((AccessTestComponent, OtherTestComponent)).id();
        world.clear_trackers();
        bevy_ecs::change_detection::DetectChangesMut::set_changed(
            &mut world.get_mut::<AccessTestComponent>(changed).unwrap(),
        );

        let mut query = ScriptQueryBuilder::new();
        query
            .optional_read(other_component.clone())
            .changed_component(test_component.clone());
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        let results = guard.query(query).unwrap();
        assert_eq!(results.len(), 1, "only the changed entity matches");
        assert_eq!(results[0].entity, changed);
        assert!(results[0].components[0].is_some());

        let mut query = ScriptQueryBuilder::new();
        query.optional_read(other_component);
        let mut results = guard.query(query).unwrap();
        results.retain(|result| result.entity == unchanged);
        assert!(
            results[0].components[0].is_none(),
            "absent optionals are none"
        );
    }
}
