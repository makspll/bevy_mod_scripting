# DAffine3

### DAffine3

- **matrix3** : glam::DMat3
- **translation** : glam::DVec3

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./daffine3/abs_diff_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./daffine3/clone.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./daffine3/eq.md) |
| `from_axis_angle(axis, angle)` | [ Creates an affine transform containing a 3D rotation around a normalized  rotation \`axis\` of \`angle](./daffine3/from_axis_angle.md) |
| `from_cols(x_axis, y_axis, z_axis, w_axis)` | [ Creates an affine transform from three column vectors\.](./daffine3/from_cols.md) |
| `from_mat3(mat3)` | [ Creates an affine transform from a 3x3 matrix \(expressing scale, shear and  rotation\)](./daffine3/from_mat3.md) |
| `from_mat3_translation(mat3, translation)` | [ Creates an affine transform from a 3x3 matrix \(expressing scale, shear and rotation\)  and a transla](./daffine3/from_mat3_translation.md) |
| `from_mat4(m)` | [ The given \`DMat4\` must be an affine transform,  i\.e\. contain no perspective transform\.](./daffine3/from_mat4.md) |
| `from_quat(rotation)` | [ Creates an affine transform from the given \`rotation\` quaternion\.](./daffine3/from_quat.md) |
| `from_rotation_translation(rotation, translation)` | [ Creates an affine transform from the given 3D \`rotation\` and \`translation\`\.  Equivalent to \`DAffine3::from\_translation\(translation\) \* DAffine3::from\_quat\(rotation\)\`](./daffine3/from_rotation_translation.md) |
| `from_rotation_x(angle)` | [ Creates an affine transform containing a 3D rotation around the x axis of  \`angle\` \(in radians\)\.](./daffine3/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates an affine transform containing a 3D rotation around the y axis of  \`angle\` \(in radians\)\.](./daffine3/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates an affine transform containing a 3D rotation around the z axis of  \`angle\` \(in radians\)\.](./daffine3/from_rotation_z.md) |
| `from_scale(scale)` | [ Creates an affine transform that changes scale\.  Note that if any scale is zero the transform will ](./daffine3/from_scale.md) |
| `from_scale_rotation_translation(scale, rotation, translation)` | [ Creates an affine transform from the given 3D \`scale\`, \`rotation\` and  \`translation\`\.  Equivalent t](./daffine3/from_scale_rotation_translation.md) |
| `from_translation(translation)` | [ Creates an affine transformation from the given 3D \`translation\`\.](./daffine3/from_translation.md) |
| `inverse(_self)` | [ Return the inverse of this transform\.  Note that if the transform is not invertible the result will](./daffine3/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./daffine3/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./daffine3/is_nan.md) |
| `look_at_lh(eye, center, up)` | [ Creates a left\-handed view transform using a camera position, an up direction, and a focal  point\. ](./daffine3/look_at_lh.md) |
| `look_at_rh(eye, center, up)` | [ Creates a right\-handed view transform using a camera position, an up direction, and a focal  point\.](./daffine3/look_at_rh.md) |
| `look_to_lh(eye, dir, up)` | [ Creates a left\-handed view transform using a camera position, an up direction, and a facing  direct](./daffine3/look_to_lh.md) |
| `look_to_rh(eye, dir, up)` | [ Creates a right\-handed view transform using a camera position, an up direction, and a facing  direc](./daffine3/look_to_rh.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./daffine3/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./daffine3/mul-1.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f64; 12\]\` array storing data in column major order\.](./daffine3/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f64; 3\]; 4\]\` 3D array storing data in  column major order\.  If you require data in row](./daffine3/to_cols_array_2d.md) |
| `transform_point3(_self, rhs)` | [ Transforms the given 3D points, applying shear, scale, rotation and translation\.](./daffine3/transform_point3.md) |
| `transform_vector3(_self, rhs)` | [ Transforms the given 3D vector, applying shear, scale and rotation \(but NOT  translation\)\.  To also](./daffine3/transform_vector3.md) |