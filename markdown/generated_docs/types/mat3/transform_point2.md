# transform\_point2

>  Transforms the given 2D vector as a point.
>  This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
>  This method assumes that `self` contains a valid affine transform.
>  # Panics
>  Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

#### Arguments

- **\_self** : `Mat3` \- No Documentation 🚧
- **rhs** : `Vec2` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec2` \- No Documentation 🚧