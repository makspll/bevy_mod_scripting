# Quat

### Quat

- **x** : f32
- **y** : f32
- **z** : f32
- **w** : f32

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./quat/abs_diff_eq.md) |
| `add(_self, rhs)` | [ Adds two quaternions\.  The sum is not guaranteed to be normalized\.  Note that addition is not the s](./quat/add.md) |
| `angle_between(_self, rhs)` | [ Returns the angle \(in radians\) for the minimal rotation  for transforming this quaternion into anot](./quat/angle_between.md) |
| `as_dquat(_self)` | [No Documentation 🚧](./quat/as_dquat.md) |
| `clone(_self)` | [No Documentation 🚧](./quat/clone.md) |
| `conjugate(_self)` | [ Returns the quaternion conjugate of \`self\`\. For a unit quaternion the  conjugate is also the invers](./quat/conjugate.md) |
| `div(_self, rhs)` | [ Divides a quaternion by a scalar value\.  The quotient is not guaranteed to be normalized\.](./quat/div.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\. The dot product is  equal to the cosine of the angle ](./quat/dot.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./quat/eq.md) |
| `from_affine3(a)` | [ Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform\.  Note if the input af](./quat/from_affine3.md) |
| `from_array(a)` | [ Creates a rotation quaternion from an array\.  \# Preconditions  This function does not check if the ](./quat/from_array.md) |
| `from_axis_angle(axis, angle)` | [ Create a quaternion for a normalized rotation \`axis\` and \`angle\` \(in radians\)\.  The axis must be a ](./quat/from_axis_angle.md) |
| `from_euler(euler, a, b, c)` | [ Creates a quaternion from the given Euler rotation sequence and the angles \(in radians\)\.](./quat/from_euler.md) |
| `from_mat3(mat)` | [ Creates a quaternion from a 3x3 rotation matrix\.  Note if the input matrix contain scales, shears, ](./quat/from_mat3.md) |
| `from_mat3a(mat)` | [ Creates a quaternion from a 3x3 SIMD aligned rotation matrix\.  Note if the input matrix contain sca](./quat/from_mat3a.md) |
| `from_mat4(mat)` | [ Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix\.  Note if t](./quat/from_mat4.md) |
| `from_rotation_arc(from, to)` | [ Gets the minimal rotation for transforming \`from\` to \`to\`\.  The rotation is in the  plane spanned b](./quat/from_rotation_arc.md) |
| `from_rotation_arc_2d(from, to)` | [ Gets the minimal rotation for transforming \`from\` to \`to\`\.  The resulting rotation is  around the z](./quat/from_rotation_arc_2d.md) |
| `from_rotation_arc_colinear(from, to)` | [ Gets the minimal rotation for transforming \`from\` to either \`to\` or \`\-to\`\.  This means  that the re](./quat/from_rotation_arc_colinear.md) |
| `from_rotation_x(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the x axis\.](./quat/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the y axis\.](./quat/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the z axis\.](./quat/from_rotation_z.md) |
| `from_scaled_axis(v)` | [ Create a quaternion that rotates \`v\.length\(\)\` radians around \`v\.normalize\(\)\`\.  \`from\_scaled\_axis\(Vec3::ZERO\)\`](./quat/from_scaled_axis.md) |
| `from_vec4(v)` | [ Creates a new rotation quaternion from a 4D vector\.  \# Preconditions  This function does not check ](./quat/from_vec4.md) |
| `from_xyzw(x, y, z, w)` | [ Creates a new rotation quaternion\.  This should generally not be called manually unless you know wh](./quat/from_xyzw.md) |
| `inverse(_self)` | [ Returns the inverse of a normalized quaternion\.  Typically quaternion inverse returns the conjugate](./quat/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./quat/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NAN\`\.](./quat/is_nan.md) |
| `is_near_identity(_self)` | [No Documentation 🚧](./quat/is_near_identity.md) |
| `is_normalized(_self)` | [ Returns whether \`self\` of length \`1\.0\` or not\.  Uses a precision threshold of \`1e\-6\`\.](./quat/is_normalized.md) |
| `length(_self)` | [ Computes the length of \`self\`\.](./quat/length.md) |
| `length_recip(_self)` | [ Computes \`1\.0 / length\(\)\`\.  For valid results, \`self\` must \_not\_ be of length zero\.](./quat/length_recip.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.  This is generally faster than \`length\(\)\` as it avoids a squ](./quat/length_squared.md) |
| `lerp(_self, end, s)` | [ Performs a linear interpolation between \`self\` and \`rhs\` based on  the value \`s\`\.  When \`s\` is \`0\.0](./quat/lerp.md) |
| `mul(_self, rhs)` | [ Multiplies two quaternions\. If they each represent a rotation, the result will  represent the combi](./quat/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./quat/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./quat/mul-2.md) |
| `mul-3(arg0, arg1)` | [No Documentation 🚧](./quat/mul-3.md) |
| `mul_quat(_self, rhs)` | [ Multiplies two quaternions\. If they each represent a rotation, the result will  represent the combi](./quat/mul_quat.md) |
| `mul_vec3(_self, rhs)` | [ Multiplies a quaternion and a 3D vector, returning the rotated vector\.  \# Panics  Will panic if \`self\`](./quat/mul_vec3.md) |
| `mul_vec3a(_self, rhs)` | [ Multiplies a quaternion and a 3D vector, returning the rotated vector\.](./quat/mul_vec3a.md) |
| `neg(_self)` | [No Documentation 🚧](./quat/neg.md) |
| `normalize(_self)` | [ Returns \`self\` normalized to length 1\.0\.  For valid results, \`self\` must \_not\_ be of length zero\.  ](./quat/normalize.md) |
| `rotate_towards(_self, rhs, max_angle)` | [ Rotates towards \`rhs\` up to \`max\_angle\` \(in radians\)\.  When \`max\_angle\` is \`0\.0\`, the result will b](./quat/rotate_towards.md) |
| `slerp(_self, end, s)` | [ Performs a spherical linear interpolation between \`self\` and \`end\`  based on the value \`s\`\.  When \`s\`](./quat/slerp.md) |
| `sub(_self, rhs)` | [ Subtracts the \`rhs\` quaternion from \`self\`\.  The difference is not guaranteed to be normalized\.](./quat/sub.md) |
| `to_array(_self)` | [ \`\[x, y, z, w\]\`](./quat/to_array.md) |
| `to_euler(_self, order)` | [ Returns the rotation angles for the given euler rotation sequence\.](./quat/to_euler.md) |
| `to_scaled_axis(_self)` | [ Returns the rotation axis scaled by the rotation in radians\.](./quat/to_scaled_axis.md) |
| `xyz(_self)` | [ Returns the vector part of the quaternion\.](./quat/xyz.md) |