# union\_point

>  Build a new rectangle formed of the union of this rectangle and a point.
>  The union is the smallest rectangle enclosing both the rectangle and the point. If the
>  point is already inside the rectangle, this method returns a copy of the rectangle.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r = IRect::new(0, 0, 5, 1); // w=5 h=1
>  let u = r.union_point(IVec2::new(3, 6));
>  assert_eq!(u.min, IVec2::ZERO);
>  assert_eq!(u.max, IVec2::new(5, 6));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧
- **other** : `IVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧