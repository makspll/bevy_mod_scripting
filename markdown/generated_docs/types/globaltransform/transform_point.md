# transform\_point

>  Transforms the given point from local space to global space, applying shear, scale, rotation and translation.
>  It can be used like this:
>  ```
>  # use bevy_transform::prelude::{GlobalTransform};
>  # use bevy_math::prelude::Vec3;
>  let global_transform = GlobalTransform::from_xyz(1., 2., 3.);
>  let local_point = Vec3::new(1., 2., 3.);
>  let global_point = global_transform.transform_point(local_point);
>  assert_eq!(global_point, Vec3::new(2., 4., 6.));
>  ```
>  ```
>  # use bevy_transform::prelude::{GlobalTransform};
>  # use bevy_math::Vec3;
>  let global_point = Vec3::new(2., 4., 6.);
>  let global_transform = GlobalTransform::from_xyz(1., 2., 3.);
>  let local_point = global_transform.affine().inverse().transform_point3(global_point);
>  assert_eq!(local_point, Vec3::new(1., 2., 3.))
>  ```
>  To apply shear, scale, and rotation *without* applying translation, different functions are available:
>  ```
>  # use bevy_transform::prelude::{GlobalTransform};
>  # use bevy_math::prelude::Vec3;
>  let global_transform = GlobalTransform::from_xyz(1., 2., 3.);
>  let local_direction = Vec3::new(1., 2., 3.);
>  let global_direction = global_transform.affine().transform_vector3(local_direction);
>  assert_eq!(global_direction, Vec3::new(1., 2., 3.));
>  let roundtripped_local_direction = global_transform.affine().inverse().transform_vector3(global_direction);
>  assert_eq!(roundtripped_local_direction, local_direction);
>  ```

#### Arguments

- **\_self** : `GlobalTransform` \- No Documentation 🚧
- **point** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec3` \- No Documentation 🚧