# Virtual

### Virtual

- **max\_delta** : bevy\_utils::Duration
- **paused** : bool
- **relative\_speed** : f64
- **effective\_speed** : f64

## Description

>  The virtual game clock representing game time.
> 
>  A specialization of the [`Time`] structure. **For method documentation, see
>  [`Time<Virtual>#impl-Time<Virtual>`].**
> 
>  Normally used as `Time<Virtual>`. It is automatically inserted as a resource
>  by [`TimePlugin`](crate::TimePlugin) and updated based on
>  [`Time<Real>`](Real). The virtual clock is automatically set as the default
>  generic [`Time`] resource for the update.
> 
>  The virtual clock differs from real time clock in that it can be paused, sped up
>  and slowed down. It also limits how much it can advance in a single update
>  in order to prevent unexpected behavior in cases where updates do not happen
>  at regular intervals (e.g. coming back after the program was suspended a long time).
> 
>  The virtual clock can be paused by calling [`pause()`](Time::pause) and
>  unpaused by calling [`unpause()`](Time::unpause). When the game clock is
>  paused [`delta()`](Time::delta) will be zero on each update, and
>  [`elapsed()`](Time::elapsed) will not grow.
>  [`effective_speed()`](Time::effective_speed) will return `0.0`. Calling
>  [`pause()`](Time::pause) will not affect value the [`delta()`](Time::delta)
>  value for the update currently being processed.
> 
>  The speed of the virtual clock can be changed by calling
>  [`set_relative_speed()`](Time::set_relative_speed). A value of `2.0` means
>  that virtual clock should advance twice as fast as real time, meaning that
>  [`delta()`](Time::delta) values will be double of what
>  [`Time<Real>::delta()`](Time::delta) reports and
>  [`elapsed()`](Time::elapsed) will go twice as fast as
>  [`Time<Real>::elapsed()`](Time::elapsed). Calling
>  [`set_relative_speed()`](Time::set_relative_speed) will not affect the
>  [`delta()`](Time::delta) value for the update currently being processed.
> 
>  The maximum amount of delta time that can be added by a single update can be
>  set by [`set_max_delta()`](Time::set_max_delta). This value serves a dual
>  purpose in the virtual clock.
> 
>  If the game temporarily freezes due to any reason, such as disk access, a
>  blocking system call, or operating system level suspend, reporting the full
>  elapsed delta time is likely to cause bugs in game logic. Usually if a
>  laptop is suspended for an hour, it doesn't make sense to try to simulate
>  the game logic for the elapsed hour when resuming. Instead it is better to
>  lose the extra time and pretend a shorter duration of time passed. Setting
>  [`max_delta()`](Time::max_delta) to a relatively short time means that the
>  impact on game logic will be minimal.
> 
>  If the game lags for some reason, meaning that it will take a longer time to
>  compute a frame than the real time that passes during the computation, then
>  we would fall behind in processing virtual time. If this situation persists,
>  and computing a frame takes longer depending on how much virtual time has
>  passed, the game would enter a "death spiral" where computing each frame
>  takes longer and longer and the game will appear to freeze. By limiting the
>  maximum time that can be added at once, we also limit the amount of virtual
>  time the game needs to compute for each frame. This means that the game will
>  run slow, and it will run slower than real time, but it will not freeze and
>  it will recover as soon as computation becomes fast again.
> 
>  You should set [`max_delta()`](Time::max_delta) to a value that is
>  approximately the minimum FPS your game should have even if heavily lagged
>  for a moment. The actual FPS when lagged will be somewhat lower than this,
>  depending on how much more time it takes to compute a frame compared to real
>  time. You should also consider how stable your FPS is, as the limit will
>  also dictate how big of an FPS drop you can accept without losing time and
>  falling behind real time.

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./virtual/clone.md) |