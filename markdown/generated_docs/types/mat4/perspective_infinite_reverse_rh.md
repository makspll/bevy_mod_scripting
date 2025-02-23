# perspective\_infinite\_reverse\_rh

>  Creates an infinite reverse right-handed perspective projection matrix with `[0,1]` depth range.
>  Similar to `perspective_infinite_rh`, but maps `Z = z_near` to a depth of `1` and `Z = infinity` to a depth of `0`.
>  # Panics
>  Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

#### Arguments

- **fov\_y\_radians** : `f32` \- No Documentation 🚧
- **aspect\_ratio** : `f32` \- No Documentation 🚧
- **z\_near** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Mat4` \- No Documentation 🚧