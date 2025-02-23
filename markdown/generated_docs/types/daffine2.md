# DAffine2

### DAffine2

- **matrix2** : glam::DMat2
- **translation** : glam::DVec2

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./daffine2/abs_diff_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./daffine2/clone.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./daffine2/eq.md) |
| `from_angle(angle)` | [ Creates an affine transform from the given rotation \`angle\`\.](./daffine2/from_angle.md) |
| `from_angle_translation(angle, translation)` | [ Creates an affine transform from the given 2D rotation \`angle\` \(in radians\) and  \`translation\`\.  Eq](./daffine2/from_angle_translation.md) |
| `from_cols(x_axis, y_axis, z_axis)` | [ Creates an affine transform from three column vectors\.](./daffine2/from_cols.md) |
| `from_mat2(matrix2)` | [ Creates an affine transform from a 2x2 matrix \(expressing scale, shear and rotation\)](./daffine2/from_mat2.md) |
| `from_mat2_translation(matrix2, translation)` | [ Creates an affine transform from a 2x2 matrix \(expressing scale, shear and rotation\) and a  transla](./daffine2/from_mat2_translation.md) |
| `from_mat3(m)` | [ The given \`DMat3\` must be an affine transform,](./daffine2/from_mat3.md) |
| `from_scale(scale)` | [ Creates an affine transform that changes scale\.  Note that if any scale is zero the transform will ](./daffine2/from_scale.md) |
| `from_scale_angle_translation(scale, angle, translation)` | [ Creates an affine transform from the given 2D \`scale\`, rotation \`angle\` \(in radians\) and  \`translation\`](./daffine2/from_scale_angle_translation.md) |
| `from_translation(translation)` | [ Creates an affine transformation from the given 2D \`translation\`\.](./daffine2/from_translation.md) |
| `inverse(_self)` | [ Return the inverse of this transform\.  Note that if the transform is not invertible the result will](./daffine2/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./daffine2/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./daffine2/is_nan.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./daffine2/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./daffine2/mul-1.md) |
| `to_cols_array(_self)` | [ Creates a \`\[f64; 6\]\` array storing data in column major order\.](./daffine2/to_cols_array.md) |
| `to_cols_array_2d(_self)` | [ Creates a \`\[\[f64; 2\]; 3\]\` 2D array storing data in  column major order\.  If you require data in row](./daffine2/to_cols_array_2d.md) |
| `transform_point2(_self, rhs)` | [ Transforms the given 2D point, applying shear, scale, rotation and translation\.](./daffine2/transform_point2.md) |
| `transform_vector2(_self, rhs)` | [ Transforms the given 2D vector, applying shear, scale and rotation \(but NOT  translation\)\.  To also](./daffine2/transform_vector2.md) |