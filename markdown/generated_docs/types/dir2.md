# Dir2

### Dir2

1. glam::Vec2

## Description

>  A normalized vector pointing in a direction in 2D space

## Functions

| Function | Summary |
| --- | --- |
| `as_vec2(_self)` | [ Returns the inner \[\`Vec2\`\]](./dir2/as_vec2.md) |
| `clone(_self)` | [No Documentation 🚧](./dir2/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./dir2/eq.md) |
| `fast_renormalize(_self)` | [ Returns \`self\` after an approximate normalization, assuming the value is already nearly normalized\.](./dir2/fast_renormalize.md) |
| `from_xy_unchecked(x, y)` | [ Create a direction from its \`x\` and \`y\` components, assuming the resulting vector is normalized\.  \#](./dir2/from_xy_unchecked.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./dir2/mul.md) |
| `neg(_self)` | [No Documentation 🚧](./dir2/neg.md) |
| `new_unchecked(value)` | [ Create a \[\`Dir2\`\] from a \[\`Vec2\`\] that is already normalized\.  \# Warning  \`value\` must be normalize](./dir2/new_unchecked.md) |
| `rotation_from(_self, other)` | [ Get the rotation that rotates \`other\` to this direction\.](./dir2/rotation_from.md) |
| `rotation_from_x(_self)` | [ Get the rotation that rotates the X\-axis to this direction\.](./dir2/rotation_from_x.md) |
| `rotation_from_y(_self)` | [ Get the rotation that rotates the Y\-axis to this direction\.](./dir2/rotation_from_y.md) |
| `rotation_to(_self, other)` | [ Get the rotation that rotates this direction to \`other\`\.](./dir2/rotation_to.md) |
| `rotation_to_x(_self)` | [ Get the rotation that rotates this direction to the X\-axis\.](./dir2/rotation_to_x.md) |
| `rotation_to_y(_self)` | [ Get the rotation that rotates this direction to the Y\-axis\.](./dir2/rotation_to_y.md) |
| `slerp(_self, rhs, s)` | [ Performs a spherical linear interpolation between \`self\` and \`rhs\`  based on the value \`s\`\.  This c](./dir2/slerp.md) |