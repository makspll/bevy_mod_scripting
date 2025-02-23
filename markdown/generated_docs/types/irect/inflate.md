# inflate

>  Create a new rectangle by expanding it evenly on all sides.
>  A positive expansion value produces a larger rectangle,
>  while a negative expansion value produces a smaller rectangle.
>  If this would result in zero or negative width or height, [`IRect::EMPTY`] is returned instead.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::new(0, 0, 5, 1); // w=5 h=1
>  let r2 = r.inflate(3); // w=11 h=7
>  assert_eq!(r2.min, IVec2::splat(-3));
>  assert_eq!(r2.max, IVec2::new(8, 4));
>  let r = IRect::new(0, -1, 4, 3); // w=4 h=4
>  let r2 = r.inflate(-1); // w=2 h=2
>  assert_eq!(r2.min, IVec2::new(1, 0));
>  assert_eq!(r2.max, IVec2::new(3, 2));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧
- **expansion** : `i32` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧