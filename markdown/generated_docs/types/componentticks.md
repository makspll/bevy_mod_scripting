# ComponentTicks

### ComponentTicks

- **added** : bevy\_ecs::component::Tick
- **changed** : bevy\_ecs::component::Tick

## Description

>  Records when a component or resource was added and when it was last mutably dereferenced (or added).

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./componentticks/clone.md) |
| `is_added(_self, last_run, this_run)` | [ Returns \`true\` if the component or resource was added after the system last ran  \(or the system is ](./componentticks/is_added.md) |
| `is_changed(_self, last_run, this_run)` | [ Returns \`true\` if the component or resource was added or mutably dereferenced after the system last](./componentticks/is_changed.md) |
| `new(change_tick)` | [ Creates a new instance with the same change tick for \`added\` and \`changed\`\.](./componentticks/new.md) |
| `set_changed(_self, change_tick)` | [ Manually sets the change tick\.  This is normally done automatically via the \[\`DerefMut\`\]\(std::ops::DerefMut\) implementation  on \[\`Mut<T>\`\]\(crate::change\_detection::Mut\)](./componentticks/set_changed.md) |