# TouchInput

### TouchInput

- **phase** : bevy\_input::touch::TouchPhase
- **position** : glam::Vec2
- **window** : bevy\_ecs::entity::Entity
- **force** : core::option::Option<bevy\_input::touch::ForceTouch>
- **id** : u64

## Description

>  A touch input event.
> 
>  ## Logic
> 
>  Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique
>  identifier for the finger is generated. When the finger is lifted, the [`TouchPhase::Ended`]
>  event is generated with the same finger id.
> 
>  After a [`TouchPhase::Started`] event has been emitted, there may be zero or more [`TouchPhase::Moved`]
>  events when the finger is moved or the touch pressure changes.
> 
>  The finger id may be reused by the system after an [`TouchPhase::Ended`] event. The user
>  should assume that a new [`TouchPhase::Started`] event received with the same id has nothing
>  to do with the old finger and is a new finger.
> 
>  A [`TouchPhase::Canceled`] event is emitted when the system has canceled tracking this
>  touch, such as when the window loses focus, or on iOS if the user moves the
>  device against their face.
> 
>  ## Note
> 
>  This event is the translated version of the `WindowEvent::Touch` from the `winit` crate.
>  It is available to the end user and can be used for game logic.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./touchinput/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./touchinput/eq.md) |