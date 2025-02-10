// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use crate::*;
pub struct BevyEcsScriptingPlugin;
#[script_bindings(
    remote,
    name = "entity_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::entity::Entity {
    fn clone(_self: Ref<bevy::ecs::entity::Entity>) {
        let output: Val<bevy::ecs::entity::Entity> = <bevy::ecs::entity::Entity as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(_self: Ref<bevy::ecs::entity::Entity>, other: Ref<bevy::ecs::entity::Entity>) {
        let output: bool = <bevy::ecs::entity::Entity as std::cmp::PartialEq<
            bevy::ecs::entity::Entity,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn from_bits(bits: u64) {
        let output: Val<bevy::ecs::entity::Entity> = bevy::ecs::entity::Entity::from_bits(
                bits,
            )
            .into();
        output
    }
    fn from_raw(index: u32) {
        let output: Val<bevy::ecs::entity::Entity> = bevy::ecs::entity::Entity::from_raw(
                index,
            )
            .into();
        output
    }
    fn generation(_self: Val<bevy::ecs::entity::Entity>) {
        let output: u32 = bevy::ecs::entity::Entity::generation(_self.into_inner())
            .into();
        output
    }
    fn index(_self: Val<bevy::ecs::entity::Entity>) {
        let output: u32 = bevy::ecs::entity::Entity::index(_self.into_inner()).into();
        output
    }
    fn to_bits(_self: Val<bevy::ecs::entity::Entity>) {
        let output: u64 = bevy::ecs::entity::Entity::to_bits(_self.into_inner()).into();
        output
    }
}
#[script_bindings(
    remote,
    name = "on_add_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::world::OnAdd {}
#[script_bindings(
    remote,
    name = "on_insert_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::world::OnInsert {}
#[script_bindings(
    remote,
    name = "on_remove_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::world::OnRemove {}
#[script_bindings(
    remote,
    name = "on_replace_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::world::OnReplace {}
#[script_bindings(
    remote,
    name = "component_id_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::component::ComponentId {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::ecs::component::ComponentId>) {
        let output: () = <bevy::ecs::component::ComponentId as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::ecs::component::ComponentId>) {
        let output: Val<bevy::ecs::component::ComponentId> = <bevy::ecs::component::ComponentId as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::ecs::component::ComponentId>,
        other: Ref<bevy::ecs::component::ComponentId>,
    ) {
        let output: bool = <bevy::ecs::component::ComponentId as std::cmp::PartialEq<
            bevy::ecs::component::ComponentId,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn index(_self: Val<bevy::ecs::component::ComponentId>) {
        let output: usize = bevy::ecs::component::ComponentId::index(_self.into_inner())
            .into();
        output
    }
    fn new(index: usize) {
        let output: Val<bevy::ecs::component::ComponentId> = bevy::ecs::component::ComponentId::new(
                index,
            )
            .into();
        output
    }
}
#[script_bindings(
    remote,
    name = "tick_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::component::Tick {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::ecs::component::Tick>) {
        let output: () = <bevy::ecs::component::Tick as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::ecs::component::Tick>) {
        let output: Val<bevy::ecs::component::Tick> = <bevy::ecs::component::Tick as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::ecs::component::Tick>,
        other: Ref<bevy::ecs::component::Tick>,
    ) {
        let output: bool = <bevy::ecs::component::Tick as std::cmp::PartialEq<
            bevy::ecs::component::Tick,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn get(_self: Val<bevy::ecs::component::Tick>) {
        let output: u32 = bevy::ecs::component::Tick::get(_self.into_inner()).into();
        output
    }
    fn is_newer_than(
        _self: Val<bevy::ecs::component::Tick>,
        last_run: Val<bevy::ecs::component::Tick>,
        this_run: Val<bevy::ecs::component::Tick>,
    ) {
        let output: bool = bevy::ecs::component::Tick::is_newer_than(
                _self.into_inner(),
                last_run.into_inner(),
                this_run.into_inner(),
            )
            .into();
        output
    }
    fn new(tick: u32) {
        let output: Val<bevy::ecs::component::Tick> = bevy::ecs::component::Tick::new(
                tick,
            )
            .into();
        output
    }
    fn set(mut _self: Mut<bevy::ecs::component::Tick>, tick: u32) {
        let output: () = bevy::ecs::component::Tick::set(&mut _self, tick).into();
        output
    }
}
#[script_bindings(
    remote,
    name = "component_ticks_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::component::ComponentTicks {
    fn clone(_self: Ref<bevy::ecs::component::ComponentTicks>) {
        let output: Val<bevy::ecs::component::ComponentTicks> = <bevy::ecs::component::ComponentTicks as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn is_added(
        _self: Ref<bevy::ecs::component::ComponentTicks>,
        last_run: Val<bevy::ecs::component::Tick>,
        this_run: Val<bevy::ecs::component::Tick>,
    ) {
        let output: bool = bevy::ecs::component::ComponentTicks::is_added(
                &_self,
                last_run.into_inner(),
                this_run.into_inner(),
            )
            .into();
        output
    }
    fn is_changed(
        _self: Ref<bevy::ecs::component::ComponentTicks>,
        last_run: Val<bevy::ecs::component::Tick>,
        this_run: Val<bevy::ecs::component::Tick>,
    ) {
        let output: bool = bevy::ecs::component::ComponentTicks::is_changed(
                &_self,
                last_run.into_inner(),
                this_run.into_inner(),
            )
            .into();
        output
    }
    fn new(change_tick: Val<bevy::ecs::component::Tick>) {
        let output: Val<bevy::ecs::component::ComponentTicks> = bevy::ecs::component::ComponentTicks::new(
                change_tick.into_inner(),
            )
            .into();
        output
    }
    fn set_changed(
        mut _self: Mut<bevy::ecs::component::ComponentTicks>,
        change_tick: Val<bevy::ecs::component::Tick>,
    ) {
        let output: () = bevy::ecs::component::ComponentTicks::set_changed(
                &mut _self,
                change_tick.into_inner(),
            )
            .into();
        output
    }
}
#[script_bindings(
    remote,
    name = "identifier_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::identifier::Identifier {
    fn clone(_self: Ref<bevy::ecs::identifier::Identifier>) {
        let output: Val<bevy::ecs::identifier::Identifier> = <bevy::ecs::identifier::Identifier as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::ecs::identifier::Identifier>,
        other: Ref<bevy::ecs::identifier::Identifier>,
    ) {
        let output: bool = <bevy::ecs::identifier::Identifier as std::cmp::PartialEq<
            bevy::ecs::identifier::Identifier,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn from_bits(value: u64) {
        let output: Val<bevy::ecs::identifier::Identifier> = bevy::ecs::identifier::Identifier::from_bits(
                value,
            )
            .into();
        output
    }
    fn low(_self: Val<bevy::ecs::identifier::Identifier>) {
        let output: u32 = bevy::ecs::identifier::Identifier::low(_self.into_inner())
            .into();
        output
    }
    fn masked_high(_self: Val<bevy::ecs::identifier::Identifier>) {
        let output: u32 = bevy::ecs::identifier::Identifier::masked_high(
                _self.into_inner(),
            )
            .into();
        output
    }
    fn to_bits(_self: Val<bevy::ecs::identifier::Identifier>) {
        let output: u64 = bevy::ecs::identifier::Identifier::to_bits(_self.into_inner())
            .into();
        output
    }
}
#[script_bindings(
    remote,
    name = "entity_hash_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::entity::EntityHash {
    fn clone(_self: Ref<bevy::ecs::entity::EntityHash>) {
        let output: Val<bevy::ecs::entity::EntityHash> = <bevy::ecs::entity::EntityHash as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(
    remote,
    name = "removed_component_entity_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::removal_detection::RemovedComponentEntity {
    fn clone(_self: Ref<bevy::ecs::removal_detection::RemovedComponentEntity>) {
        let output: Val<bevy::ecs::removal_detection::RemovedComponentEntity> = <bevy::ecs::removal_detection::RemovedComponentEntity as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(
    remote,
    name = "system_id_marker_functions",
    bms_core_path = "bevy_mod_scripting_core"
)]
impl bevy::ecs::system::SystemIdMarker {}
impl ::bevy::app::Plugin for BevyEcsScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        register_entity_functions(&mut world);
        register_on_add_functions(&mut world);
        register_on_insert_functions(&mut world);
        register_on_remove_functions(&mut world);
        register_on_replace_functions(&mut world);
        register_component_id_functions(&mut world);
        register_tick_functions(&mut world);
        register_component_ticks_functions(&mut world);
        register_identifier_functions(&mut world);
        register_entity_hash_functions(&mut world);
        register_removed_component_entity_functions(&mut world);
        register_system_id_marker_functions(&mut world);
    }
}
