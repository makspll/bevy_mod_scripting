# Parent

### Parent

1. bevy\_ecs::entity::Entity

## Description

>  Holds a reference to the parent entity of this entity.
>  This component should only be present on entities that actually have a parent entity.
> 
>  Parent entity must have this entity stored in its [`Children`] component.
>  It is hard to set up parent/child relationships manually,
>  consider using higher level utilities like [`BuildChildren::with_children`].
> 
>  See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].
> 
>  [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt
>  [`Query`]: bevy_ecs::system::Query
>  [`Children`]: super::children::Children
>  [`BuildChildren::with_children`]: crate::child_builder::BuildChildren::with_children

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./parent/assert_receiver_is_total_eq.md) |
| `eq(_self, other)` | [No Documentation 🚧](./parent/eq.md) |
| `get(_self)` | [ Gets the \[\`Entity\`\] ID of the parent\.](./parent/get.md) |