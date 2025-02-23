# URect

### URect

- **min** : glam::UVec2
- **max** : glam::UVec2

## Description

>  A rectangle defined by two opposite corners.
> 
>  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
>  stored in `URect::min` and `URect::max`, respectively. The minimum/maximum invariant
>  must be upheld by the user when directly assigning the fields, otherwise some methods
>  produce invalid results. It is generally recommended to use one of the constructor
>  methods instead, which will ensure this invariant is met, unless you already have
>  the minimum and maximum corners.

## Functions

| Function | Summary |
| --- | --- |
| `as_irect(_self)` | [ Returns self as \[\`IRect\`\] \(i32\)](./urect/as_irect.md) |
| `as_rect(_self)` | [ Returns self as \[\`Rect\`\] \(f32\)](./urect/as_rect.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./urect/assert_receiver_is_total_eq.md) |
| `center(_self)` | [ The center point of the rectangle\.  \# Rounding Behavior  If the \(min \+ max\) contains odd numbers th](./urect/center.md) |
| `clone(_self)` | [No Documentation 🚧](./urect/clone.md) |
| `contains(_self, point)` | [ Check if a point lies within this rectangle, inclusive of its edges\.  \# Examples  \`\`\`  \# use bevy\_math::URect;  let r = URect::new\(0, 0, 5, 1\); // w=5 h=1  assert\!\(r\.contains\(r\.center\(\)\)\);  assert\!\(r\.contains\(r\.min\)\);  assert\!\(r\.contains\(r\.max\)\);  \`\`\`](./urect/contains.md) |
| `eq(_self, other)` | [No Documentation 🚧](./urect/eq.md) |
| `from_center_half_size(origin, half_size)` | [ Create a new rectangle from its center and half\-size\.  \# Panics  This method panics if any of the c](./urect/from_center_half_size.md) |
| `from_center_size(origin, size)` | [ Create a new rectangle from its center and size\.  \# Rounding Behavior  If the size contains odd num](./urect/from_center_size.md) |
| `from_corners(p0, p1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./urect/from_corners.md) |
| `half_size(_self)` | [ Rectangle half\-size\.  \# Rounding Behavior  If the full size contains odd numbers they will be round](./urect/half_size.md) |
| `height(_self)` | [ Rectangle height \(max\.y \- min\.y\)\.  \# Examples  \`\`\`  \# use bevy\_math::URect;  let r = URect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.height\(\), 1\);  \`](./urect/height.md) |
| `inflate(_self, expansion)` | [ Create a new rectangle by expanding it evenly on all sides\.  A positive expansion value produces a ](./urect/inflate.md) |
| `intersect(_self, other)` | [ Build a new rectangle formed of the intersection of this rectangle and another rectangle\.  The inte](./urect/intersect.md) |
| `is_empty(_self)` | [ Check if the rectangle is empty\.  \# Examples  \`\`\`  \# use bevy\_math::\{URect, UVec2\};  let r = URect::from\_corners\(UVec2::ZERO, UVec2::new\(0, 1\)\); // w=0 h=1  assert\!\(r\.is\_empty\(\)\);  \`\`\`](./urect/is_empty.md) |
| `new(x0, y0, x1, y1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./urect/new.md) |
| `size(_self)` | [ Rectangle size\.  \# Examples  \`\`\`  \# use bevy\_math::\{URect, UVec2\};  let r = URect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.size\(\), UVec2::new\(5, 1\)\);  \`](./urect/size.md) |
| `union(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and another rectangle\.  The union is th](./urect/union.md) |
| `union_point(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and a point\.  The union is the smallest](./urect/union_point.md) |
| `width(_self)` | [ Rectangle width \(max\.x \- min\.x\)\.  \# Examples  \`\`\`  \# use bevy\_math::URect;  let r = URect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.width\(\), 5\);  \`](./urect/width.md) |