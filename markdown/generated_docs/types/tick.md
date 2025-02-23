# Tick

### Tick

- **tick** : u32

## Description

>  A value that tracks when a system ran relative to other systems.
>  This is used to power change detection.
> 
>  *Note* that a system that hasn't been run yet has a `Tick` of 0.

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./tick/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./tick/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./tick/eq.md) |
| `get(_self)` | [ Gets the value of this change tick\.](./tick/get.md) |
| `is_newer_than(_self, last_run, this_run)` | [ Returns \`true\` if this \`Tick\` occurred since the system's \`last\_run\`\.  \`this\_run\` is the current ti](./tick/is_newer_than.md) |
| `new(tick)` | [ Creates a new \[\`Tick\`\] wrapping the given value\.](./tick/new.md) |
| `set(_self, tick)` | [ Sets the value of this change tick\.](./tick/set.md) |