# from\_center\_size

>  Create a new rectangle from its center and size.
>  # Rounding Behavior
>  If the size contains odd numbers they will be rounded down to the nearest whole number.
>  # Panics
>  This method panics if any of the components of the size is negative or if `origin - (size / 2)` results in any negatives.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r = URect::from_center_size(UVec2::ONE, UVec2::splat(2)); // w=2 h=2
>  assert_eq!(r.min, UVec2::splat(0));
>  assert_eq!(r.max, UVec2::splat(2));
>  ```

#### Arguments

- **origin** : `UVec2` \- No Documentation 🚧
- **size** : `UVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧