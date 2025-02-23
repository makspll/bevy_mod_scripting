# to\_euler

>  Extract Euler angles with the given Euler rotation order.
>  Note if the input matrix contains scales, shears, or other non-rotation transformations then
>  the resulting Euler angles will be ill-defined.
>  # Panics
>  Will panic if any input matrix column is not normalized when `glam_assert` is enabled.

#### Arguments

- **\_self** : `Mat3` \- No Documentation 🚧
- **order** : `EulerRot` \- No Documentation 🚧

#### Returns

- **arg0** : `(f32, f32, f32)` \- No Documentation 🚧