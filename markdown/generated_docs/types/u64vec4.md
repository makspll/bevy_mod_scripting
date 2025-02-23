# U64Vec4

### U64Vec4

- **x** : u64
- **y** : u64
- **z** : u64
- **w** : u64

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `add(_self, rhs)` | [No Documentation 🚧](./u64vec4/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./u64vec4/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./u64vec4/add-2.md) |
| `as_dvec4(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./u64vec4/as_dvec4.md) |
| `as_i64vec4(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./u64vec4/as_i64vec4.md) |
| `as_ivec4(_self)` | [ Casts all elements of \`self\` to \`i32\`\.](./u64vec4/as_ivec4.md) |
| `as_uvec4(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./u64vec4/as_uvec4.md) |
| `as_vec4(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./u64vec4/as_vec4.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./u64vec4/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`u64::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./u64vec4/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./u64vec4/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec4/cmpne.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./u64vec4/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./u64vec4/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./u64vec4/div-2.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./u64vec4/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./u64vec4/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./u64vec4/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./u64vec4/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./u64vec4/eq.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./u64vec4/from_array.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./u64vec4/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./u64vec4/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./u64vec4/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./u64vec4/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./u64vec4/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./u64vec4/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./u64vec4/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./u64vec4/mul-2.md) |
| `new(x, y, z, w)` | [ Creates a new vector\.](./u64vec4/new.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./u64vec4/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./u64vec4/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./u64vec4/rem-2.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./u64vec4/saturating_add.md) |
| `saturating_add_signed(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and signed vector \`rhs\`\.  In other wo](./u64vec4/saturating_add_signed.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./u64vec4/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./u64vec4/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./u64vec4/saturating_sub.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./u64vec4/select.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./u64vec4/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./u64vec4/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./u64vec4/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./u64vec4/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y, z, w\]\`](./u64vec4/to_array.md) |
| `truncate(_self)` | [ Creates a 3D vector from the \`x\`, \`y\` and \`z\` elements of \`self\`, discarding \`w\`\.  Truncation to \[\`U64Vec3\`\]](./u64vec4/truncate.md) |
| `with_w(_self, w)` | [ Creates a 4D vector from \`self\` with the given value of \`w\`\.](./u64vec4/with_w.md) |
| `with_x(_self, x)` | [ Creates a 4D vector from \`self\` with the given value of \`x\`\.](./u64vec4/with_x.md) |
| `with_y(_self, y)` | [ Creates a 4D vector from \`self\` with the given value of \`y\`\.](./u64vec4/with_y.md) |
| `with_z(_self, z)` | [ Creates a 4D vector from \`self\` with the given value of \`z\`\.](./u64vec4/with_z.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./u64vec4/wrapping_add.md) |
| `wrapping_add_signed(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and signed vector \`rhs\`\.  In other word](./u64vec4/wrapping_add_signed.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./u64vec4/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./u64vec4/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./u64vec4/wrapping_sub.md) |