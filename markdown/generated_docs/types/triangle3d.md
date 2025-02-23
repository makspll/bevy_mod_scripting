# Triangle3d

### Triangle3d

- **vertices** : \[glam::Vec3; 3\]

## Description

>  A 3D triangle primitive.

## Functions

| Function | Summary |
| --- | --- |
| `centroid(_self)` | [ Get the centroid of the triangle\.  This function finds the geometric center of the triangle by aver](./triangle3d/centroid.md) |
| `circumcenter(_self)` | [ Get the circumcenter of the triangle\.](./triangle3d/circumcenter.md) |
| `clone(_self)` | [No Documentation 🚧](./triangle3d/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./triangle3d/eq.md) |
| `is_acute(_self)` | [ Checks if the triangle is acute, meaning all angles are less than 90 degrees](./triangle3d/is_acute.md) |
| `is_degenerate(_self)` | [ Checks if the triangle is degenerate, meaning it has zero area\.  A triangle is degenerate if the cr](./triangle3d/is_degenerate.md) |
| `is_obtuse(_self)` | [ Checks if the triangle is obtuse, meaning one angle is greater than 90 degrees](./triangle3d/is_obtuse.md) |
| `new(a, b, c)` | [ Create a new \[\`Triangle3d\`\] from points \`a\`, \`b\`, and \`c\`\.](./triangle3d/new.md) |
| `reverse(_self)` | [ Reverse the triangle by swapping the first and last vertices\.](./triangle3d/reverse.md) |
| `reversed(_self)` | [ This triangle but reversed\.](./triangle3d/reversed.md) |