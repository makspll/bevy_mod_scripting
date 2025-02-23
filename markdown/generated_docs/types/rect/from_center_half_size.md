# from\_center\_half\_size

>  Create a new rectangle from its center and half-size.
>  # Panics
>  This method panics if any of the components of the half-size is negative.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2
>  assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));
>  assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));
>  ```

#### Arguments

- **origin** : `Vec2` \- No Documentation 🚧
- **half\_size** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧