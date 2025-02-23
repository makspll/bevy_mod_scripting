# AxisSettings

### AxisSettings

- **livezone\_upperbound** : f32
- **deadzone\_upperbound** : f32
- **deadzone\_lowerbound** : f32
- **livezone\_lowerbound** : f32
- **threshold** : f32

## Description

>  Settings for a [`GamepadAxis`].
> 
>  It is used inside the [`GamepadSettings`] to define the sensitivity range and
>  threshold for an axis.
>  Values that are higher than `livezone_upperbound` will be rounded up to 1.0.
>  Values that are lower than `livezone_lowerbound` will be rounded down to -1.0.
>  Values that are in-between `deadzone_lowerbound` and `deadzone_upperbound` will be rounded
>  to 0.0.
>  Otherwise, values will not be rounded.
> 
>  The valid range is `[-1.0, 1.0]`.

## Functions

| Function | Summary |
| --- | --- |
| `clamp(_self, new_value)` | [ Clamps the \`raw\_value\` according to the \`AxisSettings\`\.](./axissettings/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./axissettings/clone.md) |
| `deadzone_lowerbound(_self)` | [ Get the value above which inputs will be rounded up to 0\.0\.](./axissettings/deadzone_lowerbound.md) |
| `deadzone_upperbound(_self)` | [ Get the value below which positive inputs will be rounded down to 0\.0\.](./axissettings/deadzone_upperbound.md) |
| `eq(_self, other)` | [No Documentation 🚧](./axissettings/eq.md) |
| `filter(_self, new_value, old_value)` | [ Filters the \`new\_value\` based on the \`old\_value\`, according to the \[\`AxisSettings\`\]\.  Returns the clamped \`new\_value\` if the change exceeds the settings threshold,  and \`None\` otherwise\.](./axissettings/filter.md) |
| `livezone_lowerbound(_self)` | [ Get the value below which negative inputs will be rounded down to \-1\.0\.](./axissettings/livezone_lowerbound.md) |
| `livezone_upperbound(_self)` | [ Get the value above which inputs will be rounded up to 1\.0\.](./axissettings/livezone_upperbound.md) |
| `set_deadzone_lowerbound(_self, value)` | [ Try to set the value above which inputs will be rounded up to 0\.0\.  If the value passed is less tha](./axissettings/set_deadzone_lowerbound.md) |
| `set_deadzone_upperbound(_self, value)` | [ Try to set the value below which positive inputs will be rounded down to 0\.0\.  If the value passed ](./axissettings/set_deadzone_upperbound.md) |
| `set_livezone_lowerbound(_self, value)` | [ Try to set the value below which negative inputs will be rounded down to \-1\.0\.  If the value passed](./axissettings/set_livezone_lowerbound.md) |
| `set_livezone_upperbound(_self, value)` | [ Try to set the value above which inputs will be rounded up to 1\.0\.  If the value passed is negative](./axissettings/set_livezone_upperbound.md) |
| `set_threshold(_self, value)` | [ Try to set the minimum value by which input must change before the changes will be applied\.  If the](./axissettings/set_threshold.md) |
| `threshold(_self)` | [ Get the minimum value by which input must change before the change is registered\.](./axissettings/threshold.md) |