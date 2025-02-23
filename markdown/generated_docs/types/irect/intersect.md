# intersect

>  Build a new rectangle formed of the intersection of this rectangle and another rectangle.
>  The intersection is the largest rectangle enclosed in both rectangles. If the intersection
>  is empty, this method returns an empty rectangle ([`IRect::is_empty()`] returns `true`), but
>  the actual values of [`IRect::min`] and [`IRect::max`] are implementation-dependent.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1
>  let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4
>  let r = r1.intersect(r2);
>  assert_eq!(r.min, IVec2::new(1, 0));
>  assert_eq!(r.max, IVec2::new(3, 1));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧
- **other** : `IRect` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧