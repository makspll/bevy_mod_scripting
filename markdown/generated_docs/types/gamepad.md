# Gamepad

### Gamepad

- **vendor\_id** : core::option::Option<u16>
- **product\_id** : core::option::Option<u16>
- **digital** : bevy\_input::button\_input::ButtonInput<bevy\_input::gamepad::GamepadButton>
- **analog** : bevy\_input::axis::Axis<bevy\_input::gamepad::GamepadInput>

## Description

>  Stores a connected gamepad's metadata such as the name and its [`GamepadButton`] and [`GamepadAxis`].
> 
>  An entity with this component is spawned automatically after [`GamepadConnectionEvent`]
>  and updated by [`gamepad_event_processing_system`].
> 
>  See also [`GamepadSettings`] for configuration.
> 
>  # Examples
> 
>  ```
>  # use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
>  # use bevy_ecs::system::Query;
>  # use bevy_core::Name;
>  #
>  fn gamepad_usage_system(gamepads: Query<(&Name, &Gamepad)>) {
>      for (name, gamepad) in &gamepads {
>          println!("{name}");
> 
>          if gamepad.just_pressed(GamepadButton::North) {
>              println!("{} just pressed North", name)
>          }
> 
>          if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)  {
>              println!("left stick X: {}", left_stick_x)
>          }
>      }
>  }
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `dpad(_self)` | [ Returns the directional pad as a \[\`Vec2\`\]](./gamepad/dpad.md) |
| `just_pressed(_self, button_type)` | [ Returns \`true\` if the \[\`GamepadButton\`\] has been pressed during the current frame\.  Note: This function does not imply information regarding the current state of \[\`ButtonInput::pressed\`\]](./gamepad/just_pressed.md) |
| `just_released(_self, button_type)` | [ Returns \`true\` if the \[\`GamepadButton\`\] has been released during the current frame\.  Note: This function does not imply information regarding the current state of \[\`ButtonInput::pressed\`\]](./gamepad/just_released.md) |
| `left_stick(_self)` | [ Returns the left stick as a \[\`Vec2\`\]](./gamepad/left_stick.md) |
| `pressed(_self, button_type)` | [ Returns \`true\` if the \[\`GamepadButton\`\] has been pressed\.](./gamepad/pressed.md) |
| `product_id(_self)` | [ Returns the USB product ID as assigned by the \[vendor\], if available\.  \[vendor\]: Self::vendor\_id](./gamepad/product_id.md) |
| `right_stick(_self)` | [ Returns the right stick as a \[\`Vec2\`\]](./gamepad/right_stick.md) |
| `vendor_id(_self)` | [ Returns the USB vendor ID as assigned by the USB\-IF, if available\.](./gamepad/vendor_id.md) |