
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;


use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{R, M, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyInputFocusScriptingPlugin;
pub(crate) fn register_input_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::InputFocus,
    >::new(world)
        .register_documented(
            "clear",
            |mut _self: M<::bevy_input_focus::InputFocus>| {
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
            |_self: R<::bevy_input_focus::InputFocus>| {
                let output: V<::bevy_input_focus::InputFocus> = {
                    {
                        let output: V<::bevy_input_focus::InputFocus> = <::bevy_input_focus::InputFocus as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::InputFocus>,
                other: R<::bevy_input_focus::InputFocus>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::InputFocus as ::core::cmp::PartialEq<
                            ::bevy_input_focus::InputFocus,
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
            "from_entity",
            |entity: V<::bevy_ecs::entity::Entity>| {
                let output: V<::bevy_input_focus::InputFocus> = {
                    {
                        let output: V<::bevy_input_focus::InputFocus> = ::bevy_input_focus::InputFocus::from_entity(
                                entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`InputFocus`] resource with the given entity.\n This is mostly useful for tests.\n WARNING: this will clear any buffered focus changes,\n so it may cause missed [`FocusGained`] and [`FocusLost`] events.\n Prefer the [`set`](InputFocus::set) method for normal use, which will preserve buffered changes.",
            &["entity"],
        )
        .register_documented(
            "set",
            |
                mut _self: M<::bevy_input_focus::InputFocus>,
                entity: V<::bevy_ecs::entity::Entity>,
                cause: V<::bevy_input_focus::FocusCause>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::InputFocus::set(
                                &mut _self,
                                entity.into_inner(),
                                cause.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Set the entity with input focus.\n When spawning entities, you may want to use the [`AutoFocus`] component instead,\n which will automatically set focus to the entity when it is spawned.\n This is particularly useful when working with bsn! scenes, where spawning may be delayed.",
            &["_self", "entity", "cause"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_input_focus::InputFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_focus_cause_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::FocusCause,
    >::new(world)
        .register_documented(
            "assert_fields_are_eq",
            |_self: R<::bevy_input_focus::FocusCause>| {
                let output: () = {
                    {
                        let output: () = <::bevy_input_focus::FocusCause as ::core::cmp::Eq>::assert_fields_are_eq(
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
            |_self: R<::bevy_input_focus::FocusCause>| {
                let output: V<::bevy_input_focus::FocusCause> = {
                    {
                        let output: V<::bevy_input_focus::FocusCause> = <::bevy_input_focus::FocusCause as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::FocusCause>,
                other: R<::bevy_input_focus::FocusCause>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::FocusCause as ::core::cmp::PartialEq<
                            ::bevy_input_focus::FocusCause,
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
            ::bevy_input_focus::FocusCause,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_input_focus_visible_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::InputFocusVisible,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::InputFocusVisible>| {
                let output: V<::bevy_input_focus::InputFocusVisible> = {
                    {
                        let output: V<::bevy_input_focus::InputFocusVisible> = <::bevy_input_focus::InputFocusVisible as ::core::clone::Clone>::clone(
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
pub(crate) fn register_acquire_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::AcquireFocus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::AcquireFocus>| {
                let output: V<::bevy_input_focus::AcquireFocus> = {
                    {
                        let output: V<::bevy_input_focus::AcquireFocus> = <::bevy_input_focus::AcquireFocus as ::core::clone::Clone>::clone(
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
            ::bevy_input_focus::AcquireFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_auto_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::AutoFocus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::AutoFocus>| {
                let output: V<::bevy_input_focus::AutoFocus> = {
                    {
                        let output: V<::bevy_input_focus::AutoFocus> = <::bevy_input_focus::AutoFocus as ::core::clone::Clone>::clone(
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
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: V<::bevy_ecs::entity::Entity>,
                b: V<::bevy_ecs::entity::Entity>,
                direction: V<::bevy_math::CompassOctant>|
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
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: V<::bevy_ecs::entity::Entity>,
                b: V<::bevy_ecs::entity::Entity>,
                direction: V<::bevy_math::CompassOctant>|
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
            "block_edge",
            |
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: V<::bevy_ecs::entity::Entity>,
                direction: V<::bevy_math::CompassOctant>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::block_edge(
                                &mut _self,
                                a.into_inner(),
                                direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds an edge blocking automatic navigation from an entity in a direction.\n Any existing edge from A in the provided direction will be overwritten.\n The reverse block will not be added, so navigation will still be possible from other entities\n in the direction.\n If you want to add a symmetrical block, use [`block_symmetrical_edge`](Self::block_symmetrical_edge) instead.\n Note that blocking a primary cardinal direction will not block intermediates.\n In other words, blocking `North` will still allow navigation towards `NorthEast`.",
            &["_self", "a", "direction"],
        )
        .register_documented(
            "block_symmetrical_edge",
            |
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                a: V<::bevy_ecs::entity::Entity>,
                b: V<::bevy_ecs::entity::Entity>,
                direction: V<::bevy_math::CompassOctant>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::block_symmetrical_edge(
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
            " Adds a symmetrical blocking edge between two entities in the navigation map.\n The blocked A -> B path will use the provided direction, while B -> A will use the [`CompassOctant::opposite`] variant.\n Any existing connections between the two entities will be overwritten.",
            &["_self", "a", "b", "direction"],
        )
        .register_documented(
            "clear",
            |
                mut _self: M<
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
                _self: R<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >|
            {
                let output: V<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                > = {
                    {
                        let output: V<
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
                _self: R<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                other: R<
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
            "get_neighbor",
            |
                _self: R<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                focus: V<::bevy_ecs::entity::Entity>,
                octant: V<::bevy_math::CompassOctant>|
            {
                let output: V<::bevy_input_focus::directional_navigation::NavNeighbor> = {
                    {
                        let output: V<
                            ::bevy_input_focus::directional_navigation::NavNeighbor,
                        > = ::bevy_input_focus::directional_navigation::DirectionalNavigationMap::get_neighbor(
                                &_self,
                                focus.into_inner(),
                                octant.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Gets the entity in a given direction from the current focus, if any.",
            &["_self", "focus", "octant"],
        )
        .register_documented(
            "remove",
            |
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                entity: V<::bevy_ecs::entity::Entity>|
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
            " Removes an entity from the navigation map, including all connections to and from it.\n Note that this is an O(n) operation, where n is the number of entities in the map,\n as we must iterate over each entity to check for connections to the removed entity.\n If you are removing multiple entities, consider using [`remove_multiple`](Self::remove_multiple) instead.",
            &["_self", "entity"],
        )
        .register_documented(
            "remove_multiple",
            |
                mut _self: M<
                    ::bevy_input_focus::directional_navigation::DirectionalNavigationMap,
                >,
                entities: V<::bevy_ecs::entity::EntityHashSet>|
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
pub(crate) fn register_auto_navigation_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::directional_navigation::AutoNavigationConfig>| {
                let output: V<
                    ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
                > = {
                    {
                        let output: V<
                            ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
                        > = <::bevy_input_focus::directional_navigation::AutoNavigationConfig as ::core::clone::Clone>::clone(
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
                _self: R<
                    ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
                >,
                other: R<
                    ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
                >|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::directional_navigation::AutoNavigationConfig as ::core::cmp::PartialEq<
                            ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
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
            ::bevy_input_focus::directional_navigation::AutoNavigationConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_nav_neighbor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::NavNeighbor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::directional_navigation::NavNeighbor>| {
                let output: V<::bevy_input_focus::directional_navigation::NavNeighbor> = {
                    {
                        let output: V<
                            ::bevy_input_focus::directional_navigation::NavNeighbor,
                        > = <::bevy_input_focus::directional_navigation::NavNeighbor as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::directional_navigation::NavNeighbor>,
                other: R<::bevy_input_focus::directional_navigation::NavNeighbor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::directional_navigation::NavNeighbor as ::core::cmp::PartialEq<
                            ::bevy_input_focus::directional_navigation::NavNeighbor,
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
            ::bevy_input_focus::directional_navigation::NavNeighbor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_nav_neighbors_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::NavNeighbors,
    >::new(world)
        .register_documented(
            "block",
            |
                mut _self: M<::bevy_input_focus::directional_navigation::NavNeighbors>,
                octant: V<::bevy_math::CompassOctant>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_input_focus::directional_navigation::NavNeighbors::block(
                                &mut _self,
                                octant.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Prevent navigation to a given [`CompassOctant`].\n Note that navigation in this direction specifically will\n be blocked. For example, blocking [`CompassOctant::North`]\n will not affect the neighbor towards [`CompassOctant::NorthWest`].",
            &["_self", "octant"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::directional_navigation::NavNeighbors>| {
                let output: V<
                    ::bevy_input_focus::directional_navigation::NavNeighbors,
                > = {
                    {
                        let output: V<
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
                _self: R<::bevy_input_focus::directional_navigation::NavNeighbors>,
                other: R<::bevy_input_focus::directional_navigation::NavNeighbors>|
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
            "get",
            |
                _self: R<::bevy_input_focus::directional_navigation::NavNeighbors>,
                octant: V<::bevy_math::CompassOctant>|
            {
                let output: V<::bevy_input_focus::directional_navigation::NavNeighbor> = {
                    {
                        let output: V<
                            ::bevy_input_focus::directional_navigation::NavNeighbor,
                        > = ::bevy_input_focus::directional_navigation::NavNeighbors::get(
                                &_self,
                                octant.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the neighbor for a given [`CompassOctant`].",
            &["_self", "octant"],
        )
        .register_documented(
            "set",
            |
                mut _self: M<::bevy_input_focus::directional_navigation::NavNeighbors>,
                octant: V<::bevy_math::CompassOctant>,
                entity: V<::bevy_ecs::entity::Entity>|
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
pub(crate) fn register_focusable_area_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::directional_navigation::FocusableArea,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::directional_navigation::FocusableArea>| {
                let output: V<
                    ::bevy_input_focus::directional_navigation::FocusableArea,
                > = {
                    {
                        let output: V<
                            ::bevy_input_focus::directional_navigation::FocusableArea,
                        > = <::bevy_input_focus::directional_navigation::FocusableArea as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::directional_navigation::FocusableArea>,
                other: R<::bevy_input_focus::directional_navigation::FocusableArea>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::directional_navigation::FocusableArea as ::core::cmp::PartialEq<
                            ::bevy_input_focus::directional_navigation::FocusableArea,
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
            ::bevy_input_focus::directional_navigation::FocusableArea,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tab_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::tab_navigation::TabIndex,
    >::new(world)
        .register_documented(
            "assert_fields_are_eq",
            |_self: R<::bevy_input_focus::tab_navigation::TabIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_input_focus::tab_navigation::TabIndex as ::core::cmp::Eq>::assert_fields_are_eq(
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
            |_self: R<::bevy_input_focus::tab_navigation::TabIndex>| {
                let output: V<::bevy_input_focus::tab_navigation::TabIndex> = {
                    {
                        let output: V<::bevy_input_focus::tab_navigation::TabIndex> = <::bevy_input_focus::tab_navigation::TabIndex as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::tab_navigation::TabIndex>,
                other: R<::bevy_input_focus::tab_navigation::TabIndex>|
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
            |_self: R<::bevy_input_focus::tab_navigation::TabGroup>| {
                let output: V<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: V<::bevy_input_focus::tab_navigation::TabGroup> = <::bevy_input_focus::tab_navigation::TabGroup as ::core::clone::Clone>::clone(
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
                let output: V<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: V<::bevy_input_focus::tab_navigation::TabGroup> = ::bevy_input_focus::tab_navigation::TabGroup::modal()
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
                let output: V<::bevy_input_focus::tab_navigation::TabGroup> = {
                    {
                        let output: V<::bevy_input_focus::tab_navigation::TabGroup> = ::bevy_input_focus::tab_navigation::TabGroup::new(
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
pub(crate) fn register_nav_action_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::tab_navigation::NavAction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::tab_navigation::NavAction>| {
                let output: V<::bevy_input_focus::tab_navigation::NavAction> = {
                    {
                        let output: V<::bevy_input_focus::tab_navigation::NavAction> = <::bevy_input_focus::tab_navigation::NavAction as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_input_focus::tab_navigation::NavAction>,
                other: R<::bevy_input_focus::tab_navigation::NavAction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_input_focus::tab_navigation::NavAction as ::core::cmp::PartialEq<
                            ::bevy_input_focus::tab_navigation::NavAction,
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
            ::bevy_input_focus::tab_navigation::NavAction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_focus_gained_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::FocusGained,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::FocusGained>| {
                let output: V<::bevy_input_focus::FocusGained> = {
                    {
                        let output: V<::bevy_input_focus::FocusGained> = <::bevy_input_focus::FocusGained as ::core::clone::Clone>::clone(
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
            ::bevy_input_focus::FocusGained,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_focus_lost_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_input_focus::FocusLost,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_input_focus::FocusLost>| {
                let output: V<::bevy_input_focus::FocusLost> = {
                    {
                        let output: V<::bevy_input_focus::FocusLost> = <::bevy_input_focus::FocusLost as ::core::clone::Clone>::clone(
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
            ::bevy_input_focus::FocusLost,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyInputFocusScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_input_focus_functions(&mut world);
        register_focus_cause_functions(&mut world);
        register_input_focus_visible_functions(&mut world);
        register_acquire_focus_functions(&mut world);
        register_auto_focus_functions(&mut world);
        register_directional_navigation_map_functions(&mut world);
        register_auto_navigation_config_functions(&mut world);
        register_nav_neighbor_functions(&mut world);
        register_nav_neighbors_functions(&mut world);
        register_focusable_area_functions(&mut world);
        register_tab_index_functions(&mut world);
        register_tab_group_functions(&mut world);
        register_nav_action_functions(&mut world);
        register_focus_gained_functions(&mut world);
        register_focus_lost_functions(&mut world);
    }
}
