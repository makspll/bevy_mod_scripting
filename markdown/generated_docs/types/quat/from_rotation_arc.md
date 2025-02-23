# from\_rotation\_arc

>  Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
>  plane spanned by the two vectors.  Will rotate at most 180 degrees.
>  The inputs must be unit vectors.
>  `from_rotation_arc(from, to) * from ≈ to`.
>  For near-singular cases (from≈to and from≈-to) the current implementation
>  is only accurate to about 0.001 (for `f32`).
>  # Panics
>  Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

#### Arguments

- **from** : `Vec3` \- No Documentation 🚧
- **to** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Quat` \- No Documentation 🚧