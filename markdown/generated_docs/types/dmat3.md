# DMat3

### DMat3

- **x\_axis** : glam::DVec3
- **y\_axis** : glam::DVec3
- **z\_axis** : glam::DVec3

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Takes the absolute value of each element in \`self\`](./dmat3/abs.md) |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./dmat3/abs_diff_eq.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./dmat3/add.md) |
| `add_mat3(_self, rhs)` | [ Adds two 3x3 matrices\.](./dmat3/add_mat3.md) |
| `as_mat3(_self)` | [No Documentation 🚧](./dmat3/as_mat3.md) |
| `clone(_self)` | [No Documentation 🚧](./dmat3/clone.md) |
| `col(_self, index)` | [ Returns the matrix column for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 2\.](./dmat3/col.md) |
| `determinant(_self)` | [ Returns the determinant of \`self\`\.](./dmat3/determinant.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./dmat3/div.md) |
| `div_scalar(_self, rhs)` | [ Divides a 3x3 matrix by a scalar\.](./dmat3/div_scalar.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./dmat3/eq.md) |
| `from_angle(angle)` | [ Creates an affine transformation matrix from the given 2D rotation \`angle\` \(in  radians\)\.  The resu](./dmat3/from_angle.md) |
| `from_axis_angle(axis, angle)` | [ Creates a 3D rotation matrix from a normalized rotation \`axis\` and \`angle\` \(in  radians\)\.  \# Panics](./dmat3/from_axis_angle.md) |
| `from_cols(x_axis, y_axis, z_axis)` | [ Creates a 3x3 matrix from three column vectors\.](./dmat3/from_cols.md) |
| `from_diagonal(diagonal)` | [ Creates a 3x3 matrix with its diagonal set to \`diagonal\` and all other entries set to 0\.](./dmat3/from_diagonal.md) |
| `from_euler(order, a, b, c)` | [ Creates a 3D rotation matrix from the given euler rotation sequence and the angles \(in  radians\)\.](./dmat3/from_euler.md) |
| `from_mat2(m)` | [ Creates an affine transformation matrix from the given 2x2 matrix\.  The resulting matrix can be use](./dmat3/from_mat2.md) |
| `from_mat4(m)` | [ Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column\.](./dmat3/from_mat4.md) |
| `from_mat4_minor(m, i, j)` | [ Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the \`i\`th column  and \`j\`th](./dmat3/from_mat4_minor.md) |
| `from_quat(rotation)` | [ Creates a 3D rotation matrix from the given quaternion\.  \# Panics  Will panic if \`rotation\` is not ](./dmat3/from_quat.md) |
| `from_rotation_x(angle)` | [ Creates a 3D rotation matrix from \`angle\` \(in radians\) around the x axis\.](./dmat3/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates a 3D rotation matrix from \`angle\` \(in radians\) around the y axis\.](./dmat3/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates a 3D rotation matrix from \`angle\` \(in radians\) around the z axis\.](./dmat3/from_rotation_z.md) |
| `from_scale(scale)` | [ Creates an affine transformation matrix from the given non\-uniform 2D \`scale\`\.  The resulting matri](./dmat3/from_scale.md) |
| `from_scale_angle_translation(scale, angle, translation)` | [ Creates an affine transformation matrix from the given 2D \`scale\`, rotation \`angle\` \(in  radians\) a](./dmat3/from_scale_angle_translation.md) |
| `from_translation(translation)` | [ Creates an affine transformation matrix from the given 2D \`translation\`\.  The resulting matrix can ](./dmat3/from_translation.md) |
| `inverse(_self)` | [ Returns the inverse of \`self\`\.  If the matrix is not invertible the returned matrix will be invalid](./dmat3/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./dmat3/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./dmat3/is_nan.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./dmat3/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./dmat3/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./dmat3/mul-2.md) |
| `mul-3(arg0, arg1)` | [No Documentation 🚧](./dmat3/mul-3.md) |
| `mul_mat3(_self, rhs)` | [ Multiplies two 3x3 matrices\.](./dmat3/mul_mat3.md) |
| `mul_scalar(_self, rhs)` | [ Multiplies a 3x3 matrix by a scalar\.](./dmat3/mul_scalar.md) |
| `mul_vec3(_self, rhs)` | [ Transforms a 3D vector\.](./dmat3/mul_vec3.md) |
| `neg(_self)` | [No Documentation 🚧](./dmat3/neg.md) |
| `row(_self, index)` | [ Returns the matrix row for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 2\.](./dmat3/row.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./dmat3/sub.md) |
| `sub_mat3(_self, rhs)` | [ Subtracts two 3x3 matrices\.](./dmat3/sub_mat3.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f64; 9\]\` array storing data in column major order\.  If you require data in row major order \`transpose\` the matrix first\.](./dmat3/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f64; 3\]; 3\]\` 3D array storing data in column major order\.  If you require data in row ](./dmat3/to_cols_array_2d.md) |
| `to_euler(_self, order)` | [ Extract Euler angles with the given Euler rotation order\.  Note if the input matrix contains scales](./dmat3/to_euler.md) |
| `transform_point2(_self, rhs)` | [ Transforms the given 2D vector as a point\.  This is the equivalent of multiplying \`rhs\` as a 3D vec](./dmat3/transform_point2.md) |
| `transform_vector2(_self, rhs)` | [ Rotates the given 2D vector\.  This is the equivalent of multiplying \`rhs\` as a 3D vector where \`z\` ](./dmat3/transform_vector2.md) |
| `transpose(_self)` | [ Returns the transpose of \`self\`\.](./dmat3/transpose.md) |