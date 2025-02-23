# from\_raw

>  Creates a new entity ID with the specified `index` and a generation of 1.
>  # Note
>  Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
>  [`Commands::spawn`](crate::system::Commands::spawn). This method should generally
>  only be used for sharing entities across apps, and only when they have a scheme
>  worked out to share an index space (which doesn't happen by default).
>  In general, one should not try to synchronize the ECS by attempting to ensure that
>  `Entity` lines up between instances, but instead insert a secondary identifier as
>  a component.

#### Arguments

- **index** : `u32` \- No Documentation 🚧

#### Returns

- **arg0** : `Entity` \- No Documentation 🚧