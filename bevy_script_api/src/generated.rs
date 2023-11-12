#![allow(clippy::all, unused_imports, deprecated)]
extern crate self as bevy_script_api;
use crate::{
    error::ReflectionError,
    script_ref::{ReflectedValue, ValueIndex},
    sub_reflect::ReflectPathElem,
};
use bevy::prelude::App;
use bevy_mod_scripting_core::prelude::*;
use std::sync::Mutex;
#[cfg(feature = "lua")]
use {
    crate::{
        common::bevy::GetWorld,
        lua::{util::LuaIndex, RegisterForeignLuaType},
    },
    bevy_mod_scripting_lua::{docs::LuaDocFragment, tealr::mlu::mlua::MetaMethod},
    bevy_mod_scripting_lua_derive::LuaProxy,
};
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::AlignContent", functions[])]
pub struct AlignContent;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::AlignItems", functions[])]
pub struct AlignItems;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::AlignSelf", functions[])]
pub struct AlignSelf;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::Direction", functions[])]
pub struct Direction;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::FlexDirection", functions[])]
pub struct FlexDirection;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::FlexWrap", functions[])]
pub struct FlexWrap;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::FocusPolicy", functions[])]
pub struct FocusPolicy;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::Interaction", functions[])]
pub struct Interaction;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::JustifyContent", functions[])]
pub struct JustifyContent;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Overflow",
    functions[r#"
    ///Show overflowing items on both axes
    #[lua(kind="Function",output(proxy))]
    fn visible () -> Self;"#,
    r#"
    ///Clip overflowing items on both axes
    #[lua(kind="Function",output(proxy))]
    fn clip () -> Self;"#,
    r#"
    ///Clip overflowing items on the x axis
    #[lua(kind="Function",output(proxy))]
    fn clip_x () -> Self;"#,
    r#"
    ///Clip overflowing items on the y axis
    #[lua(kind="Function",output(proxy))]
    fn clip_y () -> Self;"#,
    r#"
    ///Overflow is visible on both axes
    #[lua(kind="Method",)]
    fn is_visible (&self, ) -> bool;"#,
    ]
)]
pub struct Overflow;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::PositionType", functions[])]
pub struct PositionType;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::Val", functions[])]
pub struct Val;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::CalculatedClip", functions[])]
pub struct CalculatedClip;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Node",
    functions[r#"
    ///The calculated node size as width and height in logical pixels
    ///automatically calculated by [`super::layout::ui_layout_system`]
    #[lua(kind="Method",output(proxy))]
    fn size (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Returns the size of the node in physical pixels based on the given scale factor and `UiScale`.
    #[lua(kind="Method",output(proxy))]
    fn physical_size (&self, scale_factor : f64, ui_scale : f64, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Returns the logical pixel coordinates of the UI node, based on its [`GlobalTransform`].
    #[lua(kind="Method",output(proxy))]
    fn logical_rect (&self, #[proxy] transform : &bevy::transform::components::GlobalTransform,) -> bevy::math::Rect;"#,
    r#"
    ///Returns the physical pixel coordinates of the UI node, based on its [`GlobalTransform`] and the scale factor.
    #[lua(kind="Method",output(proxy))]
    fn physical_rect (&self, #[proxy] transform : &bevy::transform::components::GlobalTransform,scale_factor : f64, ui_scale : f64, ) -> bevy::math::Rect;"#,
    ]
)]
pub struct Node;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::Style", functions[])]
pub struct Style;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::UiImage",
    functions[r#"
    ///flip the image along its x-axis
    #[lua(kind="Method",output(proxy))]
    fn with_flip_x (self, ) -> Self;"#,
    r#"
    ///flip the image along its y-axis
    #[lua(kind="Method",output(proxy))]
    fn with_flip_y (self, ) -> Self;"#,
    ]
)]
pub struct UiImage;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::widget::Button", functions[])]
pub struct Button;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::Display", functions[])]
pub struct Display;
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::animation::AnimationPlayer",
    functions[r#"
    ///Pause the animation
    #[lua(kind="MutatingMethod",)]
    fn pause (&mut self, );"#,
    r#"
    ///Unpause the animation
    #[lua(kind="MutatingMethod",)]
    fn resume (&mut self, );"#,
    r#"
    ///Is the animation paused
    #[lua(kind="Method",)]
    fn is_paused (&self, ) -> bool;"#,
    r#"
    ///Speed of the animation playback
    #[lua(kind="Method",)]
    fn speed (&self, ) -> f32;"#,
    r#"
    ///Time elapsed playing the animation
    #[lua(kind="Method",)]
    fn elapsed (&self, ) -> f32;"#,
    ]
)]
pub struct AnimationPlayer;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::core::Name", functions[])]
pub struct Name;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::gltf::GltfExtras", functions[])]
pub struct GltfExtras;
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::hierarchy::Children",
    functions[r#"
    ///Swaps the child at `a_index` with the child at `b_index`.
    #[lua(kind="MutatingMethod",)]
    fn swap (&mut self, a_index : usize, b_index : usize, );"#,
    ]
)]
pub struct Children;
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::hierarchy::Parent",
    functions[r#"
    ///Gets the [`Entity`] ID of the parent.
    #[lua(kind="Method",output(proxy))]
    fn get (&self, ) -> bevy::ecs::entity::Entity;"#,
    ]
)]
pub struct Parent;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::text::Text2dBounds", functions[])]
pub struct Text2dBounds;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::Text",
    functions[r#"
    ///Returns this [`Text`] with a new [`TextAlignment`].
    #[lua(kind="Method",output(proxy))]
    fn with_alignment (self, #[proxy] alignment : bevy::text::TextAlignment,) -> Self;"#,
    r#"
    ///Returns this [`Text`] with soft wrapping disabled.
    ///Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.
    #[lua(kind="Method",output(proxy))]
    fn with_no_wrap (self, ) -> Self;"#,
    ]
)]
pub struct Text;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::text::TextAlignment", functions[])]
pub struct TextAlignment;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::TextSection",
    functions[r#"
    ///Create an empty [`TextSection`] from a style. Useful when the value will be set dynamically.
    #[lua(kind="Function",output(proxy))]
    fn from_style (#[proxy] style : bevy::text::TextStyle,) -> Self;"#,
    ]
)]
pub struct TextSection;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::text::TextStyle", functions[])]
pub struct TextStyle;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::Stopwatch",
    functions[r#"
    ///Create a new unpaused `Stopwatch` with no elapsed time.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///let stopwatch = Stopwatch::new();
    ///assert_eq!(stopwatch.elapsed_secs(), 0.0);
    ///assert_eq!(stopwatch.paused(), false);
    ///```
    #[lua(kind="Function",output(proxy))]
    fn new () -> Self;"#,
    r#"
    ///Returns the elapsed time since the last [`reset`](Stopwatch::reset)
    ///of the stopwatch, in seconds.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut stopwatch = Stopwatch::new();
    ///stopwatch.tick(Duration::from_secs(1));
    ///assert_eq!(stopwatch.elapsed_secs(), 1.0);
    ///```
    ///
    ///# See Also
    ///
    ///[`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
    ///[`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.
    #[lua(kind="Method",)]
    fn elapsed_secs (&self, ) -> f32;"#,
    r#"
    ///Returns the elapsed time since the last [`reset`](Stopwatch::reset)
    ///of the stopwatch, in seconds, as f64.
    ///
    ///# See Also
    ///
    ///[`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
    ///[`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.
    #[lua(kind="Method",)]
    fn elapsed_secs_f64 (&self, ) -> f64;"#,
    r#"
    ///Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
    ///paused will not have any effect on the elapsed time.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut stopwatch = Stopwatch::new();
    ///stopwatch.pause();
    ///stopwatch.tick(Duration::from_secs_f32(1.5));
    ///assert!(stopwatch.paused());
    ///assert_eq!(stopwatch.elapsed_secs(), 0.0);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn pause (&mut self, );"#,
    r#"
    ///Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut stopwatch = Stopwatch::new();
    ///stopwatch.pause();
    ///stopwatch.tick(Duration::from_secs_f32(1.0));
    ///stopwatch.unpause();
    ///stopwatch.tick(Duration::from_secs_f32(1.0));
    ///assert!(!stopwatch.paused());
    ///assert_eq!(stopwatch.elapsed_secs(), 1.0);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn unpause (&mut self, );"#,
    r#"
    ///Returns `true` if the stopwatch is paused.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///let mut stopwatch = Stopwatch::new();
    ///assert!(!stopwatch.paused());
    ///stopwatch.pause();
    ///assert!(stopwatch.paused());
    ///stopwatch.unpause();
    ///assert!(!stopwatch.paused());
    ///```
    #[lua(kind="Method",)]
    fn paused (&self, ) -> bool;"#,
    r#"
    ///Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut stopwatch = Stopwatch::new();
    ///stopwatch.tick(Duration::from_secs_f32(1.5));
    ///stopwatch.reset();
    ///assert_eq!(stopwatch.elapsed_secs(), 0.0);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn reset (&mut self, );"#,
    ]
)]
pub struct Stopwatch;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::Timer",
    functions[r#"
    ///Returns `true` if the timer has reached its duration at least once.
    ///See also [`Timer::just_finished`](Timer::just_finished).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(1.5));
    ///assert!(timer.finished());
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert!(timer.finished());
    ///```
    #[lua(kind="Method",)]
    fn finished (&self, ) -> bool;"#,
    r#"
    ///Returns `true` only on the tick the timer reached its duration.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(1.5));
    ///assert!(timer.just_finished());
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert!(!timer.just_finished());
    ///```
    #[lua(kind="Method",)]
    fn just_finished (&self, ) -> bool;"#,
    r#"
    ///Returns the time elapsed on the timer as an `f32`.
    ///See also [`Timer::elapsed`](Timer::elapsed).
    #[lua(kind="Method",)]
    fn elapsed_secs (&self, ) -> f32;"#,
    r#"
    ///Pauses the Timer. Disables the ticking of the timer.
    ///
    ///See also [`Stopwatch::pause`](Stopwatch::pause).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///timer.pause();
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert_eq!(timer.elapsed_secs(), 0.0);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn pause (&mut self, );"#,
    r#"
    ///Unpauses the Timer. Resumes the ticking of the timer.
    ///
    ///See also [`Stopwatch::unpause()`](Stopwatch::unpause).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///timer.pause();
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///timer.unpause();
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert_eq!(timer.elapsed_secs(), 0.5);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn unpause (&mut self, );"#,
    r#"
    ///Returns `true` if the timer is paused.
    ///
    ///See also [`Stopwatch::paused`](Stopwatch::paused).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///assert!(!timer.paused());
    ///timer.pause();
    ///assert!(timer.paused());
    ///timer.unpause();
    ///assert!(!timer.paused());
    ///```
    #[lua(kind="Method",)]
    fn paused (&self, ) -> bool;"#,
    r#"
    ///Resets the timer. The reset doesn't affect the `paused` state of the timer.
    ///
    ///See also [`Stopwatch::reset`](Stopwatch::reset).
    ///
    ///Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(1.5));
    ///timer.reset();
    ///assert!(!timer.finished());
    ///assert!(!timer.just_finished());
    ///assert_eq!(timer.elapsed_secs(), 0.0);
    ///```
    #[lua(kind="MutatingMethod",)]
    fn reset (&mut self, );"#,
    r#"
    ///Returns the percentage of the timer elapsed time (goes from 0.0 to 1.0).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert_eq!(timer.percent(), 0.25);
    ///```
    #[lua(kind="Method",)]
    fn percent (&self, ) -> f32;"#,
    r#"
    ///Returns the percentage of the timer remaining time (goes from 1.0 to 0.0).
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert_eq!(timer.percent_left(), 0.75);
    ///```
    #[lua(kind="Method",)]
    fn percent_left (&self, ) -> f32;"#,
    r#"
    ///Returns the remaining time in seconds
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::cmp::Ordering;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///let result = timer.remaining_secs().total_cmp(&1.5);
    ///assert_eq!(Ordering::Equal, result);
    ///```
    #[lua(kind="Method",)]
    fn remaining_secs (&self, ) -> f32;"#,
    r#"
    ///Returns the number of times a repeating timer
    ///finished during the last [`tick`](Timer<T>::tick) call.
    ///
    ///For non repeating-timers, this method will only ever
    ///return 0 or 1.
    ///
    ///# Examples
    ///```
    ///# use bevy_time::*;
    ///use std::time::Duration;
    ///let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    ///timer.tick(Duration::from_secs_f32(6.0));
    ///assert_eq!(timer.times_finished_this_tick(), 6);
    ///timer.tick(Duration::from_secs_f32(2.0));
    ///assert_eq!(timer.times_finished_this_tick(), 2);
    ///timer.tick(Duration::from_secs_f32(0.5));
    ///assert_eq!(timer.times_finished_this_tick(), 0);
    ///```
    #[lua(kind="Method",)]
    fn times_finished_this_tick (&self, ) -> u32;"#,
    ]
)]
pub struct Timer;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ecs::entity::Entity",
    functions[r#"
    ///Creates a new entity ID with the specified `index` and a generation of 0.
    ///
    ///# Note
    ///
    ///Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
    ///[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
    ///only be used for sharing entities across apps, and only when they have a scheme
    ///worked out to share an index space (which doesn't happen by default).
    ///
    ///In general, one should not try to synchronize the ECS by attempting to ensure that
    ///`Entity` lines up between instances, but instead insert a secondary identifier as
    ///a component.
    #[lua(kind="Function",output(proxy))]
    fn from_raw (index : u32, ) -> bevy::ecs::entity::Entity;"#,
    r#"
    ///Convert to a form convenient for passing outside of rust.
    ///
    ///Only useful for identifying entities within the same instance of an application. Do not use
    ///for serialization between runs.
    ///
    ///No particular structure is guaranteed for the returned bits.
    #[lua(kind="Method",)]
    fn to_bits (self, ) -> u64;"#,
    r#"
    ///Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
    ///
    ///Only useful when applied to results from `to_bits` in the same instance of an application.
    #[lua(kind="Function",output(proxy))]
    fn from_bits (bits : u64, ) -> Self;"#,
    r#"
    ///Return a transiently unique identifier.
    ///
    ///No two simultaneously-live entities share the same index, but dead entities' indices may collide
    ///with both live and dead entities. Useful for compactly representing entities within a
    ///specific snapshot of the world, such as when serializing.
    #[lua(kind="Method",)]
    fn index (self, ) -> u32;"#,
    r#"
    ///Returns the generation of this Entity's index. The generation is incremented each time an
    ///entity with a given index is despawned. This serves as a "count" of the number of times a
    ///given index has been reused (index, generation) pairs uniquely identify a given Entity.
    #[lua(kind="Method",)]
    fn generation (self, ) -> u32;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::ecs::storage::SparseSetIndex",)]
    fn sparse_set_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function",as_trait="bevy::ecs::storage::SparseSetIndex",output(proxy))]
    fn get_sparse_set_index (value : usize, ) -> Self;"#,
    ]
)]
pub struct Entity;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::Transform",
    functions[r#"
    ///Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
    ///is used for z-ordering elements: higher `z`-value will be in front of lower
    ///`z`-value.
    #[lua(kind="Function",output(proxy))]
    fn from_xyz (x : f32, y : f32, z : f32, ) -> Self;"#,
    r#"
    ///Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
    ///transformation matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_matrix (#[proxy] matrix : bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
    ///all axes.
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
    ///all axes.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation (#[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
    ///all axes.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
    ///points towards the `target` position and [`Transform::up`] points towards `up`.
    ///
    ///In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
    ///* if `target` is the same as the transform translation, `Vec3::Z` is used instead
    ///* if `up` is zero, `Vec3::Y` is used instead
    ///* if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction
    #[lua(kind="Method",output(proxy))]
    fn looking_at (self, #[proxy] target : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
    ///points in the given `direction` and [`Transform::up`] points towards `up`.
    ///
    ///In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
    ///* if `direction` is zero, `Vec3::Z` is used instead
    ///* if `up` is zero, `Vec3::Y` is used instead
    ///* if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction
    #[lua(kind="Method",output(proxy))]
    fn looking_to (self, #[proxy] direction : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns this [`Transform`] with a new translation.
    #[lua(kind="Method",output(proxy))]
    fn with_translation (self, #[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns this [`Transform`] with a new rotation.
    #[lua(kind="Method",output(proxy))]
    fn with_rotation (self, #[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Returns this [`Transform`] with a new scale.
    #[lua(kind="Method",output(proxy))]
    fn with_scale (self, #[proxy] scale : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns the 3d affine transformation matrix from this transforms translation,
    ///rotation, and scale.
    #[lua(kind="Method",output(proxy))]
    fn compute_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    r#"
    ///Returns the 3d affine transformation matrix from this transforms translation,
    ///rotation, and scale.
    #[lua(kind="Method",output(proxy))]
    fn compute_affine (&self, ) -> bevy::math::f32::Affine3A;"#,
    r#"
    ///Get the unit vector in the local `X` direction.
    #[lua(kind="Method",output(proxy))]
    fn local_x (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`-local_x()`][Transform::local_x()]
    #[lua(kind="Method",output(proxy))]
    fn left (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`local_x()`][Transform::local_x()]
    #[lua(kind="Method",output(proxy))]
    fn right (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Get the unit vector in the local `Y` direction.
    #[lua(kind="Method",output(proxy))]
    fn local_y (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`local_y()`][Transform::local_y]
    #[lua(kind="Method",output(proxy))]
    fn up (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`-local_y()`][Transform::local_y]
    #[lua(kind="Method",output(proxy))]
    fn down (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Get the unit vector in the local `Z` direction.
    #[lua(kind="Method",output(proxy))]
    fn local_z (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`-local_z()`][Transform::local_z]
    #[lua(kind="Method",output(proxy))]
    fn forward (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Equivalent to [`local_z()`][Transform::local_z]
    #[lua(kind="Method",output(proxy))]
    fn back (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Rotates this [`Transform`] by the given rotation.
    ///
    ///If this [`Transform`] has a parent, the `rotation` is relative to the rotation of the parent.
    ///
    ///# Examples
    ///
    ///- [`3d_rotation`]
    ///
    ///[`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
    #[lua(kind="MutatingMethod",)]
    fn rotate (&mut self, #[proxy] rotation : bevy::math::f32::Quat,);"#,
    r#"
    ///Rotates this [`Transform`] around the given `axis` by `angle` (in radians).
    ///
    ///If this [`Transform`] has a parent, the `axis` is relative to the rotation of the parent.
    #[lua(kind="MutatingMethod",)]
    fn rotate_axis (&mut self, #[proxy] axis : bevy::math::f32::Vec3,angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around the `X` axis by `angle` (in radians).
    ///
    ///If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
    #[lua(kind="MutatingMethod",)]
    fn rotate_x (&mut self, angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).
    ///
    ///If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
    #[lua(kind="MutatingMethod",)]
    fn rotate_y (&mut self, angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).
    ///
    ///If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
    #[lua(kind="MutatingMethod",)]
    fn rotate_z (&mut self, angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] by the given `rotation`.
    ///
    ///The `rotation` is relative to this [`Transform`]'s current rotation.
    #[lua(kind="MutatingMethod",)]
    fn rotate_local (&mut self, #[proxy] rotation : bevy::math::f32::Quat,);"#,
    r#"
    ///Rotates this [`Transform`] around its local `axis` by `angle` (in radians).
    #[lua(kind="MutatingMethod",)]
    fn rotate_local_axis (&mut self, #[proxy] axis : bevy::math::f32::Vec3,angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).
    #[lua(kind="MutatingMethod",)]
    fn rotate_local_x (&mut self, angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).
    #[lua(kind="MutatingMethod",)]
    fn rotate_local_y (&mut self, angle : f32, );"#,
    r#"
    ///Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).
    #[lua(kind="MutatingMethod",)]
    fn rotate_local_z (&mut self, angle : f32, );"#,
    r#"
    ///Translates this [`Transform`] around a `point` in space.
    ///
    ///If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.
    #[lua(kind="MutatingMethod",)]
    fn translate_around (&mut self, #[proxy] point : bevy::math::f32::Vec3,#[proxy] rotation : bevy::math::f32::Quat,);"#,
    r#"
    ///Rotates this [`Transform`] around a `point` in space.
    ///
    ///If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.
    #[lua(kind="MutatingMethod",)]
    fn rotate_around (&mut self, #[proxy] point : bevy::math::f32::Vec3,#[proxy] rotation : bevy::math::f32::Quat,);"#,
    r#"
    ///Rotates this [`Transform`] so that [`Transform::forward`] points towards the `target` position,
    ///and [`Transform::up`] points towards `up`.
    ///
    ///In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
    ///* if `target` is the same as the transtorm translation, `Vec3::Z` is used instead
    ///* if `up` is zero, `Vec3::Y` is used instead
    ///* if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction
    #[lua(kind="MutatingMethod",)]
    fn look_at (&mut self, #[proxy] target : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,);"#,
    r#"
    ///Rotates this [`Transform`] so that [`Transform::forward`] points in the given `direction`
    ///and [`Transform::up`] points towards `up`.
    ///
    ///In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
    ///* if `direction` is zero, `Vec3::NEG_Z` is used instead
    ///* if `up` is zero, `Vec3::Y` is used instead
    ///* if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction
    #[lua(kind="MutatingMethod",)]
    fn look_to (&mut self, #[proxy] direction : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,);"#,
    r#"
    ///Multiplies `self` with `transform` component by component, returning the
    ///resulting [`Transform`]
    #[lua(kind="Method",output(proxy))]
    fn mul_transform (&self, #[proxy] transform : bevy::transform::components::Transform,) -> Self;"#,
    r#"
    ///Transforms the given `point`, applying scale, rotation and translation.
    ///
    ///If this [`Transform`] has a parent, this will transform a `point` that is
    ///relative to the parent's [`Transform`] into one relative to this [`Transform`].
    ///
    ///If this [`Transform`] does not have a parent, this will transform a `point`
    ///that is in global space into one relative to this [`Transform`].
    ///
    ///If you want to transform a `point` in global space to the local space of this [`Transform`],
    ///consider using [`GlobalTransform::transform_point()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_point (&self, #[proxy] point : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    ]
)]
pub struct Transform;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::GlobalTransform",
    functions[r#"
    ///Returns the 3d affine transformation matrix as a [`Mat4`].
    #[lua(kind="Method",output(proxy))]
    fn compute_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    r#"
    ///Returns the 3d affine transformation matrix as an [`Affine3A`].
    #[lua(kind="Method",output(proxy))]
    fn affine (&self, ) -> bevy::math::f32::Affine3A;"#,
    r#"
    ///Returns the transformation as a [`Transform`].
    ///
    ///The transform is expected to be non-degenerate and without shearing, or the output
    ///will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn compute_transform (&self, ) -> bevy::transform::components::Transform;"#,
    r#"
    ///Returns the [`Transform`] `self` would have if it was a child of an entity
    ///with the `parent` [`GlobalTransform`].
    ///
    ///This is useful if you want to "reparent" an [`Entity`](bevy_ecs::entity::Entity).
    ///Say you have an entity `e1` that you want to turn into a child of `e2`,
    ///but you want `e1` to keep the same global transform, even after re-parenting. You would use:
    ///
    ///```rust
    ///# use bevy_transform::prelude::{GlobalTransform, Transform};
    ///# use bevy_ecs::prelude::{Entity, Query, Component, Commands};
    ///# use bevy_hierarchy::{prelude::Parent, BuildChildren};
    ///#[derive(Component)]
    ///struct ToReparent {
    ///    new_parent: Entity,
    ///}
    ///fn reparent_system(
    ///    mut commands: Commands,
    ///    mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,
    ///    transforms: Query<&GlobalTransform>,
    ///) {
    ///    for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {
    ///        if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {
    ///            *transform = initial.reparented_to(parent_transform);
    ///            commands.entity(entity)
    ///                .remove::<ToReparent>()
    ///                .set_parent(to_reparent.new_parent);
    ///        }
    ///    }
    ///}
    ///```
    ///
    ///The transform is expected to be non-degenerate and without shearing, or the output
    ///will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn reparented_to (&self, #[proxy] parent : &bevy::transform::components::GlobalTransform,) -> bevy::transform::components::Transform;"#,
    r#"
    ///Return the local right vector (X).
    #[lua(kind="Method",output(proxy))]
    fn right (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Return the local left vector (-X).
    #[lua(kind="Method",output(proxy))]
    fn left (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Return the local up vector (Y).
    #[lua(kind="Method",output(proxy))]
    fn up (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Return the local down vector (-Y).
    #[lua(kind="Method",output(proxy))]
    fn down (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Return the local back vector (Z).
    #[lua(kind="Method",output(proxy))]
    fn back (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Return the local forward vector (-Z).
    #[lua(kind="Method",output(proxy))]
    fn forward (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Get the translation as a [`Vec3`].
    #[lua(kind="Method",output(proxy))]
    fn translation (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Get the translation as a [`Vec3A`].
    #[lua(kind="Method",output(proxy))]
    fn translation_vec3a (&self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Get an upper bound of the radius from the given `extents`.
    #[lua(kind="Method",)]
    fn radius_vec3a (&self, #[proxy] extents : bevy::math::f32::Vec3A,) -> f32;"#,
    r#"
    ///Transforms the given `point`, applying shear, scale, rotation and translation.
    ///
    ///This moves `point` into the local space of this [`GlobalTransform`].
    #[lua(kind="Method",output(proxy))]
    fn transform_point (&self, #[proxy] point : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Multiplies `self` with `transform` component by component, returning the
    ///resulting [`GlobalTransform`]
    #[lua(kind="Method",output(proxy))]
    fn mul_transform (&self, #[proxy] transform : bevy::transform::components::Transform,) -> Self;"#,
    ]
)]
pub struct GlobalTransform;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::AmbientLight", functions[])]
pub struct AmbientLight;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::CubemapVisibleEntities", functions[])]
pub struct CubemapVisibleEntities;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::DirectionalLight", functions[])]
pub struct DirectionalLight;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::DirectionalLightShadowMap",
    functions[]
)]
pub struct DirectionalLightShadowMap;
#[derive(LuaProxy)]
#[proxy(derive(), remote = "bevy::pbr::NotShadowCaster", functions[])]
pub struct NotShadowCaster;
#[derive(LuaProxy)]
#[proxy(derive(), remote = "bevy::pbr::NotShadowReceiver", functions[])]
pub struct NotShadowReceiver;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::PointLight", functions[])]
pub struct PointLight;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::PointLightShadowMap", functions[])]
pub struct PointLightShadowMap;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::AlphaMode", functions[])]
pub struct AlphaMode;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::pbr::wireframe::Wireframe", functions[])]
pub struct Wireframe;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::wireframe::WireframeConfig",
    functions[]
)]
pub struct WireframeConfig;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::core_3d::Camera3dDepthLoadOp",
    functions[]
)]
pub struct Camera3dDepthLoadOp;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::clear_color::ClearColor",
    functions[]
)]
pub struct ClearColor;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::clear_color::ClearColorConfig",
    functions[]
)]
pub struct ClearColorConfig;
#[derive(LuaProxy)]
#[proxy(derive(clone), remote = "bevy::core_pipeline::core_2d::Camera2d", functions[])]
pub struct Camera2d;
#[derive(LuaProxy)]
#[proxy(derive(clone), remote = "bevy::core_pipeline::core_3d::Camera3d", functions[])]
pub struct Camera3d;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::Anchor",
    functions[r#"
    #[lua(kind="Method",output(proxy))]
    fn as_vec (&self, ) -> bevy::math::f32::Vec2;"#,
    ]
)]
pub struct Anchor;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::sprite::Mesh2dHandle", functions[])]
pub struct Mesh2dHandle;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::TextureAtlasSprite",
    functions[r#"
    ///Create a new [`TextureAtlasSprite`] with a sprite index,
    ///it should be valid in the corresponding [`TextureAtlas`]
    #[lua(kind="Function",output(proxy))]
    fn new (index : usize, ) -> bevy::sprite::TextureAtlasSprite;"#,
    ]
)]
pub struct TextureAtlasSprite;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::sprite::Sprite", functions[])]
pub struct Sprite;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::visibility::RenderLayers",
    functions[r#"
    ///Create a new `RenderLayers` that belongs to all layers.
    #[lua(kind="Function",output(proxy))]
    fn all () -> Self;"#,
    r#"
    ///Create a new `RenderLayers` that belongs to no layers.
    #[lua(kind="Function",output(proxy))]
    fn none () -> Self;"#,
    r#"
    ///Determine if a `RenderLayers` intersects another.
    ///
    ///`RenderLayers`s intersect if they share any common layers.
    ///
    ///A `RenderLayers` with no layers will not match any other
    ///`RenderLayers`, even another with no layers.
    #[lua(kind="Method",)]
    fn intersects (&self, #[proxy] other : &bevy::render::view::visibility::RenderLayers,) -> bool;"#,
    ]
)]
pub struct RenderLayers;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::visibility::Visibility",
    functions[]
)]
pub struct Visibility;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::visibility::VisibleEntities",
    functions[r#"
    #[lua(kind="Method",)]
    fn len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method",)]
    fn is_empty (&self, ) -> bool;"#,
    ]
)]
pub struct VisibleEntities;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::visibility::ComputedVisibility",
    functions[r#"
    ///Whether this entity is visible to something this frame. This is true if and only if [`Self::is_visible_in_hierarchy`] and [`Self::is_visible_in_view`]
    ///are true. This is the canonical method to call to determine if an entity should be drawn.
    ///This value is updated in [`PostUpdate`] by the [`VisibilitySystems::CheckVisibility`] system set.
    ///Reading it during [`Update`](bevy_app::Update) will yield the value from the previous frame.
    #[lua(kind="Method",)]
    fn is_visible (&self, ) -> bool;"#,
    r#"
    ///Whether this entity is visible in the entity hierarchy, which is determined by the [`Visibility`] component.
    ///This takes into account "visibility inheritance". If any of this entity's ancestors (see [`Parent`]) are hidden, this entity
    ///will be hidden as well. This value is updated in the [`VisibilitySystems::VisibilityPropagate`], which lives in the [`PostUpdate`] schedule.
    #[lua(kind="Method",)]
    fn is_visible_in_hierarchy (&self, ) -> bool;"#,
    r#"
    ///Whether this entity is visible in _any_ view (Cameras, Lights, etc). Each entity type (and view type) should choose how to set this
    ///value. For cameras and drawn entities, this will take into account [`RenderLayers`].
    ///
    ///This value is reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] during [`PostUpdate`].
    ///Each entity type then chooses how to set this field in the [`VisibilitySystems::CheckVisibility`] system set, in [`PostUpdate`].
    ///Meshes might use frustum culling to decide if they are visible in a view.
    ///Other entities might just set this to `true` every frame.
    #[lua(kind="Method",)]
    fn is_visible_in_view (&self, ) -> bool;"#,
    r#"
    ///Sets `is_visible_in_view` to `true`. This is not reversible for a given frame, as it encodes whether or not this is visible in
    ///_any_ view. This will be automatically reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] and then set
    ///to the proper value in [`VisibilitySystems::CheckVisibility`]. This should _only_ be set in systems with the [`VisibilitySystems::CheckVisibility`]
    ///label. Don't call this unless you are defining a custom visibility system. For normal user-defined entity visibility, see [`Visibility`].
    #[lua(kind="MutatingMethod",)]
    fn set_visible_in_view (&mut self, );"#,
    ]
)]
pub struct ComputedVisibility;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::mesh::skinning::SkinnedMesh",
    functions[]
)]
pub struct SkinnedMesh;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::render::camera::ScalingMode", functions[])]
pub struct ScalingMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::color::Color",
    functions[r#"
    ///New `Color` from sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0.0, 1.0]
    ///* `g` - Green channel. [0.0, 1.0]
    ///* `b` - Blue channel. [0.0, 1.0]
    ///
    ///See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].
    #[lua(kind="Function",output(proxy))]
    fn rgb (r : f32, g : f32, b : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` from sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0.0, 1.0]
    ///* `g` - Green channel. [0.0, 1.0]
    ///* `b` - Blue channel. [0.0, 1.0]
    ///* `a` - Alpha channel. [0.0, 1.0]
    ///
    ///See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].
    #[lua(kind="Function",output(proxy))]
    fn rgba (r : f32, g : f32, b : f32, a : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` from linear RGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0.0, 1.0]
    ///* `g` - Green channel. [0.0, 1.0]
    ///* `b` - Blue channel. [0.0, 1.0]
    ///
    ///See also [`Color::rgb`], [`Color::rgba_linear`].
    #[lua(kind="Function",output(proxy))]
    fn rgb_linear (r : f32, g : f32, b : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` from linear RGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0.0, 1.0]
    ///* `g` - Green channel. [0.0, 1.0]
    ///* `b` - Blue channel. [0.0, 1.0]
    ///* `a` - Alpha channel. [0.0, 1.0]
    ///
    ///See also [`Color::rgba`], [`Color::rgb_linear`].
    #[lua(kind="Function",output(proxy))]
    fn rgba_linear (r : f32, g : f32, b : f32, a : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` with HSL representation in sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `hue` - Hue channel. [0.0, 360.0]
    ///* `saturation` - Saturation channel. [0.0, 1.0]
    ///* `lightness` - Lightness channel. [0.0, 1.0]
    ///
    ///See also [`Color::hsla`].
    #[lua(kind="Function",output(proxy))]
    fn hsl (hue : f32, saturation : f32, lightness : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` with HSL representation in sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `hue` - Hue channel. [0.0, 360.0]
    ///* `saturation` - Saturation channel. [0.0, 1.0]
    ///* `lightness` - Lightness channel. [0.0, 1.0]
    ///* `alpha` - Alpha channel. [0.0, 1.0]
    ///
    ///See also [`Color::hsl`].
    #[lua(kind="Function",output(proxy))]
    fn hsla (hue : f32, saturation : f32, lightness : f32, alpha : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` with LCH representation in sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `lightness` - Lightness channel. [0.0, 1.5]
    ///* `chroma` - Chroma channel. [0.0, 1.5]
    ///* `hue` - Hue channel. [0.0, 360.0]
    ///
    ///See also [`Color::lcha`].
    #[lua(kind="Function",output(proxy))]
    fn lch (lightness : f32, chroma : f32, hue : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` with LCH representation in sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `lightness` - Lightness channel. [0.0, 1.5]
    ///* `chroma` - Chroma channel. [0.0, 1.5]
    ///* `hue` - Hue channel. [0.0, 360.0]
    ///* `alpha` - Alpha channel. [0.0, 1.0]
    ///
    ///See also [`Color::lch`].
    #[lua(kind="Function",output(proxy))]
    fn lcha (lightness : f32, chroma : f32, hue : f32, alpha : f32, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` from sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0, 255]
    ///* `g` - Green channel. [0, 255]
    ///* `b` - Blue channel. [0, 255]
    ///
    ///See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].
    #[lua(kind="Function",output(proxy))]
    fn rgb_u8 (r : u8, g : u8, b : u8, ) -> bevy::render::color::Color;"#,
    r#"
    ///New `Color` from sRGB colorspace.
    ///
    ///# Arguments
    ///
    ///* `r` - Red channel. [0, 255]
    ///* `g` - Green channel. [0, 255]
    ///* `b` - Blue channel. [0, 255]
    ///* `a` - Alpha channel. [0, 255]
    ///
    ///See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].
    #[lua(kind="Function",output(proxy))]
    fn rgba_u8 (r : u8, g : u8, b : u8, a : u8, ) -> bevy::render::color::Color;"#,
    r#"
    ///Get red in sRGB colorspace.
    #[lua(kind="Method",)]
    fn r (&self, ) -> f32;"#,
    r#"
    ///Get green in sRGB colorspace.
    #[lua(kind="Method",)]
    fn g (&self, ) -> f32;"#,
    r#"
    ///Get blue in sRGB colorspace.
    #[lua(kind="Method",)]
    fn b (&self, ) -> f32;"#,
    r#"
    ///Returns this color with red set to a new value in sRGB colorspace.
    #[lua(kind="Method",output(proxy))]
    fn with_r (self, r : f32, ) -> Self;"#,
    r#"
    ///Returns this color with green set to a new value in sRGB colorspace.
    #[lua(kind="Method",output(proxy))]
    fn with_g (self, g : f32, ) -> Self;"#,
    r#"
    ///Returns this color with blue set to a new value in sRGB colorspace.
    #[lua(kind="Method",output(proxy))]
    fn with_b (self, b : f32, ) -> Self;"#,
    r#"
    ///Get alpha.
    #[lua(kind="Method",)]
    fn a (&self, ) -> f32;"#,
    r#"
    ///Returns this color with a new alpha value.
    #[lua(kind="Method",output(proxy))]
    fn with_a (self, a : f32, ) -> Self;"#,
    ]
)]
pub struct Color;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::primitives::Aabb",
    functions[r#"
    #[lua(kind="Function",output(proxy))]
    fn from_min_max (#[proxy] minimum : bevy::math::f32::Vec3,#[proxy] maximum : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn min (&self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn max (&self, ) -> bevy::math::f32::Vec3A;"#,
    ]
)]
pub struct Aabb;
#[derive(LuaProxy)]
#[proxy(derive(debug), remote = "bevy::render::primitives::CubemapFrusta", functions[])]
pub struct CubemapFrusta;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::primitives::Frustum",
    functions[r#"
    ///Returns a frustum derived from `view_projection`.
    #[lua(kind="Function",output(proxy))]
    fn from_view_projection (#[proxy] view_projection : &bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Returns a frustum derived from `view_projection`,
    ///but with a custom far plane.
    #[lua(kind="Function",output(proxy))]
    fn from_view_projection_custom_far (#[proxy] view_projection : &bevy::math::f32::Mat4,#[proxy] view_translation : &bevy::math::f32::Vec3,#[proxy] view_backward : &bevy::math::f32::Vec3,far : f32, ) -> Self;"#,
    r#"
    ///Checks if an Oriented Bounding Box (obb) intersects the frustum.
    #[lua(kind="Method",)]
    fn intersects_obb (&self, #[proxy] aabb : &bevy::render::primitives::Aabb,#[proxy] model_to_world : &bevy::math::f32::Mat4,intersect_near : bool, intersect_far : bool, ) -> bool;"#,
    ]
)]
pub struct Frustum;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::Msaa",
    functions[r#"
    #[lua(kind="Method",)]
    fn samples (&self, ) -> u32;"#,
    ]
)]
pub struct Msaa;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::Camera",
    functions[r#"
    ///The projection matrix computed using this camera's [`CameraProjection`].
    #[lua(kind="Method",output(proxy))]
    fn projection_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    ]
)]
pub struct Camera;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::RenderTarget",
    functions[]
)]
pub struct RenderTarget;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::render::camera::Viewport", functions[])]
pub struct Viewport;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::Projection",
    functions[r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",output(proxy))]
    fn get_projection_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    r#"
    #[lua(kind="MutatingMethod",as_trait="bevy::render::camera::CameraProjection",)]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",)]
    fn far (&self, ) -> f32;"#,
    ]
)]
pub struct Projection;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::OrthographicProjection",
    functions[r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",output(proxy))]
    fn get_projection_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    r#"
    #[lua(kind="MutatingMethod",as_trait="bevy::render::camera::CameraProjection",)]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",)]
    fn far (&self, ) -> f32;"#,
    ]
)]
pub struct OrthographicProjection;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::PerspectiveProjection",
    functions[r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",output(proxy))]
    fn get_projection_matrix (&self, ) -> bevy::math::f32::Mat4;"#,
    r#"
    #[lua(kind="MutatingMethod",as_trait="bevy::render::camera::CameraProjection",)]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::render::camera::CameraProjection",)]
    fn far (&self, ) -> f32;"#,
    ]
)]
pub struct PerspectiveProjection;
#[derive(LuaProxy)]
#[proxy(derive(), remote = "bevy::render::camera::CameraRenderGraph", functions[])]
pub struct CameraRenderGraph;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::asset::AssetPathId",
    functions[r#"
    ///Gets the id of the source path.
    #[lua(kind="Method",output(proxy))]
    fn source_path_id (&self, ) -> bevy::asset::SourcePathId;"#,
    r#"
    ///Gets the id of the sub-asset label.
    #[lua(kind="Method",output(proxy))]
    fn label_id (&self, ) -> bevy::asset::LabelId;"#,
    ]
)]
pub struct AssetPathId;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::asset::LabelId", functions[])]
pub struct LabelId;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::asset::SourcePathId", functions[])]
pub struct SourcePathId;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::asset::HandleId", functions[])]
pub struct HandleId;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Vec2",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f32, y : f32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec2,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 3D vector from `self` and the given `z` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, z : f32, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f32;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f32;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f32;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f32, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f32, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f32, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    ///conjunction with the [`rotate()`][Self::rotate()] method, e.g.
    ///`Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
    ///and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f32, ) -> Self;"#,
    r#"
    ///Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
    ///
    ///The inputs do not need to be unit vectors however they must be non-zero.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns a vector that is equal to `self` rotated by 90 degrees.
    #[lua(kind="Method",output(proxy))]
    fn perp (self, ) -> Self;"#,
    r#"
    ///The perpendicular dot product of `self` and `rhs`.
    ///Also known as the wedge product, 2D cross product, and determinant.
    #[lua(kind="Method",)]
    fn perp_dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    ///then this just rotation. This is what you usually want. Otherwise,
    ///it will be like a rotation with a multiplication by `self`'s length.
    #[lua(kind="Method",output(proxy))]
    fn rotate (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec2 (&self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec2 (&self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec2 (&self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f32::Vec4;"#,
    ]
)]
pub struct Vec2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Vec3",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f32, y : f32, z : f32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec3,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 4D vector from `self` and the given `w` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, w : f32, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    ///Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Computes the cross product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn cross (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f32;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f32;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f32;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f32, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f32, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f32, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Returns the angle (in radians) between two vectors.
    ///
    ///The inputs do not need to be unit vectors however they must be non-zero.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns some vector that is orthogonal to the given one.
    ///
    ///The input vector must be finite and non-zero.
    ///
    ///The output vector is not necessarily unit length. For that use
    ///[`Self::any_orthonormal_vector()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn any_orthogonal_vector (&self, ) -> Self;"#,
    r#"
    ///Returns any unit vector that is orthogonal to the given one.
    ///
    ///The input vector must be unit length.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn any_orthonormal_vector (&self, ) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec3 (&self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec3 (&self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec3 (&self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::f32::Vec4;"#,
    ]
)]
pub struct Vec3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Vec3A",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f32, y : f32, z : f32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec3A,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 4D vector from `self` and the given `w` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, w : f32, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    ///Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Computes the cross product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn cross (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec3A;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f32;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f32;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f32;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f32, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f32, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f32, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Returns the angle (in radians) between two vectors.
    ///
    ///The inputs do not need to be unit vectors however they must be non-zero.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns some vector that is orthogonal to the given one.
    ///
    ///The input vector must be finite and non-zero.
    ///
    ///The output vector is not necessarily unit length. For that use
    ///[`Self::any_orthonormal_vector()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn any_orthogonal_vector (&self, ) -> Self;"#,
    r#"
    ///Returns any unit vector that is orthogonal to the given one.
    ///
    ///The input vector must be unit length.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn any_orthonormal_vector (&self, ) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec3 (&self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec3 (&self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec3 (&self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::f32::Vec4;"#,
    ]
)]
pub struct Vec3A;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Vec4",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f32, y : f32, z : f32, w : f32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec4A,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    ///Truncation to [`Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    ///
    ///To truncate to [`Vec3A`] use [`Vec3A::from()`].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec4A;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f32;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f32;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f32;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f32, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f32, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f32, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f32, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec4 (&self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec4 (&self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec4 (&self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xw (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yw (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zw (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wx (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wy (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wz (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ww (self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xww (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yww (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zww (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzw (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwx (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwy (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn www (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzww (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzw (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwx (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwy (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwz (self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwww (self, ) -> bevy::math::f32::Vec4;"#,
    ]
)]
pub struct Vec4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::bool::BVec2",
    functions[r#"
    ///Creates a new vector mask.
    #[lua(kind="Function",output(proxy))]
    fn new (x : bool, y : bool, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : bool, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 2 bits set from the elements of `self`.
    ///
    ///A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn bitmask (self, ) -> u32;"#,
    r#"
    ///Returns true if any of the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn any (self, ) -> bool;"#,
    r#"
    ///Returns true if all the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn all (self, ) -> bool;"#,
    r#"
    ///Tests the value at `index`.
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="Method",)]
    fn test (&self, index : usize, ) -> bool;"#,
    r#"
    ///Sets the element at `index`.
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="MutatingMethod",)]
    fn set (&mut self, index : usize, value : bool, );"#,
    ]
)]
pub struct BVec2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::bool::BVec3",
    functions[r#"
    ///Creates a new vector mask.
    #[lua(kind="Function",output(proxy))]
    fn new (x : bool, y : bool, z : bool, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : bool, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set from the elements of `self`.
    ///
    ///A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn bitmask (self, ) -> u32;"#,
    r#"
    ///Returns true if any of the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn any (self, ) -> bool;"#,
    r#"
    ///Returns true if all the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn all (self, ) -> bool;"#,
    r#"
    ///Tests the value at `index`.
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",)]
    fn test (&self, index : usize, ) -> bool;"#,
    r#"
    ///Sets the element at `index`.
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="MutatingMethod",)]
    fn set (&mut self, index : usize, value : bool, );"#,
    ]
)]
pub struct BVec3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::bool::BVec4",
    functions[r#"
    ///Creates a new vector mask.
    #[lua(kind="Function",output(proxy))]
    fn new (x : bool, y : bool, z : bool, w : bool, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : bool, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    ///A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn bitmask (self, ) -> u32;"#,
    r#"
    ///Returns true if any of the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn any (self, ) -> bool;"#,
    r#"
    ///Returns true if all the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn all (self, ) -> bool;"#,
    r#"
    ///Tests the value at `index`.
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",)]
    fn test (&self, index : usize, ) -> bool;"#,
    r#"
    ///Sets the element at `index`.
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="MutatingMethod",)]
    fn set (&mut self, index : usize, value : bool, );"#,
    ]
)]
pub struct BVec4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::bool::BVec3A",
    functions[r#"
    ///Creates a new vector mask.
    #[lua(kind="Function",output(proxy))]
    fn new (x : bool, y : bool, z : bool, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : bool, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set from the elements of `self`.
    ///
    ///A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn bitmask (self, ) -> u32;"#,
    r#"
    ///Returns true if any of the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn any (self, ) -> bool;"#,
    r#"
    ///Returns true if all the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn all (self, ) -> bool;"#,
    r#"
    ///Tests the value at `index`.
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",)]
    fn test (&self, index : usize, ) -> bool;"#,
    r#"
    ///Sets the element at `index`.
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="MutatingMethod",)]
    fn set (&mut self, index : usize, value : bool, );"#,
    ]
)]
pub struct BVec3A;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::bool::BVec4A",
    functions[r#"
    ///Creates a new vector mask.
    #[lua(kind="Function",output(proxy))]
    fn new (x : bool, y : bool, z : bool, w : bool, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : bool, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    ///A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn bitmask (self, ) -> u32;"#,
    r#"
    ///Returns true if any of the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn any (self, ) -> bool;"#,
    r#"
    ///Returns true if all the elements are true, false otherwise.
    #[lua(kind="Method",)]
    fn all (self, ) -> bool;"#,
    r#"
    ///Tests the value at `index`.
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",)]
    fn test (&self, index : usize, ) -> bool;"#,
    r#"
    ///Sets the element at `index`.
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="MutatingMethod",)]
    fn set (&mut self, index : usize, value : bool, );"#,
    ]
)]
pub struct BVec4A;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DVec2",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f64, y : f64, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f64, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec2,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 3D vector from `self` and the given `z` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, z : f64, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f64::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f64;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f64;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f64;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f64;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f64;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f64, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f64, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f64, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    ///conjunction with the [`rotate()`][Self::rotate()] method, e.g.
    ///`DVec2::from_angle(PI).rotate(DVec2::Y)` will create the vector `[-1, 0]`
    ///and rotate [`DVec2::Y`] around it returning `-DVec2::Y`.
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f64, ) -> Self;"#,
    r#"
    ///Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
    ///
    ///The inputs do not need to be unit vectors however they must be non-zero.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns a vector that is equal to `self` rotated by 90 degrees.
    #[lua(kind="Method",output(proxy))]
    fn perp (self, ) -> Self;"#,
    r#"
    ///The perpendicular dot product of `self` and `rhs`.
    ///Also known as the wedge product, 2D cross product, and determinant.
    #[lua(kind="Method",)]
    fn perp_dot (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    ///then this just rotation. This is what you usually want. Otherwise,
    ///it will be like a rotation with a multiplication by `self`'s length.
    #[lua(kind="Method",output(proxy))]
    fn rotate (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec2 (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec2 (&self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec2 (&self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f64::DVec4;"#,
    ]
)]
pub struct DVec2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DVec3",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f64, y : f64, z : f64, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f64, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec3,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 4D vector from `self` and the given `w` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, w : f64, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    ///Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Computes the cross product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn cross (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f64::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f64;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f64;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f64;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f64;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f64;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f64, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f64, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f64, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Returns the angle (in radians) between two vectors.
    ///
    ///The inputs do not need to be unit vectors however they must be non-zero.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns some vector that is orthogonal to the given one.
    ///
    ///The input vector must be finite and non-zero.
    ///
    ///The output vector is not necessarily unit length. For that use
    ///[`Self::any_orthonormal_vector()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn any_orthogonal_vector (&self, ) -> Self;"#,
    r#"
    ///Returns any unit vector that is orthogonal to the given one.
    ///
    ///The input vector must be unit length.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn any_orthonormal_vector (&self, ) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3 (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3a (&self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec3 (&self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec3 (&self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::f64::DVec4;"#,
    ]
)]
pub struct DVec3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DVec4",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : f64, y : f64, z : f64, w : f64, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : f64, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec4,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    ///Truncation to [`DVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`f64::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> f64;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> f64;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    ///- `1.0` if the number is positive, `+0.0` or `INFINITY`
    ///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    ///- `NAN` if the number is `NAN`
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[lua(kind="Method",output(proxy))]
    fn copysign (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.  If any element is either
    ///`NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    ///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[lua(kind="Method",output(proxy))]
    fn is_nan_mask (self, ) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f64;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is faster than `length()` as it avoids a square root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f64;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f64;"#,
    r#"
    ///Computes the Euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    ///See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    ///In particular, if the input is zero (or very close to zero), or non-finite,
    ///the result of this operation will be zero.
    ///
    ///See also [`Self::try_normalize()`].
    #[lua(kind="Method",output(proxy))]
    fn normalize_or_zero (self, ) -> Self;"#,
    r#"
    ///Returns whether `self` is length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be of non-zero length.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector projection of `self` onto `rhs`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn project_onto_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns the vector rejection of `self` from `rhs`.
    ///
    ///The vector rejection is the vector perpendicular to the projection of `self` onto
    ///`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    ///`rhs` must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn reject_from_normalized (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the nearest integer to a number for each element of `self`.
    ///Round half-way cases away from 0.0.
    #[lua(kind="Method",output(proxy))]
    fn round (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the largest integer less than or equal to a number for each
    ///element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn floor (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the smallest integer greater than or equal to a number for
    ///each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn ceil (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the integer part each element of `self`. This means numbers are
    ///always truncated towards zero.
    #[lua(kind="Method",output(proxy))]
    fn trunc (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing the fractional part of the vector, e.g. `self -
    ///self.floor()`.
    ///
    ///Note that this is fast but not precise for large numbers.
    #[lua(kind="Method",output(proxy))]
    fn fract (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing `e^self` (the exponential function) for each element of
    ///`self`.
    #[lua(kind="Method",output(proxy))]
    fn exp (self, ) -> Self;"#,
    r#"
    ///Returns a vector containing each element of `self` raised to the power of `n`.
    #[lua(kind="Method",output(proxy))]
    fn powf (self, n : f64, ) -> Self;"#,
    r#"
    ///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn recip (self, ) -> Self;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    ///will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    ///extrapolated.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] rhs : Self,s : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs` is
    ///less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two vectors contain similar elements. It works best when
    ///comparing with a known value. The `max_abs_diff` that should be used used depends on
    ///the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Returns a vector with a length no less than `min` and no more than `max`
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp_length (self, min : f64, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no more than `max`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_max (self, max : f64, ) -> Self;"#,
    r#"
    ///Returns a vector with a length no less than `min`
    #[lua(kind="Method",output(proxy))]
    fn clamp_length_min (self, min : f64, ) -> Self;"#,
    r#"
    ///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    ///error, yielding a more accurate result than an unfused multiply-add.
    ///
    ///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    ///architecture has a dedicated fma CPU instruction. However, this is not always true,
    ///and will be heavily dependant on designing algorithms with specific target hardware in
    ///mind.
    #[lua(kind="Method",output(proxy))]
    fn mul_add (self, #[proxy] a : Self,#[proxy] b : Self,) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec4 (&self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec4 (&self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec4 (&self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xw (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yw (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zw (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wx (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wy (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wz (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ww (self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xww (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yww (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zww (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzw (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwx (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwy (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn www (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzww (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzw (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwx (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwy (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwz (self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwww (self, ) -> bevy::math::f64::DVec4;"#,
    ]
)]
pub struct DVec4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::i32::IVec2",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : i32, y : i32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : i32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec2,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 3D vector from `self` and the given `z` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, z : i32, ) -> bevy::math::i32::IVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`i32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> i32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> i32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    /// - `0` if the number is zero
    /// - `1` if the number is positive
    /// - `-1` if the number is negative
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> i32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Returns a vector that is equal to `self` rotated by 90 degrees.
    #[lua(kind="Method",output(proxy))]
    fn perp (self, ) -> Self;"#,
    r#"
    ///The perpendicular dot product of `self` and `rhs`.
    ///Also known as the wedge product, 2D cross product, and determinant.
    #[lua(kind="Method",)]
    fn perp_dot (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    ///then this just rotation. This is what you usually want. Otherwise,
    ///it will be like a rotation with a multiplication by `self`'s length.
    #[lua(kind="Method",output(proxy))]
    fn rotate (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec2 (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec2 (&self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec2 (&self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::i32::IVec4;"#,
    ]
)]
pub struct IVec2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::i32::IVec3",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : i32, y : i32, z : i32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : i32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec3,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 4D vector from `self` and the given `w` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, w : i32, ) -> bevy::math::i32::IVec4;"#,
    r#"
    ///Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    ///Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Computes the cross product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn cross (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`i32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> i32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> i32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    /// - `0` if the number is zero
    /// - `1` if the number is positive
    /// - `-1` if the number is negative
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> i32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3 (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3a (&self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec3 (&self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec3 (&self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::i32::IVec4;"#,
    ]
)]
pub struct IVec3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::i32::IVec4",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : i32, y : i32, z : i32, w : i32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : i32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec4,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    ///Truncation to [`IVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`i32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> i32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> i32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector containing the absolute value of each element of `self`.
    #[lua(kind="Method",output(proxy))]
    fn abs (self, ) -> Self;"#,
    r#"
    ///Returns a vector with elements representing the sign of `self`.
    ///
    /// - `0` if the number is zero
    /// - `1` if the number is positive
    /// - `-1` if the number is negative
    #[lua(kind="Method",output(proxy))]
    fn signum (self, ) -> Self;"#,
    r#"
    ///Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
    ///
    ///A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    ///into the first lowest bit, element `y` into the second, etc.
    #[lua(kind="Method",)]
    fn is_negative_bitmask (self, ) -> u32;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> i32;"#,
    r#"
    ///Compute the squared euclidean distance between two points in space.
    #[lua(kind="Method",)]
    fn distance_squared (self, #[proxy] rhs : Self,) -> i32;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec4 (&self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec4 (&self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Casts all elements of `self` to `u32`.
    #[lua(kind="Method",output(proxy))]
    fn as_uvec4 (&self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xw (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yw (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zw (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wx (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wy (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wz (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ww (self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xww (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yww (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zww (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzw (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwx (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwy (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwz (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn www (self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzww (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzw (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwx (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwy (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwz (self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwww (self, ) -> bevy::math::i32::IVec4;"#,
    ]
)]
pub struct IVec4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::u32::UVec2",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : u32, y : u32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : u32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec2,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 3D vector from `self` and the given `z` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, z : u32, ) -> bevy::math::u32::UVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> u32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`u32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> u32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> u32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec2;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> u32;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec2 (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec2 (&self, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec2 (&self, ) -> bevy::math::i32::IVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::Vec2Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::u32::UVec4;"#,
    ]
)]
pub struct UVec2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::u32::UVec3",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : u32, y : u32, z : u32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : u32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec3,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 4D vector from `self` and the given `w` value.
    #[lua(kind="Method",output(proxy))]
    fn extend (self, w : u32, ) -> bevy::math::u32::UVec4;"#,
    r#"
    ///Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    ///Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> u32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Computes the cross product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn cross (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`u32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> u32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> u32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec3;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> u32;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3 (&self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec3a (&self, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec3 (&self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec3 (&self, ) -> bevy::math::i32::IVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec3Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::u32::UVec4;"#,
    ]
)]
pub struct UVec3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::u32::UVec4",
    functions[r#"
    ///Creates a new vector.
    #[lua(kind="Function",output(proxy))]
    fn new (x : u32, y : u32, z : u32, w : u32, ) -> Self;"#,
    r#"
    ///Creates a vector with all elements set to `v`.
    #[lua(kind="Function",output(proxy))]
    fn splat (v : u32, ) -> Self;"#,
    r#"
    ///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    ///for each element of `self`.
    ///
    ///A true element in the mask uses the corresponding element from `if_true`, and false
    ///uses the element from `if_false`.
    #[lua(kind="Function",output(proxy))]
    fn select (#[proxy] mask : bevy::math::bool::BVec4,#[proxy] if_true : Self,#[proxy] if_false : Self,) -> Self;"#,
    r#"
    ///Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    ///Truncation to [`UVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    #[lua(kind="Method",output(proxy))]
    fn truncate (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> u32;"#,
    r#"
    ///Returns a vector where every component is the dot product of `self` and `rhs`.
    #[lua(kind="Method",output(proxy))]
    fn dot_into_vec (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn min (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    ///In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[lua(kind="Method",output(proxy))]
    fn max (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Component-wise clamping of values, similar to [`u32::clamp`].
    ///
    ///Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    ///# Panics
    ///
    ///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn clamp (self, #[proxy] min : Self,#[proxy] max : Self,) -> Self;"#,
    r#"
    ///Returns the horizontal minimum of `self`.
    ///
    ///In other words this computes `min(x, y, ..)`.
    #[lua(kind="Method",)]
    fn min_element (self, ) -> u32;"#,
    r#"
    ///Returns the horizontal maximum of `self`.
    ///
    ///In other words this computes `max(x, y, ..)`.
    #[lua(kind="Method",)]
    fn max_element (self, ) -> u32;"#,
    r#"
    ///Returns a vector mask containing the result of a `==` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpeq (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `!=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpne (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpge (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `>` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmpgt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<=` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmple (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Returns a vector mask containing the result of a `<` comparison for each element of
    ///`self` and `rhs`.
    ///
    ///In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    ///elements.
    #[lua(kind="Method",output(proxy))]
    fn cmplt (self, #[proxy] rhs : Self,) -> bevy::math::bool::BVec4;"#,
    r#"
    ///Computes the squared length of `self`.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> u32;"#,
    r#"
    ///Casts all elements of `self` to `f32`.
    #[lua(kind="Method",output(proxy))]
    fn as_vec4 (&self, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Casts all elements of `self` to `f64`.
    #[lua(kind="Method",output(proxy))]
    fn as_dvec4 (&self, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Casts all elements of `self` to `i32`.
    #[lua(kind="Method",output(proxy))]
    fn as_ivec4 (&self, ) -> bevy::math::i32::IVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xw (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yw (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zw (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wx (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wy (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wz (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ww (self, ) -> bevy::math::u32::UVec2;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xww (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yww (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zww (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzw (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwx (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwy (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwz (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn www (self, ) -> bevy::math::u32::UVec3;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xxww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xywz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xyww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xzww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn xwww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yxww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yywz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yyww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn yzww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn ywww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zxww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zywz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zyww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zzww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn zwww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wxww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wywz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wyww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wzww (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwxw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwyw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwzw (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwx (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwy (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwwz (self, ) -> bevy::math::u32::UVec4;"#,
    r#"
    #[lua(kind="Method",as_trait="bevy::math::swizzles::Vec4Swizzles",output(proxy))]
    fn wwww (self, ) -> bevy::math::u32::UVec4;"#,
    ]
)]
pub struct UVec4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Mat3",
    functions[r#"
    ///Creates a 3x3 matrix from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec3,#[proxy] y_axis : bevy::math::f32::Vec3,#[proxy] z_axis : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] m : bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given quaternion.
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    ///radians).
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f32::Vec3,angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    ///radians).
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] order : bevy::math::EulerRot,a : f32, b : f32, c : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D rotation `angle` (in
    ///radians).
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    ///radians) and `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle_translation (#[proxy] scale : bevy::math::f32::Vec2,angle : f32, #[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    ///# Panics
    ///
    ///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat2 (#[proxy] m : bevy::math::f32::Mat2,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f32;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Transforms the given 2D vector as a point.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_point2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Rotates the given 2D vector.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Transforms a 3D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms a [`Vec3A`].
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Multiplies a 3x3 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_dmat3 (&self, ) -> bevy::math::f64::DMat3;"#,
    ]
)]
pub struct Mat3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Mat2",
    functions[r#"
    ///Creates a 2x2 matrix from two column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec2,#[proxy] y_axis : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle (#[proxy] scale : bevy::math::f32::Vec2,angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix containing a rotation of `angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f32::Mat3,) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3a (#[proxy] m : bevy::math::f32::Mat3A,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f32;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Transforms a 2D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Multiplies a 2x2 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_dmat2 (&self, ) -> bevy::math::f64::DMat2;"#,
    ]
)]
pub struct Mat2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Mat3A",
    functions[r#"
    ///Creates a 3x3 matrix from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec3A,#[proxy] y_axis : bevy::math::f32::Vec3A,#[proxy] z_axis : bevy::math::f32::Vec3A,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] m : bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given quaternion.
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    ///radians).
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f32::Vec3,angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    ///radians).
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] order : bevy::math::EulerRot,a : f32, b : f32, c : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D rotation `angle` (in
    ///radians).
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    ///radians) and `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle_translation (#[proxy] scale : bevy::math::f32::Vec2,angle : f32, #[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    ///# Panics
    ///
    ///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat2 (#[proxy] m : bevy::math::f32::Mat2,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f32;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Transforms the given 2D vector as a point.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_point2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Rotates the given 2D vector.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Transforms a 3D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms a [`Vec3A`].
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Multiplies a 3x3 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_dmat3 (&self, ) -> bevy::math::f64::DMat3;"#,
    ]
)]
pub struct Mat3A;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Mat4",
    functions[r#"
    ///Creates a 4x4 matrix from four column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec4,#[proxy] y_axis : bevy::math::f32::Vec4,#[proxy] z_axis : bevy::math::f32::Vec4,#[proxy] w_axis : bevy::math::f32::Vec4,) -> Self;"#,
    r#"
    ///Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f32::Vec4,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    ///`translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale_rotation_translation (#[proxy] scale : bevy::math::f32::Vec3,#[proxy] rotation : bevy::math::f32::Quat,#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_translation (#[proxy] rotation : bevy::math::f32::Quat,#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3x3 linear transformation
    ///matrix.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f32::Mat3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3x3 linear transformation
    ///matrix.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat3a (#[proxy] m : bevy::math::f32::Mat3A,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around a normalized
    ///rotation `axis` of `angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f32::Vec3,angle : f32, ) -> Self;"#,
    r#"
    ///Creates a affine transformation matrix containing a rotation from the given euler
    ///rotation sequence and angles (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] order : bevy::math::EulerRot,a : f32, b : f32, c : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the x axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the y axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the z axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f32;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Creates a left-handed view matrix using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_lh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] dir : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view matrix using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_rh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] dir : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a left-handed view matrix using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_lh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] center : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view matrix using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_rh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] center : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a right-handed perspective projection matrix with [-1,1] depth range.
    ///This is the same as the OpenGL `gluPerspective` function.
    ///See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
    #[lua(kind="Function",output(proxy))]
    fn perspective_rh_gl (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, z_far : f32, ) -> Self;"#,
    r#"
    ///Creates a left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    ///enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_lh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, z_far : f32, ) -> Self;"#,
    r#"
    ///Creates a right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    ///enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_rh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, z_far : f32, ) -> Self;"#,
    r#"
    ///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_lh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, ) -> Self;"#,
    r#"
    ///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_reverse_lh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, ) -> Self;"#,
    r#"
    ///Creates an infinite right-handed perspective projection matrix with
    ///`[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_rh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, ) -> Self;"#,
    r#"
    ///Creates an infinite reverse right-handed perspective projection matrix
    ///with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_reverse_rh (fov_y_radians : f32, aspect_ratio : f32, z_near : f32, ) -> Self;"#,
    r#"
    ///Creates a right-handed orthographic projection matrix with `[-1,1]` depth
    ///range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    ///See
    ///<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
    #[lua(kind="Function",output(proxy))]
    fn orthographic_rh_gl (left : f32, right : f32, bottom : f32, top : f32, near : f32, far : f32, ) -> Self;"#,
    r#"
    ///Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn orthographic_lh (left : f32, right : f32, bottom : f32, top : f32, near : f32, far : f32, ) -> Self;"#,
    r#"
    ///Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn orthographic_rh (left : f32, right : f32, bottom : f32, top : f32, near : f32, far : f32, ) -> Self;"#,
    r#"
    ///Transforms the given 3D vector as a point, applying perspective correction.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    ///The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    ///This method assumes that `self` contains a projective transform.
    #[lua(kind="Method",output(proxy))]
    fn project_point3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms the given 3D vector as a point.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    ///`1.0`.
    ///
    ///This method assumes that `self` contains a valid affine transform. It does not perform
    ///a persective divide, if `self` contains a perspective transform, or if you are unsure,
    ///the [`Self::project_point3()`] method should be used instead.
    ///
    ///# Panics
    ///
    ///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms the give 3D vector as a direction.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    ///`0.0`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms the given [`Vec3A`] as 3D point.
    ///
    ///This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `1.0`.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Transforms the give [`Vec3A`] as 3D vector.
    ///
    ///This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `0.0`.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Transforms a 4D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec4 (&self, #[proxy] rhs : bevy::math::f32::Vec4,) -> bevy::math::f32::Vec4;"#,
    r#"
    ///Multiplies a 4x4 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f32, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_dmat4 (&self, ) -> bevy::math::f64::DMat4;"#,
    ]
)]
pub struct Mat4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DMat2",
    functions[r#"
    ///Creates a 2x2 matrix from two column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f64::DVec2,#[proxy] y_axis : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle (#[proxy] scale : bevy::math::f64::DVec2,angle : f64, ) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix containing a rotation of `angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f64::DMat3,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 1.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f64;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Transforms a 2D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec2 (&self, #[proxy] rhs : bevy::math::f64::DVec2,) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Multiplies a 2x2 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_mat2 (&self, ) -> bevy::math::f32::Mat2;"#,
    ]
)]
pub struct DMat2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DMat3",
    functions[r#"
    ///Creates a 3x3 matrix from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f64::DVec3,#[proxy] y_axis : bevy::math::f64::DVec3,#[proxy] z_axis : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] m : bevy::math::f64::DMat4,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given quaternion.
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f64::DQuat,) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    ///radians).
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f64::DVec3,angle : f64, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    ///radians).
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] order : bevy::math::EulerRot,a : f64, b : f64, c : f64, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D rotation `angle` (in
    ///radians).
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    ///radians) and `translation`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle_translation (#[proxy] scale : bevy::math::f64::DVec2,angle : f64, #[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    ///# Panics
    ///
    ///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    ///The resulting matrix can be used to transform 2D points and vectors. See
    ///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat2 (#[proxy] m : bevy::math::f64::DMat2,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 2.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f64;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Transforms the given 2D vector as a point.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_point2 (&self, #[proxy] rhs : bevy::math::f64::DVec2,) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Rotates the given 2D vector.
    ///
    ///This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector2 (&self, #[proxy] rhs : bevy::math::f64::DVec2,) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Transforms a 3D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Multiplies a 3x3 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_mat3 (&self, ) -> bevy::math::f32::Mat3;"#,
    ]
)]
pub struct DMat3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DMat4",
    functions[r#"
    ///Creates a 4x4 matrix from four column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f64::DVec4,#[proxy] y_axis : bevy::math::f64::DVec4,#[proxy] z_axis : bevy::math::f64::DVec4,#[proxy] w_axis : bevy::math::f64::DVec4,) -> Self;"#,
    r#"
    ///Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[lua(kind="Function",output(proxy))]
    fn from_diagonal (#[proxy] diagonal : bevy::math::f64::DVec4,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    ///`translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale_rotation_translation (#[proxy] scale : bevy::math::f64::DVec3,#[proxy] rotation : bevy::math::f64::DQuat,#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_translation (#[proxy] rotation : bevy::math::f64::DQuat,#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f64::DQuat,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3x3 linear transformation
    ///matrix.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f64::DMat3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix from the given 3D `translation`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around a normalized
    ///rotation `axis` of `angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f64::DVec3,angle : f64, ) -> Self;"#,
    r#"
    ///Creates a affine transformation matrix containing a rotation from the given euler
    ///rotation sequence and angles (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] order : bevy::math::EulerRot,a : f64, b : f64, c : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the x axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the y axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing a 3D rotation around the z axis of
    ///`angle` (in radians).
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
    ///
    ///The resulting matrix can be used to transform 3D points and vectors. See
    ///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    ///# Panics
    ///
    ///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Returns the matrix column for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",output(proxy))]
    fn col (&self, index : usize, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Returns the matrix row for the given `index`.
    ///
    ///# Panics
    ///
    ///Panics if `index` is greater than 3.
    #[lua(kind="Method",output(proxy))]
    fn row (&self, index : usize, ) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns the transpose of `self`.
    #[lua(kind="Method",output(proxy))]
    fn transpose (&self, ) -> Self;"#,
    r#"
    ///Returns the determinant of `self`.
    #[lua(kind="Method",)]
    fn determinant (&self, ) -> f64;"#,
    r#"
    ///Returns the inverse of `self`.
    ///
    ///If the matrix is not invertible the returned matrix will be invalid.
    ///
    ///# Panics
    ///
    ///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    r#"
    ///Creates a left-handed view matrix using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_lh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] dir : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view matrix using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_rh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] dir : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a left-handed view matrix using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_lh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] center : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view matrix using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_rh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] center : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a right-handed perspective projection matrix with [-1,1] depth range.
    ///This is the same as the OpenGL `gluPerspective` function.
    ///See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
    #[lua(kind="Function",output(proxy))]
    fn perspective_rh_gl (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, z_far : f64, ) -> Self;"#,
    r#"
    ///Creates a left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    ///enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_lh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, z_far : f64, ) -> Self;"#,
    r#"
    ///Creates a right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    ///enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_rh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, z_far : f64, ) -> Self;"#,
    r#"
    ///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_lh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, ) -> Self;"#,
    r#"
    ///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    ///# Panics
    ///
    ///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_reverse_lh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, ) -> Self;"#,
    r#"
    ///Creates an infinite right-handed perspective projection matrix with
    ///`[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_rh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, ) -> Self;"#,
    r#"
    ///Creates an infinite reverse right-handed perspective projection matrix
    ///with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn perspective_infinite_reverse_rh (fov_y_radians : f64, aspect_ratio : f64, z_near : f64, ) -> Self;"#,
    r#"
    ///Creates a right-handed orthographic projection matrix with `[-1,1]` depth
    ///range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    ///See
    ///<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
    #[lua(kind="Function",output(proxy))]
    fn orthographic_rh_gl (left : f64, right : f64, bottom : f64, top : f64, near : f64, far : f64, ) -> Self;"#,
    r#"
    ///Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn orthographic_lh (left : f64, right : f64, bottom : f64, top : f64, near : f64, far : f64, ) -> Self;"#,
    r#"
    ///Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
    #[lua(kind="Function",output(proxy))]
    fn orthographic_rh (left : f64, right : f64, bottom : f64, top : f64, near : f64, far : f64, ) -> Self;"#,
    r#"
    ///Transforms the given 3D vector as a point, applying perspective correction.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    ///The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    ///This method assumes that `self` contains a projective transform.
    #[lua(kind="Method",output(proxy))]
    fn project_point3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Transforms the given 3D vector as a point.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    ///`1.0`.
    ///
    ///This method assumes that `self` contains a valid affine transform. It does not perform
    ///a persective divide, if `self` contains a perspective transform, or if you are unsure,
    ///the [`Self::project_point3()`] method should be used instead.
    ///
    ///# Panics
    ///
    ///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Transforms the give 3D vector as a direction.
    ///
    ///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    ///`0.0`.
    ///
    ///This method assumes that `self` contains a valid affine transform.
    ///
    ///# Panics
    ///
    ///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Transforms a 4D vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec4 (&self, #[proxy] rhs : bevy::math::f64::DVec4,) -> bevy::math::f64::DVec4;"#,
    r#"
    ///Multiplies a 4x4 matrix by a scalar.
    #[lua(kind="Method",output(proxy))]
    fn mul_scalar (&self, rhs : f64, ) -> Self;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two matrices contain similar elements. It works best
    ///when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_mat4 (&self, ) -> bevy::math::f32::Mat4;"#,
    ]
)]
pub struct DMat4;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Affine2",
    functions[r#"
    ///Creates an affine transform from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec2,#[proxy] y_axis : bevy::math::f32::Vec2,#[proxy] z_axis : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transform that changes scale.
    ///Note that if any scale is zero the transform will be non-invertible.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given rotation `angle`.
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation from the given 2D `translation`.
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
    #[lua(kind="Function",output(proxy))]
    fn from_mat2 (#[proxy] matrix2 : bevy::math::f32::Mat2,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
    ///translation vector.
    ///
    ///Equivalent to
    ///`Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`
    #[lua(kind="Function",output(proxy))]
    fn from_mat2_translation (#[proxy] matrix2 : bevy::math::f32::Mat2,#[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
    ///`translation`.
    ///
    ///Equivalent to `Affine2::from_translation(translation) *
    ///Affine2::from_angle(angle) * Affine2::from_scale(scale)`
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle_translation (#[proxy] scale : bevy::math::f32::Vec2,angle : f32, #[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 2D rotation `angle` (in radians) and
    ///`translation`.
    ///
    ///Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`
    #[lua(kind="Function",output(proxy))]
    fn from_angle_translation (angle : f32, #[proxy] translation : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///The given `Mat3` must be an affine transform,
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f32::Mat3,) -> Self;"#,
    r#"
    ///The given [`Mat3A`] must be an affine transform,
    #[lua(kind="Function",output(proxy))]
    fn from_mat3a (#[proxy] m : bevy::math::f32::Mat3A,) -> Self;"#,
    r#"
    ///Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[lua(kind="Method",output(proxy))]
    fn transform_point2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    ///translation).
    ///
    ///To also apply translation, use [`Self::transform_point2()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector2 (&self, #[proxy] rhs : bevy::math::f32::Vec2,) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///
    ///If any element is either `NaN`, positive or negative infinity, this will return
    ///`false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two 3x4 matrices contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Return the inverse of this transform.
    ///
    ///Note that if the transform is not invertible the result will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    ]
)]
pub struct Affine2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Affine3A",
    functions[r#"
    ///Creates an affine transform from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f32::Vec3A,#[proxy] y_axis : bevy::math::f32::Vec3A,#[proxy] z_axis : bevy::math::f32::Vec3A,#[proxy] w_axis : bevy::math::f32::Vec3A,) -> Self;"#,
    r#"
    ///Creates an affine transform that changes scale.
    ///Note that if any scale is zero the transform will be non-invertible.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given `rotation` quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f32::Quat,) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around a normalized
    ///rotation `axis` of `angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f32::Vec3,angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the x axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the y axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the z axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f32, ) -> Self;"#,
    r#"
    ///Creates an affine transformation from the given 3D `translation`.
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 3x3 matrix (expressing scale, shear and
    ///rotation)
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] mat3 : bevy::math::f32::Mat3,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
    ///and a translation vector.
    ///
    ///Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`
    #[lua(kind="Function",output(proxy))]
    fn from_mat3_translation (#[proxy] mat3 : bevy::math::f32::Mat3,#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 3D `scale`, `rotation` and
    ///`translation`.
    ///
    ///Equivalent to `Affine3A::from_translation(translation) *
    ///Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`
    #[lua(kind="Function",output(proxy))]
    fn from_scale_rotation_translation (#[proxy] scale : bevy::math::f32::Vec3,#[proxy] rotation : bevy::math::f32::Quat,#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 3D `rotation` and `translation`.
    ///
    ///Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_translation (#[proxy] rotation : bevy::math::f32::Quat,#[proxy] translation : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///The given `Mat4` must be an affine transform,
    ///i.e. contain no perspective transform.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] m : bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Creates a left-handed view transform using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_lh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] dir : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view transform using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_rh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] dir : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a left-handed view transform using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_lh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] center : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view transform using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_rh (#[proxy] eye : bevy::math::f32::Vec3,#[proxy] center : bevy::math::f32::Vec3,#[proxy] up : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Transforms the given 3D points, applying shear, scale, rotation and translation.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    ///translation).
    ///
    ///To also apply translation, use [`Self::transform_point3()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3 (&self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Transforms the given [`Vec3A`], applying shear, scale, rotation and translation.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Transforms the given [`Vec3A`], applying shear, scale and rotation (but NOT
    ///translation).
    ///
    ///To also apply translation, use [`Self::transform_point3a()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3a (&self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///
    ///If any element is either `NaN`, positive or negative infinity, this will return
    ///`false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two 3x4 matrices contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Return the inverse of this transform.
    ///
    ///Note that if the transform is not invertible the result will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    ]
)]
pub struct Affine3A;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DAffine2",
    functions[r#"
    ///Creates an affine transform from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f64::DVec2,#[proxy] y_axis : bevy::math::f64::DVec2,#[proxy] z_axis : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transform that changes scale.
    ///Note that if any scale is zero the transform will be non-invertible.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given rotation `angle`.
    #[lua(kind="Function",output(proxy))]
    fn from_angle (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation from the given 2D `translation`.
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
    #[lua(kind="Function",output(proxy))]
    fn from_mat2 (#[proxy] matrix2 : bevy::math::f64::DMat2,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
    ///translation vector.
    ///
    ///Equivalent to
    ///`DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`
    #[lua(kind="Function",output(proxy))]
    fn from_mat2_translation (#[proxy] matrix2 : bevy::math::f64::DMat2,#[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
    ///`translation`.
    ///
    ///Equivalent to `DAffine2::from_translation(translation) *
    ///DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`
    #[lua(kind="Function",output(proxy))]
    fn from_scale_angle_translation (#[proxy] scale : bevy::math::f64::DVec2,angle : f64, #[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 2D rotation `angle` (in radians) and
    ///`translation`.
    ///
    ///Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`
    #[lua(kind="Function",output(proxy))]
    fn from_angle_translation (angle : f64, #[proxy] translation : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///The given `DMat3` must be an affine transform,
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] m : bevy::math::f64::DMat3,) -> Self;"#,
    r#"
    ///Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[lua(kind="Method",output(proxy))]
    fn transform_point2 (&self, #[proxy] rhs : bevy::math::f64::DVec2,) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    ///translation).
    ///
    ///To also apply translation, use [`Self::transform_point2()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector2 (&self, #[proxy] rhs : bevy::math::f64::DVec2,) -> bevy::math::f64::DVec2;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///
    ///If any element is either `NaN`, positive or negative infinity, this will return
    ///`false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two 3x4 matrices contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Return the inverse of this transform.
    ///
    ///Note that if the transform is not invertible the result will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    ]
)]
pub struct DAffine2;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DAffine3",
    functions[r#"
    ///Creates an affine transform from three column vectors.
    #[lua(kind="Function",output(proxy))]
    fn from_cols (#[proxy] x_axis : bevy::math::f64::DVec3,#[proxy] y_axis : bevy::math::f64::DVec3,#[proxy] z_axis : bevy::math::f64::DVec3,#[proxy] w_axis : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transform that changes scale.
    ///Note that if any scale is zero the transform will be non-invertible.
    #[lua(kind="Function",output(proxy))]
    fn from_scale (#[proxy] scale : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given `rotation` quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_quat (#[proxy] rotation : bevy::math::f64::DQuat,) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around a normalized
    ///rotation `axis` of `angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f64::DVec3,angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the x axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the y axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transform containing a 3D rotation around the z axis of
    ///`angle` (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f64, ) -> Self;"#,
    r#"
    ///Creates an affine transformation from the given 3D `translation`.
    #[lua(kind="Function",output(proxy))]
    fn from_translation (#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 3x3 matrix (expressing scale, shear and
    ///rotation)
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] mat3 : bevy::math::f64::DMat3,) -> Self;"#,
    r#"
    ///Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
    ///and a translation vector.
    ///
    ///Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`
    #[lua(kind="Function",output(proxy))]
    fn from_mat3_translation (#[proxy] mat3 : bevy::math::f64::DMat3,#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 3D `scale`, `rotation` and
    ///`translation`.
    ///
    ///Equivalent to `DAffine3::from_translation(translation) *
    ///DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`
    #[lua(kind="Function",output(proxy))]
    fn from_scale_rotation_translation (#[proxy] scale : bevy::math::f64::DVec3,#[proxy] rotation : bevy::math::f64::DQuat,#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates an affine transform from the given 3D `rotation` and `translation`.
    ///
    ///Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_translation (#[proxy] rotation : bevy::math::f64::DQuat,#[proxy] translation : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///The given `DMat4` must be an affine transform,
    ///i.e. contain no perspective transform.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] m : bevy::math::f64::DMat4,) -> Self;"#,
    r#"
    ///Creates a left-handed view transform using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_lh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] dir : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view transform using a camera position, an up direction, and a facing
    ///direction.
    ///
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[lua(kind="Function",output(proxy))]
    fn look_to_rh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] dir : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a left-handed view transform using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_lh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] center : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a right-handed view transform using a camera position, an up direction, and a focal
    ///point.
    ///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    ///# Panics
    ///
    ///Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn look_at_rh (#[proxy] eye : bevy::math::f64::DVec3,#[proxy] center : bevy::math::f64::DVec3,#[proxy] up : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Transforms the given 3D points, applying shear, scale, rotation and translation.
    #[lua(kind="Method",output(proxy))]
    fn transform_point3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    ///translation).
    ///
    ///To also apply translation, use [`Self::transform_point3()`] instead.
    #[lua(kind="Method",output(proxy))]
    fn transform_vector3 (&self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///
    ///If any element is either `NaN`, positive or negative infinity, this will return
    ///`false`.
    #[lua(kind="Method",)]
    fn is_finite (&self, ) -> bool;"#,
    r#"
    ///Returns `true` if any elements are `NaN`.
    #[lua(kind="Method",)]
    fn is_nan (&self, ) -> bool;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two 3x4 matrices contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (&self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Return the inverse of this transform.
    ///
    ///Note that if the transform is not invertible the result will be invalid.
    #[lua(kind="Method",output(proxy))]
    fn inverse (&self, ) -> Self;"#,
    ]
)]
pub struct DAffine3;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f32::Quat",
    functions[r#"
    ///Creates a new rotation quaternion.
    ///
    ///This should generally not be called manually unless you know what you are doing.
    ///Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    ///`from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    ///# Preconditions
    ///
    ///This function does not check if the input is normalized, it is up to the user to
    ///provide normalized input or to normalized the resulting quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_xyzw (x : f32, y : f32, z : f32, w : f32, ) -> Self;"#,
    r#"
    ///Creates a new rotation quaternion from a 4D vector.
    ///
    ///# Preconditions
    ///
    ///This function does not check if the input is normalized, it is up to the user to
    ///provide normalized input or to normalized the resulting quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_vec4 (#[proxy] v : bevy::math::f32::Vec4,) -> Self;"#,
    r#"
    ///Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    ///
    ///The axis must be a unit vector.
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f32::Vec3,angle : f32, ) -> Self;"#,
    r#"
    ///Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    ///`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_scaled_axis (#[proxy] v : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the x axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the y axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the z axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f32, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] euler : bevy::math::EulerRot,a : f32, b : f32, c : f32, ) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] mat : &bevy::math::f32::Mat3,) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 SIMD aligned rotation matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3a (#[proxy] mat : &bevy::math::f32::Mat3A,) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] mat : &bevy::math::f32::Mat4,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    ///plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`from_rotation_arc(from, to) * from ≈ to`.
    ///
    ///For near-singular cases (from≈to and from≈-to) the current implementation
    ///is only accurate to about 0.001 (for `f32`).
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc (#[proxy] from : bevy::math::f32::Vec3,#[proxy] to : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    ///that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    ///The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    ///degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc_colinear (#[proxy] from : bevy::math::f32::Vec3,#[proxy] to : bevy::math::f32::Vec3,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    ///around the z axis. Will rotate at most 180 degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    ///For near-singular cases (from≈to and from≈-to) the current implementation
    ///is only accurate to about 0.001 (for `f32`).
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc_2d (#[proxy] from : bevy::math::f32::Vec2,#[proxy] to : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Returns the rotation axis scaled by the rotation in radians.
    #[lua(kind="Method",output(proxy))]
    fn to_scaled_axis (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Returns the vector part of the quaternion.
    #[lua(kind="Method",output(proxy))]
    fn xyz (self, ) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Returns the quaternion conjugate of `self`. For a unit quaternion the
    ///conjugate is also the inverse.
    #[lua(kind="Method",output(proxy))]
    fn conjugate (self, ) -> Self;"#,
    r#"
    ///Returns the inverse of a normalized quaternion.
    ///
    ///Typically quaternion inverse returns the conjugate of a normalized quaternion.
    ///Because `self` is assumed to already be unit length this method *does not* normalize
    ///before returning the conjugate.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (self, ) -> Self;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`. The dot product is
    ///equal to the cosine of the angle between two quaternion rotations.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f32;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is generally faster than `length()` as it avoids a square
    ///root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f32;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f32;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Returns whether `self` of length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    #[lua(kind="Method",)]
    fn is_near_identity (self, ) -> bool;"#,
    r#"
    ///Returns the angle (in radians) for the minimal rotation
    ///for transforming this quaternion into another.
    ///
    ///Both quaternions must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f32;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two quaternions contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f32, ) -> bool;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on
    ///the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s`
    ///is `1.0`, the result will be equal to `rhs`.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] end : Self,s : f32, ) -> Self;"#,
    r#"
    ///Performs a spherical linear interpolation between `self` and `end`
    ///based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s`
    ///is `1.0`, the result will be equal to `end`.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn slerp (self, #[proxy] end : Self,s : f32, ) -> Self;"#,
    r#"
    ///Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3 (self, #[proxy] rhs : bevy::math::f32::Vec3,) -> bevy::math::f32::Vec3;"#,
    r#"
    ///Multiplies two quaternions. If they each represent a rotation, the result will
    ///represent the combined rotation.
    ///
    ///Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn mul_quat (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[lua(kind="Function",output(proxy))]
    fn from_affine3 (#[proxy] a : &bevy::math::f32::Affine3A,) -> Self;"#,
    r#"
    ///Multiplies a quaternion and a 3D vector, returning the rotated vector.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3a (self, #[proxy] rhs : bevy::math::f32::Vec3A,) -> bevy::math::f32::Vec3A;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_f64 (self, ) -> bevy::math::f64::DQuat;"#,
    ]
)]
pub struct Quat;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::f64::DQuat",
    functions[r#"
    ///Creates a new rotation quaternion.
    ///
    ///This should generally not be called manually unless you know what you are doing.
    ///Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    ///`from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    ///# Preconditions
    ///
    ///This function does not check if the input is normalized, it is up to the user to
    ///provide normalized input or to normalized the resulting quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_xyzw (x : f64, y : f64, z : f64, w : f64, ) -> Self;"#,
    r#"
    ///Creates a new rotation quaternion from a 4D vector.
    ///
    ///# Preconditions
    ///
    ///This function does not check if the input is normalized, it is up to the user to
    ///provide normalized input or to normalized the resulting quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_vec4 (#[proxy] v : bevy::math::f64::DVec4,) -> Self;"#,
    r#"
    ///Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    ///
    ///The axis must be a unit vector.
    ///
    ///# Panics
    ///
    ///Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_axis_angle (#[proxy] axis : bevy::math::f64::DVec3,angle : f64, ) -> Self;"#,
    r#"
    ///Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    ///`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[lua(kind="Function",output(proxy))]
    fn from_scaled_axis (#[proxy] v : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the x axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_x (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the y axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_y (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the `angle` (in radians) around the z axis.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_z (angle : f64, ) -> Self;"#,
    r#"
    ///Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    #[lua(kind="Function",output(proxy))]
    fn from_euler (#[proxy] euler : bevy::math::EulerRot,a : f64, b : f64, c : f64, ) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_mat3 (#[proxy] mat : &bevy::math::f64::DMat3,) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[lua(kind="Function",output(proxy))]
    fn from_mat4 (#[proxy] mat : &bevy::math::f64::DMat4,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    ///plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`from_rotation_arc(from, to) * from ≈ to`.
    ///
    ///For near-singular cases (from≈to and from≈-to) the current implementation
    ///is only accurate to about 0.001 (for `f32`).
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc (#[proxy] from : bevy::math::f64::DVec3,#[proxy] to : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    ///that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    ///The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    ///degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc_colinear (#[proxy] from : bevy::math::f64::DVec3,#[proxy] to : bevy::math::f64::DVec3,) -> Self;"#,
    r#"
    ///Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    ///around the z axis. Will rotate at most 180 degrees.
    ///
    ///The inputs must be unit vectors.
    ///
    ///`from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    ///For near-singular cases (from≈to and from≈-to) the current implementation
    ///is only accurate to about 0.001 (for `f32`).
    ///
    ///# Panics
    ///
    ///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Function",output(proxy))]
    fn from_rotation_arc_2d (#[proxy] from : bevy::math::f64::DVec2,#[proxy] to : bevy::math::f64::DVec2,) -> Self;"#,
    r#"
    ///Returns the rotation axis scaled by the rotation in radians.
    #[lua(kind="Method",output(proxy))]
    fn to_scaled_axis (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Returns the vector part of the quaternion.
    #[lua(kind="Method",output(proxy))]
    fn xyz (self, ) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Returns the quaternion conjugate of `self`. For a unit quaternion the
    ///conjugate is also the inverse.
    #[lua(kind="Method",output(proxy))]
    fn conjugate (self, ) -> Self;"#,
    r#"
    ///Returns the inverse of a normalized quaternion.
    ///
    ///Typically quaternion inverse returns the conjugate of a normalized quaternion.
    ///Because `self` is assumed to already be unit length this method *does not* normalize
    ///before returning the conjugate.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn inverse (self, ) -> Self;"#,
    r#"
    ///Computes the dot product of `self` and `rhs`. The dot product is
    ///equal to the cosine of the angle between two quaternion rotations.
    #[lua(kind="Method",)]
    fn dot (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Computes the length of `self`.
    #[lua(kind="Method",)]
    fn length (self, ) -> f64;"#,
    r#"
    ///Computes the squared length of `self`.
    ///
    ///This is generally faster than `length()` as it avoids a square
    ///root operation.
    #[lua(kind="Method",)]
    fn length_squared (self, ) -> f64;"#,
    r#"
    ///Computes `1.0 / length()`.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    #[lua(kind="Method",)]
    fn length_recip (self, ) -> f64;"#,
    r#"
    ///Returns `self` normalized to length 1.0.
    ///
    ///For valid results, `self` must _not_ be of length zero.
    ///
    ///Panics
    ///
    ///Will panic if `self` is zero length when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn normalize (self, ) -> Self;"#,
    r#"
    ///Returns `true` if, and only if, all elements are finite.
    ///If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[lua(kind="Method",)]
    fn is_finite (self, ) -> bool;"#,
    r#"
    #[lua(kind="Method",)]
    fn is_nan (self, ) -> bool;"#,
    r#"
    ///Returns whether `self` of length `1.0` or not.
    ///
    ///Uses a precision threshold of `1e-6`.
    #[lua(kind="Method",)]
    fn is_normalized (self, ) -> bool;"#,
    r#"
    #[lua(kind="Method",)]
    fn is_near_identity (self, ) -> bool;"#,
    r#"
    ///Returns the angle (in radians) for the minimal rotation
    ///for transforming this quaternion into another.
    ///
    ///Both quaternions must be normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",)]
    fn angle_between (self, #[proxy] rhs : Self,) -> f64;"#,
    r#"
    ///Returns true if the absolute difference of all elements between `self` and `rhs`
    ///is less than or equal to `max_abs_diff`.
    ///
    ///This can be used to compare if two quaternions contain similar elements. It works
    ///best when comparing with a known value. The `max_abs_diff` that should be used used
    ///depends on the values being compared against.
    ///
    ///For more see
    ///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[lua(kind="Method",)]
    fn abs_diff_eq (self, #[proxy] rhs : Self,max_abs_diff : f64, ) -> bool;"#,
    r#"
    ///Performs a linear interpolation between `self` and `rhs` based on
    ///the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s`
    ///is `1.0`, the result will be equal to `rhs`.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn lerp (self, #[proxy] end : Self,s : f64, ) -> Self;"#,
    r#"
    ///Performs a spherical linear interpolation between `self` and `end`
    ///based on the value `s`.
    ///
    ///When `s` is `0.0`, the result will be equal to `self`.  When `s`
    ///is `1.0`, the result will be equal to `end`.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn slerp (self, #[proxy] end : Self,s : f64, ) -> Self;"#,
    r#"
    ///Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    ///# Panics
    ///
    ///Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn mul_vec3 (self, #[proxy] rhs : bevy::math::f64::DVec3,) -> bevy::math::f64::DVec3;"#,
    r#"
    ///Multiplies two quaternions. If they each represent a rotation, the result will
    ///represent the combined rotation.
    ///
    ///Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    ///# Panics
    ///
    ///Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[lua(kind="Method",output(proxy))]
    fn mul_quat (self, #[proxy] rhs : Self,) -> Self;"#,
    r#"
    ///Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[lua(kind="Function",output(proxy))]
    fn from_affine3 (#[proxy] a : &bevy::math::f64::DAffine3,) -> Self;"#,
    r#"
    #[lua(kind="Method",output(proxy))]
    fn as_f32 (self, ) -> bevy::math::f32::Quat;"#,
    ]
)]
pub struct DQuat;
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::math::EulerRot", functions[])]
pub struct EulerRot;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Rect",
    functions[r#"
    ///Create a new rectangle from two corner points.
    ///
    ///The two points do not need to be the minimum and/or maximum corners.
    ///They only need to be two opposite corners.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::Rect;
    ///let r = Rect::new(0., 4., 10., 6.); // w=10 h=2
    ///let r = Rect::new(2., 3., 5., -1.); // w=3 h=4
    ///```
    #[lua(kind="Function",output(proxy))]
    fn new (x0 : f32, y0 : f32, x1 : f32, y1 : f32, ) -> Self;"#,
    r#"
    ///Create a new rectangle from two corner points.
    ///
    ///The two points do not need to be the minimum and/or maximum corners.
    ///They only need to be two opposite corners.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///// Unit rect from [0,0] to [1,1]
    ///let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
    ///// Same; the points do not need to be ordered
    ///let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
    ///```
    #[lua(kind="Function",output(proxy))]
    fn from_corners (#[proxy] p0 : bevy::math::f32::Vec2,#[proxy] p1 : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Create a new rectangle from its center and size.
    ///
    ///# Panics
    ///
    ///This method panics if any of the components of the size is negative.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
    ///assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
    ///assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
    ///```
    #[lua(kind="Function",output(proxy))]
    fn from_center_size (#[proxy] origin : bevy::math::f32::Vec2,#[proxy] size : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Create a new rectangle from its center and half-size.
    ///
    ///# Panics
    ///
    ///This method panics if any of the components of the half-size is negative.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2
    ///assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));
    ///assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));
    ///```
    #[lua(kind="Function",output(proxy))]
    fn from_center_half_size (#[proxy] origin : bevy::math::f32::Vec2,#[proxy] half_size : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Check if the rectangle is empty.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::from_corners(Vec2::ZERO, Vec2::new(0., 1.)); // w=0 h=1
    ///assert!(r.is_empty());
    ///```
    #[lua(kind="Method",)]
    fn is_empty (&self, ) -> bool;"#,
    r#"
    ///Rectangle width (max.x - min.x).
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::Rect;
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!((r.width() - 5.).abs() <= 1e-5);
    ///```
    #[lua(kind="Method",)]
    fn width (&self, ) -> f32;"#,
    r#"
    ///Rectangle height (max.y - min.y).
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::Rect;
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!((r.height() - 1.).abs() <= 1e-5);
    ///```
    #[lua(kind="Method",)]
    fn height (&self, ) -> f32;"#,
    r#"
    ///Rectangle size.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!(r.size().abs_diff_eq(Vec2::new(5., 1.), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn size (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Rectangle half-size.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!(r.half_size().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn half_size (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///The center point of the rectangle.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!(r.center().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn center (&self, ) -> bevy::math::f32::Vec2;"#,
    r#"
    ///Check if a point lies within this rectangle, inclusive of its edges.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::Rect;
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///assert!(r.contains(r.center()));
    ///assert!(r.contains(r.min));
    ///assert!(r.contains(r.max));
    ///```
    #[lua(kind="Method",)]
    fn contains (&self, #[proxy] point : bevy::math::f32::Vec2,) -> bool;"#,
    r#"
    ///Build a new rectangle formed of the union of this rectangle and another rectangle.
    ///
    ///The union is the smallest rectangle enclosing both rectangles.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
    ///let r = r1.union(r2);
    ///assert!(r.min.abs_diff_eq(Vec2::new(0., -1.), 1e-5));
    ///assert!(r.max.abs_diff_eq(Vec2::new(5., 3.), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn union (&self, #[proxy] other : Self,) -> Self;"#,
    r#"
    ///Build a new rectangle formed of the union of this rectangle and a point.
    ///
    ///The union is the smallest rectangle enclosing both the rectangle and the point. If the
    ///point is already inside the rectangle, this method returns a copy of the rectangle.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///let u = r.union_point(Vec2::new(3., 6.));
    ///assert!(u.min.abs_diff_eq(Vec2::ZERO, 1e-5));
    ///assert!(u.max.abs_diff_eq(Vec2::new(5., 6.), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn union_point (&self, #[proxy] other : bevy::math::f32::Vec2,) -> Self;"#,
    r#"
    ///Build a new rectangle formed of the intersection of this rectangle and another rectangle.
    ///
    ///The intersection is the largest rectangle enclosed in both rectangles. If the intersection
    ///is empty, this method returns an empty rectangle ([`Rect::is_empty()`] returns `true`), but
    ///the actual values of [`Rect::min`] and [`Rect::max`] are implementation-dependent.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
    ///let r = r1.intersect(r2);
    ///assert!(r.min.abs_diff_eq(Vec2::new(1., 0.), 1e-5));
    ///assert!(r.max.abs_diff_eq(Vec2::new(3., 1.), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn intersect (&self, #[proxy] other : Self,) -> Self;"#,
    r#"
    ///Create a new rectangle with a constant inset.
    ///
    ///The inset is the extra border on all sides. A positive inset produces a larger rectangle,
    ///while a negative inset is allowed and produces a smaller rectangle. If the inset is negative
    ///and its absolute value is larger than the rectangle half-size, the created rectangle is empty.
    ///
    ///# Examples
    ///
    ///```rust
    ///# use bevy_math::{Rect, Vec2};
    ///let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
    ///let r2 = r.inset(3.); // w=11 h=7
    ///assert!(r2.min.abs_diff_eq(Vec2::splat(-3.), 1e-5));
    ///assert!(r2.max.abs_diff_eq(Vec2::new(8., 4.), 1e-5));
    ///
    ///let r = Rect::new(0., -1., 6., 7.); // w=6 h=8
    ///let r2 = r.inset(-2.); // w=11 h=7
    ///assert!(r2.min.abs_diff_eq(Vec2::new(2., 1.), 1e-5));
    ///assert!(r2.max.abs_diff_eq(Vec2::new(4., 5.), 1e-5));
    ///```
    #[lua(kind="Method",output(proxy))]
    fn inset (&self, inset : f32, ) -> Self;"#,
    ]
)]
pub struct Rect;
#[cfg(feature = "lua")]
crate::impl_tealr_generic!(pub (crate) struct T);
#[cfg(feature = "lua")]
#[derive(Default)]
pub(crate) struct BevyAPIGlobals;
#[cfg(feature = "lua")]
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for BevyAPIGlobals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        Ok(())
    }
}
#[cfg(feature = "lua")]
pub struct LuaBevyAPIProvider;
#[cfg(feature = "lua")]
impl APIProvider for LuaBevyAPIProvider {
    type APITarget = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = LuaDocFragment;
    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        let ctx = ctx
            .get_mut()
            .expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(BevyAPIGlobals, ctx)
            .map_err(|e| ScriptError::Other(e.to_string()))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(LuaDocFragment::new("BevyAPI", |tw| tw))
    }
    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set(
                "entity",
                crate::lua::bevy::LuaEntity::new(script_data.entity),
            )
            .map_err(ScriptError::new_other)?;
        globals
            .set::<_, crate::lua::bevy::LuaScriptData>("script", script_data.into())
            .map_err(ScriptError::new_other)?;
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set("world", crate::lua::bevy::LuaWorld::new(world_ptr))
            .map_err(ScriptError::new_other)
    }
    fn register_with_app(&self, app: &mut App) {}
}
