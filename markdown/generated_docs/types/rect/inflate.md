# inflate

>  Create a new rectangle by expanding it evenly on all sides.
>  A positive expansion value produces a larger rectangle,
>  while a negative expansion value produces a smaller rectangle.
>  If this would result in zero or negative width or height, [`Rect::EMPTY`] is returned instead.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
>  let r2 = r.inflate(3.); // w=11 h=7
>  assert!(r2.min.abs_diff_eq(Vec2::splat(-3.), 1e-5));
>  assert!(r2.max.abs_diff_eq(Vec2::new(8., 4.), 1e-5));
>  let r = Rect::new(0., -1., 6., 7.); // w=6 h=8
>  let r2 = r.inflate(-2.); // w=11 h=7
>  assert!(r2.min.abs_diff_eq(Vec2::new(2., 1.), 1e-5));
>  assert!(r2.max.abs_diff_eq(Vec2::new(4., 5.), 1e-5));
>  ```

#### Arguments

- **\_self** : `Rect` \- No Documentation 🚧
- **expansion** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧