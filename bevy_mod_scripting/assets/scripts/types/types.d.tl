global record types
--Defines how each line is aligned within the flexbox.
--
--
--
--It only applies if [`FlexWrap::Wrap`] is present and if there are multiple lines of items.
--
	record LuaAlignContent
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaAlignContent):(string)


	end
--How items are aligned according to the cross axis
--
	record LuaAlignItems
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaAlignItems):(string)


	end
--Works like [`AlignItems`] but applies only to a single item
--
	record LuaAlignSelf
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaAlignSelf):(string)


	end
--Defines the text direction
--
--
--
--For example English is written LTR (left-to-right) while Arabic is written RTL (right-to-left).
--
	record LuaDirection
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaDirection):(string)


	end
--Defines how flexbox items are ordered within a flexbox
--
	record LuaFlexDirection
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaFlexDirection):(string)


	end
--Defines if flexbox items appear on a single line or on multiple lines
--
	record LuaFlexWrap
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaFlexWrap):(string)


	end
--Describes whether the node should block interactions with lower nodes
--
	record LuaFocusPolicy
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaFocusPolicy):(string)


	end
--Describes what type of input interaction has occurred for a UI node.
--
--
--
--This is commonly queried with a `Changed<Interaction>` filter.
--
--
--
--Updated in [`ui_focus_system`].
--
--
--
--If a UI node has both [`Interaction`] and [`ComputedVisibility`] components,
--
--[`Interaction`] will always be [`Interaction::None`]
--
--when [`ComputedVisibility::is_visible()`] is false.
--
--This ensures that hidden UI nodes are not interactable,
--
--and do not end up stuck in an active state if hidden at the wrong time.
--
--
--
--Note that you can also control the visibility of a node using the [`Display`](crate::ui_node::Display) property,
--
--which fully collapses it during layout calculations.
--
	record LuaInteraction
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaInteraction):(string)


	end
--Defines how items are aligned according to the main axis
--
	record LuaJustifyContent
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaJustifyContent):(string)


	end
--Whether to show or hide overflowing items
--
	record LuaOverflow
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaOverflow):(string)


	end
--The strategy used to position this node
--
	record LuaPositionType
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaPositionType):(string)


	end
--An enum that describes possible types of value in flexbox layout options
--
	record LuaVal
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaVal):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)


	end
--The calculated clip of the node
--
	record LuaCalculatedClip
		userdata

		-- Fields
		-- The rect of the clip
		clip : LuaRect

		-- Meta methods
		metamethod __tostring: function(LuaCalculatedClip):(string)


	end
--The calculated size of the node
--
	record LuaCalculatedSize
		userdata

		-- Fields
		-- The size of the node
		size : ReflectedValue

		-- Meta methods
		metamethod __tostring: function(LuaCalculatedSize):(string)


	end
--Describes the size of a UI node
--
	record LuaNode
		userdata

		-- Fields
		-- The size of the node as width and height in pixels
		size : LuaVec2

		-- Meta methods
		metamethod __tostring: function(LuaNode):(string)


	end
--Describes the style of a UI node
--
--
--
--It uses the [Flexbox](https://cssreference.io/flexbox/) system.
--
--
--
--**Note:** Bevy's UI is upside down compared to how Flexbox normally works, to stay consistent with engine paradigms about layouting from
--
--the upper left corner of the display
--
	record LuaStyle
		userdata

		-- Fields
		-- Whether to arrange this node and its children with flexbox layout
		--
		-- 
		--
		-- If this is set to [`Display::None`], this node will be collapsed.
		display : LuaDisplay

		-- Whether to arrange this node relative to other nodes, or positioned absolutely
		position_type : LuaPositionType

		-- Which direction the content of this node should go
		direction : LuaDirection

		-- Whether to use column or row layout
		flex_direction : LuaFlexDirection

		-- How to wrap nodes
		flex_wrap : LuaFlexWrap

		-- How items are aligned according to the cross axis
		align_items : LuaAlignItems

		-- Like align_items but for only this item
		align_self : LuaAlignSelf

		-- How to align each line, only applies if flex_wrap is set to
		--
		-- [`FlexWrap::Wrap`] and there are multiple lines of items
		align_content : LuaAlignContent

		-- How items align according to the main axis
		justify_content : LuaJustifyContent

		-- The position of the node as described by its Rect
		position : ReflectedValue

		-- The margin of the node
		margin : ReflectedValue

		-- The padding of the node
		padding : ReflectedValue

		-- The border of the node
		border : ReflectedValue

		-- Defines how much a flexbox item should grow if there's space available
		flex_grow : number

		-- How to shrink if there's not enough space available
		flex_shrink : number

		-- The initial size of the item
		flex_basis : LuaVal

		-- The size of the flexbox
		size : ReflectedValue

		-- The minimum size of the flexbox
		min_size : ReflectedValue

		-- The maximum size of the flexbox
		max_size : ReflectedValue

		-- The aspect ratio of the flexbox
		aspect_ratio : ReflectedValue

		-- How to handle overflow
		overflow : LuaOverflow

		-- Meta methods
		metamethod __tostring: function(LuaStyle):(string)


	end
--The color of the node
--
	record LuaUiColor
		userdata

		-- Fields
		_0 : LuaColor

		-- Meta methods
		metamethod __tostring: function(LuaUiColor):(string)


	end
--The image of the node
--
	record LuaUiImage
		userdata

		-- Fields
		_0 : ReflectedValue

		-- Meta methods
		metamethod __tostring: function(LuaUiImage):(string)


	end
--Marker struct for buttons
--
	record LuaButton
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaButton):(string)


	end
--Describes how to resize the Image node
--
	record LuaImageMode
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaImageMode):(string)


	end
--Whether to use a Flexbox layout model.
--
--
--
--Part of the [`Style`] component.
--
	record LuaDisplay
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaDisplay):(string)


	end
--Animation controls
--
	record LuaAnimationPlayer
		userdata

		-- Pure methods
		--Is the animation paused
		is_paused: function(LuaAnimationPlayer):(boolean)

		--Speed of the animation playback
		speed: function(LuaAnimationPlayer):(number)

		--Time elapsed playing the animation
		elapsed: function(LuaAnimationPlayer):(number)

		-- Mutating methods
		--Pause the animation
		pause: function(LuaAnimationPlayer):()

		--Unpause the animation
		resume: function(LuaAnimationPlayer):()


	end
--Component used to identify an entity. Stores a hash for faster comparisons
--
--The hash is eagerly re-computed upon each update to the name.
--
--
--
--[`Name`] should not be treated as a globally unique identifier for entities,
--
--as multiple entities can have the same name.  [`bevy_ecs::entity::Entity`] should be
--
--used instead as the default unique identifier.
--
	record LuaName
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaName):(string)


	end
--Collection of static methods for [`LuaName`].
	record LuaNameClass
		userdata


	end
	record LuaGltfExtras
		userdata

		-- Fields
		value : string

		-- Meta methods
		metamethod __tostring: function(LuaGltfExtras):(string)


	end
--Contains references to the child entities of this entity
--
	record LuaChildren
		userdata

		-- Mutating methods
		--Swaps the child at `a_index` with the child at `b_index`
		swap: function(LuaChildren,(integer),(integer)):()

		-- Meta methods
		metamethod __tostring: function(LuaChildren):(string)


	end
--Collection of static methods for [`LuaChildren`].
	record LuaChildrenClass
		userdata


	end
--Holds a reference to the parent entity of this entity.
--
--This component should only be present on entities that actually have a parent entity.
--
	record LuaParent
		userdata

		-- Pure methods
		--Gets the [`Entity`] ID of the parent.
		get: function(LuaParent):(LuaEntity)

		-- Meta methods
		metamethod __tostring: function(LuaParent):(string)


	end
--The maximum width and height of text. The text will wrap according to the specified size.
--
--Characters out of the bounds after wrapping will be truncated. Text is aligned according to the
--
--specified `TextAlignment`.
--
--
--
--Note: only characters that are completely out of the bounds will be truncated, so this is not a
--
--reliable limit if it is necessary to contain the text strictly in the bounds. Currently this
--
--component is mainly useful for text wrapping only.
--
	record LuaText2dBounds
		userdata

		-- Fields
		size : LuaVec2

		-- Meta methods
		metamethod __tostring: function(LuaText2dBounds):(string)


	end
--The calculated size of text drawn in 2D scene.
--
	record LuaText2dSize
		userdata

		-- Fields
		size : LuaVec2

		-- Meta methods
		metamethod __tostring: function(LuaText2dSize):(string)


	end
	record LuaText
		userdata

		-- Fields
		sections : ReflectedValue

		alignment : LuaTextAlignment

		-- Pure methods
		--Returns this [`Text`] with a new [`TextAlignment`].
		with_alignment: function(LuaText,LuaTextAlignment):(LuaText)

		-- Meta methods
		metamethod __tostring: function(LuaText):(string)


	end
--Collection of static methods for [`LuaText`].
	record LuaTextClass
		userdata


	end
	record LuaTextAlignment
		userdata

		-- Fields
		vertical : LuaVerticalAlign

		horizontal : LuaHorizontalAlign

		-- Meta methods
		metamethod __tostring: function(LuaTextAlignment):(string)


	end
	record LuaTextSection
		userdata

		-- Fields
		value : string

		style : LuaTextStyle

		-- Pure functions
		--Create an empty [`TextSection`] from a style. Useful when the value will be set dynamically.
		from_style: function(LuaTextStyle):(LuaTextSection)

		-- Meta methods
		metamethod __tostring: function(LuaTextSection):(string)


	end
--Collection of static methods for [`LuaTextSection`].
	record LuaTextSectionClass
		userdata

		-- Pure functions
		--Create an empty [`TextSection`] from a style. Useful when the value will be set dynamically.
		from_style: function(LuaTextStyle):(LuaTextSection)


	end
	record LuaTextStyle
		userdata

		-- Fields
		font : ReflectedValue

		font_size : number

		color : LuaColor

		-- Meta methods
		metamethod __tostring: function(LuaTextStyle):(string)


	end
--Describes horizontal alignment preference for positioning & bounds.
--
	record LuaHorizontalAlign
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaHorizontalAlign):(string)


	end
--Describes vertical alignment preference for positioning & bounds. Currently a placeholder
--
--for future functionality.
--
	record LuaVerticalAlign
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaVerticalAlign):(string)


	end
--A Stopwatch is a struct that track elapsed time when started.
--
--
--
--# Examples
--
--
--
--```
--
--# use bevy_time::*;
--
--use std::time::Duration;
--
--let mut stopwatch = Stopwatch::new();
--
--assert_eq!(stopwatch.elapsed_secs(), 0.0);
--
--
--
--stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second
--
--assert_eq!(stopwatch.elapsed_secs(), 1.0);
--
--
--
--stopwatch.pause();
--
--stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick
--
--assert_eq!(stopwatch.elapsed_secs(), 1.0);
--
--
--
--stopwatch.reset(); // reset the stopwatch
--
--assert!(stopwatch.paused());
--
--assert_eq!(stopwatch.elapsed_secs(), 0.0);
--
--```
--
	record LuaStopwatch
		userdata

		-- Pure methods
		--Returns the elapsed time since the last [`reset`](Stopwatch::reset)
		--
		--of the stopwatch, in seconds.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut stopwatch = Stopwatch::new();
		--
		--stopwatch.tick(Duration::from_secs(1));
		--
		--assert_eq!(stopwatch.elapsed_secs(), 1.0);
		--
		--```
		--
		--
		--
		--# See Also
		--
		--
		--
		--[`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
		elapsed_secs: function(LuaStopwatch):(number)

		--Returns `true` if the stopwatch is paused.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut stopwatch = Stopwatch::new();
		--
		--assert!(!stopwatch.paused());
		--
		--stopwatch.pause();
		--
		--assert!(stopwatch.paused());
		--
		--stopwatch.unpause();
		--
		--assert!(!stopwatch.paused());
		--
		--```
		paused: function(LuaStopwatch):(boolean)

		-- Mutating methods
		--Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
		--
		--paused will not have any effect on the elapsed time.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut stopwatch = Stopwatch::new();
		--
		--stopwatch.pause();
		--
		--stopwatch.tick(Duration::from_secs_f32(1.5));
		--
		--assert!(stopwatch.paused());
		--
		--assert_eq!(stopwatch.elapsed_secs(), 0.0);
		--
		--```
		pause: function(LuaStopwatch):()

		--Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut stopwatch = Stopwatch::new();
		--
		--stopwatch.pause();
		--
		--stopwatch.tick(Duration::from_secs_f32(1.0));
		--
		--stopwatch.unpause();
		--
		--stopwatch.tick(Duration::from_secs_f32(1.0));
		--
		--assert!(!stopwatch.paused());
		--
		--assert_eq!(stopwatch.elapsed_secs(), 1.0);
		--
		--```
		unpause: function(LuaStopwatch):()

		--Resets the stopwatch.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut stopwatch = Stopwatch::new();
		--
		--stopwatch.tick(Duration::from_secs_f32(1.5));
		--
		--stopwatch.reset();
		--
		--assert_eq!(stopwatch.elapsed_secs(), 0.0);
		--
		--```
		reset: function(LuaStopwatch):()

		-- Pure functions
		--Create a new unpaused `Stopwatch` with no elapsed time.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let stopwatch = Stopwatch::new();
		--
		--assert_eq!(stopwatch.elapsed_secs(), 0.0);
		--
		--assert_eq!(stopwatch.paused(), false);
		--
		--```
		new: function():(LuaStopwatch)

		-- Meta methods
		metamethod __tostring: function(LuaStopwatch):(string)


	end
--Collection of static methods for [`LuaStopwatch`].
	record LuaStopwatchClass
		userdata

		-- Pure functions
		--Create a new unpaused `Stopwatch` with no elapsed time.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let stopwatch = Stopwatch::new();
		--
		--assert_eq!(stopwatch.elapsed_secs(), 0.0);
		--
		--assert_eq!(stopwatch.paused(), false);
		--
		--```
		new: function():(LuaStopwatch)


	end
--Tracks elapsed time. Enters the finished state once `duration` is reached.
--
--
--
--Non repeating timers will stop tracking and stay in the finished state until reset.
--
--Repeating timers will only be in the finished state on each tick `duration` is reached or
--
--exceeded, and can still be reset at any given point.
--
--
--
--Paused timers will not have elapsed time increased.
--
	record LuaTimer
		userdata

		-- Pure methods
		--Returns `true` if the timer has reached its duration.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--timer.tick(Duration::from_secs_f32(1.5));
		--
		--assert!(timer.finished());
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert!(timer.finished());
		--
		--```
		finished: function(LuaTimer):(boolean)

		--Returns `true` only on the tick the timer reached its duration.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--timer.tick(Duration::from_secs_f32(1.5));
		--
		--assert!(timer.just_finished());
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert!(!timer.just_finished());
		--
		--```
		just_finished: function(LuaTimer):(boolean)

		--Returns the time elapsed on the timer as a `f32`.
		--
		--See also [`Timer::elapsed`](Timer::elapsed).
		elapsed_secs: function(LuaTimer):(number)

		--Returns `true` if the timer is repeating.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut timer = Timer::from_seconds(1.0, true);
		--
		--assert!(timer.repeating());
		--
		--```
		repeating: function(LuaTimer):(boolean)

		--Returns `true` if the timer is paused.
		--
		--
		--
		--See also [`Stopwatch::paused`](Stopwatch::paused).
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--assert!(!timer.paused());
		--
		--timer.pause();
		--
		--assert!(timer.paused());
		--
		--timer.unpause();
		--
		--assert!(!timer.paused());
		--
		--```
		paused: function(LuaTimer):(boolean)

		--Returns the percentage of the timer elapsed time (goes from 0.0 to 1.0).
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(2.0, false);
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert_eq!(timer.percent(), 0.25);
		--
		--```
		percent: function(LuaTimer):(number)

		--Returns the percentage of the timer remaining time (goes from 0.0 to 1.0).
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(2.0, false);
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert_eq!(timer.percent_left(), 0.75);
		--
		--```
		percent_left: function(LuaTimer):(number)

		--Returns the number of times a repeating timer
		--
		--finished during the last [`tick`](Timer<T>::tick) call.
		--
		--
		--
		--For non repeating-timers, this method will only ever
		--
		--return 0 or 1.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, true);
		--
		--timer.tick(Duration::from_secs_f32(6.0));
		--
		--assert_eq!(timer.times_finished_this_tick(), 6);
		--
		--timer.tick(Duration::from_secs_f32(2.0));
		--
		--assert_eq!(timer.times_finished_this_tick(), 2);
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert_eq!(timer.times_finished_this_tick(), 0);
		--
		--```
		times_finished_this_tick: function(LuaTimer):(integer)

		-- Mutating methods
		--Sets whether the timer is repeating or not.
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut timer = Timer::from_seconds(1.0, true);
		--
		--timer.set_repeating(false);
		--
		--assert!(!timer.repeating());
		--
		--```
		set_repeating: function(LuaTimer,boolean):()

		--Pauses the Timer. Disables the ticking of the timer.
		--
		--
		--
		--See also [`Stopwatch::pause`](Stopwatch::pause).
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--timer.pause();
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert_eq!(timer.elapsed_secs(), 0.0);
		--
		--```
		pause: function(LuaTimer):()

		--Unpauses the Timer. Resumes the ticking of the timer.
		--
		--
		--
		--See also [`Stopwatch::unpause()`](Stopwatch::unpause).
		--
		--
		--
		--# Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--timer.pause();
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--timer.unpause();
		--
		--timer.tick(Duration::from_secs_f32(0.5));
		--
		--assert_eq!(timer.elapsed_secs(), 0.5);
		--
		--```
		unpause: function(LuaTimer):()

		--Resets the timer. the reset doesn't affect the `paused` state of the timer.
		--
		--
		--
		--See also [`Stopwatch::reset`](Stopwatch::reset).
		--
		--
		--
		--Examples
		--
		--```
		--
		--# use bevy_time::*;
		--
		--use std::time::Duration;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--timer.tick(Duration::from_secs_f32(1.5));
		--
		--timer.reset();
		--
		--assert!(!timer.finished());
		--
		--assert!(!timer.just_finished());
		--
		--assert_eq!(timer.elapsed_secs(), 0.0);
		--
		--```
		reset: function(LuaTimer):()

		-- Pure functions
		--Creates a new timer with a given duration in seconds.
		--
		--
		--
		--# Example
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--```
		from_seconds: function((number),(boolean)):(LuaTimer)

		-- Meta methods
		metamethod __tostring: function(LuaTimer):(string)


	end
--Collection of static methods for [`LuaTimer`].
	record LuaTimerClass
		userdata

		-- Pure functions
		--Creates a new timer with a given duration in seconds.
		--
		--
		--
		--# Example
		--
		--```
		--
		--# use bevy_time::*;
		--
		--let mut timer = Timer::from_seconds(1.0, false);
		--
		--```
		from_seconds: function((number),(boolean)):(LuaTimer)


	end
--Lightweight identifier of an [entity](crate::entity).
--
--
--
--The identifier is implemented using a [generational index]: a combination of an ID and a generation.
--
--This allows fast insertion after data removal in an array while minimizing loss of spatial locality.
--
--
--
--[generational index]: https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
--
--
--
--# Usage
--
--
--
--This data type is returned by iterating a `Query` that has `Entity` as part of its query fetch type parameter ([learn more]).
--
--It can also be obtained by calling [`EntityCommands::id`] or [`EntityMut::id`].
--
--
--
--```
--
--# use bevy_ecs::prelude::*;
--
--#
--
--fn setup(mut commands: Commands) {
--
--    // Calling `spawn` returns `EntityCommands`.
--
--    let entity = commands.spawn().id();
--
--}
--
--
--
--fn exclusive_system(world: &mut World) {
--
--    // Calling `spawn` returns `EntityMut`.
--
--    let entity = world.spawn().id();
--
--}
--
--#
--
--# bevy_ecs::system::assert_is_system(setup);
--
--# bevy_ecs::system::IntoExclusiveSystem::exclusive_system(exclusive_system);
--
--```
--
--
--
--It can be used to refer to a specific entity to apply [`EntityCommands`], or to call [`Query::get`] (or similar methods) to access its components.
--
--
--
--```
--
--# use bevy_ecs::prelude::*;
--
--#
--
--# #[derive(Component)]
--
--# struct Expired;
--
--#
--
--fn dispose_expired_food(mut commands: Commands, query: Query<Entity, With<Expired>>) {
--
--    for food_entity in &query {
--
--        commands.entity(food_entity).despawn();
--
--    }
--
--}
--
--#
--
--# bevy_ecs::system::assert_is_system(dispose_expired_food);
--
--```
--
--
--
--[learn more]: crate::system::Query#entity-id-access
--
--[`EntityCommands::id`]: crate::system::EntityCommands::id
--
--[`EntityMut::id`]: crate::world::EntityMut::id
--
--[`EntityCommands`]: crate::system::EntityCommands
--
--[`Query::get`]: crate::system::Query::get
--
	record LuaEntity
		userdata

		-- Pure methods
		--Convert to a form convenient for passing outside of rust.
		--
		--
		--
		--Only useful for identifying entities within the same instance of an application. Do not use
		--
		--for serialization between runs.
		--
		--
		--
		--No particular structure is guaranteed for the returned bits.
		to_bits: function(LuaEntity):(integer)

		--Return a transiently unique identifier.
		--
		--
		--
		--No two simultaneously-live entities share the same ID, but dead entities' IDs may collide
		--
		--with both live and dead entities. Useful for compactly representing entities within a
		--
		--specific snapshot of the world, such as when serializing.
		id: function(LuaEntity):(integer)

		--Returns the generation of this Entity's id. The generation is incremented each time an
		--
		--entity with a given id is despawned. This serves as a "count" of the number of times a
		--
		--given id has been reused (id, generation) pairs uniquely identify a given Entity.
		generation: function(LuaEntity):(integer)

		-- Pure functions
		--Creates a new entity reference with the specified `id` and a generation of 0.
		--
		--
		--
		--# Note
		--
		--
		--
		--Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
		--
		--[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
		--
		--only be used for sharing entities across apps, and only when they have a scheme
		--
		--worked out to share an ID space (which doesn't happen by default).
		--
		--
		--
		--In general, one should not try to synchronize the ECS by attempting to ensure that
		--
		--`Entity` lines up between instances, but instead insert a secondary identifier as
		--
		--a component.
		--
		--
		--
		--There are still some use cases where it might be appropriate to use this function
		--
		--externally.
		--
		--
		--
		--## Examples
		--
		--
		--
		--Initializing a collection (e.g. `array` or `Vec`) with a known size:
		--
		--
		--
		--```no_run
		--
		--# use bevy_ecs::prelude::*;
		--
		--let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
		--
		--
		--
		--```
		--
		--
		--
		--Deriving `Reflect` for a component that has an `Entity` field:
		--
		--
		--
		--```no_run
		--
		--# use bevy_ecs::{prelude::*, component::*};
		--
		--# use bevy_reflect::Reflect;
		--
		--#[derive(Reflect, Component)]
		--
		--#[reflect(Component)]
		--
		--pub struct MyStruct {
		--
		--    pub entity: Entity,
		--
		--}
		--
		--
		--
		--impl FromWorld for MyStruct {
		--
		--    fn from_world(_world: &mut World) -> Self {
		--
		--        Self {
		--
		--            entity: Entity::from_raw(u32::MAX),
		--
		--        }
		--
		--    }
		--
		--}
		--
		--```
		from_raw: function(integer):(LuaEntity)

		--Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
		--
		--
		--
		--Only useful when applied to results from `to_bits` in the same instance of an application.
		from_bits: function(integer):(LuaEntity)

		-- Meta methods
		metamethod __tostring: function(LuaEntity):(string)


	end
--Collection of static methods for [`LuaEntity`].
	record LuaEntityClass
		userdata

		-- Pure functions
		--Creates a new entity reference with the specified `id` and a generation of 0.
		--
		--
		--
		--# Note
		--
		--
		--
		--Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
		--
		--[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
		--
		--only be used for sharing entities across apps, and only when they have a scheme
		--
		--worked out to share an ID space (which doesn't happen by default).
		--
		--
		--
		--In general, one should not try to synchronize the ECS by attempting to ensure that
		--
		--`Entity` lines up between instances, but instead insert a secondary identifier as
		--
		--a component.
		--
		--
		--
		--There are still some use cases where it might be appropriate to use this function
		--
		--externally.
		--
		--
		--
		--## Examples
		--
		--
		--
		--Initializing a collection (e.g. `array` or `Vec`) with a known size:
		--
		--
		--
		--```no_run
		--
		--# use bevy_ecs::prelude::*;
		--
		--let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
		--
		--
		--
		--```
		--
		--
		--
		--Deriving `Reflect` for a component that has an `Entity` field:
		--
		--
		--
		--```no_run
		--
		--# use bevy_ecs::{prelude::*, component::*};
		--
		--# use bevy_reflect::Reflect;
		--
		--#[derive(Reflect, Component)]
		--
		--#[reflect(Component)]
		--
		--pub struct MyStruct {
		--
		--    pub entity: Entity,
		--
		--}
		--
		--
		--
		--impl FromWorld for MyStruct {
		--
		--    fn from_world(_world: &mut World) -> Self {
		--
		--        Self {
		--
		--            entity: Entity::from_raw(u32::MAX),
		--
		--        }
		--
		--    }
		--
		--}
		--
		--```
		from_raw: function(integer):(LuaEntity)

		--Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
		--
		--
		--
		--Only useful when applied to results from `to_bits` in the same instance of an application.
		from_bits: function(integer):(LuaEntity)


	end
--Describe the position of an entity. If the entity has a parent, the position is relative
--
--to its parent position.
--
--
--
--* To place or move an entity, you should set its [`Transform`].
--
--* To get the global transform of an entity, you should get its [`GlobalTransform`].
--
--* To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
--
--  * You may use the [`TransformBundle`](crate::TransformBundle) to guarantee this.
--
--
--
--## [`Transform`] and [`GlobalTransform`]
--
--
--
--[`Transform`] is the position of an entity relative to its parent position, or the reference
--
--frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
--
--
--
--[`GlobalTransform`] is the position of an entity relative to the reference frame.
--
--
--
--[`GlobalTransform`] is updated from [`Transform`] in the system
--
--[`transform_propagate_system`](crate::transform_propagate_system).
--
--
--
--This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
--
--update the [`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
--
--before the [`GlobalTransform`] is updated.
--
	record LuaTransform
		userdata

		-- Fields
		-- Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
		translation : LuaVec3

		-- Rotation of the entity.
		rotation : LuaQuat

		-- Scale of the entity.
		scale : LuaVec3

		-- Pure methods
		--Updates and returns this [`Transform`] by rotating it so that its unit vector in the
		--
		--local `Z` direction is toward `target` and its unit vector in the local `Y` direction
		--
		--is toward `up`.
		looking_at: function(LuaTransform,(LuaVec3),(LuaVec3)):(LuaTransform)

		--Returns this [`Transform`] with a new translation.
		with_translation: function(LuaTransform,LuaVec3):(LuaTransform)

		--Returns this [`Transform`] with a new rotation.
		with_rotation: function(LuaTransform,LuaQuat):(LuaTransform)

		--Returns this [`Transform`] with a new scale.
		with_scale: function(LuaTransform,LuaVec3):(LuaTransform)

		--Returns the 3d affine transformation matrix from this transforms translation,
		--
		--rotation, and scale.
		compute_matrix: function(LuaTransform):(LuaMat4)

		--Returns the 3d affine transformation matrix from this transforms translation,
		--
		--rotation, and scale.
		compute_affine: function(LuaTransform):(LuaAffine3A)

		--Get the unit vector in the local `X` direction.
		local_x: function(LuaTransform):(LuaVec3)

		--Equivalent to [`-local_x()`][Transform::local_x()]
		left: function(LuaTransform):(LuaVec3)

		--Equivalent to [`local_x()`][Transform::local_x()]
		right: function(LuaTransform):(LuaVec3)

		--Get the unit vector in the local `Y` direction.
		local_y: function(LuaTransform):(LuaVec3)

		--Equivalent to [`local_y()`][Transform::local_y]
		up: function(LuaTransform):(LuaVec3)

		--Equivalent to [`-local_y()`][Transform::local_y]
		down: function(LuaTransform):(LuaVec3)

		--Get the unit vector in the local `Z` direction.
		local_z: function(LuaTransform):(LuaVec3)

		--Equivalent to [`-local_z()`][Transform::local_z]
		forward: function(LuaTransform):(LuaVec3)

		--Equivalent to [`local_z()`][Transform::local_z]
		back: function(LuaTransform):(LuaVec3)

		--Multiplies `self` with `transform` component by component, returning the
		--
		--resulting [`Transform`]
		mul_transform: function(LuaTransform,LuaTransform):(LuaTransform)

		--Returns a [`Vec3`] of this [`Transform`] applied to `value`.
		mul_vec3: function(LuaTransform,LuaVec3):(LuaVec3)

		-- Mutating methods
		--Rotates this [`Transform`] by the given rotation.
		--
		--
		--
		--If this [`Transform`] has a parent, the `rotation` is relative to the rotation of the parent.
		rotate: function(LuaTransform,LuaQuat):()

		--Rotates this [`Transform`] around the given `axis` by `angle` (in radians).
		--
		--
		--
		--If this [`Transform`] has a parent, the `axis` is relative to the rotation of the parent.
		rotate_axis: function(LuaTransform,(LuaVec3),(number)):()

		--Rotates this [`Transform`] around the `X` axis by `angle` (in radians).
		--
		--
		--
		--If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
		rotate_x: function(LuaTransform,number):()

		--Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).
		--
		--
		--
		--If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
		rotate_y: function(LuaTransform,number):()

		--Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).
		--
		--
		--
		--If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.
		rotate_z: function(LuaTransform,number):()

		--Rotates this [`Transform`] by the given `rotation`.
		--
		--
		--
		--The `rotation` is relative to this [`Transform`]'s current rotation.
		rotate_local: function(LuaTransform,LuaQuat):()

		--Rotates this [`Transform`] around its local `axis` by `angle` (in radians).
		rotate_local_axis: function(LuaTransform,(LuaVec3),(number)):()

		--Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).
		rotate_local_x: function(LuaTransform,number):()

		--Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).
		rotate_local_y: function(LuaTransform,number):()

		--Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).
		rotate_local_z: function(LuaTransform,number):()

		--Translates this [`Transform`] around a `point` in space.
		--
		--
		--
		--If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.
		translate_around: function(LuaTransform,(LuaVec3),(LuaQuat)):()

		--Rotates this [`Transform`] around a `point` in space.
		--
		--
		--
		--If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.
		rotate_around: function(LuaTransform,(LuaVec3),(LuaQuat)):()

		--Rotates this [`Transform`] so that its local negative `Z` direction is toward
		--
		--`target` and its local `Y` direction is toward `up`.
		look_at: function(LuaTransform,(LuaVec3),(LuaVec3)):()

		--Changes the `scale` of this [`Transform`], multiplying the current `scale` by
		--
		--`scale_factor`.
		apply_non_uniform_scale: function(LuaTransform,LuaVec3):()

		-- Pure functions
		--Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
		--
		--is used for z-ordering elements: higher `z`-value will be in front of lower
		--
		--`z`-value.
		from_xyz: function((number),(number),(number)):(LuaTransform)

		--Creates a new identity [`Transform`], with no translation, rotation, and a scale of 1 on
		--
		--all axes.
		identity: function():(LuaTransform)

		--Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
		--
		--transformation matrix.
		from_matrix: function(LuaMat4):(LuaTransform)

		--Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
		--
		--all axes.
		from_translation: function(LuaVec3):(LuaTransform)

		--Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
		--
		--all axes.
		from_rotation: function(LuaQuat):(LuaTransform)

		--Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
		--
		--all axes.
		from_scale: function(LuaVec3):(LuaTransform)

		-- Meta methods
		metamethod __tostring: function(LuaTransform):(string)

		-- Meta functions
		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaTransform`].
	record LuaTransformClass
		userdata

		-- Pure functions
		--Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
		--
		--is used for z-ordering elements: higher `z`-value will be in front of lower
		--
		--`z`-value.
		from_xyz: function((number),(number),(number)):(LuaTransform)

		--Creates a new identity [`Transform`], with no translation, rotation, and a scale of 1 on
		--
		--all axes.
		identity: function():(LuaTransform)

		--Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
		--
		--transformation matrix.
		from_matrix: function(LuaMat4):(LuaTransform)

		--Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
		--
		--all axes.
		from_translation: function(LuaVec3):(LuaTransform)

		--Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
		--
		--all axes.
		from_rotation: function(LuaQuat):(LuaTransform)

		--Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
		--
		--all axes.
		from_scale: function(LuaVec3):(LuaTransform)

		-- Meta functions
		metamethod __mul: function((any),(any)):(any)


	end
--Describe the position of an entity relative to the reference frame.
--
--
--
--* To place or move an entity, you should set its [`Transform`].
--
--* To get the global transform of an entity, you should get its [`GlobalTransform`].
--
--* For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].
--
--  * You may use the [`TransformBundle`](crate::TransformBundle) to guarantee this.
--
--
--
--## [`Transform`] and [`GlobalTransform`]
--
--
--
--[`Transform`] is the position of an entity relative to its parent position, or the reference
--
--frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
--
--
--
--[`GlobalTransform`] is the position of an entity relative to the reference frame.
--
--
--
--[`GlobalTransform`] is updated from [`Transform`] in the system
--
--[`transform_propagate_system`](crate::transform_propagate_system).
--
--
--
--This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
--
--update the [`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
--
--before the [`GlobalTransform`] is updated.
--
	record LuaGlobalTransform
		userdata

		-- Pure methods
		--Returns the 3d affine transformation matrix as a [`Mat4`].
		compute_matrix: function(LuaGlobalTransform):(LuaMat4)

		--Returns the 3d affine transformation matrix as an [`Affine3A`].
		affine: function(LuaGlobalTransform):(LuaAffine3A)

		--Returns the transformation as a [`Transform`].
		--
		--
		--
		--The transform is expected to be non-degenerate and without shearing, or the output
		--
		--will be invalid.
		compute_transform: function(LuaGlobalTransform):(LuaTransform)

		--Return the local right vector (X).
		right: function(LuaGlobalTransform):(LuaVec3)

		--Return the local left vector (-X).
		left: function(LuaGlobalTransform):(LuaVec3)

		--Return the local up vector (Y).
		up: function(LuaGlobalTransform):(LuaVec3)

		--Return the local down vector (-Y).
		down: function(LuaGlobalTransform):(LuaVec3)

		--Return the local back vector (Z).
		back: function(LuaGlobalTransform):(LuaVec3)

		--Return the local forward vector (-Z).
		forward: function(LuaGlobalTransform):(LuaVec3)

		--Get the translation as a [`Vec3`].
		translation: function(LuaGlobalTransform):(LuaVec3)

		--Get the translation as a [`Vec3A`].
		translation_vec3a: function(LuaGlobalTransform):(LuaVec3A)

		--Get an upper bound of the radius from the given `extents`.
		radius_vec3a: function(LuaGlobalTransform,LuaVec3A):(number)

		--Returns a [`Vec3`] of this [`Transform`] applied to `value`.
		mul_vec3: function(LuaGlobalTransform,LuaVec3):(LuaVec3)

		--Multiplies `self` with `transform` component by component, returning the
		--
		--resulting [`GlobalTransform`]
		mul_transform: function(LuaGlobalTransform,LuaTransform):(LuaGlobalTransform)

		-- Pure functions
		--Creates a new identity [`GlobalTransform`], that maps all points in space to themselves.
		identity: function():(LuaGlobalTransform)

		-- Meta methods
		metamethod __tostring: function(LuaGlobalTransform):(string)

		-- Meta functions
		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaGlobalTransform`].
	record LuaGlobalTransformClass
		userdata

		-- Pure functions
		--Creates a new identity [`GlobalTransform`], that maps all points in space to themselves.
		identity: function():(LuaGlobalTransform)

		-- Meta functions
		metamethod __mul: function((any),(any)):(any)


	end
--An ambient light, which lights the entire scene equally.
--
	record LuaAmbientLight
		userdata

		-- Fields
		color : LuaColor

		-- A direct scale factor multiplied with `color` before being passed to the shader.
		brightness : number

		-- Meta methods
		metamethod __tostring: function(LuaAmbientLight):(string)


	end
	record LuaCubemapVisibleEntities
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaCubemapVisibleEntities):(string)


	end
--A Directional light.
--
--
--
--Directional lights don't exist in reality but they are a good
--
--approximation for light sources VERY far away, like the sun or
--
--the moon.
--
--
--
--Valid values for `illuminance` are:
--
--
--
--| Illuminance (lux) | Surfaces illuminated by                        |
--
--|-------------------|------------------------------------------------|
--
--| 0.0001            | Moonless, overcast night sky (starlight)       |
--
--| 0.002             | Moonless clear night sky with airglow          |
--
--| 0.05–0.3          | Full moon on a clear night                     |
--
--| 3.4               | Dark limit of civil twilight under a clear sky |
--
--| 20–50             | Public areas with dark surroundings            |
--
--| 50                | Family living room lights                      |
--
--| 80                | Office building hallway/toilet lighting        |
--
--| 100               | Very dark overcast day                         |
--
--| 150               | Train station platforms                        |
--
--| 320–500           | Office lighting                                |
--
--| 400               | Sunrise or sunset on a clear day.              |
--
--| 1000              | Overcast day; typical TV studio lighting       |
--
--| 10,000–25,000     | Full daylight (not direct sun)                 |
--
--| 32,000–100,000    | Direct sunlight                                |
--
--
--
--Source: [Wikipedia](https://en.wikipedia.org/wiki/Lux)
--
	record LuaDirectionalLight
		userdata

		-- Fields
		color : LuaColor

		-- Illuminance in lux
		illuminance : number

		shadows_enabled : boolean

		shadow_projection : LuaOrthographicProjection

		shadow_depth_bias : number

		-- A bias applied along the direction of the fragment's surface normal. It is scaled to the
		--
		-- shadow map's texel size so that it is automatically adjusted to the orthographic projection.
		shadow_normal_bias : number

		-- Meta methods
		metamethod __tostring: function(LuaDirectionalLight):(string)


	end
	record LuaDirectionalLightShadowMap
		userdata

		-- Fields
		size : integer

		-- Meta methods
		metamethod __tostring: function(LuaDirectionalLightShadowMap):(string)


	end
--Add this component to make a [`Mesh`](bevy_render::mesh::Mesh) not cast shadows.
--
	record LuaNotShadowCaster
		userdata


	end
--Add this component to make a [`Mesh`](bevy_render::mesh::Mesh) not receive shadows.
--
	record LuaNotShadowReceiver
		userdata


	end
--A light that emits light in all directions from a central point.
--
--
--
--Real-world values for `intensity` (luminous power in lumens) based on the electrical power
--
--consumption of the type of real-world light are:
--
--
--
--| Luminous Power (lumen) (i.e. the intensity member) | Incandescent non-halogen (Watts) | Incandescent halogen (Watts) | Compact fluorescent (Watts) | LED (Watts |
--
--|------|-----|----|--------|-------|
--
--| 200  | 25  |    | 3-5    | 3     |
--
--| 450  | 40  | 29 | 9-11   | 5-8   |
--
--| 800  | 60  |    | 13-15  | 8-12  |
--
--| 1100 | 75  | 53 | 18-20  | 10-16 |
--
--| 1600 | 100 | 72 | 24-28  | 14-17 |
--
--| 2400 | 150 |    | 30-52  | 24-30 |
--
--| 3100 | 200 |    | 49-75  | 32    |
--
--| 4000 | 300 |    | 75-100 | 40.5  |
--
--
--
--Source: [Wikipedia](https://en.wikipedia.org/wiki/Lumen_(unit)#Lighting)
--
	record LuaPointLight
		userdata

		-- Fields
		color : LuaColor

		intensity : number

		range : number

		radius : number

		shadows_enabled : boolean

		shadow_depth_bias : number

		-- A bias applied along the direction of the fragment's surface normal. It is scaled to the
		--
		-- shadow map's texel size so that it can be small close to the camera and gets larger further
		--
		-- away.
		shadow_normal_bias : number

		-- Meta methods
		metamethod __tostring: function(LuaPointLight):(string)


	end
	record LuaPointLightShadowMap
		userdata

		-- Fields
		size : integer

		-- Meta methods
		metamethod __tostring: function(LuaPointLightShadowMap):(string)


	end
--Alpha mode
--
	record LuaAlphaMode
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaAlphaMode):(string)


	end
--Controls whether an entity should rendered in wireframe-mode if the [`WireframePlugin`] is enabled
--
	record LuaWireframe
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaWireframe):(string)


	end
	record LuaWireframeConfig
		userdata

		-- Fields
		-- Whether to show wireframes for all meshes. If `false`, only meshes with a [Wireframe] component will be rendered.
		global : boolean

		-- Meta methods
		metamethod __tostring: function(LuaWireframeConfig):(string)


	end
--The depth clear operation to perform for the main 3d pass.
--
	record LuaCamera3dDepthLoadOp
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaCamera3dDepthLoadOp):(string)


	end
--When used as a resource, sets the color that is used to clear the screen between frames.
--
--
--
--This color appears as the "background" color for simple apps, when
--
--there are portions of the screen with nothing rendered.
--
	record LuaClearColor
		userdata

		-- Fields
		_0 : LuaColor

		-- Meta methods
		metamethod __tostring: function(LuaClearColor):(string)


	end
	record LuaClearColorConfig
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaClearColorConfig):(string)


	end
	record LuaCamera2d
		userdata

		-- Fields
		clear_color : LuaClearColorConfig


	end
--Configuration for the "main 3d render graph".
--
	record LuaCamera3d
		userdata

		-- Fields
		-- The clear color operation to perform for the main 3d pass.
		clear_color : LuaClearColorConfig

		-- The depth clear operation to perform for the main 3d pass.
		depth_load_op : LuaCamera3dDepthLoadOp


	end
--How a sprite is positioned relative to its [`Transform`](bevy_transform::components::Transform).
--
--It defaults to `Anchor::Center`.
--
	record LuaAnchor
		userdata

		-- Pure methods
		as_vec: function(LuaAnchor):(LuaVec2)

		-- Meta methods
		metamethod __tostring: function(LuaAnchor):(string)


	end
--Component for rendering with meshes in the 2d pipeline, usually with a [2d material](crate::Material2d) such as [`ColorMaterial`](crate::ColorMaterial).
--
--
--
--It wraps a [`Handle<Mesh>`] to differentiate from the 3d pipelines which use the handles directly as components
--
	record LuaMesh2dHandle
		userdata

		-- Fields
		_0 : ReflectedValue

		-- Meta methods
		metamethod __tostring: function(LuaMesh2dHandle):(string)


	end
	record LuaTextureAtlasSprite
		userdata

		-- Fields
		color : LuaColor

		index : integer

		flip_x : boolean

		flip_y : boolean

		-- An optional custom size for the sprite that will be used when rendering, instead of the size
		--
		-- of the sprite's image in the atlas
		custom_size : ReflectedValue

		anchor : LuaAnchor

		-- Pure functions
		new: function(integer):(LuaTextureAtlasSprite)

		-- Meta methods
		metamethod __tostring: function(LuaTextureAtlasSprite):(string)


	end
--Collection of static methods for [`LuaTextureAtlasSprite`].
	record LuaTextureAtlasSpriteClass
		userdata

		-- Pure functions
		new: function(integer):(LuaTextureAtlasSprite)


	end
	record LuaSprite
		userdata

		-- Fields
		-- The sprite's color tint
		color : LuaColor

		-- Flip the sprite along the `X` axis
		flip_x : boolean

		-- Flip the sprite along the `Y` axis
		flip_y : boolean

		-- An optional custom size for the sprite that will be used when rendering, instead of the size
		--
		-- of the sprite's image
		custom_size : ReflectedValue

		-- [`Anchor`] point of the sprite in the world
		anchor : LuaAnchor

		-- Meta methods
		metamethod __tostring: function(LuaSprite):(string)


	end
--A rectangle defined by two points. There is no defined origin, so 0,0 could be anywhere
--
--(top-left, bottom-left, etc)
--
	record LuaRect
		userdata

		-- Fields
		-- The beginning point of the rect
		min : LuaVec2

		-- The ending point of the rect
		max : LuaVec2

		-- Pure methods
		width: function(LuaRect):(number)

		height: function(LuaRect):(number)

		size: function(LuaRect):(LuaVec2)

		-- Meta methods
		metamethod __tostring: function(LuaRect):(string)


	end
--Describes which rendering layers an entity belongs to.
--
--
--
--Cameras with this component will only render entities with intersecting
--
--layers.
--
--
--
--There are 32 layers numbered `0` - [`TOTAL_LAYERS`](RenderLayers::TOTAL_LAYERS). Entities may
--
--belong to one or more layers, or no layer at all.
--
--
--
--The [`Default`] instance of `RenderLayers` contains layer `0`, the first layer.
--
--
--
--An entity with this component without any layers is invisible.
--
--
--
--Entities without this component belong to layer `0`.
--
	record LuaRenderLayers
		userdata

		-- Pure methods
		--Determine if a `RenderLayers` intersects another.
		--
		--
		--
		--`RenderLayers`s intersect if they share any common layers.
		--
		--
		--
		--A `RenderLayers` with no layers will not match any other
		--
		--`RenderLayers`, even another with no layers.
		intersects: function(LuaRenderLayers,LuaRenderLayers):(boolean)

		-- Pure functions
		--Create a new `RenderLayers` that belongs to all layers.
		all: function():(LuaRenderLayers)

		--Create a new `RenderLayers` that belongs to no layers.
		none: function():(LuaRenderLayers)

		-- Meta methods
		metamethod __tostring: function(LuaRenderLayers):(string)


	end
--Collection of static methods for [`LuaRenderLayers`].
	record LuaRenderLayersClass
		userdata

		-- Pure functions
		--Create a new `RenderLayers` that belongs to all layers.
		all: function():(LuaRenderLayers)

		--Create a new `RenderLayers` that belongs to no layers.
		none: function():(LuaRenderLayers)


	end
--User indication of whether an entity is visible. Propagates down the entity hierarchy.
--
--If an entity is hidden in this way,  all [`Children`] (and all of their children and so on) will also be hidden.
--
--This is done by setting the values of their [`ComputedVisibility`] component.
--
	record LuaVisibility
		userdata

		-- Fields
		-- Indicates whether this entity is visible. Hidden values will propagate down the entity hierarchy.
		--
		-- If this entity is hidden, all of its descendants will be hidden as well. See [`Children`] and [`Parent`] for
		--
		-- hierarchy info.
		is_visible : boolean

		-- Pure functions
		--Creates a new [`Visibility`], set as visible
		visible: function():(LuaVisibility)

		-- Meta methods
		metamethod __tostring: function(LuaVisibility):(string)


	end
--Collection of static methods for [`LuaVisibility`].
	record LuaVisibilityClass
		userdata

		-- Pure functions
		--Creates a new [`Visibility`], set as visible
		visible: function():(LuaVisibility)


	end
--Collection of entities visible from the current view.
--
--
--
--This component contains all entities which are visible from the currently
--
--rendered view. The collection is updated automatically by the [`check_visibility()`]
--
--system, and renderers can use it to optimize rendering of a particular view, to
--
--prevent drawing items not visible from that view.
--
--
--
--This component is intended to be attached to the same entity as the [`Camera`] and
--
--the [`Frustum`] defining the view.
--
--
--
--Currently this component is ignored by the sprite renderer, so sprite rendering
--
--is not optimized per view.
--
	record LuaVisibleEntities
		userdata

		-- Pure methods
		len: function(LuaVisibleEntities):(integer)

		is_empty: function(LuaVisibleEntities):(boolean)

		-- Meta methods
		metamethod __tostring: function(LuaVisibleEntities):(string)


	end
--Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
--
	record LuaComputedVisibility
		userdata

		-- Pure methods
		--Whether this entity is visible to something this frame. This is true if and only if [`Self::is_visible_in_hierarchy`] and [`Self::is_visible_in_view`]
		--
		--are true. This is the canonical method to call to determine if an entity should be drawn.
		--
		--This value is updated in [`CoreStage::PostUpdate`] during the [`VisibilitySystems::CheckVisibility`] system label. Reading it from the
		--
		--[`CoreStage::Update`] stage will yield the value from the previous frame.
		is_visible: function(LuaComputedVisibility):(boolean)

		--Whether this entity is visible in the entity hierarchy, which is determined by the [`Visibility`] component.
		--
		--This takes into account "visibility inheritance". If any of this entity's ancestors (see [`Parent`]) are hidden, this entity
		--
		--will be hidden as well. This value is updated in the [`CoreStage::PostUpdate`] stage in the
		--
		--[`VisibilitySystems::VisibilityPropagate`] system label.
		is_visible_in_hierarchy: function(LuaComputedVisibility):(boolean)

		--Whether this entity is visible in _any_ view (Cameras, Lights, etc). Each entity type (and view type) should choose how to set this
		--
		--value. For cameras and drawn entities, this will take into account [`RenderLayers`].
		--
		--
		--
		--This value is reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] during [`CoreStage::PostUpdate`].
		--
		--Each entity type then chooses how to set this field in the [`CoreStage::PostUpdate`] stage in the
		--
		--[`VisibilitySystems::CheckVisibility`] system label. Meshes might use frustum culling to decide if they are visible in a view.
		--
		--Other entities might just set this to `true` every frame.
		is_visible_in_view: function(LuaComputedVisibility):(boolean)

		-- Mutating methods
		--Sets `is_visible_in_view` to `true`. This is not reversible for a given frame, as it encodes whether or not this is visible in
		--
		--_any_ view. This will be automatically reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] and then set
		--
		--to the proper value in [`VisibilitySystems::CheckVisibility`]. This should _only_ be set in systems with the [`VisibilitySystems::CheckVisibility`]
		--
		--label. Don't call this unless you are defining a custom visibility system. For normal user-defined entity visibility, see [`Visibility`].
		set_visible_in_view: function(LuaComputedVisibility):()

		-- Pure functions
		--Creates a new [`ComputedVisibility`], set as not visible
		not_visible: function():(LuaComputedVisibility)

		-- Meta methods
		metamethod __tostring: function(LuaComputedVisibility):(string)


	end
--Collection of static methods for [`LuaComputedVisibility`].
	record LuaComputedVisibilityClass
		userdata

		-- Pure functions
		--Creates a new [`ComputedVisibility`], set as not visible
		not_visible: function():(LuaComputedVisibility)


	end
	record LuaSkinnedMesh
		userdata

		-- Fields
		inverse_bindposes : ReflectedValue

		joints : ReflectedValue

		-- Meta methods
		metamethod __tostring: function(LuaSkinnedMesh):(string)


	end
	record LuaScalingMode
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaScalingMode):(string)


	end
	record LuaWindowOrigin
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaWindowOrigin):(string)


	end
	record LuaColor
		userdata

		-- Pure methods
		--Get red in sRGB colorspace.
		r: function(LuaColor):(number)

		--Get green in sRGB colorspace.
		g: function(LuaColor):(number)

		--Get blue in sRGB colorspace.
		b: function(LuaColor):(number)

		--Get alpha.
		a: function(LuaColor):(number)

		--Converts a `Color` to variant `Color::Rgba`
		as_rgba: function(LuaColor):(LuaColor)

		--Converts a `Color` to variant `Color::RgbaLinear`
		as_rgba_linear: function(LuaColor):(LuaColor)

		--Converts a `Color` to variant `Color::Hsla`
		as_hsla: function(LuaColor):(LuaColor)

		--Converts `Color` to a `u32` from sRGB colorspace.
		--
		--
		--
		--Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
		--
		--`A` will be the most significant byte and `R` the least significant.
		as_rgba_u32: function(LuaColor):(integer)

		--Converts Color to a u32 from linear RGB colorspace.
		--
		--
		--
		--Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
		--
		--`A` will be the most significant byte and `R` the least significant.
		as_linear_rgba_u32: function(LuaColor):(integer)

		-- Pure functions
		--New `Color` from sRGB colorspace.
		rgb: function((number),(number),(number)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgba: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` from linear RGB colorspace.
		rgb_linear: function((number),(number),(number)):(LuaColor)

		--New `Color` from linear RGB colorspace.
		rgba_linear: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` with HSL representation in sRGB colorspace.
		hsl: function((number),(number),(number)):(LuaColor)

		--New `Color` with HSL representation in sRGB colorspace.
		hsla: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgb_u8: function((integer),(integer),(integer)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgba_u8: function((integer),(integer),(integer),(integer)):(LuaColor)

		-- Meta methods
		metamethod __tostring: function(LuaColor):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaColor`].
	record LuaColorClass
		userdata

		-- Pure functions
		--New `Color` from sRGB colorspace.
		rgb: function((number),(number),(number)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgba: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` from linear RGB colorspace.
		rgb_linear: function((number),(number),(number)):(LuaColor)

		--New `Color` from linear RGB colorspace.
		rgba_linear: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` with HSL representation in sRGB colorspace.
		hsl: function((number),(number),(number)):(LuaColor)

		--New `Color` with HSL representation in sRGB colorspace.
		hsla: function((number),(number),(number),(number)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgb_u8: function((integer),(integer),(integer)):(LuaColor)

		--New `Color` from sRGB colorspace.
		rgba_u8: function((integer),(integer),(integer),(integer)):(LuaColor)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--An Axis-Aligned Bounding Box
--
	record LuaAabb
		userdata

		-- Fields
		center : LuaVec3A

		half_extents : LuaVec3A

		-- Pure methods
		min: function(LuaAabb):(LuaVec3A)

		max: function(LuaAabb):(LuaVec3A)

		-- Pure functions
		from_min_max: function((LuaVec3),(LuaVec3)):(LuaAabb)

		-- Meta methods
		metamethod __tostring: function(LuaAabb):(string)


	end
--Collection of static methods for [`LuaAabb`].
	record LuaAabbClass
		userdata

		-- Pure functions
		from_min_max: function((LuaVec3),(LuaVec3)):(LuaAabb)


	end
	record LuaCubemapFrusta
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaCubemapFrusta):(string)


	end
--A frustum defined by the 6 containing planes
--
--Planes are ordered left, right, top, bottom, near, far
--
--Normals point into the contained volume
--
	record LuaFrustum
		userdata

		-- Pure methods
		intersects_obb: function(LuaFrustum,(LuaAabb),(LuaMat4),(boolean)):(boolean)

		-- Pure functions
		from_view_projection: function((LuaMat4),(LuaVec3),(LuaVec3),(number)):(LuaFrustum)

		-- Meta methods
		metamethod __tostring: function(LuaFrustum):(string)


	end
--Collection of static methods for [`LuaFrustum`].
	record LuaFrustumClass
		userdata

		-- Pure functions
		from_view_projection: function((LuaMat4),(LuaVec3),(LuaVec3),(number)):(LuaFrustum)


	end
--Configuration resource for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing).
--
--
--
--# Example
--
--```
--
--# use bevy_app::prelude::App;
--
--# use bevy_render::prelude::Msaa;
--
--App::new()
--
--    .insert_resource(Msaa { samples: 4 })
--
--    .run();
--
--```
--
	record LuaMsaa
		userdata

		-- Fields
		-- The number of samples to run for Multi-Sample Anti-Aliasing. Higher numbers result in
		--
		-- smoother edges.
		--
		-- Defaults to 4.
		--
		-- 
		--
		-- Note that WGPU currently only supports 1 or 4 samples.
		--
		-- Ultimately we plan on supporting whatever is natively supported on a given device.
		--
		-- Check out this issue for more info: <https://github.com/gfx-rs/wgpu/issues/1832>
		samples : integer


	end
	record LuaCamera
		userdata

		-- Fields
		-- If set, this camera will render to the given [`Viewport`] rectangle within the configured [`RenderTarget`].
		viewport : ReflectedValue

		-- Cameras with a lower priority will be rendered before cameras with a higher priority.
		priority : integer

		-- If this is set to true, this camera will be rendered to its specified [`RenderTarget`]. If false, this
		--
		-- camera will not be rendered.
		is_active : boolean

		-- The method used to calculate this camera's depth. This will be used for projections and visibility.
		depth_calculation : LuaDepthCalculation

		-- The "target" that this camera will render to.
		target : LuaRenderTarget

		-- Pure methods
		--The projection matrix computed using this camera's [`CameraProjection`].
		projection_matrix: function(LuaCamera):(LuaMat4)

		-- Meta methods
		metamethod __tostring: function(LuaCamera):(string)


	end
--The "target" that a [`Camera`] will render to. For example, this could be a [`Window`](bevy_window::Window)
--
--swapchain or an [`Image`].
--
	record LuaRenderTarget
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaRenderTarget):(string)


	end
--Render viewport configuration for the [`Camera`] component.
--
--
--
--The viewport defines the area on the render target to which the camera renders its image.
--
--You can overlay multiple cameras in a single window using viewports to create effects like
--
--split screen, minimaps, and character viewers.
--
	record LuaViewport
		userdata

		-- Fields
		-- The physical position to render this viewport to within the [`RenderTarget`] of this [`Camera`].
		--
		-- (0,0) corresponds to the top-left corner
		physical_position : LuaUVec2

		-- The physical size of the viewport rectangle to render to within the [`RenderTarget`] of this [`Camera`].
		--
		-- The origin of the rectangle is in the top-left corner.
		physical_size : LuaUVec2

		-- The minimum and maximum depth to render (on a scale from 0.0 to 1.0).
		depth : ReflectedValue

		-- Meta methods
		metamethod __tostring: function(LuaViewport):(string)


	end
--A configurable [`CameraProjection`] that can select its projection type at runtime.
--
	record LuaProjection
		userdata

		-- Pure methods
		get_projection_matrix: function(LuaProjection):(LuaMat4)

		depth_calculation: function(LuaProjection):(LuaDepthCalculation)

		far: function(LuaProjection):(number)

		-- Mutating methods
		update: function(LuaProjection,(number),(number)):()

		-- Meta methods
		metamethod __tostring: function(LuaProjection):(string)


	end
	record LuaOrthographicProjection
		userdata

		-- Fields
		left : number

		right : number

		bottom : number

		top : number

		near : number

		_far : number

		window_origin : LuaWindowOrigin

		scaling_mode : LuaScalingMode

		scale : number

		_depth_calculation : LuaDepthCalculation

		-- Pure methods
		get_projection_matrix: function(LuaOrthographicProjection):(LuaMat4)

		depth_calculation: function(LuaOrthographicProjection):(LuaDepthCalculation)

		far: function(LuaOrthographicProjection):(number)

		-- Mutating methods
		update: function(LuaOrthographicProjection,(number),(number)):()

		-- Meta methods
		metamethod __tostring: function(LuaOrthographicProjection):(string)


	end
	record LuaPerspectiveProjection
		userdata

		-- Fields
		fov : number

		aspect_ratio : number

		near : number

		_far : number

		-- Pure methods
		get_projection_matrix: function(LuaPerspectiveProjection):(LuaMat4)

		depth_calculation: function(LuaPerspectiveProjection):(LuaDepthCalculation)

		far: function(LuaPerspectiveProjection):(number)

		-- Mutating methods
		update: function(LuaPerspectiveProjection,(number),(number)):()

		-- Meta methods
		metamethod __tostring: function(LuaPerspectiveProjection):(string)


	end
	record LuaDepthCalculation
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaDepthCalculation):(string)


	end
--Configures the [`RenderGraph`](crate::render_graph::RenderGraph) name assigned to be run for a given [`Camera`] entity.
--
	record LuaCameraRenderGraph
		userdata


	end
--Collection of static methods for [`LuaCameraRenderGraph`].
	record LuaCameraRenderGraphClass
		userdata


	end
--An unique identifier to an asset path.
--
	record LuaAssetPathId
		userdata

		-- Pure methods
		--Gets the id of the source path.
		source_path_id: function(LuaAssetPathId):(LuaSourcePathId)

		--Gets the id of the sub-asset label.
		label_id: function(LuaAssetPathId):(LuaLabelId)

		-- Meta methods
		metamethod __tostring: function(LuaAssetPathId):(string)


	end
--An unique identifier to a sub-asset label.
--
	record LuaLabelId
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaLabelId):(string)


	end
--An unique identifier to the source path of an asset.
--
	record LuaSourcePathId
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaSourcePathId):(string)


	end
--A unique, stable asset id.
--
	record LuaHandleId
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaHandleId):(string)


	end
--Collection of static methods for [`LuaHandleId`].
	record LuaHandleIdClass
		userdata


	end
--A 2-dimensional vector.
--
	record LuaVec2
		userdata

		-- Fields
		x : number

		y : number

		-- Pure methods
		--Creates a 3D vector from `self` and the given `z` value.
		extend: function(LuaVec2,number):(LuaVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaVec2,LuaVec2):(number)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaVec2,LuaVec2):(LuaVec2)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaVec2,LuaVec2):(LuaVec2)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaVec2,(LuaVec2),(LuaVec2)):(LuaVec2)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaVec2):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaVec2):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaVec2,LuaVec2):(LuaBVec2)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaVec2):(LuaVec2)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaVec2):(LuaVec2)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaVec2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaVec2):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaVec2):(LuaBVec2)

		--Computes the length of `self`.
		length: function(LuaVec2):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaVec2):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaVec2):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaVec2,LuaVec2):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaVec2,LuaVec2):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaVec2):(LuaVec2)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaVec2):(LuaVec2)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaVec2):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaVec2,LuaVec2):(LuaVec2)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaVec2,LuaVec2):(LuaVec2)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaVec2,LuaVec2):(LuaVec2)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaVec2,LuaVec2):(LuaVec2)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaVec2):(LuaVec2)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaVec2):(LuaVec2)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaVec2):(LuaVec2)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaVec2):(LuaVec2)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaVec2):(LuaVec2)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaVec2,number):(LuaVec2)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaVec2):(LuaVec2)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaVec2,(LuaVec2),(number)):(LuaVec2)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaVec2,(LuaVec2),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaVec2,(number),(number)):(LuaVec2)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaVec2,number):(LuaVec2)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaVec2,number):(LuaVec2)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaVec2,(LuaVec2),(LuaVec2)):(LuaVec2)

		--Returns the angle (in radians) between `self` and `rhs`.
		--
		--
		--
		--The input vectors do not need to be unit length however they must be non-zero.
		angle_between: function(LuaVec2,LuaVec2):(number)

		--Returns a vector that is equal to `self` rotated by 90 degrees.
		perp: function(LuaVec2):(LuaVec2)

		--The perpendicular dot product of `self` and `rhs`.
		--
		--Also known as the wedge product, 2D cross product, and determinant.
		perp_dot: function(LuaVec2,LuaVec2):(number)

		--Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
		--
		--then this just rotation. This is what you usually want. Otherwise,
		--
		--it will be like a rotation with a multiplication by `self`'s length.
		rotate: function(LuaVec2,LuaVec2):(LuaVec2)

		--Casts all elements of `self` to `f64`.
		as_dvec2: function(LuaVec2):(LuaDVec2)

		--Casts all elements of `self` to `i32`.
		as_ivec2: function(LuaVec2):(LuaIVec2)

		--Casts all elements of `self` to `u32`.
		as_uvec2: function(LuaVec2):(LuaUVec2)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number)):(LuaVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaVec2),(LuaVec2)):(LuaVec2)

		--Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
		--
		--conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
		--
		--create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
		from_angle: function(number):(LuaVec2)

		-- Meta methods
		metamethod __tostring: function(LuaVec2):(string)

		metamethod __unm: function(LuaVec2):(LuaVec2)

		metamethod __index: function(LuaVec2,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaVec2,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaVec2`].
	record LuaVec2Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number)):(LuaVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaVec2),(LuaVec2)):(LuaVec2)

		--Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
		--
		--conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
		--
		--create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
		from_angle: function(number):(LuaVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3-dimensional vector.
--
	record LuaVec3
		userdata

		-- Fields
		x : number

		y : number

		z : number

		-- Pure methods
		--Creates a 4D vector from `self` and the given `w` value.
		extend: function(LuaVec3,number):(LuaVec4)

		--Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
		--
		--
		--
		--Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
		truncate: function(LuaVec3):(LuaVec2)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaVec3,LuaVec3):(number)

		--Computes the cross product of `self` and `rhs`.
		cross: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaVec3,LuaVec3):(LuaVec3)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaVec3,(LuaVec3),(LuaVec3)):(LuaVec3)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaVec3):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaVec3):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaVec3,LuaVec3):(LuaBVec3)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaVec3):(LuaVec3)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaVec3):(LuaVec3)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaVec3):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaVec3):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaVec3):(LuaBVec3)

		--Computes the length of `self`.
		length: function(LuaVec3):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaVec3):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaVec3):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaVec3,LuaVec3):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaVec3,LuaVec3):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaVec3):(LuaVec3)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaVec3):(LuaVec3)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaVec3):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaVec3,LuaVec3):(LuaVec3)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaVec3):(LuaVec3)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaVec3):(LuaVec3)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaVec3):(LuaVec3)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaVec3):(LuaVec3)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaVec3):(LuaVec3)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaVec3,number):(LuaVec3)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaVec3):(LuaVec3)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaVec3,(LuaVec3),(number)):(LuaVec3)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaVec3,(LuaVec3),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaVec3,(number),(number)):(LuaVec3)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaVec3,number):(LuaVec3)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaVec3,number):(LuaVec3)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaVec3,(LuaVec3),(LuaVec3)):(LuaVec3)

		--Returns the angle (in radians) between two vectors.
		--
		--
		--
		--The input vectors do not need to be unit length however they must be non-zero.
		angle_between: function(LuaVec3,LuaVec3):(number)

		--Returns some vector that is orthogonal to the given one.
		--
		--
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--The output vector is not necessarily unit-length.
		--
		--For that use [`Self::any_orthonormal_vector`] instead.
		any_orthogonal_vector: function(LuaVec3):(LuaVec3)

		--Returns any unit-length vector that is orthogonal to the given one.
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		any_orthonormal_vector: function(LuaVec3):(LuaVec3)

		--Casts all elements of `self` to `f64`.
		as_dvec3: function(LuaVec3):(LuaDVec3)

		--Casts all elements of `self` to `i32`.
		as_ivec3: function(LuaVec3):(LuaIVec3)

		--Casts all elements of `self` to `u32`.
		as_uvec3: function(LuaVec3):(LuaUVec3)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaVec3),(LuaVec3)):(LuaVec3)

		-- Meta methods
		metamethod __tostring: function(LuaVec3):(string)

		metamethod __unm: function(LuaVec3):(LuaVec3)

		metamethod __index: function(LuaVec3,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaVec3,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaVec3`].
	record LuaVec3Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaVec3),(LuaVec3)):(LuaVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3-dimensional vector with SIMD support.
--
--
--
--This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
--
--better performance than the `Vec3` type.
--
--
--
--It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
--
	record LuaVec3A
		userdata

		-- Pure methods
		--Creates a 4D vector from `self` and the given `w` value.
		extend: function(LuaVec3A,number):(LuaVec4)

		--Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
		--
		--
		--
		--Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
		truncate: function(LuaVec3A):(LuaVec2)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaVec3A,LuaVec3A):(number)

		--Computes the cross product of `self` and `rhs`.
		cross: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaVec3A,(LuaVec3A),(LuaVec3A)):(LuaVec3A)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaVec3A):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaVec3A):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaVec3A,LuaVec3A):(LuaBVec3A)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaVec3A):(LuaVec3A)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaVec3A):(LuaVec3A)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaVec3A):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaVec3A):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaVec3A):(LuaBVec3A)

		--Computes the length of `self`.
		length: function(LuaVec3A):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaVec3A):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaVec3A):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaVec3A,LuaVec3A):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaVec3A,LuaVec3A):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaVec3A):(LuaVec3A)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaVec3A):(LuaVec3A)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaVec3A):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaVec3A,LuaVec3A):(LuaVec3A)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaVec3A):(LuaVec3A)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaVec3A):(LuaVec3A)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaVec3A):(LuaVec3A)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaVec3A):(LuaVec3A)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaVec3A):(LuaVec3A)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaVec3A,number):(LuaVec3A)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaVec3A):(LuaVec3A)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaVec3A,(LuaVec3A),(number)):(LuaVec3A)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaVec3A,(LuaVec3A),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaVec3A,(number),(number)):(LuaVec3A)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaVec3A,number):(LuaVec3A)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaVec3A,number):(LuaVec3A)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaVec3A,(LuaVec3A),(LuaVec3A)):(LuaVec3A)

		--Returns the angle (in radians) between two vectors.
		--
		--
		--
		--The input vectors do not need to be unit length however they must be non-zero.
		angle_between: function(LuaVec3A,LuaVec3A):(number)

		--Returns some vector that is orthogonal to the given one.
		--
		--
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--The output vector is not necessarily unit-length.
		--
		--For that use [`Self::any_orthonormal_vector`] instead.
		any_orthogonal_vector: function(LuaVec3A):(LuaVec3A)

		--Returns any unit-length vector that is orthogonal to the given one.
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		any_orthonormal_vector: function(LuaVec3A):(LuaVec3A)

		--Casts all elements of `self` to `f64`.
		as_dvec3: function(LuaVec3A):(LuaDVec3)

		--Casts all elements of `self` to `i32`.
		as_ivec3: function(LuaVec3A):(LuaIVec3)

		--Casts all elements of `self` to `u32`.
		as_uvec3: function(LuaVec3A):(LuaUVec3)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaVec3A)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec3A)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3A),(LuaVec3A),(LuaVec3A)):(LuaVec3A)

		-- Meta methods
		metamethod __tostring: function(LuaVec3A):(string)

		metamethod __unm: function(LuaVec3A):(LuaVec3A)

		metamethod __index: function(LuaVec3A,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaVec3A,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaVec3A`].
	record LuaVec3AClass
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaVec3A)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec3A)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3A),(LuaVec3A),(LuaVec3A)):(LuaVec3A)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 4-dimensional vector with SIMD support.
--
--
--
--This type uses 16 byte aligned SIMD vector type for storage.
--
	record LuaVec4
		userdata

		-- Pure methods
		--Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
		--
		--
		--
		--Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
		--
		--
		--
		--To truncate to `Vec3A` use `Vec3A::from()`.
		truncate: function(LuaVec4):(LuaVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaVec4,LuaVec4):(number)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaVec4,LuaVec4):(LuaVec4)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaVec4,LuaVec4):(LuaVec4)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaVec4,(LuaVec4),(LuaVec4)):(LuaVec4)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaVec4):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaVec4):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaVec4,LuaVec4):(LuaBVec4A)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaVec4):(LuaVec4)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaVec4):(LuaVec4)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaVec4):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaVec4):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaVec4):(LuaBVec4A)

		--Computes the length of `self`.
		length: function(LuaVec4):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaVec4):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaVec4):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaVec4,LuaVec4):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaVec4,LuaVec4):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaVec4):(LuaVec4)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaVec4):(LuaVec4)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaVec4):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaVec4,LuaVec4):(LuaVec4)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaVec4,LuaVec4):(LuaVec4)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaVec4,LuaVec4):(LuaVec4)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaVec4,LuaVec4):(LuaVec4)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaVec4):(LuaVec4)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaVec4):(LuaVec4)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaVec4):(LuaVec4)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaVec4):(LuaVec4)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaVec4):(LuaVec4)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaVec4,number):(LuaVec4)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaVec4):(LuaVec4)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaVec4,(LuaVec4),(number)):(LuaVec4)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaVec4,(LuaVec4),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaVec4,(number),(number)):(LuaVec4)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaVec4,number):(LuaVec4)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaVec4,number):(LuaVec4)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaVec4,(LuaVec4),(LuaVec4)):(LuaVec4)

		--Casts all elements of `self` to `f64`.
		as_dvec4: function(LuaVec4):(LuaDVec4)

		--Casts all elements of `self` to `i32`.
		as_ivec4: function(LuaVec4):(LuaIVec4)

		--Casts all elements of `self` to `u32`.
		as_uvec4: function(LuaVec4):(LuaUVec4)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number),(number)):(LuaVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4A),(LuaVec4),(LuaVec4)):(LuaVec4)

		-- Meta methods
		metamethod __tostring: function(LuaVec4):(string)

		metamethod __unm: function(LuaVec4):(LuaVec4)

		metamethod __index: function(LuaVec4,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaVec4,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaVec4`].
	record LuaVec4Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number),(number)):(LuaVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4A),(LuaVec4),(LuaVec4)):(LuaVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 2-dimensional boolean vector.
--
	record LuaBVec2
		userdata

		-- Fields
		x : boolean

		y : boolean

		-- Pure methods
		--Returns a bitmask with the lowest two bits set from the elements of `self`.
		--
		--
		--
		--A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
		--
		--into the first lowest bit, element `y` into the second, etc.
		bitmask: function(LuaBVec2):(integer)

		--Returns true if any of the elements are true, false otherwise.
		any: function(LuaBVec2):(boolean)

		--Returns true if all the elements are true, false otherwise.
		all: function(LuaBVec2):(boolean)

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean)):(LuaBVec2)

		-- Meta methods
		metamethod __tostring: function(LuaBVec2):(string)


	end
--Collection of static methods for [`LuaBVec2`].
	record LuaBVec2Class
		userdata

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean)):(LuaBVec2)


	end
--A 3-dimensional boolean vector.
--
	record LuaBVec3
		userdata

		-- Fields
		x : boolean

		y : boolean

		z : boolean

		-- Pure methods
		--Returns a bitmask with the lowest two bits set from the elements of `self`.
		--
		--
		--
		--A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
		--
		--into the first lowest bit, element `y` into the second, etc.
		bitmask: function(LuaBVec3):(integer)

		--Returns true if any of the elements are true, false otherwise.
		any: function(LuaBVec3):(boolean)

		--Returns true if all the elements are true, false otherwise.
		all: function(LuaBVec3):(boolean)

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean)):(LuaBVec3)

		-- Meta methods
		metamethod __tostring: function(LuaBVec3):(string)


	end
--Collection of static methods for [`LuaBVec3`].
	record LuaBVec3Class
		userdata

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean)):(LuaBVec3)


	end
--A 4-dimensional boolean vector.
--
	record LuaBVec4
		userdata

		-- Fields
		x : boolean

		y : boolean

		z : boolean

		w : boolean

		-- Pure methods
		--Returns a bitmask with the lowest two bits set from the elements of `self`.
		--
		--
		--
		--A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
		--
		--into the first lowest bit, element `y` into the second, etc.
		bitmask: function(LuaBVec4):(integer)

		--Returns true if any of the elements are true, false otherwise.
		any: function(LuaBVec4):(boolean)

		--Returns true if all the elements are true, false otherwise.
		all: function(LuaBVec4):(boolean)

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4)

		-- Meta methods
		metamethod __tostring: function(LuaBVec4):(string)


	end
--Collection of static methods for [`LuaBVec4`].
	record LuaBVec4Class
		userdata

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4)


	end
--A 3-dimensional SIMD vector mask.
--
--
--
--This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available
--
--`BVec3A` will be a type alias for `BVec3`.
--
	record LuaBVec3A
		userdata

		-- Pure methods
		--Returns a bitmask with the lowest two bits set from the elements of `self`.
		--
		--
		--
		--A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
		--
		--into the first lowest bit, element `y` into the second, etc.
		bitmask: function(LuaBVec3A):(integer)

		--Returns true if any of the elements are true, false otherwise.
		any: function(LuaBVec3A):(boolean)

		--Returns true if all the elements are true, false otherwise.
		all: function(LuaBVec3A):(boolean)

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean)):(LuaBVec3A)

		-- Meta methods
		metamethod __tostring: function(LuaBVec3A):(string)


	end
--Collection of static methods for [`LuaBVec3A`].
	record LuaBVec3AClass
		userdata

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean)):(LuaBVec3A)


	end
--A 4-dimensional SIMD vector mask.
--
--
--
--This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available
--
--`BVec4A` will be a type alias for `BVec4`.
--
	record LuaBVec4A
		userdata

		-- Pure methods
		--Returns a bitmask with the lowest two bits set from the elements of `self`.
		--
		--
		--
		--A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
		--
		--into the first lowest bit, element `y` into the second, etc.
		bitmask: function(LuaBVec4A):(integer)

		--Returns true if any of the elements are true, false otherwise.
		any: function(LuaBVec4A):(boolean)

		--Returns true if all the elements are true, false otherwise.
		all: function(LuaBVec4A):(boolean)

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4A)

		-- Meta methods
		metamethod __tostring: function(LuaBVec4A):(string)


	end
--Collection of static methods for [`LuaBVec4A`].
	record LuaBVec4AClass
		userdata

		-- Pure functions
		--Creates a new vector mask.
		new: function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4A)


	end
--A 2-dimensional vector.
--
	record LuaDVec2
		userdata

		-- Fields
		x : number

		y : number

		-- Pure methods
		--Creates a 3D vector from `self` and the given `z` value.
		extend: function(LuaDVec2,number):(LuaDVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaDVec2,LuaDVec2):(number)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaDVec2,(LuaDVec2),(LuaDVec2)):(LuaDVec2)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaDVec2):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaDVec2):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaDVec2,LuaDVec2):(LuaBVec2)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaDVec2):(LuaDVec2)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaDVec2):(LuaDVec2)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDVec2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDVec2):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaDVec2):(LuaBVec2)

		--Computes the length of `self`.
		length: function(LuaDVec2):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaDVec2):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaDVec2):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaDVec2,LuaDVec2):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaDVec2,LuaDVec2):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaDVec2):(LuaDVec2)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaDVec2):(LuaDVec2)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaDVec2):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaDVec2):(LuaDVec2)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaDVec2):(LuaDVec2)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaDVec2):(LuaDVec2)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaDVec2):(LuaDVec2)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaDVec2):(LuaDVec2)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaDVec2,number):(LuaDVec2)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaDVec2):(LuaDVec2)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaDVec2,(LuaDVec2),(number)):(LuaDVec2)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDVec2,(LuaDVec2),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaDVec2,(number),(number)):(LuaDVec2)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaDVec2,number):(LuaDVec2)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaDVec2,number):(LuaDVec2)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaDVec2,(LuaDVec2),(LuaDVec2)):(LuaDVec2)

		--Returns the angle (in radians) between `self` and `rhs`.
		--
		--
		--
		--The input vectors do not need to be unit length however they must be non-zero.
		angle_between: function(LuaDVec2,LuaDVec2):(number)

		--Returns a vector that is equal to `self` rotated by 90 degrees.
		perp: function(LuaDVec2):(LuaDVec2)

		--The perpendicular dot product of `self` and `rhs`.
		--
		--Also known as the wedge product, 2D cross product, and determinant.
		perp_dot: function(LuaDVec2,LuaDVec2):(number)

		--Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
		--
		--then this just rotation. This is what you usually want. Otherwise,
		--
		--it will be like a rotation with a multiplication by `self`'s length.
		rotate: function(LuaDVec2,LuaDVec2):(LuaDVec2)

		--Casts all elements of `self` to `f32`.
		as_vec2: function(LuaDVec2):(LuaVec2)

		--Casts all elements of `self` to `i32`.
		as_ivec2: function(LuaDVec2):(LuaIVec2)

		--Casts all elements of `self` to `u32`.
		as_uvec2: function(LuaDVec2):(LuaUVec2)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number)):(LuaDVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaDVec2),(LuaDVec2)):(LuaDVec2)

		--Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
		--
		--conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
		--
		--create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
		from_angle: function(number):(LuaDVec2)

		-- Meta methods
		metamethod __tostring: function(LuaDVec2):(string)

		metamethod __unm: function(LuaDVec2):(LuaDVec2)

		metamethod __index: function(LuaDVec2,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaDVec2,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDVec2`].
	record LuaDVec2Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number)):(LuaDVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaDVec2),(LuaDVec2)):(LuaDVec2)

		--Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
		--
		--conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
		--
		--create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
		from_angle: function(number):(LuaDVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3-dimensional vector.
--
	record LuaDVec3
		userdata

		-- Fields
		x : number

		y : number

		z : number

		-- Pure methods
		--Creates a 4D vector from `self` and the given `w` value.
		extend: function(LuaDVec3,number):(LuaDVec4)

		--Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
		--
		--
		--
		--Truncation may also be performed by using `self.xy()` or `DVec2::from()`.
		truncate: function(LuaDVec3):(LuaDVec2)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaDVec3,LuaDVec3):(number)

		--Computes the cross product of `self` and `rhs`.
		cross: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaDVec3,(LuaDVec3),(LuaDVec3)):(LuaDVec3)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaDVec3):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaDVec3):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaDVec3,LuaDVec3):(LuaBVec3)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaDVec3):(LuaDVec3)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaDVec3):(LuaDVec3)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDVec3):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDVec3):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaDVec3):(LuaBVec3)

		--Computes the length of `self`.
		length: function(LuaDVec3):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaDVec3):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaDVec3):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaDVec3,LuaDVec3):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaDVec3,LuaDVec3):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaDVec3):(LuaDVec3)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaDVec3):(LuaDVec3)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaDVec3):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaDVec3,LuaDVec3):(LuaDVec3)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaDVec3):(LuaDVec3)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaDVec3):(LuaDVec3)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaDVec3):(LuaDVec3)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaDVec3):(LuaDVec3)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaDVec3):(LuaDVec3)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaDVec3,number):(LuaDVec3)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaDVec3):(LuaDVec3)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaDVec3,(LuaDVec3),(number)):(LuaDVec3)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDVec3,(LuaDVec3),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaDVec3,(number),(number)):(LuaDVec3)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaDVec3,number):(LuaDVec3)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaDVec3,number):(LuaDVec3)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaDVec3,(LuaDVec3),(LuaDVec3)):(LuaDVec3)

		--Returns the angle (in radians) between two vectors.
		--
		--
		--
		--The input vectors do not need to be unit length however they must be non-zero.
		angle_between: function(LuaDVec3,LuaDVec3):(number)

		--Returns some vector that is orthogonal to the given one.
		--
		--
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--The output vector is not necessarily unit-length.
		--
		--For that use [`Self::any_orthonormal_vector`] instead.
		any_orthogonal_vector: function(LuaDVec3):(LuaDVec3)

		--Returns any unit-length vector that is orthogonal to the given one.
		--
		--The input vector must be finite and non-zero.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		any_orthonormal_vector: function(LuaDVec3):(LuaDVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3: function(LuaDVec3):(LuaVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3a: function(LuaDVec3):(LuaVec3A)

		--Casts all elements of `self` to `i32`.
		as_ivec3: function(LuaDVec3):(LuaIVec3)

		--Casts all elements of `self` to `u32`.
		as_uvec3: function(LuaDVec3):(LuaUVec3)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaDVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaDVec3),(LuaDVec3)):(LuaDVec3)

		-- Meta methods
		metamethod __tostring: function(LuaDVec3):(string)

		metamethod __unm: function(LuaDVec3):(LuaDVec3)

		metamethod __index: function(LuaDVec3,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaDVec3,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDVec3`].
	record LuaDVec3Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number)):(LuaDVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaDVec3),(LuaDVec3)):(LuaDVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 4-dimensional vector.
--
	record LuaDVec4
		userdata

		-- Fields
		x : number

		y : number

		z : number

		w : number

		-- Pure methods
		--Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
		--
		--
		--
		--Truncation to `DVec3` may also be performed by using `self.xyz()` or `DVec3::from()`.
		truncate: function(LuaDVec4):(LuaDVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaDVec4,LuaDVec4):(number)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaDVec4,(LuaDVec4),(LuaDVec4)):(LuaDVec4)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaDVec4):(number)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaDVec4):(number)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaDVec4,LuaDVec4):(LuaBVec4)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaDVec4):(LuaDVec4)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaDVec4):(LuaDVec4)

		--Returns `true` if, and only if, all elements are finite.  If any element is either
		--
		--`NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDVec4):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDVec4):(boolean)

		--Performs `is_nan` on each element of self, returning a vector mask of the results.
		--
		--
		--
		--In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
		is_nan_mask: function(LuaDVec4):(LuaBVec4)

		--Computes the length of `self`.
		length: function(LuaDVec4):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is faster than `length()` as it avoids a square root operation.
		length_squared: function(LuaDVec4):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaDVec4):(number)

		--Computes the Euclidean distance between two points in space.
		distance: function(LuaDVec4,LuaDVec4):(number)

		--Compute the squared euclidean distance between two points in space.
		distance_squared: function(LuaDVec4,LuaDVec4):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero, nor very close to zero.
		--
		--
		--
		--See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaDVec4):(LuaDVec4)

		--Returns `self` normalized to length 1.0 if possible, else returns zero.
		--
		--
		--
		--In particular, if the input is zero (or very close to zero), or non-finite,
		--
		--the result of this operation will be zero.
		--
		--
		--
		--See also [`Self::try_normalize`].
		normalize_or_zero: function(LuaDVec4):(LuaDVec4)

		--Returns whether `self` is length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaDVec4):(boolean)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is zero length when `glam_assert` is enabled.
		project_onto: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be of non-zero length.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
		reject_from: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Returns the vector projection of `self` onto `rhs`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		project_onto_normalized: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Returns the vector rejection of `self` from `rhs`.
		--
		--
		--
		--The vector rejection is the vector perpendicular to the projection of `self` onto
		--
		--`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
		--
		--
		--
		--`rhs` must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rhs` is not normalized when `glam_assert` is enabled.
		reject_from_normalized: function(LuaDVec4,LuaDVec4):(LuaDVec4)

		--Returns a vector containing the nearest integer to a number for each element of `self`.
		--
		--Round half-way cases away from 0.0.
		round: function(LuaDVec4):(LuaDVec4)

		--Returns a vector containing the largest integer less than or equal to a number for each
		--
		--element of `self`.
		floor: function(LuaDVec4):(LuaDVec4)

		--Returns a vector containing the smallest integer greater than or equal to a number for
		--
		--each element of `self`.
		ceil: function(LuaDVec4):(LuaDVec4)

		--Returns a vector containing the fractional part of the vector, e.g. `self -
		--
		--self.floor()`.
		--
		--
		--
		--Note that this is fast but not precise for large numbers.
		fract: function(LuaDVec4):(LuaDVec4)

		--Returns a vector containing `e^self` (the exponential function) for each element of
		--
		--`self`.
		exp: function(LuaDVec4):(LuaDVec4)

		--Returns a vector containing each element of `self` raised to the power of `n`.
		powf: function(LuaDVec4,number):(LuaDVec4)

		--Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
		recip: function(LuaDVec4):(LuaDVec4)

		--Performs a linear interpolation between `self` and `rhs` based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
		--
		--will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
		--
		--extrapolated.
		lerp: function(LuaDVec4,(LuaDVec4),(number)):(LuaDVec4)

		--Returns true if the absolute difference of all elements between `self` and `rhs` is
		--
		--less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two vectors contain similar elements. It works best when
		--
		--comparing with a known value. The `max_abs_diff` that should be used used depends on
		--
		--the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDVec4,(LuaDVec4),(number)):(boolean)

		--Returns a vector with a length no less than `min` and no more than `max`
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp_length: function(LuaDVec4,(number),(number)):(LuaDVec4)

		--Returns a vector with a length no more than `max`
		clamp_length_max: function(LuaDVec4,number):(LuaDVec4)

		--Returns a vector with a length no less than `min`
		clamp_length_min: function(LuaDVec4,number):(LuaDVec4)

		--Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
		--
		--error, yielding a more accurate result than an unfused multiply-add.
		--
		--
		--
		--Using `mul_add` *may* be more performant than an unfused multiply-add if the target
		--
		--architecture has a dedicated fma CPU instruction. However, this is not always true,
		--
		--and will be heavily dependant on designing algorithms with specific target hardware in
		--
		--mind.
		mul_add: function(LuaDVec4,(LuaDVec4),(LuaDVec4)):(LuaDVec4)

		--Casts all elements of `self` to `f32`.
		as_vec4: function(LuaDVec4):(LuaVec4)

		--Casts all elements of `self` to `i32`.
		as_ivec4: function(LuaDVec4):(LuaIVec4)

		--Casts all elements of `self` to `u32`.
		as_uvec4: function(LuaDVec4):(LuaUVec4)

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number),(number)):(LuaDVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaDVec4),(LuaDVec4)):(LuaDVec4)

		-- Meta methods
		metamethod __tostring: function(LuaDVec4):(string)

		metamethod __unm: function(LuaDVec4):(LuaDVec4)

		metamethod __index: function(LuaDVec4,integer):(number)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaDVec4,(integer),(number)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDVec4`].
	record LuaDVec4Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((number),(number),(number),(number)):(LuaDVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(number):(LuaDVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaDVec4),(LuaDVec4)):(LuaDVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 2-dimensional vector.
--
	record LuaIVec2
		userdata

		-- Fields
		x : integer

		y : integer

		-- Pure methods
		--Creates a 3D vector from `self` and the given `z` value.
		extend: function(LuaIVec2,integer):(LuaIVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaIVec2,LuaIVec2):(integer)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaIVec2,LuaIVec2):(LuaIVec2)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaIVec2,LuaIVec2):(LuaIVec2)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaIVec2,(LuaIVec2),(LuaIVec2)):(LuaIVec2)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaIVec2):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaIVec2):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaIVec2,LuaIVec2):(LuaBVec2)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaIVec2):(LuaIVec2)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaIVec2):(LuaIVec2)

		--Returns a vector that is equal to `self` rotated by 90 degrees.
		perp: function(LuaIVec2):(LuaIVec2)

		--The perpendicular dot product of `self` and `rhs`.
		--
		--Also known as the wedge product, 2D cross product, and determinant.
		perp_dot: function(LuaIVec2,LuaIVec2):(integer)

		--Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
		--
		--then this just rotation. This is what you usually want. Otherwise,
		--
		--it will be like a rotation with a multiplication by `self`'s length.
		rotate: function(LuaIVec2,LuaIVec2):(LuaIVec2)

		--Casts all elements of `self` to `f32`.
		as_vec2: function(LuaIVec2):(LuaVec2)

		--Casts all elements of `self` to `f64`.
		as_dvec2: function(LuaIVec2):(LuaDVec2)

		--Casts all elements of `self` to `u32`.
		as_uvec2: function(LuaIVec2):(LuaUVec2)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer)):(LuaIVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaIVec2),(LuaIVec2)):(LuaIVec2)

		-- Meta methods
		metamethod __tostring: function(LuaIVec2):(string)

		metamethod __unm: function(LuaIVec2):(LuaIVec2)

		metamethod __index: function(LuaIVec2,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaIVec2,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaIVec2`].
	record LuaIVec2Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer)):(LuaIVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaIVec2),(LuaIVec2)):(LuaIVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3-dimensional vector.
--
	record LuaIVec3
		userdata

		-- Fields
		x : integer

		y : integer

		z : integer

		-- Pure methods
		--Creates a 4D vector from `self` and the given `w` value.
		extend: function(LuaIVec3,integer):(LuaIVec4)

		--Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
		--
		--
		--
		--Truncation may also be performed by using `self.xy()` or `IVec2::from()`.
		truncate: function(LuaIVec3):(LuaIVec2)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaIVec3,LuaIVec3):(integer)

		--Computes the cross product of `self` and `rhs`.
		cross: function(LuaIVec3,LuaIVec3):(LuaIVec3)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaIVec3,LuaIVec3):(LuaIVec3)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaIVec3,LuaIVec3):(LuaIVec3)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaIVec3,(LuaIVec3),(LuaIVec3)):(LuaIVec3)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaIVec3):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaIVec3):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaIVec3,LuaIVec3):(LuaBVec3)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaIVec3):(LuaIVec3)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaIVec3):(LuaIVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3: function(LuaIVec3):(LuaVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3a: function(LuaIVec3):(LuaVec3A)

		--Casts all elements of `self` to `f64`.
		as_dvec3: function(LuaIVec3):(LuaDVec3)

		--Casts all elements of `self` to `u32`.
		as_uvec3: function(LuaIVec3):(LuaUVec3)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer)):(LuaIVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaIVec3),(LuaIVec3)):(LuaIVec3)

		-- Meta methods
		metamethod __tostring: function(LuaIVec3):(string)

		metamethod __unm: function(LuaIVec3):(LuaIVec3)

		metamethod __index: function(LuaIVec3,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaIVec3,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaIVec3`].
	record LuaIVec3Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer)):(LuaIVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaIVec3),(LuaIVec3)):(LuaIVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 4-dimensional vector.
--
	record LuaIVec4
		userdata

		-- Fields
		x : integer

		y : integer

		z : integer

		w : integer

		-- Pure methods
		--Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
		--
		--
		--
		--Truncation to `IVec3` may also be performed by using `self.xyz()` or `IVec3::from()`.
		truncate: function(LuaIVec4):(LuaIVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaIVec4,LuaIVec4):(integer)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaIVec4,LuaIVec4):(LuaIVec4)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaIVec4,LuaIVec4):(LuaIVec4)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaIVec4,(LuaIVec4),(LuaIVec4)):(LuaIVec4)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaIVec4):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaIVec4):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaIVec4,LuaIVec4):(LuaBVec4)

		--Returns a vector containing the absolute value of each element of `self`.
		abs: function(LuaIVec4):(LuaIVec4)

		--Returns a vector with elements representing the sign of `self`.
		--
		--
		--
		--- `1.0` if the number is positive, `+0.0` or `INFINITY`
		--
		--- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
		--
		--- `NAN` if the number is `NAN`
		signum: function(LuaIVec4):(LuaIVec4)

		--Casts all elements of `self` to `f32`.
		as_vec4: function(LuaIVec4):(LuaVec4)

		--Casts all elements of `self` to `f64`.
		as_dvec4: function(LuaIVec4):(LuaDVec4)

		--Casts all elements of `self` to `u32`.
		as_uvec4: function(LuaIVec4):(LuaUVec4)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer),(integer)):(LuaIVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaIVec4),(LuaIVec4)):(LuaIVec4)

		-- Meta methods
		metamethod __tostring: function(LuaIVec4):(string)

		metamethod __unm: function(LuaIVec4):(LuaIVec4)

		metamethod __index: function(LuaIVec4,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaIVec4,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaIVec4`].
	record LuaIVec4Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer),(integer)):(LuaIVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaIVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaIVec4),(LuaIVec4)):(LuaIVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 2-dimensional vector.
--
	record LuaUVec2
		userdata

		-- Fields
		x : integer

		y : integer

		-- Pure methods
		--Creates a 3D vector from `self` and the given `z` value.
		extend: function(LuaUVec2,integer):(LuaUVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaUVec2,LuaUVec2):(integer)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaUVec2,LuaUVec2):(LuaUVec2)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaUVec2,LuaUVec2):(LuaUVec2)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaUVec2,(LuaUVec2),(LuaUVec2)):(LuaUVec2)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaUVec2):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaUVec2):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaUVec2,LuaUVec2):(LuaBVec2)

		--Casts all elements of `self` to `f32`.
		as_vec2: function(LuaUVec2):(LuaVec2)

		--Casts all elements of `self` to `f64`.
		as_dvec2: function(LuaUVec2):(LuaDVec2)

		--Casts all elements of `self` to `i32`.
		as_ivec2: function(LuaUVec2):(LuaIVec2)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer)):(LuaUVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaUVec2),(LuaUVec2)):(LuaUVec2)

		-- Meta methods
		metamethod __tostring: function(LuaUVec2):(string)

		metamethod __index: function(LuaUVec2,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaUVec2,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaUVec2`].
	record LuaUVec2Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer)):(LuaUVec2)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec2)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec2),(LuaUVec2),(LuaUVec2)):(LuaUVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3-dimensional vector.
--
	record LuaUVec3
		userdata

		-- Fields
		x : integer

		y : integer

		z : integer

		-- Pure methods
		--Creates a 4D vector from `self` and the given `w` value.
		extend: function(LuaUVec3,integer):(LuaUVec4)

		--Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
		--
		--
		--
		--Truncation may also be performed by using `self.xy()` or `UVec2::from()`.
		truncate: function(LuaUVec3):(LuaUVec2)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaUVec3,LuaUVec3):(integer)

		--Computes the cross product of `self` and `rhs`.
		cross: function(LuaUVec3,LuaUVec3):(LuaUVec3)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaUVec3,LuaUVec3):(LuaUVec3)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaUVec3,LuaUVec3):(LuaUVec3)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaUVec3,(LuaUVec3),(LuaUVec3)):(LuaUVec3)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaUVec3):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaUVec3):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaUVec3,LuaUVec3):(LuaBVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3: function(LuaUVec3):(LuaVec3)

		--Casts all elements of `self` to `f32`.
		as_vec3a: function(LuaUVec3):(LuaVec3A)

		--Casts all elements of `self` to `f64`.
		as_dvec3: function(LuaUVec3):(LuaDVec3)

		--Casts all elements of `self` to `i32`.
		as_ivec3: function(LuaUVec3):(LuaIVec3)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer)):(LuaUVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaUVec3),(LuaUVec3)):(LuaUVec3)

		-- Meta methods
		metamethod __tostring: function(LuaUVec3):(string)

		metamethod __index: function(LuaUVec3,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaUVec3,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaUVec3`].
	record LuaUVec3Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer)):(LuaUVec3)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec3)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec3),(LuaUVec3),(LuaUVec3)):(LuaUVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 4-dimensional vector.
--
	record LuaUVec4
		userdata

		-- Fields
		x : integer

		y : integer

		z : integer

		w : integer

		-- Pure methods
		--Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
		--
		--
		--
		--Truncation to `UVec3` may also be performed by using `self.xyz()` or `UVec3::from()`.
		truncate: function(LuaUVec4):(LuaUVec3)

		--Computes the dot product of `self` and `rhs`.
		dot: function(LuaUVec4,LuaUVec4):(integer)

		--Returns a vector containing the minimum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
		min: function(LuaUVec4,LuaUVec4):(LuaUVec4)

		--Returns a vector containing the maximum values for each element of `self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
		max: function(LuaUVec4,LuaUVec4):(LuaUVec4)

		--Component-wise clamping of values, similar to [`f32::clamp`].
		--
		--
		--
		--Each element in `min` must be less-or-equal to the corresponding element in `max`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `min` is greater than `max` when `glam_assert` is enabled.
		clamp: function(LuaUVec4,(LuaUVec4),(LuaUVec4)):(LuaUVec4)

		--Returns the horizontal minimum of `self`.
		--
		--
		--
		--In other words this computes `min(x, y, ..)`.
		min_element: function(LuaUVec4):(integer)

		--Returns the horizontal maximum of `self`.
		--
		--
		--
		--In other words this computes `max(x, y, ..)`.
		max_element: function(LuaUVec4):(integer)

		--Returns a vector mask containing the result of a `==` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
		--
		--elements.
		cmpeq: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `!=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
		--
		--elements.
		cmpne: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
		--
		--elements.
		cmpge: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `>` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
		--
		--elements.
		cmpgt: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<=` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
		--
		--elements.
		cmple: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Returns a vector mask containing the result of a `<` comparison for each element of
		--
		--`self` and `rhs`.
		--
		--
		--
		--In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
		--
		--elements.
		cmplt: function(LuaUVec4,LuaUVec4):(LuaBVec4)

		--Casts all elements of `self` to `f32`.
		as_vec4: function(LuaUVec4):(LuaVec4)

		--Casts all elements of `self` to `f64`.
		as_dvec4: function(LuaUVec4):(LuaDVec4)

		--Casts all elements of `self` to `i32`.
		as_ivec4: function(LuaUVec4):(LuaIVec4)

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer),(integer)):(LuaUVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaUVec4),(LuaUVec4)):(LuaUVec4)

		-- Meta methods
		metamethod __tostring: function(LuaUVec4):(string)

		metamethod __index: function(LuaUVec4,integer):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaUVec4,(integer),(integer)):()

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaUVec4`].
	record LuaUVec4Class
		userdata

		-- Pure functions
		--Creates a new vector.
		new: function((integer),(integer),(integer),(integer)):(LuaUVec4)

		--Creates a vector with all elements set to `v`.
		splat: function(integer):(LuaUVec4)

		--Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
		--
		--for each element of `self`.
		--
		--
		--
		--A true element in the mask uses the corresponding element from `if_true`, and false
		--
		--uses the element from `if_false`.
		select: function((LuaBVec4),(LuaUVec4),(LuaUVec4)):(LuaUVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)

		metamethod __mod: function((any),(any)):(any)


	end
--A 3x3 column major matrix.
--
--
--
--This 3x3 matrix type features convenience methods for creating and using linear and
--
--affine transformations. If you are primarily dealing with 2D affine transformations the
--
--[`Affine2`](crate::Affine2) type is much faster and more space efficient than
--
--using a 3x3 matrix.
--
--
--
--Linear transformations including 3D rotation and scale can be created using methods
--
--such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
--
--[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
--
--[`Self::from_rotation_z()`].
--
--
--
--The resulting matrices can be use to transform 3D vectors using regular vector
--
--multiplication.
--
--
--
--Affine transformations including 2D translation, rotation and scale can be created
--
--using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
--
--[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
--
--
--
--The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
--
--are provided for performing affine transforms on 2D vectors and points. These multiply
--
--2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
--
--vectors respectively. These methods assume that `Self` contains a valid affine
--
--transform.
--
	record LuaMat3
		userdata

		-- Fields
		x_axis : LuaVec3

		y_axis : LuaVec3

		z_axis : LuaVec3

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		col: function(LuaMat3,integer):(LuaVec3)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		row: function(LuaMat3,integer):(LuaVec3)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaMat3):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaMat3):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaMat3):(LuaMat3)

		--Returns the determinant of `self`.
		determinant: function(LuaMat3):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaMat3):(LuaMat3)

		--Transforms the given 2D vector as a point.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_point2: function(LuaMat3,LuaVec2):(LuaVec2)

		--Rotates the given 2D vector.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_vector2: function(LuaMat3,LuaVec2):(LuaVec2)

		--Transforms a 3D vector.
		mul_vec3: function(LuaMat3,LuaVec3):(LuaVec3)

		--Transforms a `Vec3A`.
		mul_vec3a: function(LuaMat3,LuaVec3A):(LuaVec3A)

		--Multiplies two 3x3 matrices.
		mul_mat3: function(LuaMat3,LuaMat3):(LuaMat3)

		--Adds two 3x3 matrices.
		add_mat3: function(LuaMat3,LuaMat3):(LuaMat3)

		--Subtracts two 3x3 matrices.
		sub_mat3: function(LuaMat3,LuaMat3):(LuaMat3)

		--Multiplies a 3x3 matrix by a scalar.
		mul_scalar: function(LuaMat3,number):(LuaMat3)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaMat3,(LuaMat3),(number)):(boolean)

		as_dmat3: function(LuaMat3):(LuaDMat3)

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat3)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec3):(LuaMat3)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaMat4):(LuaMat3)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat3)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat3)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaVec2):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaMat3)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec2):(LuaMat3)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaMat2):(LuaMat3)

		-- Meta methods
		metamethod __tostring: function(LuaMat3):(string)

		metamethod __unm: function(LuaMat3):(LuaMat3)

		-- Mutating MetaMethods
		metamethod __index: function(LuaMat3,integer):(LuaVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaMat3`].
	record LuaMat3Class
		userdata

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat3)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec3):(LuaMat3)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaMat4):(LuaMat3)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat3)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat3)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaVec2):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaMat3)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaMat3)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec2):(LuaMat3)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaMat2):(LuaMat3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 2x2 column major matrix.
--
	record LuaMat2
		userdata

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 1.
		col: function(LuaMat2,integer):(LuaVec2)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 1.
		row: function(LuaMat2,integer):(LuaVec2)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaMat2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaMat2):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaMat2):(LuaMat2)

		--Returns the determinant of `self`.
		determinant: function(LuaMat2):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaMat2):(LuaMat2)

		--Transforms a 2D vector.
		mul_vec2: function(LuaMat2,LuaVec2):(LuaVec2)

		--Multiplies two 2x2 matrices.
		mul_mat2: function(LuaMat2,LuaMat2):(LuaMat2)

		--Adds two 2x2 matrices.
		add_mat2: function(LuaMat2,LuaMat2):(LuaMat2)

		--Subtracts two 2x2 matrices.
		sub_mat2: function(LuaMat2,LuaMat2):(LuaMat2)

		--Multiplies a 2x2 matrix by a scalar.
		mul_scalar: function(LuaMat2,number):(LuaMat2)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaMat2,(LuaMat2),(number)):(boolean)

		as_dmat2: function(LuaMat2):(LuaDMat2)

		-- Pure functions
		--Creates a 2x2 matrix from two column vectors.
		from_cols: function((LuaVec2),(LuaVec2)):(LuaMat2)

		--Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec2):(LuaMat2)

		--Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
		--
		--`angle` (in radians).
		from_scale_angle: function((LuaVec2),(number)):(LuaMat2)

		--Creates a 2x2 matrix containing a rotation of `angle` (in radians).
		from_angle: function(number):(LuaMat2)

		--Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
		from_mat3: function(LuaMat3):(LuaMat2)

		-- Meta methods
		metamethod __tostring: function(LuaMat2):(string)

		metamethod __unm: function(LuaMat2):(LuaMat2)

		-- Mutating MetaMethods
		metamethod __index: function(LuaMat2,integer):(LuaVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaMat2`].
	record LuaMat2Class
		userdata

		-- Pure functions
		--Creates a 2x2 matrix from two column vectors.
		from_cols: function((LuaVec2),(LuaVec2)):(LuaMat2)

		--Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec2):(LuaMat2)

		--Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
		--
		--`angle` (in radians).
		from_scale_angle: function((LuaVec2),(number)):(LuaMat2)

		--Creates a 2x2 matrix containing a rotation of `angle` (in radians).
		from_angle: function(number):(LuaMat2)

		--Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
		from_mat3: function(LuaMat3):(LuaMat2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 3x3 column major matrix.
--
--
--
--This 3x3 matrix type features convenience methods for creating and using linear and
--
--affine transformations. If you are primarily dealing with 2D affine transformations the
--
--[`Affine2`](crate::Affine2) type is much faster and more space efficient than
--
--using a 3x3 matrix.
--
--
--
--Linear transformations including 3D rotation and scale can be created using methods
--
--such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
--
--[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
--
--[`Self::from_rotation_z()`].
--
--
--
--The resulting matrices can be use to transform 3D vectors using regular vector
--
--multiplication.
--
--
--
--Affine transformations including 2D translation, rotation and scale can be created
--
--using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
--
--[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
--
--
--
--The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
--
--are provided for performing affine transforms on 2D vectors and points. These multiply
--
--2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
--
--vectors respectively. These methods assume that `Self` contains a valid affine
--
--transform.
--
	record LuaMat3A
		userdata

		-- Fields
		x_axis : LuaVec3A

		y_axis : LuaVec3A

		z_axis : LuaVec3A

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		col: function(LuaMat3A,integer):(LuaVec3A)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		row: function(LuaMat3A,integer):(LuaVec3A)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaMat3A):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaMat3A):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaMat3A):(LuaMat3A)

		--Returns the determinant of `self`.
		determinant: function(LuaMat3A):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaMat3A):(LuaMat3A)

		--Transforms the given 2D vector as a point.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_point2: function(LuaMat3A,LuaVec2):(LuaVec2)

		--Rotates the given 2D vector.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_vector2: function(LuaMat3A,LuaVec2):(LuaVec2)

		--Transforms a 3D vector.
		mul_vec3: function(LuaMat3A,LuaVec3):(LuaVec3)

		--Transforms a `Vec3A`.
		mul_vec3a: function(LuaMat3A,LuaVec3A):(LuaVec3A)

		--Multiplies two 3x3 matrices.
		mul_mat3: function(LuaMat3A,LuaMat3A):(LuaMat3A)

		--Adds two 3x3 matrices.
		add_mat3: function(LuaMat3A,LuaMat3A):(LuaMat3A)

		--Subtracts two 3x3 matrices.
		sub_mat3: function(LuaMat3A,LuaMat3A):(LuaMat3A)

		--Multiplies a 3x3 matrix by a scalar.
		mul_scalar: function(LuaMat3A,number):(LuaMat3A)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaMat3A,(LuaMat3A),(number)):(boolean)

		as_dmat3: function(LuaMat3A):(LuaDMat3)

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaMat3A)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec3):(LuaMat3A)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaMat4):(LuaMat3A)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat3A)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat3A)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaVec2):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaMat3A)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec2):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaMat2):(LuaMat3A)

		-- Meta methods
		metamethod __tostring: function(LuaMat3A):(string)

		metamethod __unm: function(LuaMat3A):(LuaMat3A)

		-- Mutating MetaMethods
		metamethod __index: function(LuaMat3A,integer):(LuaVec3A)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaMat3A`].
	record LuaMat3AClass
		userdata

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaMat3A)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec3):(LuaMat3A)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaMat4):(LuaMat3A)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat3A)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat3A)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaMat3A)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaVec2):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaMat3A)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec2):(LuaMat3A)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaMat2):(LuaMat3A)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 4x4 column major matrix.
--
--
--
--This 4x4 matrix type features convenience methods for creating and using affine transforms and
--
--perspective projections. If you are primarily dealing with 3D affine transformations
--
--considering using [`Affine3A`](crate::Affine3A) which is faster than a 4x4 matrix
--
--for some affine operations.
--
--
--
--Affine transformations including 3D translation, rotation and scale can be created
--
--using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
--
--[`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
--
--
--
--Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
--
--left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
--
--systems. The resulting matrix is also an affine transformation.
--
--
--
--The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
--
--are provided for performing affine transformations on 3D vectors and points. These
--
--multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
--
--for vectors respectively. These methods assume that `Self` contains a valid affine
--
--transform.
--
--
--
--Perspective projections can be created using methods such as
--
--[`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
--
--[`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
--
--[`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
--
--[`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
--
--
--
--The resulting perspective project can be use to transform 3D vectors as points with
--
--perspective correction using the [`Self::project_point3()`] convenience method.
--
	record LuaMat4
		userdata

		-- Fields
		x_axis : LuaVec4

		y_axis : LuaVec4

		z_axis : LuaVec4

		w_axis : LuaVec4

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 3.
		col: function(LuaMat4,integer):(LuaVec4)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 3.
		row: function(LuaMat4,integer):(LuaVec4)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaMat4):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaMat4):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaMat4):(LuaMat4)

		--Returns the determinant of `self`.
		determinant: function(LuaMat4):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaMat4):(LuaMat4)

		--Transforms the given 3D vector as a point, applying perspective correction.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
		--
		--The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
		--
		--
		--
		--This method assumes that `self` contains a projective transform.
		project_point3: function(LuaMat4,LuaVec3):(LuaVec3)

		--Transforms the given 3D vector as a point.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
		--
		--`1.0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform. It does not perform
		--
		--a persective divide, if `self` contains a perspective transform, or if you are unsure,
		--
		--the [`Self::project_point3()`] method should be used instead.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
		transform_point3: function(LuaMat4,LuaVec3):(LuaVec3)

		--Transforms the give 3D vector as a direction.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
		--
		--`0.0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
		transform_vector3: function(LuaMat4,LuaVec3):(LuaVec3)

		--Transforms the given `Vec3A` as 3D point.
		--
		--
		--
		--This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `1.0`.
		transform_point3a: function(LuaMat4,LuaVec3A):(LuaVec3A)

		--Transforms the give `Vec3A` as 3D vector.
		--
		--
		--
		--This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `0.0`.
		transform_vector3a: function(LuaMat4,LuaVec3A):(LuaVec3A)

		--Transforms a 4D vector.
		mul_vec4: function(LuaMat4,LuaVec4):(LuaVec4)

		--Multiplies two 4x4 matrices.
		mul_mat4: function(LuaMat4,LuaMat4):(LuaMat4)

		--Adds two 4x4 matrices.
		add_mat4: function(LuaMat4,LuaMat4):(LuaMat4)

		--Subtracts two 4x4 matrices.
		sub_mat4: function(LuaMat4,LuaMat4):(LuaMat4)

		--Multiplies a 4x4 matrix by a scalar.
		mul_scalar: function(LuaMat4,number):(LuaMat4)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaMat4,(LuaMat4),(number)):(boolean)

		as_dmat4: function(LuaMat4):(LuaDMat4)

		-- Pure functions
		--Creates a 4x4 matrix from two column vectors.
		from_cols: function((LuaVec4),(LuaVec4),(LuaVec4),(LuaVec4)):(LuaMat4)

		--Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec4):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_scale_rotation_translation: function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_rotation_translation: function((LuaQuat),(LuaVec3)):(LuaMat4)

		--Creates an affine transformation matrix from the given `rotation` quaternion.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat4)

		--Creates an affine transformation matrix from the given 3x3 linear transformation
		--
		--matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_mat3: function(LuaMat3):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_translation: function(LuaVec3):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat4)

		--Creates a affine transformation matrix containing a rotation from the given euler
		--
		--rotation sequence and angles (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_x: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_y: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_z: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec3):(LuaMat4)

		--Creates a left-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

		--Creates a right-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

		--Creates a right-handed perspective projection matrix with [-1,1] depth range.
		--
		--This is the same as the OpenGL `gluPerspective` function.
		--
		--See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
		perspective_rh_gl: function((number),(number),(number),(number)):(LuaMat4)

		--Creates a left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_lh: function((number),(number),(number),(number)):(LuaMat4)

		--Creates a right-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_rh: function((number),(number),(number),(number)):(LuaMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_lh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_reverse_lh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite right-handed perspective projection matrix with
		--
		--`[0,1]` depth range.
		perspective_infinite_rh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite reverse right-handed perspective projection matrix
		--
		--with `[0,1]` depth range.
		perspective_infinite_reverse_rh: function((number),(number),(number)):(LuaMat4)

		--Creates a right-handed orthographic projection matrix with `[-1,1]` depth
		--
		--range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
		--
		--See
		--
		--<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
		orthographic_rh_gl: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		--Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_lh: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		--Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_rh: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		-- Meta methods
		metamethod __tostring: function(LuaMat4):(string)

		metamethod __unm: function(LuaMat4):(LuaMat4)

		-- Mutating MetaMethods
		metamethod __index: function(LuaMat4,integer):(LuaVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaMat4`].
	record LuaMat4Class
		userdata

		-- Pure functions
		--Creates a 4x4 matrix from two column vectors.
		from_cols: function((LuaVec4),(LuaVec4),(LuaVec4),(LuaVec4)):(LuaMat4)

		--Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaVec4):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_scale_rotation_translation: function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_rotation_translation: function((LuaQuat),(LuaVec3)):(LuaMat4)

		--Creates an affine transformation matrix from the given `rotation` quaternion.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaQuat):(LuaMat4)

		--Creates an affine transformation matrix from the given 3x3 linear transformation
		--
		--matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_mat3: function(LuaMat3):(LuaMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_translation: function(LuaVec3):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaMat4)

		--Creates a affine transformation matrix containing a rotation from the given euler
		--
		--rotation sequence and angles (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_x: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_y: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_z: function(number):(LuaMat4)

		--Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaVec3):(LuaMat4)

		--Creates a left-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

		--Creates a right-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

		--Creates a right-handed perspective projection matrix with [-1,1] depth range.
		--
		--This is the same as the OpenGL `gluPerspective` function.
		--
		--See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
		perspective_rh_gl: function((number),(number),(number),(number)):(LuaMat4)

		--Creates a left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_lh: function((number),(number),(number),(number)):(LuaMat4)

		--Creates a right-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_rh: function((number),(number),(number),(number)):(LuaMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_lh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_reverse_lh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite right-handed perspective projection matrix with
		--
		--`[0,1]` depth range.
		perspective_infinite_rh: function((number),(number),(number)):(LuaMat4)

		--Creates an infinite reverse right-handed perspective projection matrix
		--
		--with `[0,1]` depth range.
		perspective_infinite_reverse_rh: function((number),(number),(number)):(LuaMat4)

		--Creates a right-handed orthographic projection matrix with `[-1,1]` depth
		--
		--range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
		--
		--See
		--
		--<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
		orthographic_rh_gl: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		--Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_lh: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		--Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_rh: function((number),(number),(number),(number),(number),(number)):(LuaMat4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 2x2 column major matrix.
--
	record LuaDMat2
		userdata

		-- Fields
		x_axis : LuaDVec2

		y_axis : LuaDVec2

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 1.
		col: function(LuaDMat2,integer):(LuaDVec2)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 1.
		row: function(LuaDMat2,integer):(LuaDVec2)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDMat2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDMat2):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaDMat2):(LuaDMat2)

		--Returns the determinant of `self`.
		determinant: function(LuaDMat2):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaDMat2):(LuaDMat2)

		--Transforms a 2D vector.
		mul_vec2: function(LuaDMat2,LuaDVec2):(LuaDVec2)

		--Multiplies two 2x2 matrices.
		mul_mat2: function(LuaDMat2,LuaDMat2):(LuaDMat2)

		--Adds two 2x2 matrices.
		add_mat2: function(LuaDMat2,LuaDMat2):(LuaDMat2)

		--Subtracts two 2x2 matrices.
		sub_mat2: function(LuaDMat2,LuaDMat2):(LuaDMat2)

		--Multiplies a 2x2 matrix by a scalar.
		mul_scalar: function(LuaDMat2,number):(LuaDMat2)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDMat2,(LuaDMat2),(number)):(boolean)

		as_mat2: function(LuaDMat2):(LuaMat2)

		-- Pure functions
		--Creates a 2x2 matrix from two column vectors.
		from_cols: function((LuaDVec2),(LuaDVec2)):(LuaDMat2)

		--Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec2):(LuaDMat2)

		--Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
		--
		--`angle` (in radians).
		from_scale_angle: function((LuaDVec2),(number)):(LuaDMat2)

		--Creates a 2x2 matrix containing a rotation of `angle` (in radians).
		from_angle: function(number):(LuaDMat2)

		--Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
		from_mat3: function(LuaDMat3):(LuaDMat2)

		-- Meta methods
		metamethod __tostring: function(LuaDMat2):(string)

		metamethod __unm: function(LuaDMat2):(LuaDMat2)

		-- Mutating MetaMethods
		metamethod __index: function(LuaDMat2,integer):(LuaDVec2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDMat2`].
	record LuaDMat2Class
		userdata

		-- Pure functions
		--Creates a 2x2 matrix from two column vectors.
		from_cols: function((LuaDVec2),(LuaDVec2)):(LuaDMat2)

		--Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec2):(LuaDMat2)

		--Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
		--
		--`angle` (in radians).
		from_scale_angle: function((LuaDVec2),(number)):(LuaDMat2)

		--Creates a 2x2 matrix containing a rotation of `angle` (in radians).
		from_angle: function(number):(LuaDMat2)

		--Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
		from_mat3: function(LuaDMat3):(LuaDMat2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 3x3 column major matrix.
--
--
--
--This 3x3 matrix type features convenience methods for creating and using linear and
--
--affine transformations. If you are primarily dealing with 2D affine transformations the
--
--[`DAffine2`](crate::DAffine2) type is much faster and more space efficient than
--
--using a 3x3 matrix.
--
--
--
--Linear transformations including 3D rotation and scale can be created using methods
--
--such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
--
--[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
--
--[`Self::from_rotation_z()`].
--
--
--
--The resulting matrices can be use to transform 3D vectors using regular vector
--
--multiplication.
--
--
--
--Affine transformations including 2D translation, rotation and scale can be created
--
--using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
--
--[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
--
--
--
--The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
--
--are provided for performing affine transforms on 2D vectors and points. These multiply
--
--2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
--
--vectors respectively. These methods assume that `Self` contains a valid affine
--
--transform.
--
	record LuaDMat3
		userdata

		-- Fields
		x_axis : LuaDVec3

		y_axis : LuaDVec3

		z_axis : LuaDVec3

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		col: function(LuaDMat3,integer):(LuaDVec3)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 2.
		row: function(LuaDMat3,integer):(LuaDVec3)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDMat3):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDMat3):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaDMat3):(LuaDMat3)

		--Returns the determinant of `self`.
		determinant: function(LuaDMat3):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaDMat3):(LuaDMat3)

		--Transforms the given 2D vector as a point.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_point2: function(LuaDMat3,LuaDVec2):(LuaDVec2)

		--Rotates the given 2D vector.
		--
		--
		--
		--This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		transform_vector2: function(LuaDMat3,LuaDVec2):(LuaDVec2)

		--Transforms a 3D vector.
		mul_vec3: function(LuaDMat3,LuaDVec3):(LuaDVec3)

		--Multiplies two 3x3 matrices.
		mul_mat3: function(LuaDMat3,LuaDMat3):(LuaDMat3)

		--Adds two 3x3 matrices.
		add_mat3: function(LuaDMat3,LuaDMat3):(LuaDMat3)

		--Subtracts two 3x3 matrices.
		sub_mat3: function(LuaDMat3,LuaDMat3):(LuaDMat3)

		--Multiplies a 3x3 matrix by a scalar.
		mul_scalar: function(LuaDMat3,number):(LuaDMat3)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDMat3,(LuaDMat3),(number)):(boolean)

		as_mat3: function(LuaDMat3):(LuaMat3)

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat3)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec3):(LuaDMat3)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaDMat4):(LuaDMat3)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaDQuat):(LuaDMat3)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDMat3)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaDVec2):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaDVec2),(number),(LuaDVec2)):(LuaDMat3)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaDVec2):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaDMat2):(LuaDMat3)

		-- Meta methods
		metamethod __tostring: function(LuaDMat3):(string)

		metamethod __unm: function(LuaDMat3):(LuaDMat3)

		-- Mutating MetaMethods
		metamethod __index: function(LuaDMat3,integer):(LuaDVec3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDMat3`].
	record LuaDMat3Class
		userdata

		-- Pure functions
		--Creates a 3x3 matrix from two column vectors.
		from_cols: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat3)

		--Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec3):(LuaDMat3)

		--Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
		from_mat4: function(LuaDMat4):(LuaDMat3)

		--Creates a 3D rotation matrix from the given quaternion.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaDQuat):(LuaDMat3)

		--Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
		--
		--radians).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDMat3)

		--Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
		--
		--radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaDMat3)

		--Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_translation: function(LuaDVec2):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D rotation `angle` (in
		--
		--radians).
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_angle: function(number):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
		--
		--radians) and `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_scale_angle_translation: function((LuaDVec2),(number),(LuaDVec2)):(LuaDMat3)

		--Creates an affine transformation matrix from the given non-uniform 2D `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaDVec2):(LuaDMat3)

		--Creates an affine transformation matrix from the given 2x2 matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 2D points and vectors. See
		--
		--[`Self::transform_point2()`] and [`Self::transform_vector2()`].
		from_mat2: function(LuaDMat2):(LuaDMat3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 4x4 column major matrix.
--
--
--
--This 4x4 matrix type features convenience methods for creating and using affine transforms and
--
--perspective projections. If you are primarily dealing with 3D affine transformations
--
--considering using [`DAffine3`](crate::DAffine3) which is faster than a 4x4 matrix
--
--for some affine operations.
--
--
--
--Affine transformations including 3D translation, rotation and scale can be created
--
--using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
--
--[`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
--
--
--
--Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
--
--left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
--
--systems. The resulting matrix is also an affine transformation.
--
--
--
--The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
--
--are provided for performing affine transformations on 3D vectors and points. These
--
--multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
--
--for vectors respectively. These methods assume that `Self` contains a valid affine
--
--transform.
--
--
--
--Perspective projections can be created using methods such as
--
--[`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
--
--[`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
--
--[`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
--
--[`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
--
--
--
--The resulting perspective project can be use to transform 3D vectors as points with
--
--perspective correction using the [`Self::project_point3()`] convenience method.
--
	record LuaDMat4
		userdata

		-- Fields
		x_axis : LuaDVec4

		y_axis : LuaDVec4

		z_axis : LuaDVec4

		w_axis : LuaDVec4

		-- Pure methods
		--Returns the matrix column for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 3.
		col: function(LuaDMat4,integer):(LuaDVec4)

		--Returns the matrix row for the given `index`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Panics if `index` is greater than 3.
		row: function(LuaDMat4,integer):(LuaDVec4)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDMat4):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDMat4):(boolean)

		--Returns the transpose of `self`.
		transpose: function(LuaDMat4):(LuaDMat4)

		--Returns the determinant of `self`.
		determinant: function(LuaDMat4):(number)

		--Returns the inverse of `self`.
		--
		--
		--
		--If the matrix is not invertible the returned matrix will be invalid.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
		inverse: function(LuaDMat4):(LuaDMat4)

		--Transforms the given 3D vector as a point, applying perspective correction.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
		--
		--The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
		--
		--
		--
		--This method assumes that `self` contains a projective transform.
		project_point3: function(LuaDMat4,LuaDVec3):(LuaDVec3)

		--Transforms the given 3D vector as a point.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
		--
		--`1.0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform. It does not perform
		--
		--a persective divide, if `self` contains a perspective transform, or if you are unsure,
		--
		--the [`Self::project_point3()`] method should be used instead.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
		transform_point3: function(LuaDMat4,LuaDVec3):(LuaDVec3)

		--Transforms the give 3D vector as a direction.
		--
		--
		--
		--This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
		--
		--`0.0`.
		--
		--
		--
		--This method assumes that `self` contains a valid affine transform.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
		transform_vector3: function(LuaDMat4,LuaDVec3):(LuaDVec3)

		--Transforms a 4D vector.
		mul_vec4: function(LuaDMat4,LuaDVec4):(LuaDVec4)

		--Multiplies two 4x4 matrices.
		mul_mat4: function(LuaDMat4,LuaDMat4):(LuaDMat4)

		--Adds two 4x4 matrices.
		add_mat4: function(LuaDMat4,LuaDMat4):(LuaDMat4)

		--Subtracts two 4x4 matrices.
		sub_mat4: function(LuaDMat4,LuaDMat4):(LuaDMat4)

		--Multiplies a 4x4 matrix by a scalar.
		mul_scalar: function(LuaDMat4,number):(LuaDMat4)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two matrices contain similar elements. It works best
		--
		--when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDMat4,(LuaDMat4),(number)):(boolean)

		as_mat4: function(LuaDMat4):(LuaMat4)

		-- Pure functions
		--Creates a 4x4 matrix from two column vectors.
		from_cols: function((LuaDVec4),(LuaDVec4),(LuaDVec4),(LuaDVec4)):(LuaDMat4)

		--Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec4):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_scale_rotation_translation: function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_rotation_translation: function((LuaDQuat),(LuaDVec3)):(LuaDMat4)

		--Creates an affine transformation matrix from the given `rotation` quaternion.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaDQuat):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3x3 linear transformation
		--
		--matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_mat3: function(LuaDMat3):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_translation: function(LuaDVec3):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDMat4)

		--Creates a affine transformation matrix containing a rotation from the given euler
		--
		--rotation sequence and angles (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_x: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_y: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_z: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaDVec3):(LuaDMat4)

		--Creates a left-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

		--Creates a right-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

		--Creates a right-handed perspective projection matrix with [-1,1] depth range.
		--
		--This is the same as the OpenGL `gluPerspective` function.
		--
		--See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
		perspective_rh_gl: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates a left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_lh: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates a right-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_rh: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_lh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_reverse_lh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite right-handed perspective projection matrix with
		--
		--`[0,1]` depth range.
		perspective_infinite_rh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite reverse right-handed perspective projection matrix
		--
		--with `[0,1]` depth range.
		perspective_infinite_reverse_rh: function((number),(number),(number)):(LuaDMat4)

		--Creates a right-handed orthographic projection matrix with `[-1,1]` depth
		--
		--range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
		--
		--See
		--
		--<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
		orthographic_rh_gl: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		--Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_lh: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		--Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_rh: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		-- Meta methods
		metamethod __tostring: function(LuaDMat4):(string)

		metamethod __unm: function(LuaDMat4):(LuaDMat4)

		-- Mutating MetaMethods
		metamethod __index: function(LuaDMat4,integer):(LuaDVec4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDMat4`].
	record LuaDMat4Class
		userdata

		-- Pure functions
		--Creates a 4x4 matrix from two column vectors.
		from_cols: function((LuaDVec4),(LuaDVec4),(LuaDVec4),(LuaDVec4)):(LuaDMat4)

		--Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
		from_diagonal: function(LuaDVec4):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_scale_rotation_translation: function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_rotation_translation: function((LuaDQuat),(LuaDVec3)):(LuaDMat4)

		--Creates an affine transformation matrix from the given `rotation` quaternion.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `rotation` is not normalized when `glam_assert` is enabled.
		from_quat: function(LuaDQuat):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3x3 linear transformation
		--
		--matrix.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_mat3: function(LuaDMat3):(LuaDMat4)

		--Creates an affine transformation matrix from the given 3D `translation`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_translation: function(LuaDVec3):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDMat4)

		--Creates a affine transformation matrix containing a rotation from the given euler
		--
		--rotation sequence and angles (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_x: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_y: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		from_rotation_z: function(number):(LuaDMat4)

		--Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
		--
		--
		--
		--The resulting matrix can be used to transform 3D points and vectors. See
		--
		--[`Self::transform_point3()`] and [`Self::transform_vector3()`].
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
		from_scale: function(LuaDVec3):(LuaDMat4)

		--Creates a left-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

		--Creates a right-handed view matrix using a camera position, an up direction, and a focal
		--
		--point.
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

		--Creates a right-handed perspective projection matrix with [-1,1] depth range.
		--
		--This is the same as the OpenGL `gluPerspective` function.
		--
		--See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
		perspective_rh_gl: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates a left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_lh: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates a right-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
		--
		--enabled.
		perspective_rh: function((number),(number),(number),(number)):(LuaDMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_lh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
		perspective_infinite_reverse_lh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite right-handed perspective projection matrix with
		--
		--`[0,1]` depth range.
		perspective_infinite_rh: function((number),(number),(number)):(LuaDMat4)

		--Creates an infinite reverse right-handed perspective projection matrix
		--
		--with `[0,1]` depth range.
		perspective_infinite_reverse_rh: function((number),(number),(number)):(LuaDMat4)

		--Creates a right-handed orthographic projection matrix with `[-1,1]` depth
		--
		--range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
		--
		--See
		--
		--<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
		orthographic_rh_gl: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		--Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_lh: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		--Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
		orthographic_rh: function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 2D affine transform, which can represent translation, rotation, scaling and shear.
--
	record LuaAffine2
		userdata

		-- Fields
		matrix2 : LuaMat2

		translation : LuaVec2

		-- Pure methods
		--Transforms the given 2D point, applying shear, scale, rotation and translation.
		transform_point2: function(LuaAffine2,LuaVec2):(LuaVec2)

		--Transforms the given 2D vector, applying shear, scale and rotation (but NOT
		--
		--translation).
		--
		--
		--
		--To also apply translation, use [`Self::transform_point2`] instead.
		transform_vector2: function(LuaAffine2,LuaVec2):(LuaVec2)

		--Returns `true` if, and only if, all elements are finite.
		--
		--
		--
		--If any element is either `NaN`, positive or negative infinity, this will return
		--
		--`false`.
		is_finite: function(LuaAffine2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaAffine2):(boolean)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two 3x4 matrices contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaAffine2,(LuaAffine2),(number)):(boolean)

		--Return the inverse of this transform.
		--
		--
		--
		--Note that if the transform is not invertible the result will be invalid.
		inverse: function(LuaAffine2):(LuaAffine2)

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaVec2),(LuaVec2),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaVec2):(LuaAffine2)

		--Creates an affine transform from the given rotation `angle`.
		from_angle: function(number):(LuaAffine2)

		--Creates an affine transformation from the given 2D `translation`.
		from_translation: function(LuaVec2):(LuaAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
		from_mat2: function(LuaMat2):(LuaAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
		--
		--translation vector.
		--
		--
		--
		--Equivalent to
		--
		--`Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`
		from_mat2_translation: function((LuaMat2),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine2::from_translation(translation) *
		--
		--Affine2::from_angle(angle) * Affine2::from_scale(scale)`
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform from the given 2D rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`
		from_angle_translation: function((number),(LuaVec2)):(LuaAffine2)

		--The given `Mat3` must be an affine transform,
		from_mat3: function(LuaMat3):(LuaAffine2)

		-- Meta methods
		metamethod __tostring: function(LuaAffine2):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaAffine2`].
	record LuaAffine2Class
		userdata

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaVec2),(LuaVec2),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaVec2):(LuaAffine2)

		--Creates an affine transform from the given rotation `angle`.
		from_angle: function(number):(LuaAffine2)

		--Creates an affine transformation from the given 2D `translation`.
		from_translation: function(LuaVec2):(LuaAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
		from_mat2: function(LuaMat2):(LuaAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
		--
		--translation vector.
		--
		--
		--
		--Equivalent to
		--
		--`Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`
		from_mat2_translation: function((LuaMat2),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine2::from_translation(translation) *
		--
		--Affine2::from_angle(angle) * Affine2::from_scale(scale)`
		from_scale_angle_translation: function((LuaVec2),(number),(LuaVec2)):(LuaAffine2)

		--Creates an affine transform from the given 2D rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`
		from_angle_translation: function((number),(LuaVec2)):(LuaAffine2)

		--The given `Mat3` must be an affine transform,
		from_mat3: function(LuaMat3):(LuaAffine2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 3D affine transform, which can represent translation, rotation, scaling and shear.
--
	record LuaAffine3A
		userdata

		-- Fields
		matrix3 : LuaMat3A

		translation : LuaVec3A

		-- Pure methods
		--Transforms the given 3D points, applying shear, scale, rotation and translation.
		transform_point3: function(LuaAffine3A,LuaVec3):(LuaVec3)

		--Transforms the given 3D vector, applying shear, scale and rotation (but NOT
		--
		--translation).
		--
		--
		--
		--To also apply translation, use [`Self::transform_point3`] instead.
		transform_vector3: function(LuaAffine3A,LuaVec3):(LuaVec3)

		--Transforms the given `Vec3A`, applying shear, scale, rotation and translation.
		transform_point3a: function(LuaAffine3A,LuaVec3A):(LuaVec3A)

		--Transforms the given `Vec3A`, applying shear, scale and rotation (but NOT
		--
		--translation).
		--
		--
		--
		--To also apply translation, use [`Self::transform_point3`] instead.
		transform_vector3a: function(LuaAffine3A,LuaVec3A):(LuaVec3A)

		--Returns `true` if, and only if, all elements are finite.
		--
		--
		--
		--If any element is either `NaN`, positive or negative infinity, this will return
		--
		--`false`.
		is_finite: function(LuaAffine3A):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaAffine3A):(boolean)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two 3x4 matrices contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaAffine3A,(LuaAffine3A),(number)):(boolean)

		--Return the inverse of this transform.
		--
		--
		--
		--Note that if the transform is not invertible the result will be invalid.
		inverse: function(LuaAffine3A):(LuaAffine3A)

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaVec3A),(LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaAffine3A)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaVec3):(LuaAffine3A)

		--Creates an affine transform from the given `rotation` quaternion.
		from_quat: function(LuaQuat):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		from_axis_angle: function((LuaVec3),(number)):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		from_rotation_x: function(number):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		from_rotation_y: function(number):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		from_rotation_z: function(number):(LuaAffine3A)

		--Creates an affine transformation from the given 3D `translation`.
		from_translation: function(LuaVec3):(LuaAffine3A)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and
		--
		--rotation)
		from_mat3: function(LuaMat3):(LuaAffine3A)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
		--
		--and a translation vector.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`
		from_mat3_translation: function((LuaMat3),(LuaVec3)):(LuaAffine3A)

		--Creates an affine transform from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) *
		--
		--Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`
		from_scale_rotation_translation: function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaAffine3A)

		--Creates an affine transform from the given 3D `rotation` and `translation`.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`
		from_rotation_translation: function((LuaQuat),(LuaVec3)):(LuaAffine3A)

		--The given `Mat4` must be an affine transform,
		--
		--i.e. contain no perspective transform.
		from_mat4: function(LuaMat4):(LuaAffine3A)

		--Creates a left-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

		--Creates a right-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

		-- Meta methods
		metamethod __tostring: function(LuaAffine3A):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaAffine3A`].
	record LuaAffine3AClass
		userdata

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaVec3A),(LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaAffine3A)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaVec3):(LuaAffine3A)

		--Creates an affine transform from the given `rotation` quaternion.
		from_quat: function(LuaQuat):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		from_axis_angle: function((LuaVec3),(number)):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		from_rotation_x: function(number):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		from_rotation_y: function(number):(LuaAffine3A)

		--Creates an affine transform containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		from_rotation_z: function(number):(LuaAffine3A)

		--Creates an affine transformation from the given 3D `translation`.
		from_translation: function(LuaVec3):(LuaAffine3A)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and
		--
		--rotation)
		from_mat3: function(LuaMat3):(LuaAffine3A)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
		--
		--and a translation vector.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`
		from_mat3_translation: function((LuaMat3),(LuaVec3)):(LuaAffine3A)

		--Creates an affine transform from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) *
		--
		--Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`
		from_scale_rotation_translation: function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaAffine3A)

		--Creates an affine transform from the given 3D `rotation` and `translation`.
		--
		--
		--
		--Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`
		from_rotation_translation: function((LuaQuat),(LuaVec3)):(LuaAffine3A)

		--The given `Mat4` must be an affine transform,
		--
		--i.e. contain no perspective transform.
		from_mat4: function(LuaMat4):(LuaAffine3A)

		--Creates a left-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

		--Creates a right-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 2D affine transform, which can represent translation, rotation, scaling and shear.
--
	record LuaDAffine2
		userdata

		-- Fields
		matrix2 : LuaDMat2

		translation : LuaDVec2

		-- Pure methods
		--Transforms the given 2D point, applying shear, scale, rotation and translation.
		transform_point2: function(LuaDAffine2,LuaDVec2):(LuaDVec2)

		--Transforms the given 2D vector, applying shear, scale and rotation (but NOT
		--
		--translation).
		--
		--
		--
		--To also apply translation, use [`Self::transform_point2`] instead.
		transform_vector2: function(LuaDAffine2,LuaDVec2):(LuaDVec2)

		--Returns `true` if, and only if, all elements are finite.
		--
		--
		--
		--If any element is either `NaN`, positive or negative infinity, this will return
		--
		--`false`.
		is_finite: function(LuaDAffine2):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDAffine2):(boolean)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two 3x4 matrices contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDAffine2,(LuaDAffine2),(number)):(boolean)

		--Return the inverse of this transform.
		--
		--
		--
		--Note that if the transform is not invertible the result will be invalid.
		inverse: function(LuaDAffine2):(LuaDAffine2)

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaDVec2),(LuaDVec2),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaDVec2):(LuaDAffine2)

		--Creates an affine transform from the given rotation `angle`.
		from_angle: function(number):(LuaDAffine2)

		--Creates an affine transformation from the given 2D `translation`.
		from_translation: function(LuaDVec2):(LuaDAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
		from_mat2: function(LuaDMat2):(LuaDAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
		--
		--translation vector.
		--
		--
		--
		--Equivalent to
		--
		--`DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`
		from_mat2_translation: function((LuaDMat2),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine2::from_translation(translation) *
		--
		--DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`
		from_scale_angle_translation: function((LuaDVec2),(number),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform from the given 2D rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`
		from_angle_translation: function((number),(LuaDVec2)):(LuaDAffine2)

		--The given `DMat3` must be an affine transform,
		from_mat3: function(LuaDMat3):(LuaDAffine2)

		-- Meta methods
		metamethod __tostring: function(LuaDAffine2):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDAffine2`].
	record LuaDAffine2Class
		userdata

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaDVec2),(LuaDVec2),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaDVec2):(LuaDAffine2)

		--Creates an affine transform from the given rotation `angle`.
		from_angle: function(number):(LuaDAffine2)

		--Creates an affine transformation from the given 2D `translation`.
		from_translation: function(LuaDVec2):(LuaDAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
		from_mat2: function(LuaDMat2):(LuaDAffine2)

		--Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
		--
		--translation vector.
		--
		--
		--
		--Equivalent to
		--
		--`DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`
		from_mat2_translation: function((LuaDMat2),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine2::from_translation(translation) *
		--
		--DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`
		from_scale_angle_translation: function((LuaDVec2),(number),(LuaDVec2)):(LuaDAffine2)

		--Creates an affine transform from the given 2D rotation `angle` (in radians) and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`
		from_angle_translation: function((number),(LuaDVec2)):(LuaDAffine2)

		--The given `DMat3` must be an affine transform,
		from_mat3: function(LuaDMat3):(LuaDAffine2)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A 3D affine transform, which can represent translation, rotation, scaling and shear.
--
	record LuaDAffine3
		userdata

		-- Fields
		matrix3 : LuaDMat3

		translation : LuaDVec3

		-- Pure methods
		--Transforms the given 3D points, applying shear, scale, rotation and translation.
		transform_point3: function(LuaDAffine3,LuaDVec3):(LuaDVec3)

		--Transforms the given 3D vector, applying shear, scale and rotation (but NOT
		--
		--translation).
		--
		--
		--
		--To also apply translation, use [`Self::transform_point3`] instead.
		transform_vector3: function(LuaDAffine3,LuaDVec3):(LuaDVec3)

		--Returns `true` if, and only if, all elements are finite.
		--
		--
		--
		--If any element is either `NaN`, positive or negative infinity, this will return
		--
		--`false`.
		is_finite: function(LuaDAffine3):(boolean)

		--Returns `true` if any elements are `NaN`.
		is_nan: function(LuaDAffine3):(boolean)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two 3x4 matrices contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDAffine3,(LuaDAffine3),(number)):(boolean)

		--Return the inverse of this transform.
		--
		--
		--
		--Note that if the transform is not invertible the result will be invalid.
		inverse: function(LuaDAffine3):(LuaDAffine3)

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaDVec3),(LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaDVec3):(LuaDAffine3)

		--Creates an affine transform from the given `rotation` quaternion.
		from_quat: function(LuaDQuat):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		from_axis_angle: function((LuaDVec3),(number)):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		from_rotation_x: function(number):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		from_rotation_y: function(number):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		from_rotation_z: function(number):(LuaDAffine3)

		--Creates an affine transformation from the given 3D `translation`.
		from_translation: function(LuaDVec3):(LuaDAffine3)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and
		--
		--rotation)
		from_mat3: function(LuaDMat3):(LuaDAffine3)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
		--
		--and a translation vector.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`
		from_mat3_translation: function((LuaDMat3),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) *
		--
		--DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`
		from_scale_rotation_translation: function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform from the given 3D `rotation` and `translation`.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`
		from_rotation_translation: function((LuaDQuat),(LuaDVec3)):(LuaDAffine3)

		--The given `DMat4` must be an affine transform,
		--
		--i.e. contain no perspective transform.
		from_mat4: function(LuaDMat4):(LuaDAffine3)

		--Creates a left-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		--Creates a right-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		-- Meta methods
		metamethod __tostring: function(LuaDAffine3):(string)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDAffine3`].
	record LuaDAffine3Class
		userdata

		-- Pure functions
		--Creates an affine transform from three column vectors.
		from_cols: function((LuaDVec3),(LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform that changes scale.
		--
		--Note that if any scale is zero the transform will be non-invertible.
		from_scale: function(LuaDVec3):(LuaDAffine3)

		--Creates an affine transform from the given `rotation` quaternion.
		from_quat: function(LuaDQuat):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around a normalized
		--
		--rotation `axis` of `angle` (in radians).
		from_axis_angle: function((LuaDVec3),(number)):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the x axis of
		--
		--`angle` (in radians).
		from_rotation_x: function(number):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the y axis of
		--
		--`angle` (in radians).
		from_rotation_y: function(number):(LuaDAffine3)

		--Creates an affine transform containing a 3D rotation around the z axis of
		--
		--`angle` (in radians).
		from_rotation_z: function(number):(LuaDAffine3)

		--Creates an affine transformation from the given 3D `translation`.
		from_translation: function(LuaDVec3):(LuaDAffine3)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and
		--
		--rotation)
		from_mat3: function(LuaDMat3):(LuaDAffine3)

		--Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
		--
		--and a translation vector.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`
		from_mat3_translation: function((LuaDMat3),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform from the given 3D `scale`, `rotation` and
		--
		--`translation`.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) *
		--
		--DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`
		from_scale_rotation_translation: function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDAffine3)

		--Creates an affine transform from the given 3D `rotation` and `translation`.
		--
		--
		--
		--Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`
		from_rotation_translation: function((LuaDQuat),(LuaDVec3)):(LuaDAffine3)

		--The given `DMat4` must be an affine transform,
		--
		--i.e. contain no perspective transform.
		from_mat4: function(LuaDMat4):(LuaDAffine3)

		--Creates a left-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_lh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		--Creates a right-handed view transform using a camera position, an up direction, and
		--
		--a focal point.
		--
		--
		--
		--For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `up` is not normalized when `glam_assert` is enabled.
		look_at_rh: function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A quaternion representing an orientation.
--
--
--
--This quaternion is intended to be of unit length but may denormalize due to
--
--floating point "error creep" which can occur when successive quaternion
--
--operations are applied.
--
--
--
--This type is 16 byte aligned.
--
	record LuaQuat
		userdata

		-- Pure methods
		--Returns the rotation axis scaled by the rotation in radians.
		to_scaled_axis: function(LuaQuat):(LuaVec3)

		--Returns the vector part of the quaternion.
		xyz: function(LuaQuat):(LuaVec3)

		--Returns the quaternion conjugate of `self`. For a unit quaternion the
		--
		--conjugate is also the inverse.
		conjugate: function(LuaQuat):(LuaQuat)

		--Returns the inverse of a normalized quaternion.
		--
		--
		--
		--Typically quaternion inverse returns the conjugate of a normalized quaternion.
		--
		--Because `self` is assumed to already be unit length this method *does not* normalize
		--
		--before returning the conjugate.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		inverse: function(LuaQuat):(LuaQuat)

		--Computes the dot product of `self` and `rhs`. The dot product is
		--
		--equal to the cosine of the angle between two quaternion rotations.
		dot: function(LuaQuat,LuaQuat):(number)

		--Computes the length of `self`.
		length: function(LuaQuat):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is generally faster than `length()` as it avoids a square
		--
		--root operation.
		length_squared: function(LuaQuat):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaQuat):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaQuat):(LuaQuat)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaQuat):(boolean)

		is_nan: function(LuaQuat):(boolean)

		--Returns whether `self` of length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaQuat):(boolean)

		is_near_identity: function(LuaQuat):(boolean)

		--Returns the angle (in radians) for the minimal rotation
		--
		--for transforming this quaternion into another.
		--
		--
		--
		--Both quaternions must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
		angle_between: function(LuaQuat,LuaQuat):(number)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two quaternions contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaQuat,(LuaQuat),(number)):(boolean)

		--Performs a linear interpolation between `self` and `rhs` based on
		--
		--the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s`
		--
		--is `1.0`, the result will be equal to `rhs`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
		lerp: function(LuaQuat,(LuaQuat),(number)):(LuaQuat)

		--Performs a spherical linear interpolation between `self` and `end`
		--
		--based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s`
		--
		--is `1.0`, the result will be equal to `end`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
		slerp: function(LuaQuat,(LuaQuat),(number)):(LuaQuat)

		--Multiplies a quaternion and a 3D vector, returning the rotated vector.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		mul_vec3: function(LuaQuat,LuaVec3):(LuaVec3)

		--Multiplies two quaternions. If they each represent a rotation, the result will
		--
		--represent the combined rotation.
		--
		--
		--
		--Note that due to floating point rounding the result may not be perfectly normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
		mul_quat: function(LuaQuat,LuaQuat):(LuaQuat)

		--Multiplies a quaternion and a 3D vector, returning the rotated vector.
		mul_vec3a: function(LuaQuat,LuaVec3A):(LuaVec3A)

		as_f64: function(LuaQuat):(LuaDQuat)

		-- Pure functions
		--Creates a new rotation quaternion.
		--
		--
		--
		--This should generally not be called manually unless you know what you are doing.
		--
		--Use one of the other constructors instead such as `identity` or `from_axis_angle`.
		--
		--
		--
		--`from_xyzw` is mostly used by unit tests and `serde` deserialization.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_xyzw: function((number),(number),(number),(number)):(LuaQuat)

		--Creates a new rotation quaternion from a 4D vector.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_vec4: function(LuaVec4):(LuaQuat)

		--Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
		--
		--The axis must be normalized (unit-length).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaQuat)

		--Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
		--
		--
		--
		--`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
		from_scaled_axis: function(LuaVec3):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaQuat)

		--Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix.
		from_mat3: function(LuaMat3):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
		from_mat4: function(LuaMat4):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
		--
		--plane spanned by the two vectors.  Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc: function((LuaVec3),(LuaVec3)):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
		--
		--that the resulting quaternion will rotate `from` so that it is colinear with `to`.
		--
		--
		--
		--The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
		--
		--degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_colinear: function((LuaVec3),(LuaVec3)):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
		--
		--around the z axis. Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc_2d(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_2d: function((LuaVec2),(LuaVec2)):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
		from_affine3: function(LuaAffine3A):(LuaQuat)

		-- Meta methods
		metamethod __tostring: function(LuaQuat):(string)

		metamethod __unm: function(LuaQuat):(LuaQuat)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaQuat`].
	record LuaQuatClass
		userdata

		-- Pure functions
		--Creates a new rotation quaternion.
		--
		--
		--
		--This should generally not be called manually unless you know what you are doing.
		--
		--Use one of the other constructors instead such as `identity` or `from_axis_angle`.
		--
		--
		--
		--`from_xyzw` is mostly used by unit tests and `serde` deserialization.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_xyzw: function((number),(number),(number),(number)):(LuaQuat)

		--Creates a new rotation quaternion from a 4D vector.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_vec4: function(LuaVec4):(LuaQuat)

		--Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
		--
		--The axis must be normalized (unit-length).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaVec3),(number)):(LuaQuat)

		--Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
		--
		--
		--
		--`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
		from_scaled_axis: function(LuaVec3):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaQuat)

		--Creates a quaternion from the `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaQuat)

		--Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix.
		from_mat3: function(LuaMat3):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
		from_mat4: function(LuaMat4):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
		--
		--plane spanned by the two vectors.  Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc: function((LuaVec3),(LuaVec3)):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
		--
		--that the resulting quaternion will rotate `from` so that it is colinear with `to`.
		--
		--
		--
		--The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
		--
		--degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_colinear: function((LuaVec3),(LuaVec3)):(LuaQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
		--
		--around the z axis. Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc_2d(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_2d: function((LuaVec2),(LuaVec2)):(LuaQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
		from_affine3: function(LuaAffine3A):(LuaQuat)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--A quaternion representing an orientation.
--
--
--
--This quaternion is intended to be of unit length but may denormalize due to
--
--floating point "error creep" which can occur when successive quaternion
--
--operations are applied.
--
	record LuaDQuat
		userdata

		-- Fields
		x : number

		y : number

		z : number

		w : number

		-- Pure methods
		--Returns the rotation axis scaled by the rotation in radians.
		to_scaled_axis: function(LuaDQuat):(LuaDVec3)

		--Returns the vector part of the quaternion.
		xyz: function(LuaDQuat):(LuaDVec3)

		--Returns the quaternion conjugate of `self`. For a unit quaternion the
		--
		--conjugate is also the inverse.
		conjugate: function(LuaDQuat):(LuaDQuat)

		--Returns the inverse of a normalized quaternion.
		--
		--
		--
		--Typically quaternion inverse returns the conjugate of a normalized quaternion.
		--
		--Because `self` is assumed to already be unit length this method *does not* normalize
		--
		--before returning the conjugate.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		inverse: function(LuaDQuat):(LuaDQuat)

		--Computes the dot product of `self` and `rhs`. The dot product is
		--
		--equal to the cosine of the angle between two quaternion rotations.
		dot: function(LuaDQuat,LuaDQuat):(number)

		--Computes the length of `self`.
		length: function(LuaDQuat):(number)

		--Computes the squared length of `self`.
		--
		--
		--
		--This is generally faster than `length()` as it avoids a square
		--
		--root operation.
		length_squared: function(LuaDQuat):(number)

		--Computes `1.0 / length()`.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		length_recip: function(LuaDQuat):(number)

		--Returns `self` normalized to length 1.0.
		--
		--
		--
		--For valid results, `self` must _not_ be of length zero.
		--
		--
		--
		--Panics
		--
		--
		--
		--Will panic if `self` is zero length when `glam_assert` is enabled.
		normalize: function(LuaDQuat):(LuaDQuat)

		--Returns `true` if, and only if, all elements are finite.
		--
		--If any element is either `NaN`, positive or negative infinity, this will return `false`.
		is_finite: function(LuaDQuat):(boolean)

		is_nan: function(LuaDQuat):(boolean)

		--Returns whether `self` of length `1.0` or not.
		--
		--
		--
		--Uses a precision threshold of `1e-6`.
		is_normalized: function(LuaDQuat):(boolean)

		is_near_identity: function(LuaDQuat):(boolean)

		--Returns the angle (in radians) for the minimal rotation
		--
		--for transforming this quaternion into another.
		--
		--
		--
		--Both quaternions must be normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
		angle_between: function(LuaDQuat,LuaDQuat):(number)

		--Returns true if the absolute difference of all elements between `self` and `rhs`
		--
		--is less than or equal to `max_abs_diff`.
		--
		--
		--
		--This can be used to compare if two quaternions contain similar elements. It works
		--
		--best when comparing with a known value. The `max_abs_diff` that should be used used
		--
		--depends on the values being compared against.
		--
		--
		--
		--For more see
		--
		--[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
		abs_diff_eq: function(LuaDQuat,(LuaDQuat),(number)):(boolean)

		--Performs a linear interpolation between `self` and `rhs` based on
		--
		--the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s`
		--
		--is `1.0`, the result will be equal to `rhs`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
		lerp: function(LuaDQuat,(LuaDQuat),(number)):(LuaDQuat)

		--Performs a spherical linear interpolation between `self` and `end`
		--
		--based on the value `s`.
		--
		--
		--
		--When `s` is `0.0`, the result will be equal to `self`.  When `s`
		--
		--is `1.0`, the result will be equal to `end`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
		slerp: function(LuaDQuat,(LuaDQuat),(number)):(LuaDQuat)

		--Multiplies a quaternion and a 3D vector, returning the rotated vector.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` is not normalized when `glam_assert` is enabled.
		mul_vec3: function(LuaDQuat,LuaDVec3):(LuaDVec3)

		--Multiplies two quaternions. If they each represent a rotation, the result will
		--
		--represent the combined rotation.
		--
		--
		--
		--Note that due to floating point rounding the result may not be perfectly normalized.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
		mul_quat: function(LuaDQuat,LuaDQuat):(LuaDQuat)

		as_f32: function(LuaDQuat):(LuaQuat)

		-- Pure functions
		--Creates a new rotation quaternion.
		--
		--
		--
		--This should generally not be called manually unless you know what you are doing.
		--
		--Use one of the other constructors instead such as `identity` or `from_axis_angle`.
		--
		--
		--
		--`from_xyzw` is mostly used by unit tests and `serde` deserialization.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_xyzw: function((number),(number),(number),(number)):(LuaDQuat)

		--Creates a new rotation quaternion from a 4D vector.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_vec4: function(LuaDVec4):(LuaDQuat)

		--Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
		--
		--The axis must be normalized (unit-length).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDQuat)

		--Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
		--
		--
		--
		--`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
		from_scaled_axis: function(LuaDVec3):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaDQuat)

		--Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix.
		from_mat3: function(LuaDMat3):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
		from_mat4: function(LuaDMat4):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
		--
		--plane spanned by the two vectors.  Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc: function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
		--
		--that the resulting quaternion will rotate `from` so that it is colinear with `to`.
		--
		--
		--
		--The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
		--
		--degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_colinear: function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
		--
		--around the z axis. Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc_2d(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_2d: function((LuaDVec2),(LuaDVec2)):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
		from_affine3: function(LuaDAffine3):(LuaDQuat)

		-- Meta methods
		metamethod __tostring: function(LuaDQuat):(string)

		metamethod __unm: function(LuaDQuat):(LuaDQuat)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Collection of static methods for [`LuaDQuat`].
	record LuaDQuatClass
		userdata

		-- Pure functions
		--Creates a new rotation quaternion.
		--
		--
		--
		--This should generally not be called manually unless you know what you are doing.
		--
		--Use one of the other constructors instead such as `identity` or `from_axis_angle`.
		--
		--
		--
		--`from_xyzw` is mostly used by unit tests and `serde` deserialization.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_xyzw: function((number),(number),(number),(number)):(LuaDQuat)

		--Creates a new rotation quaternion from a 4D vector.
		--
		--
		--
		--# Preconditions
		--
		--
		--
		--This function does not check if the input is normalized, it is up to the user to
		--
		--provide normalized input or to normalized the resulting quaternion.
		from_vec4: function(LuaDVec4):(LuaDQuat)

		--Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
		--
		--The axis must be normalized (unit-length).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `axis` is not normalized when `glam_assert` is enabled.
		from_axis_angle: function((LuaDVec3),(number)):(LuaDQuat)

		--Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
		--
		--
		--
		--`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
		from_scaled_axis: function(LuaDVec3):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the x axis.
		from_rotation_x: function(number):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the y axis.
		from_rotation_y: function(number):(LuaDQuat)

		--Creates a quaternion from the `angle` (in radians) around the z axis.
		from_rotation_z: function(number):(LuaDQuat)

		--Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
		from_euler: function((LuaEulerRot),(number),(number),(number)):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix.
		from_mat3: function(LuaDMat3):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
		from_mat4: function(LuaDMat4):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
		--
		--plane spanned by the two vectors.  Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc: function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
		--
		--that the resulting quaternion will rotate `from` so that it is colinear with `to`.
		--
		--
		--
		--The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
		--
		--degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_colinear: function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

		--Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
		--
		--around the z axis. Will rotate at most 180 degrees.
		--
		--
		--
		--The input vectors must be normalized (unit-length).
		--
		--
		--
		--`from_rotation_arc_2d(from, to) * from ≈ to`.
		--
		--
		--
		--For near-singular cases (from≈to and from≈-to) the current implementation
		--
		--is only accurate to about 0.001 (for `f32`).
		--
		--
		--
		--# Panics
		--
		--
		--
		--Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
		from_rotation_arc_2d: function((LuaDVec2),(LuaDVec2)):(LuaDQuat)

		--Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
		from_affine3: function(LuaDAffine3):(LuaDQuat)

		-- Meta functions
		metamethod __add: function((any),(any)):(any)

		metamethod __sub: function((any),(any)):(any)

		metamethod __div: function((any),(any)):(any)

		metamethod __mul: function((any),(any)):(any)


	end
--Euler rotation sequences.
--
--
--
--The angles are applied starting from the right.
--
--E.g. XYZ will first apply the z-axis rotation.
--
--
--
--YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
--
--
--
--The two-axis rotations (e.g. ZYZ) are not fully tested and have to be treated with caution.
--
	record LuaEulerRot
		userdata

		-- Meta methods
		metamethod __tostring: function(LuaEulerRot):(string)


	end
--This type represents a generic reflected value.
--
--If you know the reflected value converts to a LuaType (via LuaProxyable), use the `as` operator to convert to said type.
--
	record ReflectedValue
		userdata

		-- Meta methods
		metamethod __tostring: function(ReflectedValue):(string)

		-- Mutating MetaMethods
		metamethod __index: function(ReflectedValue,any):(ReflectedValue)

		metamethod __newindex: function(ReflectedValue,(any),(any)):()


	end
--Represents the bevy world all scripts live in.
--
--Provides ways to interact with and modify the world.
--
	record LuaWorld
		userdata

		-- Pure methods
		--Retrieves children entities of the parent entity if it has any.
		get_children: function(LuaWorld,LuaEntity):({LuaEntity})

		--Retrieves the parent entity of the given entity if it has any.
		get_parent: function(LuaWorld,LuaEntity):(LuaEntity)

		--Attaches children entities to the given parent entity.
		push_children: function(LuaWorld,(LuaEntity),({LuaEntity})):()

		--Attaches child entity to the given parent entity.
		push_child: function(LuaWorld,(LuaEntity),(LuaEntity)):()

		--Removes children entities from the given parent entity.
		remove_children: function(LuaWorld,(LuaEntity),({LuaEntity})):()

		--Removes child entity from the given parent entity.
		remove_child: function(LuaWorld,(LuaEntity),(LuaEntity)):()

		--Inserts children entities to the given parent entity at the given index.
		--
		--Despawns the given entity and the entity's children recursively
		insert_children: function(LuaWorld,(LuaEntity),(integer),({LuaEntity})):()

		--Inserts child entity to the given parent entity at the given index.
		insert_child: function(LuaWorld,(LuaEntity),(integer),(LuaEntity)):()

		--Despawns the given entity's children recursively
		despawn_children_recursive: function(LuaWorld,LuaEntity):()

		--Despawns the given entity and the entity's children recursively
		despawn_recursive: function(LuaWorld,LuaEntity):()

		--Inserts children entities to the given parent entity at the given index.
		--
		--Despawns the given entity and the entity's children recursively
		insert_children: function(LuaWorld,LuaEntity):()

		--Retrieves type information given either a short (`MyType`) or fully qualified rust type name (`MyModule::MyType`).
		--
		--Returns `nil` if no such type exists or if one wasn't registered on the rust side.
		--
		--
		--
		--
		--This is used extensively in [`LuaWorld`]
		get_type_by_name: function(LuaWorld,string):(LuaTypeRegistration)

		--Inserts a component of the given type to the given entity by instantiating a default version of it.
		--
		--The component can then be modified using field access.
		add_default_component: function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(ReflectedValue)

		--Retrieves a component of the given type from the given entity.
		--
		--If such a component does not exist returns `nil`.
		get_component: function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(ReflectedValue)

		--Returns `true` if the given entity contains a component of the given type.
		has_component: function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(boolean)

		--Removes the given component from the given entity, does nothing if it doesn't exist on the entity.
		remove_component: function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):()

		--Retrieves a resource of the given type from the world.
		--
		--If such a resource does not exist returns `nil`.
		get_resource: function(LuaWorld,LuaTypeRegistration):(ReflectedValue)

		--Removes the given resource from the world, if one doesn't exist it does nothing.
		remove_resource: function(LuaWorld,LuaTypeRegistration):()

		--Returns `true` if the world contains a resource of the given type.
		has_resource: function(LuaWorld,LuaTypeRegistration):(boolean)

		--Spawns a new entity and returns its Entity ID
		spawn: function(LuaWorld):(LuaEntity)

		--Despawns the given entity if it exists, returns true if deletion was successfull
		despawn: function(LuaWorld,LuaEntity):(boolean)


	end
--Collection of static methods for [`LuaWorld`].
	record LuaWorldClass
		userdata


	end
	record LuaScriptData
		userdata

		-- Fields
		--The unique ID of this script
		sid : integer


	end
--Collection of static methods for [`LuaScriptData`].
	record LuaScriptDataClass
		userdata


	end
--An object representing an existing and registered rust type.
--
--Can be obtained via [`LuaWorld::get_type_by_name`].
--
	record LuaTypeRegistration
		userdata

		-- Fields
		--The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type
		short_name : string

		--The full name of the type
		type_name : string


	end
--A reference to the Vec<T> Rust type.
--
--The indexing begins at 1.
--
	record LuaVec<T>
		userdata

		-- Mutating methods
		push: function(LuaVec<T>,any):()

		pop: function(LuaVec<T>):(any)

		clear: function(LuaVec<T>):()

		insert: function(LuaVec<T>,(integer),(any)):()

		remove: function(LuaVec<T>,integer):(any)

		-- Meta methods
		metamethod __tostring: function(LuaVec<T>):(string)

		metamethod __index: function(LuaVec<T>,integer):(ReflectedValue)

		metamethod __pairs: function(LuaVec<T>):(function():((any),(any)))

		metamethod __len: function(LuaVec<T>):(integer)

		-- Mutating MetaMethods
		metamethod __newindex: function(LuaVec<T>,(integer),(any)):()


	end
	record LuaLifeState
		userdata

		-- Fields
		cells : LuaVec<boolean>

		-- Meta methods
		metamethod __tostring: function(LuaLifeState):(string)


	end
end
global Name: types.LuaNameClass
global Children: types.LuaChildrenClass
global Text: types.LuaTextClass
global TextSection: types.LuaTextSectionClass
global Stopwatch: types.LuaStopwatchClass
global Timer: types.LuaTimerClass
global Entity: types.LuaEntityClass
global Transform: types.LuaTransformClass
global GlobalTransform: types.LuaGlobalTransformClass
global TextureAtlasSprite: types.LuaTextureAtlasSpriteClass
global RenderLayers: types.LuaRenderLayersClass
global Visibility: types.LuaVisibilityClass
global ComputedVisibility: types.LuaComputedVisibilityClass
global Color: types.LuaColorClass
global Aabb: types.LuaAabbClass
global Frustum: types.LuaFrustumClass
global CameraRenderGraph: types.LuaCameraRenderGraphClass
global HandleId: types.LuaHandleIdClass
global Vec2: types.LuaVec2Class
global Vec3: types.LuaVec3Class
global Vec3A: types.LuaVec3AClass
global Vec4: types.LuaVec4Class
global BVec2: types.LuaBVec2Class
global BVec3: types.LuaBVec3Class
global BVec4: types.LuaBVec4Class
global BVec3A: types.LuaBVec3AClass
global BVec4A: types.LuaBVec4AClass
global DVec2: types.LuaDVec2Class
global DVec3: types.LuaDVec3Class
global DVec4: types.LuaDVec4Class
global IVec2: types.LuaIVec2Class
global IVec3: types.LuaIVec3Class
global IVec4: types.LuaIVec4Class
global UVec2: types.LuaUVec2Class
global UVec3: types.LuaUVec3Class
global UVec4: types.LuaUVec4Class
global Mat3: types.LuaMat3Class
global Mat2: types.LuaMat2Class
global Mat3A: types.LuaMat3AClass
global Mat4: types.LuaMat4Class
global DMat2: types.LuaDMat2Class
global DMat3: types.LuaDMat3Class
global DMat4: types.LuaDMat4Class
global Affine2: types.LuaAffine2Class
global Affine3A: types.LuaAffine3AClass
global DAffine2: types.LuaDAffine2Class
global DAffine3: types.LuaDAffine3Class
global Quat: types.LuaQuatClass
global DQuat: types.LuaDQuatClass
global world: types.LuaWorld
global script: types.LuaScriptData
global entity: types.LuaEntity
return types