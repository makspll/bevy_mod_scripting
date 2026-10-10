#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{M, R, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyPickingScriptingPlugin;
pub(crate) fn register_pointer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Pointer,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::Pointer>| {
            let output: V<::bevy_picking::events::Pointer> = {
                {
                    let output: V<::bevy_picking::events::Pointer> =
                        <::bevy_picking::events::Pointer as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::Pointer>, other: R<::bevy_picking::events::Pointer>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_picking::events::Pointer as ::std::cmp::PartialEq<
                        ::bevy_picking::events::Pointer,
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
        "location",
        |_self: R<::bevy_picking::events::Pointer>| {
            let output: V<::bevy_picking::pointer::Location> = {
                {
                    let output: V<::bevy_picking::pointer::Location> =
                        ::bevy_picking::events::Pointer::location(&_self).into();
                    output
                }
            };
            output
        },
        " Returns the [`Location`] of this pointer.",
        &["_self"],
    )
    .register_documented(
        "new",
        |id: V<::bevy_picking::pointer::PointerId>,
         location: V<::bevy_picking::pointer::Location>| {
            let output: V<::bevy_picking::events::Pointer> = {
                {
                    let output: V<::bevy_picking::events::Pointer> =
                        ::bevy_picking::events::Pointer::new(
                            id.into_inner(),
                            location.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Creates a new [`Pointer`].",
        &["id", "location"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::events::Pointer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_backfaces_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::prelude::RayCastBackfaces,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::prelude::RayCastBackfaces>| {
            let output: V<::bevy_picking::prelude::RayCastBackfaces> = {
                {
                    let output: V<::bevy_picking::prelude::RayCastBackfaces> =
                        <::bevy_picking::prelude::RayCastBackfaces as ::std::clone::Clone>::clone(
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
            ::bevy_picking::prelude::RayCastBackfaces,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::prelude::RayCastVisibility,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::prelude::RayCastVisibility>| {
            let output: V<::bevy_picking::prelude::RayCastVisibility> = {
                {
                    let output: V<::bevy_picking::prelude::RayCastVisibility> =
                        <::bevy_picking::prelude::RayCastVisibility as ::std::clone::Clone>::clone(
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
            ::bevy_picking::prelude::RayCastVisibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_picking_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::mesh_picking::MeshPickingCamera,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::mesh_picking::MeshPickingCamera>| {
                let output: V<::bevy_picking::mesh_picking::MeshPickingCamera> = {
                    {
                        let output: V<::bevy_picking::mesh_picking::MeshPickingCamera> = <::bevy_picking::mesh_picking::MeshPickingCamera as ::std::clone::Clone>::clone(
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
            ::bevy_picking::mesh_picking::MeshPickingCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_picking_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::mesh_picking::MeshPickingSettings,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::mesh_picking::MeshPickingSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_button_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerButton,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerButton>| {
            let output: V<::bevy_picking::pointer::PointerButton> = {
                {
                    let output: V<::bevy_picking::pointer::PointerButton> =
                        <::bevy_picking::pointer::PointerButton as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::pointer::PointerButton>,
         other: R<::bevy_picking::pointer::PointerButton>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::PointerButton as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerButton,
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
            ::bevy_picking::pointer::PointerButton,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pickable_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::Pickable,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::Pickable>| {
                let output: V<::bevy_picking::Pickable> = {
                    {
                        let output: V<::bevy_picking::Pickable> = <::bevy_picking::Pickable as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_picking::Pickable>, other: R<::bevy_picking::Pickable>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::Pickable as ::std::cmp::PartialEq<
                            ::bevy_picking::Pickable,
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
            ::bevy_picking::Pickable,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_picking_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::PickingSettings,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::PickingSettings>| {
            let output: V<::bevy_picking::PickingSettings> = {
                {
                    let output: V<::bevy_picking::PickingSettings> =
                        <::bevy_picking::PickingSettings as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_picking::PickingSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerMap,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerMap>| {
            let output: V<::bevy_picking::pointer::PointerMap> = {
                {
                    let output: V<::bevy_picking::pointer::PointerMap> =
                        <::bevy_picking::pointer::PointerMap as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_picking::pointer::PointerMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::backend::prelude::RayMap,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::backend::prelude::RayMap>| {
            let output: V<::bevy_picking::backend::prelude::RayMap> = {
                {
                    let output: V<::bevy_picking::backend::prelude::RayMap> =
                        <::bevy_picking::backend::prelude::RayMap as ::std::clone::Clone>::clone(
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
            ::bevy_picking::backend::prelude::RayMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_input_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerInput,
    >::new(world)
        .register_documented(
            "button_just_pressed",
            |
                _self: R<::bevy_picking::pointer::PointerInput>,
                target_button: V<::bevy_picking::pointer::PointerButton>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerInput::button_just_pressed(
                                &_self,
                                target_button.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the `target_button` of this pointer was just pressed.",
            &["_self", "target_button"],
        )
        .register_documented(
            "button_just_released",
            |
                _self: R<::bevy_picking::pointer::PointerInput>,
                target_button: V<::bevy_picking::pointer::PointerButton>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerInput::button_just_released(
                                &_self,
                                target_button.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the `target_button` of this pointer was just released.",
            &["_self", "target_button"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::pointer::PointerInput>| {
                let output: V<::bevy_picking::pointer::PointerInput> = {
                    {
                        let output: V<::bevy_picking::pointer::PointerInput> = <::bevy_picking::pointer::PointerInput as ::std::clone::Clone>::clone(
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
            "new",
            |
                pointer_id: V<::bevy_picking::pointer::PointerId>,
                location: V<::bevy_picking::pointer::Location>,
                action: V<::bevy_picking::pointer::PointerAction>|
            {
                let output: V<::bevy_picking::pointer::PointerInput> = {
                    {
                        let output: V<::bevy_picking::pointer::PointerInput> = ::bevy_picking::pointer::PointerInput::new(
                                pointer_id.into_inner(),
                                location.into_inner(),
                                action.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new pointer input event.\n Note that `location` refers to the position of the pointer *after* the event occurred.",
            &["pointer_id", "location", "action"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::pointer::PointerInput,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_hits_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::backend::PointerHits,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::backend::PointerHits>| {
            let output: V<::bevy_picking::backend::PointerHits> = {
                {
                    let output: V<::bevy_picking::backend::PointerHits> =
                        <::bevy_picking::backend::PointerHits as ::std::clone::Clone>::clone(
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
            ::bevy_picking::backend::PointerHits,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hover_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::HoverMap,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::hover::HoverMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_previous_hover_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::PreviousHoverMap,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::hover::PreviousHoverMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerState,
    >::new(world)
        .register_documented(
            "clear",
            |
                mut _self: M<::bevy_picking::events::PointerState>,
                pointer_id: V<::bevy_picking::pointer::PointerId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_picking::events::PointerState::clear(
                                &mut _self,
                                pointer_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clears all the data associated with all of the buttons on a pointer. Does not free the underlying memory.",
            &["_self", "pointer_id"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::events::PointerState>| {
                let output: V<::bevy_picking::events::PointerState> = {
                    {
                        let output: V<::bevy_picking::events::PointerState> = <::bevy_picking::events::PointerState as ::std::clone::Clone>::clone(
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
            ::bevy_picking::events::PointerState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_cancel_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerCancel,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerCancel>| {
            let output: V<::bevy_picking::events::PointerCancel> = {
                {
                    let output: V<::bevy_picking::events::PointerCancel> =
                        <::bevy_picking::events::PointerCancel as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerCancel>,
         other: R<::bevy_picking::events::PointerCancel>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerCancel as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerCancel,
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
            ::bevy_picking::events::PointerCancel,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_click_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerClick,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerClick>| {
            let output: V<::bevy_picking::events::PointerClick> = {
                {
                    let output: V<::bevy_picking::events::PointerClick> =
                        <::bevy_picking::events::PointerClick as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerClick>,
         other: R<::bevy_picking::events::PointerClick>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerClick as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerClick,
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
            ::bevy_picking::events::PointerClick,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_press_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerPress,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerPress>| {
            let output: V<::bevy_picking::events::PointerPress> = {
                {
                    let output: V<::bevy_picking::events::PointerPress> =
                        <::bevy_picking::events::PointerPress as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerPress>,
         other: R<::bevy_picking::events::PointerPress>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerPress as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerPress,
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
            ::bevy_picking::events::PointerPress,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_drop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragDrop,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragDrop>| {
            let output: V<::bevy_picking::events::PointerDragDrop> = {
                {
                    let output: V<::bevy_picking::events::PointerDragDrop> =
                        <::bevy_picking::events::PointerDragDrop as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragDrop>,
         other: R<::bevy_picking::events::PointerDragDrop>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragDrop as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragDrop,
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
            ::bevy_picking::events::PointerDragDrop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_end_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragEnd,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragEnd>| {
            let output: V<::bevy_picking::events::PointerDragEnd> = {
                {
                    let output: V<::bevy_picking::events::PointerDragEnd> =
                        <::bevy_picking::events::PointerDragEnd as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragEnd>,
         other: R<::bevy_picking::events::PointerDragEnd>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragEnd as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragEnd,
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
            ::bevy_picking::events::PointerDragEnd,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_enter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragEnter,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragEnter>| {
            let output: V<::bevy_picking::events::PointerDragEnter> = {
                {
                    let output: V<::bevy_picking::events::PointerDragEnter> =
                        <::bevy_picking::events::PointerDragEnter as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragEnter>,
         other: R<::bevy_picking::events::PointerDragEnter>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragEnter as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragEnter,
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
            ::bevy_picking::events::PointerDragEnter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDrag,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDrag>| {
            let output: V<::bevy_picking::events::PointerDrag> = {
                {
                    let output: V<::bevy_picking::events::PointerDrag> =
                        <::bevy_picking::events::PointerDrag as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::PointerDrag>,
         other: R<::bevy_picking::events::PointerDrag>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDrag as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDrag,
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
            ::bevy_picking::events::PointerDrag,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_leave_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragLeave,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragLeave>| {
            let output: V<::bevy_picking::events::PointerDragLeave> = {
                {
                    let output: V<::bevy_picking::events::PointerDragLeave> =
                        <::bevy_picking::events::PointerDragLeave as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragLeave>,
         other: R<::bevy_picking::events::PointerDragLeave>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragLeave as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragLeave,
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
            ::bevy_picking::events::PointerDragLeave,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_over_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragOver,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragOver>| {
            let output: V<::bevy_picking::events::PointerDragOver> = {
                {
                    let output: V<::bevy_picking::events::PointerDragOver> =
                        <::bevy_picking::events::PointerDragOver as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragOver>,
         other: R<::bevy_picking::events::PointerDragOver>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragOver as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragOver,
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
            ::bevy_picking::events::PointerDragOver,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_drag_start_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerDragStart,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerDragStart>| {
            let output: V<::bevy_picking::events::PointerDragStart> = {
                {
                    let output: V<::bevy_picking::events::PointerDragStart> =
                        <::bevy_picking::events::PointerDragStart as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerDragStart>,
         other: R<::bevy_picking::events::PointerDragStart>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerDragStart as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerDragStart,
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
            ::bevy_picking::events::PointerDragStart,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_move_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerMove,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerMove>| {
            let output: V<::bevy_picking::events::PointerMove> = {
                {
                    let output: V<::bevy_picking::events::PointerMove> =
                        <::bevy_picking::events::PointerMove as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::PointerMove>,
         other: R<::bevy_picking::events::PointerMove>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerMove as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerMove,
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
            ::bevy_picking::events::PointerMove,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_out_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerOut,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerOut>| {
            let output: V<::bevy_picking::events::PointerOut> = {
                {
                    let output: V<::bevy_picking::events::PointerOut> =
                        <::bevy_picking::events::PointerOut as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::PointerOut>,
         other: R<::bevy_picking::events::PointerOut>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerOut as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerOut,
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
            ::bevy_picking::events::PointerOut,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_over_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerOver,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerOver>| {
            let output: V<::bevy_picking::events::PointerOver> = {
                {
                    let output: V<::bevy_picking::events::PointerOver> =
                        <::bevy_picking::events::PointerOver as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::PointerOver>,
         other: R<::bevy_picking::events::PointerOver>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerOver as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerOver,
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
            ::bevy_picking::events::PointerOver,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_leave_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerLeave,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerLeave>| {
            let output: V<::bevy_picking::events::PointerLeave> = {
                {
                    let output: V<::bevy_picking::events::PointerLeave> =
                        <::bevy_picking::events::PointerLeave as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerLeave>,
         other: R<::bevy_picking::events::PointerLeave>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerLeave as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerLeave,
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
            ::bevy_picking::events::PointerLeave,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_enter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerEnter,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerEnter>| {
            let output: V<::bevy_picking::events::PointerEnter> = {
                {
                    let output: V<::bevy_picking::events::PointerEnter> =
                        <::bevy_picking::events::PointerEnter as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerEnter>,
         other: R<::bevy_picking::events::PointerEnter>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerEnter as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerEnter,
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
            ::bevy_picking::events::PointerEnter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_release_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerRelease,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerRelease>| {
            let output: V<::bevy_picking::events::PointerRelease> = {
                {
                    let output: V<::bevy_picking::events::PointerRelease> =
                        <::bevy_picking::events::PointerRelease as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerRelease>,
         other: R<::bevy_picking::events::PointerRelease>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerRelease as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerRelease,
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
            ::bevy_picking::events::PointerRelease,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_scroll_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerScroll,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerScroll>| {
            let output: V<::bevy_picking::events::PointerScroll> = {
                {
                    let output: V<::bevy_picking::events::PointerScroll> =
                        <::bevy_picking::events::PointerScroll as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::events::PointerScroll>,
         other: R<::bevy_picking::events::PointerScroll>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::PointerScroll as ::std::cmp::PartialEq<
                            ::bevy_picking::events::PointerScroll,
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
        "to_lines",
        |_self: R<::bevy_picking::events::PointerScroll>,
         conversion_ratio: R<::bevy_input::mouse::MouseScrollPixelsPerLine>| {
            let output: V<::bevy_picking::events::PointerScroll> = {
                {
                    let output: V<::bevy_picking::events::PointerScroll> =
                        ::bevy_picking::events::PointerScroll::to_lines(&_self, &conversion_ratio)
                            .into();
                    output
                }
            };
            output
        },
        " Converts the units to [`MouseScrollUnit::Line`]",
        &["_self", "conversion_ratio"],
    )
    .register_documented(
        "to_pixels",
        |_self: R<::bevy_picking::events::PointerScroll>,
         conversion_ratio: R<::bevy_input::mouse::MouseScrollPixelsPerLine>| {
            let output: V<::bevy_picking::events::PointerScroll> = {
                {
                    let output: V<::bevy_picking::events::PointerScroll> =
                        ::bevy_picking::events::PointerScroll::to_pixels(&_self, &conversion_ratio)
                            .into();
                    output
                }
            };
            output
        },
        " Converts the units to [`MouseScrollUnit::Pixel`]",
        &["_self", "conversion_ratio"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::events::PointerScroll,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hit_data_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::backend::HitData,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::backend::HitData>| {
            let output: V<::bevy_picking::backend::HitData> = {
                {
                    let output: V<::bevy_picking::backend::HitData> =
                        <::bevy_picking::backend::HitData as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::backend::HitData>, other: R<::bevy_picking::backend::HitData>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::backend::HitData as ::std::cmp::PartialEq<
                            ::bevy_picking::backend::HitData,
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
            ::bevy_picking::backend::HitData,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerId,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerId>| {
            let output: V<::bevy_picking::pointer::PointerId> = {
                {
                    let output: V<::bevy_picking::pointer::PointerId> =
                        <::bevy_picking::pointer::PointerId as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::pointer::PointerId>,
         other: R<::bevy_picking::pointer::PointerId>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::PointerId as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerId,
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
        "get_touch_id",
        |_self: R<::bevy_picking::pointer::PointerId>| {
            let output: ::std::option::Option<u64> = {
                {
                    let output: ::std::option::Option<u64> =
                        ::bevy_picking::pointer::PointerId::get_touch_id(&_self).into();
                    output
                }
            };
            output
        },
        " Returns the touch id if the pointer is a touch input.",
        &["_self"],
    )
    .register_documented(
        "is_custom",
        |_self: R<::bevy_picking::pointer::PointerId>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_picking::pointer::PointerId::is_custom(&_self).into();
                    output
                }
            };
            output
        },
        " Returns true if the pointer is a custom input.",
        &["_self"],
    )
    .register_documented(
        "is_mouse",
        |_self: R<::bevy_picking::pointer::PointerId>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_picking::pointer::PointerId::is_mouse(&_self).into();
                    output
                }
            };
            output
        },
        " Returns true if the pointer is the mouse.",
        &["_self"],
    )
    .register_documented(
        "is_touch",
        |_self: R<::bevy_picking::pointer::PointerId>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_picking::pointer::PointerId::is_touch(&_self).into();
                    output
                }
            };
            output
        },
        " Returns true if the pointer is a touch input.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::pointer::PointerId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_location_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerLocation,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerLocation>| {
            let output: V<::bevy_picking::pointer::PointerLocation> = {
                {
                    let output: V<::bevy_picking::pointer::PointerLocation> =
                        <::bevy_picking::pointer::PointerLocation as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::pointer::PointerLocation>,
         other: R<::bevy_picking::pointer::PointerLocation>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::PointerLocation as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerLocation,
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
        |location: V<::bevy_picking::pointer::Location>| {
            let output: V<::bevy_picking::pointer::PointerLocation> = {
                {
                    let output: V<::bevy_picking::pointer::PointerLocation> =
                        ::bevy_picking::pointer::PointerLocation::new(location.into_inner()).into();
                    output
                }
            };
            output
        },
        "Returns a [`PointerLocation`] associated with the given location",
        &["location"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::pointer::PointerLocation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::backend::ray::RayId,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::backend::ray::RayId>| {
            let output: V<::bevy_picking::backend::ray::RayId> = {
                {
                    let output: V<::bevy_picking::backend::ray::RayId> =
                        <::bevy_picking::backend::ray::RayId as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::backend::ray::RayId>,
         other: R<::bevy_picking::backend::ray::RayId>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::backend::ray::RayId as ::std::cmp::PartialEq<
                            ::bevy_picking::backend::ray::RayId,
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
        |camera: V<::bevy_ecs::entity::Entity>, pointer: V<::bevy_picking::pointer::PointerId>| {
            let output: V<::bevy_picking::backend::ray::RayId> = {
                {
                    let output: V<::bevy_picking::backend::ray::RayId> =
                        ::bevy_picking::backend::ray::RayId::new(
                            camera.into_inner(),
                            pointer.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Construct a [`RayId`].",
        &["camera", "pointer"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::backend::ray::RayId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_cursor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::cursor::DefaultCursor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::cursor::DefaultCursor>| {
            let output: V<::bevy_picking::cursor::DefaultCursor> = {
                {
                    let output: V<::bevy_picking::cursor::DefaultCursor> =
                        <::bevy_picking::cursor::DefaultCursor as ::std::clone::Clone>::clone(
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
            ::bevy_picking::cursor::DefaultCursor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_entity_cursor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::cursor::EntityCursor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::cursor::EntityCursor>| {
            let output: V<::bevy_picking::cursor::EntityCursor> = {
                {
                    let output: V<::bevy_picking::cursor::EntityCursor> =
                        <::bevy_picking::cursor::EntityCursor as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::cursor::EntityCursor>,
         other: R<::bevy_picking::cursor::EntityCursor>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::cursor::EntityCursor as ::std::cmp::PartialEq<
                            ::bevy_picking::cursor::EntityCursor,
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
            ::bevy_picking::cursor::EntityCursor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_override_cursor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::cursor::OverrideCursor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::cursor::OverrideCursor>| {
            let output: V<::bevy_picking::cursor::OverrideCursor> = {
                {
                    let output: V<::bevy_picking::cursor::OverrideCursor> =
                        <::bevy_picking::cursor::OverrideCursor as ::std::clone::Clone>::clone(
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
            ::bevy_picking::cursor::OverrideCursor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_location_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::Location,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::Location>| {
            let output: V<::bevy_picking::pointer::Location> = {
                {
                    let output: V<::bevy_picking::pointer::Location> =
                        <::bevy_picking::pointer::Location as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::pointer::Location>,
         other: R<::bevy_picking::pointer::Location>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::Location as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::Location,
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
            ::bevy_picking::pointer::Location,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_action_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerAction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerAction>| {
            let output: V<::bevy_picking::pointer::PointerAction> = {
                {
                    let output: V<::bevy_picking::pointer::PointerAction> =
                        <::bevy_picking::pointer::PointerAction as ::std::clone::Clone>::clone(
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
            ::bevy_picking::pointer::PointerAction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_entry_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragEntry,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::DragEntry>| {
            let output: V<::bevy_picking::events::DragEntry> = {
                {
                    let output: V<::bevy_picking::events::DragEntry> =
                        <::bevy_picking::events::DragEntry as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::events::DragEntry>,
         other: R<::bevy_picking::events::DragEntry>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::events::DragEntry as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragEntry,
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
            ::bevy_picking::events::DragEntry,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_button_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::PointerButtonState,
    >::new(world)
    .register_documented(
        "clear",
        |mut _self: M<::bevy_picking::events::PointerButtonState>| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_picking::events::PointerButtonState::clear(&mut _self).into();
                    output
                }
            };
            output
        },
        " Clears all press and drag data tracked for this button on its pointer.",
        &["_self"],
    )
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::events::PointerButtonState>| {
            let output: V<::bevy_picking::events::PointerButtonState> = {
                {
                    let output: V<::bevy_picking::events::PointerButtonState> =
                        <::bevy_picking::events::PointerButtonState as ::std::clone::Clone>::clone(
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
            ::bevy_picking::events::PointerButtonState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hovered_entity_ancestors_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::HoveredEntityAncestors,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::events::HoveredEntityAncestors>| {
                let output: V<::bevy_picking::events::HoveredEntityAncestors> = {
                    {
                        let output: V<::bevy_picking::events::HoveredEntityAncestors> = <::bevy_picking::events::HoveredEntityAncestors as ::std::clone::Clone>::clone(
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
            ::bevy_picking::events::HoveredEntityAncestors,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_interaction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerInteraction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::pointer::PointerInteraction>| {
                let output: V<::bevy_picking::pointer::PointerInteraction> = {
                    {
                        let output: V<::bevy_picking::pointer::PointerInteraction> = <::bevy_picking::pointer::PointerInteraction as ::std::clone::Clone>::clone(
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
            ::bevy_picking::pointer::PointerInteraction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_press_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerPressState,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PointerPressState>| {
            let output: V<::bevy_picking::pointer::PointerPressState> = {
                {
                    let output: V<::bevy_picking::pointer::PointerPressState> =
                        <::bevy_picking::pointer::PointerPressState as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::pointer::PointerPressState>,
         other: R<::bevy_picking::pointer::PointerPressState>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::PointerPressState as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerPressState,
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
        "is_any_pressed",
        |_self: R<::bevy_picking::pointer::PointerPressState>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_picking::pointer::PointerPressState::is_any_pressed(&_self).into();
                    output
                }
            };
            output
        },
        " Returns true if any pointer button is pressed.",
        &["_self"],
    )
    .register_documented(
        "is_middle_pressed",
        |_self: R<::bevy_picking::pointer::PointerPressState>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_picking::pointer::PointerPressState::is_middle_pressed(&_self)
                            .into();
                    output
                }
            };
            output
        },
        " Returns true if the middle (tertiary) pointer button is pressed.",
        &["_self"],
    )
    .register_documented(
        "is_primary_pressed",
        |_self: R<::bevy_picking::pointer::PointerPressState>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_picking::pointer::PointerPressState::is_primary_pressed(&_self)
                            .into();
                    output
                }
            };
            output
        },
        " Returns true if the primary pointer button is pressed.",
        &["_self"],
    )
    .register_documented(
        "is_secondary_pressed",
        |_self: R<::bevy_picking::pointer::PointerPressState>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_picking::pointer::PointerPressState::is_secondary_pressed(&_self)
                            .into();
                    output
                }
            };
            output
        },
        " Returns true if the secondary pointer button is pressed.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::pointer::PointerPressState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_picking_interaction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::PickingInteraction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::hover::PickingInteraction>| {
            let output: V<::bevy_picking::hover::PickingInteraction> = {
                {
                    let output: V<::bevy_picking::hover::PickingInteraction> =
                        <::bevy_picking::hover::PickingInteraction as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::hover::PickingInteraction>,
         other: R<::bevy_picking::hover::PickingInteraction>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::hover::PickingInteraction as ::std::cmp::PartialEq<
                            ::bevy_picking::hover::PickingInteraction,
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
            ::bevy_picking::hover::PickingInteraction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hovered_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::Hovered,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::hover::Hovered>| {
            let output: V<::bevy_picking::hover::Hovered> = {
                {
                    let output: V<::bevy_picking::hover::Hovered> =
                        <::bevy_picking::hover::Hovered as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_picking::hover::Hovered>, other: R<::bevy_picking::hover::Hovered>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_picking::hover::Hovered as ::std::cmp::PartialEq<
                        ::bevy_picking::hover::Hovered,
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
        |_self: R<::bevy_picking::hover::Hovered>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_picking::hover::Hovered::get(&_self).into();
                    output
                }
            };
            output
        },
        " Get whether the entity is currently hovered.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::hover::Hovered,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directly_hovered_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::DirectlyHovered,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::hover::DirectlyHovered>| {
            let output: V<::bevy_picking::hover::DirectlyHovered> = {
                {
                    let output: V<::bevy_picking::hover::DirectlyHovered> =
                        <::bevy_picking::hover::DirectlyHovered as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::hover::DirectlyHovered>,
         other: R<::bevy_picking::hover::DirectlyHovered>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::hover::DirectlyHovered as ::std::cmp::PartialEq<
                            ::bevy_picking::hover::DirectlyHovered,
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
        |_self: R<::bevy_picking::hover::DirectlyHovered>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_picking::hover::DirectlyHovered::get(&_self).into();
                    output
                }
            };
            output
        },
        " Get whether the entity is currently hovered.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_picking::hover::DirectlyHovered,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_pointer_input_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::input::PointerInputSettings,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::input::PointerInputSettings>| {
                let output: V<::bevy_picking::input::PointerInputSettings> = {
                    {
                        let output: V<::bevy_picking::input::PointerInputSettings> = <::bevy_picking::input::PointerInputSettings as ::std::clone::Clone>::clone(
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
            ::bevy_picking::input::PointerInputSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_mesh_hit_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::mesh_picking::ray_cast::RayMeshHit,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::mesh_picking::ray_cast::RayMeshHit>| {
                let output: V<::bevy_picking::mesh_picking::ray_cast::RayMeshHit> = {
                    {
                        let output: V<
                            ::bevy_picking::mesh_picking::ray_cast::RayMeshHit,
                        > = <::bevy_picking::mesh_picking::ray_cast::RayMeshHit as ::std::clone::Clone>::clone(
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
            ::bevy_picking::mesh_picking::ray_cast::RayMeshHit,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_backfaces_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::mesh_picking::ray_cast::Backfaces,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::mesh_picking::ray_cast::Backfaces>| {
                let output: V<::bevy_picking::mesh_picking::ray_cast::Backfaces> = {
                    {
                        let output: V<
                            ::bevy_picking::mesh_picking::ray_cast::Backfaces,
                        > = <::bevy_picking::mesh_picking::ray_cast::Backfaces as ::std::clone::Clone>::clone(
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
            ::bevy_picking::mesh_picking::ray_cast::Backfaces,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_simplified_mesh_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh>| {
                let output: V<::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh> = {
                    {
                        let output: V<
                            ::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh,
                        > = <::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh as ::std::clone::Clone>::clone(
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
            ::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_press_direction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PressDirection,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_picking::pointer::PressDirection>| {
            let output: V<::bevy_picking::pointer::PressDirection> = {
                {
                    let output: V<::bevy_picking::pointer::PressDirection> =
                        <::bevy_picking::pointer::PressDirection as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_picking::pointer::PressDirection>,
         other: R<::bevy_picking::pointer::PressDirection>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_picking::pointer::PressDirection as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PressDirection,
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
            ::bevy_picking::pointer::PressDirection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyPickingScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_pointer_functions(&mut world);
        register_ray_cast_backfaces_functions(&mut world);
        register_ray_cast_visibility_functions(&mut world);
        register_mesh_picking_camera_functions(&mut world);
        register_mesh_picking_settings_functions(&mut world);
        register_pointer_button_functions(&mut world);
        register_pickable_functions(&mut world);
        register_picking_settings_functions(&mut world);
        register_pointer_map_functions(&mut world);
        register_ray_map_functions(&mut world);
        register_pointer_input_functions(&mut world);
        register_pointer_hits_functions(&mut world);
        register_hover_map_functions(&mut world);
        register_previous_hover_map_functions(&mut world);
        register_pointer_state_functions(&mut world);
        register_pointer_cancel_functions(&mut world);
        register_pointer_click_functions(&mut world);
        register_pointer_press_functions(&mut world);
        register_pointer_drag_drop_functions(&mut world);
        register_pointer_drag_end_functions(&mut world);
        register_pointer_drag_enter_functions(&mut world);
        register_pointer_drag_functions(&mut world);
        register_pointer_drag_leave_functions(&mut world);
        register_pointer_drag_over_functions(&mut world);
        register_pointer_drag_start_functions(&mut world);
        register_pointer_move_functions(&mut world);
        register_pointer_out_functions(&mut world);
        register_pointer_over_functions(&mut world);
        register_pointer_leave_functions(&mut world);
        register_pointer_enter_functions(&mut world);
        register_pointer_release_functions(&mut world);
        register_pointer_scroll_functions(&mut world);
        register_hit_data_functions(&mut world);
        register_pointer_id_functions(&mut world);
        register_pointer_location_functions(&mut world);
        register_ray_id_functions(&mut world);
        register_default_cursor_functions(&mut world);
        register_entity_cursor_functions(&mut world);
        register_override_cursor_functions(&mut world);
        register_location_functions(&mut world);
        register_pointer_action_functions(&mut world);
        register_drag_entry_functions(&mut world);
        register_pointer_button_state_functions(&mut world);
        register_hovered_entity_ancestors_functions(&mut world);
        register_pointer_interaction_functions(&mut world);
        register_pointer_press_state_functions(&mut world);
        register_picking_interaction_functions(&mut world);
        register_hovered_functions(&mut world);
        register_directly_hovered_functions(&mut world);
        register_pointer_input_settings_functions(&mut world);
        register_ray_mesh_hit_functions(&mut world);
        register_backfaces_functions(&mut world);
        register_simplified_mesh_functions(&mut world);
        register_press_direction_functions(&mut world);
    }
}
