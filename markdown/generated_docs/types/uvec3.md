# UVec3

### UVec3

- **x** : u32
- **y** : u32
- **z** : u32

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `add(_self, rhs)` | [No Documentation 🚧](./uvec3/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./uvec3/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./uvec3/add-2.md) |
| `as_dvec3(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./uvec3/as_dvec3.md) |
| `as_i64vec3(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./uvec3/as_i64vec3.md) |
| `as_ivec3(_self)` | [ Casts all elements of \`self\` to \`i32\`\.](./uvec3/as_ivec3.md) |
| `as_u64vec3(_self)` | [ Casts all elements of \`self\` to \`u64\`\.](./uvec3/as_u64vec3.md) |
| `as_vec3(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./uvec3/as_vec3.md) |
| `as_vec3a(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./uvec3/as_vec3a.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./uvec3/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`u32::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./uvec3/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./uvec3/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./uvec3/cmpne.md) |
| `cross(_self, rhs)` | [ Computes the cross product of \`self\` and \`rhs\`\.](./uvec3/cross.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./uvec3/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./uvec3/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./uvec3/div-2.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./uvec3/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./uvec3/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./uvec3/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./uvec3/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./uvec3/eq.md) |
| `extend(_self, w)` | [ Creates a 4D vector from \`self\` and the given \`w\` value\.](./uvec3/extend.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./uvec3/from_array.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./uvec3/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./uvec3/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./uvec3/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./uvec3/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./uvec3/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./uvec3/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./uvec3/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./uvec3/mul-2.md) |
| `new(x, y, z)` | [ Creates a new vector\.](./uvec3/new.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./uvec3/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./uvec3/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./uvec3/rem-2.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./uvec3/saturating_add.md) |
| `saturating_add_signed(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and signed vector \`rhs\`\.  In other wo](./uvec3/saturating_add_signed.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./uvec3/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./uvec3/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./uvec3/saturating_sub.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./uvec3/select.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./uvec3/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./uvec3/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./uvec3/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./uvec3/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y, z\]\`](./uvec3/to_array.md) |
| `truncate(_self)` | [ Creates a 2D vector from the \`x\` and \`y\` elements of \`self\`, discarding \`z\`\.  Truncation may also b](./uvec3/truncate.md) |
| `with_x(_self, x)` | [ Creates a 3D vector from \`self\` with the given value of \`x\`\.](./uvec3/with_x.md) |
| `with_y(_self, y)` | [ Creates a 3D vector from \`self\` with the given value of \`y\`\.](./uvec3/with_y.md) |
| `with_z(_self, z)` | [ Creates a 3D vector from \`self\` with the given value of \`z\`\.](./uvec3/with_z.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./uvec3/wrapping_add.md) |
| `wrapping_add_signed(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and signed vector \`rhs\`\.  In other word](./uvec3/wrapping_add_signed.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./uvec3/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./uvec3/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./uvec3/wrapping_sub.md) |