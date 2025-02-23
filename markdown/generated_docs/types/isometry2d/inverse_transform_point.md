# inverse\_transform\_point

>  Transform a point by rotating and translating it using the inverse of this isometry.
>  This is more efficient than `iso.inverse().transform_point(point)` for one-shot cases.
>  If the same isometry is used multiple times, it is more efficient to instead compute
>  the inverse once and use that for each transformation.

#### Arguments

- **\_self** : `Isometry2d` \- No Documentation 🚧
- **point** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec2` \- No Documentation 🚧