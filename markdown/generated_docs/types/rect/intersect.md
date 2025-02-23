# intersect

>  Build a new rectangle formed of the intersection of this rectangle and another rectangle.
>  The intersection is the largest rectangle enclosed in both rectangles. If the intersection
>  is empty, this method returns an empty rectangle ([`Rect::is_empty()`] returns `true`), but
>  the actual values of [`Rect::min`] and [`Rect::max`] are implementation-dependent.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
>  let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
>  let r = r1.intersect(r2);
>  assert!(r.min.abs_diff_eq(Vec2::new(1., 0.), 1e-5));
>  assert!(r.max.abs_diff_eq(Vec2::new(3., 1.), 1e-5));
>  ```

#### Arguments

- **\_self** : `Rect` \- No Documentation 🚧
- **other** : `Rect` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧