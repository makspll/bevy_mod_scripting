# perspective\_lh

>  Creates a left-handed perspective projection matrix with `[0,1]` depth range.
>  Useful to map the standard left-handed coordinate system into what WebGPU/Metal/Direct3D expect.
>  # Panics
>  Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
>  enabled.

#### Arguments

- **fov\_y\_radians** : `f32` \- No Documentation 🚧
- **aspect\_ratio** : `f32` \- No Documentation 🚧
- **z\_near** : `f32` \- No Documentation 🚧
- **z\_far** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Mat4` \- No Documentation 🚧