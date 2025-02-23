# GlobalTransform

### GlobalTransform

1. glam::Affine3A

## Description

>  [`GlobalTransform`] is an affine transformation from entity-local coordinates to worldspace coordinates.
> 
>  You cannot directly mutate [`GlobalTransform`]; instead, you change an entity's transform by manipulating
>  its [`Transform`], which indirectly causes Bevy to update its [`GlobalTransform`].
> 
>  * To get the global transform of an entity, you should get its [`GlobalTransform`].
>  * For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].
>    * ~You may use the [`TransformBundle`](crate::bundles::TransformBundle) to guarantee this.~
>      [`TransformBundle`](crate::bundles::TransformBundle) is now deprecated.
>      [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.
> 
>  ## [`Transform`] and [`GlobalTransform`]
> 
>  [`Transform`] transforms an entity relative to its parent's reference frame, or relative to world space coordinates,
>  if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
> 
>  [`GlobalTransform`] is managed by Bevy; it is computed by successively applying the [`Transform`] of each ancestor
>  entity which has a Transform. This is done automatically by Bevy-internal systems in the system set
>  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
> 
>  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
>  update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag
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
| `affine(_self)` | [ Returns the 3d affine transformation matrix as an \[\`Affine3A\`\]\.](./globaltransform/affine.md) |
| `back(_self)` | [Return the local back vector \(Z\)\.](./globaltransform/back.md) |
| `clone(_self)` | [No Documentation 🚧](./globaltransform/clone.md) |
| `compute_matrix(_self)` | [ Returns the 3d affine transformation matrix as a \[\`Mat4\`\]\.](./globaltransform/compute_matrix.md) |
| `compute_transform(_self)` | [ Returns the transformation as a \[\`Transform\`\]\.  The transform is expected to be non\-degenerate and without shearing, or the output  will be invalid\.](./globaltransform/compute_transform.md) |
| `down(_self)` | [Return the local down vector \(\-Y\)\.](./globaltransform/down.md) |
| `eq(_self, other)` | [No Documentation 🚧](./globaltransform/eq.md) |
| `forward(_self)` | [Return the local forward vector \(\-Z\)\.](./globaltransform/forward.md) |
| `from_isometry(iso)` | [No Documentation 🚧](./globaltransform/from_isometry.md) |
| `from_rotation(rotation)` | [No Documentation 🚧](./globaltransform/from_rotation.md) |
| `from_scale(scale)` | [No Documentation 🚧](./globaltransform/from_scale.md) |
| `from_translation(translation)` | [No Documentation 🚧](./globaltransform/from_translation.md) |
| `from_xyz(x, y, z)` | [No Documentation 🚧](./globaltransform/from_xyz.md) |
| `left(_self)` | [Return the local left vector \(\-X\)\.](./globaltransform/left.md) |
| `mul(_self, value)` | [No Documentation 🚧](./globaltransform/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./globaltransform/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./globaltransform/mul-2.md) |
| `mul_transform(_self, transform)` | [ Multiplies \`self\` with \`transform\` component by component, returning the  resulting \[\`GlobalTransform\`\]](./globaltransform/mul_transform.md) |
| `radius_vec3a(_self, extents)` | [ Get an upper bound of the radius from the given \`extents\`\.](./globaltransform/radius_vec3a.md) |
| `reparented_to(_self, parent)` | [ Returns the \[\`Transform\`\] \`self\` would have if it was a child of an entity  with the \`parent\` \[\`GlobalTransform\`](./globaltransform/reparented_to.md) |
| `right(_self)` | [Return the local right vector \(X\)\.](./globaltransform/right.md) |
| `rotation(_self)` | [ Get the rotation as a \[\`Quat\`\]\.  The transform is expected to be non\-degenerate and without shearing, or the output will be invalid\.  \# Warning  This is calculated using \`to\_scale\_rotation\_translation\`, meaning that you  should probably use it directly if you also need translation or scale\.](./globaltransform/rotation.md) |
| `scale(_self)` | [ Get the scale as a \[\`Vec3\`\]\.  The transform is expected to be non\-degenerate and without shearing, or the output will be invalid\.  Some of the computations overlap with \`to\_scale\_rotation\_translation\`, which means you should use  it instead if you also need rotation\.](./globaltransform/scale.md) |
| `to_isometry(_self)` | [ Returns the isometric part of the transformation as an \[isometry\]\. Any scaling done by the  transformation will be ignored\.  The transform is expected to be non\-degenerate and without shearing, or the output  will be invalid\.  \[isometry\]](./globaltransform/to_isometry.md) |
| `transform_point(_self, point)` | [ Transforms the given point from local space to global space, applying shear, scale, rotation and tr](./globaltransform/transform_point.md) |
| `translation(_self)` | [ Get the translation as a \[\`Vec3\`\]\.](./globaltransform/translation.md) |
| `translation_vec3a(_self)` | [ Get the translation as a \[\`Vec3A\`\]\.](./globaltransform/translation_vec3a.md) |
| `up(_self)` | [Return the local up vector \(Y\)\.](./globaltransform/up.md) |