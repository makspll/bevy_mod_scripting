# KeyboardInput

### KeyboardInput

- **key\_code** : bevy\_input::keyboard::KeyCode
- **logical\_key** : bevy\_input::keyboard::Key
- **state** : bevy\_input::ButtonState
- **repeat** : bool
- **window** : bevy\_ecs::entity::Entity

## Description

>  A keyboard input event.
> 
>  This event is the translated version of the `WindowEvent::KeyboardInput` from the `winit` crate.
>  It is available to the end user and can be used for game logic.
> 
>  ## Usage
> 
>  The event is consumed inside of the [`keyboard_input_system`]
>  to update the [`ButtonInput<KeyCode>`](ButtonInput<KeyCode>) resource.

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./keyboardinput/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./keyboardinput/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./keyboardinput/eq.md) |