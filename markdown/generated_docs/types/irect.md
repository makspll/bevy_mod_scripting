# IRect

### IRect

- **min** : glam::IVec2
- **max** : glam::IVec2

## Description

>  A rectangle defined by two opposite corners.
> 
>  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
>  stored in `IRect::min` and `IRect::max`, respectively. The minimum/maximum invariant
>  must be upheld by the user when directly assigning the fields, otherwise some methods
>  produce invalid results. It is generally recommended to use one of the constructor
>  methods instead, which will ensure this invariant is met, unless you already have
>  the minimum and maximum corners.

## Functions

| Function | Summary |
| --- | --- |
| `as_rect(_self)` | [ Returns self as \[\`Rect\`\] \(f32\)](./irect/as_rect.md) |
| `as_urect(_self)` | [ Returns self as \[\`URect\`\] \(u32\)](./irect/as_urect.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./irect/assert_receiver_is_total_eq.md) |
| `center(_self)` | [ The center point of the rectangle\.  \# Rounding Behavior  If the \(min \+ max\) contains odd numbers th](./irect/center.md) |
| `clone(_self)` | [No Documentation 🚧](./irect/clone.md) |
| `contains(_self, point)` | [ Check if a point lies within this rectangle, inclusive of its edges\.  \# Examples  \`\`\`  \# use bevy\_math::IRect;  let r = IRect::new\(0, 0, 5, 1\); // w=5 h=1  assert\!\(r\.contains\(r\.center\(\)\)\);  assert\!\(r\.contains\(r\.min\)\);  assert\!\(r\.contains\(r\.max\)\);  \`\`\`](./irect/contains.md) |
| `eq(_self, other)` | [No Documentation 🚧](./irect/eq.md) |
| `from_center_half_size(origin, half_size)` | [ Create a new rectangle from its center and half\-size\.  \# Panics  This method panics if any of the c](./irect/from_center_half_size.md) |
| `from_center_size(origin, size)` | [ Create a new rectangle from its center and size\.  \# Rounding Behavior  If the size contains odd num](./irect/from_center_size.md) |
| `from_corners(p0, p1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./irect/from_corners.md) |
| `half_size(_self)` | [ Rectangle half\-size\.  \# Rounding Behavior  If the full size contains odd numbers they will be round](./irect/half_size.md) |
| `height(_self)` | [ Rectangle height \(max\.y \- min\.y\)\.  \# Examples  \`\`\`  \# use bevy\_math::IRect;  let r = IRect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.height\(\), 1\);  \`](./irect/height.md) |
| `inflate(_self, expansion)` | [ Create a new rectangle by expanding it evenly on all sides\.  A positive expansion value produces a ](./irect/inflate.md) |
| `intersect(_self, other)` | [ Build a new rectangle formed of the intersection of this rectangle and another rectangle\.  The inte](./irect/intersect.md) |
| `is_empty(_self)` | [ Check if the rectangle is empty\.  \# Examples  \`\`\`  \# use bevy\_math::\{IRect, IVec2\};  let r = IRect::from\_corners\(IVec2::ZERO, IVec2::new\(0, 1\)\); // w=0 h=1  assert\!\(r\.is\_empty\(\)\);  \`\`\`](./irect/is_empty.md) |
| `new(x0, y0, x1, y1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./irect/new.md) |
| `size(_self)` | [ Rectangle size\.  \# Examples  \`\`\`  \# use bevy\_math::\{IRect, IVec2\};  let r = IRect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.size\(\), IVec2::new\(5, 1\)\);  \`](./irect/size.md) |
| `union(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and another rectangle\.  The union is th](./irect/union.md) |
| `union_point(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and a point\.  The union is the smallest](./irect/union_point.md) |
| `width(_self)` | [ Rectangle width \(max\.x \- min\.x\)\.  \# Examples  \`\`\`  \# use bevy\_math::IRect;  let r = IRect::new\(0, 0, 5, 1\); // w=5 h=1  assert\_eq\!\(r\.width\(\), 5\);  \`](./irect/width.md) |