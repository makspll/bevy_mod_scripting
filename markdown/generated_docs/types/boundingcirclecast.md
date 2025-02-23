# BoundingCircleCast

### BoundingCircleCast

- **ray** : bevy\_math::bounding::raycast2d::RayCast2d
- **circle** : bevy\_math::bounding::bounded2d::BoundingCircle

## Description

>  An intersection test that casts a [`BoundingCircle`] along a ray.

## Functions

| Function | Summary |
| --- | --- |
| `circle_collision_at(_self, circle)` | [ Get the distance at which the \[\`BoundingCircle\`\]s collide, if at all\.](./boundingcirclecast/circle_collision_at.md) |
| `clone(_self)` | [No Documentation 🚧](./boundingcirclecast/clone.md) |
| `from_ray(circle, ray, max)` | [ Construct a \[\`BoundingCircleCast\`\] from a \[\`BoundingCircle\`\], \[\`Ray2d\`\], and max distance\.](./boundingcirclecast/from_ray.md) |
| `new(circle, origin, direction, max)` | [ Construct a \[\`BoundingCircleCast\`\] from a \[\`BoundingCircle\`\], origin, \[\`Dir2\`\], and max distance\.](./boundingcirclecast/new.md) |