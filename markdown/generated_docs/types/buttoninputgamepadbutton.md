# ButtonInput<GamepadButton>

### ButtonInput

- **pressed** : bevy\_utils::hashbrown::HashSet<bevy\_input::gamepad::GamepadButton, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>
- **just\_pressed** : bevy\_utils::hashbrown::HashSet<bevy\_input::gamepad::GamepadButton, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>
- **just\_released** : bevy\_utils::hashbrown::HashSet<bevy\_input::gamepad::GamepadButton, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>

## Description

>  A "press-able" input of type `T`.
> 
>  ## Usage
> 
>  This type can be used as a resource to keep the current state of an input, by reacting to
>  events from the input. For a given input value:
> 
>  * [`ButtonInput::pressed`] will return `true` between a press and a release event.
>  * [`ButtonInput::just_pressed`] will return `true` for one frame after a press event.
>  * [`ButtonInput::just_released`] will return `true` for one frame after a release event.
> 
>  ## Multiple systems
> 
>  In case multiple systems are checking for [`ButtonInput::just_pressed`] or [`ButtonInput::just_released`]
>  but only one should react, for example when modifying a
>  [`Resource`], you should consider clearing the input state, either by:
> 
>  * Using [`ButtonInput::clear_just_pressed`] or [`ButtonInput::clear_just_released`] instead.
>  * Calling [`ButtonInput::clear`] or [`ButtonInput::reset`] immediately after the state change.
> 
>  ## Performance
> 
>  For all operations, the following conventions are used:
>  - **n** is the number of stored inputs.
>  - **m** is the number of input arguments passed to the method.
>  - **\***-suffix denotes an amortized cost.
>  - **~**-suffix denotes an expected cost.
> 
>  See Rust's [std::collections doc on performance](https://doc.rust-lang.org/std/collections/index.html#performance) for more details on the conventions used here.
> 
>  | **[`ButtonInput`] operations**          | **Computational complexity** |
>  |-----------------------------------|------------------------------------|
>  | [`ButtonInput::any_just_pressed`]       | *O*(m)~                      |
>  | [`ButtonInput::any_just_released`]      | *O*(m)~                      |
>  | [`ButtonInput::any_pressed`]            | *O*(m)~                      |
>  | [`ButtonInput::get_just_pressed`]       | *O*(n)                       |
>  | [`ButtonInput::get_just_released`]      | *O*(n)                       |
>  | [`ButtonInput::get_pressed`]            | *O*(n)                       |
>  | [`ButtonInput::just_pressed`]           | *O*(1)~                      |
>  | [`ButtonInput::just_released`]          | *O*(1)~                      |
>  | [`ButtonInput::pressed`]                | *O*(1)~                      |
>  | [`ButtonInput::press`]                  | *O*(1)~*                     |
>  | [`ButtonInput::release`]                | *O*(1)~*                     |
>  | [`ButtonInput::release_all`]            | *O*(n)~*                     |
>  | [`ButtonInput::clear_just_pressed`]     | *O*(1)~                      |
>  | [`ButtonInput::clear_just_released`]    | *O*(1)~                      |
>  | [`ButtonInput::reset_all`]              | *O*(n)                       |
>  | [`ButtonInput::clear`]                  | *O*(n)                       |
> 
>  ## Window focus
> 
>  `ButtonInput<KeyCode>` is tied to window focus. For example, if the user holds a button
>  while the window loses focus, [`ButtonInput::just_released`] will be triggered. Similarly if the window
>  regains focus, [`ButtonInput::just_pressed`] will be triggered.
> 
>  `ButtonInput<GamepadButton>` is independent of window focus.
> 
>  ## Examples
> 
>  Reading and checking against the current set of pressed buttons:
>  ```no_run
>  # use bevy_app::{App, NoopPluginGroup as DefaultPlugins, Update};
>  # use bevy_ecs::{prelude::{IntoSystemConfigs, Res, Resource, resource_changed}, schedule::Condition};
>  # use bevy_input::{ButtonInput, prelude::{KeyCode, MouseButton}};
> 
>  fn main() {
>      App::new()
>          .add_plugins(DefaultPlugins)
>          .add_systems(
>              Update,
>              print_mouse.run_if(resource_changed::<ButtonInput<MouseButton>>),
>          )
>          .add_systems(
>              Update,
>              print_keyboard.run_if(resource_changed::<ButtonInput<KeyCode>>),
>          )
>          .run();
>  }
> 
>  fn print_mouse(mouse: Res<ButtonInput<MouseButton>>) {
>      println!("Mouse: {:?}", mouse.get_pressed().collect::<Vec<_>>());
>  }
> 
>  fn print_keyboard(keyboard: Res<ButtonInput<KeyCode>>) {
>      if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
>          && keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
>          && keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
>          && keyboard.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight])
>          && keyboard.pressed(KeyCode::KeyL)
>      {
>          println!("On Windows this opens LinkedIn.");
>      } else {
>          println!("keyboard: {:?}", keyboard.get_pressed().collect::<Vec<_>>());
>      }
>  }
>  ```
> 
>  ## Note
> 
>  When adding this resource for a new input type, you should:
> 
>  * Call the [`ButtonInput::press`] method for each press event.
>  * Call the [`ButtonInput::release`] method for each release event.
>  * Call the [`ButtonInput::clear`] method at each frame start, before processing events.
> 
>  Note: Calling `clear` from a [`ResMut`] will trigger change detection.
>  It may be preferable to use [`DetectChangesMut::bypass_change_detection`]
>  to avoid causing the resource to always be marked as changed.
> 
>  [`ResMut`]: bevy_ecs::system::ResMut
>  [`DetectChangesMut::bypass_change_detection`]: bevy_ecs::change_detection::DetectChangesMut::bypass_change_detection

## Functions

