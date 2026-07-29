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
pub struct BevyUiWidgetsScriptingPlugin;
pub(crate) fn register_activate_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::Activate,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::Activate>| {
                let output: V<::bevy_ui_widgets::Activate> = {
                    {
                        let output: V<::bevy_ui_widgets::Activate> = <::bevy_ui_widgets::Activate as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_widgets::Activate>,
                other: R<::bevy_ui_widgets::Activate>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_widgets::Activate as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::Activate,
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
            ::bevy_ui_widgets::Activate,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_button_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::Button,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::Button>| {
                let output: V<::bevy_ui_widgets::Button> = {
                    {
                        let output: V<::bevy_ui_widgets::Button> = <::bevy_ui_widgets::Button as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::Button,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_activate_on_press_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ActivateOnPress,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::ActivateOnPress>| {
            let output: V<::bevy_ui_widgets::ActivateOnPress> = {
                {
                    let output: V<::bevy_ui_widgets::ActivateOnPress> =
                        <::bevy_ui_widgets::ActivateOnPress as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::ActivateOnPress,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_checkbox_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::Checkbox,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::Checkbox>| {
                let output: V<::bevy_ui_widgets::Checkbox> = {
                    {
                        let output: V<::bevy_ui_widgets::Checkbox> = <::bevy_ui_widgets::Checkbox as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::Checkbox,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_set_checked_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SetChecked,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::SetChecked,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_toggle_checked_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ToggleChecked,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::ToggleChecked,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_active_descendant_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ActiveDescendant,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::ActiveDescendant>| {
            let output: V<::bevy_ui_widgets::ActiveDescendant> = {
                {
                    let output: V<::bevy_ui_widgets::ActiveDescendant> =
                        <::bevy_ui_widgets::ActiveDescendant as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::ActiveDescendant,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_list_item_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ListItem,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::ListItem>| {
                let output: V<::bevy_ui_widgets::ListItem> = {
                    {
                        let output: V<::bevy_ui_widgets::ListItem> = <::bevy_ui_widgets::ListItem as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::ListItem,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_action_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuAction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuAction>| {
            let output: V<::bevy_ui_widgets::MenuAction> = {
                {
                    let output: V<::bevy_ui_widgets::MenuAction> =
                        <::bevy_ui_widgets::MenuAction as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::MenuAction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_event_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuEvent,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuEvent>| {
            let output: V<::bevy_ui_widgets::MenuEvent> = {
                {
                    let output: V<::bevy_ui_widgets::MenuEvent> =
                        <::bevy_ui_widgets::MenuEvent as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_ui_widgets::MenuEvent,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuLayout,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuLayout>| {
            let output: V<::bevy_ui_widgets::MenuLayout> = {
                {
                    let output: V<::bevy_ui_widgets::MenuLayout> =
                        <::bevy_ui_widgets::MenuLayout as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::MenuLayout>, other: R<::bevy_ui_widgets::MenuLayout>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui_widgets::MenuLayout as ::std::cmp::PartialEq<
                        ::bevy_ui_widgets::MenuLayout,
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
            ::bevy_ui_widgets::MenuLayout,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_focus_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuFocusState,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuFocusState>| {
            let output: V<::bevy_ui_widgets::MenuFocusState> = {
                {
                    let output: V<::bevy_ui_widgets::MenuFocusState> =
                        <::bevy_ui_widgets::MenuFocusState as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::MenuFocusState>,
         other: R<::bevy_ui_widgets::MenuFocusState>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui_widgets::MenuFocusState as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::MenuFocusState,
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
            ::bevy_ui_widgets::MenuFocusState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_popup_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuPopup,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuPopup>| {
            let output: V<::bevy_ui_widgets::MenuPopup> = {
                {
                    let output: V<::bevy_ui_widgets::MenuPopup> =
                        <::bevy_ui_widgets::MenuPopup as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_ui_widgets::MenuPopup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_item_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuItem,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::MenuItem>| {
                let output: V<::bevy_ui_widgets::MenuItem> = {
                    {
                        let output: V<::bevy_ui_widgets::MenuItem> = <::bevy_ui_widgets::MenuItem as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::MenuItem,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_menu_button_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::MenuButton,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::MenuButton>| {
            let output: V<::bevy_ui_widgets::MenuButton> = {
                {
                    let output: V<::bevy_ui_widgets::MenuButton> =
                        <::bevy_ui_widgets::MenuButton as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::MenuButton,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_popover_side_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::popover::PopoverSide,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::popover::PopoverSide>| {
            let output: V<::bevy_ui_widgets::popover::PopoverSide> = {
                {
                    let output: V<::bevy_ui_widgets::popover::PopoverSide> =
                        <::bevy_ui_widgets::popover::PopoverSide as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_ui_widgets::popover::PopoverSide>,
         other: R<::bevy_ui_widgets::popover::PopoverSide>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui_widgets::popover::PopoverSide as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::popover::PopoverSide,
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
        "mirror",
        |_self: R<::bevy_ui_widgets::popover::PopoverSide>| {
            let output: V<::bevy_ui_widgets::popover::PopoverSide> = {
                {
                    let output: V<::bevy_ui_widgets::popover::PopoverSide> =
                        ::bevy_ui_widgets::popover::PopoverSide::mirror(&_self).into();
                    output
                }
            };
            output
        },
        " Returns the side that is the mirror image of this side.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::popover::PopoverSide,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_popover_align_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::popover::PopoverAlign,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::popover::PopoverAlign>| {
            let output: V<::bevy_ui_widgets::popover::PopoverAlign> = {
                {
                    let output: V<::bevy_ui_widgets::popover::PopoverAlign> =
                        <::bevy_ui_widgets::popover::PopoverAlign as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_ui_widgets::popover::PopoverAlign>,
         other: R<::bevy_ui_widgets::popover::PopoverAlign>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui_widgets::popover::PopoverAlign as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::popover::PopoverAlign,
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
            ::bevy_ui_widgets::popover::PopoverAlign,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_popover_placement_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::popover::PopoverPlacement,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::popover::PopoverPlacement>| {
                let output: V<::bevy_ui_widgets::popover::PopoverPlacement> = {
                    {
                        let output: V<::bevy_ui_widgets::popover::PopoverPlacement> = <::bevy_ui_widgets::popover::PopoverPlacement as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_widgets::popover::PopoverPlacement>,
                other: R<::bevy_ui_widgets::popover::PopoverPlacement>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_widgets::popover::PopoverPlacement as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::popover::PopoverPlacement,
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
            ::bevy_ui_widgets::popover::PopoverPlacement,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_popover_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::popover::Popover,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::popover::Popover>| {
            let output: V<::bevy_ui_widgets::popover::Popover> = {
                {
                    let output: V<::bevy_ui_widgets::popover::Popover> =
                        <::bevy_ui_widgets::popover::Popover as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::popover::Popover>,
         other: R<::bevy_ui_widgets::popover::Popover>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui_widgets::popover::Popover as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::popover::Popover,
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
            ::bevy_ui_widgets::popover::Popover,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radio_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::RadioGroup,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::RadioGroup>| {
            let output: V<::bevy_ui_widgets::RadioGroup> = {
                {
                    let output: V<::bevy_ui_widgets::RadioGroup> =
                        <::bevy_ui_widgets::RadioGroup as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::RadioGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radio_button_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::RadioButton,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::RadioButton>| {
            let output: V<::bevy_ui_widgets::RadioButton> = {
                {
                    let output: V<::bevy_ui_widgets::RadioButton> =
                        <::bevy_ui_widgets::RadioButton as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::RadioButton,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scroll_area_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ScrollArea,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::ScrollArea>| {
            let output: V<::bevy_ui_widgets::ScrollArea> = {
                {
                    let output: V<::bevy_ui_widgets::ScrollArea> =
                        <::bevy_ui_widgets::ScrollArea as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::ScrollArea,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_control_orientation_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ControlOrientation,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::ControlOrientation>| {
            let output: V<::bevy_ui_widgets::ControlOrientation> = {
                {
                    let output: V<::bevy_ui_widgets::ControlOrientation> =
                        <::bevy_ui_widgets::ControlOrientation as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_ui_widgets::ControlOrientation>,
         other: R<::bevy_ui_widgets::ControlOrientation>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui_widgets::ControlOrientation as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::ControlOrientation,
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
            ::bevy_ui_widgets::ControlOrientation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scrollbar_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::Scrollbar,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::Scrollbar>| {
                let output: V<::bevy_ui_widgets::Scrollbar> = {
                    {
                        let output: V<::bevy_ui_widgets::Scrollbar> = <::bevy_ui_widgets::Scrollbar as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_widgets::Scrollbar>,
                other: R<::bevy_ui_widgets::Scrollbar>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_widgets::Scrollbar as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::Scrollbar,
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
                target: V<::bevy_ecs::entity::Entity>,
                orientation: V<::bevy_ui_widgets::ControlOrientation>,
                min_thumb_length: f32|
            {
                let output: V<::bevy_ui_widgets::Scrollbar> = {
                    {
                        let output: V<::bevy_ui_widgets::Scrollbar> = ::bevy_ui_widgets::Scrollbar::new(
                                target.into_inner(),
                                orientation.into_inner(),
                                min_thumb_length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new scrollbar.\n # Arguments\n * `target` - The scrollable entity that this scrollbar will control.\n * `orientation` - The orientation of the scrollbar (horizontal or vertical).\n * `min_thumb_length` - The minimum size of the scrollbar's thumb, in pixels.",
            &["target", "orientation", "min_thumb_length"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::Scrollbar,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scrollbar_drag_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ScrollbarDragState,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::ScrollbarDragState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scrollbar_thumb_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::ScrollbarThumb,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::ScrollbarThumb>| {
            let output: V<::bevy_ui_widgets::ScrollbarThumb> = {
                {
                    let output: V<::bevy_ui_widgets::ScrollbarThumb> =
                        <::bevy_ui_widgets::ScrollbarThumb as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::ScrollbarThumb,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_orientation_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderOrientation,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::SliderOrientation>| {
                let output: V<::bevy_ui_widgets::SliderOrientation> = {
                    {
                        let output: V<::bevy_ui_widgets::SliderOrientation> = <::bevy_ui_widgets::SliderOrientation as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_widgets::SliderOrientation>,
                other: R<::bevy_ui_widgets::SliderOrientation>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_widgets::SliderOrientation as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::SliderOrientation,
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
            "is_vertical",
            |
                _self: V<::bevy_ui_widgets::SliderOrientation>,
                node: R<::bevy_ui::ComputedNode>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui_widgets::SliderOrientation::is_vertical(
                                _self.into_inner(),
                                &node,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolve the orientation to a boolean indicating whether the slider is vertical,\n using the node dimensions for auto-detection.",
            &["_self", "node"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::SliderOrientation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_track_click_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::TrackClick,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::TrackClick>| {
            let output: V<::bevy_ui_widgets::TrackClick> = {
                {
                    let output: V<::bevy_ui_widgets::TrackClick> =
                        <::bevy_ui_widgets::TrackClick as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::TrackClick>, other: R<::bevy_ui_widgets::TrackClick>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui_widgets::TrackClick as ::std::cmp::PartialEq<
                        ::bevy_ui_widgets::TrackClick,
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
            ::bevy_ui_widgets::TrackClick,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::Slider,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::Slider>| {
                let output: V<::bevy_ui_widgets::Slider> = {
                    {
                        let output: V<::bevy_ui_widgets::Slider> = <::bevy_ui_widgets::Slider as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::Slider,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_drag_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderDragState,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::SliderDragState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_value_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderValue,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SliderValue>| {
            let output: V<::bevy_ui_widgets::SliderValue> = {
                {
                    let output: V<::bevy_ui_widgets::SliderValue> =
                        <::bevy_ui_widgets::SliderValue as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::SliderValue>, other: R<::bevy_ui_widgets::SliderValue>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui_widgets::SliderValue as ::std::cmp::PartialEq<
                        ::bevy_ui_widgets::SliderValue,
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
            ::bevy_ui_widgets::SliderValue,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_range_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderRange,
    >::new(world)
        .register_documented(
            "center",
            |_self: R<::bevy_ui_widgets::SliderRange>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::center(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the center value of the range.",
            &["_self"],
        )
        .register_documented(
            "clamp",
            |_self: R<::bevy_ui_widgets::SliderRange>, value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::clamp(
                                &_self,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Constrain a value between the minimum and maximum allowed values for this slider.",
            &["_self", "value"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_widgets::SliderRange>| {
                let output: V<::bevy_ui_widgets::SliderRange> = {
                    {
                        let output: V<::bevy_ui_widgets::SliderRange> = <::bevy_ui_widgets::SliderRange as ::std::clone::Clone>::clone(
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
            "end",
            |_self: R<::bevy_ui_widgets::SliderRange>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::end(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the maximum allowed value for this slider.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: R<::bevy_ui_widgets::SliderRange>,
                other: R<::bevy_ui_widgets::SliderRange>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_widgets::SliderRange as ::std::cmp::PartialEq<
                            ::bevy_ui_widgets::SliderRange,
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
            |start: f32, end: f32| {
                let output: V<::bevy_ui_widgets::SliderRange> = {
                    {
                        let output: V<::bevy_ui_widgets::SliderRange> = ::bevy_ui_widgets::SliderRange::new(
                                start,
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new slider range with the given start and end values.",
            &["start", "end"],
        )
        .register_documented(
            "span",
            |_self: R<::bevy_ui_widgets::SliderRange>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::span(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the full span of the range (max - min).",
            &["_self"],
        )
        .register_documented(
            "start",
            |_self: R<::bevy_ui_widgets::SliderRange>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::start(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the minimum allowed value for this slider.",
            &["_self"],
        )
        .register_documented(
            "thumb_position",
            |_self: R<::bevy_ui_widgets::SliderRange>, value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui_widgets::SliderRange::thumb_position(
                                &_self,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the position of the thumb on the slider, as a value between 0 and 1, taking\n into account the proportion of the value between the minimum and maximum limits.",
            &["_self", "value"],
        )
        .register_documented(
            "with_end",
            |_self: R<::bevy_ui_widgets::SliderRange>, end: f32| {
                let output: V<::bevy_ui_widgets::SliderRange> = {
                    {
                        let output: V<::bevy_ui_widgets::SliderRange> = ::bevy_ui_widgets::SliderRange::with_end(
                                &_self,
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a new instance of a `SliderRange` with a new end position.",
            &["_self", "end"],
        )
        .register_documented(
            "with_start",
            |_self: R<::bevy_ui_widgets::SliderRange>, start: f32| {
                let output: V<::bevy_ui_widgets::SliderRange> = {
                    {
                        let output: V<::bevy_ui_widgets::SliderRange> = ::bevy_ui_widgets::SliderRange::with_start(
                                &_self,
                                start,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a new instance of a `SliderRange` with a new start position.",
            &["_self", "start"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui_widgets::SliderRange,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_step_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderStep,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SliderStep>| {
            let output: V<::bevy_ui_widgets::SliderStep> = {
                {
                    let output: V<::bevy_ui_widgets::SliderStep> =
                        <::bevy_ui_widgets::SliderStep as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_ui_widgets::SliderStep>, other: R<::bevy_ui_widgets::SliderStep>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui_widgets::SliderStep as ::std::cmp::PartialEq<
                        ::bevy_ui_widgets::SliderStep,
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
            ::bevy_ui_widgets::SliderStep,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_thumb_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderThumb,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SliderThumb>| {
            let output: V<::bevy_ui_widgets::SliderThumb> = {
                {
                    let output: V<::bevy_ui_widgets::SliderThumb> =
                        <::bevy_ui_widgets::SliderThumb as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::SliderThumb,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_precision_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderPrecision,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SliderPrecision>| {
            let output: V<::bevy_ui_widgets::SliderPrecision> = {
                {
                    let output: V<::bevy_ui_widgets::SliderPrecision> =
                        <::bevy_ui_widgets::SliderPrecision as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::SliderPrecision,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_set_slider_value_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SetSliderValue,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SetSliderValue>| {
            let output: V<::bevy_ui_widgets::SetSliderValue> = {
                {
                    let output: V<::bevy_ui_widgets::SetSliderValue> =
                        <::bevy_ui_widgets::SetSliderValue as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::SetSliderValue,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slider_value_change_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SliderValueChange,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SliderValueChange>| {
            let output: V<::bevy_ui_widgets::SliderValueChange> = {
                {
                    let output: V<::bevy_ui_widgets::SliderValueChange> =
                        <::bevy_ui_widgets::SliderValueChange as ::std::clone::Clone>::clone(
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
            ::bevy_ui_widgets::SliderValueChange,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_select_all_on_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_widgets::SelectAllOnFocus,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_ui_widgets::SelectAllOnFocus>| {
            let output: V<::bevy_ui_widgets::SelectAllOnFocus> = {
                {
                    let output: V<::bevy_ui_widgets::SelectAllOnFocus> =
                        <::bevy_ui_widgets::SelectAllOnFocus as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui_widgets::SelectAllOnFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyUiWidgetsScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_activate_functions(&mut world);
        register_button_functions(&mut world);
        register_activate_on_press_functions(&mut world);
        register_checkbox_functions(&mut world);
        register_set_checked_functions(&mut world);
        register_toggle_checked_functions(&mut world);
        register_active_descendant_functions(&mut world);
        register_list_item_functions(&mut world);
        register_menu_action_functions(&mut world);
        register_menu_event_functions(&mut world);
        register_menu_layout_functions(&mut world);
        register_menu_focus_state_functions(&mut world);
        register_menu_popup_functions(&mut world);
        register_menu_item_functions(&mut world);
        register_menu_button_functions(&mut world);
        register_popover_side_functions(&mut world);
        register_popover_align_functions(&mut world);
        register_popover_placement_functions(&mut world);
        register_popover_functions(&mut world);
        register_radio_group_functions(&mut world);
        register_radio_button_functions(&mut world);
        register_scroll_area_functions(&mut world);
        register_control_orientation_functions(&mut world);
        register_scrollbar_functions(&mut world);
        register_scrollbar_drag_state_functions(&mut world);
        register_scrollbar_thumb_functions(&mut world);
        register_slider_orientation_functions(&mut world);
        register_track_click_functions(&mut world);
        register_slider_functions(&mut world);
        register_slider_drag_state_functions(&mut world);
        register_slider_value_functions(&mut world);
        register_slider_range_functions(&mut world);
        register_slider_step_functions(&mut world);
        register_slider_thumb_functions(&mut world);
        register_slider_precision_functions(&mut world);
        register_set_slider_value_functions(&mut world);
        register_slider_value_change_functions(&mut world);
        register_select_all_on_focus_functions(&mut world);
    }
}
