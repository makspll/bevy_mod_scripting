# isometry\_into\_xy

>  Computes an [`Isometry3d`] which transforms points from the plane in 3D space with the given
>  `origin` to the XY-plane.
>  ## Guarantees
>  * the transformation is a [congruence] meaning it will preserve all distances and angles of
>    the transformed geometry
>  * uses the least rotation possible to transform the geometry
>  * if two geometries are transformed with the same isometry, then the relations between
>    them, like distances, are also preserved
>  * compared to projections, the transformation is lossless (up to floating point errors)
>    reversible
>  ## Non-Guarantees
>  * the rotation used is generally not unique
>  * the orientation of the transformed geometry in the XY plane might be arbitrary, to
>    enforce some kind of alignment the user has to use an extra transformation ontop of this
>    one
>  See [`isometries_xy`] for example usescases.
>  [congruence]: https://en.wikipedia.org/wiki/Congruence_(geometry)
>  [`isometries_xy`]: `InfinitePlane3d::isometries_xy`

#### Arguments

- **\_self** : `InfinitePlane3d` \- No Documentation 🚧
- **origin** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Isometry3d` \- No Documentation 🚧