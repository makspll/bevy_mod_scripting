# from\_affine3

>  Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
>  Note if the input affine matrix contain scales, shears, or other non-rotation
>  transformations then the resulting quaternion will be ill-defined.
>  # Panics
>  Will panic if any input affine matrix column is not normalized when `glam_assert` is
>  enabled.

#### Arguments

- **a** : `Affine3A` \- No Documentation 🚧

#### Returns

- **arg0** : `Quat` \- No Documentation 🚧