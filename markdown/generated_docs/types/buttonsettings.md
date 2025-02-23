# ButtonSettings

### ButtonSettings

- **press\_threshold** : f32
- **release\_threshold** : f32

## Description

>  Manages settings for gamepad buttons.
> 
>  It is used inside [`GamepadSettings`] to define the threshold for a [`GamepadButton`]
>  to be considered pressed or released. A button is considered pressed if the `press_threshold`
>  value is surpassed and released if the `release_threshold` value is undercut.
> 
>  Allowed values: `0.0 <= ``release_threshold`` <= ``press_threshold`` <= 1.0`

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./buttonsettings/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./buttonsettings/eq.md) |
| `is_pressed(_self, value)` | [ Returns \`true\` if the button is pressed\.  A button is considered pressed if the \`value\` passed is g](./buttonsettings/is_pressed.md) |
| `is_released(_self, value)` | [ Returns \`true\` if the button is released\.  A button is considered released if the \`value\` passed is](./buttonsettings/is_released.md) |
| `press_threshold(_self)` | [ Get the button input threshold above which the button is considered pressed\.](./buttonsettings/press_threshold.md) |
| `release_threshold(_self)` | [ Get the button input threshold below which the button is considered released\.](./buttonsettings/release_threshold.md) |
| `set_press_threshold(_self, value)` | [ Try to set the button input threshold above which the button is considered pressed\.  If the value p](./buttonsettings/set_press_threshold.md) |
| `set_release_threshold(_self, value)` | [ Try to set the button input threshold below which the button is considered released\. If the  value ](./buttonsettings/set_release_threshold.md) |