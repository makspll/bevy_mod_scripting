# from\_rotation\_arc\_colinear

>  Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
>  that the resulting quaternion will rotate `from` so that it is colinear with `to`.
>  The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
>  degrees.
>  The inputs must be unit vectors.
>  `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
>  # Panics
>  Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

#### Arguments

- **from** : `DVec3` \- No Documentation 🚧
- **to** : `DVec3` \- No Documentation 🚧

#### Returns

- **arg0** : `DQuat` \- No Documentation 🚧