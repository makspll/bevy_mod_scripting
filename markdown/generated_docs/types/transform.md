# Transform

### Transform

- **translation** : glam::Vec3
- **rotation** : glam::Quat
- **scale** : glam::Vec3

## Description

>  Describe the position of an entity. If the entity has a parent, the position is relative
>  to its parent position.
> 
>  * To place or move an entity, you should set its [`Transform`].
>  * To get the global transform of an entity, you should get its [`GlobalTransform`].
>  * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
>    * ~You may use the [`TransformBundle`](crate::bundles::TransformBundle) to guarantee this.~
>      [`TransformBundle`](crate::bundles::TransformBundle) is now deprecated.
>      [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.
> 
>  ## [`Transform`] and [`GlobalTransform`]
> 
>  [`Transform`] is the position of an entity relative to its parent position, or the reference
>  frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
> 
>  [`GlobalTransform`] is the position of an entity relative to the reference frame.
> 
>  [`GlobalTransform`] is updated from [`Transform`] by systems in the system set
>  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
> 
>  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
>  update the [`Transform`] of an entity during this set or after, you will notice a 1 frame lag
>  before the [`GlobalTransform`] is updated.
> 
>  # Examples
> 
>  - [`transform`][transform_example]
> 
>  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs

## Functions

| Function | Summary |
| --- | --- |
| `back(_self)` | [ Equivalent to \[\`local\_z\(\)\`\]\[Transform::local\_z\]](./transform/back.md) |
| `clone(_self)` | [No Documentation 🚧](./transform/clone.md) |
| `compute_affine(_self)` | [ Returns the 3d affine transformation matrix from this transforms translation,  rotation, and scale\.](./transform/compute_affine.md) |
| `compute_matrix(_self)` | [ Returns the 3d affine transformation matrix from this transforms translation,  rotation, and scale\.](./transform/compute_matrix.md) |
| `down(_self)` | [ Equivalent to \[\`\-local\_y\(\)\`\]\[Transform::local\_y\]](./transform/down.md) |
| `eq(_self, other)` | [No Documentation 🚧](./transform/eq.md) |
| `forward(_self)` | [ Equivalent to \[\`\-local\_z\(\)\`\]\[Transform::local\_z\]](./transform/forward.md) |
| `from_isometry(iso)` | [ Creates a new \[\`Transform\`\] that is equivalent to the given \[isometry\]\.  \[isometry\]: Isometry3d](./transform/from_isometry.md) |
| `from_matrix(world_from_local)` | [ Extracts the translation, rotation, and scale from \`matrix\`\. It must be a 3d affine  transformation](./transform/from_matrix.md) |
| `from_rotation(rotation)` | [ Creates a new \[\`Transform\`\], with \`rotation\`\. Translation will be 0 and scale 1 on  all axes\.](./transform/from_rotation.md) |
| `from_scale(scale)` | [ Creates a new \[\`Transform\`\], with \`scale\`\. Translation will be 0 and rotation 0 on  all axes\.](./transform/from_scale.md) |
| `from_translation(translation)` | [ Creates a new \[\`Transform\`\], with \`translation\`\. Rotation will be 0 and scale 1 on  all axes\.](./transform/from_translation.md) |
| `from_xyz(x, y, z)` | [ Creates a new \[\`Transform\`\] at the position \`\(x, y, z\)\`\. In 2d, the \`z\` component  is used for z\-ordering elements: higher \`z\`\-value will be in front of lower  \`z\`\-value\.](./transform/from_xyz.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, translation, rotation and scale all are  finite\. If any of them con](./transform/is_finite.md) |
| `left(_self)` | [ Equivalent to \[\`\-local\_x\(\)\`\]\[Transform::local\_x\(\)\]](./transform/left.md) |
| `local_x(_self)` | [ Get the unit vector in the local \`X\` direction\.](./transform/local_x.md) |
| `local_y(_self)` | [ Get the unit vector in the local \`Y\` direction\.](./transform/local_y.md) |
| `local_z(_self)` | [ Get the unit vector in the local \`Z\` direction\.](./transform/local_z.md) |
| `mul(_self, value)` | [No Documentation 🚧](./transform/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./transform/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./transform/mul-2.md) |
| `mul_transform(_self, transform)` | [ Multiplies \`self\` with \`transform\` component by component, returning the  resulting \[\`Transform\`\]](./transform/mul_transform.md) |
| `right(_self)` | [ Equivalent to \[\`local\_x\(\)\`\]\[Transform::local\_x\(\)\]](./transform/right.md) |
| `rotate(_self, rotation)` | [ Rotates this \[\`Transform\`\] by the given rotation\.  If this \[\`Transform\`\] has a parent, the \`rotation\`](./transform/rotate.md) |
| `rotate_around(_self, point, rotation)` | [ Rotates this \[\`Transform\`\] around a \`point\` in space\.  If this \[\`Transform\`\] has a parent, the \`point\`](./transform/rotate_around.md) |
| `rotate_axis(_self, axis, angle)` | [ Rotates this \[\`Transform\`\] around the given \`axis\` by \`angle\` \(in radians\)\.  If this \[\`Transform\`\] ](./transform/rotate_axis.md) |
| `rotate_local(_self, rotation)` | [ Rotates this \[\`Transform\`\] by the given \`rotation\`\.  The \`rotation\` is relative to this \[\`Transform](./transform/rotate_local.md) |
| `rotate_local_axis(_self, axis, angle)` | [ Rotates this \[\`Transform\`\] around its local \`axis\` by \`angle\` \(in radians\)\.](./transform/rotate_local_axis.md) |
| `rotate_local_x(_self, angle)` | [ Rotates this \[\`Transform\`\] around its local \`X\` axis by \`angle\` \(in radians\)\.](./transform/rotate_local_x.md) |
| `rotate_local_y(_self, angle)` | [ Rotates this \[\`Transform\`\] around its local \`Y\` axis by \`angle\` \(in radians\)\.](./transform/rotate_local_y.md) |
| `rotate_local_z(_self, angle)` | [ Rotates this \[\`Transform\`\] around its local \`Z\` axis by \`angle\` \(in radians\)\.](./transform/rotate_local_z.md) |
| `rotate_x(_self, angle)` | [ Rotates this \[\`Transform\`\] around the \`X\` axis by \`angle\` \(in radians\)\.  If this \[\`Transform\`\] has ](./transform/rotate_x.md) |
| `rotate_y(_self, angle)` | [ Rotates this \[\`Transform\`\] around the \`Y\` axis by \`angle\` \(in radians\)\.  If this \[\`Transform\`\] has ](./transform/rotate_y.md) |
| `rotate_z(_self, angle)` | [ Rotates this \[\`Transform\`\] around the \`Z\` axis by \`angle\` \(in radians\)\.  If this \[\`Transform\`\] has ](./transform/rotate_z.md) |
| `to_isometry(_self)` | [ Get the \[isometry\] defined by this transform's rotation and translation, ignoring scale\.  \[isometry](./transform/to_isometry.md) |
| `transform_point(_self, point)` | [ Transforms the given \`point\`, applying scale, rotation and translation\.  If this \[\`Transform\`\] has an ancestor entity with a \[\`Transform\`\]](./transform/transform_point.md) |
| `translate_around(_self, point, rotation)` | [ Translates this \[\`Transform\`\] around a \`point\` in space\.  If this \[\`Transform\`\] has a parent, the \`point\`](./transform/translate_around.md) |
| `up(_self)` | [ Equivalent to \[\`local\_y\(\)\`\]\[Transform::local\_y\]](./transform/up.md) |
| `with_rotation(_self, rotation)` | [ Returns this \[\`Transform\`\] with a new rotation\.](./transform/with_rotation.md) |
| `with_scale(_self, scale)` | [ Returns this \[\`Transform\`\] with a new scale\.](./transform/with_scale.md) |
| `with_translation(_self, translation)` | [ Returns this \[\`Transform\`\] with a new translation\.](./transform/with_translation.md) |