# perspective\_infinite\_rh

>  Creates an infinite right-handed perspective projection matrix with `[0,1]` depth range.
>  Like `perspective_rh`, but with an infinite value for `z_far`.
>  The result is that points near `z_near` are mapped to depth `0`, and as they move towards infinity the depth approaches `1`.
>  # Panics
>  Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
>  enabled.

#### Arguments

- **fov\_y\_radians** : `f32` \- No Documentation 🚧
- **aspect\_ratio** : `f32` \- No Documentation 🚧
- **z\_near** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Mat4` \- No Documentation 🚧