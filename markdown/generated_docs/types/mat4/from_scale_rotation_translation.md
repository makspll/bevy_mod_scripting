# from\_scale\_rotation\_translation

>  Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
>  `translation`.
>  The resulting matrix can be used to transform 3D points and vectors. See
>  [`Self::transform_point3()`] and [`Self::transform_vector3()`].
>  # Panics
>  Will panic if `rotation` is not normalized when `glam_assert` is enabled.

#### Arguments

- **scale** : `Vec3` \- No Documentation 🚧
- **rotation** : `Quat` \- No Documentation 🚧
- **translation** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Mat4` \- No Documentation 🚧