# from\_mat3

>  Creates a quaternion from a 3x3 rotation matrix.
>  Note if the input matrix contain scales, shears, or other non-rotation transformations then
>  the resulting quaternion will be ill-defined.
>  # Panics
>  Will panic if any input matrix column is not normalized when `glam_assert` is enabled.

#### Arguments

- **mat** : `DMat3` \- No Documentation 🚧

#### Returns

- **arg0** : `DQuat` \- No Documentation 🚧