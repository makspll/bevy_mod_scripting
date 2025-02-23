# Children

### Children

1. smallvec::SmallVec<\[bevy\_ecs::entity::Entity; 8\]>

## Description

>  Contains references to the child entities of this entity.
> 
>  Each child must contain a [`Parent`] component that points back to this entity.
>  This component rarely needs to be created manually,
>  consider using higher level utilities like [`BuildChildren::with_children`]
>  which are safer and easier to use.
> 
>  See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].
> 
>  [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt
>  [`Query`]: bevy_ecs::system::Query
>  [`Parent`]: crate::components::parent::Parent
>  [`BuildChildren::with_children`]: crate::child_builder::BuildChildren::with_children

## Functions

| Function | Summary |
| --- | --- |
| `swap(_self, a_index, b_index)` | [ Swaps the child at \`a\_index\` with the child at \`b\_index\`\.](./children/swap.md) |