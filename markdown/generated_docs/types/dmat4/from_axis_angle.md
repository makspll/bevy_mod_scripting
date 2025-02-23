# from\_axis\_angle

>  Creates an affine transformation matrix containing a 3D rotation around a normalized
>  rotation `axis` of `angle` (in radians).
>  The resulting matrix can be used to transform 3D points and vectors. See
>  [`Self::transform_point3()`] and [`Self::transform_vector3()`].
>  # Panics
>  Will panic if `axis` is not normalized when `glam_assert` is enabled.

#### Arguments

- **axis** : `DVec3` \- No Documentation 🚧
- **angle** : `f64` \- No Documentation 🚧

#### Returns

- **arg0** : `DMat4` \- No Documentation 🚧