# U64Vec2

### U64Vec2

- **x** : u64
- **y** : u64

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `add(_self, rhs)` | [No Documentation 🚧](./u64vec2/add.md) |
| `add-1(arg0, arg1)` | [No Documentation 🚧](./u64vec2/add-1.md) |
| `add-2(arg0, arg1)` | [No Documentation 🚧](./u64vec2/add-2.md) |
| `as_dvec2(_self)` | [ Casts all elements of \`self\` to \`f64\`\.](./u64vec2/as_dvec2.md) |
| `as_i64vec2(_self)` | [ Casts all elements of \`self\` to \`i64\`\.](./u64vec2/as_i64vec2.md) |
| `as_ivec2(_self)` | [ Casts all elements of \`self\` to \`i32\`\.](./u64vec2/as_ivec2.md) |
| `as_uvec2(_self)` | [ Casts all elements of \`self\` to \`u32\`\.](./u64vec2/as_uvec2.md) |
| `as_vec2(_self)` | [ Casts all elements of \`self\` to \`f32\`\.](./u64vec2/as_vec2.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./u64vec2/assert_receiver_is_total_eq.md) |
| `clamp(_self, min, max)` | [ Component\-wise clamping of values, similar to \[\`u64::clamp\`\]\.  Each element in \`min\` must be less\-or\-equal to the corresponding element in \`max\`\.  \# Panics  Will panic if \`min\` is greater than \`max\` when \`glam\_assert\` is enabled\.](./u64vec2/clamp.md) |
| `clone(_self)` | [No Documentation 🚧](./u64vec2/clone.md) |
| `cmpeq(_self, rhs)` | [ Returns a vector mask containing the result of a \`==\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmpeq.md) |
| `cmpge(_self, rhs)` | [ Returns a vector mask containing the result of a \`>=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmpge.md) |
| `cmpgt(_self, rhs)` | [ Returns a vector mask containing the result of a \`>\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmpgt.md) |
| `cmple(_self, rhs)` | [ Returns a vector mask containing the result of a \`<=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmple.md) |
| `cmplt(_self, rhs)` | [ Returns a vector mask containing the result of a \`<\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmplt.md) |
| `cmpne(_self, rhs)` | [ Returns a vector mask containing the result of a \`\!=\` comparison for each element of  \`self\` and \`rhs\`](./u64vec2/cmpne.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./u64vec2/div.md) |
| `div-1(arg0, arg1)` | [No Documentation 🚧](./u64vec2/div-1.md) |
| `div-2(arg0, arg1)` | [No Documentation 🚧](./u64vec2/div-2.md) |
| `dot(_self, rhs)` | [ Computes the dot product of \`self\` and \`rhs\`\.](./u64vec2/dot.md) |
| `dot_into_vec(_self, rhs)` | [ Returns a vector where every component is the dot product of \`self\` and \`rhs\`\.](./u64vec2/dot_into_vec.md) |
| `element_product(_self)` | [ Returns the product of all elements of \`self\`\.  In other words, this computes \`self\.x \* self\.y \* \.\.](./u64vec2/element_product.md) |
| `element_sum(_self)` | [ Returns the sum of all elements of \`self\`\.  In other words, this computes \`self\.x \+ self\.y \+ \.\.\`\.](./u64vec2/element_sum.md) |
| `eq(_self, other)` | [No Documentation 🚧](./u64vec2/eq.md) |
| `extend(_self, z)` | [ Creates a 3D vector from \`self\` and the given \`z\` value\.](./u64vec2/extend.md) |
| `from_array(a)` | [ Creates a new vector from an array\.](./u64vec2/from_array.md) |
| `length_squared(_self)` | [ Computes the squared length of \`self\`\.](./u64vec2/length_squared.md) |
| `max(_self, rhs)` | [ Returns a vector containing the maximum values for each element of \`self\` and \`rhs\`\.  In other word](./u64vec2/max.md) |
| `max_element(_self)` | [ Returns the horizontal maximum of \`self\`\.  In other words this computes \`max\(x, y, \.\.\)\`\.](./u64vec2/max_element.md) |
| `min(_self, rhs)` | [ Returns a vector containing the minimum values for each element of \`self\` and \`rhs\`\.  In other word](./u64vec2/min.md) |
| `min_element(_self)` | [ Returns the horizontal minimum of \`self\`\.  In other words this computes \`min\(x, y, \.\.\)\`\.](./u64vec2/min_element.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./u64vec2/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./u64vec2/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./u64vec2/mul-2.md) |
| `new(x, y)` | [ Creates a new vector\.](./u64vec2/new.md) |
| `rem(_self, rhs)` | [No Documentation 🚧](./u64vec2/rem.md) |
| `rem-1(arg0, arg1)` | [No Documentation 🚧](./u64vec2/rem-1.md) |
| `rem-2(arg0, arg1)` | [No Documentation 🚧](./u64vec2/rem-2.md) |
| `saturating_add(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and \`rhs\`\.  In other words this compu](./u64vec2/saturating_add.md) |
| `saturating_add_signed(_self, rhs)` | [ Returns a vector containing the saturating addition of \`self\` and signed vector \`rhs\`\.  In other wo](./u64vec2/saturating_add_signed.md) |
| `saturating_div(_self, rhs)` | [ Returns a vector containing the saturating division of \`self\` and \`rhs\`\.  In other words this compu](./u64vec2/saturating_div.md) |
| `saturating_mul(_self, rhs)` | [ Returns a vector containing the saturating multiplication of \`self\` and \`rhs\`\.  In other words this](./u64vec2/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Returns a vector containing the saturating subtraction of \`self\` and \`rhs\`\.  In other words this co](./u64vec2/saturating_sub.md) |
| `select(mask, if_true, if_false)` | [ Creates a vector from the elements in \`if\_true\` and \`if\_false\`, selecting which to use  for each el](./u64vec2/select.md) |
| `splat(v)` | [ Creates a vector with all elements set to \`v\`\.](./u64vec2/splat.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./u64vec2/sub.md) |
| `sub-1(arg0, arg1)` | [No Documentation 🚧](./u64vec2/sub-1.md) |
| `sub-2(arg0, arg1)` | [No Documentation 🚧](./u64vec2/sub-2.md) |
| `to_array(_self)` | [ \`\[x, y\]\`](./u64vec2/to_array.md) |
| `with_x(_self, x)` | [ Creates a 2D vector from \`self\` with the given value of \`x\`\.](./u64vec2/with_x.md) |
| `with_y(_self, y)` | [ Creates a 2D vector from \`self\` with the given value of \`y\`\.](./u64vec2/with_y.md) |
| `wrapping_add(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and \`rhs\`\.  In other words this compute](./u64vec2/wrapping_add.md) |
| `wrapping_add_signed(_self, rhs)` | [ Returns a vector containing the wrapping addition of \`self\` and signed vector \`rhs\`\.  In other word](./u64vec2/wrapping_add_signed.md) |
| `wrapping_div(_self, rhs)` | [ Returns a vector containing the wrapping division of \`self\` and \`rhs\`\.  In other words this compute](./u64vec2/wrapping_div.md) |
| `wrapping_mul(_self, rhs)` | [ Returns a vector containing the wrapping multiplication of \`self\` and \`rhs\`\.  In other words this c](./u64vec2/wrapping_mul.md) |
| `wrapping_sub(_self, rhs)` | [ Returns a vector containing the wrapping subtraction of \`self\` and \`rhs\`\.  In other words this comp](./u64vec2/wrapping_sub.md) |