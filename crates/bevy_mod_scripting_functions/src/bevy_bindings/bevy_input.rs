    #![allow(clippy::all)]
    #![allow(unused, deprecated, dead_code)]
    use bevy_mod_scripting_core::bindings::{
        ReflectReference,
        function::{
            from::{Ref, Mut, Val},
            namespace::NamespaceBuilder,
        },
    };
    use bevy_mod_scripting_derive::script_bindings;
    use crate::*;
    pub struct BevyInputScriptingPlugin;
    pub(crate) fn register_gamepad_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::Gamepad,
        >::new(world)
            .register_documented(
                "dpad",
                |_self: Ref<bevy::input::gamepad::Gamepad>| {
                    let output: Val<bevy::math::Vec2> = {
                        {
                            let output: Val<bevy::math::Vec2> = bevy::input::gamepad::Gamepad::dpad(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the directional pad as a [`Vec2`].",
                &["_self"],
            )
            .register_documented(
                "just_pressed",
                |
                    _self: Ref<bevy::input::gamepad::Gamepad>,
                    button_type: Val<bevy::input::gamepad::GamepadButton>|
                {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::Gamepad::just_pressed(
                                    &_self,
                                    button_type.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns `true` if the [`GamepadButton`] has been pressed during the current frame.\n Note: This function does not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_released`].",
                &["_self", "button_type"],
            )
            .register_documented(
                "just_released",
                |
                    _self: Ref<bevy::input::gamepad::Gamepad>,
                    button_type: Val<bevy::input::gamepad::GamepadButton>|
                {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::Gamepad::just_released(
                                    &_self,
                                    button_type.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns `true` if the [`GamepadButton`] has been released during the current frame.\n Note: This function does not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_pressed`].",
                &["_self", "button_type"],
            )
            .register_documented(
                "left_stick",
                |_self: Ref<bevy::input::gamepad::Gamepad>| {
                    let output: Val<bevy::math::Vec2> = {
                        {
                            let output: Val<bevy::math::Vec2> = bevy::input::gamepad::Gamepad::left_stick(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the left stick as a [`Vec2`].",
                &["_self"],
            )
            .register_documented(
                "pressed",
                |
                    _self: Ref<bevy::input::gamepad::Gamepad>,
                    button_type: Val<bevy::input::gamepad::GamepadButton>|
                {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::Gamepad::pressed(
                                    &_self,
                                    button_type.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns `true` if the [`GamepadButton`] has been pressed.",
                &["_self", "button_type"],
            )
            .register_documented(
                "product_id",
                |_self: Ref<bevy::input::gamepad::Gamepad>| {
                    let output: ::core::option::Option<u16> = {
                        {
                            let output: ::core::option::Option<u16> = bevy::input::gamepad::Gamepad::product_id(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the USB product ID as assigned by the [vendor], if available.\n [vendor]: Self::vendor_id",
                &["_self"],
            )
            .register_documented(
                "right_stick",
                |_self: Ref<bevy::input::gamepad::Gamepad>| {
                    let output: Val<bevy::math::Vec2> = {
                        {
                            let output: Val<bevy::math::Vec2> = bevy::input::gamepad::Gamepad::right_stick(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the right stick as a [`Vec2`].",
                &["_self"],
            )
            .register_documented(
                "vendor_id",
                |_self: Ref<bevy::input::gamepad::Gamepad>| {
                    let output: ::core::option::Option<u16> = {
                        {
                            let output: ::core::option::Option<u16> = bevy::input::gamepad::Gamepad::vendor_id(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns the USB vendor ID as assigned by the USB-IF, if available.",
                &["_self"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::Gamepad,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_axis_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadAxis,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::gamepad::GamepadAxis>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::gamepad::GamepadAxis as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::gamepad::GamepadAxis>| {
                    let output: Val<bevy::input::gamepad::GamepadAxis> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadAxis> = <bevy::input::gamepad::GamepadAxis as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadAxis>,
                    other: Ref<bevy::input::gamepad::GamepadAxis>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadAxis as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadAxis,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadAxis,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_button_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadButton,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::gamepad::GamepadButton>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::gamepad::GamepadButton as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::gamepad::GamepadButton>| {
                    let output: Val<bevy::input::gamepad::GamepadButton> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadButton> = <bevy::input::gamepad::GamepadButton as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadButton>,
                    other: Ref<bevy::input::gamepad::GamepadButton>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadButton as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadButton,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadButton,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_settings_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadSettings,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadSettings>| {
                    let output: Val<bevy::input::gamepad::GamepadSettings> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadSettings> = <bevy::input::gamepad::GamepadSettings as ::core::clone::Clone>::clone(
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadSettings,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_key_code_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::KeyCode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::KeyCode>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::KeyCode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::KeyCode>| {
                    let output: Val<bevy::input::keyboard::KeyCode> = {
                        {
                            let output: Val<bevy::input::keyboard::KeyCode> = <bevy::input::keyboard::KeyCode as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::KeyCode>,
                    other: Ref<bevy::input::keyboard::KeyCode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::KeyCode as ::core::cmp::PartialEq<
                                bevy::input::keyboard::KeyCode,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::KeyCode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_mouse_button_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::MouseButton,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::mouse::MouseButton>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::mouse::MouseButton as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::mouse::MouseButton>| {
                    let output: Val<bevy::input::mouse::MouseButton> = {
                        {
                            let output: Val<bevy::input::mouse::MouseButton> = <bevy::input::mouse::MouseButton as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::MouseButton>,
                    other: Ref<bevy::input::mouse::MouseButton>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::MouseButton as ::core::cmp::PartialEq<
                                bevy::input::mouse::MouseButton,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::MouseButton,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_touch_input_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::touch::TouchInput,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::touch::TouchInput>| {
                    let output: Val<bevy::input::touch::TouchInput> = {
                        {
                            let output: Val<bevy::input::touch::TouchInput> = <bevy::input::touch::TouchInput as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::touch::TouchInput>,
                    other: Ref<bevy::input::touch::TouchInput>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::touch::TouchInput as ::core::cmp::PartialEq<
                                bevy::input::touch::TouchInput,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::touch::TouchInput,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_keyboard_focus_lost_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::KeyboardFocusLost,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::KeyboardFocusLost>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::KeyboardFocusLost as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::KeyboardFocusLost>| {
                    let output: Val<bevy::input::keyboard::KeyboardFocusLost> = {
                        {
                            let output: Val<bevy::input::keyboard::KeyboardFocusLost> = <bevy::input::keyboard::KeyboardFocusLost as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::KeyboardFocusLost>,
                    other: Ref<bevy::input::keyboard::KeyboardFocusLost>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::KeyboardFocusLost as ::core::cmp::PartialEq<
                                bevy::input::keyboard::KeyboardFocusLost,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::KeyboardFocusLost,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_keyboard_input_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::KeyboardInput,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::KeyboardInput>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::KeyboardInput as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::KeyboardInput>| {
                    let output: Val<bevy::input::keyboard::KeyboardInput> = {
                        {
                            let output: Val<bevy::input::keyboard::KeyboardInput> = <bevy::input::keyboard::KeyboardInput as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::KeyboardInput>,
                    other: Ref<bevy::input::keyboard::KeyboardInput>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::KeyboardInput as ::core::cmp::PartialEq<
                                bevy::input::keyboard::KeyboardInput,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::KeyboardInput,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_accumulated_mouse_motion_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::AccumulatedMouseMotion,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::mouse::AccumulatedMouseMotion>| {
                    let output: Val<bevy::input::mouse::AccumulatedMouseMotion> = {
                        {
                            let output: Val<
                                bevy::input::mouse::AccumulatedMouseMotion,
                            > = <bevy::input::mouse::AccumulatedMouseMotion as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::AccumulatedMouseMotion>,
                    other: Ref<bevy::input::mouse::AccumulatedMouseMotion>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::AccumulatedMouseMotion as ::core::cmp::PartialEq<
                                bevy::input::mouse::AccumulatedMouseMotion,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::AccumulatedMouseMotion,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_accumulated_mouse_scroll_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::AccumulatedMouseScroll,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::mouse::AccumulatedMouseScroll>| {
                    let output: Val<bevy::input::mouse::AccumulatedMouseScroll> = {
                        {
                            let output: Val<
                                bevy::input::mouse::AccumulatedMouseScroll,
                            > = <bevy::input::mouse::AccumulatedMouseScroll as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::AccumulatedMouseScroll>,
                    other: Ref<bevy::input::mouse::AccumulatedMouseScroll>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::AccumulatedMouseScroll as ::core::cmp::PartialEq<
                                bevy::input::mouse::AccumulatedMouseScroll,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::AccumulatedMouseScroll,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_mouse_button_input_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::MouseButtonInput,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::mouse::MouseButtonInput>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::mouse::MouseButtonInput as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::mouse::MouseButtonInput>| {
                    let output: Val<bevy::input::mouse::MouseButtonInput> = {
                        {
                            let output: Val<bevy::input::mouse::MouseButtonInput> = <bevy::input::mouse::MouseButtonInput as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::MouseButtonInput>,
                    other: Ref<bevy::input::mouse::MouseButtonInput>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::MouseButtonInput as ::core::cmp::PartialEq<
                                bevy::input::mouse::MouseButtonInput,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::MouseButtonInput,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_mouse_motion_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::MouseMotion,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::mouse::MouseMotion>| {
                    let output: Val<bevy::input::mouse::MouseMotion> = {
                        {
                            let output: Val<bevy::input::mouse::MouseMotion> = <bevy::input::mouse::MouseMotion as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::MouseMotion>,
                    other: Ref<bevy::input::mouse::MouseMotion>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::MouseMotion as ::core::cmp::PartialEq<
                                bevy::input::mouse::MouseMotion,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::MouseMotion,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_mouse_wheel_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::MouseWheel,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::mouse::MouseWheel>| {
                    let output: Val<bevy::input::mouse::MouseWheel> = {
                        {
                            let output: Val<bevy::input::mouse::MouseWheel> = <bevy::input::mouse::MouseWheel as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::MouseWheel>,
                    other: Ref<bevy::input::mouse::MouseWheel>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::MouseWheel as ::core::cmp::PartialEq<
                                bevy::input::mouse::MouseWheel,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::MouseWheel,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_axis_changed_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadAxisChangedEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadAxisChangedEvent>| {
                    let output: Val<bevy::input::gamepad::GamepadAxisChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadAxisChangedEvent,
                            > = <bevy::input::gamepad::GamepadAxisChangedEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadAxisChangedEvent>,
                    other: Ref<bevy::input::gamepad::GamepadAxisChangedEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadAxisChangedEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadAxisChangedEvent,
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
                |
                    entity: Val<bevy::ecs::entity::Entity>,
                    axis: Val<bevy::input::gamepad::GamepadAxis>,
                    value: f32|
                {
                    let output: Val<bevy::input::gamepad::GamepadAxisChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadAxisChangedEvent,
                            > = bevy::input::gamepad::GamepadAxisChangedEvent::new(
                                    entity.into_inner(),
                                    axis.into_inner(),
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new [`GamepadAxisChangedEvent`].",
                &["entity", "axis", "value"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadAxisChangedEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_button_changed_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadButtonChangedEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadButtonChangedEvent>| {
                    let output: Val<bevy::input::gamepad::GamepadButtonChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadButtonChangedEvent,
                            > = <bevy::input::gamepad::GamepadButtonChangedEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadButtonChangedEvent>,
                    other: Ref<bevy::input::gamepad::GamepadButtonChangedEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadButtonChangedEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadButtonChangedEvent,
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
                |
                    entity: Val<bevy::ecs::entity::Entity>,
                    button: Val<bevy::input::gamepad::GamepadButton>,
                    state: Val<bevy::input::ButtonState>,
                    value: f32|
                {
                    let output: Val<bevy::input::gamepad::GamepadButtonChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadButtonChangedEvent,
                            > = bevy::input::gamepad::GamepadButtonChangedEvent::new(
                                    entity.into_inner(),
                                    button.into_inner(),
                                    state.into_inner(),
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new [`GamepadButtonChangedEvent`].",
                &["entity", "button", "state", "value"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadButtonChangedEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_button_state_changed_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadButtonStateChangedEvent,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::gamepad::GamepadButtonStateChangedEvent>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::gamepad::GamepadButtonStateChangedEvent as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::gamepad::GamepadButtonStateChangedEvent>| {
                    let output: Val<
                        bevy::input::gamepad::GamepadButtonStateChangedEvent,
                    > = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadButtonStateChangedEvent,
                            > = <bevy::input::gamepad::GamepadButtonStateChangedEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadButtonStateChangedEvent>,
                    other: Ref<bevy::input::gamepad::GamepadButtonStateChangedEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadButtonStateChangedEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadButtonStateChangedEvent,
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
                |
                    entity: Val<bevy::ecs::entity::Entity>,
                    button: Val<bevy::input::gamepad::GamepadButton>,
                    state: Val<bevy::input::ButtonState>|
                {
                    let output: Val<
                        bevy::input::gamepad::GamepadButtonStateChangedEvent,
                    > = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadButtonStateChangedEvent,
                            > = bevy::input::gamepad::GamepadButtonStateChangedEvent::new(
                                    entity.into_inner(),
                                    button.into_inner(),
                                    state.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new [`GamepadButtonStateChangedEvent`].",
                &["entity", "button", "state"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadButtonStateChangedEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_connection_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadConnection,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadConnection>| {
                    let output: Val<bevy::input::gamepad::GamepadConnection> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadConnection> = <bevy::input::gamepad::GamepadConnection as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadConnection>,
                    other: Ref<bevy::input::gamepad::GamepadConnection>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadConnection as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadConnection,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadConnection,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_connection_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadConnectionEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadConnectionEvent>| {
                    let output: Val<bevy::input::gamepad::GamepadConnectionEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadConnectionEvent,
                            > = <bevy::input::gamepad::GamepadConnectionEvent as ::core::clone::Clone>::clone(
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
                "connected",
                |_self: Ref<bevy::input::gamepad::GamepadConnectionEvent>| {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::GamepadConnectionEvent::connected(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Whether the gamepad is connected.",
                &["_self"],
            )
            .register_documented(
                "disconnected",
                |_self: Ref<bevy::input::gamepad::GamepadConnectionEvent>| {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::GamepadConnectionEvent::disconnected(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Whether the gamepad is disconnected.",
                &["_self"],
            )
            .register_documented(
                "eq",
                |
                    _self: Ref<bevy::input::gamepad::GamepadConnectionEvent>,
                    other: Ref<bevy::input::gamepad::GamepadConnectionEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadConnectionEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadConnectionEvent,
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
                |
                    gamepad: Val<bevy::ecs::entity::Entity>,
                    connection: Val<bevy::input::gamepad::GamepadConnection>|
                {
                    let output: Val<bevy::input::gamepad::GamepadConnectionEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadConnectionEvent,
                            > = bevy::input::gamepad::GamepadConnectionEvent::new(
                                    gamepad.into_inner(),
                                    connection.into_inner(),
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a [`GamepadConnectionEvent`].",
                &["gamepad", "connection"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadConnectionEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_event_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadEvent>| {
                    let output: Val<bevy::input::gamepad::GamepadEvent> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadEvent> = <bevy::input::gamepad::GamepadEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadEvent>,
                    other: Ref<bevy::input::gamepad::GamepadEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadEvent,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_input_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadInput,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::gamepad::GamepadInput>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::gamepad::GamepadInput as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::gamepad::GamepadInput>| {
                    let output: Val<bevy::input::gamepad::GamepadInput> = {
                        {
                            let output: Val<bevy::input::gamepad::GamepadInput> = <bevy::input::gamepad::GamepadInput as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadInput>,
                    other: Ref<bevy::input::gamepad::GamepadInput>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadInput as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadInput,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadInput,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_rumble_request_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadRumbleRequest,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadRumbleRequest>| {
                    let output: Val<bevy::input::gamepad::GamepadRumbleRequest> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadRumbleRequest,
                            > = <bevy::input::gamepad::GamepadRumbleRequest as ::core::clone::Clone>::clone(
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
                "gamepad",
                |_self: Ref<bevy::input::gamepad::GamepadRumbleRequest>| {
                    let output: Val<bevy::ecs::entity::Entity> = {
                        {
                            let output: Val<bevy::ecs::entity::Entity> = bevy::input::gamepad::GamepadRumbleRequest::gamepad(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the [`Entity`] associated with this request.",
                &["_self"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadRumbleRequest,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_raw_gamepad_axis_changed_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::RawGamepadAxisChangedEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::RawGamepadAxisChangedEvent>| {
                    let output: Val<bevy::input::gamepad::RawGamepadAxisChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::RawGamepadAxisChangedEvent,
                            > = <bevy::input::gamepad::RawGamepadAxisChangedEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::RawGamepadAxisChangedEvent>,
                    other: Ref<bevy::input::gamepad::RawGamepadAxisChangedEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::RawGamepadAxisChangedEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::RawGamepadAxisChangedEvent,
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
                |
                    gamepad: Val<bevy::ecs::entity::Entity>,
                    axis_type: Val<bevy::input::gamepad::GamepadAxis>,
                    value: f32|
                {
                    let output: Val<bevy::input::gamepad::RawGamepadAxisChangedEvent> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::RawGamepadAxisChangedEvent,
                            > = bevy::input::gamepad::RawGamepadAxisChangedEvent::new(
                                    gamepad.into_inner(),
                                    axis_type.into_inner(),
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a [`RawGamepadAxisChangedEvent`].",
                &["gamepad", "axis_type", "value"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::RawGamepadAxisChangedEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_raw_gamepad_button_changed_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::RawGamepadButtonChangedEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::RawGamepadButtonChangedEvent>| {
                    let output: Val<
                        bevy::input::gamepad::RawGamepadButtonChangedEvent,
                    > = {
                        {
                            let output: Val<
                                bevy::input::gamepad::RawGamepadButtonChangedEvent,
                            > = <bevy::input::gamepad::RawGamepadButtonChangedEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::RawGamepadButtonChangedEvent>,
                    other: Ref<bevy::input::gamepad::RawGamepadButtonChangedEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::RawGamepadButtonChangedEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::RawGamepadButtonChangedEvent,
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
                |
                    gamepad: Val<bevy::ecs::entity::Entity>,
                    button_type: Val<bevy::input::gamepad::GamepadButton>,
                    value: f32|
                {
                    let output: Val<
                        bevy::input::gamepad::RawGamepadButtonChangedEvent,
                    > = {
                        {
                            let output: Val<
                                bevy::input::gamepad::RawGamepadButtonChangedEvent,
                            > = bevy::input::gamepad::RawGamepadButtonChangedEvent::new(
                                    gamepad.into_inner(),
                                    button_type.into_inner(),
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a [`RawGamepadButtonChangedEvent`].",
                &["gamepad", "button_type", "value"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::RawGamepadButtonChangedEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_raw_gamepad_event_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::RawGamepadEvent,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::RawGamepadEvent>| {
                    let output: Val<bevy::input::gamepad::RawGamepadEvent> = {
                        {
                            let output: Val<bevy::input::gamepad::RawGamepadEvent> = <bevy::input::gamepad::RawGamepadEvent as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::RawGamepadEvent>,
                    other: Ref<bevy::input::gamepad::RawGamepadEvent>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::RawGamepadEvent as ::core::cmp::PartialEq<
                                bevy::input::gamepad::RawGamepadEvent,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::RawGamepadEvent,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_pinch_gesture_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gestures::PinchGesture,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gestures::PinchGesture>| {
                    let output: Val<bevy::input::gestures::PinchGesture> = {
                        {
                            let output: Val<bevy::input::gestures::PinchGesture> = <bevy::input::gestures::PinchGesture as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gestures::PinchGesture>,
                    other: Ref<bevy::input::gestures::PinchGesture>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gestures::PinchGesture as ::core::cmp::PartialEq<
                                bevy::input::gestures::PinchGesture,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gestures::PinchGesture,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_rotation_gesture_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gestures::RotationGesture,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gestures::RotationGesture>| {
                    let output: Val<bevy::input::gestures::RotationGesture> = {
                        {
                            let output: Val<bevy::input::gestures::RotationGesture> = <bevy::input::gestures::RotationGesture as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gestures::RotationGesture>,
                    other: Ref<bevy::input::gestures::RotationGesture>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gestures::RotationGesture as ::core::cmp::PartialEq<
                                bevy::input::gestures::RotationGesture,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gestures::RotationGesture,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_double_tap_gesture_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gestures::DoubleTapGesture,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gestures::DoubleTapGesture>| {
                    let output: Val<bevy::input::gestures::DoubleTapGesture> = {
                        {
                            let output: Val<bevy::input::gestures::DoubleTapGesture> = <bevy::input::gestures::DoubleTapGesture as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gestures::DoubleTapGesture>,
                    other: Ref<bevy::input::gestures::DoubleTapGesture>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gestures::DoubleTapGesture as ::core::cmp::PartialEq<
                                bevy::input::gestures::DoubleTapGesture,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gestures::DoubleTapGesture,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_pan_gesture_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gestures::PanGesture,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gestures::PanGesture>| {
                    let output: Val<bevy::input::gestures::PanGesture> = {
                        {
                            let output: Val<bevy::input::gestures::PanGesture> = <bevy::input::gestures::PanGesture as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gestures::PanGesture>,
                    other: Ref<bevy::input::gestures::PanGesture>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gestures::PanGesture as ::core::cmp::PartialEq<
                                bevy::input::gestures::PanGesture,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gestures::PanGesture,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_button_state_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::ButtonState,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::ButtonState>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::ButtonState as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::ButtonState>| {
                    let output: Val<bevy::input::ButtonState> = {
                        {
                            let output: Val<bevy::input::ButtonState> = <bevy::input::ButtonState as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::ButtonState>,
                    other: Ref<bevy::input::ButtonState>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::ButtonState as ::core::cmp::PartialEq<
                                bevy::input::ButtonState,
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
                "is_pressed",
                |_self: Ref<bevy::input::ButtonState>| {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::ButtonState::is_pressed(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Is this button pressed?",
                &["_self"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::ButtonState,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_button_settings_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::ButtonSettings,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::ButtonSettings>| {
                    let output: Val<bevy::input::gamepad::ButtonSettings> = {
                        {
                            let output: Val<bevy::input::gamepad::ButtonSettings> = <bevy::input::gamepad::ButtonSettings as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::ButtonSettings>,
                    other: Ref<bevy::input::gamepad::ButtonSettings>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::ButtonSettings as ::core::cmp::PartialEq<
                                bevy::input::gamepad::ButtonSettings,
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
                "is_pressed",
                |_self: Ref<bevy::input::gamepad::ButtonSettings>, value: f32| {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::ButtonSettings::is_pressed(
                                    &_self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns `true` if the button is pressed.\n A button is considered pressed if the `value` passed is greater than or equal to the press threshold.",
                &["_self", "value"],
            )
            .register_documented(
                "is_released",
                |_self: Ref<bevy::input::gamepad::ButtonSettings>, value: f32| {
                    let output: bool = {
                        {
                            let output: bool = bevy::input::gamepad::ButtonSettings::is_released(
                                    &_self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Returns `true` if the button is released.\n A button is considered released if the `value` passed is lower than or equal to the release threshold.",
                &["_self", "value"],
            )
            .register_documented(
                "press_threshold",
                |_self: Ref<bevy::input::gamepad::ButtonSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::ButtonSettings::press_threshold(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the button input threshold above which the button is considered pressed.",
                &["_self"],
            )
            .register_documented(
                "release_threshold",
                |_self: Ref<bevy::input::gamepad::ButtonSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::ButtonSettings::release_threshold(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the button input threshold below which the button is considered released.",
                &["_self"],
            )
            .register_documented(
                "set_press_threshold",
                |mut _self: Mut<bevy::input::gamepad::ButtonSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::ButtonSettings::set_press_threshold(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the button input threshold above which the button is considered pressed.\n If the value passed is outside the range [release threshold..=1.0], the value will not be changed.\n Returns the new value of the press threshold.",
                &["_self", "value"],
            )
            .register_documented(
                "set_release_threshold",
                |mut _self: Mut<bevy::input::gamepad::ButtonSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::ButtonSettings::set_release_threshold(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the button input threshold below which the button is considered released. If the\n value passed is outside the range [0.0..=press threshold], the value will not be changed.\n Returns the new value of the release threshold.",
                &["_self", "value"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::ButtonSettings,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_axis_settings_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::AxisSettings,
        >::new(world)
            .register_documented(
                "clamp",
                |_self: Ref<bevy::input::gamepad::AxisSettings>, raw_value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::clamp(
                                    &_self,
                                    raw_value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Clamps the `raw_value` according to the `AxisSettings`.",
                &["_self", "raw_value"],
            )
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: Val<bevy::input::gamepad::AxisSettings> = {
                        {
                            let output: Val<bevy::input::gamepad::AxisSettings> = <bevy::input::gamepad::AxisSettings as ::core::clone::Clone>::clone(
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
                "deadzone_lowerbound",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::deadzone_lowerbound(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the value above which inputs will be rounded up to 0.0.",
                &["_self"],
            )
            .register_documented(
                "deadzone_upperbound",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::deadzone_upperbound(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the value below which positive inputs will be rounded down to 0.0.",
                &["_self"],
            )
            .register_documented(
                "eq",
                |
                    _self: Ref<bevy::input::gamepad::AxisSettings>,
                    other: Ref<bevy::input::gamepad::AxisSettings>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::AxisSettings as ::core::cmp::PartialEq<
                                bevy::input::gamepad::AxisSettings,
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
                "livezone_lowerbound",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::livezone_lowerbound(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the value below which negative inputs will be rounded down to -1.0.",
                &["_self"],
            )
            .register_documented(
                "livezone_upperbound",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::livezone_upperbound(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the value above which inputs will be rounded up to 1.0.",
                &["_self"],
            )
            .register_documented(
                "set_deadzone_lowerbound",
                |mut _self: Mut<bevy::input::gamepad::AxisSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::set_deadzone_lowerbound(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the value above which inputs will be rounded up to 0.0.\n If the value passed is less than -1.0 or less than `livezone_lowerbound`,\n the value will not be changed.\n Returns the new value of `deadzone_lowerbound`.",
                &["_self", "value"],
            )
            .register_documented(
                "set_deadzone_upperbound",
                |mut _self: Mut<bevy::input::gamepad::AxisSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::set_deadzone_upperbound(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the value below which positive inputs will be rounded down to 0.0.\n If the value passed is negative or greater than `livezone_upperbound`,\n the value will not be changed.\n Returns the new value of `deadzone_upperbound`.",
                &["_self", "value"],
            )
            .register_documented(
                "set_livezone_lowerbound",
                |mut _self: Mut<bevy::input::gamepad::AxisSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::set_livezone_lowerbound(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the value below which negative inputs will be rounded down to -1.0.\n If the value passed is positive or greater than `deadzone_lowerbound`,\n the value will not be changed.\n Returns the new value of `livezone_lowerbound`.",
                &["_self", "value"],
            )
            .register_documented(
                "set_livezone_upperbound",
                |mut _self: Mut<bevy::input::gamepad::AxisSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::set_livezone_upperbound(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the value above which inputs will be rounded up to 1.0.\n If the value passed is negative or less than `deadzone_upperbound`,\n the value will not be changed.\n Returns the new value of `livezone_upperbound`.",
                &["_self", "value"],
            )
            .register_documented(
                "set_threshold",
                |mut _self: Mut<bevy::input::gamepad::AxisSettings>, value: f32| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::set_threshold(
                                    &mut _self,
                                    value,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Try to set the minimum value by which input must change before the changes will be applied.\n If the value passed is not within [0.0..=2.0], the value will not be changed.\n Returns the new value of threshold.",
                &["_self", "value"],
            )
            .register_documented(
                "threshold",
                |_self: Ref<bevy::input::gamepad::AxisSettings>| {
                    let output: f32 = {
                        {
                            let output: f32 = bevy::input::gamepad::AxisSettings::threshold(
                                    &_self,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Get the minimum value by which input must change before the change is registered.",
                &["_self"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::AxisSettings,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_button_axis_settings_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::ButtonAxisSettings,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::ButtonAxisSettings>| {
                    let output: Val<bevy::input::gamepad::ButtonAxisSettings> = {
                        {
                            let output: Val<bevy::input::gamepad::ButtonAxisSettings> = <bevy::input::gamepad::ButtonAxisSettings as ::core::clone::Clone>::clone(
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::ButtonAxisSettings,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_gamepad_rumble_intensity_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::gamepad::GamepadRumbleIntensity,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::gamepad::GamepadRumbleIntensity>| {
                    let output: Val<bevy::input::gamepad::GamepadRumbleIntensity> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadRumbleIntensity,
                            > = <bevy::input::gamepad::GamepadRumbleIntensity as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::gamepad::GamepadRumbleIntensity>,
                    other: Ref<bevy::input::gamepad::GamepadRumbleIntensity>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::gamepad::GamepadRumbleIntensity as ::core::cmp::PartialEq<
                                bevy::input::gamepad::GamepadRumbleIntensity,
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
                "strong_motor",
                |intensity: f32| {
                    let output: Val<bevy::input::gamepad::GamepadRumbleIntensity> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadRumbleIntensity,
                            > = bevy::input::gamepad::GamepadRumbleIntensity::strong_motor(
                                    intensity,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new rumble intensity with strong motor intensity set to the given value.\n Clamped within the `0.0` to `1.0` range.",
                &["intensity"],
            )
            .register_documented(
                "weak_motor",
                |intensity: f32| {
                    let output: Val<bevy::input::gamepad::GamepadRumbleIntensity> = {
                        {
                            let output: Val<
                                bevy::input::gamepad::GamepadRumbleIntensity,
                            > = bevy::input::gamepad::GamepadRumbleIntensity::weak_motor(
                                    intensity,
                                )
                                .into();
                            output
                        }
                    };
                    output
                },
                " Creates a new rumble intensity with weak motor intensity set to the given value.\n Clamped within the `0.0` to `1.0` range.",
                &["intensity"],
            );
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::gamepad::GamepadRumbleIntensity,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_key_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::Key,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::Key>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::Key as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::Key>| {
                    let output: Val<bevy::input::keyboard::Key> = {
                        {
                            let output: Val<bevy::input::keyboard::Key> = <bevy::input::keyboard::Key as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::Key>,
                    other: Ref<bevy::input::keyboard::Key>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::Key as ::core::cmp::PartialEq<
                                bevy::input::keyboard::Key,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::Key,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_native_key_code_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::NativeKeyCode,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::NativeKeyCode>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::NativeKeyCode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::NativeKeyCode>| {
                    let output: Val<bevy::input::keyboard::NativeKeyCode> = {
                        {
                            let output: Val<bevy::input::keyboard::NativeKeyCode> = <bevy::input::keyboard::NativeKeyCode as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::NativeKeyCode>,
                    other: Ref<bevy::input::keyboard::NativeKeyCode>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::NativeKeyCode as ::core::cmp::PartialEq<
                                bevy::input::keyboard::NativeKeyCode,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::NativeKeyCode,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_native_key_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::keyboard::NativeKey,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::keyboard::NativeKey>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::keyboard::NativeKey as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::keyboard::NativeKey>| {
                    let output: Val<bevy::input::keyboard::NativeKey> = {
                        {
                            let output: Val<bevy::input::keyboard::NativeKey> = <bevy::input::keyboard::NativeKey as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::keyboard::NativeKey>,
                    other: Ref<bevy::input::keyboard::NativeKey>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::keyboard::NativeKey as ::core::cmp::PartialEq<
                                bevy::input::keyboard::NativeKey,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::keyboard::NativeKey,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_mouse_scroll_unit_functions(
        world: &mut bevy::ecs::world::World,
    ) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::mouse::MouseScrollUnit,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::mouse::MouseScrollUnit>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::mouse::MouseScrollUnit as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::mouse::MouseScrollUnit>| {
                    let output: Val<bevy::input::mouse::MouseScrollUnit> = {
                        {
                            let output: Val<bevy::input::mouse::MouseScrollUnit> = <bevy::input::mouse::MouseScrollUnit as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::mouse::MouseScrollUnit>,
                    other: Ref<bevy::input::mouse::MouseScrollUnit>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::mouse::MouseScrollUnit as ::core::cmp::PartialEq<
                                bevy::input::mouse::MouseScrollUnit,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::mouse::MouseScrollUnit,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_touch_phase_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::touch::TouchPhase,
        >::new(world)
            .register_documented(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::input::touch::TouchPhase>| {
                    let output: () = {
                        {
                            let output: () = <bevy::input::touch::TouchPhase as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
                |_self: Ref<bevy::input::touch::TouchPhase>| {
                    let output: Val<bevy::input::touch::TouchPhase> = {
                        {
                            let output: Val<bevy::input::touch::TouchPhase> = <bevy::input::touch::TouchPhase as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::touch::TouchPhase>,
                    other: Ref<bevy::input::touch::TouchPhase>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::touch::TouchPhase as ::core::cmp::PartialEq<
                                bevy::input::touch::TouchPhase,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::touch::TouchPhase,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    pub(crate) fn register_force_touch_functions(world: &mut bevy::ecs::world::World) {
        bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
            bevy::input::touch::ForceTouch,
        >::new(world)
            .register_documented(
                "clone",
                |_self: Ref<bevy::input::touch::ForceTouch>| {
                    let output: Val<bevy::input::touch::ForceTouch> = {
                        {
                            let output: Val<bevy::input::touch::ForceTouch> = <bevy::input::touch::ForceTouch as ::core::clone::Clone>::clone(
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
                    _self: Ref<bevy::input::touch::ForceTouch>,
                    other: Ref<bevy::input::touch::ForceTouch>|
                {
                    let output: bool = {
                        {
                            let output: bool = <bevy::input::touch::ForceTouch as ::core::cmp::PartialEq<
                                bevy::input::touch::ForceTouch,
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
        let registry = world
            .get_resource_or_init::<bevy::ecs::reflect::AppTypeRegistry>();
        let mut registry = registry.write();
        registry
            .register_type_data::<
                bevy::input::touch::ForceTouch,
                bevy_mod_scripting_core::bindings::MarkAsGenerated,
            >();
    }
    impl ::bevy::app::Plugin for BevyInputScriptingPlugin {
        fn build(&self, app: &mut ::bevy::prelude::App) {
            let mut world = app.world_mut();
            register_gamepad_functions(&mut world);
            register_gamepad_axis_functions(&mut world);
            register_gamepad_button_functions(&mut world);
            register_gamepad_settings_functions(&mut world);
            register_key_code_functions(&mut world);
            register_mouse_button_functions(&mut world);
            register_touch_input_functions(&mut world);
            register_keyboard_focus_lost_functions(&mut world);
            register_keyboard_input_functions(&mut world);
            register_accumulated_mouse_motion_functions(&mut world);
            register_accumulated_mouse_scroll_functions(&mut world);
            register_mouse_button_input_functions(&mut world);
            register_mouse_motion_functions(&mut world);
            register_mouse_wheel_functions(&mut world);
            register_gamepad_axis_changed_event_functions(&mut world);
            register_gamepad_button_changed_event_functions(&mut world);
            register_gamepad_button_state_changed_event_functions(&mut world);
            register_gamepad_connection_functions(&mut world);
            register_gamepad_connection_event_functions(&mut world);
            register_gamepad_event_functions(&mut world);
            register_gamepad_input_functions(&mut world);
            register_gamepad_rumble_request_functions(&mut world);
            register_raw_gamepad_axis_changed_event_functions(&mut world);
            register_raw_gamepad_button_changed_event_functions(&mut world);
            register_raw_gamepad_event_functions(&mut world);
            register_pinch_gesture_functions(&mut world);
            register_rotation_gesture_functions(&mut world);
            register_double_tap_gesture_functions(&mut world);
            register_pan_gesture_functions(&mut world);
            register_button_state_functions(&mut world);
            register_button_settings_functions(&mut world);
            register_axis_settings_functions(&mut world);
            register_button_axis_settings_functions(&mut world);
            register_gamepad_rumble_intensity_functions(&mut world);
            register_key_functions(&mut world);
            register_native_key_code_functions(&mut world);
            register_native_key_functions(&mut world);
            register_mouse_scroll_unit_functions(&mut world);
            register_touch_phase_functions(&mut world);
            register_force_touch_functions(&mut world);
        }
    }