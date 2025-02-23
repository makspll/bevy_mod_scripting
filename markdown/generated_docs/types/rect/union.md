# union

>  Build a new rectangle formed of the union of this rectangle and another rectangle.
>  The union is the smallest rectangle enclosing both rectangles.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
>  let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
>  let r = r1.union(r2);
>  assert!(r.min.abs_diff_eq(Vec2::new(0., -1.), 1e-5));
>  assert!(r.max.abs_diff_eq(Vec2::new(5., 3.), 1e-5));
>  ```

#### Arguments

- **\_self** : `Rect` \- No Documentation 🚧
- **other** : `Rect` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧