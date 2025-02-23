# Rect

### Rect

- **min** : glam::Vec2
- **max** : glam::Vec2

## Description

>  A rectangle defined by two opposite corners.
> 
>  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
>  stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant
>  must be upheld by the user when directly assigning the fields, otherwise some methods
>  produce invalid results. It is generally recommended to use one of the constructor
>  methods instead, which will ensure this invariant is met, unless you already have
>  the minimum and maximum corners.

## Functions

| Function | Summary |
| --- | --- |
| `as_irect(_self)` | [ Returns self as \[\`IRect\`\] \(i32\)](./rect/as_irect.md) |
| `as_urect(_self)` | [ Returns self as \[\`URect\`\] \(u32\)](./rect/as_urect.md) |
| `center(_self)` | [ The center point of the rectangle\.  \# Examples  \`\`\`  \# use bevy\_math::\{Rect, Vec2\};  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(r\.center\(\)\.abs\_diff\_eq\(Vec2::new\(2\.5, 0\.5\), 1e\-5\)\);  \`\`\`](./rect/center.md) |
| `clone(_self)` | [No Documentation 🚧](./rect/clone.md) |
| `contains(_self, point)` | [ Check if a point lies within this rectangle, inclusive of its edges\.  \# Examples  \`\`\`  \# use bevy\_math::Rect;  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(r\.contains\(r\.center\(\)\)\);  assert\!\(r\.contains\(r\.min\)\);  assert\!\(r\.contains\(r\.max\)\);  \`\`\`](./rect/contains.md) |
| `eq(_self, other)` | [No Documentation 🚧](./rect/eq.md) |
| `from_center_half_size(origin, half_size)` | [ Create a new rectangle from its center and half\-size\.  \# Panics  This method panics if any of the c](./rect/from_center_half_size.md) |
| `from_center_size(origin, size)` | [ Create a new rectangle from its center and size\.  \# Panics  This method panics if any of the compon](./rect/from_center_size.md) |
| `from_corners(p0, p1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./rect/from_corners.md) |
| `half_size(_self)` | [ Rectangle half\-size\.  \# Examples  \`\`\`  \# use bevy\_math::\{Rect, Vec2\};  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(r\.half\_size\(\)\.abs\_diff\_eq\(Vec2::new\(2\.5, 0\.5\), 1e\-5\)\);  \`](./rect/half_size.md) |
| `height(_self)` | [ Rectangle height \(max\.y \- min\.y\)\.  \# Examples  \`\`\`  \# use bevy\_math::Rect;  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(\(r\.height\(\) \- 1\.\)\.abs\(\) <= 1e\-5\);  \`\`\`](./rect/height.md) |
| `inflate(_self, expansion)` | [ Create a new rectangle by expanding it evenly on all sides\.  A positive expansion value produces a ](./rect/inflate.md) |
| `intersect(_self, other)` | [ Build a new rectangle formed of the intersection of this rectangle and another rectangle\.  The inte](./rect/intersect.md) |
| `is_empty(_self)` | [ Check if the rectangle is empty\.  \# Examples  \`\`\`  \# use bevy\_math::\{Rect, Vec2\};  let r = Rect::from\_corners\(Vec2::ZERO, Vec2::new\(0\., 1\.\)\); // w=0 h=1  assert\!\(r\.is\_empty\(\)\);  \`\`\`](./rect/is_empty.md) |
| `new(x0, y0, x1, y1)` | [ Create a new rectangle from two corner points\.  The two points do not need to be the minimum and/or](./rect/new.md) |
| `normalize(_self, other)` | [ Build a new rectangle from this one with its coordinates expressed  relative to \`other\` in a normal](./rect/normalize.md) |
| `size(_self)` | [ Rectangle size\.  \# Examples  \`\`\`  \# use bevy\_math::\{Rect, Vec2\};  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(r\.size\(\)\.abs\_diff\_eq\(Vec2::new\(5\., 1\.\), 1e\-5\)\);  \`\`\`](./rect/size.md) |
| `union(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and another rectangle\.  The union is th](./rect/union.md) |
| `union_point(_self, other)` | [ Build a new rectangle formed of the union of this rectangle and a point\.  The union is the smallest](./rect/union_point.md) |
| `width(_self)` | [ Rectangle width \(max\.x \- min\.x\)\.  \# Examples  \`\`\`  \# use bevy\_math::Rect;  let r = Rect::new\(0\., 0\., 5\., 1\.\); // w=5 h=1  assert\!\(\(r\.width\(\) \- 5\.\)\.abs\(\) <= 1e\-5\);  \`\`\`](./rect/width.md) |