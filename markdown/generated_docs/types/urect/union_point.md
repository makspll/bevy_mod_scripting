# union\_point

>  Build a new rectangle formed of the union of this rectangle and a point.
>  The union is the smallest rectangle enclosing both the rectangle and the point. If the
>  point is already inside the rectangle, this method returns a copy of the rectangle.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r = URect::new(0, 0, 5, 1); // w=5 h=1
>  let u = r.union_point(UVec2::new(3, 6));
>  assert_eq!(u.min, UVec2::ZERO);
>  assert_eq!(u.max, UVec2::new(5, 6));
>  ```

#### Arguments

- **\_self** : `URect` \- No Documentation 🚧
- **other** : `UVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧