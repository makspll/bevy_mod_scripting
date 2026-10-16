//! Buttons: the keyboard, mouse and gamepad button resources, read by static functions and held by methods.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_input::{ButtonInput, gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::{
        from::{M, R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;

use crate::with_resource_or;

/// Methods of a [`ButtonInput`] resource, for when a script holds the resource.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! button_input_methods {
    ($ty:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            core
        )]
        impl ButtonInput<$ty> {
            /// Whether the button is held down.
            fn pressed(_self: R<ButtonInput<$ty>>, button: R<$ty>) -> bool {
                _self.pressed(*button)
            }

            /// Whether the button was pressed during the last update.
            fn just_pressed(_self: R<ButtonInput<$ty>>, button: R<$ty>) -> bool {
                _self.just_pressed(*button)
            }

            /// Whether the button was released during the last update.
            fn just_released(_self: R<ButtonInput<$ty>>, button: R<$ty>) -> bool {
                _self.just_released(*button)
            }

            /// The buttons which are held down.
            fn get_pressed(_self: R<ButtonInput<$ty>>) -> Vec<V<$ty>> {
                _self.get_pressed().cloned().map(V).collect()
            }

            /// Holds the button down, as if it was pressed.
            fn press(mut _self: M<ButtonInput<$ty>>, button: R<$ty>) {
                _self.press(*button);
            }

            /// Lets go of the button, as if it was released.
            fn release(mut _self: M<ButtonInput<$ty>>, button: R<$ty>) {
                _self.release(*button);
            }
        }
    };
}

// The static functions are written out per type: with a generic `$ty` the attribute
// macro rejects the `FunctionCallContext` plus `R<$ty>` signature.

#[script_bindings(
    remote,
    name = "key_code_input_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl KeyCode {
    /// Whether the key is held down, false if the input resource is missing.
    fn pressed(ctxt: FunctionCallContext, key: R<KeyCode>) -> Result<bool, InteropError> {
        let key = *key;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<KeyCode>| {
            input.pressed(key)
        })
    }

    /// Whether the key was pressed during the last update.
    fn just_pressed(ctxt: FunctionCallContext, key: R<KeyCode>) -> Result<bool, InteropError> {
        let key = *key;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<KeyCode>| {
            input.just_pressed(key)
        })
    }

    /// Whether the key was released during the last update.
    fn just_released(ctxt: FunctionCallContext, key: R<KeyCode>) -> Result<bool, InteropError> {
        let key = *key;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<KeyCode>| {
            input.just_released(key)
        })
    }

    /// Whether any of the keys is held down.
    fn any_pressed(ctxt: FunctionCallContext, keys: Vec<V<KeyCode>>) -> Result<bool, InteropError> {
        let keys: Vec<KeyCode> = keys.into_iter().map(|key| key.0).collect();
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<KeyCode>| {
            input.any_pressed(keys)
        })
    }

    /// Whether any of the keys was pressed during the last update.
    fn any_just_pressed(
        ctxt: FunctionCallContext,
        keys: Vec<V<KeyCode>>,
    ) -> Result<bool, InteropError> {
        let keys: Vec<KeyCode> = keys.into_iter().map(|key| key.0).collect();
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<KeyCode>| {
            input.any_just_pressed(keys)
        })
    }

    /// The keys which are held down.
    fn get_pressed(ctxt: FunctionCallContext) -> Result<Vec<V<KeyCode>>, InteropError> {
        with_resource_or(
            &ctxt.world()?,
            Vec::new(),
            |input: &ButtonInput<KeyCode>| input.get_pressed().cloned().map(V).collect(),
        )
    }

    /// The keys which were pressed during the last update.
    fn get_just_pressed(ctxt: FunctionCallContext) -> Result<Vec<V<KeyCode>>, InteropError> {
        with_resource_or(
            &ctxt.world()?,
            Vec::new(),
            |input: &ButtonInput<KeyCode>| input.get_just_pressed().cloned().map(V).collect(),
        )
    }
}

#[script_bindings(
    remote,
    name = "mouse_button_input_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl MouseButton {
    /// Whether the mouse button is held down, false if the input resource is missing.
    fn pressed(ctxt: FunctionCallContext, button: R<MouseButton>) -> Result<bool, InteropError> {
        let button = *button;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<MouseButton>| {
            input.pressed(button)
        })
    }

    /// Whether the mouse button was pressed during the last update.
    fn just_pressed(
        ctxt: FunctionCallContext,
        button: R<MouseButton>,
    ) -> Result<bool, InteropError> {
        let button = *button;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<MouseButton>| {
            input.just_pressed(button)
        })
    }

    /// Whether the mouse button was released during the last update.
    fn just_released(
        ctxt: FunctionCallContext,
        button: R<MouseButton>,
    ) -> Result<bool, InteropError> {
        let button = *button;
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<MouseButton>| {
            input.just_released(button)
        })
    }

    /// Whether any of the mouse buttons is held down.
    fn any_pressed(
        ctxt: FunctionCallContext,
        buttons: Vec<V<MouseButton>>,
    ) -> Result<bool, InteropError> {
        let buttons: Vec<MouseButton> = buttons.into_iter().map(|button| button.0).collect();
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<MouseButton>| {
            input.any_pressed(buttons)
        })
    }

    /// Whether any of the mouse buttons was pressed during the last update.
    fn any_just_pressed(
        ctxt: FunctionCallContext,
        buttons: Vec<V<MouseButton>>,
    ) -> Result<bool, InteropError> {
        let buttons: Vec<MouseButton> = buttons.into_iter().map(|button| button.0).collect();
        with_resource_or(&ctxt.world()?, false, |input: &ButtonInput<MouseButton>| {
            input.any_just_pressed(buttons)
        })
    }

    /// The mouse buttons which are held down.
    fn get_pressed(ctxt: FunctionCallContext) -> Result<Vec<V<MouseButton>>, InteropError> {
        with_resource_or(
            &ctxt.world()?,
            Vec::new(),
            |input: &ButtonInput<MouseButton>| input.get_pressed().cloned().map(V).collect(),
        )
    }

    /// The mouse buttons which were pressed during the last update.
    fn get_just_pressed(ctxt: FunctionCallContext) -> Result<Vec<V<MouseButton>>, InteropError> {
        with_resource_or(
            &ctxt.world()?,
            Vec::new(),
            |input: &ButtonInput<MouseButton>| input.get_just_pressed().cloned().map(V).collect(),
        )
    }
}

button_input_methods!(KeyCode, "button_input_key_code_functions");
button_input_methods!(MouseButton, "button_input_mouse_button_functions");
button_input_methods!(GamepadButton, "button_input_gamepad_button_functions");

/// Registers the functions of buttons.
pub(crate) fn register(world: &mut World) {
    register_key_code_input_functions(world);
    register_mouse_button_input_functions(world);
    register_button_input_key_code_functions(world);
    register_button_input_mouse_button_functions(world);
    register_button_input_gamepad_button_functions(world);
}
