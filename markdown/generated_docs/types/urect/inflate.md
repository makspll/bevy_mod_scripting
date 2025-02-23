# inflate

>  Create a new rectangle by expanding it evenly on all sides.
>  A positive expansion value produces a larger rectangle,
>  while a negative expansion value produces a smaller rectangle.
>  If this would result in zero width or height, [`URect::EMPTY`] is returned instead.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r = URect::new(4, 4, 6, 6); // w=2 h=2
>  let r2 = r.inflate(1); // w=4 h=4
>  assert_eq!(r2.min, UVec2::splat(3));
>  assert_eq!(r2.max, UVec2::splat(7));
>  let r = URect::new(4, 4, 8, 8); // w=4 h=4
>  let r2 = r.inflate(-1); // w=2 h=2
>  assert_eq!(r2.min, UVec2::splat(5));
>  assert_eq!(r2.max, UVec2::splat(7));
>  ```

#### Arguments

- **\_self** : `URect` \- No Documentation 🚧
- **expansion** : `i32` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧