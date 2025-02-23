# Real

### Real

- **startup** : bevy\_utils::Instant
- **first\_update** : core::option::Option<bevy\_utils::Instant>
- **last\_update** : core::option::Option<bevy\_utils::Instant>

## Description

>  Real time clock representing elapsed wall clock time.
> 
>  A specialization of the [`Time`] structure. **For method documentation, see
>  [`Time<Real>#impl-Time<Real>`].**
> 
>  It is automatically inserted as a resource by
>  [`TimePlugin`](crate::TimePlugin) and updated with time instants according
>  to [`TimeUpdateStrategy`](crate::TimeUpdateStrategy).[^disclaimer]
> 
>  Note:
>  Using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration)
>  allows for mocking the wall clock for testing purposes.
>  Besides this use case, it is not recommended to do this, as it will no longer
>  represent "wall clock" time as intended.
> 
>  The [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values of this
>  clock should be used for anything which deals specifically with real time
>  (wall clock time). It will not be affected by relative game speed
>  adjustments, pausing or other adjustments.[^disclaimer]
> 
>  The clock does not count time from [`startup()`](Time::startup) to
>  [`first_update()`](Time::first_update()) into elapsed, but instead will
>  start counting time from the first update call. [`delta()`](Time::delta) and
>  [`elapsed()`](Time::elapsed) will report zero on the first update as there
>  is no previous update instant. This means that a [`delta()`](Time::delta) of
>  zero must be handled without errors in application logic, as it may
>  theoretically also happen at other times.
> 
>  [`Instant`]s for [`startup()`](Time::startup),
>  [`first_update()`](Time::first_update) and
>  [`last_update()`](Time::last_update) are recorded and accessible.
> 
>  [^disclaimer]: When using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration),
>      [`Time<Real>#impl-Time<Real>`] is only a *mock* of wall clock time.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./real/clone.md) |