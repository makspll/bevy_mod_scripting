// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation, bindings::ReflectReference,
};
use crate::{
    bindings::proxy::{
        LuaReflectRefProxy, LuaReflectRefMutProxy, LuaReflectValProxy, LuaValProxy,
        LuaIdentityProxy,
    },
    RegisterLua, tealr::mlu::mlua::IntoLua,
};
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::entity::Entity",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::entity::Entity>,
    ) -> LuaReflectValProxy<bevy::ecs::entity::Entity>;

"#,
    r#"
/// Creates a new entity ID with the specified `index` and a generation of 1.
/// # Note
/// Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
/// [`Commands::spawn`](crate::system::Commands::spawn). This method should generally
/// only be used for sharing entities across apps, and only when they have a scheme
/// worked out to share an index space (which doesn't happen by default).
/// In general, one should not try to synchronize the ECS by attempting to ensure that
/// `Entity` lines up between instances, but instead insert a secondary identifier as
/// a component.

    #[lua()]
    fn from_raw(index: u32) -> LuaReflectValProxy<bevy::ecs::entity::Entity>;

"#,
    r#"
/// Convert to a form convenient for passing outside of rust.
/// Only useful for identifying entities within the same instance of an application. Do not use
/// for serialization between runs.
/// No particular structure is guaranteed for the returned bits.

    #[lua()]
    fn to_bits(_self: LuaReflectValProxy<bevy::ecs::entity::Entity>) -> u64;

"#,
    r#"
/// Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
/// Only useful when applied to results from `to_bits` in the same instance of an application.
/// # Panics
/// This method will likely panic if given `u64` values that did not come from [`Entity::to_bits`].

    #[lua()]
    fn from_bits(bits: u64) -> LuaReflectValProxy<bevy::ecs::entity::Entity>;

"#,
    r#"
/// Return a transiently unique identifier.
/// No two simultaneously-live entities share the same index, but dead entities' indices may collide
/// with both live and dead entities. Useful for compactly representing entities within a
/// specific snapshot of the world, such as when serializing.

    #[lua()]
    fn index(_self: LuaReflectValProxy<bevy::ecs::entity::Entity>) -> u32;

"#,
    r#"
/// Returns the generation of this Entity's index. The generation is incremented each time an
/// entity with a given index is despawned. This serves as a "count" of the number of times a
/// given index has been reused (index, generation) pairs uniquely identify a given Entity.

    #[lua()]
    fn generation(_self: LuaReflectValProxy<bevy::ecs::entity::Entity>) -> u32;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::ecs::entity::Entity>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::ecs::entity::Entity>,
        other: LuaReflectRefProxy<bevy::ecs::entity::Entity>,
    ) -> bool;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Entity {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::world::OnAdd",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct OnAdd {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::world::OnInsert",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct OnInsert {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::world::OnRemove",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct OnRemove {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::world::OnReplace",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct OnReplace {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::component::ComponentId",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentId>,
    ) -> LuaReflectValProxy<bevy::ecs::component::ComponentId>;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::ecs::component::ComponentId>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentId>,
        other: LuaReflectRefProxy<bevy::ecs::component::ComponentId>,
    ) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentId>,
    ) -> ();

"#,
    r#"
/// Creates a new [`ComponentId`].
/// The `index` is a unique value associated with each type of component in a given world.
/// Usually, this value is taken from a counter incremented for each type of component registered with the world.

    #[lua()]
    fn new(index: usize) -> LuaReflectValProxy<bevy::ecs::component::ComponentId>;

"#,
    r#"
/// Returns the index of the current component.

    #[lua()]
    fn index(_self: LuaReflectValProxy<bevy::ecs::component::ComponentId>) -> usize;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct ComponentId();
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::component::Tick",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::component::Tick>,
    ) -> LuaReflectValProxy<bevy::ecs::component::Tick>;

"#,
    r#"
/// Creates a new [`Tick`] wrapping the given value.

    #[lua()]
    fn new(tick: u32) -> LuaReflectValProxy<bevy::ecs::component::Tick>;

"#,
    r#"
/// Gets the value of this change tick.

    #[lua()]
    fn get(_self: LuaReflectValProxy<bevy::ecs::component::Tick>) -> u32;

"#,
    r#"
/// Sets the value of this change tick.

    #[lua()]
    fn set(_self: LuaReflectRefMutProxy<bevy::ecs::component::Tick>, tick: u32) -> ();

"#,
    r#"
/// Returns `true` if this `Tick` occurred since the system's `last_run`.
/// `this_run` is the current tick of the system, used as a reference to help deal with wraparound.

    #[lua()]
    fn is_newer_than(
        _self: LuaReflectValProxy<bevy::ecs::component::Tick>,
        last_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
        this_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
    ) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::ecs::component::Tick>,
    ) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::ecs::component::Tick>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::ecs::component::Tick>,
        other: LuaReflectRefProxy<bevy::ecs::component::Tick>,
    ) -> bool;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Tick {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::component::ComponentTicks",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentTicks>,
    ) -> LuaReflectValProxy<bevy::ecs::component::ComponentTicks>;

"#,
    r#"
/// Returns `true` if the component or resource was added after the system last ran
/// (or the system is running for the first time).

    #[lua()]
    fn is_added(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentTicks>,
        last_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
        this_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
    ) -> bool;

"#,
    r#"
/// Returns `true` if the component or resource was added or mutably dereferenced after the system last ran
/// (or the system is running for the first time).

    #[lua()]
    fn is_changed(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentTicks>,
        last_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
        this_run: LuaReflectValProxy<bevy::ecs::component::Tick>,
    ) -> bool;

"#,
    r#"
/// Returns the tick recording the time this component or resource was most recently changed.

    #[lua()]
    fn last_changed_tick(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentTicks>,
    ) -> LuaReflectValProxy<bevy::ecs::component::Tick>;

"#,
    r#"
/// Returns the tick recording the time this component or resource was added.

    #[lua()]
    fn added_tick(
        _self: LuaReflectRefProxy<bevy::ecs::component::ComponentTicks>,
    ) -> LuaReflectValProxy<bevy::ecs::component::Tick>;

"#,
    r#"
/// Manually sets the change tick.
/// This is normally done automatically via the [`DerefMut`](std::ops::DerefMut) implementation
/// on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.
/// However, components and resources that make use of interior mutability might require manual updates.
/// # Example
/// ```no_run
/// # use bevy_ecs::{world::World, component::ComponentTicks};
/// let world: World = unimplemented!();
/// let component_ticks: ComponentTicks = unimplemented!();
/// component_ticks.set_changed(world.read_change_tick());
/// ```

    #[lua()]
    fn set_changed(
        _self: LuaReflectRefMutProxy<bevy::ecs::component::ComponentTicks>,
        change_tick: LuaReflectValProxy<bevy::ecs::component::Tick>,
    ) -> ();

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct ComponentTicks {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::identifier::Identifier",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::identifier::Identifier>,
    ) -> LuaReflectValProxy<bevy::ecs::identifier::Identifier>;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::ecs::identifier::Identifier>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::ecs::identifier::Identifier>,
        other: LuaReflectRefProxy<bevy::ecs::identifier::Identifier>,
    ) -> bool;

"#,
    r#"
/// Returns the value of the low segment of the [`Identifier`].

    #[lua()]
    fn low(_self: LuaReflectValProxy<bevy::ecs::identifier::Identifier>) -> u32;

"#,
    r#"
/// Returns the masked value of the high segment of the [`Identifier`].
/// Does not include the flag bits.

    #[lua()]
    fn masked_high(_self: LuaReflectValProxy<bevy::ecs::identifier::Identifier>) -> u32;

"#,
    r#"
/// Convert the [`Identifier`] into a `u64`.

    #[lua()]
    fn to_bits(_self: LuaReflectValProxy<bevy::ecs::identifier::Identifier>) -> u64;

"#,
    r#"
/// Convert a `u64` into an [`Identifier`].
/// # Panics
/// This method will likely panic if given `u64` values that did not come from [`Identifier::to_bits`].

    #[lua()]
    fn from_bits(value: u64) -> LuaReflectValProxy<bevy::ecs::identifier::Identifier>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Identifier {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::entity::EntityHash",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::entity::EntityHash>,
    ) -> LuaReflectValProxy<bevy::ecs::entity::EntityHash>;

"#]
)]
pub struct EntityHash {}
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::removal_detection::RemovedComponentEntity",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::ecs::removal_detection::RemovedComponentEntity>,
    ) -> LuaReflectValProxy<bevy::ecs::removal_detection::RemovedComponentEntity>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct RemovedComponentEntity();
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::ecs::system::SystemIdMarker",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[]
)]
pub struct SystemIdMarker {}
#[derive(Default)]
pub(crate) struct Globals;
impl crate::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: crate::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> crate::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance("Entity", crate::tealr::mlu::UserDataProxy::<LuaEntity>::new)?;
        instances
            .add_instance(
                "ComponentId",
                crate::tealr::mlu::UserDataProxy::<LuaComponentId>::new,
            )?;
        instances
            .add_instance("Tick", crate::tealr::mlu::UserDataProxy::<LuaTick>::new)?;
        instances
            .add_instance(
                "Identifier",
                crate::tealr::mlu::UserDataProxy::<LuaIdentifier>::new,
            )?;
        Ok(())
    }
}
fn bevy_ecs_context_initializer(
    _: &bevy_mod_scripting_core::script::ScriptId,
    ctx: &mut crate::prelude::Lua,
) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
    crate::tealr::mlu::set_global_env(Globals, ctx)?;
    Ok(())
}
pub struct BevyEcsScriptingPlugin;
impl bevy::app::Plugin for BevyEcsScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_lua_proxy::<bevy::ecs::entity::Entity>();
        app.register_lua_proxy::<bevy::ecs::world::OnAdd>();
        app.register_lua_proxy::<bevy::ecs::world::OnInsert>();
        app.register_lua_proxy::<bevy::ecs::world::OnRemove>();
        app.register_lua_proxy::<bevy::ecs::world::OnReplace>();
        app.register_lua_proxy::<bevy::ecs::component::ComponentId>();
        app.register_lua_proxy::<bevy::ecs::component::Tick>();
        app.register_lua_proxy::<bevy::ecs::component::ComponentTicks>();
        app.register_lua_proxy::<bevy::ecs::identifier::Identifier>();
        app.register_lua_proxy::<bevy::ecs::entity::EntityHash>();
        app.register_lua_proxy::<bevy::ecs::removal_detection::RemovedComponentEntity>();
        app.register_lua_proxy::<bevy::ecs::system::SystemIdMarker>();
        app.add_context_initializer::<()>(bevy_ecs_context_initializer);
        app.add_documentation_fragment(
            crate::docs::LuaDocumentationFragment::new(
                "BevyEcsAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaEntity>()
                        .process_type::<crate::tealr::mlu::UserDataProxy<LuaEntity>>()
                        .process_type::<LuaOnAdd>()
                        .process_type::<LuaOnInsert>()
                        .process_type::<LuaOnRemove>()
                        .process_type::<LuaOnReplace>()
                        .process_type::<LuaComponentId>()
                        .process_type::<
                            crate::tealr::mlu::UserDataProxy<LuaComponentId>,
                        >()
                        .process_type::<LuaTick>()
                        .process_type::<crate::tealr::mlu::UserDataProxy<LuaTick>>()
                        .process_type::<LuaComponentTicks>()
                        .process_type::<LuaIdentifier>()
                        .process_type::<
                            crate::tealr::mlu::UserDataProxy<LuaIdentifier>,
                        >()
                        .process_type::<LuaEntityHash>()
                        .process_type::<LuaRemovedComponentEntity>()
                        .process_type::<LuaSystemIdMarker>()
                },
            ),
        );
    }
}
