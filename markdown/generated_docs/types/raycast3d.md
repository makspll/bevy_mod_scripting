# RayCast3d

### RayCast3d

- **origin** : glam::Vec3A
- **direction** : bevy\_math::direction::Dir3A
- **max** : f32
- **direction\_recip** : glam::Vec3A

## Description

>  A raycast intersection test for 3D bounding volumes

## Functions

| Function | Summary |
| --- | --- |
| `aabb_intersection_at(_self, aabb)` | [ Get the distance of an intersection with an \[\`Aabb3d\`\], if any\.](./raycast3d/aabb_intersection_at.md) |
| `clone(_self)` | [No Documentation 🚧](./raycast3d/clone.md) |
| `direction_recip(_self)` | [ Get the cached multiplicative inverse of the direction of the ray\.](./raycast3d/direction_recip.md) |
| `from_ray(ray, max)` | [ Construct a \[\`RayCast3d\`\] from a \[\`Ray3d\`\] and max distance\.](./raycast3d/from_ray.md) |
| `sphere_intersection_at(_self, sphere)` | [ Get the distance of an intersection with a \[\`BoundingSphere\`\], if any\.](./raycast3d/sphere_intersection_at.md) |