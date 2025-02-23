# normalize

>  Build a new rectangle from this one with its coordinates expressed
>  relative to `other` in a normalized ([0..1] x [0..1]) coordinate system.
>  # Examples
>  ```
>  # use bevy_math::{Rect, Vec2};
>  let r = Rect::new(2., 3., 4., 6.);
>  let s = Rect::new(0., 0., 10., 10.);
>  let n = r.normalize(s);
>  assert_eq!(n.min.x, 0.2);
>  assert_eq!(n.min.y, 0.3);
>  assert_eq!(n.max.x, 0.4);
>  assert_eq!(n.max.y, 0.6);
>  ```

#### Arguments

- **\_self** : `Rect` \- No Documentation 🚧
- **other** : `Rect` \- No Documentation 🚧

#### Returns

- **arg0** : `Rect` \- No Documentation 🚧