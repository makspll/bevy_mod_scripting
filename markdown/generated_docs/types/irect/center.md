# center

>  The center point of the rectangle.
>  # Rounding Behavior
>  If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::new(0, 0, 5, 2); // w=5 h=2
>  assert_eq!(r.center(), IVec2::new(2, 1));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧

#### Returns

- **arg0** : `IVec2` \- No Documentation 🚧