# from\_center\_size

>  Create a new rectangle from its center and size.
>  # Panics
>  This method panics if any of the components of the size is negative.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
>  assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
>  assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
>  ```

#### Arguments

- **origin** : `Vec2` \- No Documentation 🚧
- **size** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧