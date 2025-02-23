# transform\_vector2

>  Rotates the given 2D vector.
>  This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
>  This method assumes that `self` contains a valid affine transform.
>  # Panics
>  Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

#### Arguments

- **\_self** : `DMat3` \- No Documentation 🚧
- **rhs** : `DVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `DVec2` \- No Documentation 🚧