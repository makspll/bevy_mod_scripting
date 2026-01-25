
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]



use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyEcsScriptingPlugin;
pub(crate) fn register_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::Entity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = <::bevy_ecs::entity::Entity as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::entity::Entity>,
                other: Ref<::bevy_ecs::entity::Entity>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::entity::Entity as ::core::cmp::PartialEq<
                            ::bevy_ecs::entity::Entity,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "from_bits",
            |bits: u64| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ecs::entity::Entity::from_bits(
                                bits,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].\n Only useful when applied to results from `to_bits` in the same instance of an application.\n # Panics\n This method will likely panic if given `u64` values that did not come from [`Entity::to_bits`].",
            &["bits"],
        )
        .register_documented(
            "from_index",
            |index: Val<::bevy_ecs::entity::EntityIndex>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ecs::entity::Entity::from_index(
                                index.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new entity ID with the specified `index` and an unspecified generation.\n # Note\n Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor\n [`Commands::spawn`](crate::system::Commands::spawn). This method should generally\n only be used for sharing entities across apps, and only when they have a scheme\n worked out to share an index space (which doesn't happen by default).\n In general, one should not try to synchronize the ECS by attempting to ensure that\n `Entity` lines up between instances, but instead insert a secondary identifier as\n a component.",
            &["index"],
        )
        .register_documented(
            "from_index_and_generation",
            |
                index: Val<::bevy_ecs::entity::EntityIndex>,
                generation: Val<::bevy_ecs::entity::EntityGeneration>|
            {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ecs::entity::Entity::from_index_and_generation(
                                index.into_inner(),
                                generation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new instance with the given index and generation.",
            &["index", "generation"],
        )
        .register_documented(
            "generation",
            |_self: Val<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_ecs::entity::EntityGeneration> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityGeneration> = ::bevy_ecs::entity::Entity::generation(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the generation of this Entity's index. The generation is incremented each time an\n entity with a given index is despawned. This serves as a \"count\" of the number of times a\n given index has been reused (index, generation) pairs uniquely identify a given Entity.",
            &["_self"],
        )
        .register_documented(
            "index",
            |_self: Val<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_ecs::entity::EntityIndex> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityIndex> = ::bevy_ecs::entity::Entity::index(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a transiently unique identifier.\n See also [`EntityIndex`].\n No two simultaneously-live entities share the same index, but dead entities' indices may collide\n with both live and dead entities. Useful for compactly representing entities within a\n specific snapshot of the world, such as when serializing.",
            &["_self"],
        )
        .register_documented(
            "index_u32",
            |_self: Val<::bevy_ecs::entity::Entity>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ecs::entity::Entity::index_u32(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to `self.index().index()`. See [`Self::index`] for details.",
            &["_self"],
        )
        .register_documented(
            "to_bits",
            |_self: Val<::bevy_ecs::entity::Entity>| {
                let output: u64 = {
                    {
                        let output: u64 = ::bevy_ecs::entity::Entity::to_bits(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert to a form convenient for passing outside of rust.\n Only useful for identifying entities within the same instance of an application. Do not use\n for serialization between runs.\n No particular structure is guaranteed for the returned bits.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::Entity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_child_of_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::hierarchy::ChildOf,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::hierarchy::ChildOf>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::hierarchy::ChildOf as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::hierarchy::ChildOf>| {
                let output: Val<::bevy_ecs::hierarchy::ChildOf> = {
                    {
                        let output: Val<::bevy_ecs::hierarchy::ChildOf> = <::bevy_ecs::hierarchy::ChildOf as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::hierarchy::ChildOf>,
                other: Ref<::bevy_ecs::hierarchy::ChildOf>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::hierarchy::ChildOf as ::core::cmp::PartialEq<
                            ::bevy_ecs::hierarchy::ChildOf,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "parent",
            |_self: Ref<::bevy_ecs::hierarchy::ChildOf>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ecs::hierarchy::ChildOf::parent(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The parent entity of this child entity.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::hierarchy::ChildOf,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_children_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::hierarchy::Children,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::hierarchy::Children>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::hierarchy::Children as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::hierarchy::Children>,
                other: Ref<::bevy_ecs::hierarchy::Children>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::hierarchy::Children as ::core::cmp::PartialEq<
                            ::bevy_ecs::hierarchy::Children,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "swap",
            |
                mut _self: Mut<::bevy_ecs::hierarchy::Children>,
                a_index: usize,
                b_index: usize|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_ecs::hierarchy::Children::swap(
                                &mut _self,
                                a_index,
                                b_index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Swaps the child at `a_index` with the child at `b_index`.",
            &["_self", "a_index", "b_index"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::hierarchy::Children,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_add_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::Add,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::Add>| {
                let output: Val<::bevy_ecs::lifecycle::Add> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::Add> = <::bevy_ecs::lifecycle::Add as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::Add,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_despawn_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::Despawn,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::Despawn>| {
                let output: Val<::bevy_ecs::lifecycle::Despawn> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::Despawn> = <::bevy_ecs::lifecycle::Despawn as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::Despawn,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_insert_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::Insert,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::Insert>| {
                let output: Val<::bevy_ecs::lifecycle::Insert> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::Insert> = <::bevy_ecs::lifecycle::Insert as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::Insert,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_remove_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::Remove,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::Remove>| {
                let output: Val<::bevy_ecs::lifecycle::Remove> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::Remove> = <::bevy_ecs::lifecycle::Remove as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::Remove,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_replace_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::Replace,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::Replace>| {
                let output: Val<::bevy_ecs::lifecycle::Replace> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::Replace> = <::bevy_ecs::lifecycle::Replace as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::Replace,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_name_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::name::Name,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::name::Name>| {
                let output: Val<::bevy_ecs::name::Name> = {
                    {
                        let output: Val<::bevy_ecs::name::Name> = <::bevy_ecs::name::Name as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_ecs::name::Name>, other: Ref<::bevy_ecs::name::Name>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::name::Name as ::core::cmp::PartialEq<
                            ::bevy_ecs::name::Name,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::name::Name,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_query_filters_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity_disabling::DefaultQueryFilters,
    >::new(world)
        .register_documented(
            "empty",
            || {
                let output: Val<::bevy_ecs::entity_disabling::DefaultQueryFilters> = {
                    {
                        let output: Val<
                            ::bevy_ecs::entity_disabling::DefaultQueryFilters,
                        > = ::bevy_ecs::entity_disabling::DefaultQueryFilters::empty()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new, completely empty [`DefaultQueryFilters`].\n This is provided as an escape hatch; in most cases you should initialize this using [`FromWorld`],\n which is automatically called when creating a new [`World`].",
            &[],
        )
        .register_documented(
            "register_disabling_component",
            |
                mut _self: Mut<::bevy_ecs::entity_disabling::DefaultQueryFilters>,
                component_id: Val<::bevy_ecs::component::ComponentId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_ecs::entity_disabling::DefaultQueryFilters::register_disabling_component(
                                &mut _self,
                                component_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds this [`ComponentId`] to the set of [`DefaultQueryFilters`],\n causing entities with this component to be excluded from queries.\n This method is idempotent, and will not add the same component multiple times.\n # Warning\n This method should only be called before the app starts, as it will not affect queries\n initialized before it is called.\n As discussed in the [module docs](crate::entity_disabling), this can have performance implications,\n as well as create interoperability issues, and should be used with caution.",
            &["_self", "component_id"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity_disabling::DefaultQueryFilters,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_component_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::component::ComponentId,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::component::ComponentId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::component::ComponentId as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::component::ComponentId>| {
                let output: Val<::bevy_ecs::component::ComponentId> = {
                    {
                        let output: Val<::bevy_ecs::component::ComponentId> = <::bevy_ecs::component::ComponentId as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::component::ComponentId>,
                other: Ref<::bevy_ecs::component::ComponentId>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::component::ComponentId as ::core::cmp::PartialEq<
                            ::bevy_ecs::component::ComponentId,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "index",
            |_self: Val<::bevy_ecs::component::ComponentId>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_ecs::component::ComponentId::index(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the index of the current component.",
            &["_self"],
        )
        .register_documented(
            "new",
            |index: usize| {
                let output: Val<::bevy_ecs::component::ComponentId> = {
                    {
                        let output: Val<::bevy_ecs::component::ComponentId> = ::bevy_ecs::component::ComponentId::new(
                                index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`ComponentId`].\n The `index` is a unique value associated with each type of component in a given world.\n Usually, this value is taken from a counter incremented for each type of component registered with the world.",
            &["index"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::component::ComponentId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tick_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::change_detection::Tick,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::change_detection::Tick>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::change_detection::Tick as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::change_detection::Tick>| {
                let output: Val<::bevy_ecs::change_detection::Tick> = {
                    {
                        let output: Val<::bevy_ecs::change_detection::Tick> = <::bevy_ecs::change_detection::Tick as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::change_detection::Tick>,
                other: Ref<::bevy_ecs::change_detection::Tick>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::change_detection::Tick as ::core::cmp::PartialEq<
                            ::bevy_ecs::change_detection::Tick,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "get",
            |_self: Val<::bevy_ecs::change_detection::Tick>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ecs::change_detection::Tick::get(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Gets the value of this change tick.",
            &["_self"],
        )
        .register_documented(
            "is_newer_than",
            |
                _self: Val<::bevy_ecs::change_detection::Tick>,
                last_run: Val<::bevy_ecs::change_detection::Tick>,
                this_run: Val<::bevy_ecs::change_detection::Tick>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ecs::change_detection::Tick::is_newer_than(
                                _self.into_inner(),
                                last_run.into_inner(),
                                this_run.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this `Tick` occurred since the system's `last_run`.\n `this_run` is the current tick of the system, used as a reference to help deal with wraparound.",
            &["_self", "last_run", "this_run"],
        )
        .register_documented(
            "new",
            |tick: u32| {
                let output: Val<::bevy_ecs::change_detection::Tick> = {
                    {
                        let output: Val<::bevy_ecs::change_detection::Tick> = ::bevy_ecs::change_detection::Tick::new(
                                tick,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Tick`] wrapping the given value.",
            &["tick"],
        )
        .register_documented(
            "set",
            |mut _self: Mut<::bevy_ecs::change_detection::Tick>, tick: u32| {
                let output: () = {
                    {
                        let output: () = ::bevy_ecs::change_detection::Tick::set(
                                &mut _self,
                                tick,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the value of this change tick.",
            &["_self", "tick"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::change_detection::Tick,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_component_ticks_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::change_detection::ComponentTicks,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::change_detection::ComponentTicks>| {
                let output: Val<::bevy_ecs::change_detection::ComponentTicks> = {
                    {
                        let output: Val<::bevy_ecs::change_detection::ComponentTicks> = <::bevy_ecs::change_detection::ComponentTicks as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "is_added",
            |
                _self: Ref<::bevy_ecs::change_detection::ComponentTicks>,
                last_run: Val<::bevy_ecs::change_detection::Tick>,
                this_run: Val<::bevy_ecs::change_detection::Tick>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ecs::change_detection::ComponentTicks::is_added(
                                &_self,
                                last_run.into_inner(),
                                this_run.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the component or resource was added after the system last ran\n (or the system is running for the first time).",
            &["_self", "last_run", "this_run"],
        )
        .register_documented(
            "is_changed",
            |
                _self: Ref<::bevy_ecs::change_detection::ComponentTicks>,
                last_run: Val<::bevy_ecs::change_detection::Tick>,
                this_run: Val<::bevy_ecs::change_detection::Tick>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ecs::change_detection::ComponentTicks::is_changed(
                                &_self,
                                last_run.into_inner(),
                                this_run.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the component or resource was added or mutably dereferenced after the system last ran\n (or the system is running for the first time).",
            &["_self", "last_run", "this_run"],
        )
        .register_documented(
            "new",
            |change_tick: Val<::bevy_ecs::change_detection::Tick>| {
                let output: Val<::bevy_ecs::change_detection::ComponentTicks> = {
                    {
                        let output: Val<::bevy_ecs::change_detection::ComponentTicks> = ::bevy_ecs::change_detection::ComponentTicks::new(
                                change_tick.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new instance with the same change tick for `added` and `changed`.",
            &["change_tick"],
        )
        .register_documented(
            "set_changed",
            |
                mut _self: Mut<::bevy_ecs::change_detection::ComponentTicks>,
                change_tick: Val<::bevy_ecs::change_detection::Tick>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_ecs::change_detection::ComponentTicks::set_changed(
                                &mut _self,
                                change_tick.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Manually sets the change tick.\n This is normally done automatically via the [`DerefMut`](core::ops::DerefMut) implementation\n on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.\n However, components and resources that make use of interior mutability might require manual updates.\n # Example\n ```no_run\n # use bevy_ecs::{world::World, change_detection::ComponentTicks};\n let world: World = unimplemented!();\n let component_ticks: ComponentTicks = unimplemented!();\n component_ticks.set_changed(world.read_change_tick());\n ```",
            &["_self", "change_tick"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::change_detection::ComponentTicks,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_hash_set_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::EntityHashSet,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::entity::EntityHashSet>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::entity::EntityHashSet as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::EntityHashSet>| {
                let output: Val<::bevy_ecs::entity::EntityHashSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityHashSet> = <::bevy_ecs::entity::EntityHashSet as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::entity::EntityHashSet>,
                other: Ref<::bevy_ecs::entity::EntityHashSet>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::entity::EntityHashSet as ::core::cmp::PartialEq<
                            ::bevy_ecs::entity::EntityHashSet,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_ecs::entity::EntityHashSet>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ecs::entity::EntityHashSet::is_empty(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the set contains no elements.",
            &["_self"],
        )
        .register_documented(
            "len",
            |_self: Ref<::bevy_ecs::entity::EntityHashSet>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_ecs::entity::EntityHashSet::len(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the number of elements in the set.",
            &["_self"],
        )
        .register_documented(
            "new",
            || {
                let output: Val<::bevy_ecs::entity::EntityHashSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityHashSet> = ::bevy_ecs::entity::EntityHashSet::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates an empty `EntityHashSet`.\n Equivalent to [`HashSet::with_hasher(EntityHash)`].\n [`HashSet::with_hasher(EntityHash)`]: HashSet::with_hasher",
            &[],
        )
        .register_documented(
            "with_capacity",
            |n: usize| {
                let output: Val<::bevy_ecs::entity::EntityHashSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityHashSet> = ::bevy_ecs::entity::EntityHashSet::with_capacity(
                                n,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates an empty `EntityHashSet` with the specified capacity.\n Equivalent to [`HashSet::with_capacity_and_hasher(n, EntityHash)`].\n [`HashSet::with_capacity_and_hasher(n, EntityHash)`]: HashSet::with_capacity_and_hasher",
            &["n"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::EntityHashSet,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_index_set_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::EntityIndexSet,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::EntityIndexSet>| {
                let output: Val<::bevy_ecs::entity::EntityIndexSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityIndexSet> = <::bevy_ecs::entity::EntityIndexSet as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::entity::EntityIndexSet>,
                other: Ref<::bevy_ecs::entity::EntityIndexSet>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::entity::EntityIndexSet as ::core::cmp::PartialEq<
                            ::bevy_ecs::entity::EntityIndexSet,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "new",
            || {
                let output: Val<::bevy_ecs::entity::EntityIndexSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityIndexSet> = ::bevy_ecs::entity::EntityIndexSet::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates an empty `EntityIndexSet`.\n Equivalent to [`IndexSet::with_hasher(EntityHash)`].\n [`IndexSet::with_hasher(EntityHash)`]: IndexSet::with_hasher",
            &[],
        )
        .register_documented(
            "with_capacity",
            |n: usize| {
                let output: Val<::bevy_ecs::entity::EntityIndexSet> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityIndexSet> = ::bevy_ecs::entity::EntityIndexSet::with_capacity(
                                n,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates an empty `EntityIndexSet` with the specified capacity.\n Equivalent to [`IndexSet::with_capacity_and_hasher(n, EntityHash)`].\n [`IndexSet::with_capacity_and_hasher(n, EntityHash)`]: IndexSet::with_capacity_and_hasher",
            &["n"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::EntityIndexSet,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::EntityIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::entity::EntityIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::entity::EntityIndex as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::EntityIndex>| {
                let output: Val<::bevy_ecs::entity::EntityIndex> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityIndex> = <::bevy_ecs::entity::EntityIndex as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::entity::EntityIndex>,
                other: Ref<::bevy_ecs::entity::EntityIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::entity::EntityIndex as ::core::cmp::PartialEq<
                            ::bevy_ecs::entity::EntityIndex,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "index",
            |_self: Val<::bevy_ecs::entity::EntityIndex>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ecs::entity::EntityIndex::index(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Gets the index of the entity.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::EntityIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_generation_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::EntityGeneration,
    >::new(world)
        .register_documented(
            "after_versions",
            |_self: Val<::bevy_ecs::entity::EntityGeneration>, versions: u32| {
                let output: Val<::bevy_ecs::entity::EntityGeneration> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityGeneration> = ::bevy_ecs::entity::EntityGeneration::after_versions(
                                _self.into_inner(),
                                versions,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`EntityGeneration`] that would result from this many more `versions` of the corresponding [`EntityIndex`] from passing.",
            &["_self", "versions"],
        )
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ecs::entity::EntityGeneration>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ecs::entity::EntityGeneration as ::core::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::EntityGeneration>| {
                let output: Val<::bevy_ecs::entity::EntityGeneration> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityGeneration> = <::bevy_ecs::entity::EntityGeneration as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ecs::entity::EntityGeneration>,
                other: Ref<::bevy_ecs::entity::EntityGeneration>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ecs::entity::EntityGeneration as ::core::cmp::PartialEq<
                            ::bevy_ecs::entity::EntityGeneration,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "from_bits",
            |bits: u32| {
                let output: Val<::bevy_ecs::entity::EntityGeneration> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityGeneration> = ::bevy_ecs::entity::EntityGeneration::from_bits(
                                bits,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reconstruct an [`EntityGeneration`] previously destructured with [`EntityGeneration::to_bits`].\n Only useful when applied to results from `to_bits` in the same instance of an application.",
            &["bits"],
        )
        .register_documented(
            "to_bits",
            |_self: Val<::bevy_ecs::entity::EntityGeneration>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ecs::entity::EntityGeneration::to_bits(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Gets some bits that represent this value.\n The bits are opaque and should not be regarded as meaningful.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::EntityGeneration,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_hash_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity::EntityHash,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity::EntityHash>| {
                let output: Val<::bevy_ecs::entity::EntityHash> = {
                    {
                        let output: Val<::bevy_ecs::entity::EntityHash> = <::bevy_ecs::entity::EntityHash as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity::EntityHash,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_disabled_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::entity_disabling::Disabled,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::entity_disabling::Disabled>| {
                let output: Val<::bevy_ecs::entity_disabling::Disabled> = {
                    {
                        let output: Val<::bevy_ecs::entity_disabling::Disabled> = <::bevy_ecs::entity_disabling::Disabled as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::entity_disabling::Disabled,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_removed_component_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::lifecycle::RemovedComponentEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ecs::lifecycle::RemovedComponentEntity>| {
                let output: Val<::bevy_ecs::lifecycle::RemovedComponentEntity> = {
                    {
                        let output: Val<::bevy_ecs::lifecycle::RemovedComponentEntity> = <::bevy_ecs::lifecycle::RemovedComponentEntity as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::lifecycle::RemovedComponentEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_observed_by_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ecs::observer::ObservedBy,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ecs::observer::ObservedBy,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyEcsScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_entity_functions(&mut world);
        register_child_of_functions(&mut world);
        register_children_functions(&mut world);
        register_add_functions(&mut world);
        register_despawn_functions(&mut world);
        register_insert_functions(&mut world);
        register_remove_functions(&mut world);
        register_replace_functions(&mut world);
        register_name_functions(&mut world);
        register_default_query_filters_functions(&mut world);
        register_component_id_functions(&mut world);
        register_tick_functions(&mut world);
        register_component_ticks_functions(&mut world);
        register_entity_hash_set_functions(&mut world);
        register_entity_index_set_functions(&mut world);
        register_entity_index_functions(&mut world);
        register_entity_generation_functions(&mut world);
        register_entity_hash_functions(&mut world);
        register_disabled_functions(&mut world);
        register_removed_component_entity_functions(&mut world);
        register_observed_by_functions(&mut world);
    }
}
