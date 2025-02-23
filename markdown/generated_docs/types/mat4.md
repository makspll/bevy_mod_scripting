# Mat4

### Mat4

- **x\_axis** : glam::Vec4
- **y\_axis** : glam::Vec4
- **z\_axis** : glam::Vec4
- **w\_axis** : glam::Vec4

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Takes the absolute value of each element in \`self\`](./mat4/abs.md) |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./mat4/abs_diff_eq.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./mat4/add.md) |
| `add_mat4(_self, rhs)` | [ Adds two 4x4 matrices\.](./mat4/add_mat4.md) |
| `as_dmat4(_self)` | [No Documentation 🚧](./mat4/as_dmat4.md) |
| `clone(_self)` | [No Documentation 🚧](./mat4/clone.md) |
| `col(_self, index)` | [ Returns the matrix column for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 3\.](./mat4/col.md) |
| `determinant(_self)` | [ Returns the determinant of \`self\`\.](./mat4/determinant.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./mat4/div.md) |
| `div_scalar(_self, rhs)` | [ Divides a 4x4 matrix by a scalar\.](./mat4/div_scalar.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./mat4/eq.md) |
| `from_axis_angle(axis, angle)` | [ Creates an affine transformation matrix containing a 3D rotation around a normalized  rotation \`axis\`](./mat4/from_axis_angle.md) |
| `from_cols(x_axis, y_axis, z_axis, w_axis)` | [ Creates a 4x4 matrix from four column vectors\.](./mat4/from_cols.md) |
| `from_diagonal(diagonal)` | [ Creates a 4x4 matrix with its diagonal set to \`diagonal\` and all other entries set to 0\.](./mat4/from_diagonal.md) |
| `from_euler(order, a, b, c)` | [ Creates a affine transformation matrix containing a rotation from the given euler  rotation sequenc](./mat4/from_euler.md) |
| `from_mat3(m)` | [ Creates an affine transformation matrix from the given 3x3 linear transformation  matrix\.  The resu](./mat4/from_mat3.md) |
| `from_mat3a(m)` | [ Creates an affine transformation matrix from the given 3x3 linear transformation  matrix\.  The resu](./mat4/from_mat3a.md) |
| `from_quat(rotation)` | [ Creates an affine transformation matrix from the given \`rotation\` quaternion\.  The resulting matrix](./mat4/from_quat.md) |
| `from_rotation_translation(rotation, translation)` | [ Creates an affine transformation matrix from the given 3D \`translation\`\.  The resulting matrix can ](./mat4/from_rotation_translation.md) |
| `from_rotation_x(angle)` | [ Creates an affine transformation matrix containing a 3D rotation around the x axis of  \`angle\` \(in ](./mat4/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates an affine transformation matrix containing a 3D rotation around the y axis of  \`angle\` \(in ](./mat4/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates an affine transformation matrix containing a 3D rotation around the z axis of  \`angle\` \(in ](./mat4/from_rotation_z.md) |
| `from_scale(scale)` | [ Creates an affine transformation matrix containing the given 3D non\-uniform \`scale\`\.  The resulting](./mat4/from_scale.md) |
| `from_scale_rotation_translation(scale, rotation, translation)` | [ Creates an affine transformation matrix from the given 3D \`scale\`, \`rotation\` and  \`translation\`\.  ](./mat4/from_scale_rotation_translation.md) |
| `from_translation(translation)` | [ Creates an affine transformation matrix from the given 3D \`translation\`\.  The resulting matrix can ](./mat4/from_translation.md) |
| `inverse(_self)` | [ Returns the inverse of \`self\`\.  If the matrix is not invertible the returned matrix will be invalid](./mat4/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./mat4/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./mat4/is_nan.md) |
| `look_at_lh(eye, center, up)` | [ Creates a left\-handed view matrix using a camera position, an up direction, and a focal  point\.  Fo](./mat4/look_at_lh.md) |
| `look_at_rh(eye, center, up)` | [ Creates a right\-handed view matrix using a camera position, an up direction, and a focal  point\.  F](./mat4/look_at_rh.md) |
| `look_to_lh(eye, dir, up)` | [ Creates a left\-handed view matrix using a camera position, an up direction, and a facing  direction](./mat4/look_to_lh.md) |
| `look_to_rh(eye, dir, up)` | [ Creates a right\-handed view matrix using a camera position, an up direction, and a facing  directio](./mat4/look_to_rh.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./mat4/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./mat4/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./mat4/mul-2.md) |
| `mul-3(arg0, arg1)` | [No Documentation 🚧](./mat4/mul-3.md) |
| `mul_mat4(_self, rhs)` | [ Multiplies two 4x4 matrices\.](./mat4/mul_mat4.md) |
| `mul_scalar(_self, rhs)` | [ Multiplies a 4x4 matrix by a scalar\.](./mat4/mul_scalar.md) |
| `mul_vec4(_self, rhs)` | [ Transforms a 4D vector\.](./mat4/mul_vec4.md) |
| `neg(_self)` | [No Documentation 🚧](./mat4/neg.md) |
| `orthographic_lh(left, right, bottom, top, near, far)` | [ Creates a left\-handed orthographic projection matrix with \`\[0,1\]\` depth range\.  Useful to map a left\-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect\.](./mat4/orthographic_lh.md) |
| `orthographic_rh(left, right, bottom, top, near, far)` | [ Creates a right\-handed orthographic projection matrix with \`\[0,1\]\` depth range\.  Useful to map a right\-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect\.](./mat4/orthographic_rh.md) |
| `orthographic_rh_gl(left, right, bottom, top, near, far)` | [ Creates a right\-handed orthographic projection matrix with \`\[\-1,1\]\` depth  range\.  This is the same as the OpenGL \`glOrtho\` function in OpenGL\.  See  <https://www\.khronos\.org/registry/OpenGL\-Refpages/gl2\.1/xhtml/glOrtho\.xml>  Useful to map a right\-handed coordinate system to the normalized device coordinates that OpenGL expects\.](./mat4/orthographic_rh_gl.md) |
| `perspective_infinite_lh(fov_y_radians, aspect_ratio, z_near)` | [ Creates an infinite left\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Like \`perspective\_lh\`, but with an infinite value for \`z\_far\`\.  The result is that points near \`z\_near\` are mapped to depth \`0\`, and as they move towards infinity the depth approaches \`1\`\.  \# Panics  Will panic if \`z\_near\` or \`z\_far\` are less than or equal to zero when \`glam\_assert\` is  enabled\.](./mat4/perspective_infinite_lh.md) |
| `perspective_infinite_reverse_lh(fov_y_radians, aspect_ratio, z_near)` | [ Creates an infinite reverse left\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Similar to \`perspective\_infinite\_lh\`, but maps \`Z = z\_near\` to a depth of \`1\` and \`Z = infinity\` to a depth of \`0\`\.  \# Panics  Will panic if \`z\_near\` is less than or equal to zero when \`glam\_assert\` is enabled\.](./mat4/perspective_infinite_reverse_lh.md) |
| `perspective_infinite_reverse_rh(fov_y_radians, aspect_ratio, z_near)` | [ Creates an infinite reverse right\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Similar to \`perspective\_infinite\_rh\`, but maps \`Z = z\_near\` to a depth of \`1\` and \`Z = infinity\` to a depth of \`0\`\.  \# Panics  Will panic if \`z\_near\` is less than or equal to zero when \`glam\_assert\` is enabled\.](./mat4/perspective_infinite_reverse_rh.md) |
| `perspective_infinite_rh(fov_y_radians, aspect_ratio, z_near)` | [ Creates an infinite right\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Like \`perspective\_rh\`, but with an infinite value for \`z\_far\`\.  The result is that points near \`z\_near\` are mapped to depth \`0\`, and as they move towards infinity the depth approaches \`1\`\.  \# Panics  Will panic if \`z\_near\` or \`z\_far\` are less than or equal to zero when \`glam\_assert\` is  enabled\.](./mat4/perspective_infinite_rh.md) |
| `perspective_lh(fov_y_radians, aspect_ratio, z_near, z_far)` | [ Creates a left\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Useful to map the standard left\-handed coordinate system into what WebGPU/Metal/Direct3D expect\.  \# Panics  Will panic if \`z\_near\` or \`z\_far\` are less than or equal to zero when \`glam\_assert\` is  enabled\.](./mat4/perspective_lh.md) |
| `perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)` | [ Creates a right\-handed perspective projection matrix with \`\[0,1\]\` depth range\.  Useful to map the standard right\-handed coordinate system into what WebGPU/Metal/Direct3D expect\.  \# Panics  Will panic if \`z\_near\` or \`z\_far\` are less than or equal to zero when \`glam\_assert\` is  enabled\.](./mat4/perspective_rh.md) |
| `perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far)` | [ Creates a right\-handed perspective projection matrix with \`\[\-1,1\]\` depth range\.  Useful to map the standard right\-handed coordinate system into what OpenGL expects\.  This is the same as the OpenGL \`gluPerspective\` function\.  See <https://www\.khronos\.org/registry/OpenGL\-Refpages/gl2\.1/xhtml/gluPerspective\.xml>](./mat4/perspective_rh_gl.md) |
| `project_point3(_self, rhs)` | [ Transforms the given 3D vector as a point, applying perspective correction\.  This is the equivalent](./mat4/project_point3.md) |
| `project_point3a(_self, rhs)` | [ Transforms the given \[\`Vec3A\`\] as a 3D point, applying perspective correction\.  This is the equivalent of multiplying the \[\`Vec3A\`\]](./mat4/project_point3a.md) |
| `row(_self, index)` | [ Returns the matrix row for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 3\.](./mat4/row.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./mat4/sub.md) |
| `sub_mat4(_self, rhs)` | [ Subtracts two 4x4 matrices\.](./mat4/sub_mat4.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f32; 16\]\` array storing data in column major order\.  If you require data in row major order \`transpose\` the matrix first\.](./mat4/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f32; 4\]; 4\]\` 4D array storing data in column major order\.  If you require data in row ](./mat4/to_cols_array_2d.md) |
| `to_euler(_self, order)` | [ Extract Euler angles with the given Euler rotation order\.  Note if the upper 3x3 matrix contain sca](./mat4/to_euler.md) |
| `transform_point3(_self, rhs)` | [ Transforms the given 3D vector as a point\.  This is the equivalent of multiplying the 3D vector as ](./mat4/transform_point3.md) |
| `transform_point3a(_self, rhs)` | [ Transforms the given \[\`Vec3A\`\] as 3D point\.  This is the equivalent of multiplying the \[\`Vec3A\`\] as](./mat4/transform_point3a.md) |
| `transform_vector3(_self, rhs)` | [ Transforms the give 3D vector as a direction\.  This is the equivalent of multiplying the 3D vector ](./mat4/transform_vector3.md) |
| `transform_vector3a(_self, rhs)` | [ Transforms the give \[\`Vec3A\`\] as 3D vector\.  This is the equivalent of multiplying the \[\`Vec3A\`\] as](./mat4/transform_vector3a.md) |
| `transpose(_self)` | [ Returns the transpose of \`self\`\.](./mat4/transpose.md) |