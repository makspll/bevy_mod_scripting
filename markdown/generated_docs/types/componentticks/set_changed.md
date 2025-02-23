# set\_changed

>  Manually sets the change tick.
>  This is normally done automatically via the [`DerefMut`](std::ops::DerefMut) implementation
>  on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.
>  However, components and resources that make use of interior mutability might require manual updates.
>  # Example
>  ```no_run
>  # use bevy_ecs::{world::World, component::ComponentTicks};
>  let world: World = unimplemented!();
>  let component_ticks: ComponentTicks = unimplemented!();
>  component_ticks.set_changed(world.read_change_tick());
>  ```

#### Arguments

- **\_self** : `ComponentTicks` \- No Documentation 🚧
- **change\_tick** : `Tick` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧