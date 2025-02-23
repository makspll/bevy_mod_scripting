# reparented\_to

>  Returns the [`Transform`] `self` would have if it was a child of an entity
>  with the `parent` [`GlobalTransform`].
>  This is useful if you want to "reparent" an [`Entity`](bevy_ecs::entity::Entity).
>  Say you have an entity `e1` that you want to turn into a child of `e2`,
>  but you want `e1` to keep the same global transform, even after re-parenting. You would use:
>  ```
>  # use bevy_transform::prelude::{GlobalTransform, Transform};
>  # use bevy_ecs::prelude::{Entity, Query, Component, Commands};
>  # use bevy_hierarchy::{prelude::Parent, BuildChildren};
>  #[derive(Component)]
>  struct ToReparent {
>      new_parent: Entity,
>  }
>  fn reparent_system(
>      mut commands: Commands,
>      mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,
>      transforms: Query<&GlobalTransform>,
>  ) {
>      for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {
>          if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {
>              *transform = initial.reparented_to(parent_transform);
>              commands.entity(entity)
>                  .remove::<ToReparent>()
>                  .set_parent(to_reparent.new_parent);
>          }
>      }
>  }
>  ```
>  The transform is expected to be non-degenerate and without shearing, or the output
>  will be invalid.

#### Arguments

- **\_self** : `GlobalTransform` \- No Documentation 🚧
- **parent** : `GlobalTransform` \- No Documentation 🚧

#### Returns

- **arg0** : `Transform` \- No Documentation 🚧