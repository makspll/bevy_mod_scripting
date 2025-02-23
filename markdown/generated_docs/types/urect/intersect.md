# intersect

>  Build a new rectangle formed of the intersection of this rectangle and another rectangle.
>  The intersection is the largest rectangle enclosed in both rectangles. If the intersection
>  is empty, this method returns an empty rectangle ([`URect::is_empty()`] returns `true`), but
>  the actual values of [`URect::min`] and [`URect::max`] are implementation-dependent.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r1 = URect::new(0, 0, 2, 2); // w=2 h=2
>  let r2 = URect::new(1, 1, 3, 3); // w=2 h=2
>  let r = r1.intersect(r2);
>  assert_eq!(r.min, UVec2::new(1, 1));
>  assert_eq!(r.max, UVec2::new(2, 2));
>  ```

#### Arguments

- **\_self** : `URect` \- No Documentation 🚧
- **other** : `URect` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧