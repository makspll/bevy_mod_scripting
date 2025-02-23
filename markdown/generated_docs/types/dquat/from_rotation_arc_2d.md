# from\_rotation\_arc\_2d

>  Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
>  around the z axis. Will rotate at most 180 degrees.
>  The inputs must be unit vectors.
>  `from_rotation_arc_2d(from, to) * from ≈ to`.
>  For near-singular cases (from≈to and from≈-to) the current implementation
>  is only accurate to about 0.001 (for `f32`).
>  # Panics
>  Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

#### Arguments

- **from** : `DVec2` \- No Documentation 🚧
- **to** : `DVec2` \- No Documentation 🚧

#### Returns

- **arg0** : `DQuat` \- No Documentation 🚧