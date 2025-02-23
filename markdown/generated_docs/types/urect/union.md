# union

>  Build a new rectangle formed of the union of this rectangle and another rectangle.
>  The union is the smallest rectangle enclosing both rectangles.
>  # Examples
>  ```
>  # use bevy_math::{URect, UVec2};
>  let r1 = URect::new(0, 0, 5, 1); // w=5 h=1
>  let r2 = URect::new(1, 0, 3, 8); // w=2 h=4
>  let r = r1.union(r2);
>  assert_eq!(r.min, UVec2::new(0, 0));
>  assert_eq!(r.max, UVec2::new(5, 8));
>  ```

#### Arguments

- **\_self** : `URect` \- No Documentation 🚧
- **other** : `URect` \- No Documentation 🚧

#### Returns

- **arg0** : `URect` \- No Documentation 🚧