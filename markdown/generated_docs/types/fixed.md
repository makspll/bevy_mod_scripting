# Fixed

### Fixed

- **timestep** : bevy\_utils::Duration
- **overstep** : bevy\_utils::Duration

## Description

>  The fixed timestep game clock following virtual time.
> 
>  A specialization of the [`Time`] structure. **For method documentation, see
>  [`Time<Fixed>#impl-Time<Fixed>`].**
>      
>  It is automatically inserted as a resource by
>  [`TimePlugin`](crate::TimePlugin) and updated based on
>  [`Time<Virtual>`](Virtual). The fixed clock is automatically set as the
>  generic [`Time`] resource during [`FixedUpdate`](bevy_app::FixedUpdate)
>  schedule processing.
> 
>  The fixed timestep clock advances in fixed-size increments, which is
>  extremely useful for writing logic (like physics) that should have
>  consistent behavior, regardless of framerate.
> 
>  The default [`timestep()`](Time::timestep) is 64 hertz, or 15625
>  microseconds. This value was chosen because using 60 hertz has the potential
>  for a pathological interaction with the monitor refresh rate where the game
>  alternates between running two fixed timesteps and zero fixed timesteps per
>  frame (for example when running two fixed timesteps takes longer than a
>  frame). Additionally, the value is a power of two which losslessly converts
>  into [`f32`] and [`f64`].
> 
>  To run a system on a fixed timestep, add it to one of the [`FixedMain`]
>  schedules, most commonly [`FixedUpdate`](bevy_app::FixedUpdate).
> 
>  This schedule is run a number of times between
>  [`PreUpdate`](bevy_app::PreUpdate) and [`Update`](bevy_app::Update)
>  according to the accumulated [`overstep()`](Time::overstep) time divided by
>  the [`timestep()`](Time::timestep). This means the schedule may run 0, 1 or
>  more times during a single update (which typically corresponds to a rendered
>  frame).
> 
>  `Time<Fixed>` and the generic [`Time`] resource will report a
>  [`delta()`](Time::delta) equal to [`timestep()`](Time::timestep) and always
>  grow [`elapsed()`](Time::elapsed) by one [`timestep()`](Time::timestep) per
>  iteration.
> 
>  The fixed timestep clock follows the [`Time<Virtual>`](Virtual) clock, which
>  means it is affected by [`pause()`](Time::pause),
>  [`set_relative_speed()`](Time::set_relative_speed) and
>  [`set_max_delta()`](Time::set_max_delta) from virtual time. If the virtual
>  clock is paused, the [`FixedUpdate`](bevy_app::FixedUpdate) schedule will
>  not run. It is guaranteed that the [`elapsed()`](Time::elapsed) time in
>  `Time<Fixed>` is always between the previous `elapsed()` and the current
>  `elapsed()` value in `Time<Virtual>`, so the values are compatible.
> 
>  Changing the timestep size while the game is running should not normally be
>  done, as having a regular interval is the point of this schedule, but it may
>  be necessary for effects like "bullet-time" if the normal granularity of the
>  fixed timestep is too big for the slowed down time. In this case,
>  [`set_timestep()`](Time::set_timestep) and be called to set a new value. The
>  new value will be used immediately for the next run of the
>  [`FixedUpdate`](bevy_app::FixedUpdate) schedule, meaning that it will affect
>  the [`delta()`](Time::delta) value for the very next
>  [`FixedUpdate`](bevy_app::FixedUpdate), even if it is still during the same
>  frame. Any [`overstep()`](Time::overstep) present in the accumulator will be
>  processed according to the new [`timestep()`](Time::timestep) value.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./fixed/clone.md) |