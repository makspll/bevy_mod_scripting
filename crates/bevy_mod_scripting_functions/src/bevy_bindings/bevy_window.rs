    #![allow(clippy::all)]
    #![allow(unused, deprecated, dead_code)]
    use bevy_mod_scripting_core::bindings::{
        ReflectReference,
        function::{
            from::{Ref, Mut, Val},
            namespace::NamespaceBuilder,
        },
    };
    use bevy_ecs::prelude::*;
    use bevy_mod_scripting_derive::script_bindings;
    use crate::*;
    pub struct BevyWindowScriptingPlugin;
    pub(crate) fn register_cursor_entered_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::CursorEntered,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::CursorEntered>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::CursorEntered as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::CursorEntered>| {
                    let output: Val<::bevy_window::prelude::CursorEntered> = {
                        {
                            let output: Val<::bevy_window::prelude::CursorEntered> = <::bevy_window::prelude::CursorEntered as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::CursorEntered>,
                    other: Ref<::bevy_window::prelude::CursorEntered>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::CursorEntered as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::CursorEntered,
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
                ::bevy_window::prelude::CursorEntered,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_cursor_left_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::CursorLeft,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::CursorLeft>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::CursorLeft as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::CursorLeft>| {
                    let output: Val<::bevy_window::prelude::CursorLeft> = {
                        {
                            let output: Val<::bevy_window::prelude::CursorLeft> = <::bevy_window::prelude::CursorLeft as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::CursorLeft>,
                    other: Ref<::bevy_window::prelude::CursorLeft>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::CursorLeft as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::CursorLeft,
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
                ::bevy_window::prelude::CursorLeft,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_cursor_moved_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::CursorMoved,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::prelude::CursorMoved>| {
                    let output: Val<::bevy_window::prelude::CursorMoved> = {
                        {
                            let output: Val<::bevy_window::prelude::CursorMoved> = <::bevy_window::prelude::CursorMoved as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::CursorMoved>,
                    other: Ref<::bevy_window::prelude::CursorMoved>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::CursorMoved as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::CursorMoved,
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
                ::bevy_window::prelude::CursorMoved,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_file_drag_and_drop_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::FileDragAndDrop,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::FileDragAndDrop>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::FileDragAndDrop as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::FileDragAndDrop>| {
                    let output: Val<::bevy_window::prelude::FileDragAndDrop> = {
                        {
                            let output: Val<::bevy_window::prelude::FileDragAndDrop> = <::bevy_window::prelude::FileDragAndDrop as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::FileDragAndDrop>,
                    other: Ref<::bevy_window::prelude::FileDragAndDrop>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::FileDragAndDrop as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::FileDragAndDrop,
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
                ::bevy_window::prelude::FileDragAndDrop,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_ime_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::Ime,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::Ime>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::Ime as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::Ime>| {
                    let output: Val<::bevy_window::prelude::Ime> = {
                        {
                            let output: Val<::bevy_window::prelude::Ime> = <::bevy_window::prelude::Ime as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::Ime>,
                    other: Ref<::bevy_window::prelude::Ime>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::Ime as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::Ime,
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
                ::bevy_window::prelude::Ime,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_monitor_selection_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::MonitorSelection,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::MonitorSelection>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::MonitorSelection as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::MonitorSelection>| {
                    let output: Val<::bevy_window::prelude::MonitorSelection> = {
                        {
                            let output: Val<::bevy_window::prelude::MonitorSelection> = <::bevy_window::prelude::MonitorSelection as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::MonitorSelection>,
                    other: Ref<::bevy_window::prelude::MonitorSelection>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::MonitorSelection as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::MonitorSelection,
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
                ::bevy_window::prelude::MonitorSelection,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_video_mode_selection_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::VideoModeSelection,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::VideoModeSelection>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::VideoModeSelection as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::VideoModeSelection>| {
                    let output: Val<::bevy_window::prelude::VideoModeSelection> = {
                        {
                            let output: Val<
                                ::bevy_window::prelude::VideoModeSelection,
                            > = <::bevy_window::prelude::VideoModeSelection as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::VideoModeSelection>,
                    other: Ref<::bevy_window::prelude::VideoModeSelection>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::VideoModeSelection as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::VideoModeSelection,
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
                ::bevy_window::prelude::VideoModeSelection,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::Window,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: Val<::bevy_window::prelude::Window> = {
                        {
                            let output: Val<::bevy_window::prelude::Window> = <::bevy_window::prelude::Window as ::core::clone::Clone>::clone(
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
                "height",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::prelude::Window::height(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area height in logical pixels.\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            )
            .register_documented(
                "physical_height",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: u32 = {
                        {
                            let output: u32 = ::bevy_window::prelude::Window::physical_height(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area height in physical pixels.\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            )
            .register_documented(
                "physical_size",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: Val<::bevy_math::UVec2> = {
                        {
                            let output: Val<::bevy_math::UVec2> = ::bevy_window::prelude::Window::physical_size(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client size in physical pixels\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            )
            .register_documented(
                "physical_width",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: u32 = {
                        {
                            let output: u32 = ::bevy_window::prelude::Window::physical_width(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area width in physical pixels.\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            )
            .register_documented(
                "scale_factor",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::prelude::Window::scale_factor(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's scale factor.\n Ratio of physical size to logical size, see [`WindowResolution`].",
                &["_self"],
            )
            .register_documented(
                "set_maximized",
                |mut _self: Mut<::bevy_window::prelude::Window>, maximized: bool| {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::Window::set_maximized(
                                    &mut _self,
                                    maximized,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Setting to true will attempt to maximize the window.\n Setting to false will attempt to un-maximize the window.",
                &["_self", "maximized"],
            )
            .register_documented(
                "set_minimized",
                |mut _self: Mut<::bevy_window::prelude::Window>, minimized: bool| {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::Window::set_minimized(
                                    &mut _self,
                                    minimized,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Setting to true will attempt to minimize the window.\n Setting to false will attempt to un-minimize the window.",
                &["_self", "minimized"],
            )
            .register_documented(
                "size",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: Val<::bevy_math::Vec2> = {
                        {
                            let output: Val<::bevy_math::Vec2> = ::bevy_window::prelude::Window::size(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client size in logical pixels\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            )
            .register_documented(
                "start_drag_move",
                |mut _self: Mut<::bevy_window::prelude::Window>| {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::Window::start_drag_move(
                                    &mut _self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Calling this will attempt to start a drag-move of the window.\n There is no guarantee that this will work unless the left mouse button was\n pressed immediately before this function was called.",
                &["_self"],
            )
            .register_documented(
                "start_drag_resize",
                |
                    mut _self: Mut<::bevy_window::prelude::Window>,
                    direction: Val<::bevy_math::CompassOctant>|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::Window::start_drag_resize(
                                    &mut _self,
                                    direction.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Calling this will attempt to start a drag-resize of the window.\n There is no guarantee that this will work unless the left mouse button was\n pressed immediately before this function was called.",
                &["_self", "direction"],
            )
            .register_documented(
                "width",
                |_self: Ref<::bevy_window::prelude::Window>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::prelude::Window::width(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area width in logical pixels.\n See [`WindowResolution`] for an explanation about logical/physical sizes.",
                &["_self"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::prelude::Window,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_moved_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::WindowMoved,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::prelude::WindowMoved>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::prelude::WindowMoved as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::prelude::WindowMoved>| {
                    let output: Val<::bevy_window::prelude::WindowMoved> = {
                        {
                            let output: Val<::bevy_window::prelude::WindowMoved> = <::bevy_window::prelude::WindowMoved as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::WindowMoved>,
                    other: Ref<::bevy_window::prelude::WindowMoved>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::WindowMoved as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::WindowMoved,
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
                ::bevy_window::prelude::WindowMoved,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_position_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::WindowPosition,
        >::new(world)
            .register_documented(
                "center",
                |
                    mut _self: Mut<::bevy_window::prelude::WindowPosition>,
                    monitor: Val<::bevy_window::prelude::MonitorSelection>|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::WindowPosition::center(
                                    &mut _self,
                                    monitor.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window to a specific monitor.",
                &["_self", "monitor"],
            )
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::prelude::WindowPosition>| {
                    let output: Val<::bevy_window::prelude::WindowPosition> = {
                        {
                            let output: Val<::bevy_window::prelude::WindowPosition> = <::bevy_window::prelude::WindowPosition as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::WindowPosition>,
                    other: Ref<::bevy_window::prelude::WindowPosition>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::WindowPosition as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::WindowPosition,
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
                |position: Val<::bevy_math::IVec2>| {
                    let output: Val<::bevy_window::prelude::WindowPosition> = {
                        {
                            let output: Val<::bevy_window::prelude::WindowPosition> = ::bevy_window::prelude::WindowPosition::new(
                                    position.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new [`WindowPosition`] at a position.",
                &["position"],
            )
            .register_documented(
                "set",
                |
                    mut _self: Mut<::bevy_window::prelude::WindowPosition>,
                    position: Val<::bevy_math::IVec2>|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::prelude::WindowPosition::set(
                                    &mut _self,
                                    position.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the position to a specific point.",
                &["_self", "position"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::prelude::WindowPosition,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_resize_constraints_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::prelude::WindowResizeConstraints,
        >::new(world)
            .register_documented(
                "check_constraints",
                |_self: Ref<::bevy_window::prelude::WindowResizeConstraints>| {
                    let output: Val<::bevy_window::prelude::WindowResizeConstraints> = {
                        {
                            let output: Val<
                                ::bevy_window::prelude::WindowResizeConstraints,
                            > = ::bevy_window::prelude::WindowResizeConstraints::check_constraints(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Checks if the constraints are valid.\n Will output warnings if it isn't.",
                &["_self"],
            )
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::prelude::WindowResizeConstraints>| {
                    let output: Val<::bevy_window::prelude::WindowResizeConstraints> = {
                        {
                            let output: Val<
                                ::bevy_window::prelude::WindowResizeConstraints,
                            > = <::bevy_window::prelude::WindowResizeConstraints as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::prelude::WindowResizeConstraints>,
                    other: Ref<::bevy_window::prelude::WindowResizeConstraints>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::prelude::WindowResizeConstraints as ::core::cmp::PartialEq<
                                ::bevy_window::prelude::WindowResizeConstraints,
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
                ::bevy_window::prelude::WindowResizeConstraints,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_event_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowEvent>| {
                    let output: Val<::bevy_window::WindowEvent> = {
                        {
                            let output: Val<::bevy_window::WindowEvent> = <::bevy_window::WindowEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowEvent>,
                    other: Ref<::bevy_window::WindowEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowEvent as ::core::cmp::PartialEq<
                                ::bevy_window::WindowEvent,
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
                ::bevy_window::WindowEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_resized_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowResized,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowResized>| {
                    let output: Val<::bevy_window::WindowResized> = {
                        {
                            let output: Val<::bevy_window::WindowResized> = <::bevy_window::WindowResized as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowResized>,
                    other: Ref<::bevy_window::WindowResized>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowResized as ::core::cmp::PartialEq<
                                ::bevy_window::WindowResized,
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
                ::bevy_window::WindowResized,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_created_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowCreated,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowCreated>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowCreated as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowCreated>| {
                    let output: Val<::bevy_window::WindowCreated> = {
                        {
                            let output: Val<::bevy_window::WindowCreated> = <::bevy_window::WindowCreated as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowCreated>,
                    other: Ref<::bevy_window::WindowCreated>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowCreated as ::core::cmp::PartialEq<
                                ::bevy_window::WindowCreated,
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
                ::bevy_window::WindowCreated,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_closing_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowClosing,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowClosing>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowClosing as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowClosing>| {
                    let output: Val<::bevy_window::WindowClosing> = {
                        {
                            let output: Val<::bevy_window::WindowClosing> = <::bevy_window::WindowClosing as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowClosing>,
                    other: Ref<::bevy_window::WindowClosing>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowClosing as ::core::cmp::PartialEq<
                                ::bevy_window::WindowClosing,
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
                ::bevy_window::WindowClosing,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_closed_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowClosed,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowClosed>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowClosed as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowClosed>| {
                    let output: Val<::bevy_window::WindowClosed> = {
                        {
                            let output: Val<::bevy_window::WindowClosed> = <::bevy_window::WindowClosed as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowClosed>,
                    other: Ref<::bevy_window::WindowClosed>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowClosed as ::core::cmp::PartialEq<
                                ::bevy_window::WindowClosed,
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
                ::bevy_window::WindowClosed,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_close_requested_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowCloseRequested,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowCloseRequested>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowCloseRequested as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowCloseRequested>| {
                    let output: Val<::bevy_window::WindowCloseRequested> = {
                        {
                            let output: Val<::bevy_window::WindowCloseRequested> = <::bevy_window::WindowCloseRequested as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowCloseRequested>,
                    other: Ref<::bevy_window::WindowCloseRequested>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowCloseRequested as ::core::cmp::PartialEq<
                                ::bevy_window::WindowCloseRequested,
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
                ::bevy_window::WindowCloseRequested,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_destroyed_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowDestroyed,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowDestroyed>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowDestroyed as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowDestroyed>| {
                    let output: Val<::bevy_window::WindowDestroyed> = {
                        {
                            let output: Val<::bevy_window::WindowDestroyed> = <::bevy_window::WindowDestroyed as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowDestroyed>,
                    other: Ref<::bevy_window::WindowDestroyed>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowDestroyed as ::core::cmp::PartialEq<
                                ::bevy_window::WindowDestroyed,
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
                ::bevy_window::WindowDestroyed,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_request_redraw_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::RequestRedraw,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::RequestRedraw>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::RequestRedraw as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::RequestRedraw>| {
                    let output: Val<::bevy_window::RequestRedraw> = {
                        {
                            let output: Val<::bevy_window::RequestRedraw> = <::bevy_window::RequestRedraw as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::RequestRedraw>,
                    other: Ref<::bevy_window::RequestRedraw>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::RequestRedraw as ::core::cmp::PartialEq<
                                ::bevy_window::RequestRedraw,
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
                ::bevy_window::RequestRedraw,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_focused_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowFocused,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowFocused>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowFocused as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowFocused>| {
                    let output: Val<::bevy_window::WindowFocused> = {
                        {
                            let output: Val<::bevy_window::WindowFocused> = <::bevy_window::WindowFocused as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowFocused>,
                    other: Ref<::bevy_window::WindowFocused>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowFocused as ::core::cmp::PartialEq<
                                ::bevy_window::WindowFocused,
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
                ::bevy_window::WindowFocused,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_occluded_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowOccluded,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowOccluded>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowOccluded as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowOccluded>| {
                    let output: Val<::bevy_window::WindowOccluded> = {
                        {
                            let output: Val<::bevy_window::WindowOccluded> = <::bevy_window::WindowOccluded as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowOccluded>,
                    other: Ref<::bevy_window::WindowOccluded>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowOccluded as ::core::cmp::PartialEq<
                                ::bevy_window::WindowOccluded,
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
                ::bevy_window::WindowOccluded,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_scale_factor_changed_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowScaleFactorChanged,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowScaleFactorChanged>| {
                    let output: Val<::bevy_window::WindowScaleFactorChanged> = {
                        {
                            let output: Val<::bevy_window::WindowScaleFactorChanged> = <::bevy_window::WindowScaleFactorChanged as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowScaleFactorChanged>,
                    other: Ref<::bevy_window::WindowScaleFactorChanged>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowScaleFactorChanged as ::core::cmp::PartialEq<
                                ::bevy_window::WindowScaleFactorChanged,
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
                ::bevy_window::WindowScaleFactorChanged,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_backend_scale_factor_changed_functions(
        world: &mut World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowBackendScaleFactorChanged,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowBackendScaleFactorChanged>| {
                    let output: Val<::bevy_window::WindowBackendScaleFactorChanged> = {
                        {
                            let output: Val<
                                ::bevy_window::WindowBackendScaleFactorChanged,
                            > = <::bevy_window::WindowBackendScaleFactorChanged as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowBackendScaleFactorChanged>,
                    other: Ref<::bevy_window::WindowBackendScaleFactorChanged>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowBackendScaleFactorChanged as ::core::cmp::PartialEq<
                                ::bevy_window::WindowBackendScaleFactorChanged,
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
                ::bevy_window::WindowBackendScaleFactorChanged,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_theme_changed_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowThemeChanged,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowThemeChanged>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowThemeChanged as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowThemeChanged>| {
                    let output: Val<::bevy_window::WindowThemeChanged> = {
                        {
                            let output: Val<::bevy_window::WindowThemeChanged> = <::bevy_window::WindowThemeChanged as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowThemeChanged>,
                    other: Ref<::bevy_window::WindowThemeChanged>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowThemeChanged as ::core::cmp::PartialEq<
                                ::bevy_window::WindowThemeChanged,
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
                ::bevy_window::WindowThemeChanged,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_app_lifecycle_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::AppLifecycle,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::AppLifecycle>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::AppLifecycle as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::AppLifecycle>| {
                    let output: Val<::bevy_window::AppLifecycle> = {
                        {
                            let output: Val<::bevy_window::AppLifecycle> = <::bevy_window::AppLifecycle as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::AppLifecycle>,
                    other: Ref<::bevy_window::AppLifecycle>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::AppLifecycle as ::core::cmp::PartialEq<
                                ::bevy_window::AppLifecycle,
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
                "is_active",
                |_self: Ref<::bevy_window::AppLifecycle>| {
                    let output: bool = {
                        {
                            let output: bool = ::bevy_window::AppLifecycle::is_active(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Return `true` if the app can be updated.",
                &["_self"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::AppLifecycle,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_primary_window_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::PrimaryWindow,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::PrimaryWindow>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::PrimaryWindow as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::PrimaryWindow>| {
                    let output: Val<::bevy_window::PrimaryWindow> = {
                        {
                            let output: Val<::bevy_window::PrimaryWindow> = <::bevy_window::PrimaryWindow as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::PrimaryWindow>,
                    other: Ref<::bevy_window::PrimaryWindow>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::PrimaryWindow as ::core::cmp::PartialEq<
                                ::bevy_window::PrimaryWindow,
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
                ::bevy_window::PrimaryWindow,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_monitor_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::Monitor,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::Monitor>| {
                    let output: Val<::bevy_window::Monitor> = {
                        {
                            let output: Val<::bevy_window::Monitor> = <::bevy_window::Monitor as ::core::clone::Clone>::clone(
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
                "physical_size",
                |_self: Ref<::bevy_window::Monitor>| {
                    let output: Val<::bevy_math::UVec2> = {
                        {
                            let output: Val<::bevy_math::UVec2> = ::bevy_window::Monitor::physical_size(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the physical size of the monitor in pixels",
                &["_self"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::Monitor,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_theme_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowTheme,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowTheme>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowTheme as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowTheme>| {
                    let output: Val<::bevy_window::WindowTheme> = {
                        {
                            let output: Val<::bevy_window::WindowTheme> = <::bevy_window::WindowTheme as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowTheme>,
                    other: Ref<::bevy_window::WindowTheme>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowTheme as ::core::cmp::PartialEq<
                                ::bevy_window::WindowTheme,
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
                ::bevy_window::WindowTheme,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_video_mode_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::VideoMode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::VideoMode>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::VideoMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::VideoMode>| {
                    let output: Val<::bevy_window::VideoMode> = {
                        {
                            let output: Val<::bevy_window::VideoMode> = <::bevy_window::VideoMode as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::VideoMode>,
                    other: Ref<::bevy_window::VideoMode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::VideoMode as ::core::cmp::PartialEq<
                                ::bevy_window::VideoMode,
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
                ::bevy_window::VideoMode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_primary_monitor_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::PrimaryMonitor,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::PrimaryMonitor>| {
                    let output: Val<::bevy_window::PrimaryMonitor> = {
                        {
                            let output: Val<::bevy_window::PrimaryMonitor> = <::bevy_window::PrimaryMonitor as ::core::clone::Clone>::clone(
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
                ::bevy_window::PrimaryMonitor,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_system_cursor_icon_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::SystemCursorIcon,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::SystemCursorIcon>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::SystemCursorIcon as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::SystemCursorIcon>| {
                    let output: Val<::bevy_window::SystemCursorIcon> = {
                        {
                            let output: Val<::bevy_window::SystemCursorIcon> = <::bevy_window::SystemCursorIcon as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::SystemCursorIcon>,
                    other: Ref<::bevy_window::SystemCursorIcon>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::SystemCursorIcon as ::core::cmp::PartialEq<
                                ::bevy_window::SystemCursorIcon,
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
                ::bevy_window::SystemCursorIcon,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_ref_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowRef,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowRef>| {
                    let output: Val<::bevy_window::WindowRef> = {
                        {
                            let output: Val<::bevy_window::WindowRef> = <::bevy_window::WindowRef as ::core::clone::Clone>::clone(
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
                ::bevy_window::WindowRef,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_normalized_window_ref_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::NormalizedWindowRef,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::NormalizedWindowRef>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::NormalizedWindowRef as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::NormalizedWindowRef>| {
                    let output: Val<::bevy_window::NormalizedWindowRef> = {
                        {
                            let output: Val<::bevy_window::NormalizedWindowRef> = <::bevy_window::NormalizedWindowRef as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::NormalizedWindowRef>,
                    other: Ref<::bevy_window::NormalizedWindowRef>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::NormalizedWindowRef as ::core::cmp::PartialEq<
                                ::bevy_window::NormalizedWindowRef,
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
                ::bevy_window::NormalizedWindowRef,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_cursor_options_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::CursorOptions,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::CursorOptions>| {
                    let output: Val<::bevy_window::CursorOptions> = {
                        {
                            let output: Val<::bevy_window::CursorOptions> = <::bevy_window::CursorOptions as ::core::clone::Clone>::clone(
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
                ::bevy_window::CursorOptions,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_present_mode_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::PresentMode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::PresentMode>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::PresentMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::PresentMode>| {
                    let output: Val<::bevy_window::PresentMode> = {
                        {
                            let output: Val<::bevy_window::PresentMode> = <::bevy_window::PresentMode as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::PresentMode>,
                    other: Ref<::bevy_window::PresentMode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::PresentMode as ::core::cmp::PartialEq<
                                ::bevy_window::PresentMode,
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
                ::bevy_window::PresentMode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_mode_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowMode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowMode>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowMode>| {
                    let output: Val<::bevy_window::WindowMode> = {
                        {
                            let output: Val<::bevy_window::WindowMode> = <::bevy_window::WindowMode as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowMode>,
                    other: Ref<::bevy_window::WindowMode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowMode as ::core::cmp::PartialEq<
                                ::bevy_window::WindowMode,
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
                ::bevy_window::WindowMode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_resolution_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowResolution,
        >::new(world)
            .register_documented(
                "base_scale_factor",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::WindowResolution::base_scale_factor(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window scale factor as reported by the window backend.\n This value is unaffected by [`WindowResolution::scale_factor_override`].",
                &["_self"],
            )
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: Val<::bevy_window::WindowResolution> = {
                        {
                            let output: Val<::bevy_window::WindowResolution> = <::bevy_window::WindowResolution as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowResolution>,
                    other: Ref<::bevy_window::WindowResolution>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowResolution as ::core::cmp::PartialEq<
                                ::bevy_window::WindowResolution,
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
                "height",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::WindowResolution::height(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area height in logical pixels.",
                &["_self"],
            )
            .register_documented(
                "new",
                |physical_width: f32, physical_height: f32| {
                    let output: Val<::bevy_window::WindowResolution> = {
                        {
                            let output: Val<::bevy_window::WindowResolution> = ::bevy_window::WindowResolution::new(
                                    physical_width,
                                    physical_height,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new [`WindowResolution`].",
                &["physical_width", "physical_height"],
            )
            .register_documented(
                "physical_height",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: u32 = {
                        {
                            let output: u32 = ::bevy_window::WindowResolution::physical_height(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area height in physical pixels.",
                &["_self"],
            )
            .register_documented(
                "physical_size",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: Val<::bevy_math::UVec2> = {
                        {
                            let output: Val<::bevy_math::UVec2> = ::bevy_window::WindowResolution::physical_size(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client size in physical pixels",
                &["_self"],
            )
            .register_documented(
                "physical_width",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: u32 = {
                        {
                            let output: u32 = ::bevy_window::WindowResolution::physical_width(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area width in physical pixels.",
                &["_self"],
            )
            .register_documented(
                "scale_factor",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::WindowResolution::scale_factor(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The ratio of physical pixels to logical pixels.\n `physical_pixels = logical_pixels * scale_factor`",
                &["_self"],
            )
            .register_documented(
                "scale_factor_override",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: ::core::option::Option<f32> = {
                        {
                            let output: ::core::option::Option<f32> = ::bevy_window::WindowResolution::scale_factor_override(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The scale factor set with [`WindowResolution::set_scale_factor_override`].\n This value may be different from the scale factor reported by the window backend.",
                &["_self"],
            )
            .register_documented(
                "set",
                |
                    mut _self: Mut<::bevy_window::WindowResolution>,
                    width: f32,
                    height: f32|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::WindowResolution::set(
                                    &mut _self,
                                    width,
                                    height,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window's logical resolution.",
                &["_self", "width", "height"],
            )
            .register_documented(
                "set_physical_resolution",
                |
                    mut _self: Mut<::bevy_window::WindowResolution>,
                    width: u32,
                    height: u32|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::WindowResolution::set_physical_resolution(
                                    &mut _self,
                                    width,
                                    height,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window's physical resolution.\n This will ignore the scale factor setting, so most of the time you should\n prefer to use [`WindowResolution::set`].",
                &["_self", "width", "height"],
            )
            .register_documented(
                "set_scale_factor",
                |mut _self: Mut<::bevy_window::WindowResolution>, scale_factor: f32| {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::WindowResolution::set_scale_factor(
                                    &mut _self,
                                    scale_factor,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window's scale factor, this may get overridden by the backend.",
                &["_self", "scale_factor"],
            )
            .register_documented(
                "set_scale_factor_and_apply_to_physical_size",
                |mut _self: Mut<::bevy_window::WindowResolution>, scale_factor: f32| {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::WindowResolution::set_scale_factor_and_apply_to_physical_size(
                                    &mut _self,
                                    scale_factor,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window's scale factor, and apply it to the currently known physical size.\n This may get overridden by the backend. This is mostly useful on window creation,\n so that the window is created with the expected size instead of waiting for a resize\n event after its creation.",
                &["_self", "scale_factor"],
            )
            .register_documented(
                "set_scale_factor_override",
                |
                    mut _self: Mut<::bevy_window::WindowResolution>,
                    scale_factor_override: ::core::option::Option<f32>|
                {
                    let output: () = {
                        {
                            let output: () = ::bevy_window::WindowResolution::set_scale_factor_override(
                                    &mut _self,
                                    scale_factor_override,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Set the window's scale factor, this will be used over what the backend decides.\n This can change the logical and physical sizes if the resulting physical\n size is not within the limits.",
                &["_self", "scale_factor_override"],
            )
            .register_documented(
                "size",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: Val<::bevy_math::Vec2> = {
                        {
                            let output: Val<::bevy_math::Vec2> = ::bevy_window::WindowResolution::size(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client size in logical pixels",
                &["_self"],
            )
            .register_documented(
                "width",
                |_self: Ref<::bevy_window::WindowResolution>| {
                    let output: f32 = {
                        {
                            let output: f32 = ::bevy_window::WindowResolution::width(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " The window's client area width in logical pixels.",
                &["_self"],
            )
            .register_documented(
                "with_scale_factor_override",
                |
                    _self: Val<::bevy_window::WindowResolution>,
                    scale_factor_override: f32|
                {
                    let output: Val<::bevy_window::WindowResolution> = {
                        {
                            let output: Val<::bevy_window::WindowResolution> = ::bevy_window::WindowResolution::with_scale_factor_override(
                                    _self.into_inner(),
                                    scale_factor_override,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Builder method for adding a scale factor override to the resolution.",
                &["_self", "scale_factor_override"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::WindowResolution,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_composite_alpha_mode_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::CompositeAlphaMode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::CompositeAlphaMode>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::CompositeAlphaMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::CompositeAlphaMode>| {
                    let output: Val<::bevy_window::CompositeAlphaMode> = {
                        {
                            let output: Val<::bevy_window::CompositeAlphaMode> = <::bevy_window::CompositeAlphaMode as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::CompositeAlphaMode>,
                    other: Ref<::bevy_window::CompositeAlphaMode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::CompositeAlphaMode as ::core::cmp::PartialEq<
                                ::bevy_window::CompositeAlphaMode,
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
                ::bevy_window::CompositeAlphaMode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_enabled_buttons_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::EnabledButtons,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::EnabledButtons>| {
                    let output: Val<::bevy_window::EnabledButtons> = {
                        {
                            let output: Val<::bevy_window::EnabledButtons> = <::bevy_window::EnabledButtons as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::EnabledButtons>,
                    other: Ref<::bevy_window::EnabledButtons>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::EnabledButtons as ::core::cmp::PartialEq<
                                ::bevy_window::EnabledButtons,
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
                ::bevy_window::EnabledButtons,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_window_level_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::WindowLevel,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::WindowLevel>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::WindowLevel as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::WindowLevel>| {
                    let output: Val<::bevy_window::WindowLevel> = {
                        {
                            let output: Val<::bevy_window::WindowLevel> = <::bevy_window::WindowLevel as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::WindowLevel>,
                    other: Ref<::bevy_window::WindowLevel>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::WindowLevel as ::core::cmp::PartialEq<
                                ::bevy_window::WindowLevel,
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
                ::bevy_window::WindowLevel,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_internal_window_state_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::InternalWindowState,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<::bevy_window::InternalWindowState>| {
                    let output: Val<::bevy_window::InternalWindowState> = {
                        {
                            let output: Val<::bevy_window::InternalWindowState> = <::bevy_window::InternalWindowState as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::InternalWindowState>,
                    other: Ref<::bevy_window::InternalWindowState>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::InternalWindowState as ::core::cmp::PartialEq<
                                ::bevy_window::InternalWindowState,
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
                "take_maximize_request",
                |mut _self: Mut<::bevy_window::InternalWindowState>| {
                    let output: ::core::option::Option<bool> = {
                        {
                            let output: ::core::option::Option<bool> = ::bevy_window::InternalWindowState::take_maximize_request(
                                    &mut _self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Consumes the current maximize request, if it exists. This should only be called by window backends.",
                &["_self"],
            )
            .register_documented(
                "take_minimize_request",
                |mut _self: Mut<::bevy_window::InternalWindowState>| {
                    let output: ::core::option::Option<bool> = {
                        {
                            let output: ::core::option::Option<bool> = ::bevy_window::InternalWindowState::take_minimize_request(
                                    &mut _self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Consumes the current minimize request, if it exists. This should only be called by window backends.",
                &["_self"],
            )
            .register_documented(
                "take_move_request",
                |mut _self: Mut<::bevy_window::InternalWindowState>| {
                    let output: bool = {
                        {
                            let output: bool = ::bevy_window::InternalWindowState::take_move_request(
                                    &mut _self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Consumes the current move request, if it exists. This should only be called by window backends.",
                &["_self"],
            );
        let registry = world.get_resource_or_init::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                ::bevy_window::InternalWindowState,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_cursor_grab_mode_functions(world: &mut World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            ::bevy_window::CursorGrabMode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<::bevy_window::CursorGrabMode>| {
                    let output: () = {
                        {
                            let output: () = <::bevy_window::CursorGrabMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<::bevy_window::CursorGrabMode>| {
                    let output: Val<::bevy_window::CursorGrabMode> = {
                        {
                            let output: Val<::bevy_window::CursorGrabMode> = <::bevy_window::CursorGrabMode as ::core::clone::Clone>::clone(
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
                    _self: Ref<::bevy_window::CursorGrabMode>,
                    other: Ref<::bevy_window::CursorGrabMode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <::bevy_window::CursorGrabMode as ::core::cmp::PartialEq<
                                ::bevy_window::CursorGrabMode,
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
                ::bevy_window::CursorGrabMode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    impl Plugin for BevyWindowScriptingPlugin {
        fn build(&self, app: &mut App) {
            let mut world = app.world_mut();
            register_cursor_entered_functions(&mut world);
            register_cursor_left_functions(&mut world);
            register_cursor_moved_functions(&mut world);
            register_file_drag_and_drop_functions(&mut world);
            register_ime_functions(&mut world);
            register_monitor_selection_functions(&mut world);
            register_video_mode_selection_functions(&mut world);
            register_window_functions(&mut world);
            register_window_moved_functions(&mut world);
            register_window_position_functions(&mut world);
            register_window_resize_constraints_functions(&mut world);
            register_window_event_functions(&mut world);
            register_window_resized_functions(&mut world);
            register_window_created_functions(&mut world);
            register_window_closing_functions(&mut world);
            register_window_closed_functions(&mut world);
            register_window_close_requested_functions(&mut world);
            register_window_destroyed_functions(&mut world);
            register_request_redraw_functions(&mut world);
            register_window_focused_functions(&mut world);
            register_window_occluded_functions(&mut world);
            register_window_scale_factor_changed_functions(&mut world);
            register_window_backend_scale_factor_changed_functions(&mut world);
            register_window_theme_changed_functions(&mut world);
            register_app_lifecycle_functions(&mut world);
            register_primary_window_functions(&mut world);
            register_monitor_functions(&mut world);
            register_window_theme_functions(&mut world);
            register_video_mode_functions(&mut world);
            register_primary_monitor_functions(&mut world);
            register_system_cursor_icon_functions(&mut world);
            register_window_ref_functions(&mut world);
            register_normalized_window_ref_functions(&mut world);
            register_cursor_options_functions(&mut world);
            register_present_mode_functions(&mut world);
            register_window_mode_functions(&mut world);
            register_window_resolution_functions(&mut world);
            register_composite_alpha_mode_functions(&mut world);
            register_enabled_buttons_functions(&mut world);
            register_window_level_functions(&mut world);
            register_internal_window_state_functions(&mut world);
            register_cursor_grab_mode_functions(&mut world);
        }
    }