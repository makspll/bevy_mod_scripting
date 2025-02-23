# Entity

Opaque Type\. 🔒

## Description

>  Lightweight identifier of an [entity](crate::entity).
> 
>  The identifier is implemented using a [generational index]: a combination of an index and a generation.
>  This allows fast insertion after data removal in an array while minimizing loss of spatial locality.
> 
>  These identifiers are only valid on the [`World`] it's sourced from. Attempting to use an `Entity` to
>  fetch entity components or metadata from a different world will either fail or return unexpected results.
> 
>  [generational index]: https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
> 
>  # Stability warning
>  For all intents and purposes, `Entity` should be treated as an opaque identifier. The internal bit
>  representation is liable to change from release to release as are the behaviors or performance
>  characteristics of any of its trait implementations (i.e. `Ord`, `Hash`, etc.). This means that changes in
>  `Entity`'s representation, though made readable through various functions on the type, are not considered
>  breaking changes under [SemVer].
> 
>  In particular, directly serializing with `Serialize` and `Deserialize` make zero guarantee of long
>  term wire format compatibility. Changes in behavior will cause serialized `Entity` values persisted
>  to long term storage (i.e. disk, databases, etc.) will fail to deserialize upon being updated.
> 
>  # Usage
> 
>  This data type is returned by iterating a `Query` that has `Entity` as part of its query fetch type parameter ([learn more]).
>  It can also be obtained by calling [`EntityCommands::id`] or [`EntityWorldMut::id`].
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # #[derive(Component)]
>  # struct SomeComponent;
>  fn setup(mut commands: Commands) {
>      // Calling `spawn` returns `EntityCommands`.
>      let entity = commands.spawn(SomeComponent).id();
>  }
> 
>  fn exclusive_system(world: &mut World) {
>      // Calling `spawn` returns `EntityWorldMut`.
>      let entity = world.spawn(SomeComponent).id();
>  }
>  #
>  # bevy_ecs::system::assert_is_system(setup);
>  # bevy_ecs::system::assert_is_system(exclusive_system);
>  ```
> 
>  It can be used to refer to a specific entity to apply [`EntityCommands`], or to call [`Query::get`] (or similar methods) to access its components.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  #
>  # #[derive(Component)]
>  # struct Expired;
>  #
>  fn dispose_expired_food(mut commands: Commands, query: Query<Entity, With<Expired>>) {
>      for food_entity in &query {
>          commands.entity(food_entity).despawn();
>      }
>  }
>  #
>  # bevy_ecs::system::assert_is_system(dispose_expired_food);
>  ```
> 
>  [learn more]: crate::system::Query#entity-id-access
>  [`EntityCommands::id`]: crate::system::EntityCommands::id
>  [`EntityWorldMut::id`]: crate::world::EntityWorldMut::id
>  [`EntityCommands`]: crate::system::EntityCommands
>  [`Query::get`]: crate::system::Query::get
>  [`World`]: crate::world::World
>  [SemVer]: https://semver.org/

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./entity/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./entity/eq.md) |
| `from_bits(bits)` | [ Reconstruct an \`Entity\` previously destructured with \[\`Entity::to\_bits\`\]\.  Only useful when applied to results from \`to\_bits\` in the same instance of an application\.  \# Panics  This method will likely panic if given \`u64\` values that did not come from \[\`Entity::to\_bits\`\]](./entity/from_bits.md) |
| `from_raw(index)` | [ Creates a new entity ID with the specified \`index\` and a generation of 1\.  \# Note  Spawning a speci](./entity/from_raw.md) |
| `generation(_self)` | [ Returns the generation of this Entity's index\. The generation is incremented each time an  entity w](./entity/generation.md) |
| `index(_self)` | [ Return a transiently unique identifier\.  No two simultaneously\-live entities share the same index, ](./entity/index.md) |
| `to_bits(_self)` | [ Convert to a form convenient for passing outside of rust\.  Only useful for identifying entities wit](./entity/to_bits.md) |