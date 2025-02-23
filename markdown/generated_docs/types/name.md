# Name

### Name

- **hash** : u64
- **name** : alloc::borrow::Cow<str>

## Description

>  Component used to identify an entity. Stores a hash for faster comparisons.
> 
>  The hash is eagerly re-computed upon each update to the name.
> 
>  [`Name`] should not be treated as a globally unique identifier for entities,
>  as multiple entities can have the same name.  [`Entity`] should be
>  used instead as the default unique identifier.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./name/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./name/eq.md) |