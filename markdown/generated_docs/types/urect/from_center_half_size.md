# from\_center\_half\_size

>  Create a new rectangle from its center and half-size.
>  # Panics
>  This method panics if any of the components of the half-size is negative or if `origin - half_size` results in any negatives.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r = URect::from_center_half_size(UVec2::ONE, UVec2::ONE); // w=2 h=2
>  assert_eq!(r.min, UVec2::splat(0));
>  assert_eq!(r.max, UVec2::splat(2));
>  ```

#### Arguments

- **origin** : `UVec2` \- No Documentation 🚧
- **half\_size** : `UVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧