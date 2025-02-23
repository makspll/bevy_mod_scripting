# Dir3

### Dir3

1. glam::Vec3

## Description

>  A normalized vector pointing in a direction in 3D space

## Functions

| Function | Summary |
| --- | --- |
| `as_vec3(_self)` | [ Returns the inner \[\`Vec3\`\]](./dir3/as_vec3.md) |
| `clone(_self)` | [No Documentation 🚧](./dir3/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./dir3/eq.md) |
| `fast_renormalize(_self)` | [ Returns \`self\` after an approximate normalization, assuming the value is already nearly normalized\.](./dir3/fast_renormalize.md) |
| `from_xyz_unchecked(x, y, z)` | [ Create a direction from its \`x\`, \`y\`, and \`z\` components, assuming the resulting vector is normaliz](./dir3/from_xyz_unchecked.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./dir3/mul.md) |
| `neg(_self)` | [No Documentation 🚧](./dir3/neg.md) |
| `new_unchecked(value)` | [ Create a \[\`Dir3\`\] from a \[\`Vec3\`\] that is already normalized\.  \# Warning  \`value\` must be normalize](./dir3/new_unchecked.md) |
| `slerp(_self, rhs, s)` | [ Performs a spherical linear interpolation between \`self\` and \`rhs\`  based on the value \`s\`\.  This c](./dir3/slerp.md) |