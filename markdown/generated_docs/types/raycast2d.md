# RayCast2d

### RayCast2d

- **ray** : bevy\_math::ray::Ray2d
- **max** : f32
- **direction\_recip** : glam::Vec2

## Description

>  A raycast intersection test for 2D bounding volumes

## Functions

| Function | Summary |
| --- | --- |
| `aabb_intersection_at(_self, aabb)` | [ Get the distance of an intersection with an \[\`Aabb2d\`\], if any\.](./raycast2d/aabb_intersection_at.md) |
| `circle_intersection_at(_self, circle)` | [ Get the distance of an intersection with a \[\`BoundingCircle\`\], if any\.](./raycast2d/circle_intersection_at.md) |
| `clone(_self)` | [No Documentation 🚧](./raycast2d/clone.md) |
| `direction_recip(_self)` | [ Get the cached multiplicative inverse of the direction of the ray\.](./raycast2d/direction_recip.md) |
| `from_ray(ray, max)` | [ Construct a \[\`RayCast2d\`\] from a \[\`Ray2d\`\] and max distance\.](./raycast2d/from_ray.md) |
| `new(origin, direction, max)` | [ Construct a \[\`RayCast2d\`\] from an origin, \[\`Dir2\`\], and max distance\.](./raycast2d/new.md) |