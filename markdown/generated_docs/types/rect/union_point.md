# union\_point

>  Build a new rectangle formed of the union of this rectangle and a point.
>  The union is the smallest rectangle enclosing both the rectangle and the point. If the
>  point is already inside the rectangle, this method returns a copy of the rectangle.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
>  let u = r.union_point(Vec2::new(3., 6.));
>  assert!(u.min.abs_diff_eq(Vec2::ZERO, 1e-5));
>  assert!(u.max.abs_diff_eq(Vec2::new(5., 6.), 1e-5));
>  ```

#### Arguments

- **\_self** : `Rect` \- No Documentation 🚧
- **other** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧