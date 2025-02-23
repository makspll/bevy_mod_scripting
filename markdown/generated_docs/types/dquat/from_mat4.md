# from\_mat4

>  Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix.
>  Note if the upper 3x3 matrix contain scales, shears, or other non-rotation transformations
>  then the resulting quaternion will be ill-defined.
>  # Panics
>  Will panic if any column of the upper 3x3 rotation matrix is not normalized when
>  `glam_assert` is enabled.

#### Arguments

- **mat** : `DMat4` \- No Documentation 🚧

#### Returns

- **arg0** : `DQuat` \- No Documentation 🚧