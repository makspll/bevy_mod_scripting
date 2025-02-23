# ComponentId

### ComponentId

1. usize

## Description

>  A value which uniquely identifies the type of a [`Component`] or [`Resource`] within a
>  [`World`].
> 
>  Each time a new `Component` type is registered within a `World` using
>  e.g. [`World::register_component`] or [`World::register_component_with_descriptor`]
>  or a Resource with e.g. [`World::init_resource`],
>  a corresponding `ComponentId` is created to track it.
> 
>  While the distinction between `ComponentId` and [`TypeId`] may seem superficial, breaking them
>  into two separate but related concepts allows components to exist outside of Rust's type system.
>  Each Rust type registered as a `Component` will have a corresponding `ComponentId`, but additional
>  `ComponentId`s may exist in a `World` to track components which cannot be
>  represented as Rust types for scripting or other advanced use-cases.
> 
>  A `ComponentId` is tightly coupled to its parent `World`. Attempting to use a `ComponentId` from
>  one `World` to access the metadata of a `Component` in a different `World` is undefined behavior
>  and must not be attempted.
> 
>  Given a type `T` which implements [`Component`], the `ComponentId` for `T` can be retrieved
>  from a `World` using [`World::component_id()`] or via [`Components::component_id()`]. Access
>  to the `ComponentId` for a [`Resource`] is available via [`Components::resource_id()`].

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./componentid/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./componentid/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./componentid/eq.md) |
| `index(_self)` | [ Returns the index of the current component\.](./componentid/index.md) |
| `new(index)` | [ Creates a new \[\`ComponentId\`\]\.  The \`index\` is a unique value associated with each type of component in a given world\.  Usually, this value is taken from a counter incremented for each type of component registered with the world\.](./componentid/new.md) |