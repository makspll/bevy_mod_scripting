# Vec3

### Vec3

- **x** : f32
- **y** : f32
- **z** : f32

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Returns a vector containing the absolute value of each element of \`self\`\.](./vec3/abs.md) |
| `abs_diff_eq(_self, rhs, max_abs_diff)` | [ Returns true if the absolute difference of all elements between \`self\` and \`rhs\` is  less than or e](./vec3/abs_diff_eq.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./vec3/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./vec3/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./vec3/add-2.md) |
| `angle_between(_self, rhs)` | [ Returns the angle \(in radians\) between two vectors in the range \`\[0, \+π\]\`\.  The inputs do not need to be unit vectors however they must be non\-zero\.](./vec3/angle_between.md) |
| `any_orthogonal_vector(_self)` | [ Returns some vector that is orthogonal to the given one\.  The input vector must be finite and non\-z](./vec3/any_orthogonal_vector.md) |
| `any_orthonormal_vector(_self)` | [ Returns any unit vector that is orthogonal to the given one\.  The input vector must be unit length\.](./vec3/any_orthonormal_vector.md) |
| `as_dvec3(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./vec3/as_dvec3.md) |
| `as_i64vec3(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./vec3/as_i64vec3.md) |
| `as_ivec3(_self)` | [ Casts all elements of \`self\` to \`i32\`\.](./vec3/as_ivec3.md) |
| `as_u64vec3(_self)` | [ Casts all elements of \`self\` to \`u64\`\.](./vec3/as_u64vec3.md) |
| `as_uvec3(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./vec3/as_uvec3.md) |
| `ceil(_self)` | [ Returns a vector containing the smallest integer greater than or equal to a number for  each elemen](./vec3/ceil.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`f32::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./vec3/clamp.md) |
| `clamp_length(_self, min, max)` | [ Returns a vector with a length no less than \`min\` and no more than \`max\`\.  \# Panics  Will panic if \`min\`](./vec3/clamp_length.md) |
| `clamp_length_max(_self, max)` | [ Returns a vector with a length no more than \`max\`\.  \# Panics  Will panic if \`max\` is negative when \`glam\_assert\` is enabled\.](./vec3/clamp_length_max.md) |
| `clamp_length_min(_self, min)` | [ Returns a vector with a length no less than \`min\`\.  \# Panics  Will panic if \`min\` is negative when \`glam\_assert\` is enabled\.](./vec3/clamp_length_min.md) |
| `clone(_self)` | [No Documentation 🚧](./vec3/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./vec3/cmpne.md) |
| `copysign(_self, rhs)` | [ Returns a vector with signs of \`rhs\` and the magnitudes of \`self\`\.](./vec3/copysign.md) |
| `cross(_self, rhs)` | [ Computes the cross product of \`self\` and \`rhs\`\.](./vec3/cross.md) |
| `distance(_self, rhs)` | [ Computes the Euclidean distance between two points in space\.](./vec3/distance.md) |
| `distance_squared(_self, rhs)` | [ Compute the squared euclidean distance between two points in space\.](./vec3/distance_squared.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./vec3/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./vec3/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./vec3/div-2.md) |
| `div_euclid(_self, rhs)` | [ Returns the element\-wise quotient of \[Euclidean division\] of \`self\` by \`rhs\`\.](./vec3/div_euclid.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./vec3/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./vec3/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./vec3/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./vec3/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./vec3/eq.md) |
| `exp(_self)` | [ Returns a vector containing \`e^self\` \(the exponential function\) for each element of  \`self\`\.](./vec3/exp.md) |
| `extend(_self, w)` | [ Creates a 4D vector from \`self\` and the given \`w\` value\.](./vec3/extend.md) |
| `floor(_self)` | [ Returns a vector containing the largest integer less than or equal to a number for each  element of](./vec3/floor.md) |
| `fract(_self)` | [ Returns a vector containing the fractional part of the vector as \`self \- self\.trunc\(\)\`\.  Note that ](./vec3/fract.md) |
| `fract_gl(_self)` | [ Returns a vector containing the fractional part of the vector as \`self \- self\.floor\(\)\`\.  Note that ](./vec3/fract_gl.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./vec3/from_array.md) |
| `is_finite(_self)` | [ Returns \`true\` if, and only if, all elements are finite\.  If any element is either  \`NaN\`, positive](./vec3/is_finite.md) |
| `is_finite_mask(_self)` | [ Performs \`is\_finite\` on each element of self, returning a vector mask of the results\.  In other words, this computes \`\[x\.is\_finite\(\), y\.is\_finite\(\), \.\.\.\]](./vec3/is_finite_mask.md) |
| `is_nan(_self)` | [ Returns \`true\` if any elements are \`NaN\`\.](./vec3/is_nan.md) |
| `is_nan_mask(_self)` | [ Performs \`is\_nan\` on each element of self, returning a vector mask of the results\.  In other words, this computes \`\[x\.is\_nan\(\), y\.is\_nan\(\), \.\.\.\]](./vec3/is_nan_mask.md) |
| `is_negative_bitmask(_self)` | [ Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of \`self\`\.  A negat](./vec3/is_negative_bitmask.md) |
| `is_normalized(_self)` | [ Returns whether \`self\` is length \`1\.0\` or not\.  Uses a precision threshold of approximately \`1e\-4\`\.](./vec3/is_normalized.md) |
| `length(_self)` | [ Computes the length of \`self\`\.](./vec3/length.md) |
| `length_recip(_self)` | [ Computes \`1\.0 / length\(\)\`\.  For valid results, \`self\` must \_not\_ be of length zero\.](./vec3/length_recip.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.  This is faster than \`length\(\)\` as it avoids a square root o](./vec3/length_squared.md) |
| `lerp(_self, rhs, s)` | [ Performs a linear interpolation between \`self\` and \`rhs\` based on the value \`s\`\.  When \`s\` is \`0\.0\`](./vec3/lerp.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./vec3/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./vec3/max_element.md) |
| `midpoint(_self, rhs)` | [ Calculates the midpoint between \`self\` and \`rhs\`\.  The midpoint is the average of, or halfway point](./vec3/midpoint.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./vec3/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./vec3/min_element.md) |
| `move_towards(_self, rhs, d)` | [ Moves towards \`rhs\` based on the value \`d\`\.  When \`d\` is \`0\.0\`, the result will be equal to \`self\`\.](./vec3/move_towards.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./vec3/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./vec3/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./vec3/mul-2.md) |
| `mul_add(_self, a, b)` | [ Fused multiply\-add\. Computes \`\(self \* a\) \+ b\` element\-wise with only one rounding  error, yielding a more accurate result than an unfused multiply\-add\.  Using \`mul\_add\` \*may\* be more performant than an unfused multiply\-add if the target  architecture has a dedicated fma CPU instruction\. However, this is not always true,  and will be heavily dependant on designing algorithms with specific target hardware in  mind\.](./vec3/mul_add.md) |
| `neg(_self)` | [No Documentation 🚧](./vec3/neg.md) |
| `new(x, y, z)` | [ Creates a new vector\.](./vec3/new.md) |
| `normalize(_self)` | [ Returns \`self\` normalized to length 1\.0\.  For valid results, \`self\` must be finite and \_not\_ of len](./vec3/normalize.md) |
| `normalize_or(_self, fallback)` | [ Returns \`self\` normalized to length 1\.0 if possible, else returns a  fallback value\.  In particular](./vec3/normalize_or.md) |
| `normalize_or_zero(_self)` | [ Returns \`self\` normalized to length 1\.0 if possible, else returns zero\.  In particular, if the inpu](./vec3/normalize_or_zero.md) |
| `powf(_self, n)` | [ Returns a vector containing each element of \`self\` raised to the power of \`n\`\.](./vec3/powf.md) |
| `project_onto(_self, rhs)` | [ Returns the vector projection of \`self\` onto \`rhs\`\.  \`rhs\` must be of non\-zero length\.  \# Panics  W](./vec3/project_onto.md) |
| `project_onto_normalized(_self, rhs)` | [ Returns the vector projection of \`self\` onto \`rhs\`\.  \`rhs\` must be normalized\.  \# Panics  Will pani](./vec3/project_onto_normalized.md) |
| `recip(_self)` | [ Returns a vector containing the reciprocal \`1\.0/n\` of each element of \`self\`\.](./vec3/recip.md) |
| `reflect(_self, normal)` | [ Returns the reflection vector for a given incident vector \`self\` and surface normal  \`normal\`\.  \`normal\`](./vec3/reflect.md) |
| `refract(_self, normal, eta)` | [ Returns the refraction direction for a given incident vector \`self\`, surface normal  \`normal\` and r](./vec3/refract.md) |
| `reject_from(_self, rhs)` | [ Returns the vector rejection of \`self\` from \`rhs\`\.  The vector rejection is the vector perpendicula](./vec3/reject_from.md) |
| `reject_from_normalized(_self, rhs)` | [ Returns the vector rejection of \`self\` from \`rhs\`\.  The vector rejection is the vector perpendicula](./vec3/reject_from_normalized.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./vec3/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./vec3/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./vec3/rem-2.md) |
| `rem_euclid(_self, rhs)` | [ Returns the element\-wise remainder of \[Euclidean division\] of \`self\` by \`rhs\`\.  \[Euclidean division](./vec3/rem_euclid.md) |
| `round(_self)` | [ Returns a vector containing the nearest integer to a number for each element of \`self\`\.  Round half](./vec3/round.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./vec3/select.md) |
| `signum(_self)` | [ Returns a vector with elements representing the sign of \`self\`\.  \- \`1\.0\` if the number is positive,](./vec3/signum.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./vec3/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./vec3/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./vec3/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./vec3/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y, z\]\`](./vec3/to_array.md) |
| `trunc(_self)` | [ Returns a vector containing the integer part each element of \`self\`\. This means numbers are  always](./vec3/trunc.md) |
| `truncate(_self)` | [ Creates a 2D vector from the \`x\` and \`y\` elements of \`self\`, discarding \`z\`\.  Truncation may also b](./vec3/truncate.md) |
| `with_x(_self, x)` | [ Creates a 3D vector from \`self\` with the given value of \`x\`\.](./vec3/with_x.md) |
| `with_y(_self, y)` | [ Creates a 3D vector from \`self\` with the given value of \`y\`\.](./vec3/with_y.md) |
| `with_z(_self, z)` | [ Creates a 3D vector from \`self\` with the given value of \`z\`\.](./vec3/with_z.md) |