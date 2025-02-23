# from\_center\_size

>  Create a new rectangle from its center and size.
>  # Rounding Behavior
>  If the size contains odd numbers they will be rounded down to the nearest whole number.
>  # Panics
>  This method panics if any of the components of the size is negative.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::from_center_size(IVec2::ZERO, IVec2::new(3, 2)); // w=2 h=2
>  assert_eq!(r.min, IVec2::splat(-1));
>  assert_eq!(r.max, IVec2::splat(1));
>  ```

#### Arguments

- **origin** : `IVec2` \- No Documentation 🚧
- **size** : `IVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧