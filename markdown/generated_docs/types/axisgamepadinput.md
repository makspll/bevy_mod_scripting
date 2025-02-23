# Axis<GamepadInput>

### Axis

- **axis\_data** : bevy\_utils::hashbrown::HashMap<bevy\_input::gamepad::GamepadInput, f32, bevy\_utils::hashbrown::hash\_map::DefaultHashBuilder>

## Description

>  Stores the position data of the input devices of type `T`.
> 
>  The values are stored as `f32`s, using [`Axis::set`].
>  Use [`Axis::get`] to retrieve the value clamped between [`Axis::MIN`] and [`Axis::MAX`]
>  inclusive, or unclamped using [`Axis::get_unclamped`].

## Functions

