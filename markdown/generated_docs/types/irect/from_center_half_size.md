# from\_center\_half\_size

>  Create a new rectangle from its center and half-size.
>  # Panics
>  This method panics if any of the components of the half-size is negative.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::from_center_half_size(IVec2::ZERO, IVec2::ONE); // w=2 h=2
>  assert_eq!(r.min, IVec2::splat(-1));
>  assert_eq!(r.max, IVec2::splat(1));
>  ```

#### Arguments

- **origin** : `IVec2` \- No Documentation 🚧
- **half\_size** : `IVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧