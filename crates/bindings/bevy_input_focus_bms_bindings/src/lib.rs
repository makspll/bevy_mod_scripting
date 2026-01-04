#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyInputFocusScriptingPlugin;
pub(crate) fn register_input_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::InputFocus,
    >::new(world)
        .register_documented(
            "clear",
            |mut _self: Mut<::bevy_input_focus::InputFocus>| {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::InputFocus::clear(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clears input focus.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_input_focus::InputFocus>| {
                let output: Val<::bevy_input_focus::InputFocus> = {
                    {
                        let output: Val<::bevy_input_focus::InputFocus> = <::bevy_input_focus::InputFocus as ::core::clone::Clone>::clone(
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
            "from_entity",
            |entity: Val<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_input_focus::InputFocus> = {
                    {
                        let output: Val<::bevy_input_focus::InputFocus> = ::bevy_input_focus::InputFocus::from_entity(
                                entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`InputFocus`] resource with the given entity.\n This is mostly useful for tests.",
            &["entity"],
        )
        .register_documented(
            "set",
            |
                mut _self: Mut<::bevy_input_focus::InputFocus>,
                entity: Val<::bevy_ecs::entity::Entity>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::InputFocus::set(
                                &mut _self,
                                entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Set the entity with input focus.",
            &["_self", "entity"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_input_focus::InputFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_input_focus_visible_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::InputFocusVisible,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_input_focus::InputFocusVisible>| {
            let output: Val<::bevy_input_focus::InputFocusVisible> = {
                {
                    let output: Val<::bevy_input_focus::InputFocusVisible> =
                        <::bevy_input_focus::InputFocusVisible as ::core::clone::Clone>::clone(
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
            ::bevy_input_focus::InputFocusVisible,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_auto_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::AutoFocus,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_input_focus::AutoFocus>| {
            let output: Val<::bevy_input_focus::AutoFocus> = {
                {
                    let output: Val<::bevy_input_focus::AutoFocus> =
                        <::bevy_input_focus::AutoFocus as ::core::clone::Clone>::clone(&_self)
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
            ::bevy_input_focus::AutoFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_navigation_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
    >::new(world)
        .register_documented(
            "add_edge",
            |
                mut _self: Mut<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: Val<::bevy_ecs::entity::Entity>,
                b: Val<::bevy_ecs::entity::Entity>,
                direction: Val<::bevy_math::CompassOctant>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::add_edge(
                                &mut _self,
                                a.into_inner(),
                                b.into_inner(),
                                direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds an edge between two entities in the navigation map.\n Any existing edge from A in the provided direction will be overwritten.\n The reverse edge will not be added, so navigation will only be possible in one direction.\n If you want to add a symmetrical edge, use [`add_symmetrical_edge`](Self::add_symmetrical_edge) instead.",
            &["_self", "a", "b", "direction"],
        )
        .register_documented(
            "add_symmetrical_edge",
            |
                mut _self: Mut<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: Val<::bevy_ecs::entity::Entity>,
                b: Val<::bevy_ecs::entity::Entity>,
                direction: Val<::bevy_math::CompassOctant>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::add_symmetrical_edge(
                                &mut _self,
                                a.into_inner(),
                                b.into_inner(),
                                direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds a symmetrical edge between two entities in the navigation map.\n The A -> B path will use the provided direction, while B -> A will use the [`CompassOctant::opposite`] variant.\n Any existing connections between the two entities will be overwritten.",
            &["_self", "a", "b", "direction"],
        )
        .register_documented(
            "clear",
            |
                mut _self: Mut<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::clear(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Completely clears the navigation map, removing all entities and connections.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >|
            {
                let output: Val<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                > = {
                    {
                        let output: Val<
                            ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                        > = <::bevy_input_focus::directional_navigation::DirectionalNavigationMap as ::core::clone::Clone>::clone(
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
                _self: Ref<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                other: Ref<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::directional_navigation::DirectionalNavigationMap as ::core::cmp::PartialEq<
                            ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
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
            "remove",
            |
                mut _self: Mut<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                entity: Val<::bevy_ecs::entity::Entity>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::remove(
                                &mut _self,
                                entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds a new entity to the navigation map, overwriting any existing neighbors for that entity.\n Removes an entity from the navigation map, including all connections to and from it.\n Note that this is an O(n) operation, where n is the number of entities in the map,\n as we must iterate over each entity to check for connections to the removed entity.\n If you are removing multiple entities, consider using [`remove_multiple`](Self::remove_multiple) instead.",
            &["_self", "entity"],
        )
        .register_documented(
            "remove_multiple",
            |
                mut _self: Mut<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                entities: Val<::bevy_ecs::entity::EntityHashSet>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::remove_multiple(
                                &mut _self,
                                entities.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Removes a collection of entities from the navigation map.\n While this is still an O(n) operation, where n is the number of entities in the map,\n it is more efficient than calling [`remove`](Self::remove) multiple times,\n as we can check for connections to all removed entities in a single pass.\n An [`EntityHashSet`] must be provided as it is noticeably faster than the standard hasher or a [`Vec`](`alloc::vec::Vec`).",
            &["_self", "entities"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_nav_neighbors_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::NavNeighbors,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_input_focus::directional_navigation::NavNeighbors>| {
                let output: Val<
                    ::bevy_input_focus::directional_navigation::NavNeighbors,
                > = {
                    {
                        let output: Val<
                            ::bevy_input_focus::directional_navigation::NavNeighbors,
                        > = <::bevy_input_focus::directional_navigation::NavNeighbors as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_input_focus::directional_navigation::NavNeighbors>,
                other: Ref<::bevy_input_focus::directional_navigation::NavNeighbors>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::directional_navigation::NavNeighbors as ::core::cmp::PartialEq<
                            ::bevy_input_focus::directional_navigation::NavNeighbors,
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
            "set",
            |
                mut _self: Mut<::bevy_input_focus::directional_navigation::NavNeighbors>,
                octant: Val<::bevy_math::CompassOctant>,
                entity: Val<::bevy_ecs::entity::Entity>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::NavNeighbors::set(
                                &mut _self,
                                octant.into_inner(),
                                entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Set the neighbor for a given [`CompassOctant`].",
            &["_self", "octant", "entity"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_input_focus::directional_navigation::NavNeighbors,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tab_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::tab_navigation::TabIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_input_focus::tab_navigation::TabIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_input_focus::tab_navigation::TabIndex as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_input_focus::tab_navigation::TabIndex>| {
                let output: Val<::bevy_input_focus::tab_navigation::TabIndex> = {
                    {
                        let output: Val<::bevy_input_focus::tab_navigation::TabIndex> = <::bevy_input_focus::tab_navigation::TabIndex as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_input_focus::tab_navigation::TabIndex>,
                other: Ref<::bevy_input_focus::tab_navigation::TabIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::tab_navigation::TabIndex as ::core::cmp::PartialEq<
                            ::bevy_input_focus::tab_navigation::TabIndex,
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
            ::bevy_input_focus::tab_navigation::TabIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tab_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::tab_navigation::TabGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_input_focus::tab_navigation::TabGroup>| {
                let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = <::bevy_input_focus::tab_navigation::TabGroup as ::core::clone::Clone>::clone(
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
            "modal",
            || {
                let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = ::bevy_input_focus::tab_navigation::TabGroup::modal()
                            .into();
                        output
                    }
                };
                output
            },
            " Create a modal tab group.",
            &[],
        )
        .register_documented(
            "new",
            |order: i32| {
                let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: Val<::bevy_input_focus::tab_navigation::TabGroup> = ::bevy_input_focus::tab_navigation::TabGroup::new(
                                order,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new tab group with the given order.",
            &["order"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_input_focus::tab_navigation::TabGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyInputFocusScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_input_focus_functions(&mut world);
        register_input_focus_visible_functions(&mut world);
        register_auto_focus_functions(&mut world);
        register_directional_navigation_map_functions(&mut world);
        register_nav_neighbors_functions(&mut world);
        register_tab_index_functions(&mut world);
        register_tab_group_functions(&mut world);
    }
}
