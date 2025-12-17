
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]



use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder, glue::safe_transmute,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyPickingScriptingPlugin;
pub(crate) fn register_release_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Release,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Release>| {
                let output: Val<::bevy_picking::events::Release> = {
                    {
                        let output: ::bevy_picking::events::Release = <::bevy_picking::events::Release as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Release>,
                other: Ref<::bevy_picking::events::Release>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Release as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Release,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Release,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_backfaces_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::prelude::RayCastBackfaces,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::prelude::RayCastBackfaces>| {
                let output: Val<::bevy_picking::prelude::RayCastBackfaces> = {
                    {
                        let output: ::bevy_picking::prelude::RayCastBackfaces = <::bevy_picking::prelude::RayCastBackfaces as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::prelude::RayCastVisibility>| {
                let output: Val<::bevy_picking::prelude::RayCastVisibility> = {
                    {
                        let output: ::bevy_picking::prelude::RayCastVisibility = <::bevy_picking::prelude::RayCastVisibility as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::mesh_picking::MeshPickingCamera>| {
                let output: Val<::bevy_picking::mesh_picking::MeshPickingCamera> = {
                    {
                        let output: ::bevy_picking::mesh_picking::MeshPickingCamera = <::bevy_picking::mesh_picking::MeshPickingCamera as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::pointer::PointerButton>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::pointer::PointerButton as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PointerButton>| {
                let output: Val<::bevy_picking::pointer::PointerButton> = {
                    {
                        let output: ::bevy_picking::pointer::PointerButton = <::bevy_picking::pointer::PointerButton as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PointerButton>,
                other: Ref<::bevy_picking::pointer::PointerButton>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::PointerButton as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerButton,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::Pickable>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::Pickable as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::Pickable>| {
                let output: Val<::bevy_picking::Pickable> = {
                    {
                        let output: ::bevy_picking::Pickable = <::bevy_picking::Pickable as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_picking::Pickable>, other: Ref<::bevy_picking::Pickable>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::Pickable as ::std::cmp::PartialEq<
                            ::bevy_picking::Pickable,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::PickingSettings>| {
                let output: Val<::bevy_picking::PickingSettings> = {
                    {
                        let output: ::bevy_picking::PickingSettings = <::bevy_picking::PickingSettings as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
pub(crate) fn register_pointer_input_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerInput,
    >::new(world)
        .register_documented(
            "button_just_pressed",
            |
                _self: Ref<::bevy_picking::pointer::PointerInput>,
                target_button: Val<::bevy_picking::pointer::PointerButton>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerInput::button_just_pressed(
                            safe_transmute(_self),
                            safe_transmute(target_button),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PointerInput>,
                target_button: Val<::bevy_picking::pointer::PointerButton>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerInput::button_just_released(
                            safe_transmute(_self),
                            safe_transmute(target_button),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if the `target_button` of this pointer was just released.",
            &["_self", "target_button"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PointerInput>| {
                let output: Val<::bevy_picking::pointer::PointerInput> = {
                    {
                        let output: ::bevy_picking::pointer::PointerInput = <::bevy_picking::pointer::PointerInput as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                pointer_id: Val<::bevy_picking::pointer::PointerId>,
                location: Val<::bevy_picking::pointer::Location>,
                action: Val<::bevy_picking::pointer::PointerAction>|
            {
                let output: Val<::bevy_picking::pointer::PointerInput> = {
                    {
                        let output: ::bevy_picking::pointer::PointerInput = ::bevy_picking::pointer::PointerInput::new(
                            safe_transmute(pointer_id),
                            safe_transmute(location),
                            safe_transmute(action),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::backend::PointerHits>| {
                let output: Val<::bevy_picking::backend::PointerHits> = {
                    {
                        let output: ::bevy_picking::backend::PointerHits = <::bevy_picking::backend::PointerHits as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
pub(crate) fn register_cancel_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Cancel,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Cancel>| {
                let output: Val<::bevy_picking::events::Cancel> = {
                    {
                        let output: ::bevy_picking::events::Cancel = <::bevy_picking::events::Cancel as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Cancel>,
                other: Ref<::bevy_picking::events::Cancel>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Cancel as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Cancel,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Cancel,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_click_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Click,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Click>| {
                let output: Val<::bevy_picking::events::Click> = {
                    {
                        let output: ::bevy_picking::events::Click = <::bevy_picking::events::Click as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Click>,
                other: Ref<::bevy_picking::events::Click>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Click as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Click,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Click,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_press_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Press,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Press>| {
                let output: Val<::bevy_picking::events::Press> = {
                    {
                        let output: ::bevy_picking::events::Press = <::bevy_picking::events::Press as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Press>,
                other: Ref<::bevy_picking::events::Press>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Press as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Press,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Press,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_drop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragDrop,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragDrop>| {
                let output: Val<::bevy_picking::events::DragDrop> = {
                    {
                        let output: ::bevy_picking::events::DragDrop = <::bevy_picking::events::DragDrop as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragDrop>,
                other: Ref<::bevy_picking::events::DragDrop>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragDrop as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragDrop,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragDrop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_end_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragEnd,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragEnd>| {
                let output: Val<::bevy_picking::events::DragEnd> = {
                    {
                        let output: ::bevy_picking::events::DragEnd = <::bevy_picking::events::DragEnd as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragEnd>,
                other: Ref<::bevy_picking::events::DragEnd>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragEnd as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragEnd,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragEnd,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_enter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragEnter,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragEnter>| {
                let output: Val<::bevy_picking::events::DragEnter> = {
                    {
                        let output: ::bevy_picking::events::DragEnter = <::bevy_picking::events::DragEnter as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragEnter>,
                other: Ref<::bevy_picking::events::DragEnter>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragEnter as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragEnter,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragEnter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Drag,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Drag>| {
                let output: Val<::bevy_picking::events::Drag> = {
                    {
                        let output: ::bevy_picking::events::Drag = <::bevy_picking::events::Drag as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Drag>,
                other: Ref<::bevy_picking::events::Drag>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Drag as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Drag,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Drag,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_leave_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragLeave,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragLeave>| {
                let output: Val<::bevy_picking::events::DragLeave> = {
                    {
                        let output: ::bevy_picking::events::DragLeave = <::bevy_picking::events::DragLeave as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragLeave>,
                other: Ref<::bevy_picking::events::DragLeave>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragLeave as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragLeave,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragLeave,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_over_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragOver,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragOver>| {
                let output: Val<::bevy_picking::events::DragOver> = {
                    {
                        let output: ::bevy_picking::events::DragOver = <::bevy_picking::events::DragOver as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragOver>,
                other: Ref<::bevy_picking::events::DragOver>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragOver as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragOver,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragOver,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_drag_start_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::DragStart,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::DragStart>| {
                let output: Val<::bevy_picking::events::DragStart> = {
                    {
                        let output: ::bevy_picking::events::DragStart = <::bevy_picking::events::DragStart as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragStart>,
                other: Ref<::bevy_picking::events::DragStart>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragStart as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragStart,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::DragStart,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_move_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Move,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Move>| {
                let output: Val<::bevy_picking::events::Move> = {
                    {
                        let output: ::bevy_picking::events::Move = <::bevy_picking::events::Move as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Move>,
                other: Ref<::bevy_picking::events::Move>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Move as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Move,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Move,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_out_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Out,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Out>| {
                let output: Val<::bevy_picking::events::Out> = {
                    {
                        let output: ::bevy_picking::events::Out = <::bevy_picking::events::Out as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Out>,
                other: Ref<::bevy_picking::events::Out>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Out as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Out,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Out,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_over_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Over,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Over>| {
                let output: Val<::bevy_picking::events::Over> = {
                    {
                        let output: ::bevy_picking::events::Over = <::bevy_picking::events::Over as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Over>,
                other: Ref<::bevy_picking::events::Over>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Over as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Over,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Over,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scroll_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::events::Scroll,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::events::Scroll>| {
                let output: Val<::bevy_picking::events::Scroll> = {
                    {
                        let output: ::bevy_picking::events::Scroll = <::bevy_picking::events::Scroll as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::Scroll>,
                other: Ref<::bevy_picking::events::Scroll>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::Scroll as ::std::cmp::PartialEq<
                            ::bevy_picking::events::Scroll,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_picking::events::Scroll,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hit_data_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::backend::HitData,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::backend::HitData>| {
                let output: Val<::bevy_picking::backend::HitData> = {
                    {
                        let output: ::bevy_picking::backend::HitData = <::bevy_picking::backend::HitData as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::backend::HitData>,
                other: Ref<::bevy_picking::backend::HitData>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::backend::HitData as ::std::cmp::PartialEq<
                            ::bevy_picking::backend::HitData,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::pointer::PointerId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: Val<::bevy_picking::pointer::PointerId> = {
                    {
                        let output: ::bevy_picking::pointer::PointerId = <::bevy_picking::pointer::PointerId as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PointerId>,
                other: Ref<::bevy_picking::pointer::PointerId>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::PointerId as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerId,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "get_touch_id",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: ::std::option::Option<u64> = {
                    {
                        let output: ::std::option::Option<u64> = ::bevy_picking::pointer::PointerId::get_touch_id(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns the touch id if the pointer is a touch input.",
            &["_self"],
        )
        .register_documented(
            "is_custom",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerId::is_custom(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if the pointer is a custom input.",
            &["_self"],
        )
        .register_documented(
            "is_mouse",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerId::is_mouse(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if the pointer is the mouse.",
            &["_self"],
        )
        .register_documented(
            "is_touch",
            |_self: Ref<::bevy_picking::pointer::PointerId>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerId::is_touch(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::pointer::PointerLocation>| {
                let output: Val<::bevy_picking::pointer::PointerLocation> = {
                    {
                        let output: ::bevy_picking::pointer::PointerLocation = <::bevy_picking::pointer::PointerLocation as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PointerLocation>,
                other: Ref<::bevy_picking::pointer::PointerLocation>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::PointerLocation as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerLocation,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "new",
            |location: Val<::bevy_picking::pointer::Location>| {
                let output: Val<::bevy_picking::pointer::PointerLocation> = {
                    {
                        let output: ::bevy_picking::pointer::PointerLocation = ::bevy_picking::pointer::PointerLocation::new(
                            safe_transmute(location),
                        );
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::backend::ray::RayId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::backend::ray::RayId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::backend::ray::RayId>| {
                let output: Val<::bevy_picking::backend::ray::RayId> = {
                    {
                        let output: ::bevy_picking::backend::ray::RayId = <::bevy_picking::backend::ray::RayId as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::backend::ray::RayId>,
                other: Ref<::bevy_picking::backend::ray::RayId>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::backend::ray::RayId as ::std::cmp::PartialEq<
                            ::bevy_picking::backend::ray::RayId,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "new",
            |
                camera: Val<::bevy_ecs::entity::Entity>,
                pointer: Val<::bevy_picking::pointer::PointerId>|
            {
                let output: Val<::bevy_picking::backend::ray::RayId> = {
                    {
                        let output: ::bevy_picking::backend::ray::RayId = ::bevy_picking::backend::ray::RayId::new(
                            safe_transmute(camera),
                            safe_transmute(pointer),
                        );
                        safe_transmute(output)
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
pub(crate) fn register_location_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::Location,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::Location>| {
                let output: Val<::bevy_picking::pointer::Location> = {
                    {
                        let output: ::bevy_picking::pointer::Location = <::bevy_picking::pointer::Location as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::Location>,
                other: Ref<::bevy_picking::pointer::Location>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::Location as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::Location,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::pointer::PointerAction>| {
                let output: Val<::bevy_picking::pointer::PointerAction> = {
                    {
                        let output: ::bevy_picking::pointer::PointerAction = <::bevy_picking::pointer::PointerAction as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::events::DragEntry>| {
                let output: Val<::bevy_picking::events::DragEntry> = {
                    {
                        let output: ::bevy_picking::events::DragEntry = <::bevy_picking::events::DragEntry as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::events::DragEntry>,
                other: Ref<::bevy_picking::events::DragEntry>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::events::DragEntry as ::std::cmp::PartialEq<
                            ::bevy_picking::events::DragEntry,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
pub(crate) fn register_pointer_interaction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerInteraction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PointerInteraction>| {
                let output: Val<::bevy_picking::pointer::PointerInteraction> = {
                    {
                        let output: ::bevy_picking::pointer::PointerInteraction = <::bevy_picking::pointer::PointerInteraction as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
pub(crate) fn register_pointer_press_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::pointer::PointerPress,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::pointer::PointerPress as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: Val<::bevy_picking::pointer::PointerPress> = {
                    {
                        let output: ::bevy_picking::pointer::PointerPress = <::bevy_picking::pointer::PointerPress as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PointerPress>,
                other: Ref<::bevy_picking::pointer::PointerPress>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::PointerPress as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PointerPress,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "is_any_pressed",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerPress::is_any_pressed(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if any pointer button is pressed.",
            &["_self"],
        )
        .register_documented(
            "is_middle_pressed",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerPress::is_middle_pressed(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if the middle (tertiary) pointer button is pressed.",
            &["_self"],
        )
        .register_documented(
            "is_primary_pressed",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerPress::is_primary_pressed(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            " Returns true if the primary pointer button is pressed.",
            &["_self"],
        )
        .register_documented(
            "is_secondary_pressed",
            |_self: Ref<::bevy_picking::pointer::PointerPress>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::pointer::PointerPress::is_secondary_pressed(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            ::bevy_picking::pointer::PointerPress,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_picking_interaction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_picking::hover::PickingInteraction,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::hover::PickingInteraction>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::hover::PickingInteraction as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::hover::PickingInteraction>| {
                let output: Val<::bevy_picking::hover::PickingInteraction> = {
                    {
                        let output: ::bevy_picking::hover::PickingInteraction = <::bevy_picking::hover::PickingInteraction as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::hover::PickingInteraction>,
                other: Ref<::bevy_picking::hover::PickingInteraction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::hover::PickingInteraction as ::std::cmp::PartialEq<
                            ::bevy_picking::hover::PickingInteraction,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::hover::Hovered>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::hover::Hovered as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::hover::Hovered>| {
                let output: Val<::bevy_picking::hover::Hovered> = {
                    {
                        let output: ::bevy_picking::hover::Hovered = <::bevy_picking::hover::Hovered as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::hover::Hovered>,
                other: Ref<::bevy_picking::hover::Hovered>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::hover::Hovered as ::std::cmp::PartialEq<
                            ::bevy_picking::hover::Hovered,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "get",
            |_self: Ref<::bevy_picking::hover::Hovered>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::hover::Hovered::get(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::hover::DirectlyHovered>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::hover::DirectlyHovered as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::hover::DirectlyHovered>| {
                let output: Val<::bevy_picking::hover::DirectlyHovered> = {
                    {
                        let output: ::bevy_picking::hover::DirectlyHovered = <::bevy_picking::hover::DirectlyHovered as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::hover::DirectlyHovered>,
                other: Ref<::bevy_picking::hover::DirectlyHovered>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::hover::DirectlyHovered as ::std::cmp::PartialEq<
                            ::bevy_picking::hover::DirectlyHovered,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "get",
            |_self: Ref<::bevy_picking::hover::DirectlyHovered>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_picking::hover::DirectlyHovered::get(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::input::PointerInputSettings>| {
                let output: Val<::bevy_picking::input::PointerInputSettings> = {
                    {
                        let output: ::bevy_picking::input::PointerInputSettings = <::bevy_picking::input::PointerInputSettings as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::mesh_picking::ray_cast::RayMeshHit>| {
                let output: Val<::bevy_picking::mesh_picking::ray_cast::RayMeshHit> = {
                    {
                        let output: ::bevy_picking::mesh_picking::ray_cast::RayMeshHit = <::bevy_picking::mesh_picking::ray_cast::RayMeshHit as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::mesh_picking::ray_cast::Backfaces>| {
                let output: Val<::bevy_picking::mesh_picking::ray_cast::Backfaces> = {
                    {
                        let output: ::bevy_picking::mesh_picking::ray_cast::Backfaces = <::bevy_picking::mesh_picking::ray_cast::Backfaces as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            |_self: Ref<::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh>| {
                let output: Val<
                    ::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh,
                > = {
                    {
                        let output: ::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh = <::bevy_picking::mesh_picking::ray_cast::SimplifiedMesh as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_picking::pointer::PressDirection>| {
                let output: () = {
                    {
                        let output: () = <::bevy_picking::pointer::PressDirection as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_picking::pointer::PressDirection>| {
                let output: Val<::bevy_picking::pointer::PressDirection> = {
                    {
                        let output: ::bevy_picking::pointer::PressDirection = <::bevy_picking::pointer::PressDirection as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_picking::pointer::PressDirection>,
                other: Ref<::bevy_picking::pointer::PressDirection>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_picking::pointer::PressDirection as ::std::cmp::PartialEq<
                            ::bevy_picking::pointer::PressDirection,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
        register_release_functions(&mut world);
        register_ray_cast_backfaces_functions(&mut world);
        register_ray_cast_visibility_functions(&mut world);
        register_mesh_picking_camera_functions(&mut world);
        register_mesh_picking_settings_functions(&mut world);
        register_pointer_button_functions(&mut world);
        register_pickable_functions(&mut world);
        register_picking_settings_functions(&mut world);
        register_pointer_input_functions(&mut world);
        register_pointer_hits_functions(&mut world);
        register_cancel_functions(&mut world);
        register_click_functions(&mut world);
        register_press_functions(&mut world);
        register_drag_drop_functions(&mut world);
        register_drag_end_functions(&mut world);
        register_drag_enter_functions(&mut world);
        register_drag_functions(&mut world);
        register_drag_leave_functions(&mut world);
        register_drag_over_functions(&mut world);
        register_drag_start_functions(&mut world);
        register_move_functions(&mut world);
        register_out_functions(&mut world);
        register_over_functions(&mut world);
        register_scroll_functions(&mut world);
        register_hit_data_functions(&mut world);
        register_pointer_id_functions(&mut world);
        register_pointer_location_functions(&mut world);
        register_ray_id_functions(&mut world);
        register_location_functions(&mut world);
        register_pointer_action_functions(&mut world);
        register_drag_entry_functions(&mut world);
        register_pointer_interaction_functions(&mut world);
        register_pointer_press_functions(&mut world);
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
