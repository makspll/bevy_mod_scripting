# ButtonAxisSettings

### ButtonAxisSettings

- **high** : f32
- **low** : f32
- **threshold** : f32

## Description

>  Settings for a [`GamepadButton`].
> 
>  It is used inside the [`GamepadSettings`] to define the sensitivity range and
>  threshold for a button axis.
> 
>  ## Logic
> 
>  - Values that are higher than or equal to `high` will be rounded to 1.0.
>  - Values that are lower than or equal to `low` will be rounded to 0.0.
>  - Otherwise, values will not be rounded.
> 
>  The valid range is from 0.0 to 1.0, inclusive.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./buttonaxissettings/clone.md) |
| `filter(_self, new_value, old_value)` | [ Filters the \`new\_value\` based on the \`old\_value\`, according to the \[\`ButtonAxisSettings\`\]\.  Returns the clamped \`new\_value\`, according to the \[\`ButtonAxisSettings\`\]](./buttonaxissettings/filter.md) |