# Time<Fixed>

### Time

- **context** : bevy\_time::fixed::Fixed
- **wrap\_period** : bevy\_utils::Duration
- **delta** : bevy\_utils::Duration
- **delta\_secs** : f32
- **delta\_secs\_f64** : f64
- **elapsed** : bevy\_utils::Duration
- **elapsed\_secs** : f32
- **elapsed\_secs\_f64** : f64
- **elapsed\_wrapped** : bevy\_utils::Duration
- **elapsed\_secs\_wrapped** : f32
- **elapsed\_secs\_wrapped\_f64** : f64

## Description

>  A generic clock resource that tracks how much it has advanced since its
>  previous update and since its creation.
> 
>  Multiple instances of this resource are inserted automatically by
>  [`TimePlugin`](crate::TimePlugin):
> 
>  - [`Time<Real>`](crate::real::Real) tracks real wall-clock time elapsed.
>  - [`Time<Virtual>`](crate::virt::Virtual) tracks virtual game time that may
>    be paused or scaled.
>  - [`Time<Fixed>`](crate::fixed::Fixed) tracks fixed timesteps based on
>    virtual time.
>  - [`Time`] is a generic clock that corresponds to "current" or "default"
>    time for systems. It contains [`Time<Virtual>`](crate::virt::Virtual)
>    except inside the [`FixedMain`](bevy_app::FixedMain) schedule when it
>    contains [`Time<Fixed>`](crate::fixed::Fixed).
> 
>  The time elapsed since the previous time this clock was advanced is saved as
>  [`delta()`](Time::delta) and the total amount of time the clock has advanced
>  is saved as [`elapsed()`](Time::elapsed). Both are represented as exact
>  [`Duration`] values with fixed nanosecond precision. The clock does not
>  support time moving backwards, but it can be updated with [`Duration::ZERO`]
>  which will set [`delta()`](Time::delta) to zero.
> 
>  These values are also available in seconds as `f32` via
>  [`delta_secs()`](Time::delta_secs) and
>  [`elapsed_secs()`](Time::elapsed_secs), and also in seconds as `f64`
>  via [`delta_secs_f64()`](Time::delta_secs_f64) and
>  [`elapsed_secs_f64()`](Time::elapsed_secs_f64).
> 
>  Since [`elapsed_secs()`](Time::elapsed_secs) will grow constantly and
>  is `f32`, it will exhibit gradual precision loss. For applications that
>  require an `f32` value but suffer from gradual precision loss there is
>  [`elapsed_secs_wrapped()`](Time::elapsed_secs_wrapped) available. The
>  same wrapped value is also available as [`Duration`] and `f64` for
>  consistency. The wrap period is by default 1 hour, and can be set by
>  [`set_wrap_period()`](Time::set_wrap_period).
> 
>  # Accessing clocks
> 
>  By default, any systems requiring current [`delta()`](Time::delta) or
>  [`elapsed()`](Time::elapsed) should use `Res<Time>` to access the default
>  time configured for the program. By default, this refers to
>  [`Time<Virtual>`](crate::virt::Virtual) except during the
>  [`FixedMain`](bevy_app::FixedMain) schedule when it refers to
>  [`Time<Fixed>`](crate::fixed::Fixed). This ensures your system can be used
>  either in [`Update`](bevy_app::Update) or
>  [`FixedUpdate`](bevy_app::FixedUpdate) schedule depending on what is needed.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  #
>  fn ambivalent_system(time: Res<Time>) {
>      println!("this how I see time: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
>  }
>  ```
> 
>  If your system needs to react based on real time (wall clock time), like for
>  user interfaces, it should use `Res<Time<Real>>`. The
>  [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values will always
>  correspond to real time and will not be affected by pause, time scaling or
>  other tweaks.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  #
>  fn real_time_system(time: Res<Time<Real>>) {
>      println!("this will always be real time: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
>  }
>  ```
> 
>  If your system specifically needs to access fixed timestep clock, even when
>  placed in `Update` schedule, you should use `Res<Time<Fixed>>`. The
>  [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values will
>  correspond to the latest fixed timestep that has been run.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  #
>  fn fixed_time_system(time: Res<Time<Fixed>>) {
>      println!("this will always be the last executed fixed timestep: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
>  }
>  ```
> 
>  Finally, if your system specifically needs to know the current virtual game
>  time, even if placed inside [`FixedUpdate`](bevy_app::FixedUpdate), for
>  example to know if the game is [`was_paused()`](Time::was_paused) or to use
>  [`effective_speed()`](Time::effective_speed), you can use
>  `Res<Time<Virtual>>`. However, if the system is placed in
>  [`FixedUpdate`](bevy_app::FixedUpdate), extra care must be used because your
>  system might be run multiple times with the same [`delta()`](Time::delta)
>  and [`elapsed()`](Time::elapsed) values as the virtual game time has not
>  changed between the iterations.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  #
>  fn fixed_time_system(time: Res<Time<Virtual>>) {
>      println!("this will be virtual time for this update: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
>      println!("also the relative speed of the game is now {}", time.effective_speed());
>  }
>  ```
> 
>  If you need to change the settings for any of the clocks, for example to
>  [`pause()`](Time::pause) the game, you should use `ResMut<Time<Virtual>>`.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  #
>  #[derive(Event)]
>  struct PauseEvent(bool);
> 
>  fn pause_system(mut time: ResMut<Time<Virtual>>, mut events: EventReader<PauseEvent>) {
>      for ev in events.read() {
>          if ev.0 {
>              time.pause();
>          } else {
>              time.unpause();
>          }
>      }
>  }
>  ```
> 
>  # Adding custom clocks
> 
>  New custom clocks can be created by creating your own struct as a context
>  and passing it to [`new_with()`](Time::new_with). These clocks can be
>  inserted as resources as normal and then accessed by systems. You can use
>  the [`advance_by()`](Time::advance_by) or [`advance_to()`](Time::advance_to)
>  methods to move the clock forwards based on your own logic.
> 
>  If you want to add methods for your time instance and they require access to
>  both your context and the generic time part, it's probably simplest to add a
>  custom trait for them and implement it for `Time<Custom>`.
> 
>  Your context struct will need to implement the [`Default`] trait because
>  [`Time`] structures support reflection. It also makes initialization trivial
>  by being able to call `app.init_resource::<Time<Custom>>()`.
> 
>  You can also replace the "generic" `Time` clock resource if the "default"
>  time for your game should not be the default virtual time provided. You can
>  get a "generic" snapshot of your clock by calling `as_generic()` and then
>  overwrite the [`Time`] resource with it. The default systems added by
>  [`TimePlugin`](crate::TimePlugin) will overwrite the [`Time`] clock during
>  [`First`](bevy_app::First) and [`FixedUpdate`](bevy_app::FixedUpdate)
>  schedules.
> 
>  ```
>  # use bevy_ecs::prelude::*;
>  # use bevy_time::prelude::*;
>  # use bevy_utils::Instant;
>  #
>  #[derive(Debug)]
>  struct Custom {
>      last_external_time: Instant,
>  }
> 
>  impl Default for Custom {
>      fn default() -> Self {
>          Self {
>              last_external_time: Instant::now(),
>          }
>      }
>  }
> 
>  trait CustomTime {
>      fn update_from_external(&mut self, instant: Instant);
>  }
> 
>  impl CustomTime for Time<Custom> {
>      fn update_from_external(&mut self, instant: Instant) {
>           let delta = instant - self.context().last_external_time;
>           self.advance_by(delta);
>           self.context_mut().last_external_time = instant;
>      }
>  }
>  ```

## Functions

