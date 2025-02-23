# reject\_from\_normalized

>  Returns the vector rejection of `self` from `rhs`.
>  The vector rejection is the vector perpendicular to the projection of `self` onto
>  `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
>  `rhs` must be normalized.
>  # Panics
>  Will panic if `rhs` is not normalized when `glam_assert` is enabled.

#### Arguments

- **\_self** : `DVec3` \- No Documentation 🚧
- **rhs** : `DVec3` \- No Documentation 🚧

#### Returns

- **arg0** : `DVec3` \- No Documentation 🚧