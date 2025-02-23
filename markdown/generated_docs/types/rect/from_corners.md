# from\_corners

>  Create a new rectangle from two corner points.
>  The two points do not need to be the minimum and/or maximum corners.
>  They only need to be two opposite corners.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  // Unit rect from [0,0] to [1,1]
>  let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
>  // Same; the points do not need to be ordered
>  let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
>  ```

#### Arguments

- **p0** : `Vec2` \- No Documentation 🚧
- **p1** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧