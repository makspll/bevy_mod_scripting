# center

>  The center point of the rectangle.
>  # Rounding Behavior
>  If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r = URect::new(0, 0, 4, 2); // w=4 h=2
>  assert_eq!(r.center(), UVec2::new(2, 1));
>  ```

#### Arguments

- **\_self** : `URect` \- No Documentation 🚧

#### Returns

- **arg0** : `UVec2` \- No Documentation 🚧