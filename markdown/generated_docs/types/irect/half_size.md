# half\_size

>  Rectangle half-size.
>  # Rounding Behavior
>  If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::new(0, 0, 4, 3); // w=4 h=3
>  assert_eq!(r.half_size(), IVec2::new(2, 1));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧

#### Returns

- **arg0** : `IVec2` \- No Documentation 🚧