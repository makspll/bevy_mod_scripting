# from\_scale\_rotation\_translation

>  Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
>  `translation`.
>  The resulting matrix can be used to transform 3D points and vectors. See
>  [`Self::transform_point3()`] and [`Self::transform_vector3()`].
>  # Panics
>  Will panic if `rotation` is not normalized when `glam_assert` is enabled.

#### Arguments

- **scale** : `DVec3` \- No Documentation 🚧
- **rotation** : `DQuat` \- No Documentation 🚧
- **translation** : `DVec3` \- No Documentation 🚧

#### Returns

- **arg0** : `DMat4` \- No Documentation 🚧