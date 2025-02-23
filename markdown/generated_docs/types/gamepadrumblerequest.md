# GamepadRumbleRequest

### Add

- **duration** : bevy\_utils::Duration
- **intensity** : bevy\_input::gamepad::GamepadRumbleIntensity
- **gamepad** : bevy\_ecs::entity::Entity

### Stop

- **gamepad** : bevy\_ecs::entity::Entity

## Description

>  An event that controls force-feedback rumbling of a [`Gamepad`] [`entity`](Entity).
> 
>  # Notes
> 
>  Does nothing if the gamepad or platform does not support rumble.
> 
>  # Example
> 
>  ```
>  # use bevy_input::gamepad::{Gamepad, GamepadRumbleRequest, GamepadRumbleIntensity};
>  # use bevy_ecs::prelude::{EventWriter, Res, Query, Entity, With};
>  # use bevy_utils::Duration;
>  fn rumble_gamepad_system(
>      mut rumble_requests: EventWriter<GamepadRumbleRequest>,
>      gamepads: Query<Entity, With<Gamepad>>,
>  ) {
>      for entity in gamepads.iter() {
>          rumble_requests.send(GamepadRumbleRequest::Add {
>              gamepad: entity,
>              intensity: GamepadRumbleIntensity::MAX,
>              duration: Duration::from_secs_f32(0.5),
>          });
>      }
>  }
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./gamepadrumblerequest/clone.md) |
| `gamepad(_self)` | [ Get the \[\`Entity\`\] associated with this request\.](./gamepadrumblerequest/gamepad.md) |