# IVec3

### IVec3

- **x** : i32
- **y** : i32
- **z** : i32

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Returns a vector containing the absolute value of each element of \`self\`\.](./ivec3/abs.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./ivec3/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./ivec3/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./ivec3/add-2.md) |
| `as_dvec3(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./ivec3/as_dvec3.md) |
| `as_i64vec3(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./ivec3/as_i64vec3.md) |
| `as_u64vec3(_self)` | [ Casts all elements of \`self\` to \`u64\`\.](./ivec3/as_u64vec3.md) |
| `as_uvec3(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./ivec3/as_uvec3.md) |
| `as_vec3(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./ivec3/as_vec3.md) |
| `as_vec3a(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./ivec3/as_vec3a.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./ivec3/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`i32::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./ivec3/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./ivec3/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./ivec3/cmpne.md) |
| `cross(_self, rhs)` | [ Computes the cross product of \`self\` and \`rhs\`\.](./ivec3/cross.md) |
| `distance_squared(_self, rhs)` | [ Compute the squared euclidean distance between two points in space\.](./ivec3/distance_squared.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./ivec3/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./ivec3/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./ivec3/div-2.md) |
| `div_euclid(_self, rhs)` | [ Returns the element\-wise quotient of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.](./ivec3/div_euclid.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./ivec3/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./ivec3/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./ivec3/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./ivec3/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./ivec3/eq.md) |
| `extend(_self, w)` | [ Creates a 4D vector from \`self\` and the given \`w\` value\.](./ivec3/extend.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./ivec3/from_array.md) |
| `is_negative_bitmask(_self)` | [ Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of \`self\`\.  A negat](./ivec3/is_negative_bitmask.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./ivec3/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./ivec3/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./ivec3/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./ivec3/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./ivec3/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./ivec3/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./ivec3/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./ivec3/mul-2.md) |
| `neg(_self)` | [No Documentation 🚧](./ivec3/neg.md) |
| `new(x, y, z)` | [ Creates a new vector\.](./ivec3/new.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./ivec3/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./ivec3/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./ivec3/rem-2.md) |
| `rem_euclid(_self, rhs)` | [ Returns the element\-wise remainder of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.  \[Euclidean division\]](./ivec3/rem_euclid.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./ivec3/saturating_add.md) |
| `saturating_add_unsigned(_self, rhs)` | [ In other words this computes \`\[self\.x\.saturating\_add\_unsigned\(rhs\.x\), self\.y\.saturating\_add\_unsigned\(rhs\.y\), \.\.\]\`\.](./ivec3/saturating_add_unsigned.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./ivec3/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./ivec3/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./ivec3/saturating_sub.md) |
| `saturating_sub_unsigned(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and unsigned vector \`rhs\`\.  In oth](./ivec3/saturating_sub_unsigned.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./ivec3/select.md) |
| `signum(_self)` | [ Returns a vector with elements representing the sign of \`self\`\.   \- \`0\` if the number is zero   \- \`1\`](./ivec3/signum.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./ivec3/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./ivec3/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./ivec3/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./ivec3/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y, z\]\`](./ivec3/to_array.md) |
| `truncate(_self)` | [ Creates a 2D vector from the \`x\` and \`y\` elements of \`self\`, discarding \`z\`\.  Truncation may also b](./ivec3/truncate.md) |
| `with_x(_self, x)` | [ Creates a 3D vector from \`self\` with the given value of \`x\`\.](./ivec3/with_x.md) |
| `with_y(_self, y)` | [ Creates a 3D vector from \`self\` with the given value of \`y\`\.](./ivec3/with_y.md) |
| `with_z(_self, z)` | [ Creates a 3D vector from \`self\` with the given value of \`z\`\.](./ivec3/with_z.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./ivec3/wrapping_add.md) |
| `wrapping_add_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and unsigned vector \`rhs\`\.  In other wo](./ivec3/wrapping_add_unsigned.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./ivec3/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./ivec3/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./ivec3/wrapping_sub.md) |
| `wrapping_sub_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and unsigned vector \`rhs\`\.  In other](./ivec3/wrapping_sub_unsigned.md) |