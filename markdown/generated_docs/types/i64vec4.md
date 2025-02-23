# I64Vec4

### I64Vec4

- **x** : i64
- **y** : i64
- **z** : i64
- **w** : i64

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Returns a vector containing the absolute value of each element of \`self\`\.](./i64vec4/abs.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./i64vec4/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./i64vec4/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./i64vec4/add-2.md) |
| `as_dvec4(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./i64vec4/as_dvec4.md) |
| `as_ivec4(_self)` | [ Casts all elements of \`self\` to \`i32\`\.](./i64vec4/as_ivec4.md) |
| `as_u64vec4(_self)` | [ Casts all elements of \`self\` to \`u64\`\.](./i64vec4/as_u64vec4.md) |
| `as_uvec4(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./i64vec4/as_uvec4.md) |
| `as_vec4(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./i64vec4/as_vec4.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./i64vec4/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`i64::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./i64vec4/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./i64vec4/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./i64vec4/cmpne.md) |
| `distance_squared(_self, rhs)` | [ Compute the squared euclidean distance between two points in space\.](./i64vec4/distance_squared.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./i64vec4/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./i64vec4/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./i64vec4/div-2.md) |
| `div_euclid(_self, rhs)` | [ Returns the element\-wise quotient of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.](./i64vec4/div_euclid.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./i64vec4/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./i64vec4/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./i64vec4/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./i64vec4/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./i64vec4/eq.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./i64vec4/from_array.md) |
| `is_negative_bitmask(_self)` | [ Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of \`self\`\.  A negat](./i64vec4/is_negative_bitmask.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./i64vec4/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./i64vec4/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./i64vec4/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./i64vec4/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./i64vec4/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./i64vec4/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./i64vec4/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./i64vec4/mul-2.md) |
| `neg(_self)` | [No Documentation 🚧](./i64vec4/neg.md) |
| `new(x, y, z, w)` | [ Creates a new vector\.](./i64vec4/new.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./i64vec4/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./i64vec4/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./i64vec4/rem-2.md) |
| `rem_euclid(_self, rhs)` | [ Returns the element\-wise remainder of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.  \[Euclidean division\]](./i64vec4/rem_euclid.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./i64vec4/saturating_add.md) |
| `saturating_add_unsigned(_self, rhs)` | [ In other words this computes \`\[self\.x\.saturating\_add\_unsigned\(rhs\.x\), self\.y\.saturating\_add\_unsigned\(rhs\.y\), \.\.\]\`\.](./i64vec4/saturating_add_unsigned.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./i64vec4/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./i64vec4/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./i64vec4/saturating_sub.md) |
| `saturating_sub_unsigned(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and unsigned vector \`rhs\`\.  In oth](./i64vec4/saturating_sub_unsigned.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./i64vec4/select.md) |
| `signum(_self)` | [ Returns a vector with elements representing the sign of \`self\`\.   \- \`0\` if the number is zero   \- \`1\`](./i64vec4/signum.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./i64vec4/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./i64vec4/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./i64vec4/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./i64vec4/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y, z, w\]\`](./i64vec4/to_array.md) |
| `truncate(_self)` | [ Creates a 3D vector from the \`x\`, \`y\` and \`z\` elements of \`self\`, discarding \`w\`\.  Truncation to \[\`I64Vec3\`\]](./i64vec4/truncate.md) |
| `with_w(_self, w)` | [ Creates a 4D vector from \`self\` with the given value of \`w\`\.](./i64vec4/with_w.md) |
| `with_x(_self, x)` | [ Creates a 4D vector from \`self\` with the given value of \`x\`\.](./i64vec4/with_x.md) |
| `with_y(_self, y)` | [ Creates a 4D vector from \`self\` with the given value of \`y\`\.](./i64vec4/with_y.md) |
| `with_z(_self, z)` | [ Creates a 4D vector from \`self\` with the given value of \`z\`\.](./i64vec4/with_z.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./i64vec4/wrapping_add.md) |
| `wrapping_add_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and unsigned vector \`rhs\`\.  In other wo](./i64vec4/wrapping_add_unsigned.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./i64vec4/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./i64vec4/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./i64vec4/wrapping_sub.md) |
| `wrapping_sub_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and unsigned vector \`rhs\`\.  In other](./i64vec4/wrapping_sub_unsigned.md) |