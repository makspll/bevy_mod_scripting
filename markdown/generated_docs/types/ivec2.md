# IVec2

### IVec2

- **x** : i32
- **y** : i32

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs(_self)` | [ Returns a vector containing the absolute value of each element of \`self\`\.](./ivec2/abs.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./ivec2/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./ivec2/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./ivec2/add-2.md) |
| `as_dvec2(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./ivec2/as_dvec2.md) |
| `as_i64vec2(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./ivec2/as_i64vec2.md) |
| `as_u64vec2(_self)` | [ Casts all elements of \`self\` to \`u64\`\.](./ivec2/as_u64vec2.md) |
| `as_uvec2(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./ivec2/as_uvec2.md) |
| `as_vec2(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./ivec2/as_vec2.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./ivec2/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`i32::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./ivec2/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./ivec2/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./ivec2/cmpne.md) |
| `distance_squared(_self, rhs)` | [ Compute the squared euclidean distance between two points in space\.](./ivec2/distance_squared.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./ivec2/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./ivec2/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./ivec2/div-2.md) |
| `div_euclid(_self, rhs)` | [ Returns the element\-wise quotient of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.](./ivec2/div_euclid.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./ivec2/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./ivec2/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./ivec2/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./ivec2/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./ivec2/eq.md) |
| `extend(_self, z)` | [ Creates a 3D vector from \`self\` and the given \`z\` value\.](./ivec2/extend.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./ivec2/from_array.md) |
| `is_negative_bitmask(_self)` | [ Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of \`self\`\.  A negat](./ivec2/is_negative_bitmask.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./ivec2/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./ivec2/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./ivec2/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./ivec2/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./ivec2/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./ivec2/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./ivec2/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./ivec2/mul-2.md) |
| `neg(_self)` | [No Documentation 🚧](./ivec2/neg.md) |
| `new(x, y)` | [ Creates a new vector\.](./ivec2/new.md) |
| `perp(_self)` | [ Returns a vector that is equal to \`self\` rotated by 90 degrees\.](./ivec2/perp.md) |
| `perp_dot(_self, rhs)` | [ The perpendicular dot product of \`self\` and \`rhs\`\.  Also known as the wedge product, 2D cross produ](./ivec2/perp_dot.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./ivec2/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./ivec2/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./ivec2/rem-2.md) |
| `rem_euclid(_self, rhs)` | [ Returns the element\-wise remainder of \[Euclidean division\] of \`self\` by \`rhs\`\.  \# Panics  This function will panic if any \`rhs\` element is 0 or the division results in overflow\.  \[Euclidean division\]](./ivec2/rem_euclid.md) |
| `rotate(_self, rhs)` | [ Returns \`rhs\` rotated by the angle of \`self\`\. If \`self\` is normalized,  then this just rotation\. Th](./ivec2/rotate.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./ivec2/saturating_add.md) |
| `saturating_add_unsigned(_self, rhs)` | [ In other words this computes \`\[self\.x\.saturating\_add\_unsigned\(rhs\.x\), self\.y\.saturating\_add\_unsigned\(rhs\.y\), \.\.\]\`\.](./ivec2/saturating_add_unsigned.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./ivec2/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./ivec2/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./ivec2/saturating_sub.md) |
| `saturating_sub_unsigned(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and unsigned vector \`rhs\`\.  In oth](./ivec2/saturating_sub_unsigned.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./ivec2/select.md) |
| `signum(_self)` | [ Returns a vector with elements representing the sign of \`self\`\.   \- \`0\` if the number is zero   \- \`1\`](./ivec2/signum.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./ivec2/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./ivec2/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./ivec2/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./ivec2/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y\]\`](./ivec2/to_array.md) |
| `with_x(_self, x)` | [ Creates a 2D vector from \`self\` with the given value of \`x\`\.](./ivec2/with_x.md) |
| `with_y(_self, y)` | [ Creates a 2D vector from \`self\` with the given value of \`y\`\.](./ivec2/with_y.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./ivec2/wrapping_add.md) |
| `wrapping_add_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and unsigned vector \`rhs\`\.  In other wo](./ivec2/wrapping_add_unsigned.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./ivec2/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./ivec2/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./ivec2/wrapping_sub.md) |
| `wrapping_sub_unsigned(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and unsigned vector \`rhs\`\.  In other](./ivec2/wrapping_sub_unsigned.md) |