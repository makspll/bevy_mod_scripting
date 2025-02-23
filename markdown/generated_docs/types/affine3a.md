# Affine3A

### Affine3A

- **matrix3** : glam::Mat3A
- **translation** : glam::Vec3A

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./affine3a/abs_diff_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./affine3a/clone.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./affine3a/eq.md) |
| `from_axis_angle(axis, angle)` | [ Creates an affine transform containing a 3D rotation around a normalized  rotation \`axis\` of \`angle](./affine3a/from_axis_angle.md) |
| `from_cols(x_axis, y_axis, z_axis, w_axis)` | [ Creates an affine transform from three column vectors\.](./affine3a/from_cols.md) |
| `from_mat3(mat3)` | [ Creates an affine transform from a 3x3 matrix \(expressing scale, shear and  rotation\)](./affine3a/from_mat3.md) |
| `from_mat3_translation(mat3, translation)` | [ Creates an affine transform from a 3x3 matrix \(expressing scale, shear and rotation\)  and a transla](./affine3a/from_mat3_translation.md) |
| `from_mat4(m)` | [ The given \`Mat4\` must be an affine transform,  i\.e\. contain no perspective transform\.](./affine3a/from_mat4.md) |
| `from_quat(rotation)` | [ Creates an affine transform from the given \`rotation\` quaternion\.](./affine3a/from_quat.md) |
| `from_rotation_translation(rotation, translation)` | [ Creates an affine transform from the given 3D \`rotation\` and \`translation\`\.  Equivalent to \`Affine3A::from\_translation\(translation\) \* Affine3A::from\_quat\(rotation\)\`](./affine3a/from_rotation_translation.md) |
| `from_rotation_x(angle)` | [ Creates an affine transform containing a 3D rotation around the x axis of  \`angle\` \(in radians\)\.](./affine3a/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates an affine transform containing a 3D rotation around the y axis of  \`angle\` \(in radians\)\.](./affine3a/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates an affine transform containing a 3D rotation around the z axis of  \`angle\` \(in radians\)\.](./affine3a/from_rotation_z.md) |
| `from_scale(scale)` | [ Creates an affine transform that changes scale\.  Note that if any scale is zero the transform will ](./affine3a/from_scale.md) |
| `from_scale_rotation_translation(scale, rotation, translation)` | [ Creates an affine transform from the given 3D \`scale\`, \`rotation\` and  \`translation\`\.  Equivalent t](./affine3a/from_scale_rotation_translation.md) |
| `from_translation(translation)` | [ Creates an affine transformation from the given 3D \`translation\`\.](./affine3a/from_translation.md) |
| `inverse(_self)` | [ Return the inverse of this transform\.  Note that if the transform is not invertible the result will](./affine3a/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./affine3a/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./affine3a/is_nan.md) |
| `look_at_lh(eye, center, up)` | [ Creates a left\-handed view transform using a camera position, an up direction, and a focal  point\. ](./affine3a/look_at_lh.md) |
| `look_at_rh(eye, center, up)` | [ Creates a right\-handed view transform using a camera position, an up direction, and a focal  point\.](./affine3a/look_at_rh.md) |
| `look_to_lh(eye, dir, up)` | [ Creates a left\-handed view transform using a camera position, an up direction, and a facing  direct](./affine3a/look_to_lh.md) |
| `look_to_rh(eye, dir, up)` | [ Creates a right\-handed view transform using a camera position, an up direction, and a facing  direc](./affine3a/look_to_rh.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./affine3a/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./affine3a/mul-1.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f32; 12\]\` array storing data in column major order\.](./affine3a/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f32; 3\]; 4\]\` 3D array storing data in  column major order\.  If you require data in row](./affine3a/to_cols_array_2d.md) |
| `transform_point3(_self, rhs)` | [ Transforms the given 3D points, applying shear, scale, rotation and translation\.](./affine3a/transform_point3.md) |
| `transform_point3a(_self, rhs)` | [ Transforms the given \[\`Vec3A\`\], applying shear, scale, rotation and translation\.](./affine3a/transform_point3a.md) |
| `transform_vector3(_self, rhs)` | [ Transforms the given 3D vector, applying shear, scale and rotation \(but NOT  translation\)\.  To also](./affine3a/transform_vector3.md) |
| `transform_vector3a(_self, rhs)` | [ Transforms the given \[\`Vec3A\`\], applying shear, scale and rotation \(but NOT  translation\)\.  To also apply translation, use \[\`Self::transform\_point3a\(\)\`\]](./affine3a/transform_vector3a.md) |