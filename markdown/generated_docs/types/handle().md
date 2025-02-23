# Handle<\(\)>

### Strong

1. alloc::sync::Arc<bevy\_asset::handle::StrongHandle>

### Weak

1. bevy\_asset::id::AssetId<\(\)>

## Description

>  A strong or weak handle to a specific [`Asset`]. If a [`Handle`] is [`Handle::Strong`], the [`Asset`] will be kept
>  alive until the [`Handle`] is dropped. If a [`Handle`] is [`Handle::Weak`], it does not necessarily reference a live [`Asset`],
>  nor will it keep assets alive.
> 
>  [`Handle`] can be cloned. If a [`Handle::Strong`] is cloned, the referenced [`Asset`] will not be freed until _all_ instances
>  of the [`Handle`] are dropped.
> 
>  [`Handle::Strong`] also provides access to useful [`Asset`] metadata, such as the [`AssetPath`] (if it exists).

## Functions

