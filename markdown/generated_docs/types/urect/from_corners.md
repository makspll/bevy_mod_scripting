# from\_corners

>  Create a new rectangle from two corner points.
>  The two points do not need to be the minimum and/or maximum corners.
>  They only need to be two opposite corners.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  // Unit rect from [0,0] to [1,1]
>  let r = URect::from_corners(UVec2::ZERO, UVec2::ONE); // w=1 h=1
>  // Same; the points do not need to be ordered
>  let r = URect::from_corners(UVec2::ONE, UVec2::ZERO); // w=1 h=1
>  ```

#### Arguments

- **p0** : `UVec2` \- No Documentation 🚧
- **p1** : `UVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧