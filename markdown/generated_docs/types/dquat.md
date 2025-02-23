# DQuat

### DQuat

- **x** : f64
- **y** : f64
- **z** : f64
- **w** : f64

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\`  is less than or e](./dquat/abs_diff_eq.md) |
| `add(_self, rhs)` | [ Adds two quaternions\.  The sum is not guaranteed to be normalized\.  Note that addition is not the s](./dquat/add.md) |
| `angle_between(_self, rhs)` | [ Returns the angle \(in radians\) for the minimal rotation  for transforming this quaternion into anot](./dquat/angle_between.md) |
| `as_quat(_self)` | [No Documentation 🚧](./dquat/as_quat.md) |
| `clone(_self)` | [No Documentation 🚧](./dquat/clone.md) |
| `conjugate(_self)` | [ Returns the quaternion conjugate of \`self\`\. For a unit quaternion the  conjugate is also the invers](./dquat/conjugate.md) |
| `div(_self, rhs)` | [ Divides a quaternion by a scalar value\.  The quotient is not guaranteed to be normalized\.](./dquat/div.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\. The dot product is  equal to the cosine of the angle ](./dquat/dot.md) |
| `eq(_self, rhs)` | [No Documentation 🚧](./dquat/eq.md) |
| `from_affine3(a)` | [ Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform\.  Note if the input af](./dquat/from_affine3.md) |
| `from_array(a)` | [ Creates a rotation quaternion from an array\.  \# Preconditions  This function does not check if the ](./dquat/from_array.md) |
| `from_axis_angle(axis, angle)` | [ Create a quaternion for a normalized rotation \`axis\` and \`angle\` \(in radians\)\.  The axis must be a ](./dquat/from_axis_angle.md) |
| `from_euler(euler, a, b, c)` | [ Creates a quaternion from the given Euler rotation sequence and the angles \(in radians\)\.](./dquat/from_euler.md) |
| `from_mat3(mat)` | [ Creates a quaternion from a 3x3 rotation matrix\.  Note if the input matrix contain scales, shears, ](./dquat/from_mat3.md) |
| `from_mat4(mat)` | [ Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix\.  Note if t](./dquat/from_mat4.md) |
| `from_rotation_arc(from, to)` | [ Gets the minimal rotation for transforming \`from\` to \`to\`\.  The rotation is in the  plane spanned b](./dquat/from_rotation_arc.md) |
| `from_rotation_arc_2d(from, to)` | [ Gets the minimal rotation for transforming \`from\` to \`to\`\.  The resulting rotation is  around the z](./dquat/from_rotation_arc_2d.md) |
| `from_rotation_arc_colinear(from, to)` | [ Gets the minimal rotation for transforming \`from\` to either \`to\` or \`\-to\`\.  This means  that the re](./dquat/from_rotation_arc_colinear.md) |
| `from_rotation_x(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the x axis\.](./dquat/from_rotation_x.md) |
| `from_rotation_y(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the y axis\.](./dquat/from_rotation_y.md) |
| `from_rotation_z(angle)` | [ Creates a quaternion from the \`angle\` \(in radians\) around the z axis\.](./dquat/from_rotation_z.md) |
| `from_scaled_axis(v)` | [ Create a quaternion that rotates \`v\.length\(\)\` radians around \`v\.normalize\(\)\`\.  \`from\_scaled\_axis\(Vec3::ZERO\)\`](./dquat/from_scaled_axis.md) |
| `from_vec4(v)` | [ Creates a new rotation quaternion from a 4D vector\.  \# Preconditions  This function does not check ](./dquat/from_vec4.md) |
| `from_xyzw(x, y, z, w)` | [ Creates a new rotation quaternion\.  This should generally not be called manually unless you know wh](./dquat/from_xyzw.md) |
| `inverse(_self)` | [ Returns the inverse of a normalized quaternion\.  Typically quaternion inverse returns the conjugate](./dquat/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either \`NaN\`, positive ](./dquat/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NAN\`\.](./dquat/is_nan.md) |
| `is_near_identity(_self)` | [No Documentation 🚧](./dquat/is_near_identity.md) |
| `is_normalized(_self)` | [ Returns whether \`self\` of length \`1\.0\` or not\.  Uses a precision threshold of \`1e\-6\`\.](./dquat/is_normalized.md) |
| `length(_self)` | [ Computes the length of \`self\`\.](./dquat/length.md) |
| `length_recip(_self)` | [ Computes \`1\.0 / length\(\)\`\.  For valid results, \`self\` must \_not\_ be of length zero\.](./dquat/length_recip.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.  This is generally faster than \`length\(\)\` as it avoids a squ](./dquat/length_squared.md) |
| `lerp(_self, end, s)` | [ Performs a linear interpolation between \`self\` and \`rhs\` based on  the value \`s\`\.  When \`s\` is \`0\.0](./dquat/lerp.md) |
| `mul(_self, rhs)` | [ Multiplies two quaternions\. If they each represent a rotation, the result will  represent the combi](./dquat/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./dquat/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./dquat/mul-2.md) |
| `mul_quat(_self, rhs)` | [ Multiplies two quaternions\. If they each represent a rotation, the result will  represent the combi](./dquat/mul_quat.md) |
| `mul_vec3(_self, rhs)` | [ Multiplies a quaternion and a 3D vector, returning the rotated vector\.  \# Panics  Will panic if \`self\`](./dquat/mul_vec3.md) |
| `neg(_self)` | [No Documentation 🚧](./dquat/neg.md) |
| `normalize(_self)` | [ Returns \`self\` normalized to length 1\.0\.  For valid results, \`self\` must \_not\_ be of length zero\.  ](./dquat/normalize.md) |
| `rotate_towards(_self, rhs, max_angle)` | [ Rotates towards \`rhs\` up to \`max\_angle\` \(in radians\)\.  When \`max\_angle\` is \`0\.0\`, the result will b](./dquat/rotate_towards.md) |
| `slerp(_self, end, s)` | [ Performs a spherical linear interpolation between \`self\` and \`end\`  based on the value \`s\`\.  When \`s\`](./dquat/slerp.md) |
| `sub(_self, rhs)` | [ Subtracts the \`rhs\` quaternion from \`self\`\.  The difference is not guaranteed to be normalized\.](./dquat/sub.md) |
| `to_array(_self)` | [ \`\[x, y, z, w\]\`](./dquat/to_array.md) |
| `to_euler(_self, order)` | [ Returns the rotation angles for the given euler rotation sequence\.](./dquat/to_euler.md) |
| `to_scaled_axis(_self)` | [ Returns the rotation axis scaled by the rotation in radians\.](./dquat/to_scaled_axis.md) |
| `xyz(_self)` | [ Returns the vector part of the quaternion\.](./dquat/xyz.md) |