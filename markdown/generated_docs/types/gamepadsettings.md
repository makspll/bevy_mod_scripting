# GamepadSettings

### GamepadSettings

- **default\_button\_settings** : bevy\_input::gamepad::ButtonSettings
- **default\_axis\_settings** : bevy\_input::gamepad::AxisSettings
- **default\_button\_axis\_settings** : bevy\_input::gamepad::ButtonAxisSettings
- **button\_settings** : bevy\_utils::hashbrown::HashMap<bevy\_input::gamepad::GamepadButton, bevy\_input::gamepad::ButtonSettings, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>
- **axis\_settings** : bevy\_utils::hashbrown::HashMap<bevy\_input::gamepad::GamepadAxis, bevy\_input::gamepad::AxisSettings, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>
- **button\_axis\_settings** : bevy\_utils::hashbrown::HashMap<bevy\_input::gamepad::GamepadButton, bevy\_input::gamepad::ButtonAxisSettings, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>

## Description

>  Gamepad settings component.
> 
>  ## Usage
> 
>  It is used to create a `bevy` component that stores the settings of [`GamepadButton`] and [`GamepadAxis`] in [`Gamepad`].
>  If no user defined [`ButtonSettings`], [`AxisSettings`], or [`ButtonAxisSettings`]
>  are defined, the default settings of each are used as a fallback accordingly.
> 
>  ## Note
> 
>  The [`GamepadSettings`] are used to determine when raw gamepad events
>  should register. Events that don't meet the change thresholds defined in [`GamepadSettings`]
>  will not register. To modify these settings, mutate the corresponding component.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./gamepadsettings/clone.md) |