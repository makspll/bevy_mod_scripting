# Dir3A

### Dir3A

1. glam::Vec3A

## Description

>  A normalized SIMD vector pointing in a direction in 3D space.
> 
>  This type stores a 16 byte aligned [`Vec3A`].
>  This may or may not be faster than [`Dir3`]: make sure to benchmark!

## Functions

| Function | Summary |
| --- | --- |
| `as_vec3a(_self)` | [ Returns the inner \[\`Vec3A\`\]](./dir3a/as_vec3a.md) |
| `clone(_self)` | [No Documentation 🚧](./dir3a/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./dir3a/eq.md) |
| `fast_renormalize(_self)` | [ Returns \`self\` after an approximate normalization, assuming the value is already nearly normalized\.](./dir3a/fast_renormalize.md) |
| `from_xyz_unchecked(x, y, z)` | [ Create a direction from its \`x\`, \`y\`, and \`z\` components, assuming the resulting vector is normaliz](./dir3a/from_xyz_unchecked.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./dir3a/mul.md) |
| `neg(_self)` | [No Documentation 🚧](./dir3a/neg.md) |
| `new_unchecked(value)` | [ Create a \[\`Dir3A\`\] from a \[\`Vec3A\`\] that is already normalized\.  \# Warning  \`value\` must be normali](./dir3a/new_unchecked.md) |
| `slerp(_self, rhs, s)` | [ Performs a spherical linear interpolation between \`self\` and \`rhs\`  based on the value \`s\`\.  This c](./dir3a/slerp.md) |