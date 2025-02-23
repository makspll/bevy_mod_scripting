# Cuboid

### Cuboid

- **half\_size** : glam::Vec3

## Description

>  A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not
>  required to be the same.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./cuboid/clone.md) |
| `closest_point(_self, point)` | [ Finds the point on the cuboid that is closest to the given \`point\`\.  If the point is outside the cu](./cuboid/closest_point.md) |
| `eq(_self, other)` | [No Documentation 🚧](./cuboid/eq.md) |
| `from_corners(point1, point2)` | [ Create a new \`Cuboid\` from two corner points](./cuboid/from_corners.md) |
| `from_length(length)` | [ Create a \`Cuboid\` from a single length\.  The resulting \`Cuboid\` will be the same size in every dire](./cuboid/from_length.md) |
| `from_size(size)` | [ Create a new \`Cuboid\` from a given full size](./cuboid/from_size.md) |
| `new(x_length, y_length, z_length)` | [ Create a new \`Cuboid\` from a full x, y, and z length](./cuboid/new.md) |
| `size(_self)` | [ Get the size of the cuboid](./cuboid/size.md) |