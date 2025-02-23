# rotate\_towards

>  Rotates towards `rhs` up to `max_angle` (in radians).
>  When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to
>  `self.angle_between(rhs)`, the result will be equal to `rhs`. If `max_angle` is negative,
>  rotates towards the exact opposite of `rhs`. Will not go past the target.
>  Both quaternions must be normalized.
>  # Panics
>  Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

#### Arguments

- **\_self** : `DQuat` \- No Documentation 🚧
- **rhs** : `DQuat` \- No Documentation 🚧
- **max\_angle** : `f64` \- No Documentation 🚧

#### Returns

- **arg0** : `DQuat` \- No Documentation 🚧