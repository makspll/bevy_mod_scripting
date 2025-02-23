# transform\_point

>  Transforms the given `point`, applying scale, rotation and translation.
>  If this [`Transform`] has an ancestor entity with a [`Transform`] component,
>  [`Transform::transform_point`] will transform a point in local space into its
>  parent transform's space.
>  If this [`Transform`] does not have a parent, [`Transform::transform_point`] will
>  transform a point in local space into worldspace coordinates.
>  If you always want to transform a point in local space to worldspace, or if you need
>  the inverse transformations, see [`GlobalTransform::transform_point()`].

#### Arguments

- **\_self** : `Transform` \- No Documentation 🚧
- **point** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec3` \- No Documentation 🚧