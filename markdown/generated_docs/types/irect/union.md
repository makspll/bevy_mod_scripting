# union

>  Build a new rectangle formed of the union of this rectangle and another rectangle.
>  The union is the smallest rectangle enclosing both rectangles.
>  # Examples
>  ```
>  # use bevy_math::{IRect, IVec2};
>  let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1
>  let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4
>  let r = r1.union(r2);
>  assert_eq!(r.min, IVec2::new(0, -1));
>  assert_eq!(r.max, IVec2::new(5, 3));
>  ```

#### Arguments

- **\_self** : `IRect` \- No Documentation 🚧
- **other** : `IRect` \- No Documentation 🚧

#### Returns

- **arg0** : `IRect` \- No Documentation 🚧