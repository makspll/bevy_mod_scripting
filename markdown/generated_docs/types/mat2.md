# Mat2

### Mat2

- **x\_axis** : glam::Vec2
- **y\_axis** : glam::Vec2

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Takes the absolute value of each element in \`self\`](./mat2/abs.md) |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./mat2/abs_diff_eq.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./mat2/add.md) |
| `add_mat2(_self, rhs)` | [ Adds two 2x2 matrices\.](./mat2/add_mat2.md) |
| `as_dmat2(_self)` | [No Documentation 🚧](./mat2/as_dmat2.md) |
| `clone(_self)` | [No Documentation 🚧](./mat2/clone.md) |
| `col(_self, index)` | [ Returns the matrix column for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 1\.](./mat2/col.md) |
| `determinant(_self)` | [ Returns the determinant of \`self\`\.](./mat2/determinant.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./mat2/div.md) |
| `div_scalar(_self, rhs)` | [ Divides a 2x2 matrix by a scalar\.](./mat2/div_scalar.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./mat2/eq.md) |
| `from_angle(angle)` | [ Creates a 2x2 matrix containing a rotation of \`angle\` \(in radians\)\.](./mat2/from_angle.md) |
| `from_cols(x_axis, y_axis)` | [ Creates a 2x2 matrix from two column vectors\.](./mat2/from_cols.md) |
| `from_diagonal(diagonal)` | [ Creates a 2x2 matrix with its diagonal set to \`diagonal\` and all other entries set to 0\.](./mat2/from_diagonal.md) |
| `from_mat3(m)` | [ Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column\.](./mat2/from_mat3.md) |
| `from_mat3_minor(m, i, j)` | [ Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the \`i\`th column  and \`j\`th](./mat2/from_mat3_minor.md) |
| `from_mat3a(m)` | [ Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column\.](./mat2/from_mat3a.md) |
| `from_mat3a_minor(m, i, j)` | [ Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the \`i\`th column  and \`j\`th](./mat2/from_mat3a_minor.md) |
| `from_scale_angle(scale, angle)` | [ Creates a 2x2 matrix containing the combining non\-uniform \`scale\` and rotation of  \`angle\` \(in radi](./mat2/from_scale_angle.md) |
| `inverse(_self)` | [ Returns the inverse of \`self\`\.  If the matrix is not invertible the returned matrix will be invalid](./mat2/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./mat2/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./mat2/is_nan.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./mat2/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./mat2/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./mat2/mul-2.md) |
| `mul_mat2(_self, rhs)` | [ Multiplies two 2x2 matrices\.](./mat2/mul_mat2.md) |
| `mul_scalar(_self, rhs)` | [ Multiplies a 2x2 matrix by a scalar\.](./mat2/mul_scalar.md) |
| `mul_vec2(_self, rhs)` | [ Transforms a 2D vector\.](./mat2/mul_vec2.md) |
| `neg(_self)` | [No Documentation 🚧](./mat2/neg.md) |
| `row(_self, index)` | [ Returns the matrix row for the given \`index\`\.  \# Panics  Panics if \`index\` is greater than 1\.](./mat2/row.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./mat2/sub.md) |
| `sub_mat2(_self, rhs)` | [ Subtracts two 2x2 matrices\.](./mat2/sub_mat2.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f32; 4\]\` array storing data in column major order\.  If you require data in row major order \`transpose\` the matrix first\.](./mat2/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f32; 2\]; 2\]\` 2D array storing data in column major order\.  If you require data in row ](./mat2/to_cols_array_2d.md) |
| `transpose(_self)` | [ Returns the transpose of \`self\`\.](./mat2/transpose.md) |