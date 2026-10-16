---@meta
---@module "World"


---@class World
--- The ECS world containing all Components, Resources and Systems. Main point of interaction with a Bevy App.
local World = {}

---@return ReflectReference
function World.fixed_time() end

---@param path string
--- The asset path, relative to the assets folder.
---@param asset_type ScriptTypeRegistration
--- The type of the asset, i.e. `bevy_mod_scripting_asset::script_asset::ScriptAsset`.
---@return ReflectReference
function World.load_asset(path,asset_type) end

---@param handle ReflectReference
--- The handle to the asset.
---@return boolean
function World.is_loaded(handle) end

---@param entity Entity
--- The entity to despawn the descendants of.
---@return nil
function World.despawn_descendants(entity) end

---@param p1 ReflectSchedule
---@param p2 string
---@param p3 string
---@return nil
function World._assert_systems_run_in_parallel(p1,p2,p3) end

---@param namespace string
--- The script namespace to register the component in, it must start with `script.`, i.e. `script.my_game`. Scripts cannot add to other namespaces.
---@param name string
--- The name of the component type, unique within the namespace. The component can later be found as `import_namespace(namespace).name`.
---@param allow_existing? boolean | nil
--- If true, returns the registration of the already registered component with this name instead of failing. Defaults to false.
---@return ScriptComponentRegistration
function World.register_new_component(namespace,name,allow_existing) end

---@return nil
function World.exit() end

---@param entity Entity
---@param registration ScriptComponentRegistration
--- The resource to add.
---@return nil
function World.add_default_component(entity,registration) end

---@param entity Entity
--- The entity to check.
---@param registration ScriptComponentRegistration
--- The component to check for.
---@return boolean
function World.has_component(entity,registration) end

---@param handle_reference ReflectReference
--- The handle to the asset (as a reflect reference).
---@param registration ScriptTypeRegistration
--- The type registration of the asset type.
---@return ReflectReference | nil
function World.get_asset(handle_reference,registration) end

---@return Entity
function World.spawn() end

---@param p1 number
---@return nil
function World._sleep(p1) end

---@param state_type ScriptTypeRegistration
---@return ReflectReference | nil
function World.get_state(state_type) end

---@param registration ScriptBundleRegistration
--- The bundle registration of the bundle to insert.
---@param value ReflectReference
--- The value of the bundle to insert.
---@return Entity
function World.spawn_bundle(registration,value) end

---@param entity Entity
--- The entity to despawn.
---@return nil
function World.despawn(entity) end

---@param entity Entity
--- The entity to insert the component into.
---@param registration ScriptComponentRegistration
--- The component registration of the component to insert.
---@param value ReflectReference
--- The value of the component to insert. Can be constructed using `construct`
---@return nil
function World.insert_component(entity,registration,value) end

---@param entity Entity
--- The parent entity to receive children
---@param index integer
--- The index to insert the children at
---@param children Entity[]
--- The children entities to insert
---@return nil
function World.insert_children(entity,index,children) end

---@return ReflectReference
function World.time() end

---@param namespace string
--- The script namespace to register the event in, it must start with `script.`, i.e. `script.my_game`.
---@param name string
--- The name of the event, unique within the namespace.
---@param allow_existing? boolean | nil
--- If true, returns the already registered event with this name instead of failing. Defaults to false.
---@return ScriptEventRegistration
function World.register_new_event(namespace,name,allow_existing) end

---@param entity Entity
--- The entity to despawn recursively.
---@return nil
function World.despawn_recursive(entity) end

---@param handle ReflectReference
--- The handle to the asset.
---@return string | nil
function World.asset_path(handle) end

---@param schedule ReflectSchedule
--- The schedule to add the system to.
---@param builder ScriptSystemBuilder
--- The system builder specifying the system and its dependencies.
---@return ReflectSystem
function World.add_system(schedule,builder) end

---@param entity Entity
--- The entity to insert the bundle into.
---@param registration ScriptBundleRegistration
--- The bundle registration of the bundle to insert.
---@param value ReflectReference
--- The value of the bundle to insert. Can be constructed like any other value, i.e. using `construct` or the type's `new` function.
---@return nil
function World.insert_bundle(entity,registration,value) end

---@param p1 string
---@return ReflectReference | nil
function World._get_entity_with_test_component(p1) end

---@param e Entity
---@return boolean
function World.has_entity(e) end

---@param handle_reference ReflectReference
---@return boolean
function World.has_asset(handle_reference) end

---@param registration ScriptResourceRegistration
--- The resource to remove.
---@return nil
function World.remove_resource(registration) end

---@param entity Entity
--- The entity to retrieve the children of.
---@return Entity[]
function World.get_children(entity) end

---@param registration ScriptTypeRegistration
--- The type of the message.
---@param message ReflectReference
--- The message value, e.g. built with `construct`.
---@return nil
function World.write_message(registration,message) end

---@param p1 ReflectSchedule
---@param p2 string
---@param p3 string
---@return nil
function World._assert_system_runs_before(p1,p2,p3) end

---@param event ScriptTypeRegistration | ScriptEventRegistration
--- The type of the event to observe, or an event registered with `register_new_event`.
---@param callback string
--- The label of the callback to run, receives the event.
---@param components? ScriptComponentRegistration[] | nil
--- The components the observer is interested in, for component lifecycle events. Ignored for script events.
---@param entities? Entity[] | nil
--- If given, the observer only runs for these entities. Ignored for script events.
---@return Entity
function World.add_observer(event,callback,components,entities) end

---@return ReflectReference
function World._get_mock_type() end

---@return ReflectReference
function World._get_mock_resource_type() end

---@param entity Entity
--- The parent entity to receive children
---@param children Entity[]
--- The children entities to push
---@return nil
function World.push_children(entity,children) end

---@param entity Entity
--- The entity to retrieve the parent of.
---@return Entity | nil
function World.get_parent(entity) end

---@param value ReflectReference
--- The asset value, of a type registered for asset reflection.
---@return ReflectReference
function World.add_asset(value) end

---@param entity Entity
--- The entity to retrieve the component from.
---@param registration ScriptComponentRegistration
--- The component to retrieve.
---@return ReflectReference | nil
function World.get_component(entity,registration) end

---@param name string
---@return Entity | nil
function World.find_entity_by_name(name) end

---@param registration ScriptResourceRegistration
--- The registration of the resource to retrieve.
---@return ReflectReference | nil
function World.get_resource(registration) end

---@param type_name string
--- The full path of the type to retrieve, i.e. `bevy_transform::components::transform::Transform` or `script.my_game.Enemy`.
---@return ScriptTypeRegistration | ScriptComponentRegistration | ScriptResourceRegistration | ScriptBundleRegistration | nil
function World.get_type_by_name(type_name) end

---@param p1 function
---@param p2 string
---@return nil
function World._assert_throws(p1,p2) end

---@return ReflectReference
function World._get_mock_component_type() end

---@param entity Entity
--- The entity to remove the component from.
---@param registration ScriptComponentRegistration
--- The component to remove.
---@return nil
function World.remove_component(entity,registration) end

---@return Entity[]
function World.gamepads() end

---@return ScriptQueryBuilder
function World.query() end

---@param name string
--- The name of the schedule to retrieve.
---@return ReflectSchedule | nil
function World.get_schedule_by_name(name) end

---@param event ScriptTypeRegistration | ScriptEventRegistration
--- The type of the event, or the script event.
---@param value? any | nil
--- The event value, e.g. built with `construct`, for Rust events. Any value (or nothing) for script events, which is passed to the observers.
---@return nil
function World.trigger_event(event,value) end

---@return ReflectReference
function World.virtual_time() end

---@param registration ScriptTypeRegistration | ScriptComponentRegistration | ScriptResourceRegistration | ScriptBundleRegistration
--- The enum type.
---@param variant_name string
--- The name of the variant.
---@return EnumVariantRef | nil
function World.get_enum_variant(registration,variant_name) end

---@param registration ScriptResourceRegistration
--- The registration of the resource to check for.
---@return boolean
function World.has_resource(registration) end

---@param handle ReflectReference
--- The handle to the asset.
---@return string
function World.load_state(handle) end

---@return ReflectReference
function World.real_time() end

---@param value ReflectReference
---@return nil
function World.set_next_state(value) end



---@class AnimationPlayer : ReflectReference
---  Animation controls.
--- 
---  Automatically added to any root animations of a scene when it is
---  spawned.
---@field  active_animations? table<NodeIndex, ActiveAnimation>
local AnimationPlayer = {}

---@param _self AnimationPlayer
---@param node NodeIndex
---@return nil
function AnimationPlayer.play(_self,node) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@param seconds number
---@return nil
function AnimationPlayer.seek_to(_self,node,seconds) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@return nil
function AnimationPlayer.set_repeat_forever(_self,node) end

---@param _self AnimationPlayer
---@return nil
function AnimationPlayer.resume_all(_self) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@return boolean
function AnimationPlayer.is_playing_animation(_self,node) end

---@param _self AnimationPlayer
---@return boolean
function AnimationPlayer.all_finished(_self) end

---@param _self AnimationPlayer
---@return nil
function AnimationPlayer.pause_all(_self) end

---@param _self AnimationPlayer
---@return boolean
function AnimationPlayer.all_paused(_self) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@param speed number
---@return nil
function AnimationPlayer.set_speed(_self,node,speed) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@return nil
function AnimationPlayer.stop(_self,node) end

---@param _self AnimationPlayer
---@param node NodeIndex
---@return boolean | nil
function AnimationPlayer.animation_is_finished(_self,node) end

---@param _self AnimationPlayer
---@return nil
function AnimationPlayer.stop_all(_self) end

---@param payload AnimationPlayer.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AnimationPlayer
function AnimationPlayer.new(payload) end



---@class AnimationGraph : ReflectReference
---  A graph structure that describes how animation clips are to be blended
---  together.
--- 
---  Applications frequently want to be able to play multiple animations at once
---  and to fine-tune the influence that animations have on a skinned mesh. Bevy
---  uses an *animation graph* to store this information. Animation graphs are a
---  directed acyclic graph (DAG) that describes how animations are to be
---  weighted and combined together. Every frame, Bevy evaluates the graph from
---  the root and blends the animations together in a bottom-up fashion to
---  produce the final pose.
--- 
---  There are three types of nodes: *blend nodes*, *add nodes*, and *clip
---  nodes*, all of which can have an associated weight. Blend nodes and add
---  nodes have no associated animation clip and combine the animations of their
---  children according to those children's weights. Clip nodes specify an
---  animation clip to play. When a graph is created, it starts with only a
---  single blend node, the root node.
--- 
---  For example, consider the following graph:
--- 
---  ```text
---  ┌────────────┐                                      
---  │            │                                      
---  │    Idle    ├─────────────────────┐                
---  │            │                     │                
---  └────────────┘                     │                
---                                     │                
---  ┌────────────┐                     │  ┌────────────┐
---  │            │                     │  │            │
---  │    Run     ├──┐                  ├──┤    Root    │
---  │            │  │  ┌────────────┐  │  │            │
---  └────────────┘  │  │   Blend    │  │  └────────────┘
---                  ├──┤            ├──┘                
---  ┌────────────┐  │  │    0.5     │                   
---  │            │  │  └────────────┘                   
---  │    Walk    ├──┘                                   
---  │            │                                      
---  └────────────┘                                      
---  ```
--- 
---  In this case, assuming that Idle, Run, and Walk are all playing with weight
---  1.0, the Run and Walk animations will be equally blended together, then
---  their weights will be halved and finally blended with the Idle animation.
---  Thus the weight of Run and Walk are effectively half of the weight of Idle.
--- 
---  Nodes can optionally have a *mask*, a bitfield that restricts the set of
---  animation targets that the node and its descendants affect. Each bit in the
---  mask corresponds to a *mask group*, which is a set of animation targets
---  (bones). An animation target can belong to any number of mask groups within
---  the context of an animation graph.
--- 
---  When the appropriate bit is set in a node's mask, neither the node nor its
---  descendants will animate any animation targets belonging to that mask group.
---  That is, setting a mask bit to 1 *disables* the animation targets in that
---  group. If an animation target belongs to multiple mask groups, masking any
---  one of the mask groups that it belongs to will mask that animation target.
---  (Thus an animation target will only be animated if *all* of its mask groups
---  are unmasked.)
--- 
---  A common use of masks is to allow characters to hold objects. For this, the
---  typical workflow is to assign each character's hand to a mask group. Then,
---  when the character picks up an object, the application masks out the hand
---  that the object is held in for the character's animation set, then positions
---  the hand's digits as necessary to grasp the object. The character's
---  animations will continue to play but will not affect the hand, which will
---  continue to be depicted as holding the object.
--- 
---  Animation graphs are assets and can be serialized to and loaded from [RON]
---  files. Canonically, such files have an `.animgraph.ron` extension.
--- 
---  The animation graph implements [RFC 51]. See that document for more
---  information.
--- 
---  [RON]: https://github.com/ron-rs/ron
--- 
---  [RFC 51]: https://github.com/bevyengine/rfcs/blob/main/rfcs/51-animation-composition.md
---@field  graph? any
---@field  root? NodeIndex
---@field  mask_groups? table<AnimationTargetId, integer>
local AnimationGraph = {}

---@param handle bevy_asset::handle::Handle<bevy_animation::AnimationClip>
---@return AnimationGraph
function AnimationGraph.from_clip(handle) end

---@param _self AnimationGraph
---@return NodeIndex
function AnimationGraph.root_node(_self) end

---@param _self AnimationGraph
---@param handle bevy_asset::handle::Handle<bevy_animation::AnimationClip>
---@param weight number
---@param parent NodeIndex
---@return NodeIndex
function AnimationGraph.add_clip(_self,handle,weight,parent) end

---@param payload AnimationGraph.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AnimationGraph
function AnimationGraph.new(payload) end



---@class PlaybackSettings : ReflectReference
---  Initial settings to be used when audio starts playing.
--- 
---  If you would like to control the audio while it is playing, query for the
---  [`AudioSink`](crate::AudioSink) or [`SpatialAudioSink`](crate::SpatialAudioSink)
---  components. Changes to this component will *not* be applied to already-playing audio.
---@field  mode? PlaybackMode
---@field  volume? Volume
---@field  speed? number
---@field  paused? boolean
---@field  muted? boolean
---@field  spatial? boolean
---@field  spatial_scale? SpatialScale | nil
---@field  start_position? Duration | nil
---@field  duration? Duration | nil
local PlaybackSettings = {}

---@param _self PlaybackSettings
---@param speed number
---@return PlaybackSettings
function PlaybackSettings.with_speed(_self,speed) end

---@return PlaybackSettings
function PlaybackSettings.remove() end

---@return PlaybackSettings
function PlaybackSettings.once() end

---@return PlaybackSettings
function PlaybackSettings.looping() end

---@param _self PlaybackSettings
---@param linear number
---@return PlaybackSettings
function PlaybackSettings.with_volume(_self,linear) end

---@param _self PlaybackSettings
---@param spatial boolean
---@return PlaybackSettings
function PlaybackSettings.with_spatial(_self,spatial) end

---@return PlaybackSettings
function PlaybackSettings.despawn() end

---@param payload PlaybackSettings.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return PlaybackSettings
function PlaybackSettings.new(payload) end



---@class Camera : ReflectReference
---  The defining [`Component`] for camera entities,
---  storing information about how and what to render through this camera.
--- 
---  The [`Camera`] component is added to an entity to define the properties of the viewpoint from
---  which rendering occurs. It defines the position of the view to render, the projection method
---  to transform the 3D objects into a 2D image, as well as the render target into which that image
---  is produced.
--- 
---  Note that a [`Camera`] needs a `CameraRenderGraph` to render anything.
---  This is typically provided by adding a [`Camera2d`] or [`Camera3d`] component,
---  but custom render graphs can also be defined. Inserting a [`Camera`] with no render
---  graph will emit an error at runtime.
--- 
---  [`Camera2d`]: crate::Camera2d
---  [`Camera3d`]: crate::Camera3d
---@field  viewport? Viewport | nil
---@field  order? integer
---@field  is_active? boolean
---@field  computed? ComputedCameraValues
---@field  output_mode? CameraOutputMode
---@field  msaa_writeback? MsaaWriteback
---@field  clear_color? ClearColorConfig
---@field  invert_culling? boolean
---@field  sub_camera_view? SubCameraView | nil
local Camera = {}

---@param _self Camera
---@param camera_transform GlobalTransform
---@param world_position Vec3
---@return Vec2 | nil
function Camera.world_to_viewport(_self,camera_transform,world_position) end

---@param _self Camera
---@return Rect | nil
function Camera.logical_viewport_rect(_self) end

---@param _self Camera
---@param camera_transform GlobalTransform
---@param viewport_position Vec2
---@return Ray3d | nil
function Camera.viewport_to_world(_self,camera_transform,viewport_position) end

---@param _self Camera
---@param camera_transform GlobalTransform
---@param viewport_position Vec2
---@return Vec2 | nil
function Camera.viewport_to_world_2d(_self,camera_transform,viewport_position) end

---@param _self Camera
---@return Vec2 | nil
function Camera.logical_viewport_size(_self) end

---@param _self Camera
---@return UVec2 | nil
function Camera.physical_viewport_size(_self) end

---@param payload Camera.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Camera
function Camera.new(payload) end



---@class OrthographicProjection : ReflectReference
---  Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`],
---  the size of objects remains the same regardless of their distance to the camera.
--- 
---  The volume contained in the projection is called the *view frustum*. Since the viewport is rectangular
---  and projection lines are parallel, the view frustum takes the shape of a cuboid.
--- 
---  Note that the scale of the projection and the apparent size of objects are inversely proportional.
---  As the size of the projection increases, the size of objects decreases.
--- 
---  # Examples
--- 
---  Configure the orthographic projection to one world unit per 100 window pixels:
--- 
---  ```
---  # use bevy_camera::{OrthographicProjection, Projection, ScalingMode};
---  let projection = Projection::Orthographic(OrthographicProjection {
---      scaling_mode: ScalingMode::WindowSize,
---      scale: 0.01,
---      ..OrthographicProjection::default_2d()
---  });
---  ```
---@field  near? number
---@field  far? number
---@field  viewport_origin? Vec2
---@field  scaling_mode? ScalingMode
---@field  scale? number
---@field  area? Rect
local OrthographicProjection = {}

---@return OrthographicProjection
function OrthographicProjection.default_2d() end

---@return OrthographicProjection
function OrthographicProjection.default_3d() end

---@param payload OrthographicProjection.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return OrthographicProjection
function OrthographicProjection.new(payload) end



---@class InheritedVisibility : ReflectReference
---  Whether or not an entity is visible in the hierarchy.
--- 
---  This is a computed component that users should not change manually.
---  To set the visibility of an entity, use [`Visibility`] instead.
---  For more information, see [module level documentation](self#what-is-the-difference-between-visibility-components).
--- 
---  This property is updated in [`VisibilityPropagate`] in the [`PostUpdate`] schedule.
---  Until then, it is not up to date with [`Visibility`] if it was changed
---  in the same frame.
--- 
---  If this is false, then [`ViewVisibility`] should also be false.
--- 
---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate
---@field  [1] boolean
local InheritedVisibility = {}

---@param _self InheritedVisibility
---@return boolean
function InheritedVisibility.get(_self) end

---@param payload InheritedVisibility.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InheritedVisibility
function InheritedVisibility.new(payload) end



---@class ViewVisibility : ReflectReference
---  Algorithmically computed indication of whether an entity is visible and should be extracted for
---  rendering.
--- 
---  Not to be confused with [`Visibility`] and [`InheritedVisibility`].
---  For more information, see [module level documentation](self#what-is-the-difference-between-visibility-components).
--- 
---  If you wish to add a custom visibility system that sets this value, be sure to add it to the
---  [`CheckVisibility`] set.
--- 
---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate
---  [`CheckVisibility`]: VisibilitySystems::CheckVisibility
---@field  [1] integer
local ViewVisibility = {}

---@param _self ViewVisibility
---@return boolean
function ViewVisibility.get(_self) end

---@param payload ViewVisibility.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ViewVisibility
function ViewVisibility.new(payload) end



---@class Visibility : ReflectReference
---  User indication of whether an entity is visible. Propagates down the entity hierarchy.
--- 
---  If an entity is hidden in this way, all [`Children`] (and all of their children and so on) who
---  are set to [`Inherited`](Self::Inherited) will also be hidden.
--- 
---  Users should set this component if they wish to change the visibility of an entity.
--- 
---  To read the visibility of an entity, query for its [`InheritedVisibility`] instead.
---  For more information, see [module level documentation](self#what-is-the-difference-between-visibility-components).
---@field  Inherited Visibility.Inherited
---@field  Hidden Visibility.Hidden
---@field  Visible Visibility.Visible
local Visibility = {}

---@param _self Visibility
---@return nil
function Visibility.toggle_inherited_visible(_self) end

---@param _self Visibility
---@return nil
function Visibility.toggle_visible_hidden(_self) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Visibility
function Visibility.new(payload) end



---@class Visibility.Inherited : EnumVariantRef
--- The `Inherited` variant of `Visibility`.
Visibility.Inherited = {}

---@param variant Visibility.Inherited
---@param payload NoFields
---@return Visibility
function Visibility.Inherited.new(variant,payload) end



---@class Visibility.Hidden : EnumVariantRef
--- The `Hidden` variant of `Visibility`.
Visibility.Hidden = {}

---@param variant Visibility.Hidden
---@param payload NoFields
---@return Visibility
function Visibility.Hidden.new(variant,payload) end



---@class Visibility.Visible : EnumVariantRef
--- The `Visible` variant of `Visibility`.
Visibility.Visible = {}

---@param variant Visibility.Visible
---@param payload NoFields
---@return Visibility
function Visibility.Visible.new(variant,payload) end



---@class RenderLayers : ReflectReference
---  Defines which rendering layers an entity belongs to.
--- 
---  A camera renders an entity only when their render layers intersect.
--- 
---  The [`Default`] instance of `RenderLayers` contains layer `0`, the first layer. Entities
---  without this component also belong to layer `0`.
--- 
---  An empty `RenderLayers` makes the entity invisible.
---@field  [1] integer[]
local RenderLayers = {}

---@param _self RenderLayers
---@param n integer
---@return RenderLayers
function RenderLayers.without(_self,n) end

---@param _self RenderLayers
---@param other RenderLayers
---@return boolean
function RenderLayers.intersects(_self,other) end

---@param n integer
---@return RenderLayers
function RenderLayers.layer(n) end

---@param _self RenderLayers
---@param n integer
---@return RenderLayers
function RenderLayers.with(_self,n) end

---@param _self RenderLayers
---@return integer[]
function RenderLayers.__pairs(_self) end

---@param _self RenderLayers
---@return integer[]
function RenderLayers.iter(_self) end

---@return RenderLayers
function RenderLayers.none() end

---@param payload RenderLayers.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RenderLayers
function RenderLayers.new(payload) end



---@class Color : ReflectReference
---  An enumerated type that can represent any of the color types in this crate.
--- 
---  This is useful when you need to store a color in a data structure that can't be generic over
---  the color type.
---  <div>
---  </div>
--- 
---  # Operations
--- 
---  [`Color`] supports all the standard color operations, such as [mixing](Mix),
---  [luminance](Luminance) and [hue](Hue) adjustment,
---  and [diffing](EuclideanDistance). These operations delegate to the concrete color space contained
---  by [`Color`], but will convert to [`Oklch`](Oklcha) for operations which aren't supported in the
---  current space. After performing the operation, if a conversion was required, the result will be
---  converted back into the original color space.
--- 
---  ```rust
---  # use bevy_color::{Hue, Color};
---  let red_hsv = Color::hsv(0., 1., 1.);
---  let red_srgb = Color::srgb(1., 0., 0.);
--- 
---  // HSV has a definition of hue, so it will be returned.
---  red_hsv.hue();
--- 
---  // SRGB doesn't have a native definition for hue.
---  // Converts to Oklch and returns that result.
---  red_srgb.hue();
---  ```
--- 
---  [`Oklch`](Oklcha) has been chosen as the intermediary space in cases where conversion is required
---  due to its perceptual uniformity and broad support for Bevy's color operations.
---  To avoid the cost of repeated conversion, and ensure consistent results where that is desired,
---  first convert this [`Color`] into your desired color space.
---@field  Srgba Color.Srgba
---@field  LinearRgba Color.LinearRgba
---@field  Hsla Color.Hsla
---@field  Hsva Color.Hsva
---@field  Hwba Color.Hwba
---@field  Laba Color.Laba
---@field  Lcha Color.Lcha
---@field  Oklaba Color.Oklaba
---@field  Oklcha Color.Oklcha
---@field  Xyza Color.Xyza
---@field  Okhsla Color.Okhsla
---@field  Okhsva Color.Okhsva
---@field  Okhwba Color.Okhwba
local Color = {}

---@param red number
---@param green number
---@param blue number
---@return Color
function Color.srgb(red,green,blue) end

---@param hex string
---@return Color
function Color.hex(hex) end

---@param hue number
---@param saturation number
---@param lightness number
---@return Color
function Color.hsl(hue,saturation,lightness) end

---@param hue number
---@param saturation number
---@param value number
---@return Color
function Color.hsv(hue,saturation,value) end

---@param red number
---@param green number
---@param blue number
---@param alpha number
---@return Color
function Color.linear_rgba(red,green,blue,alpha) end

---@param hue number
---@param saturation number
---@param value number
---@param alpha number
---@return Color
function Color.hsva(hue,saturation,value,alpha) end

---@param red number
---@param green number
---@param blue number
---@param alpha number
---@return Color
function Color.srgba(red,green,blue,alpha) end

---@param red number
---@param green number
---@param blue number
---@return Color
function Color.linear_rgb(red,green,blue) end

---@return Color
function Color.none() end

---@param _self Color
---@return number
function Color.alpha(_self) end

---@param _self Color
---@param other Color
---@return boolean
function Color.__eq(_self,other) end

---@param _self Color
---@param other Color
---@return boolean
function Color.eq(_self,other) end

---@param _self Color
---@param other Color
---@param t number
---@return Color
function Color.mix(_self,other,t) end

---@param _self Color
---@return LinearRgba
function Color.to_linear(_self) end

---@param hue number
---@param saturation number
---@param lightness number
---@param alpha number
---@return Color
function Color.hsla(hue,saturation,lightness,alpha) end

---@param _self Color
---@return string
function Color.to_hex(_self) end

---@param _self Color
---@return Srgba
function Color.to_srgba(_self) end

---@param red integer
---@param green integer
---@param blue integer
---@param alpha integer
---@return Color
function Color.srgba_u8(red,green,blue,alpha) end

---@param _self Color
---@param amount number
---@return Color
function Color.lighter(_self,amount) end

---@param _self Color
---@return boolean
function Color.is_fully_transparent(_self) end

---@param _self Color
---@param amount number
---@return Color
function Color.darker(_self,amount) end

---@param red integer
---@param green integer
---@param blue integer
---@return Color
function Color.srgb_u8(red,green,blue) end

---@param _self Color
---@return number
function Color.luminance(_self) end

---@return Color
function Color.black() end

---@param _self Color
---@param alpha number
---@return nil
function Color.set_alpha(_self,alpha) end

---@param _self Color
---@param alpha number
---@return Color
function Color.with_alpha(_self,alpha) end

---@param name string
---@return Color | nil
function Color.named(name) end

---@return Color
function Color.white() end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Color
function Color.new(payload) end



---@class Color.Srgba : EnumVariantRef
--- The `Srgba` variant of `Color`.
Color.Srgba = {}

---@param variant Color.Srgba
---@param payload Color.Srgba.Fields
---@return Color
function Color.Srgba.new(variant,payload) end



---@class Color.LinearRgba : EnumVariantRef
--- The `LinearRgba` variant of `Color`.
Color.LinearRgba = {}

---@param variant Color.LinearRgba
---@param payload Color.LinearRgba.Fields
---@return Color
function Color.LinearRgba.new(variant,payload) end



---@class Color.Hsla : EnumVariantRef
--- The `Hsla` variant of `Color`.
Color.Hsla = {}

---@param variant Color.Hsla
---@param payload Color.Hsla.Fields
---@return Color
function Color.Hsla.new(variant,payload) end



---@class Color.Hsva : EnumVariantRef
--- The `Hsva` variant of `Color`.
Color.Hsva = {}

---@param variant Color.Hsva
---@param payload Color.Hsva.Fields
---@return Color
function Color.Hsva.new(variant,payload) end



---@class Color.Hwba : EnumVariantRef
--- The `Hwba` variant of `Color`.
Color.Hwba = {}

---@param variant Color.Hwba
---@param payload Color.Hwba.Fields
---@return Color
function Color.Hwba.new(variant,payload) end



---@class Color.Laba : EnumVariantRef
--- The `Laba` variant of `Color`.
Color.Laba = {}

---@param variant Color.Laba
---@param payload Color.Laba.Fields
---@return Color
function Color.Laba.new(variant,payload) end



---@class Color.Lcha : EnumVariantRef
--- The `Lcha` variant of `Color`.
Color.Lcha = {}

---@param variant Color.Lcha
---@param payload Color.Lcha.Fields
---@return Color
function Color.Lcha.new(variant,payload) end



---@class Color.Oklaba : EnumVariantRef
--- The `Oklaba` variant of `Color`.
Color.Oklaba = {}

---@param variant Color.Oklaba
---@param payload Color.Oklaba.Fields
---@return Color
function Color.Oklaba.new(variant,payload) end



---@class Color.Oklcha : EnumVariantRef
--- The `Oklcha` variant of `Color`.
Color.Oklcha = {}

---@param variant Color.Oklcha
---@param payload Color.Oklcha.Fields
---@return Color
function Color.Oklcha.new(variant,payload) end



---@class Color.Xyza : EnumVariantRef
--- The `Xyza` variant of `Color`.
Color.Xyza = {}

---@param variant Color.Xyza
---@param payload Color.Xyza.Fields
---@return Color
function Color.Xyza.new(variant,payload) end



---@class Color.Okhsla : EnumVariantRef
--- The `Okhsla` variant of `Color`.
Color.Okhsla = {}

---@param variant Color.Okhsla
---@param payload Color.Okhsla.Fields
---@return Color
function Color.Okhsla.new(variant,payload) end



---@class Color.Okhsva : EnumVariantRef
--- The `Okhsva` variant of `Color`.
Color.Okhsva = {}

---@param variant Color.Okhsva
---@param payload Color.Okhsva.Fields
---@return Color
function Color.Okhsva.new(variant,payload) end



---@class Color.Okhwba : EnumVariantRef
--- The `Okhwba` variant of `Color`.
Color.Okhwba = {}

---@param variant Color.Okhwba
---@param payload Color.Okhwba.Fields
---@return Color
function Color.Okhwba.new(variant,payload) end



---@class Hsla : ReflectReference
---  Color in Hue-Saturation-Lightness (HSL) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).
---  <div>
---  </div>
---@field  hue? number
---@field  saturation? number
---@field  lightness? number
---@field  alpha? number
local Hsla = {}

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Hsla
function Hsla.new(a,b,c,alpha) end

---@param _self Hsla
---@return Color
function Hsla.to_color(_self) end



---@class Hsva : ReflectReference
---  Color in Hue-Saturation-Value (HSV) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).
---  <div>
---  </div>
---@field  hue? number
---@field  saturation? number
---@field  value? number
---@field  alpha? number
local Hsva = {}

---@param _self Hsva
---@return Color
function Hsva.to_color(_self) end

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Hsva
function Hsva.new(a,b,c,alpha) end



---@class Hwba : ReflectReference
---  Color in Hue-Whiteness-Blackness (HWB) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).
---  <div>
---  </div>
---@field  hue? number
---@field  whiteness? number
---@field  blackness? number
---@field  alpha? number
local Hwba = {}

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Hwba
function Hwba.new(a,b,c,alpha) end

---@param _self Hwba
---@return Color
function Hwba.to_color(_self) end



---@class Laba : ReflectReference
---  Color in LAB color space, with alpha
---  <div>
---  </div>
---@field  lightness? number
---@field  a? number
---@field  b? number
---@field  alpha? number
local Laba = {}

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Laba
function Laba.new(a,b,c,alpha) end

---@param _self Laba
---@return Color
function Laba.to_color(_self) end



---@class Lcha : ReflectReference
---  Color in LCH color space, with alpha
---  <div>
---  </div>
---@field  lightness? number
---@field  chroma? number
---@field  hue? number
---@field  alpha? number
local Lcha = {}

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Lcha
function Lcha.new(a,b,c,alpha) end

---@param _self Lcha
---@return Color
function Lcha.to_color(_self) end



---@class LinearRgba : ReflectReference
---  Linear RGB color with alpha.
--- 
---  SDR colors are in `[0.0, 1.0]`. Values above `1.0` are HDR intensities.
---  <div>
---  </div>
---@field  red? number
---@field  green? number
---@field  blue? number
---@field  alpha? number
---@operator mul(number): LinearRgba
local LinearRgba = {}

---@param _self LinearRgba
---@return Color
function LinearRgba.to_color(_self) end

---@param _self LinearRgba
---@param other LinearRgba
---@return boolean
function LinearRgba.__eq(_self,other) end

---@param _self LinearRgba
---@param other LinearRgba
---@return boolean
function LinearRgba.eq(_self,other) end

---@param red number
---@param green number
---@param blue number
---@return LinearRgba
function LinearRgba.rgb(red,green,blue) end

---@param _self LinearRgba
---@param factor number
---@return LinearRgba
function LinearRgba.mul(_self,factor) end

---@param red number
---@param green number
---@param blue number
---@param alpha number
---@return LinearRgba
function LinearRgba.new(red,green,blue,alpha) end



---@class Oklaba : ReflectReference
---  Color in Oklab color space, with alpha
---  <div>
---  </div>
---@field  lightness? number
---@field  a? number
---@field  b? number
---@field  alpha? number
local Oklaba = {}

---@param _self Oklaba
---@return Color
function Oklaba.to_color(_self) end

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Oklaba
function Oklaba.new(a,b,c,alpha) end



---@class Oklcha : ReflectReference
---  Color in Oklch color space, with alpha
---  <div>
---  </div>
---@field  lightness? number
---@field  chroma? number
---@field  hue? number
---@field  alpha? number
local Oklcha = {}

---@param _self Oklcha
---@return Color
function Oklcha.to_color(_self) end

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Oklcha
function Oklcha.new(a,b,c,alpha) end



---@class Srgba : ReflectReference
---  Non-linear standard RGB with alpha.
--- 
---  SDR colors are in `[0.0, 1.0]`. Values above `1.0` are HDR intensities.
---  <div>
---  </div>
---@field  red? number
---@field  green? number
---@field  blue? number
---@field  alpha? number
local Srgba = {}

---@param _self Srgba
---@param other Srgba
---@return boolean
function Srgba.__eq(_self,other) end

---@param _self Srgba
---@param other Srgba
---@return boolean
function Srgba.eq(_self,other) end

---@param _self Srgba
---@param alpha number
---@return Srgba
function Srgba.with_alpha(_self,alpha) end

---@param _self Srgba
---@return Color
function Srgba.to_color(_self) end

---@param red number
---@param green number
---@param blue number
---@return Srgba
function Srgba.rgb(red,green,blue) end

---@param _self Srgba
---@return string
function Srgba.to_hex(_self) end

---@param hex string
---@return Srgba
function Srgba.hex(hex) end

---@param red number
---@param green number
---@param blue number
---@param alpha number
---@return Srgba
function Srgba.new(red,green,blue,alpha) end



---@class Xyza : ReflectReference
---  [CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.
--- 
---  The SDR channel bounds below are the D65 white point, `(0.9505, 1.0, 1.0889)`.
---  <div>
---  </div>
---@field  x? number
---@field  y? number
---@field  z? number
---@field  alpha? number
local Xyza = {}

---@param a number
---@param b number
---@param c number
---@param alpha number
---@return Xyza
function Xyza.new(a,b,c,alpha) end

---@param _self Xyza
---@return Color
function Xyza.to_color(_self) end



---@class EaseFunction : ReflectReference
---  Curve functions over the [unit interval], commonly used for easing transitions.
--- 
---  `EaseFunction` can be used on its own to interpolate between `0.0` and `1.0`.
---  It can also be combined with [`EasingCurve`] to interpolate between other
---  intervals and types, including vectors and rotations.
--- 
---  # Example
--- 
---  [`sample`] the smoothstep function at various points. This will return `None`
---  if the parameter is outside the unit interval.
--- 
---  ```
---  # use bevy_math::prelude::*;
---  # use bevy_curve::prelude::*;
---  let f = EaseFunction::SmoothStep;
--- 
---  assert_eq!(f.sample(-1.0), None);
---  assert_eq!(f.sample(0.0), Some(0.0));
---  assert_eq!(f.sample(0.5), Some(0.5));
---  assert_eq!(f.sample(1.0), Some(1.0));
---  assert_eq!(f.sample(2.0), None);
---  ```
--- 
---  [`sample_clamped`] will clamp the parameter to the unit interval, so it
---  always returns a value.
--- 
---  ```
---  # use bevy_math::prelude::*;
---  # use bevy_curve::prelude::*;
---  # let f = EaseFunction::SmoothStep;
---  assert_eq!(f.sample_clamped(-1.0), 0.0);
---  assert_eq!(f.sample_clamped(0.0), 0.0);
---  assert_eq!(f.sample_clamped(0.5), 0.5);
---  assert_eq!(f.sample_clamped(1.0), 1.0);
---  assert_eq!(f.sample_clamped(2.0), 1.0);
---  ```
--- 
---  [`sample`]: EaseFunction::sample
---  [`sample_clamped`]: EaseFunction::sample_clamped
---  [unit interval]: `Interval::UNIT`
---@field  Linear EaseFunction.Linear
---@field  QuadraticIn EaseFunction.QuadraticIn
---@field  QuadraticOut EaseFunction.QuadraticOut
---@field  QuadraticInOut EaseFunction.QuadraticInOut
---@field  CubicIn EaseFunction.CubicIn
---@field  CubicOut EaseFunction.CubicOut
---@field  CubicInOut EaseFunction.CubicInOut
---@field  QuarticIn EaseFunction.QuarticIn
---@field  QuarticOut EaseFunction.QuarticOut
---@field  QuarticInOut EaseFunction.QuarticInOut
---@field  QuinticIn EaseFunction.QuinticIn
---@field  QuinticOut EaseFunction.QuinticOut
---@field  QuinticInOut EaseFunction.QuinticInOut
---@field  SmoothStepIn EaseFunction.SmoothStepIn
---@field  SmoothStepOut EaseFunction.SmoothStepOut
---@field  SmoothStep EaseFunction.SmoothStep
---@field  SmootherStepIn EaseFunction.SmootherStepIn
---@field  SmootherStepOut EaseFunction.SmootherStepOut
---@field  SmootherStep EaseFunction.SmootherStep
---@field  SineIn EaseFunction.SineIn
---@field  SineOut EaseFunction.SineOut
---@field  SineInOut EaseFunction.SineInOut
---@field  CircularIn EaseFunction.CircularIn
---@field  CircularOut EaseFunction.CircularOut
---@field  CircularInOut EaseFunction.CircularInOut
---@field  ExponentialIn EaseFunction.ExponentialIn
---@field  ExponentialOut EaseFunction.ExponentialOut
---@field  ExponentialInOut EaseFunction.ExponentialInOut
---@field  ElasticIn EaseFunction.ElasticIn
---@field  ElasticOut EaseFunction.ElasticOut
---@field  ElasticInOut EaseFunction.ElasticInOut
---@field  BackIn EaseFunction.BackIn
---@field  BackOut EaseFunction.BackOut
---@field  BackInOut EaseFunction.BackInOut
---@field  BounceIn EaseFunction.BounceIn
---@field  BounceOut EaseFunction.BounceOut
---@field  BounceInOut EaseFunction.BounceInOut
---@field  Steps EaseFunction.Steps
---@field  Elastic EaseFunction.Elastic
local EaseFunction = {}

---@param _self EaseFunction
---@param t number
---@return number
function EaseFunction.sample(_self,t) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return EaseFunction
function EaseFunction.new(payload) end



---@class EaseFunction.Linear : EnumVariantRef
--- The `Linear` variant of `EaseFunction`.
EaseFunction.Linear = {}

---@param variant EaseFunction.Linear
---@param payload NoFields
---@return EaseFunction
function EaseFunction.Linear.new(variant,payload) end



---@class EaseFunction.QuadraticIn : EnumVariantRef
--- The `QuadraticIn` variant of `EaseFunction`.
EaseFunction.QuadraticIn = {}

---@param variant EaseFunction.QuadraticIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuadraticIn.new(variant,payload) end



---@class EaseFunction.QuadraticOut : EnumVariantRef
--- The `QuadraticOut` variant of `EaseFunction`.
EaseFunction.QuadraticOut = {}

---@param variant EaseFunction.QuadraticOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuadraticOut.new(variant,payload) end



---@class EaseFunction.QuadraticInOut : EnumVariantRef
--- The `QuadraticInOut` variant of `EaseFunction`.
EaseFunction.QuadraticInOut = {}

---@param variant EaseFunction.QuadraticInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuadraticInOut.new(variant,payload) end



---@class EaseFunction.CubicIn : EnumVariantRef
--- The `CubicIn` variant of `EaseFunction`.
EaseFunction.CubicIn = {}

---@param variant EaseFunction.CubicIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CubicIn.new(variant,payload) end



---@class EaseFunction.CubicOut : EnumVariantRef
--- The `CubicOut` variant of `EaseFunction`.
EaseFunction.CubicOut = {}

---@param variant EaseFunction.CubicOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CubicOut.new(variant,payload) end



---@class EaseFunction.CubicInOut : EnumVariantRef
--- The `CubicInOut` variant of `EaseFunction`.
EaseFunction.CubicInOut = {}

---@param variant EaseFunction.CubicInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CubicInOut.new(variant,payload) end



---@class EaseFunction.QuarticIn : EnumVariantRef
--- The `QuarticIn` variant of `EaseFunction`.
EaseFunction.QuarticIn = {}

---@param variant EaseFunction.QuarticIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuarticIn.new(variant,payload) end



---@class EaseFunction.QuarticOut : EnumVariantRef
--- The `QuarticOut` variant of `EaseFunction`.
EaseFunction.QuarticOut = {}

---@param variant EaseFunction.QuarticOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuarticOut.new(variant,payload) end



---@class EaseFunction.QuarticInOut : EnumVariantRef
--- The `QuarticInOut` variant of `EaseFunction`.
EaseFunction.QuarticInOut = {}

---@param variant EaseFunction.QuarticInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuarticInOut.new(variant,payload) end



---@class EaseFunction.QuinticIn : EnumVariantRef
--- The `QuinticIn` variant of `EaseFunction`.
EaseFunction.QuinticIn = {}

---@param variant EaseFunction.QuinticIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuinticIn.new(variant,payload) end



---@class EaseFunction.QuinticOut : EnumVariantRef
--- The `QuinticOut` variant of `EaseFunction`.
EaseFunction.QuinticOut = {}

---@param variant EaseFunction.QuinticOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuinticOut.new(variant,payload) end



---@class EaseFunction.QuinticInOut : EnumVariantRef
--- The `QuinticInOut` variant of `EaseFunction`.
EaseFunction.QuinticInOut = {}

---@param variant EaseFunction.QuinticInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.QuinticInOut.new(variant,payload) end



---@class EaseFunction.SmoothStepIn : EnumVariantRef
--- The `SmoothStepIn` variant of `EaseFunction`.
EaseFunction.SmoothStepIn = {}

---@param variant EaseFunction.SmoothStepIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmoothStepIn.new(variant,payload) end



---@class EaseFunction.SmoothStepOut : EnumVariantRef
--- The `SmoothStepOut` variant of `EaseFunction`.
EaseFunction.SmoothStepOut = {}

---@param variant EaseFunction.SmoothStepOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmoothStepOut.new(variant,payload) end



---@class EaseFunction.SmoothStep : EnumVariantRef
--- The `SmoothStep` variant of `EaseFunction`.
EaseFunction.SmoothStep = {}

---@param variant EaseFunction.SmoothStep
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmoothStep.new(variant,payload) end



---@class EaseFunction.SmootherStepIn : EnumVariantRef
--- The `SmootherStepIn` variant of `EaseFunction`.
EaseFunction.SmootherStepIn = {}

---@param variant EaseFunction.SmootherStepIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmootherStepIn.new(variant,payload) end



---@class EaseFunction.SmootherStepOut : EnumVariantRef
--- The `SmootherStepOut` variant of `EaseFunction`.
EaseFunction.SmootherStepOut = {}

---@param variant EaseFunction.SmootherStepOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmootherStepOut.new(variant,payload) end



---@class EaseFunction.SmootherStep : EnumVariantRef
--- The `SmootherStep` variant of `EaseFunction`.
EaseFunction.SmootherStep = {}

---@param variant EaseFunction.SmootherStep
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SmootherStep.new(variant,payload) end



---@class EaseFunction.SineIn : EnumVariantRef
--- The `SineIn` variant of `EaseFunction`.
EaseFunction.SineIn = {}

---@param variant EaseFunction.SineIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SineIn.new(variant,payload) end



---@class EaseFunction.SineOut : EnumVariantRef
--- The `SineOut` variant of `EaseFunction`.
EaseFunction.SineOut = {}

---@param variant EaseFunction.SineOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SineOut.new(variant,payload) end



---@class EaseFunction.SineInOut : EnumVariantRef
--- The `SineInOut` variant of `EaseFunction`.
EaseFunction.SineInOut = {}

---@param variant EaseFunction.SineInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.SineInOut.new(variant,payload) end



---@class EaseFunction.CircularIn : EnumVariantRef
--- The `CircularIn` variant of `EaseFunction`.
EaseFunction.CircularIn = {}

---@param variant EaseFunction.CircularIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CircularIn.new(variant,payload) end



---@class EaseFunction.CircularOut : EnumVariantRef
--- The `CircularOut` variant of `EaseFunction`.
EaseFunction.CircularOut = {}

---@param variant EaseFunction.CircularOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CircularOut.new(variant,payload) end



---@class EaseFunction.CircularInOut : EnumVariantRef
--- The `CircularInOut` variant of `EaseFunction`.
EaseFunction.CircularInOut = {}

---@param variant EaseFunction.CircularInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.CircularInOut.new(variant,payload) end



---@class EaseFunction.ExponentialIn : EnumVariantRef
--- The `ExponentialIn` variant of `EaseFunction`.
EaseFunction.ExponentialIn = {}

---@param variant EaseFunction.ExponentialIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ExponentialIn.new(variant,payload) end



---@class EaseFunction.ExponentialOut : EnumVariantRef
--- The `ExponentialOut` variant of `EaseFunction`.
EaseFunction.ExponentialOut = {}

---@param variant EaseFunction.ExponentialOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ExponentialOut.new(variant,payload) end



---@class EaseFunction.ExponentialInOut : EnumVariantRef
--- The `ExponentialInOut` variant of `EaseFunction`.
EaseFunction.ExponentialInOut = {}

---@param variant EaseFunction.ExponentialInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ExponentialInOut.new(variant,payload) end



---@class EaseFunction.ElasticIn : EnumVariantRef
--- The `ElasticIn` variant of `EaseFunction`.
EaseFunction.ElasticIn = {}

---@param variant EaseFunction.ElasticIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ElasticIn.new(variant,payload) end



---@class EaseFunction.ElasticOut : EnumVariantRef
--- The `ElasticOut` variant of `EaseFunction`.
EaseFunction.ElasticOut = {}

---@param variant EaseFunction.ElasticOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ElasticOut.new(variant,payload) end



---@class EaseFunction.ElasticInOut : EnumVariantRef
--- The `ElasticInOut` variant of `EaseFunction`.
EaseFunction.ElasticInOut = {}

---@param variant EaseFunction.ElasticInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.ElasticInOut.new(variant,payload) end



---@class EaseFunction.BackIn : EnumVariantRef
--- The `BackIn` variant of `EaseFunction`.
EaseFunction.BackIn = {}

---@param variant EaseFunction.BackIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BackIn.new(variant,payload) end



---@class EaseFunction.BackOut : EnumVariantRef
--- The `BackOut` variant of `EaseFunction`.
EaseFunction.BackOut = {}

---@param variant EaseFunction.BackOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BackOut.new(variant,payload) end



---@class EaseFunction.BackInOut : EnumVariantRef
--- The `BackInOut` variant of `EaseFunction`.
EaseFunction.BackInOut = {}

---@param variant EaseFunction.BackInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BackInOut.new(variant,payload) end



---@class EaseFunction.BounceIn : EnumVariantRef
--- The `BounceIn` variant of `EaseFunction`.
EaseFunction.BounceIn = {}

---@param variant EaseFunction.BounceIn
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BounceIn.new(variant,payload) end



---@class EaseFunction.BounceOut : EnumVariantRef
--- The `BounceOut` variant of `EaseFunction`.
EaseFunction.BounceOut = {}

---@param variant EaseFunction.BounceOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BounceOut.new(variant,payload) end



---@class EaseFunction.BounceInOut : EnumVariantRef
--- The `BounceInOut` variant of `EaseFunction`.
EaseFunction.BounceInOut = {}

---@param variant EaseFunction.BounceInOut
---@param payload NoFields
---@return EaseFunction
function EaseFunction.BounceInOut.new(variant,payload) end



---@class EaseFunction.Steps : EnumVariantRef
--- The `Steps` variant of `EaseFunction`.
EaseFunction.Steps = {}

---@param variant EaseFunction.Steps
---@param payload EaseFunction.Steps.Fields
---@return EaseFunction
function EaseFunction.Steps.new(variant,payload) end



---@class EaseFunction.Elastic : EnumVariantRef
--- The `Elastic` variant of `EaseFunction`.
EaseFunction.Elastic = {}

---@param variant EaseFunction.Elastic
---@param payload EaseFunction.Elastic.Fields
---@return EaseFunction
function EaseFunction.Elastic.new(variant,payload) end



---@class Entity : ReflectReference
---  Unique identifier for an entity in a [`World`].
---  Note that this is just an id, not the entity itself.
---  Further, the entity this id refers to may no longer exist in the [`World`].
---  For more information about entities, their ids, and how to use them, see the module [docs](crate::entity).
--- 
---  # Aliasing
--- 
---  Once an entity is despawned, it ceases to exist.
---  However, its [`Entity`] id is still present, and may still be contained in some data.
---  This becomes problematic because it is possible for a later entity to be spawned at the exact same id!
---  If this happens, which is rare but very possible, it will be logged.
--- 
---  Aliasing can happen without warning.
---  Holding onto a [`Entity`] id corresponding to an entity well after that entity was despawned can cause un-intuitive behavior for both ordering, and comparing in general.
---  To prevent these bugs, it is generally best practice to stop holding an [`Entity`] or [`EntityGeneration`] value as soon as you know it has been despawned.
---  If you must do otherwise, do not assume the [`Entity`] id corresponds to the same entity it originally did.
---  See [`EntityGeneration`]'s docs for more information about aliasing and why it occurs.
--- 
---  # Stability warning
---  For all intents and purposes, `Entity` should be treated as an opaque identifier. The internal bit
---  representation is liable to change from release to release as are the behaviors or performance
---  characteristics of any of its trait implementations (i.e. `Ord`, `Hash`, etc.). This means that changes in
---  `Entity`'s representation, though made readable through various functions on the type, are not considered
---  breaking changes under [SemVer].
--- 
---  In particular, directly serializing with `Serialize` and `Deserialize` make zero guarantee of long
---  term wire format compatibility. Changes in behavior will cause serialized `Entity` values persisted
---  to long term storage (i.e. disk, databases, etc.) will fail to deserialize upon being updated.
--- 
---  # Usage
--- 
---  This data type is returned by iterating a `Query` that has `Entity` as part of its query fetch type parameter ([learn more]).
---  It can also be obtained by calling [`EntityCommands::id`] or [`EntityWorldMut::id`].
--- 
---  ```
---  # use bevy_ecs::prelude::*;
---  # #[derive(Component)]
---  # struct SomeComponent;
---  fn setup(mut commands: Commands) {
---      // Calling `spawn` returns `EntityCommands`.
---      let entity = commands.spawn(SomeComponent).id();
---  }
--- 
---  fn exclusive_system(world: &mut World) {
---      // Calling `spawn` returns `EntityWorldMut`.
---      let entity = world.spawn(SomeComponent).id();
---  }
---  #
---  # bevy_ecs::system::assert_is_system(setup);
---  # bevy_ecs::system::assert_is_system(exclusive_system);
---  ```
--- 
---  It can be used to refer to a specific entity to apply [`EntityCommands`], or to call [`Query::get`] (or similar methods) to access its components.
--- 
---  ```
---  # use bevy_ecs::prelude::*;
---  #
---  # #[derive(Component)]
---  # struct Expired;
---  #
---  fn dispose_expired_food(mut commands: Commands, query: Query<Entity, With<Expired>>) {
---      for food_entity in &query {
---          commands.entity(food_entity).despawn();
---      }
---  }
---  #
---  # bevy_ecs::system::assert_is_system(dispose_expired_food);
---  ```
--- 
---  [learn more]: crate::system::Query#entity-id-access
---  [`EntityCommands::id`]: crate::system::EntityCommands::id
---  [`EntityWorldMut::id`]: crate::world::EntityWorldMut::id
---  [`EntityCommands`]: crate::system::EntityCommands
---  [`Query::get`]: crate::system::Query::get
---  [`World`]: crate::world::World
---  [SemVer]: https://semver.org/
local Entity = {}

---@param _self Entity
---@return integer
function Entity.index(_self) end

---@param bits integer
---@return Entity | nil
function Entity.from_bits(bits) end

---@param _self Entity
---@param other Entity
---@return boolean
function Entity.__eq(_self,other) end

---@param _self Entity
---@param other Entity
---@return boolean
function Entity.eq(_self,other) end

---@param _self Entity
---@return integer
function Entity.generation(_self) end

---@param _self Entity
---@return integer
function Entity.to_bits(_self) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Entity
function Entity.new(payload) end



---@class ChildOf : ReflectReference
---  Stores the parent entity of this child entity with this component.
--- 
---  This is a [`Relationship`] component, and creates the canonical
---  "parent / child" hierarchy. This is the "source of truth" component, and it pairs with
---  the [`Children`] [`RelationshipTarget`](crate::relationship::RelationshipTarget).
--- 
---  This relationship should be used for things like:
--- 
---  1. Organizing entities in a scene
---  2. Propagating configuration or data inherited from a parent, such as "visibility" or "world-space global transforms".
---  3. Ensuring a hierarchy is despawned when an entity is despawned.
--- 
---  [`ChildOf`] contains a single "target" [`Entity`]. When [`ChildOf`] is inserted on a "source" entity,
---  the "target" entity will automatically (and immediately, via a component hook) have a [`Children`]
---  component inserted, and the "source" entity will be added to that [`Children`] instance.
--- 
---  If the [`ChildOf`] component is replaced with a different "target" entity, the old target's [`Children`]
---  will be automatically (and immediately, via a component hook) be updated to reflect that change.
--- 
---  Likewise, when the [`ChildOf`] component is removed, the "source" entity will be removed from the old
---  target's [`Children`]. If this results in [`Children`] being empty, [`Children`] will be automatically removed.
--- 
---  When a parent is despawned, all children (and their descendants) will _also_ be despawned.
--- 
---  You can create parent-child relationships in a variety of ways. The most direct way is to insert a [`ChildOf`] component:
--- 
---  ```
---  # use bevy_ecs::prelude::*;
---  # let mut world = World::new();
---  let root = world.spawn_empty().id();
---  let child1 = world.spawn(ChildOf(root)).id();
---  let child2 = world.spawn(ChildOf(root)).id();
---  let grandchild = world.spawn(ChildOf(child1)).id();
--- 
---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1, child2]);
---  assert_eq!(&**world.entity(child1).get::<Children>().unwrap(), &[grandchild]);
--- 
---  world.entity_mut(child2).remove::<ChildOf>();
---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1]);
--- 
---  world.entity_mut(root).despawn();
---  assert!(world.get_entity(root).is_err());
---  assert!(world.get_entity(child1).is_err());
---  assert!(world.get_entity(grandchild).is_err());
---  ```
--- 
---  However if you are spawning many children, you might want to use the [`EntityWorldMut::with_children`] helper instead:
--- 
---  ```
---  # use bevy_ecs::prelude::*;
---  # let mut world = World::new();
---  let mut child1 = None;
---  let mut child2 = None;
---  let mut grandchild = None;
---  let root = world.spawn_empty().with_children(|p| {
---      child1 = Some(p.spawn_empty().with_children(|p| {
---          grandchild = Some(p.spawn_empty().id());
---      }).id());
---      child2 = Some(p.spawn_empty().id());
---  }).id();
--- 
---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1.unwrap(), child2.unwrap()]);
---  assert_eq!(&**world.entity(child1.unwrap()).get::<Children>().unwrap(), &[grandchild.unwrap()]);
---  ```
--- 
---  [`Relationship`]: crate::relationship::Relationship
---@field  [1] Entity
local ChildOf = {}

---@param _self ChildOf
---@return Entity
function ChildOf.parent(_self) end

---@param payload ChildOf.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ChildOf
function ChildOf.new(payload) end



---@class Name : ReflectReference
---  Component used to identify an entity. Stores a hash for faster comparisons.
--- 
---  The hash is eagerly re-computed upon each update to the name.
--- 
---  [`Name`] should not be treated as a globally unique identifier for entities,
---  as multiple entities can have the same name.  [`Entity`] should be
---  used instead as the default unique identifier.
---@field  [1] HashedStr
local Name = {}

---@param _self Name
---@return string
function Name.as_str(_self) end

---@param _self Name
---@param name string
---@return nil
function Name.set(_self,name) end

---@param _self Name
---@param other Name | string
---@return boolean
function Name.__eq(_self,other) end

---@param _self Name
---@param other Name | string
---@return boolean
function Name.eq(_self,other) end

---@param name string
---@return Name
function Name.new(name) end



---@class Image : ReflectReference
---  An image, optimized for usage in rendering.
--- 
---  ## Remote Inspection
--- 
---  To transmit an [`Image`] between two running Bevy apps, e.g. through BRP, use
--- `SerializedImage`, which requires the `serialize` feature.
---  This type is only meant for short-term transmission between same versions and should not be stored anywhere.
local Image = {}

---@param _self Image
---@return integer
function Image.width(_self) end

---@param _self Image
---@return UVec2
function Image.size(_self) end

---@param _self Image
---@return integer
function Image.height(_self) end

---@param _self Image
---@return Vec2
function Image.size_f32(_self) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Image
function Image.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<Image>
function Image.load(path) end

---@param value Image
--- The asset value.
---@return Handle<Image>
function Image.add_asset(value) end

---@param handle Handle<Image>
--- The handle to the asset.
---@return Image | nil
function Image.get_asset(handle) end



---@class TextureAtlasLayout : ReflectReference
---  Stores a map used to lookup the position of a texture in a [`TextureAtlas`].
---  This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.
--- 
---  Optionally it can store a mapping from sub texture handles to the related area index (see
---  [`TextureAtlasBuilder`]).
--- 
---  [Example usage animating sprite.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
---  [Example usage animating sprite in response to an event.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
---  [Example usage loading sprite sheet.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
--- 
---  [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
---@field  size? UVec2
---@field  textures? URect[]
---@operator len: integer
local TextureAtlasLayout = {}

---@param _self TextureAtlasLayout
---@return integer
function TextureAtlasLayout.len(_self) end

---@param tile_size UVec2
---@param columns integer
---@param rows integer
---@param padding? UVec2 | nil
---@param offset? UVec2 | nil
---@return TextureAtlasLayout
function TextureAtlasLayout.from_grid(tile_size,columns,rows,padding,offset) end

---@param payload TextureAtlasLayout.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TextureAtlasLayout
function TextureAtlasLayout.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<TextureAtlasLayout>
function TextureAtlasLayout.load(path) end

---@param value TextureAtlasLayout
--- The asset value.
---@return Handle<TextureAtlasLayout>
function TextureAtlasLayout.add_asset(value) end

---@param handle Handle<TextureAtlasLayout>
--- The handle to the asset.
---@return TextureAtlasLayout | nil
function TextureAtlasLayout.get_asset(handle) end



---@class Gamepad : ReflectReference
---  Stores a connected gamepad's metadata such as the name and its [`GamepadButton`] and [`GamepadAxis`].
--- 
---  An entity with this component is spawned automatically after [`GamepadConnectionEvent`]
---  and updated by [`gamepad_event_processing_system`].
--- 
---  See also [`GamepadSettings`] for configuration.
--- 
---  # Examples
--- 
---  ```
---  # use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
---  # use bevy_ecs::system::Query;
---  # use bevy_ecs::name::Name;
---  #
---  fn gamepad_usage_system(gamepads: Query<(&Name, &Gamepad)>) {
---      for (name, gamepad) in &gamepads {
---          println!("{name}");
--- 
---          if gamepad.just_pressed(GamepadButton::North) {
---              println!("{} just pressed North", name)
---          }
--- 
---          if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)  {
---              println!("left stick X: {}", left_stick_x)
---          }
---      }
---  }
---  ```
---@field  vendor_id? integer | nil
---@field  product_id? integer | nil
---@field  digital? any
---@field  analog? any
local Gamepad = {}

---@param _self Gamepad
---@return Vec2
function Gamepad.dpad(_self) end

---@param _self Gamepad
---@param button GamepadButton
---@return boolean
function Gamepad.pressed(_self,button) end

---@param _self Gamepad
---@return Vec2
function Gamepad.right_stick(_self) end

---@param _self Gamepad
---@param axis GamepadAxis
---@return number | nil
function Gamepad.get(_self,axis) end

---@param _self Gamepad
---@return Vec2
function Gamepad.left_stick(_self) end

---@param _self Gamepad
---@param button GamepadButton
---@return boolean
function Gamepad.just_pressed(_self,button) end

---@param _self Gamepad
---@param button GamepadButton
---@return boolean
function Gamepad.just_released(_self,button) end

---@param payload Gamepad.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Gamepad
function Gamepad.new(payload) end



---@class KeyCode : ReflectReference
---  The key code of a [`KeyboardInput`].
--- 
---  ## Usage
--- 
---  It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<ButtonInput<KeyCode>>`.
--- 
---  Code representing the location of a physical key
---  This mostly conforms to the [`UI Events Specification's KeyboardEvent.code`] with a few
---  exceptions:
---  - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and
---    `SuperRight` here.
---  - The key that the specification calls "Super" is reported as `Unidentified` here.
--- 
---  [`UI Events Specification's KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
--- 
---  ## Updating
--- 
---  The resource is updated inside of the [`keyboard_input_system`].
---@field  Unidentified KeyCode.Unidentified
---@field  Backquote KeyCode.Backquote
---@field  Backslash KeyCode.Backslash
---@field  BracketLeft KeyCode.BracketLeft
---@field  BracketRight KeyCode.BracketRight
---@field  Comma KeyCode.Comma
---@field  Digit0 KeyCode.Digit0
---@field  Digit1 KeyCode.Digit1
---@field  Digit2 KeyCode.Digit2
---@field  Digit3 KeyCode.Digit3
---@field  Digit4 KeyCode.Digit4
---@field  Digit5 KeyCode.Digit5
---@field  Digit6 KeyCode.Digit6
---@field  Digit7 KeyCode.Digit7
---@field  Digit8 KeyCode.Digit8
---@field  Digit9 KeyCode.Digit9
---@field  Equal KeyCode.Equal
---@field  IntlBackslash KeyCode.IntlBackslash
---@field  IntlRo KeyCode.IntlRo
---@field  IntlYen KeyCode.IntlYen
---@field  KeyA KeyCode.KeyA
---@field  KeyB KeyCode.KeyB
---@field  KeyC KeyCode.KeyC
---@field  KeyD KeyCode.KeyD
---@field  KeyE KeyCode.KeyE
---@field  KeyF KeyCode.KeyF
---@field  KeyG KeyCode.KeyG
---@field  KeyH KeyCode.KeyH
---@field  KeyI KeyCode.KeyI
---@field  KeyJ KeyCode.KeyJ
---@field  KeyK KeyCode.KeyK
---@field  KeyL KeyCode.KeyL
---@field  KeyM KeyCode.KeyM
---@field  KeyN KeyCode.KeyN
---@field  KeyO KeyCode.KeyO
---@field  KeyP KeyCode.KeyP
---@field  KeyQ KeyCode.KeyQ
---@field  KeyR KeyCode.KeyR
---@field  KeyS KeyCode.KeyS
---@field  KeyT KeyCode.KeyT
---@field  KeyU KeyCode.KeyU
---@field  KeyV KeyCode.KeyV
---@field  KeyW KeyCode.KeyW
---@field  KeyX KeyCode.KeyX
---@field  KeyY KeyCode.KeyY
---@field  KeyZ KeyCode.KeyZ
---@field  Minus KeyCode.Minus
---@field  Period KeyCode.Period
---@field  Quote KeyCode.Quote
---@field  Semicolon KeyCode.Semicolon
---@field  Slash KeyCode.Slash
---@field  AltLeft KeyCode.AltLeft
---@field  AltRight KeyCode.AltRight
---@field  Backspace KeyCode.Backspace
---@field  CapsLock KeyCode.CapsLock
---@field  ContextMenu KeyCode.ContextMenu
---@field  ControlLeft KeyCode.ControlLeft
---@field  ControlRight KeyCode.ControlRight
---@field  Enter KeyCode.Enter
---@field  SuperLeft KeyCode.SuperLeft
---@field  SuperRight KeyCode.SuperRight
---@field  ShiftLeft KeyCode.ShiftLeft
---@field  ShiftRight KeyCode.ShiftRight
---@field  Space KeyCode.Space
---@field  Tab KeyCode.Tab
---@field  Convert KeyCode.Convert
---@field  KanaMode KeyCode.KanaMode
---@field  Lang1 KeyCode.Lang1
---@field  Lang2 KeyCode.Lang2
---@field  Lang3 KeyCode.Lang3
---@field  Lang4 KeyCode.Lang4
---@field  Lang5 KeyCode.Lang5
---@field  NonConvert KeyCode.NonConvert
---@field  Delete KeyCode.Delete
---@field  End KeyCode.End
---@field  Help KeyCode.Help
---@field  Home KeyCode.Home
---@field  Insert KeyCode.Insert
---@field  PageDown KeyCode.PageDown
---@field  PageUp KeyCode.PageUp
---@field  ArrowDown KeyCode.ArrowDown
---@field  ArrowLeft KeyCode.ArrowLeft
---@field  ArrowRight KeyCode.ArrowRight
---@field  ArrowUp KeyCode.ArrowUp
---@field  NumLock KeyCode.NumLock
---@field  Numpad0 KeyCode.Numpad0
---@field  Numpad1 KeyCode.Numpad1
---@field  Numpad2 KeyCode.Numpad2
---@field  Numpad3 KeyCode.Numpad3
---@field  Numpad4 KeyCode.Numpad4
---@field  Numpad5 KeyCode.Numpad5
---@field  Numpad6 KeyCode.Numpad6
---@field  Numpad7 KeyCode.Numpad7
---@field  Numpad8 KeyCode.Numpad8
---@field  Numpad9 KeyCode.Numpad9
---@field  NumpadAdd KeyCode.NumpadAdd
---@field  NumpadBackspace KeyCode.NumpadBackspace
---@field  NumpadClear KeyCode.NumpadClear
---@field  NumpadClearEntry KeyCode.NumpadClearEntry
---@field  NumpadComma KeyCode.NumpadComma
---@field  NumpadDecimal KeyCode.NumpadDecimal
---@field  NumpadDivide KeyCode.NumpadDivide
---@field  NumpadEnter KeyCode.NumpadEnter
---@field  NumpadEqual KeyCode.NumpadEqual
---@field  NumpadHash KeyCode.NumpadHash
---@field  NumpadMemoryAdd KeyCode.NumpadMemoryAdd
---@field  NumpadMemoryClear KeyCode.NumpadMemoryClear
---@field  NumpadMemoryRecall KeyCode.NumpadMemoryRecall
---@field  NumpadMemoryStore KeyCode.NumpadMemoryStore
---@field  NumpadMemorySubtract KeyCode.NumpadMemorySubtract
---@field  NumpadMultiply KeyCode.NumpadMultiply
---@field  NumpadParenLeft KeyCode.NumpadParenLeft
---@field  NumpadParenRight KeyCode.NumpadParenRight
---@field  NumpadStar KeyCode.NumpadStar
---@field  NumpadSubtract KeyCode.NumpadSubtract
---@field  Escape KeyCode.Escape
---@field  Fn KeyCode.Fn
---@field  FnLock KeyCode.FnLock
---@field  PrintScreen KeyCode.PrintScreen
---@field  ScrollLock KeyCode.ScrollLock
---@field  Pause KeyCode.Pause
---@field  BrowserBack KeyCode.BrowserBack
---@field  BrowserFavorites KeyCode.BrowserFavorites
---@field  BrowserForward KeyCode.BrowserForward
---@field  BrowserHome KeyCode.BrowserHome
---@field  BrowserRefresh KeyCode.BrowserRefresh
---@field  BrowserSearch KeyCode.BrowserSearch
---@field  BrowserStop KeyCode.BrowserStop
---@field  Eject KeyCode.Eject
---@field  LaunchApp1 KeyCode.LaunchApp1
---@field  LaunchApp2 KeyCode.LaunchApp2
---@field  LaunchMail KeyCode.LaunchMail
---@field  MediaPlayPause KeyCode.MediaPlayPause
---@field  MediaSelect KeyCode.MediaSelect
---@field  MediaStop KeyCode.MediaStop
---@field  MediaTrackNext KeyCode.MediaTrackNext
---@field  MediaTrackPrevious KeyCode.MediaTrackPrevious
---@field  Power KeyCode.Power
---@field  Sleep KeyCode.Sleep
---@field  AudioVolumeDown KeyCode.AudioVolumeDown
---@field  AudioVolumeMute KeyCode.AudioVolumeMute
---@field  AudioVolumeUp KeyCode.AudioVolumeUp
---@field  WakeUp KeyCode.WakeUp
---@field  Meta KeyCode.Meta
---@field  Hyper KeyCode.Hyper
---@field  Turbo KeyCode.Turbo
---@field  Abort KeyCode.Abort
---@field  Resume KeyCode.Resume
---@field  Suspend KeyCode.Suspend
---@field  Again KeyCode.Again
---@field  Copy KeyCode.Copy
---@field  Cut KeyCode.Cut
---@field  Find KeyCode.Find
---@field  Open KeyCode.Open
---@field  Paste KeyCode.Paste
---@field  Props KeyCode.Props
---@field  Select KeyCode.Select
---@field  Undo KeyCode.Undo
---@field  Hiragana KeyCode.Hiragana
---@field  Katakana KeyCode.Katakana
---@field  F1 KeyCode.F1
---@field  F2 KeyCode.F2
---@field  F3 KeyCode.F3
---@field  F4 KeyCode.F4
---@field  F5 KeyCode.F5
---@field  F6 KeyCode.F6
---@field  F7 KeyCode.F7
---@field  F8 KeyCode.F8
---@field  F9 KeyCode.F9
---@field  F10 KeyCode.F10
---@field  F11 KeyCode.F11
---@field  F12 KeyCode.F12
---@field  F13 KeyCode.F13
---@field  F14 KeyCode.F14
---@field  F15 KeyCode.F15
---@field  F16 KeyCode.F16
---@field  F17 KeyCode.F17
---@field  F18 KeyCode.F18
---@field  F19 KeyCode.F19
---@field  F20 KeyCode.F20
---@field  F21 KeyCode.F21
---@field  F22 KeyCode.F22
---@field  F23 KeyCode.F23
---@field  F24 KeyCode.F24
---@field  F25 KeyCode.F25
---@field  F26 KeyCode.F26
---@field  F27 KeyCode.F27
---@field  F28 KeyCode.F28
---@field  F29 KeyCode.F29
---@field  F30 KeyCode.F30
---@field  F31 KeyCode.F31
---@field  F32 KeyCode.F32
---@field  F33 KeyCode.F33
---@field  F34 KeyCode.F34
---@field  F35 KeyCode.F35
local KeyCode = {}

---@param key KeyCode
---@return boolean
function KeyCode.just_released(key) end

---@param keys KeyCode[]
---@return boolean
function KeyCode.any_just_pressed(keys) end

---@return KeyCode[]
function KeyCode.get_just_pressed() end

---@param key KeyCode
---@return boolean
function KeyCode.pressed(key) end

---@param keys KeyCode[]
---@return boolean
function KeyCode.any_pressed(keys) end

---@param key KeyCode
---@return boolean
function KeyCode.just_pressed(key) end

---@return KeyCode[]
function KeyCode.get_pressed() end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return KeyCode
function KeyCode.new(payload) end



---@class KeyCode.Unidentified : EnumVariantRef
--- The `Unidentified` variant of `KeyCode`.
KeyCode.Unidentified = {}

---@param variant KeyCode.Unidentified
---@param payload KeyCode.Unidentified.Fields
---@return KeyCode
function KeyCode.Unidentified.new(variant,payload) end



---@class KeyCode.Backquote : EnumVariantRef
--- The `Backquote` variant of `KeyCode`.
KeyCode.Backquote = {}

---@param variant KeyCode.Backquote
---@param payload NoFields
---@return KeyCode
function KeyCode.Backquote.new(variant,payload) end



---@class KeyCode.Backslash : EnumVariantRef
--- The `Backslash` variant of `KeyCode`.
KeyCode.Backslash = {}

---@param variant KeyCode.Backslash
---@param payload NoFields
---@return KeyCode
function KeyCode.Backslash.new(variant,payload) end



---@class KeyCode.BracketLeft : EnumVariantRef
--- The `BracketLeft` variant of `KeyCode`.
KeyCode.BracketLeft = {}

---@param variant KeyCode.BracketLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.BracketLeft.new(variant,payload) end



---@class KeyCode.BracketRight : EnumVariantRef
--- The `BracketRight` variant of `KeyCode`.
KeyCode.BracketRight = {}

---@param variant KeyCode.BracketRight
---@param payload NoFields
---@return KeyCode
function KeyCode.BracketRight.new(variant,payload) end



---@class KeyCode.Comma : EnumVariantRef
--- The `Comma` variant of `KeyCode`.
KeyCode.Comma = {}

---@param variant KeyCode.Comma
---@param payload NoFields
---@return KeyCode
function KeyCode.Comma.new(variant,payload) end



---@class KeyCode.Digit0 : EnumVariantRef
--- The `Digit0` variant of `KeyCode`.
KeyCode.Digit0 = {}

---@param variant KeyCode.Digit0
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit0.new(variant,payload) end



---@class KeyCode.Digit1 : EnumVariantRef
--- The `Digit1` variant of `KeyCode`.
KeyCode.Digit1 = {}

---@param variant KeyCode.Digit1
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit1.new(variant,payload) end



---@class KeyCode.Digit2 : EnumVariantRef
--- The `Digit2` variant of `KeyCode`.
KeyCode.Digit2 = {}

---@param variant KeyCode.Digit2
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit2.new(variant,payload) end



---@class KeyCode.Digit3 : EnumVariantRef
--- The `Digit3` variant of `KeyCode`.
KeyCode.Digit3 = {}

---@param variant KeyCode.Digit3
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit3.new(variant,payload) end



---@class KeyCode.Digit4 : EnumVariantRef
--- The `Digit4` variant of `KeyCode`.
KeyCode.Digit4 = {}

---@param variant KeyCode.Digit4
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit4.new(variant,payload) end



---@class KeyCode.Digit5 : EnumVariantRef
--- The `Digit5` variant of `KeyCode`.
KeyCode.Digit5 = {}

---@param variant KeyCode.Digit5
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit5.new(variant,payload) end



---@class KeyCode.Digit6 : EnumVariantRef
--- The `Digit6` variant of `KeyCode`.
KeyCode.Digit6 = {}

---@param variant KeyCode.Digit6
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit6.new(variant,payload) end



---@class KeyCode.Digit7 : EnumVariantRef
--- The `Digit7` variant of `KeyCode`.
KeyCode.Digit7 = {}

---@param variant KeyCode.Digit7
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit7.new(variant,payload) end



---@class KeyCode.Digit8 : EnumVariantRef
--- The `Digit8` variant of `KeyCode`.
KeyCode.Digit8 = {}

---@param variant KeyCode.Digit8
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit8.new(variant,payload) end



---@class KeyCode.Digit9 : EnumVariantRef
--- The `Digit9` variant of `KeyCode`.
KeyCode.Digit9 = {}

---@param variant KeyCode.Digit9
---@param payload NoFields
---@return KeyCode
function KeyCode.Digit9.new(variant,payload) end



---@class KeyCode.Equal : EnumVariantRef
--- The `Equal` variant of `KeyCode`.
KeyCode.Equal = {}

---@param variant KeyCode.Equal
---@param payload NoFields
---@return KeyCode
function KeyCode.Equal.new(variant,payload) end



---@class KeyCode.IntlBackslash : EnumVariantRef
--- The `IntlBackslash` variant of `KeyCode`.
KeyCode.IntlBackslash = {}

---@param variant KeyCode.IntlBackslash
---@param payload NoFields
---@return KeyCode
function KeyCode.IntlBackslash.new(variant,payload) end



---@class KeyCode.IntlRo : EnumVariantRef
--- The `IntlRo` variant of `KeyCode`.
KeyCode.IntlRo = {}

---@param variant KeyCode.IntlRo
---@param payload NoFields
---@return KeyCode
function KeyCode.IntlRo.new(variant,payload) end



---@class KeyCode.IntlYen : EnumVariantRef
--- The `IntlYen` variant of `KeyCode`.
KeyCode.IntlYen = {}

---@param variant KeyCode.IntlYen
---@param payload NoFields
---@return KeyCode
function KeyCode.IntlYen.new(variant,payload) end



---@class KeyCode.KeyA : EnumVariantRef
--- The `KeyA` variant of `KeyCode`.
KeyCode.KeyA = {}

---@param variant KeyCode.KeyA
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyA.new(variant,payload) end



---@class KeyCode.KeyB : EnumVariantRef
--- The `KeyB` variant of `KeyCode`.
KeyCode.KeyB = {}

---@param variant KeyCode.KeyB
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyB.new(variant,payload) end



---@class KeyCode.KeyC : EnumVariantRef
--- The `KeyC` variant of `KeyCode`.
KeyCode.KeyC = {}

---@param variant KeyCode.KeyC
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyC.new(variant,payload) end



---@class KeyCode.KeyD : EnumVariantRef
--- The `KeyD` variant of `KeyCode`.
KeyCode.KeyD = {}

---@param variant KeyCode.KeyD
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyD.new(variant,payload) end



---@class KeyCode.KeyE : EnumVariantRef
--- The `KeyE` variant of `KeyCode`.
KeyCode.KeyE = {}

---@param variant KeyCode.KeyE
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyE.new(variant,payload) end



---@class KeyCode.KeyF : EnumVariantRef
--- The `KeyF` variant of `KeyCode`.
KeyCode.KeyF = {}

---@param variant KeyCode.KeyF
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyF.new(variant,payload) end



---@class KeyCode.KeyG : EnumVariantRef
--- The `KeyG` variant of `KeyCode`.
KeyCode.KeyG = {}

---@param variant KeyCode.KeyG
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyG.new(variant,payload) end



---@class KeyCode.KeyH : EnumVariantRef
--- The `KeyH` variant of `KeyCode`.
KeyCode.KeyH = {}

---@param variant KeyCode.KeyH
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyH.new(variant,payload) end



---@class KeyCode.KeyI : EnumVariantRef
--- The `KeyI` variant of `KeyCode`.
KeyCode.KeyI = {}

---@param variant KeyCode.KeyI
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyI.new(variant,payload) end



---@class KeyCode.KeyJ : EnumVariantRef
--- The `KeyJ` variant of `KeyCode`.
KeyCode.KeyJ = {}

---@param variant KeyCode.KeyJ
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyJ.new(variant,payload) end



---@class KeyCode.KeyK : EnumVariantRef
--- The `KeyK` variant of `KeyCode`.
KeyCode.KeyK = {}

---@param variant KeyCode.KeyK
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyK.new(variant,payload) end



---@class KeyCode.KeyL : EnumVariantRef
--- The `KeyL` variant of `KeyCode`.
KeyCode.KeyL = {}

---@param variant KeyCode.KeyL
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyL.new(variant,payload) end



---@class KeyCode.KeyM : EnumVariantRef
--- The `KeyM` variant of `KeyCode`.
KeyCode.KeyM = {}

---@param variant KeyCode.KeyM
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyM.new(variant,payload) end



---@class KeyCode.KeyN : EnumVariantRef
--- The `KeyN` variant of `KeyCode`.
KeyCode.KeyN = {}

---@param variant KeyCode.KeyN
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyN.new(variant,payload) end



---@class KeyCode.KeyO : EnumVariantRef
--- The `KeyO` variant of `KeyCode`.
KeyCode.KeyO = {}

---@param variant KeyCode.KeyO
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyO.new(variant,payload) end



---@class KeyCode.KeyP : EnumVariantRef
--- The `KeyP` variant of `KeyCode`.
KeyCode.KeyP = {}

---@param variant KeyCode.KeyP
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyP.new(variant,payload) end



---@class KeyCode.KeyQ : EnumVariantRef
--- The `KeyQ` variant of `KeyCode`.
KeyCode.KeyQ = {}

---@param variant KeyCode.KeyQ
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyQ.new(variant,payload) end



---@class KeyCode.KeyR : EnumVariantRef
--- The `KeyR` variant of `KeyCode`.
KeyCode.KeyR = {}

---@param variant KeyCode.KeyR
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyR.new(variant,payload) end



---@class KeyCode.KeyS : EnumVariantRef
--- The `KeyS` variant of `KeyCode`.
KeyCode.KeyS = {}

---@param variant KeyCode.KeyS
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyS.new(variant,payload) end



---@class KeyCode.KeyT : EnumVariantRef
--- The `KeyT` variant of `KeyCode`.
KeyCode.KeyT = {}

---@param variant KeyCode.KeyT
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyT.new(variant,payload) end



---@class KeyCode.KeyU : EnumVariantRef
--- The `KeyU` variant of `KeyCode`.
KeyCode.KeyU = {}

---@param variant KeyCode.KeyU
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyU.new(variant,payload) end



---@class KeyCode.KeyV : EnumVariantRef
--- The `KeyV` variant of `KeyCode`.
KeyCode.KeyV = {}

---@param variant KeyCode.KeyV
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyV.new(variant,payload) end



---@class KeyCode.KeyW : EnumVariantRef
--- The `KeyW` variant of `KeyCode`.
KeyCode.KeyW = {}

---@param variant KeyCode.KeyW
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyW.new(variant,payload) end



---@class KeyCode.KeyX : EnumVariantRef
--- The `KeyX` variant of `KeyCode`.
KeyCode.KeyX = {}

---@param variant KeyCode.KeyX
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyX.new(variant,payload) end



---@class KeyCode.KeyY : EnumVariantRef
--- The `KeyY` variant of `KeyCode`.
KeyCode.KeyY = {}

---@param variant KeyCode.KeyY
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyY.new(variant,payload) end



---@class KeyCode.KeyZ : EnumVariantRef
--- The `KeyZ` variant of `KeyCode`.
KeyCode.KeyZ = {}

---@param variant KeyCode.KeyZ
---@param payload NoFields
---@return KeyCode
function KeyCode.KeyZ.new(variant,payload) end



---@class KeyCode.Minus : EnumVariantRef
--- The `Minus` variant of `KeyCode`.
KeyCode.Minus = {}

---@param variant KeyCode.Minus
---@param payload NoFields
---@return KeyCode
function KeyCode.Minus.new(variant,payload) end



---@class KeyCode.Period : EnumVariantRef
--- The `Period` variant of `KeyCode`.
KeyCode.Period = {}

---@param variant KeyCode.Period
---@param payload NoFields
---@return KeyCode
function KeyCode.Period.new(variant,payload) end



---@class KeyCode.Quote : EnumVariantRef
--- The `Quote` variant of `KeyCode`.
KeyCode.Quote = {}

---@param variant KeyCode.Quote
---@param payload NoFields
---@return KeyCode
function KeyCode.Quote.new(variant,payload) end



---@class KeyCode.Semicolon : EnumVariantRef
--- The `Semicolon` variant of `KeyCode`.
KeyCode.Semicolon = {}

---@param variant KeyCode.Semicolon
---@param payload NoFields
---@return KeyCode
function KeyCode.Semicolon.new(variant,payload) end



---@class KeyCode.Slash : EnumVariantRef
--- The `Slash` variant of `KeyCode`.
KeyCode.Slash = {}

---@param variant KeyCode.Slash
---@param payload NoFields
---@return KeyCode
function KeyCode.Slash.new(variant,payload) end



---@class KeyCode.AltLeft : EnumVariantRef
--- The `AltLeft` variant of `KeyCode`.
KeyCode.AltLeft = {}

---@param variant KeyCode.AltLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.AltLeft.new(variant,payload) end



---@class KeyCode.AltRight : EnumVariantRef
--- The `AltRight` variant of `KeyCode`.
KeyCode.AltRight = {}

---@param variant KeyCode.AltRight
---@param payload NoFields
---@return KeyCode
function KeyCode.AltRight.new(variant,payload) end



---@class KeyCode.Backspace : EnumVariantRef
--- The `Backspace` variant of `KeyCode`.
KeyCode.Backspace = {}

---@param variant KeyCode.Backspace
---@param payload NoFields
---@return KeyCode
function KeyCode.Backspace.new(variant,payload) end



---@class KeyCode.CapsLock : EnumVariantRef
--- The `CapsLock` variant of `KeyCode`.
KeyCode.CapsLock = {}

---@param variant KeyCode.CapsLock
---@param payload NoFields
---@return KeyCode
function KeyCode.CapsLock.new(variant,payload) end



---@class KeyCode.ContextMenu : EnumVariantRef
--- The `ContextMenu` variant of `KeyCode`.
KeyCode.ContextMenu = {}

---@param variant KeyCode.ContextMenu
---@param payload NoFields
---@return KeyCode
function KeyCode.ContextMenu.new(variant,payload) end



---@class KeyCode.ControlLeft : EnumVariantRef
--- The `ControlLeft` variant of `KeyCode`.
KeyCode.ControlLeft = {}

---@param variant KeyCode.ControlLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.ControlLeft.new(variant,payload) end



---@class KeyCode.ControlRight : EnumVariantRef
--- The `ControlRight` variant of `KeyCode`.
KeyCode.ControlRight = {}

---@param variant KeyCode.ControlRight
---@param payload NoFields
---@return KeyCode
function KeyCode.ControlRight.new(variant,payload) end



---@class KeyCode.Enter : EnumVariantRef
--- The `Enter` variant of `KeyCode`.
KeyCode.Enter = {}

---@param variant KeyCode.Enter
---@param payload NoFields
---@return KeyCode
function KeyCode.Enter.new(variant,payload) end



---@class KeyCode.SuperLeft : EnumVariantRef
--- The `SuperLeft` variant of `KeyCode`.
KeyCode.SuperLeft = {}

---@param variant KeyCode.SuperLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.SuperLeft.new(variant,payload) end



---@class KeyCode.SuperRight : EnumVariantRef
--- The `SuperRight` variant of `KeyCode`.
KeyCode.SuperRight = {}

---@param variant KeyCode.SuperRight
---@param payload NoFields
---@return KeyCode
function KeyCode.SuperRight.new(variant,payload) end



---@class KeyCode.ShiftLeft : EnumVariantRef
--- The `ShiftLeft` variant of `KeyCode`.
KeyCode.ShiftLeft = {}

---@param variant KeyCode.ShiftLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.ShiftLeft.new(variant,payload) end



---@class KeyCode.ShiftRight : EnumVariantRef
--- The `ShiftRight` variant of `KeyCode`.
KeyCode.ShiftRight = {}

---@param variant KeyCode.ShiftRight
---@param payload NoFields
---@return KeyCode
function KeyCode.ShiftRight.new(variant,payload) end



---@class KeyCode.Space : EnumVariantRef
--- The `Space` variant of `KeyCode`.
KeyCode.Space = {}

---@param variant KeyCode.Space
---@param payload NoFields
---@return KeyCode
function KeyCode.Space.new(variant,payload) end



---@class KeyCode.Tab : EnumVariantRef
--- The `Tab` variant of `KeyCode`.
KeyCode.Tab = {}

---@param variant KeyCode.Tab
---@param payload NoFields
---@return KeyCode
function KeyCode.Tab.new(variant,payload) end



---@class KeyCode.Convert : EnumVariantRef
--- The `Convert` variant of `KeyCode`.
KeyCode.Convert = {}

---@param variant KeyCode.Convert
---@param payload NoFields
---@return KeyCode
function KeyCode.Convert.new(variant,payload) end



---@class KeyCode.KanaMode : EnumVariantRef
--- The `KanaMode` variant of `KeyCode`.
KeyCode.KanaMode = {}

---@param variant KeyCode.KanaMode
---@param payload NoFields
---@return KeyCode
function KeyCode.KanaMode.new(variant,payload) end



---@class KeyCode.Lang1 : EnumVariantRef
--- The `Lang1` variant of `KeyCode`.
KeyCode.Lang1 = {}

---@param variant KeyCode.Lang1
---@param payload NoFields
---@return KeyCode
function KeyCode.Lang1.new(variant,payload) end



---@class KeyCode.Lang2 : EnumVariantRef
--- The `Lang2` variant of `KeyCode`.
KeyCode.Lang2 = {}

---@param variant KeyCode.Lang2
---@param payload NoFields
---@return KeyCode
function KeyCode.Lang2.new(variant,payload) end



---@class KeyCode.Lang3 : EnumVariantRef
--- The `Lang3` variant of `KeyCode`.
KeyCode.Lang3 = {}

---@param variant KeyCode.Lang3
---@param payload NoFields
---@return KeyCode
function KeyCode.Lang3.new(variant,payload) end



---@class KeyCode.Lang4 : EnumVariantRef
--- The `Lang4` variant of `KeyCode`.
KeyCode.Lang4 = {}

---@param variant KeyCode.Lang4
---@param payload NoFields
---@return KeyCode
function KeyCode.Lang4.new(variant,payload) end



---@class KeyCode.Lang5 : EnumVariantRef
--- The `Lang5` variant of `KeyCode`.
KeyCode.Lang5 = {}

---@param variant KeyCode.Lang5
---@param payload NoFields
---@return KeyCode
function KeyCode.Lang5.new(variant,payload) end



---@class KeyCode.NonConvert : EnumVariantRef
--- The `NonConvert` variant of `KeyCode`.
KeyCode.NonConvert = {}

---@param variant KeyCode.NonConvert
---@param payload NoFields
---@return KeyCode
function KeyCode.NonConvert.new(variant,payload) end



---@class KeyCode.Delete : EnumVariantRef
--- The `Delete` variant of `KeyCode`.
KeyCode.Delete = {}

---@param variant KeyCode.Delete
---@param payload NoFields
---@return KeyCode
function KeyCode.Delete.new(variant,payload) end



---@class KeyCode.End : EnumVariantRef
--- The `End` variant of `KeyCode`.
KeyCode.End = {}

---@param variant KeyCode.End
---@param payload NoFields
---@return KeyCode
function KeyCode.End.new(variant,payload) end



---@class KeyCode.Help : EnumVariantRef
--- The `Help` variant of `KeyCode`.
KeyCode.Help = {}

---@param variant KeyCode.Help
---@param payload NoFields
---@return KeyCode
function KeyCode.Help.new(variant,payload) end



---@class KeyCode.Home : EnumVariantRef
--- The `Home` variant of `KeyCode`.
KeyCode.Home = {}

---@param variant KeyCode.Home
---@param payload NoFields
---@return KeyCode
function KeyCode.Home.new(variant,payload) end



---@class KeyCode.Insert : EnumVariantRef
--- The `Insert` variant of `KeyCode`.
KeyCode.Insert = {}

---@param variant KeyCode.Insert
---@param payload NoFields
---@return KeyCode
function KeyCode.Insert.new(variant,payload) end



---@class KeyCode.PageDown : EnumVariantRef
--- The `PageDown` variant of `KeyCode`.
KeyCode.PageDown = {}

---@param variant KeyCode.PageDown
---@param payload NoFields
---@return KeyCode
function KeyCode.PageDown.new(variant,payload) end



---@class KeyCode.PageUp : EnumVariantRef
--- The `PageUp` variant of `KeyCode`.
KeyCode.PageUp = {}

---@param variant KeyCode.PageUp
---@param payload NoFields
---@return KeyCode
function KeyCode.PageUp.new(variant,payload) end



---@class KeyCode.ArrowDown : EnumVariantRef
--- The `ArrowDown` variant of `KeyCode`.
KeyCode.ArrowDown = {}

---@param variant KeyCode.ArrowDown
---@param payload NoFields
---@return KeyCode
function KeyCode.ArrowDown.new(variant,payload) end



---@class KeyCode.ArrowLeft : EnumVariantRef
--- The `ArrowLeft` variant of `KeyCode`.
KeyCode.ArrowLeft = {}

---@param variant KeyCode.ArrowLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.ArrowLeft.new(variant,payload) end



---@class KeyCode.ArrowRight : EnumVariantRef
--- The `ArrowRight` variant of `KeyCode`.
KeyCode.ArrowRight = {}

---@param variant KeyCode.ArrowRight
---@param payload NoFields
---@return KeyCode
function KeyCode.ArrowRight.new(variant,payload) end



---@class KeyCode.ArrowUp : EnumVariantRef
--- The `ArrowUp` variant of `KeyCode`.
KeyCode.ArrowUp = {}

---@param variant KeyCode.ArrowUp
---@param payload NoFields
---@return KeyCode
function KeyCode.ArrowUp.new(variant,payload) end



---@class KeyCode.NumLock : EnumVariantRef
--- The `NumLock` variant of `KeyCode`.
KeyCode.NumLock = {}

---@param variant KeyCode.NumLock
---@param payload NoFields
---@return KeyCode
function KeyCode.NumLock.new(variant,payload) end



---@class KeyCode.Numpad0 : EnumVariantRef
--- The `Numpad0` variant of `KeyCode`.
KeyCode.Numpad0 = {}

---@param variant KeyCode.Numpad0
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad0.new(variant,payload) end



---@class KeyCode.Numpad1 : EnumVariantRef
--- The `Numpad1` variant of `KeyCode`.
KeyCode.Numpad1 = {}

---@param variant KeyCode.Numpad1
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad1.new(variant,payload) end



---@class KeyCode.Numpad2 : EnumVariantRef
--- The `Numpad2` variant of `KeyCode`.
KeyCode.Numpad2 = {}

---@param variant KeyCode.Numpad2
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad2.new(variant,payload) end



---@class KeyCode.Numpad3 : EnumVariantRef
--- The `Numpad3` variant of `KeyCode`.
KeyCode.Numpad3 = {}

---@param variant KeyCode.Numpad3
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad3.new(variant,payload) end



---@class KeyCode.Numpad4 : EnumVariantRef
--- The `Numpad4` variant of `KeyCode`.
KeyCode.Numpad4 = {}

---@param variant KeyCode.Numpad4
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad4.new(variant,payload) end



---@class KeyCode.Numpad5 : EnumVariantRef
--- The `Numpad5` variant of `KeyCode`.
KeyCode.Numpad5 = {}

---@param variant KeyCode.Numpad5
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad5.new(variant,payload) end



---@class KeyCode.Numpad6 : EnumVariantRef
--- The `Numpad6` variant of `KeyCode`.
KeyCode.Numpad6 = {}

---@param variant KeyCode.Numpad6
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad6.new(variant,payload) end



---@class KeyCode.Numpad7 : EnumVariantRef
--- The `Numpad7` variant of `KeyCode`.
KeyCode.Numpad7 = {}

---@param variant KeyCode.Numpad7
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad7.new(variant,payload) end



---@class KeyCode.Numpad8 : EnumVariantRef
--- The `Numpad8` variant of `KeyCode`.
KeyCode.Numpad8 = {}

---@param variant KeyCode.Numpad8
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad8.new(variant,payload) end



---@class KeyCode.Numpad9 : EnumVariantRef
--- The `Numpad9` variant of `KeyCode`.
KeyCode.Numpad9 = {}

---@param variant KeyCode.Numpad9
---@param payload NoFields
---@return KeyCode
function KeyCode.Numpad9.new(variant,payload) end



---@class KeyCode.NumpadAdd : EnumVariantRef
--- The `NumpadAdd` variant of `KeyCode`.
KeyCode.NumpadAdd = {}

---@param variant KeyCode.NumpadAdd
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadAdd.new(variant,payload) end



---@class KeyCode.NumpadBackspace : EnumVariantRef
--- The `NumpadBackspace` variant of `KeyCode`.
KeyCode.NumpadBackspace = {}

---@param variant KeyCode.NumpadBackspace
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadBackspace.new(variant,payload) end



---@class KeyCode.NumpadClear : EnumVariantRef
--- The `NumpadClear` variant of `KeyCode`.
KeyCode.NumpadClear = {}

---@param variant KeyCode.NumpadClear
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadClear.new(variant,payload) end



---@class KeyCode.NumpadClearEntry : EnumVariantRef
--- The `NumpadClearEntry` variant of `KeyCode`.
KeyCode.NumpadClearEntry = {}

---@param variant KeyCode.NumpadClearEntry
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadClearEntry.new(variant,payload) end



---@class KeyCode.NumpadComma : EnumVariantRef
--- The `NumpadComma` variant of `KeyCode`.
KeyCode.NumpadComma = {}

---@param variant KeyCode.NumpadComma
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadComma.new(variant,payload) end



---@class KeyCode.NumpadDecimal : EnumVariantRef
--- The `NumpadDecimal` variant of `KeyCode`.
KeyCode.NumpadDecimal = {}

---@param variant KeyCode.NumpadDecimal
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadDecimal.new(variant,payload) end



---@class KeyCode.NumpadDivide : EnumVariantRef
--- The `NumpadDivide` variant of `KeyCode`.
KeyCode.NumpadDivide = {}

---@param variant KeyCode.NumpadDivide
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadDivide.new(variant,payload) end



---@class KeyCode.NumpadEnter : EnumVariantRef
--- The `NumpadEnter` variant of `KeyCode`.
KeyCode.NumpadEnter = {}

---@param variant KeyCode.NumpadEnter
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadEnter.new(variant,payload) end



---@class KeyCode.NumpadEqual : EnumVariantRef
--- The `NumpadEqual` variant of `KeyCode`.
KeyCode.NumpadEqual = {}

---@param variant KeyCode.NumpadEqual
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadEqual.new(variant,payload) end



---@class KeyCode.NumpadHash : EnumVariantRef
--- The `NumpadHash` variant of `KeyCode`.
KeyCode.NumpadHash = {}

---@param variant KeyCode.NumpadHash
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadHash.new(variant,payload) end



---@class KeyCode.NumpadMemoryAdd : EnumVariantRef
--- The `NumpadMemoryAdd` variant of `KeyCode`.
KeyCode.NumpadMemoryAdd = {}

---@param variant KeyCode.NumpadMemoryAdd
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMemoryAdd.new(variant,payload) end



---@class KeyCode.NumpadMemoryClear : EnumVariantRef
--- The `NumpadMemoryClear` variant of `KeyCode`.
KeyCode.NumpadMemoryClear = {}

---@param variant KeyCode.NumpadMemoryClear
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMemoryClear.new(variant,payload) end



---@class KeyCode.NumpadMemoryRecall : EnumVariantRef
--- The `NumpadMemoryRecall` variant of `KeyCode`.
KeyCode.NumpadMemoryRecall = {}

---@param variant KeyCode.NumpadMemoryRecall
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMemoryRecall.new(variant,payload) end



---@class KeyCode.NumpadMemoryStore : EnumVariantRef
--- The `NumpadMemoryStore` variant of `KeyCode`.
KeyCode.NumpadMemoryStore = {}

---@param variant KeyCode.NumpadMemoryStore
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMemoryStore.new(variant,payload) end



---@class KeyCode.NumpadMemorySubtract : EnumVariantRef
--- The `NumpadMemorySubtract` variant of `KeyCode`.
KeyCode.NumpadMemorySubtract = {}

---@param variant KeyCode.NumpadMemorySubtract
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMemorySubtract.new(variant,payload) end



---@class KeyCode.NumpadMultiply : EnumVariantRef
--- The `NumpadMultiply` variant of `KeyCode`.
KeyCode.NumpadMultiply = {}

---@param variant KeyCode.NumpadMultiply
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadMultiply.new(variant,payload) end



---@class KeyCode.NumpadParenLeft : EnumVariantRef
--- The `NumpadParenLeft` variant of `KeyCode`.
KeyCode.NumpadParenLeft = {}

---@param variant KeyCode.NumpadParenLeft
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadParenLeft.new(variant,payload) end



---@class KeyCode.NumpadParenRight : EnumVariantRef
--- The `NumpadParenRight` variant of `KeyCode`.
KeyCode.NumpadParenRight = {}

---@param variant KeyCode.NumpadParenRight
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadParenRight.new(variant,payload) end



---@class KeyCode.NumpadStar : EnumVariantRef
--- The `NumpadStar` variant of `KeyCode`.
KeyCode.NumpadStar = {}

---@param variant KeyCode.NumpadStar
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadStar.new(variant,payload) end



---@class KeyCode.NumpadSubtract : EnumVariantRef
--- The `NumpadSubtract` variant of `KeyCode`.
KeyCode.NumpadSubtract = {}

---@param variant KeyCode.NumpadSubtract
---@param payload NoFields
---@return KeyCode
function KeyCode.NumpadSubtract.new(variant,payload) end



---@class KeyCode.Escape : EnumVariantRef
--- The `Escape` variant of `KeyCode`.
KeyCode.Escape = {}

---@param variant KeyCode.Escape
---@param payload NoFields
---@return KeyCode
function KeyCode.Escape.new(variant,payload) end



---@class KeyCode.Fn : EnumVariantRef
--- The `Fn` variant of `KeyCode`.
KeyCode.Fn = {}

---@param variant KeyCode.Fn
---@param payload NoFields
---@return KeyCode
function KeyCode.Fn.new(variant,payload) end



---@class KeyCode.FnLock : EnumVariantRef
--- The `FnLock` variant of `KeyCode`.
KeyCode.FnLock = {}

---@param variant KeyCode.FnLock
---@param payload NoFields
---@return KeyCode
function KeyCode.FnLock.new(variant,payload) end



---@class KeyCode.PrintScreen : EnumVariantRef
--- The `PrintScreen` variant of `KeyCode`.
KeyCode.PrintScreen = {}

---@param variant KeyCode.PrintScreen
---@param payload NoFields
---@return KeyCode
function KeyCode.PrintScreen.new(variant,payload) end



---@class KeyCode.ScrollLock : EnumVariantRef
--- The `ScrollLock` variant of `KeyCode`.
KeyCode.ScrollLock = {}

---@param variant KeyCode.ScrollLock
---@param payload NoFields
---@return KeyCode
function KeyCode.ScrollLock.new(variant,payload) end



---@class KeyCode.Pause : EnumVariantRef
--- The `Pause` variant of `KeyCode`.
KeyCode.Pause = {}

---@param variant KeyCode.Pause
---@param payload NoFields
---@return KeyCode
function KeyCode.Pause.new(variant,payload) end



---@class KeyCode.BrowserBack : EnumVariantRef
--- The `BrowserBack` variant of `KeyCode`.
KeyCode.BrowserBack = {}

---@param variant KeyCode.BrowserBack
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserBack.new(variant,payload) end



---@class KeyCode.BrowserFavorites : EnumVariantRef
--- The `BrowserFavorites` variant of `KeyCode`.
KeyCode.BrowserFavorites = {}

---@param variant KeyCode.BrowserFavorites
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserFavorites.new(variant,payload) end



---@class KeyCode.BrowserForward : EnumVariantRef
--- The `BrowserForward` variant of `KeyCode`.
KeyCode.BrowserForward = {}

---@param variant KeyCode.BrowserForward
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserForward.new(variant,payload) end



---@class KeyCode.BrowserHome : EnumVariantRef
--- The `BrowserHome` variant of `KeyCode`.
KeyCode.BrowserHome = {}

---@param variant KeyCode.BrowserHome
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserHome.new(variant,payload) end



---@class KeyCode.BrowserRefresh : EnumVariantRef
--- The `BrowserRefresh` variant of `KeyCode`.
KeyCode.BrowserRefresh = {}

---@param variant KeyCode.BrowserRefresh
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserRefresh.new(variant,payload) end



---@class KeyCode.BrowserSearch : EnumVariantRef
--- The `BrowserSearch` variant of `KeyCode`.
KeyCode.BrowserSearch = {}

---@param variant KeyCode.BrowserSearch
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserSearch.new(variant,payload) end



---@class KeyCode.BrowserStop : EnumVariantRef
--- The `BrowserStop` variant of `KeyCode`.
KeyCode.BrowserStop = {}

---@param variant KeyCode.BrowserStop
---@param payload NoFields
---@return KeyCode
function KeyCode.BrowserStop.new(variant,payload) end



---@class KeyCode.Eject : EnumVariantRef
--- The `Eject` variant of `KeyCode`.
KeyCode.Eject = {}

---@param variant KeyCode.Eject
---@param payload NoFields
---@return KeyCode
function KeyCode.Eject.new(variant,payload) end



---@class KeyCode.LaunchApp1 : EnumVariantRef
--- The `LaunchApp1` variant of `KeyCode`.
KeyCode.LaunchApp1 = {}

---@param variant KeyCode.LaunchApp1
---@param payload NoFields
---@return KeyCode
function KeyCode.LaunchApp1.new(variant,payload) end



---@class KeyCode.LaunchApp2 : EnumVariantRef
--- The `LaunchApp2` variant of `KeyCode`.
KeyCode.LaunchApp2 = {}

---@param variant KeyCode.LaunchApp2
---@param payload NoFields
---@return KeyCode
function KeyCode.LaunchApp2.new(variant,payload) end



---@class KeyCode.LaunchMail : EnumVariantRef
--- The `LaunchMail` variant of `KeyCode`.
KeyCode.LaunchMail = {}

---@param variant KeyCode.LaunchMail
---@param payload NoFields
---@return KeyCode
function KeyCode.LaunchMail.new(variant,payload) end



---@class KeyCode.MediaPlayPause : EnumVariantRef
--- The `MediaPlayPause` variant of `KeyCode`.
KeyCode.MediaPlayPause = {}

---@param variant KeyCode.MediaPlayPause
---@param payload NoFields
---@return KeyCode
function KeyCode.MediaPlayPause.new(variant,payload) end



---@class KeyCode.MediaSelect : EnumVariantRef
--- The `MediaSelect` variant of `KeyCode`.
KeyCode.MediaSelect = {}

---@param variant KeyCode.MediaSelect
---@param payload NoFields
---@return KeyCode
function KeyCode.MediaSelect.new(variant,payload) end



---@class KeyCode.MediaStop : EnumVariantRef
--- The `MediaStop` variant of `KeyCode`.
KeyCode.MediaStop = {}

---@param variant KeyCode.MediaStop
---@param payload NoFields
---@return KeyCode
function KeyCode.MediaStop.new(variant,payload) end



---@class KeyCode.MediaTrackNext : EnumVariantRef
--- The `MediaTrackNext` variant of `KeyCode`.
KeyCode.MediaTrackNext = {}

---@param variant KeyCode.MediaTrackNext
---@param payload NoFields
---@return KeyCode
function KeyCode.MediaTrackNext.new(variant,payload) end



---@class KeyCode.MediaTrackPrevious : EnumVariantRef
--- The `MediaTrackPrevious` variant of `KeyCode`.
KeyCode.MediaTrackPrevious = {}

---@param variant KeyCode.MediaTrackPrevious
---@param payload NoFields
---@return KeyCode
function KeyCode.MediaTrackPrevious.new(variant,payload) end



---@class KeyCode.Power : EnumVariantRef
--- The `Power` variant of `KeyCode`.
KeyCode.Power = {}

---@param variant KeyCode.Power
---@param payload NoFields
---@return KeyCode
function KeyCode.Power.new(variant,payload) end



---@class KeyCode.Sleep : EnumVariantRef
--- The `Sleep` variant of `KeyCode`.
KeyCode.Sleep = {}

---@param variant KeyCode.Sleep
---@param payload NoFields
---@return KeyCode
function KeyCode.Sleep.new(variant,payload) end



---@class KeyCode.AudioVolumeDown : EnumVariantRef
--- The `AudioVolumeDown` variant of `KeyCode`.
KeyCode.AudioVolumeDown = {}

---@param variant KeyCode.AudioVolumeDown
---@param payload NoFields
---@return KeyCode
function KeyCode.AudioVolumeDown.new(variant,payload) end



---@class KeyCode.AudioVolumeMute : EnumVariantRef
--- The `AudioVolumeMute` variant of `KeyCode`.
KeyCode.AudioVolumeMute = {}

---@param variant KeyCode.AudioVolumeMute
---@param payload NoFields
---@return KeyCode
function KeyCode.AudioVolumeMute.new(variant,payload) end



---@class KeyCode.AudioVolumeUp : EnumVariantRef
--- The `AudioVolumeUp` variant of `KeyCode`.
KeyCode.AudioVolumeUp = {}

---@param variant KeyCode.AudioVolumeUp
---@param payload NoFields
---@return KeyCode
function KeyCode.AudioVolumeUp.new(variant,payload) end



---@class KeyCode.WakeUp : EnumVariantRef
--- The `WakeUp` variant of `KeyCode`.
KeyCode.WakeUp = {}

---@param variant KeyCode.WakeUp
---@param payload NoFields
---@return KeyCode
function KeyCode.WakeUp.new(variant,payload) end



---@class KeyCode.Meta : EnumVariantRef
--- The `Meta` variant of `KeyCode`.
KeyCode.Meta = {}

---@param variant KeyCode.Meta
---@param payload NoFields
---@return KeyCode
function KeyCode.Meta.new(variant,payload) end



---@class KeyCode.Hyper : EnumVariantRef
--- The `Hyper` variant of `KeyCode`.
KeyCode.Hyper = {}

---@param variant KeyCode.Hyper
---@param payload NoFields
---@return KeyCode
function KeyCode.Hyper.new(variant,payload) end



---@class KeyCode.Turbo : EnumVariantRef
--- The `Turbo` variant of `KeyCode`.
KeyCode.Turbo = {}

---@param variant KeyCode.Turbo
---@param payload NoFields
---@return KeyCode
function KeyCode.Turbo.new(variant,payload) end



---@class KeyCode.Abort : EnumVariantRef
--- The `Abort` variant of `KeyCode`.
KeyCode.Abort = {}

---@param variant KeyCode.Abort
---@param payload NoFields
---@return KeyCode
function KeyCode.Abort.new(variant,payload) end



---@class KeyCode.Resume : EnumVariantRef
--- The `Resume` variant of `KeyCode`.
KeyCode.Resume = {}

---@param variant KeyCode.Resume
---@param payload NoFields
---@return KeyCode
function KeyCode.Resume.new(variant,payload) end



---@class KeyCode.Suspend : EnumVariantRef
--- The `Suspend` variant of `KeyCode`.
KeyCode.Suspend = {}

---@param variant KeyCode.Suspend
---@param payload NoFields
---@return KeyCode
function KeyCode.Suspend.new(variant,payload) end



---@class KeyCode.Again : EnumVariantRef
--- The `Again` variant of `KeyCode`.
KeyCode.Again = {}

---@param variant KeyCode.Again
---@param payload NoFields
---@return KeyCode
function KeyCode.Again.new(variant,payload) end



---@class KeyCode.Copy : EnumVariantRef
--- The `Copy` variant of `KeyCode`.
KeyCode.Copy = {}

---@param variant KeyCode.Copy
---@param payload NoFields
---@return KeyCode
function KeyCode.Copy.new(variant,payload) end



---@class KeyCode.Cut : EnumVariantRef
--- The `Cut` variant of `KeyCode`.
KeyCode.Cut = {}

---@param variant KeyCode.Cut
---@param payload NoFields
---@return KeyCode
function KeyCode.Cut.new(variant,payload) end



---@class KeyCode.Find : EnumVariantRef
--- The `Find` variant of `KeyCode`.
KeyCode.Find = {}

---@param variant KeyCode.Find
---@param payload NoFields
---@return KeyCode
function KeyCode.Find.new(variant,payload) end



---@class KeyCode.Open : EnumVariantRef
--- The `Open` variant of `KeyCode`.
KeyCode.Open = {}

---@param variant KeyCode.Open
---@param payload NoFields
---@return KeyCode
function KeyCode.Open.new(variant,payload) end



---@class KeyCode.Paste : EnumVariantRef
--- The `Paste` variant of `KeyCode`.
KeyCode.Paste = {}

---@param variant KeyCode.Paste
---@param payload NoFields
---@return KeyCode
function KeyCode.Paste.new(variant,payload) end



---@class KeyCode.Props : EnumVariantRef
--- The `Props` variant of `KeyCode`.
KeyCode.Props = {}

---@param variant KeyCode.Props
---@param payload NoFields
---@return KeyCode
function KeyCode.Props.new(variant,payload) end



---@class KeyCode.Select : EnumVariantRef
--- The `Select` variant of `KeyCode`.
KeyCode.Select = {}

---@param variant KeyCode.Select
---@param payload NoFields
---@return KeyCode
function KeyCode.Select.new(variant,payload) end



---@class KeyCode.Undo : EnumVariantRef
--- The `Undo` variant of `KeyCode`.
KeyCode.Undo = {}

---@param variant KeyCode.Undo
---@param payload NoFields
---@return KeyCode
function KeyCode.Undo.new(variant,payload) end



---@class KeyCode.Hiragana : EnumVariantRef
--- The `Hiragana` variant of `KeyCode`.
KeyCode.Hiragana = {}

---@param variant KeyCode.Hiragana
---@param payload NoFields
---@return KeyCode
function KeyCode.Hiragana.new(variant,payload) end



---@class KeyCode.Katakana : EnumVariantRef
--- The `Katakana` variant of `KeyCode`.
KeyCode.Katakana = {}

---@param variant KeyCode.Katakana
---@param payload NoFields
---@return KeyCode
function KeyCode.Katakana.new(variant,payload) end



---@class KeyCode.F1 : EnumVariantRef
--- The `F1` variant of `KeyCode`.
KeyCode.F1 = {}

---@param variant KeyCode.F1
---@param payload NoFields
---@return KeyCode
function KeyCode.F1.new(variant,payload) end



---@class KeyCode.F2 : EnumVariantRef
--- The `F2` variant of `KeyCode`.
KeyCode.F2 = {}

---@param variant KeyCode.F2
---@param payload NoFields
---@return KeyCode
function KeyCode.F2.new(variant,payload) end



---@class KeyCode.F3 : EnumVariantRef
--- The `F3` variant of `KeyCode`.
KeyCode.F3 = {}

---@param variant KeyCode.F3
---@param payload NoFields
---@return KeyCode
function KeyCode.F3.new(variant,payload) end



---@class KeyCode.F4 : EnumVariantRef
--- The `F4` variant of `KeyCode`.
KeyCode.F4 = {}

---@param variant KeyCode.F4
---@param payload NoFields
---@return KeyCode
function KeyCode.F4.new(variant,payload) end



---@class KeyCode.F5 : EnumVariantRef
--- The `F5` variant of `KeyCode`.
KeyCode.F5 = {}

---@param variant KeyCode.F5
---@param payload NoFields
---@return KeyCode
function KeyCode.F5.new(variant,payload) end



---@class KeyCode.F6 : EnumVariantRef
--- The `F6` variant of `KeyCode`.
KeyCode.F6 = {}

---@param variant KeyCode.F6
---@param payload NoFields
---@return KeyCode
function KeyCode.F6.new(variant,payload) end



---@class KeyCode.F7 : EnumVariantRef
--- The `F7` variant of `KeyCode`.
KeyCode.F7 = {}

---@param variant KeyCode.F7
---@param payload NoFields
---@return KeyCode
function KeyCode.F7.new(variant,payload) end



---@class KeyCode.F8 : EnumVariantRef
--- The `F8` variant of `KeyCode`.
KeyCode.F8 = {}

---@param variant KeyCode.F8
---@param payload NoFields
---@return KeyCode
function KeyCode.F8.new(variant,payload) end



---@class KeyCode.F9 : EnumVariantRef
--- The `F9` variant of `KeyCode`.
KeyCode.F9 = {}

---@param variant KeyCode.F9
---@param payload NoFields
---@return KeyCode
function KeyCode.F9.new(variant,payload) end



---@class KeyCode.F10 : EnumVariantRef
--- The `F10` variant of `KeyCode`.
KeyCode.F10 = {}

---@param variant KeyCode.F10
---@param payload NoFields
---@return KeyCode
function KeyCode.F10.new(variant,payload) end



---@class KeyCode.F11 : EnumVariantRef
--- The `F11` variant of `KeyCode`.
KeyCode.F11 = {}

---@param variant KeyCode.F11
---@param payload NoFields
---@return KeyCode
function KeyCode.F11.new(variant,payload) end



---@class KeyCode.F12 : EnumVariantRef
--- The `F12` variant of `KeyCode`.
KeyCode.F12 = {}

---@param variant KeyCode.F12
---@param payload NoFields
---@return KeyCode
function KeyCode.F12.new(variant,payload) end



---@class KeyCode.F13 : EnumVariantRef
--- The `F13` variant of `KeyCode`.
KeyCode.F13 = {}

---@param variant KeyCode.F13
---@param payload NoFields
---@return KeyCode
function KeyCode.F13.new(variant,payload) end



---@class KeyCode.F14 : EnumVariantRef
--- The `F14` variant of `KeyCode`.
KeyCode.F14 = {}

---@param variant KeyCode.F14
---@param payload NoFields
---@return KeyCode
function KeyCode.F14.new(variant,payload) end



---@class KeyCode.F15 : EnumVariantRef
--- The `F15` variant of `KeyCode`.
KeyCode.F15 = {}

---@param variant KeyCode.F15
---@param payload NoFields
---@return KeyCode
function KeyCode.F15.new(variant,payload) end



---@class KeyCode.F16 : EnumVariantRef
--- The `F16` variant of `KeyCode`.
KeyCode.F16 = {}

---@param variant KeyCode.F16
---@param payload NoFields
---@return KeyCode
function KeyCode.F16.new(variant,payload) end



---@class KeyCode.F17 : EnumVariantRef
--- The `F17` variant of `KeyCode`.
KeyCode.F17 = {}

---@param variant KeyCode.F17
---@param payload NoFields
---@return KeyCode
function KeyCode.F17.new(variant,payload) end



---@class KeyCode.F18 : EnumVariantRef
--- The `F18` variant of `KeyCode`.
KeyCode.F18 = {}

---@param variant KeyCode.F18
---@param payload NoFields
---@return KeyCode
function KeyCode.F18.new(variant,payload) end



---@class KeyCode.F19 : EnumVariantRef
--- The `F19` variant of `KeyCode`.
KeyCode.F19 = {}

---@param variant KeyCode.F19
---@param payload NoFields
---@return KeyCode
function KeyCode.F19.new(variant,payload) end



---@class KeyCode.F20 : EnumVariantRef
--- The `F20` variant of `KeyCode`.
KeyCode.F20 = {}

---@param variant KeyCode.F20
---@param payload NoFields
---@return KeyCode
function KeyCode.F20.new(variant,payload) end



---@class KeyCode.F21 : EnumVariantRef
--- The `F21` variant of `KeyCode`.
KeyCode.F21 = {}

---@param variant KeyCode.F21
---@param payload NoFields
---@return KeyCode
function KeyCode.F21.new(variant,payload) end



---@class KeyCode.F22 : EnumVariantRef
--- The `F22` variant of `KeyCode`.
KeyCode.F22 = {}

---@param variant KeyCode.F22
---@param payload NoFields
---@return KeyCode
function KeyCode.F22.new(variant,payload) end



---@class KeyCode.F23 : EnumVariantRef
--- The `F23` variant of `KeyCode`.
KeyCode.F23 = {}

---@param variant KeyCode.F23
---@param payload NoFields
---@return KeyCode
function KeyCode.F23.new(variant,payload) end



---@class KeyCode.F24 : EnumVariantRef
--- The `F24` variant of `KeyCode`.
KeyCode.F24 = {}

---@param variant KeyCode.F24
---@param payload NoFields
---@return KeyCode
function KeyCode.F24.new(variant,payload) end



---@class KeyCode.F25 : EnumVariantRef
--- The `F25` variant of `KeyCode`.
KeyCode.F25 = {}

---@param variant KeyCode.F25
---@param payload NoFields
---@return KeyCode
function KeyCode.F25.new(variant,payload) end



---@class KeyCode.F26 : EnumVariantRef
--- The `F26` variant of `KeyCode`.
KeyCode.F26 = {}

---@param variant KeyCode.F26
---@param payload NoFields
---@return KeyCode
function KeyCode.F26.new(variant,payload) end



---@class KeyCode.F27 : EnumVariantRef
--- The `F27` variant of `KeyCode`.
KeyCode.F27 = {}

---@param variant KeyCode.F27
---@param payload NoFields
---@return KeyCode
function KeyCode.F27.new(variant,payload) end



---@class KeyCode.F28 : EnumVariantRef
--- The `F28` variant of `KeyCode`.
KeyCode.F28 = {}

---@param variant KeyCode.F28
---@param payload NoFields
---@return KeyCode
function KeyCode.F28.new(variant,payload) end



---@class KeyCode.F29 : EnumVariantRef
--- The `F29` variant of `KeyCode`.
KeyCode.F29 = {}

---@param variant KeyCode.F29
---@param payload NoFields
---@return KeyCode
function KeyCode.F29.new(variant,payload) end



---@class KeyCode.F30 : EnumVariantRef
--- The `F30` variant of `KeyCode`.
KeyCode.F30 = {}

---@param variant KeyCode.F30
---@param payload NoFields
---@return KeyCode
function KeyCode.F30.new(variant,payload) end



---@class KeyCode.F31 : EnumVariantRef
--- The `F31` variant of `KeyCode`.
KeyCode.F31 = {}

---@param variant KeyCode.F31
---@param payload NoFields
---@return KeyCode
function KeyCode.F31.new(variant,payload) end



---@class KeyCode.F32 : EnumVariantRef
--- The `F32` variant of `KeyCode`.
KeyCode.F32 = {}

---@param variant KeyCode.F32
---@param payload NoFields
---@return KeyCode
function KeyCode.F32.new(variant,payload) end



---@class KeyCode.F33 : EnumVariantRef
--- The `F33` variant of `KeyCode`.
KeyCode.F33 = {}

---@param variant KeyCode.F33
---@param payload NoFields
---@return KeyCode
function KeyCode.F33.new(variant,payload) end



---@class KeyCode.F34 : EnumVariantRef
--- The `F34` variant of `KeyCode`.
KeyCode.F34 = {}

---@param variant KeyCode.F34
---@param payload NoFields
---@return KeyCode
function KeyCode.F34.new(variant,payload) end



---@class KeyCode.F35 : EnumVariantRef
--- The `F35` variant of `KeyCode`.
KeyCode.F35 = {}

---@param variant KeyCode.F35
---@param payload NoFields
---@return KeyCode
function KeyCode.F35.new(variant,payload) end



---@class MouseButton : ReflectReference
---  A button on a mouse device.
--- 
---  ## Usage
--- 
---  It is used as the generic `T` value of an [`ButtonInput`] to create a `bevy`
---  resource.
--- 
---  ## Updating
--- 
---  The resource is updated inside of the [`mouse_button_input_system`].
---@field  Left MouseButton.Left
---@field  Right MouseButton.Right
---@field  Middle MouseButton.Middle
---@field  Back MouseButton.Back
---@field  Forward MouseButton.Forward
---@field  Other MouseButton.Other
local MouseButton = {}

---@return MouseButton[]
function MouseButton.get_just_pressed() end

---@return MouseButton[]
function MouseButton.get_pressed() end

---@param button MouseButton
---@return boolean
function MouseButton.pressed(button) end

---@param button MouseButton
---@return boolean
function MouseButton.just_released(button) end

---@param buttons MouseButton[]
---@return boolean
function MouseButton.any_pressed(buttons) end

---@param buttons MouseButton[]
---@return boolean
function MouseButton.any_just_pressed(buttons) end

---@param button MouseButton
---@return boolean
function MouseButton.just_pressed(button) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MouseButton
function MouseButton.new(payload) end



---@class MouseButton.Left : EnumVariantRef
--- The `Left` variant of `MouseButton`.
MouseButton.Left = {}

---@param variant MouseButton.Left
---@param payload NoFields
---@return MouseButton
function MouseButton.Left.new(variant,payload) end



---@class MouseButton.Right : EnumVariantRef
--- The `Right` variant of `MouseButton`.
MouseButton.Right = {}

---@param variant MouseButton.Right
---@param payload NoFields
---@return MouseButton
function MouseButton.Right.new(variant,payload) end



---@class MouseButton.Middle : EnumVariantRef
--- The `Middle` variant of `MouseButton`.
MouseButton.Middle = {}

---@param variant MouseButton.Middle
---@param payload NoFields
---@return MouseButton
function MouseButton.Middle.new(variant,payload) end



---@class MouseButton.Back : EnumVariantRef
--- The `Back` variant of `MouseButton`.
MouseButton.Back = {}

---@param variant MouseButton.Back
---@param payload NoFields
---@return MouseButton
function MouseButton.Back.new(variant,payload) end



---@class MouseButton.Forward : EnumVariantRef
--- The `Forward` variant of `MouseButton`.
MouseButton.Forward = {}

---@param variant MouseButton.Forward
---@param payload NoFields
---@return MouseButton
function MouseButton.Forward.new(variant,payload) end



---@class MouseButton.Other : EnumVariantRef
--- The `Other` variant of `MouseButton`.
MouseButton.Other = {}

---@param variant MouseButton.Other
---@param payload MouseButton.Other.Fields
---@return MouseButton
function MouseButton.Other.new(variant,payload) end



---@class InputFocus : ReflectReference
---  Resource representing which entity has input focus, if any. Input events (other than pointer-like inputs) will be
---  dispatched to the current focus entity, or to the primary window if no entity has focus.
--- 
---  Changing the input focus is as easy as modifying this resource.
--- 
---  To detect when an entity gains or loses focus, listen for the [`FocusGained`] and [`FocusLost`] events.
--- 
---  # Examples
--- 
---  From within a system:
--- 
---  ```rust
---  use bevy_ecs::prelude::*;
---  use bevy_input_focus::{FocusCause, InputFocus};
--- 
---  fn clear_focus(mut input_focus: ResMut<InputFocus>) {
---    input_focus.clear();
---  }
---  ```
--- 
---  With exclusive (or deferred) world access:
--- 
---  ```rust
---  use bevy_ecs::prelude::*;
---  use bevy_input_focus::{FocusCause, InputFocus};
--- 
---  fn set_focus_from_world(world: &mut World) {
---      let entity = world.spawn_empty().id();
--- 
---      // Fetch the resource from the world
---      let mut input_focus = world.resource_mut::<InputFocus>();
---      // Then mutate it!
---      input_focus.set(entity, FocusCause::Navigated);
--- 
---      // Or you can just insert a fresh copy of the resource
---      // which will overwrite the existing one.
---      world.insert_resource(InputFocus::from_entity(entity));
---  }
---  ```
---@field  current_focus? Entity | nil
---@field  recorded_changes? [Entity, FocusCause] | nil[]
---@field  original_focus? Entity | nil
local InputFocus = {}

---@param _self InputFocus
---@return nil
function InputFocus.clear(_self) end

---@param _self InputFocus
---@param entity Entity
---@return nil
function InputFocus.set(_self,entity) end

---@param _self InputFocus
---@return Entity | nil
function InputFocus.get(_self) end

---@param payload InputFocus.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InputFocus
function InputFocus.new(payload) end



---@class Dir2 : ReflectReference
---  A normalized vector pointing in a direction in 2D space
---@field  [1] Vec2
---@operator unm: Dir2
---@operator mul(number): Vec2
local Dir2 = {}

---@return Dir2
function Dir2.x() end

---@param v Vec2
---@return Dir2 | nil
function Dir2.new(v) end

---@param _self Dir2
---@param other Dir2
---@param t number
---@return Dir2
function Dir2.slerp(_self,other,t) end

---@return Dir2
function Dir2.y() end

---@param _self Dir2
---@return Dir2
function Dir2.neg(_self) end

---@param _self Dir2
---@return Vec2
function Dir2.as_vec2(_self) end

---@param _self Dir2
---@param length number
---@return Vec2
function Dir2.mul(_self,length) end

---@return Dir2
function Dir2.neg_y() end

---@return Dir2
function Dir2.neg_x() end

---@param _self Dir2
---@param other Dir2
---@return boolean
function Dir2.__eq(_self,other) end

---@param _self Dir2
---@param other Dir2
---@return boolean
function Dir2.eq(_self,other) end



---@class Dir3 : ReflectReference
---  A normalized vector pointing in a direction in 3D space
---@field  [1] Vec3
---@operator unm: Dir3
---@operator mul(number): Vec3
local Dir3 = {}

---@param _self Dir3
---@param other Dir3
---@param t number
---@return Dir3
function Dir3.slerp(_self,other,t) end

---@return Dir3
function Dir3.neg_y() end

---@param _self Dir3
---@return Dir3
function Dir3.neg(_self) end

---@param _self Dir3
---@param length number
---@return Vec3
function Dir3.mul(_self,length) end

---@return Dir3
function Dir3.x() end

---@param _self Dir3
---@return Vec3
function Dir3.as_vec3(_self) end

---@param _self Dir3
---@param other Dir3
---@return boolean
function Dir3.__eq(_self,other) end

---@param _self Dir3
---@param other Dir3
---@return boolean
function Dir3.eq(_self,other) end

---@return Dir3
function Dir3.z() end

---@return Dir3
function Dir3.neg_x() end

---@return Dir3
function Dir3.y() end

---@return Dir3
function Dir3.neg_z() end

---@param v Vec3
---@return Dir3 | nil
function Dir3.new(v) end



---@class Rect : ReflectReference
---  A rectangle defined by two opposite corners.
--- 
---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
---  stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant
---  must be upheld by the user when directly assigning the fields, otherwise some methods
---  produce invalid results. It is generally recommended to use one of the constructor
---  methods instead, which will ensure this invariant is met, unless you already have
---  the minimum and maximum corners.
---@field  min? Vec2
---@field  max? Vec2
local Rect = {}

---@param _self Rect
---@return number
function Rect.width(_self) end

---@param p0 Vec2
---@param p1 Vec2
---@return Rect
function Rect.from_corners(p0,p1) end

---@param _self Rect
---@return Vec2
function Rect.size(_self) end

---@param _self Rect
---@param other Rect
---@return Rect
function Rect.union(_self,other) end

---@param _self Rect
---@return Vec2
function Rect.center(_self) end

---@param _self Rect
---@param other Rect
---@return Rect
function Rect.intersect(_self,other) end

---@param _self Rect
---@param point Vec2
---@return boolean
function Rect.contains(_self,point) end

---@param _self Rect
---@param expansion number
---@return Rect
function Rect.inflate(_self,expansion) end

---@param _self Rect
---@return boolean
function Rect.is_empty(_self) end

---@param _self Rect
---@return Vec2
function Rect.half_size(_self) end

---@param x0 number
---@param y0 number
---@param x1 number
---@param y1 number
---@return Rect
function Rect.new(x0,y0,x1,y1) end

---@param origin Vec2
---@param size Vec2
---@return Rect
function Rect.from_center_size(origin,size) end

---@param _self Rect
---@return number
function Rect.height(_self) end

---@param origin Vec2
---@param half_size Vec2
---@return Rect
function Rect.from_center_half_size(origin,half_size) end

---@param _self Rect
---@param other Rect
---@return boolean
function Rect.__eq(_self,other) end

---@param _self Rect
---@param other Rect
---@return boolean
function Rect.eq(_self,other) end



---@class Rot2 : ReflectReference
---  A 2D rotation.
--- 
---  # Example
--- 
---  ```
---  # use approx::assert_relative_eq;
---  # use bevy_math::{Rot2, Vec2};
---  use std::f32::consts::PI;
--- 
---  // Create rotations from counterclockwise angles in radians or degrees
---  let rotation1 = Rot2::radians(PI / 2.0);
---  let rotation2 = Rot2::degrees(45.0);
--- 
---  // Get the angle back as radians or degrees
---  assert_eq!(rotation1.as_degrees(), 90.0);
---  assert_eq!(rotation2.as_radians(), PI / 4.0);
--- 
---  // "Add" rotations together using `*`
---  #[cfg(feature = "approx")]
---  assert_relative_eq!(rotation1 * rotation2, Rot2::degrees(135.0));
--- 
---  // Rotate vectors
---  #[cfg(feature = "approx")]
---  assert_relative_eq!(rotation1 * Vec2::X, Vec2::Y);
---  ```
---@field  cos? number
---@field  sin? number
---@operator mul(Rot2 | Vec2): Rot2 | Vec2
local Rot2 = {}

---@param _self Rot2
---@param other Rot2
---@param t number
---@return Rot2
function Rot2.nlerp(_self,other,t) end

---@return Rot2
function Rot2.identity() end

---@param _self Rot2
---@param other Rot2
---@param t number
---@return Rot2
function Rot2.slerp(_self,other,t) end

---@param degrees number
---@return Rot2
function Rot2.degrees(degrees) end

---@param radians number
---@return Rot2
function Rot2.radians(radians) end

---@param _self Rot2
---@param rhs Rot2 | Vec2
---@return Rot2 | Vec2
function Rot2.mul(_self,rhs) end

---@param _self Rot2
---@return number
function Rot2.as_degrees(_self) end

---@param _self Rot2
---@return Rot2
function Rot2.inverse(_self) end

---@param fraction number
---@return Rot2
function Rot2.turn_fraction(fraction) end

---@param _self Rot2
---@return number
function Rot2.as_radians(_self) end

---@param _self Rot2
---@param other Rot2
---@return boolean
function Rot2.__eq(_self,other) end

---@param _self Rot2
---@param other Rot2
---@return boolean
function Rot2.eq(_self,other) end

---@param payload Rot2.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Rot2
function Rot2.new(payload) end



---@class Mesh : ReflectReference
---  A 3D object made out of vertices representing triangles, lines, or points,
---  with "attribute" values for each vertex.
--- 
---  Meshes can be automatically generated by a bevy `AssetLoader` (generally by loading a `Gltf` file),
---  or by converting a [primitive](bevy_shape) using [`into`](Into).
---  It is also possible to create one manually. They can be edited after creation.
--- 
---  Meshes can be rendered with a [`Mesh2d`](crate::Mesh2d) and `MeshMaterial2d`
---  or [`Mesh3d`](crate::Mesh3d) and `MeshMaterial3d` for 2D and 3D respectively.
--- 
---  A [`Mesh`] in Bevy is equivalent to a "primitive" in the glTF format, for a
---  glTF Mesh representation, see `GltfMesh`.
--- 
---  ## Manual creation
--- 
---  The following function will construct a flat mesh, to be rendered with a
---  `StandardMaterial` or `ColorMaterial`:
--- 
---  ```
---  # use bevy_mesh::{Mesh, Indices, PrimitiveTopology};
---  # use bevy_asset::RenderAssetUsages;
---  fn create_simple_parallelogram() -> Mesh {
---      // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
---      Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
---          // Add 4 vertices, each with its own position attribute (coordinate in
---          // 3D space), for each of the corners of the parallelogram.
---          .with_inserted_attribute(
---              Mesh::ATTRIBUTE_POSITION,
---              vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]
---          )
---          // Assign a UV coordinate to each vertex.
---          .with_inserted_attribute(
---              Mesh::ATTRIBUTE_UV_0,
---              vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]
---          )
---          // Assign normals (everything points outwards)
---          .with_inserted_attribute(
---              Mesh::ATTRIBUTE_NORMAL,
---              vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
---          )
---          // After defining all the vertices and their attributes, build each triangle using the
---          // indices of the vertices that make it up in a counter-clockwise order.
---          .with_inserted_indices(Indices::U32(vec![
---              // First triangle
---              0, 3, 1,
---              // Second triangle
---              1, 3, 2
---          ]))
---  }
---  ```
--- 
---  You can see how it looks like [here](https://github.com/bevyengine/bevy/blob/main/assets/docs/Mesh.png),
---  used in a [`Mesh3d`](crate::Mesh3d) with a square bevy logo texture, with added axis, points,
---  lines and text for clarity.
--- 
---  ## Other examples
--- 
---  For further visualization, explanation, and examples, see the built-in Bevy examples,
---  and the [implementation of the built-in shapes](https://github.com/bevyengine/bevy/tree/main/crates/bevy_mesh/src/primitives).
---  In particular, [generate_custom_mesh](https://github.com/bevyengine/bevy/blob/main/examples/3d/generate_custom_mesh.rs)
---  teaches you to access and modify the attributes of a [`Mesh`] after creating it.
--- 
---  ## Common points of confusion
--- 
---  - UV maps in Bevy start at the top-left, see [`ATTRIBUTE_UV_0`](Mesh::ATTRIBUTE_UV_0),
---    other APIs can have other conventions, `OpenGL` starts at bottom-left.
---  - It is possible and sometimes useful for multiple vertices to have the same
---    [position attribute](Mesh::ATTRIBUTE_POSITION) value,
---    it's a common technique in 3D modeling for complex UV mapping or other calculations.
---  - Bevy performs frustum culling based on the `Aabb` of meshes, which is calculated
---    and added automatically for new meshes only. If a mesh is modified, the entity's `Aabb`
---    needs to be updated manually or deleted so that it is re-calculated.
--- 
---  ## Use with `StandardMaterial`
--- 
---  To render correctly with `StandardMaterial`, a mesh needs to have properly defined:
---  - [`UVs`](Mesh::ATTRIBUTE_UV_0): Bevy needs to know how to map a texture onto the mesh
---    (also true for `ColorMaterial`).
---  - [`Normals`](Mesh::ATTRIBUTE_NORMAL): Bevy needs to know how light interacts with your mesh.
---    [0.0, 0.0, 1.0] is very common for simple flat meshes on the XY plane,
---    because simple meshes are smooth and they don't require complex light calculations.
---  - Vertex winding order: by default, `StandardMaterial.cull_mode` is `Some(Face::Back)`,
---    which means that Bevy would *only* render the "front" of each triangle, which
---    is the side of the triangle from where the vertices appear in a *counter-clockwise* order.
--- 
---  ## Remote Inspection
--- 
---  To transmit a [`Mesh`] between two running Bevy apps, e.g. through BRP, use [`SerializedMesh`].
---  This type is only meant for short-term transmission between same versions and should not be stored anywhere.
---@field  indices? any
---@field  morph_targets? any
---@field  morph_target_names? any
---@field  asset_usage? RenderAssetUsages
---@field  enable_raytracing? boolean
---@field  attribute_compression? MeshAttributeCompressionFlags
---@field  final_aabb? Aabb3d | nil
---@field  final_uv_ranges? Aabb2d | nil[]
---@field  skinned_mesh_bounds? SkinnedMeshBounds | nil
local Mesh = {}

---@param shape ReflectReference
---@return Mesh
function Mesh.from_shape(shape) end

---@param _self Mesh
---@return integer
function Mesh.count_vertices(_self) end

---@param payload Mesh.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Mesh
function Mesh.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<Mesh>
function Mesh.load(path) end

---@param value Mesh
--- The asset value.
---@return Handle<Mesh>
function Mesh.add_asset(value) end

---@param handle Handle<Mesh>
--- The handle to the asset.
---@return Mesh | nil
function Mesh.get_asset(handle) end



---@class Gizmos : ReflectReference
---  Immediate mode debug drawing: lines and shapes drawn this frame.
--- 
---  Every function draws straight away, the shapes are shown for the current frame only.
---  Needs the gizmo plugin, without it the functions fail with an error.
--- 
---  ```multi-lang-example,lua,ignore
---  local Color = import_namespace("color").Color
---  local Gizmos = import_namespace("gizmos").Gizmos
---  local Vec3 = import_namespace("math").Vec3
---  Gizmos.line(Vec3.zero(), Vec3.new(1, 0, 0), Color.white())
---  ```
--- 
---  ```multi-lang-example,rhai,ignore
---  let Color = import_namespace.call("color").Color;
---  let Gizmos = import_namespace.call("gizmos").Gizmos;
---  let Vec3 = import_namespace.call("math").Vec3;
---  Gizmos.line.call(Vec3.zero.call(), Vec3.new_.call(1.0, 0.0, 0.0), Color.white.call());
---  ```
local Gizmos = {}

---@param transform Transform
---@param length number
---@return nil
function Gizmos.axes(transform,length) end

---@param position Vec2
---@param radius number
---@param color Color
---@return nil
function Gizmos.circle_2d(position,radius,color) end

---@param points Vec3[]
---@param color Color
---@return nil
function Gizmos.linestrip(points,color) end

---@param start Vec2
---@param _end Vec2
---@param color Color
---@return nil
function Gizmos.line_2d(start,_end,color) end

---@param transform Transform
---@param color Color
---@return nil
function Gizmos.cuboid(transform,color) end

---@param origin Vec3
---@param vector Vec3
---@param color Color
---@return nil
function Gizmos.ray(origin,vector,color) end

---@param position Vec3
---@param radius number
---@param color Color
---@return nil
function Gizmos.sphere(position,radius,color) end

---@param position Vec3
---@param size Vec2
---@param color Color
---@return nil
function Gizmos.rect(position,size,color) end

---@param origin Vec2
---@param vector Vec2
---@param color Color
---@return nil
function Gizmos.ray_2d(origin,vector,color) end

---@param position Vec3
---@param half_size number
---@param color Color
---@return nil
function Gizmos.cross(position,half_size,color) end

---@param start Vec3
---@param _end Vec3
---@param color Color
---@return nil
function Gizmos.line(start,_end,color) end

---@param start Vec2
---@param _end Vec2
---@param color Color
---@return nil
function Gizmos.arrow_2d(start,_end,color) end

---@param start Vec3
---@param _end Vec3
---@param color Color
---@return nil
function Gizmos.arrow(start,_end,color) end

---@param position Vec2
---@param size Vec2
---@param color Color
---@return nil
function Gizmos.rect_2d(position,size,color) end

---@param points Vec2[]
---@param color Color
---@return nil
function Gizmos.linestrip_2d(points,color) end

---@param position Vec3
---@param radius number
---@param color Color
---@return nil
function Gizmos.circle(position,radius,color) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Gizmos
function Gizmos.new(payload) end



---@class EnumVariantRef : ReflectReference
---  A reference to a variant of an enum, independent of any value of that enum.
--- 
---  This is what scripts see when accessing `MyEnum.Variant`, it can be compared to
---  the result of `value:variant()` and used to construct values of the variant.
---@field  type_id? TypeId
---@field  variant_index? integer
local EnumVariantRef = {}

---@param variant EnumVariantRef
--- The variant.
---@param other any
--- The variant or value to compare with.
---@return boolean
function EnumVariantRef.__eq(variant,other) end

---@param variant EnumVariantRef
--- The variant.
---@param other any
--- The variant or value to compare with.
---@return boolean
function EnumVariantRef.eq(variant,other) end

---@param variant EnumVariantRef
--- The variant to construct.
---@param payload table<string, any>
--- The fields of the variant, keyed by name or index, or a list of the fields in order.
---@return ReflectReference
function EnumVariantRef.new(variant,payload) end

---@param variant EnumVariantRef
--- The variant.
---@return string
function EnumVariantRef.variant_name(variant) end



---@class ScriptBundleRegistration : ReflectReference
---  A reference to a bundle type's reflection registration, i.e. a type with `ReflectBundle` type data.
--- 
---  In general think of this as a handle to a type.
local ScriptBundleRegistration = {}

---@param registration ScriptBundleRegistration
--- The type registration.
---@return string
function ScriptBundleRegistration.type_name(registration) end

---@param registration ScriptBundleRegistration
--- The type registration.
---@return string
function ScriptBundleRegistration.short_name(registration) end

---@param registration ScriptBundleRegistration
--- The type registration.
---@return string
function ScriptBundleRegistration.namespace(registration) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptBundleRegistration
function ScriptBundleRegistration.new(payload) end



---@class ScriptComponentRegistration : ReflectReference
---  A reference to a component type's reflection registration.
--- 
---  In general think of this as a handle to a type.
--- 
---  Not to be confused with script registered dynamic components, although this can point to a script registered component.
---@field  registration? ScriptTypeRegistration
---@field  component_id? ComponentId
---@field  is_dynamic_script_component? boolean
---@field  script_namespace? string | nil
local ScriptComponentRegistration = {}

---@param registration ScriptComponentRegistration
--- The type registration.
---@return string
function ScriptComponentRegistration.type_name(registration) end

---@param registration ScriptComponentRegistration
--- The type registration.
---@return string
function ScriptComponentRegistration.short_name(registration) end

---@param registration ScriptComponentRegistration
--- The type registration.
---@return string
function ScriptComponentRegistration.namespace(registration) end

---@param payload ScriptComponentRegistration.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptComponentRegistration
function ScriptComponentRegistration.new(payload) end



---@class ScriptQueryBuilder : ReflectReference
---  The query builder is used to build ECS queries which retrieve spefific components filtered by specific conditions.
--- 
---  For example:
---  ```rust,ignore
---  builder.component(componentA)
---      .component(componentB)
---      .with(componentC)
---      .without(componentD)  
---  ```
--- 
---  Will retrieve entities which:
---  - Have componentA
---  - Have componentB
---  - Have componentC
---  - Do not have componentD
--- 
---  As well as references to components:
---  - componentA
---  - componentB
local ScriptQueryBuilder = {}

---@param query ScriptQueryBuilder
--- The query to add the component to
---@param components ScriptComponentRegistration
---@return ScriptQueryBuilder
function ScriptQueryBuilder.component(query,components) end

---@param query ScriptQueryBuilder
--- The query to build.
---@return ScriptQueryResult[]
function ScriptQueryBuilder.build(query) end

---@param query ScriptQueryBuilder
--- The query to add the component to
---@param with ScriptComponentRegistration
---@return ScriptQueryBuilder
function ScriptQueryBuilder.with(query,with) end

---@param query ScriptQueryBuilder
--- The query to add the component to
---@param without ScriptComponentRegistration
---@return ScriptQueryBuilder
function ScriptQueryBuilder.without(query,without) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptQueryBuilder
function ScriptQueryBuilder.new(payload) end



---@class ScriptQueryResult : ReflectReference
---  A result from a query.
local ScriptQueryResult = {}

---@param query ScriptQueryResult
--- The query result to retrieve the entity from.
---@return Entity
function ScriptQueryResult.entity(query) end

---@param query ScriptQueryResult
--- The query result to retrieve the components from.
---@return ReflectReference[]
function ScriptQueryResult.components(query) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptQueryResult
function ScriptQueryResult.new(payload) end



---@class ScriptResourceRegistration : ReflectReference
---  A reference to a resource type's reflection registration.
--- 
---  In general think of this as a handle to a type.
---@field  registration? ScriptTypeRegistration
---@field  resource_id? ComponentId
local ScriptResourceRegistration = {}

---@param registration ScriptResourceRegistration
--- The type registration.
---@return string
function ScriptResourceRegistration.namespace(registration) end

---@param registration ScriptResourceRegistration
--- The type registration.
---@return string
function ScriptResourceRegistration.short_name(registration) end

---@param registration ScriptResourceRegistration
--- The type registration.
---@return string
function ScriptResourceRegistration.type_name(registration) end

---@param payload ScriptResourceRegistration.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptResourceRegistration
function ScriptResourceRegistration.new(payload) end



---@class ScriptTypeRegistration : ReflectReference
---  A reference to a type which is not a `Resource` or `Component`.
--- 
---  In general think of this as a handle to a type.
local ScriptTypeRegistration = {}

---@param registration ScriptTypeRegistration
--- The type registration.
---@return string
function ScriptTypeRegistration.namespace(registration) end

---@param registration ScriptTypeRegistration
--- The type registration.
---@return string
function ScriptTypeRegistration.type_name(registration) end

---@param registration ScriptTypeRegistration
--- The type registration.
---@return string
function ScriptTypeRegistration.short_name(registration) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptTypeRegistration
function ScriptTypeRegistration.new(payload) end



---@class ScriptEventRegistration : ReflectReference
---  A handle to a script registered event type.
local ScriptEventRegistration = {}

---@param registration ScriptEventRegistration
--- The event registration.
---@return string
function ScriptEventRegistration.short_name(registration) end

---@param registration ScriptEventRegistration
--- The event registration.
---@return string
function ScriptEventRegistration.namespace(registration) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptEventRegistration
function ScriptEventRegistration.new(payload) end



---@class ScriptSystemBuilder : ReflectReference
---  A builder for systems living in scripts
local ScriptSystemBuilder = {}

---@param builder ScriptSystemBuilder
--- The system builder to add the dependency to.
---@param system ReflectSystem
--- The system to run after.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.after(builder,system) end

---@param builder ScriptSystemBuilder
--- The system builder to make exclusive.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.exclusive(builder) end

---@param builder ScriptSystemBuilder
--- The system builder to add the dependency to.
---@param system ReflectSystem
--- The system to run before.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.before(builder,system) end

---@param builder ScriptSystemBuilder
--- The system builder to add the resource to.
---@param resource ScriptResourceRegistration
--- The resource to add.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.resource(builder,resource) end

---@param builder ScriptSystemBuilder
--- The system builder to add the query to.
---@param query ScriptQueryBuilder
--- The query to add.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.query(builder,query) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptSystemBuilder
function ScriptSystemBuilder.new(payload) end



---@class Recipients : ReflectReference
---  Describes the designated recipients of a script event
---@field  AllScripts Recipients.AllScripts
---@field  AllContexts Recipients.AllContexts
---@field  ScriptEntity Recipients.ScriptEntity
---@field  StaticScript Recipients.StaticScript
local Recipients = {}

---@return Recipients
function Recipients.all_contexts() end

---@param entity Entity
--- The entity the script is attached to.
---@param script Handle<ScriptAsset>
--- The script asset.
---@return Recipients
function Recipients.entity_script(entity,script) end

---@return Recipients
function Recipients.all_scripts() end

---@param script Handle<ScriptAsset>
--- The script asset.
---@return Recipients
function Recipients.static_script(script) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Recipients
function Recipients.new(payload) end



---@class Recipients.AllScripts : EnumVariantRef
--- The `AllScripts` variant of `Recipients`.
Recipients.AllScripts = {}

---@param variant Recipients.AllScripts
---@param payload NoFields
---@return Recipients
function Recipients.AllScripts.new(variant,payload) end



---@class Recipients.AllContexts : EnumVariantRef
--- The `AllContexts` variant of `Recipients`.
Recipients.AllContexts = {}

---@param variant Recipients.AllContexts
---@param payload NoFields
---@return Recipients
function Recipients.AllContexts.new(variant,payload) end



---@class Recipients.ScriptEntity : EnumVariantRef
--- The `ScriptEntity` variant of `Recipients`.
Recipients.ScriptEntity = {}

---@param variant Recipients.ScriptEntity
---@param payload Recipients.ScriptEntity.Fields
---@return Recipients
function Recipients.ScriptEntity.new(variant,payload) end



---@class Recipients.StaticScript : EnumVariantRef
--- The `StaticScript` variant of `Recipients`.
Recipients.StaticScript = {}

---@param variant Recipients.StaticScript
---@param payload Recipients.StaticScript.Fields
---@return Recipients
function Recipients.StaticScript.new(variant,payload) end



---@class ScriptAttachment : ReflectReference
---  Specifies a unique attachment of a script. These attachments are mapped to [`bevy_mod_scripting_core::ContextKey`]'s depending on the context policy used.
---@field  EntityScript ScriptAttachment.EntityScript
---@field  StaticScript ScriptAttachment.StaticScript
local ScriptAttachment = {}

---@param entity Entity
--- The entity to attach the script to.
---@param script Handle<ScriptAsset>
--- The script asset to attach to the entity.
---@return ScriptAttachment
function ScriptAttachment.new_entity_script(entity,script) end

---@param script Handle<ScriptAsset>
--- The script asset to create the attachment from.
---@return ScriptAttachment
function ScriptAttachment.new_static_script(script) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptAttachment
function ScriptAttachment.new(payload) end



---@class ScriptAttachment.EntityScript : EnumVariantRef
--- The `EntityScript` variant of `ScriptAttachment`.
ScriptAttachment.EntityScript = {}

---@param variant ScriptAttachment.EntityScript
---@param payload ScriptAttachment.EntityScript.Fields
---@return ScriptAttachment
function ScriptAttachment.EntityScript.new(variant,payload) end



---@class ScriptAttachment.StaticScript : EnumVariantRef
--- The `StaticScript` variant of `ScriptAttachment`.
ScriptAttachment.StaticScript = {}

---@param variant ScriptAttachment.StaticScript
---@param payload ScriptAttachment.StaticScript.Fields
---@return ScriptAttachment
function ScriptAttachment.StaticScript.new(variant,payload) end



---@class StandardMaterial : ReflectReference
---  A material with "standard" properties used in PBR lighting.
---  Standard property values with pictures here:
---  <https://google.github.io/filament/notes/material_properties.html>.
--- 
---  May be created directly from a [`Color`] or an [`Image`].
--- 
---  The `StandardMaterial` can be extended with more data and custom
---  shaders using [`ExtendedMaterial`]. Examples of how to do this can
---  be found in the Bevy examples.
---@field  base_color? Color
---@field  base_color_channel? UvChannel
---@field  base_color_texture? Handle<Image> | nil
---@field  emissive? LinearRgba
---@field  emissive_exposure_weight? number
---@field  emissive_channel? UvChannel
---@field  emissive_texture? Handle<Image> | nil
---@field  perceptual_roughness? number
---@field  metallic? number
---@field  metallic_roughness_channel? UvChannel
---@field  metallic_roughness_texture? Handle<Image> | nil
---@field  reflectance? number
---@field  specular_tint? Color
---@field  diffuse_transmission? number
---@field  specular_transmission? number
---@field  thickness? number
---@field  ior? number
---@field  attenuation_distance? number
---@field  attenuation_color? Color
---@field  normal_map_channel? UvChannel
---@field  normal_map_texture? Handle<Image> | nil
---@field  flip_normal_map_y? boolean
---@field  occlusion_channel? UvChannel
---@field  occlusion_texture? Handle<Image> | nil
---@field  clearcoat? number
---@field  clearcoat_perceptual_roughness? number
---@field  anisotropy_strength? number
---@field  anisotropy_rotation? number
---@field  double_sided? boolean
---@field  unlit? boolean
---@field  fog_enabled? boolean
---@field  alpha_mode? AlphaMode
---@field  depth_bias? number
---@field  depth_map? Handle<Image> | nil
---@field  parallax_depth_scale? number
---@field  parallax_mapping_method? ParallaxMappingMethod
---@field  max_parallax_layer_count? number
---@field  lightmap_exposure? number
---@field  opaque_render_method? OpaqueRendererMethod
---@field  deferred_lighting_pass_id? integer
---@field  uv_transform? Affine2
local StandardMaterial = {}

---@param color Color
---@return StandardMaterial
function StandardMaterial.from_color(color) end

---@param payload StandardMaterial.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return StandardMaterial
function StandardMaterial.new(payload) end



---@class Instant : ReflectReference
local Instant = {}

---@param _self Instant
---@param earlier Instant
---@return Duration
function Instant.duration_since(_self,earlier) end

---@param _self Instant
---@return Duration
function Instant.elapsed(_self) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Instant
function Instant.new(payload) end



---@class Annulus : ReflectReference
---  A primitive shape formed by the region between two circles, also known as a ring.
---@field  inner_circle? Circle
---@field  outer_circle? Circle
local Annulus = {}

---@param inner_radius number
---@param outer_radius number
---@return Annulus
function Annulus.new(inner_radius,outer_radius) end



---@class Circle : ReflectReference
---  A circle primitive, representing the set of points some distance from the origin
---@field  radius? number
local Circle = {}

---@param radius number
---@return Circle
function Circle.new(radius) end



---@class Rectangle : ReflectReference
---  A rectangle primitive, which is like a square, except that the width and height can be different
---@field  half_size? Vec2
local Rectangle = {}

---@param width number
---@param height number
---@return Rectangle
function Rectangle.new(width,height) end

---@param size Vec2
---@return Rectangle
function Rectangle.from_size(size) end



---@class RegularPolygon : ReflectReference
---  A polygon centered on the origin where all vertices lie on a circle, equally far apart.
---@field  circumcircle? Circle
---@field  sides? integer
local RegularPolygon = {}

---@param circumradius number
---@param sides integer
---@return RegularPolygon
function RegularPolygon.new(circumradius,sides) end



---@class Capsule3d : ReflectReference
---  A 3D capsule primitive centered on the origin
---  A three-dimensional capsule is defined as a surface at a distance (radius) from a line
---@field  radius? number
---@field  half_length? number
local Capsule3d = {}

---@param radius number
---@param length number
---@return Capsule3d
function Capsule3d.new(radius,length) end



---@class Cuboid : ReflectReference
---  A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not
---  required to be the same.
---@field  half_size? Vec3
local Cuboid = {}

---@param x_length number
---@param y_length number
---@param z_length number
---@return Cuboid
function Cuboid.new(x_length,y_length,z_length) end

---@param size Vec3
---@return Cuboid
function Cuboid.from_size(size) end



---@class Cylinder : ReflectReference
---  A cylinder primitive centered on the origin
---@field  radius? number
---@field  half_height? number
local Cylinder = {}

---@param radius number
---@param height number
---@return Cylinder
function Cylinder.new(radius,height) end



---@class Plane3d : ReflectReference
---  A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.
---@field  normal? Dir3
---@field  half_size? Vec2
local Plane3d = {}

---@param normal Vec3
---@param half_size Vec2
---@return Plane3d
function Plane3d.new(normal,half_size) end



---@class Sphere : ReflectReference
---  A sphere primitive, representing the set of all points some distance from the origin
---@field  radius? number
local Sphere = {}

---@param radius number
---@return Sphere
function Sphere.new(radius) end



---@class Torus : ReflectReference
---  A torus primitive, often representing a ring or donut shape
---  The set of points some distance from a circle centered at the origin
---@field  minor_radius? number
---@field  major_radius? number
local Torus = {}

---@param inner_radius number
---@param outer_radius number
---@return Torus
function Torus.new(inner_radius,outer_radius) end



---@class Ray2d : ReflectReference
---  An infinite half-line starting at `origin` and going in `direction` in 2D space.
---@field  origin? Vec2
---@field  direction? Dir2
local Ray2d = {}

---@param _self Ray2d
---@param plane_origin Vec2
---@param plane_normal Vec2
---@return number | nil
function Ray2d.intersect_plane(_self,plane_origin,plane_normal) end

---@param origin Vec2
---@param direction Vec2
---@return Ray2d
function Ray2d.new(origin,direction) end

---@param _self Ray2d
---@param distance number
---@return Vec2
function Ray2d.get_point(_self,distance) end



---@class Ray3d : ReflectReference
---  An infinite half-line starting at `origin` and going in `direction` in 3D space.
---@field  origin? Vec3
---@field  direction? Dir3
local Ray3d = {}

---@param origin Vec3
---@param direction Vec3
---@return Ray3d
function Ray3d.new(origin,direction) end

---@param _self Ray3d
---@param plane_origin Vec3
---@param plane_normal Vec3
---@return number | nil
function Ray3d.intersect_plane(_self,plane_origin,plane_normal) end

---@param _self Ray3d
---@param distance number
---@return Vec3
function Ray3d.get_point(_self,distance) end



---@class Anchor : ReflectReference
---  Normalized (relative to its size) offset of a 2d renderable entity from its [`Transform`].
---@field  [1] Vec2
local Anchor = {}

---@return Anchor
function Anchor.center_right() end

---@return Anchor
function Anchor.top_center() end

---@return Anchor
function Anchor.top_left() end

---@return Anchor
function Anchor.top_right() end

---@return Anchor
function Anchor.center() end

---@return Anchor
function Anchor.bottom_right() end

---@return Anchor
function Anchor.bottom_center() end

---@param _self Anchor
---@return Vec2
function Anchor.as_vec(_self) end

---@return Anchor
function Anchor.bottom_left() end

---@return Anchor
function Anchor.center_left() end

---@param payload Anchor.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Anchor
function Anchor.new(payload) end



---@class Sprite : ReflectReference
---  Describes a sprite to be rendered to a 2D camera
---@field  image? Handle<Image>
---@field  texture_atlas? TextureAtlas | nil
---@field  color? Color
---@field  flip_x? boolean
---@field  flip_y? boolean
---@field  custom_size? Vec2 | nil
---@field  rect? Rect | nil
---@field  image_mode? SpriteImageMode
---@field  alpha_mode? SpriteAlphaMode
local Sprite = {}

---@param image Handle<Image>
---@return Sprite
function Sprite.from_image(image) end

---@param image Handle<Image>
---@param atlas TextureAtlas
---@return Sprite
function Sprite.from_atlas_image(image,atlas) end

---@param size Vec2
---@return Sprite
function Sprite.sized(size) end

---@param color Color
---@param size Vec2
---@return Sprite
function Sprite.from_color(color,size) end

---@param payload Sprite.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Sprite
function Sprite.new(payload) end



---@class Text2d : ReflectReference
---  The top-level 2D text component.
--- 
---  Adding `Text2d` to an entity will pull in required components for setting up 2d text.
---  [Example usage.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/text2d.rs)
--- 
---  The string in this component is the first 'text span' in a hierarchy of text spans that are collected into
---  a [`ComputedTextBlock`]. See `TextSpan` for the component used by children of entities with [`Text2d`].
--- 
---  With `Text2d` the `justify` field of [`TextLayout`] only affects the internal alignment of a block of text and not its
---  relative position, which is controlled by the [`Anchor`] component.
---  This means that for a block of text consisting of only one line that doesn't wrap, the `justify` field will have no effect.
--- 
--- 
---  ```
---  # use bevy_asset::Handle;
---  # use bevy_color::Color;
---  # use bevy_color::palettes::basic::BLUE;
---  # use bevy_ecs::world::World;
---  # use bevy_text::{Font, FontSize, Justify, TextLayout, TextFont, TextColor, TextSpan};
---  # use bevy_sprite::Text2d;
---  #
---  # let font_handle: Handle<Font> = Default::default();
---  # let mut world = World::default();
---  #
---  // Basic usage.
---  world.spawn(Text2d::new("hello world!"));
--- 
---  // With non-default style.
---  world.spawn((
---      Text2d::new("hello world!"),
---      TextFont {
---          font: font_handle.clone().into(),
---          font_size: FontSize::Px(60.0),
---          ..Default::default()
---      },
---      TextColor(BLUE.into()),
---  ));
--- 
---  // With text justification.
---  world.spawn((
---      Text2d::new("hello world\nand bevy!"),
---      TextLayout::justify(Justify::Center)
---  ));
--- 
---  // With spans
---  world.spawn(Text2d::new("hello ")).with_children(|parent| {
---      parent.spawn(TextSpan::new("world"));
---      parent.spawn((TextSpan::new("!"), TextColor(BLUE.into())));
---  });
---  ```
--- 
---  Viewport-based `FontSize` variants for `Text2d` are resolved based on the primary window’s logical size.
---@field  [1] string
local Text2d = {}

---@param _self Text2d
---@return string
function Text2d.as_str(_self) end

---@param text string
---@return Text2d
function Text2d.new(text) end

---@param _self Text2d
---@param text string
---@return nil
function Text2d.set(_self,text) end



---@class ReflectSchedule : ReflectReference
---  A reflectable schedule.
---@field  type_path? string
---@field  label? ReflectableScheduleLabel
local ReflectSchedule = {}

---@param schedule ReflectSchedule
--- The schedule to retrieve the systems from.
---@return ReflectSystem[]
function ReflectSchedule.systems(schedule) end

---@param schedule ReflectSchedule
--- The schedule to render.
---@return string
function ReflectSchedule.render_dot(schedule) end

---@param schedule ReflectSchedule
--- The schedule to retrieve the system from.
---@param name string
--- The identifier or full path of the system to retrieve.
---@return ReflectSystem | nil
function ReflectSchedule.get_system_by_name(schedule,name) end

---@param payload ReflectSchedule.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReflectSchedule
function ReflectSchedule.new(payload) end



---@class ReflectSystem : ReflectReference
---  A reflectable system.
local ReflectSystem = {}

---@param system ReflectSystem
--- The system to retrieve the identifier from.
---@return string
function ReflectSystem.identifier(system) end

---@param system ReflectSystem
--- The system to retrieve the path from.
---@return string
function ReflectSystem.path(system) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReflectSystem
function ReflectSystem.new(payload) end



---@class TextFont : ReflectReference
---  `TextFont` determines the style of a text span within a [`ComputedTextBlock`], specifically
---  the font face, the font size, the line height, and the antialiasing method.
---@field  font? FontSource
---@field  font_size? FontSize
---@field  weight? FontWeight
---@field  width? FontWidth
---@field  style? FontStyle
---@field  font_smoothing? FontSmoothing
---@field  font_features? FontFeatures
---@field  font_variations? FontVariations
local TextFont = {}

---@param _self TextFont
---@param font bevy_asset::handle::Handle<bevy_text::font::Font>
---@return TextFont
function TextFont.with_font(_self,font) end

---@param _self TextFont
---@param font_size number
---@return TextFont
function TextFont.with_font_size(_self,font_size) end

---@param font_size number
---@return TextFont
function TextFont.from_font_size(font_size) end

---@param payload TextFont.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TextFont
function TextFont.new(payload) end



---@class TextLayout : ReflectReference
---  Component with text format settings for a block of text.
--- 
---  A block of text is composed of text spans, which each have a separate string value and [`TextFont`]. Text
---  spans associated with a text block are collected into [`ComputedTextBlock`] for layout, and then inserted
---  to [`TextLayoutInfo`] for rendering.
--- 
---  See `Text2d` in `bevy_sprite` for the core component of 2d text, and `Text` in `bevy_ui` for UI text.
---@field  justify? Justify
---@field  linebreak? LineBreak
local TextLayout = {}

---@return TextLayout
function TextLayout.new_with_no_wrap() end

---@param justify Justify
---@return TextLayout
function TextLayout.new_with_justify(justify) end

---@param payload TextLayout.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TextLayout
function TextLayout.new(payload) end



---@class TextSpan : ReflectReference
---  A span of text in a tree of spans.
--- 
---  A `TextSpan` is only valid when it exists as a child of a parent that has either `Text` or
---  `Text2d`. The parent's `Text` / `Text2d` component contains the base text content. Any children
---  with `TextSpan` extend this text by appending their content to the parent's text in sequence to
---  form a [`ComputedTextBlock`]. The parent's [`TextLayout`] determines the layout of the block
---  but each node has its own [`TextFont`] and [`TextColor`].
---@field  [1] string
local TextSpan = {}

---@param _self TextSpan
---@param text string
---@return nil
function TextSpan.set(_self,text) end

---@param _self TextSpan
---@return string
function TextSpan.as_str(_self) end

---@param text string
---@return TextSpan
function TextSpan.new(text) end



---@class Stopwatch : ReflectReference
---  A Stopwatch is a struct that tracks elapsed time when started.
--- 
---  Note that in order to advance the stopwatch [`tick`](Stopwatch::tick) **MUST** be called.
---  # Examples
--- 
---  ```
---  # use bevy_time::*;
---  use std::time::Duration;
---  let mut stopwatch = Stopwatch::new();
---  assert_eq!(stopwatch.elapsed_secs(), 0.0);
--- 
---  stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second
---  assert_eq!(stopwatch.elapsed_secs(), 1.0);
--- 
---  stopwatch.pause();
---  stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick
---  assert_eq!(stopwatch.elapsed_secs(), 1.0);
--- 
---  stopwatch.reset(); // reset the stopwatch
---  assert!(stopwatch.is_paused());
---  assert_eq!(stopwatch.elapsed_secs(), 0.0);
---  ```
---@field  elapsed? Duration
---@field  is_paused? boolean
local Stopwatch = {}

---@param _self Stopwatch
---@param elapsed Duration
---@return nil
function Stopwatch.set_elapsed(_self,elapsed) end

---@param _self Stopwatch
---@return number
function Stopwatch.elapsed_secs(_self) end

---@param _self Stopwatch
---@param delta number | Duration
---@return nil
function Stopwatch.tick(_self,delta) end

---@return Stopwatch
function Stopwatch.new() end

---@param _self Stopwatch
---@return nil
function Stopwatch.pause(_self) end

---@param _self Stopwatch
---@return nil
function Stopwatch.unpause(_self) end

---@param _self Stopwatch
---@return nil
function Stopwatch.reset(_self) end

---@param _self Stopwatch
---@return Duration
function Stopwatch.elapsed(_self) end

---@param _self Stopwatch
---@return boolean
function Stopwatch.is_paused(_self) end



---@class Timer : ReflectReference
---  Tracks elapsed time. Enters the finished state once `duration` is reached.
--- 
---  Note that in order to advance the timer [`tick`](Timer::tick) **MUST** be called.
--- 
---  # Timer modes
--- 
---  There are two timer modes ([`TimerMode`]):
--- 
---  - Non repeating timers will stop tracking and stay in the finished state until reset.
---  - Repeating timers will only be in the finished state on each tick `duration` is reached or
---    exceeded, and can still be reset at any given point.
--- 
---  # Pausing timers
--- 
---  You can pause a timer using [`Timer::pause`]. Paused timers will not have elapsed time increased.
--- 
---  # Elapsing multiple times a frame
--- 
---  Repeating timers might elapse multiple times per frame if the time is advanced by more than the timer duration.
---  You can check how many times a timer elapsed each tick with [`Timer::times_finished_this_tick`].
---  For non-repeating timers, this will always be 0 or 1.
---@field  stopwatch? Stopwatch
---@field  duration? Duration
---@field  mode? TimerMode
---@field  finished? boolean
---@field  times_finished_this_tick? integer
local Timer = {}

---@param _self Timer
---@param elapsed Duration
---@return nil
function Timer.set_elapsed(_self,elapsed) end

---@param _self Timer
---@return number
function Timer.elapsed_secs(_self) end

---@param _self Timer
---@return Duration
function Timer.remaining(_self) end

---@param _self Timer
---@return Duration
function Timer.elapsed(_self) end

---@param _self Timer
---@return number
function Timer.fraction_remaining(_self) end

---@param _self Timer
---@return nil
function Timer.reset(_self) end

---@param _self Timer
---@return boolean
function Timer.paused(_self) end

---@param secs number
---@param mode TimerMode
---@return Timer
function Timer.from_seconds(secs,mode) end

---@param duration Duration
---@param mode TimerMode
---@return Timer
function Timer.new(duration,mode) end

---@param _self Timer
---@return boolean
function Timer.just_finished(_self) end

---@param _self Timer
---@return TimerMode
function Timer.mode(_self) end

---@param _self Timer
---@return boolean
function Timer.finished(_self) end

---@param _self Timer
---@return nil
function Timer.almost_finish(_self) end

---@param _self Timer
---@return integer
function Timer.times_finished_this_tick(_self) end

---@param _self Timer
---@return number
function Timer.fraction(_self) end

---@param _self Timer
---@param duration Duration
---@return nil
function Timer.set_duration(_self,duration) end

---@param _self Timer
---@return nil
function Timer.unpause(_self) end

---@param _self Timer
---@return nil
function Timer.pause(_self) end

---@param _self Timer
---@param delta number | Duration
---@return nil
function Timer.tick(_self,delta) end

---@param _self Timer
---@param mode TimerMode
---@return nil
function Timer.set_mode(_self,mode) end

---@param _self Timer
---@return number
function Timer.remaining_secs(_self) end

---@param _self Timer
---@return Duration
function Timer.duration(_self) end



---@class GlobalTransform : ReflectReference
---  [`GlobalTransform`] is an affine transformation from entity-local coordinates to worldspace coordinates.
--- 
---  You cannot directly mutate [`GlobalTransform`]; instead, you change an entity's transform by manipulating
---  its [`Transform`], which indirectly causes Bevy to update its [`GlobalTransform`].
--- 
---  * To get the global transform of an entity, you should get its [`GlobalTransform`].
---  * For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].
---    [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.
--- 
---  ## [`Transform`] and [`GlobalTransform`]
--- 
---  [`Transform`] transforms an entity relative to its parent's reference frame, or relative to world space coordinates,
---  if it doesn't have a [`ChildOf`](bevy_ecs::hierarchy::ChildOf) component.
--- 
---  [`GlobalTransform`] is managed by Bevy; it is computed by successively applying the [`Transform`] of each ancestor
---  entity which has a Transform. This is done automatically by Bevy-internal systems in the [`TransformSystems::Propagate`]
---  system set.
--- 
---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
---  update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag
---  before the [`GlobalTransform`] is updated.
--- 
---  [`TransformSystems::Propagate`]: crate::TransformSystems::Propagate
--- 
---  # Examples
--- 
---  - [`transform`][transform_example]
--- 
---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  [1] Affine3A
---@operator mul(GlobalTransform | Transform): GlobalTransform
local GlobalTransform = {}

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.back(_self) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.translation(_self) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.scale(_self) end

---@param _self GlobalTransform
---@param rhs GlobalTransform | Transform
---@return GlobalTransform
function GlobalTransform.mul(_self,rhs) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.left(_self) end

---@param _self GlobalTransform
---@param point Vec3
---@return Vec3
function GlobalTransform.transform_point(_self,point) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.up(_self) end

---@param _self GlobalTransform
---@return Quat
function GlobalTransform.rotation(_self) end

---@return GlobalTransform
function GlobalTransform.identity() end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.forward(_self) end

---@param _self GlobalTransform
---@return Mat4
function GlobalTransform.to_matrix(_self) end

---@param _self GlobalTransform
---@param other GlobalTransform
---@return boolean
function GlobalTransform.__eq(_self,other) end

---@param _self GlobalTransform
---@param other GlobalTransform
---@return boolean
function GlobalTransform.eq(_self,other) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.right(_self) end

---@param _self GlobalTransform
---@return Transform
function GlobalTransform.compute_transform(_self) end

---@param x number
---@param y number
---@param z number
---@return GlobalTransform
function GlobalTransform.from_xyz(x,y,z) end

---@param _self GlobalTransform
---@return Vec3
function GlobalTransform.down(_self) end

---@param _self GlobalTransform
---@param parent GlobalTransform
---@return Transform
function GlobalTransform.reparented_to(_self,parent) end

---@param payload GlobalTransform.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GlobalTransform
function GlobalTransform.new(payload) end



---@class Transform : ReflectReference
---  Describe the position of an entity. If the entity has a parent, the position is relative
---  to its parent position.
--- 
---  * To place or move an entity, you should set its [`Transform`].
---  * To get the global transform of an entity, you should get its [`GlobalTransform`].
---  * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
---    [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.
--- 
---  Transforms compose from right to left: if `t1` and `t2` are transforms, then `t1 * t2`
---  corresponds to applying `t2` *first*, *then* applying `t1`.
--- 
---  ## [`Transform`] and [`GlobalTransform`]
--- 
---  [`Transform`] is the position of an entity relative to its parent position, or the reference
---  frame if it doesn't have a [`ChildOf`](bevy_ecs::hierarchy::ChildOf) component.
--- 
---  [`GlobalTransform`] is the position of an entity relative to the reference frame.
--- 
---  [`GlobalTransform`] is updated from [`Transform`] in the [`TransformSystems::Propagate`]
---  system set.
--- 
---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
---  update the [`Transform`] of an entity during this set or after, you will notice a 1 frame lag
---  before the [`GlobalTransform`] is updated.
--- 
---  [`TransformSystems::Propagate`]: crate::TransformSystems::Propagate
--- 
---  # Examples
--- 
---  - [`transform`][transform_example]
--- 
---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  translation? Vec3
---@field  rotation? Quat
---@field  scale? Vec3
---@operator mul(Transform | Vec3): Transform | Vec3
local Transform = {}

---@param _self Transform
---@return Vec3
function Transform.local_z(_self) end

---@param _self Transform
---@return Vec3
function Transform.up(_self) end

---@param _self Transform
---@param translation Vec3
---@return Transform
function Transform.with_translation(_self,translation) end

---@param _self Transform
---@return Vec3
function Transform.local_y(_self) end

---@param x number
---@param y number
---@param z number
---@return Transform
function Transform.from_xyz(x,y,z) end

---@param translation Vec3
---@return Transform
function Transform.from_translation(translation) end

---@param _self Transform
---@param rotation Quat
---@return nil
function Transform.rotate(_self,rotation) end

---@param _self Transform
---@param point Vec3
---@return Vec3
function Transform.transform_point(_self,point) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_z(_self,angle) end

---@param _self Transform
---@return Mat4
function Transform.to_matrix(_self) end

---@return Transform
function Transform.identity() end

---@param _self Transform
---@param rotation Quat
---@return Transform
function Transform.with_rotation(_self,rotation) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_local_y(_self,angle) end

---@param _self Transform
---@param axis Vec3
---@param angle number
---@return nil
function Transform.rotate_axis(_self,axis,angle) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_local_z(_self,angle) end

---@param _self Transform
---@return Vec3
function Transform.right(_self) end

---@param _self Transform
---@param point Vec3
---@param rotation Quat
---@return nil
function Transform.translate_around(_self,point,rotation) end

---@param _self Transform
---@param point Vec3
---@param rotation Quat
---@return nil
function Transform.rotate_around(_self,point,rotation) end

---@param _self Transform
---@return Vec3
function Transform.local_x(_self) end

---@param _self Transform
---@param rotation Quat
---@return nil
function Transform.rotate_local(_self,rotation) end

---@param _self Transform
---@param scale Vec3
---@return Transform
function Transform.with_scale(_self,scale) end

---@param _self Transform
---@return Vec3
function Transform.forward(_self) end

---@param _self Transform
---@param direction Vec3
---@param up Vec3
---@return Transform
function Transform.looking_to(_self,direction,up) end

---@param _self Transform
---@param target Vec3
---@param up Vec3
---@return nil
function Transform.look_at(_self,target,up) end

---@param _self Transform
---@return Vec3
function Transform.back(_self) end

---@param _self Transform
---@param target Vec3
---@param up Vec3
---@return Transform
function Transform.looking_at(_self,target,up) end

---@param _self Transform
---@param rhs Transform | Vec3
---@return Transform | Vec3
function Transform.mul(_self,rhs) end

---@param _self Transform
---@param other Transform
---@return Transform
function Transform.mul_transform(_self,other) end

---@param _self Transform
---@return Vec3
function Transform.down(_self) end

---@param rotation Quat
---@return Transform
function Transform.from_rotation(rotation) end

---@param _self Transform
---@param other Transform
---@return boolean
function Transform.__eq(_self,other) end

---@param _self Transform
---@param other Transform
---@return boolean
function Transform.eq(_self,other) end

---@param _self Transform
---@return boolean
function Transform.is_finite(_self) end

---@param _self Transform
---@return Vec3
function Transform.left(_self) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_x(_self,angle) end

---@param _self Transform
---@param direction Vec3
---@param up Vec3
---@return nil
function Transform.look_to(_self,direction,up) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_y(_self,angle) end

---@param matrix Mat4
---@return Transform
function Transform.from_matrix(matrix) end

---@param _self Transform
---@param angle number
---@return nil
function Transform.rotate_local_x(_self,angle) end

---@param scale Vec3
---@return Transform
function Transform.from_scale(scale) end

---@param payload Transform.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Transform
function Transform.new(payload) end



---@class UiRect : ReflectReference
---  A type which is commonly used to define margins, paddings and borders.
--- 
---  # Examples
--- 
---  ## Margin
--- 
---  A margin is used to create space around UI elements, outside of any defined borders.
--- 
---  ```
---  # use bevy_ui::{UiRect, Val};
---  #
---  let margin = UiRect::all(Val::Auto); // Centers the UI element
---  ```
--- 
---  ## Padding
--- 
---  A padding is used to create space around UI elements, inside of any defined borders.
--- 
---  ```
---  # use bevy_ui::{UiRect, Val};
---  #
---  let padding = UiRect {
---      left: Val::Px(10.0),
---      right: Val::Px(20.0),
---      top: Val::Px(30.0),
---      bottom: Val::Px(40.0),
---  };
---  ```
--- 
---  ## Borders
--- 
---  A border is used to define the width of the border of a UI element.
--- 
---  ```
---  # use bevy_ui::{UiRect, Val};
---  #
---  let border = UiRect {
---      left: Val::Px(10.0),
---      right: Val::Px(20.0),
---      top: Val::Px(30.0),
---      bottom: Val::Px(40.0),
---  };
---  ```
---@field  left? Val
---@field  right? Val
---@field  top? Val
---@field  bottom? Val
local UiRect = {}

---@param left Val
---@param right Val
---@param top Val
---@param bottom Val
---@return UiRect
function UiRect.new(left,right,top,bottom) end

---@param value Val
---@return UiRect
function UiRect.vertical(value) end

---@param value Val
---@return UiRect
function UiRect.all(value) end

---@param left number
---@param right number
---@param top number
---@param bottom number
---@return UiRect
function UiRect.percent(left,right,top,bottom) end

---@param value Val
---@return UiRect
function UiRect.horizontal(value) end

---@param left number
---@param right number
---@param top number
---@param bottom number
---@return UiRect
function UiRect.px(left,right,top,bottom) end

---@param horizontal Val
---@param vertical Val
---@return UiRect
function UiRect.axes(horizontal,vertical) end

---@return UiRect
function UiRect.zero() end



---@class Val : ReflectReference
---  Represents the possible value types for layout properties.
--- 
---  This enum allows specifying values for various [`Node`](crate::Node) properties in different units,
---  such as logical pixels, percentages, or automatically determined values.
--- 
---  `Val` also implements [`core::str::FromStr`] to allow parsing values from strings in the format `#.#px`. Whitespaces between the value and unit is allowed. The following units are supported:
---  * `px`: logical pixels
---  * `%`: percentage
---  * `vw`: percentage of the viewport width
---  * `vh`: percentage of the viewport height
---  * `vmin`: percentage of the viewport's smaller dimension
---  * `vmax`: percentage of the viewport's larger dimension
--- 
---  Additionally, `auto` will be parsed as [`Val::Auto`].
---@field  Auto Val.Auto
---@field  Px Val.Px
---@field  Percent Val.Percent
---@field  Vw Val.Vw
---@field  Vh Val.Vh
---@field  VMin Val.VMin
---@field  VMax Val.VMax
---@field  Em Val.Em
---@field  Rem Val.Rem
local Val = {}

---@return Val
function Val.auto() end

---@param value number
---@return Val
function Val.px(value) end

---@param value number
---@return Val
function Val.vmin(value) end

---@param value number
---@return Val
function Val.vw(value) end

---@param value number
---@return Val
function Val.percent(value) end

---@param value number
---@return Val
function Val.vmax(value) end

---@param value number
---@return Val
function Val.vh(value) end

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Val
function Val.new(payload) end



---@class Val.Auto : EnumVariantRef
--- The `Auto` variant of `Val`.
Val.Auto = {}

---@param variant Val.Auto
---@param payload NoFields
---@return Val
function Val.Auto.new(variant,payload) end



---@class Val.Px : EnumVariantRef
--- The `Px` variant of `Val`.
Val.Px = {}

---@param variant Val.Px
---@param payload Val.Px.Fields
---@return Val
function Val.Px.new(variant,payload) end



---@class Val.Percent : EnumVariantRef
--- The `Percent` variant of `Val`.
Val.Percent = {}

---@param variant Val.Percent
---@param payload Val.Percent.Fields
---@return Val
function Val.Percent.new(variant,payload) end



---@class Val.Vw : EnumVariantRef
--- The `Vw` variant of `Val`.
Val.Vw = {}

---@param variant Val.Vw
---@param payload Val.Vw.Fields
---@return Val
function Val.Vw.new(variant,payload) end



---@class Val.Vh : EnumVariantRef
--- The `Vh` variant of `Val`.
Val.Vh = {}

---@param variant Val.Vh
---@param payload Val.Vh.Fields
---@return Val
function Val.Vh.new(variant,payload) end



---@class Val.VMin : EnumVariantRef
--- The `VMin` variant of `Val`.
Val.VMin = {}

---@param variant Val.VMin
---@param payload Val.VMin.Fields
---@return Val
function Val.VMin.new(variant,payload) end



---@class Val.VMax : EnumVariantRef
--- The `VMax` variant of `Val`.
Val.VMax = {}

---@param variant Val.VMax
---@param payload Val.VMax.Fields
---@return Val
function Val.VMax.new(variant,payload) end



---@class Val.Em : EnumVariantRef
--- The `Em` variant of `Val`.
Val.Em = {}

---@param variant Val.Em
---@param payload Val.Em.Fields
---@return Val
function Val.Em.new(variant,payload) end



---@class Val.Rem : EnumVariantRef
--- The `Rem` variant of `Val`.
Val.Rem = {}

---@param variant Val.Rem
---@param payload Val.Rem.Fields
---@return Val
function Val.Rem.new(variant,payload) end



---@class BorderColor : ReflectReference
---  The border color of the UI node.
---@field  top? Color
---@field  right? Color
---@field  bottom? Color
---@field  left? Color
local BorderColor = {}

---@param color Color
---@return BorderColor
function BorderColor.all(color) end

---@param payload BorderColor.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BorderColor
function BorderColor.new(payload) end



---@class BorderRadius : ReflectReference
---  Used to add rounded corners to a UI node. You can set a UI node to have uniformly
---  rounded corners, elliptical corners, or specify a different radius for each corner. If a given
---  radius exceeds half the length of the node's height or width, the radius will be calculated
---  as half the height or width.
--- 
---  Percentage horizontal and vertical values are based on the node's width and height, respectively.
--- 
---  # Example
---  ```rust
---  # use bevy_ecs::prelude::*;
---  # use bevy_ui::prelude::*;
---  # use bevy_color::palettes::basic::{BLUE};
---  fn setup_ui(mut commands: Commands) {
---      commands.spawn((
---          Node {
---              width: Val::Px(100.),
---              height: Val::Px(100.),
---              border: UiRect::all(Val::Px(2.)),
---              border_radius: BorderRadius {
---                  // rounded corners, x and y radii equal
---                  top_left: CornerRadius::circular(px(10.)),
---                  top_right: percent(20.).into(),
---                  // elliptical corner
---                  bottom_right: CornerRadius::new(px(30.), px(20.)),
---                  bottom_left: CornerRadius { x: px(10.), y: px(40.) },
---              },
---              ..Default::default()
---          },
---          BackgroundColor(BLUE.into()),
---      ));
---  }
---  ```
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
---@field  top_left? CornerRadius
---@field  top_right? CornerRadius
---@field  bottom_right? CornerRadius
---@field  bottom_left? CornerRadius
local BorderRadius = {}

---@param radius Val
---@return BorderRadius
function BorderRadius.all(radius) end

---@return BorderRadius
function BorderRadius.max() end

---@param top_left number
---@param top_right number
---@param bottom_right number
---@param bottom_left number
---@return BorderRadius
function BorderRadius.percent(top_left,top_right,bottom_right,bottom_left) end

---@param top_left number
---@param top_right number
---@param bottom_right number
---@param bottom_left number
---@return BorderRadius
function BorderRadius.px(top_left,top_right,bottom_right,bottom_left) end

---@return BorderRadius
function BorderRadius.zero() end

---@param payload BorderRadius.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BorderRadius
function BorderRadius.new(payload) end



---@class ComputedNode : ReflectReference
---  Provides the computed size and layout properties of a UI [`Node`].
--- 
---  All of the fields are automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`)
---  during `PostUpdate` in the [`UiSystems::Layout`](super::UiSystems) set.
--- 
---  The fields are measured in physical pixels.
---  You can multiply by the `inverse_scale_factor` field to convert back to logical pixels.
--- 
---  Fields in this struct are public but should only rarely be directly modified.
---  For example, in a scrollbar you may want to derive the handle's size from the proportion of
---  scrollable content in-view. You can directly modify `ComputedNode` after layout to set the
---  handle size without a frame delay.
---@field  size? Vec2
---@field  content_size? Vec2
---@field  scrollbar_size? Vec2
---@field  scroll_position? Vec2
---@field  outline_width? number
---@field  outline_offset? number
---@field  unrounded_size? Vec2
---@field  border? BorderRect
---@field  border_radius? ResolvedBorderRadius
---@field  padding? BorderRect
---@field  inverse_scale_factor? number
---@field  em_size? EmSize
---@field  rem_size? RemSize
local ComputedNode = {}

---@param _self ComputedNode
---@return boolean
function ComputedNode.is_empty(_self) end

---@param _self ComputedNode
---@return number
function ComputedNode.inverse_scale_factor(_self) end

---@param _self ComputedNode
---@return Vec2
function ComputedNode.size(_self) end

---@param _self ComputedNode
---@return Vec2
function ComputedNode.content_size(_self) end

---@param payload ComputedNode.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ComputedNode
function ComputedNode.new(payload) end



---@class Overflow : ReflectReference
---  Whether to show or hide overflowing items
---@field  x? OverflowAxis
---@field  y? OverflowAxis
local Overflow = {}

---@return Overflow
function Overflow.clip_y() end

---@return Overflow
function Overflow.scroll() end

---@return Overflow
function Overflow.clip_x() end

---@return Overflow
function Overflow.hidden() end

---@return Overflow
function Overflow.clip() end

---@return Overflow
function Overflow.visible() end

---@param payload Overflow.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Overflow
function Overflow.new(payload) end



---@class ImageNode : ReflectReference
---  A UI Node that renders an image.
---@field  color? Color
---@field  image? Handle<Image>
---@field  texture_atlas? TextureAtlas | nil
---@field  flip_x? boolean
---@field  flip_y? boolean
---@field  rect? Rect | nil
---@field  image_mode? NodeImageMode
---@field  visual_box? VisualBox
local ImageNode = {}

---@param image Handle<Image>
---@return ImageNode
function ImageNode.new(image) end

---@param _self ImageNode
---@param color Color
---@return ImageNode
function ImageNode.with_color(_self,color) end



---@class Text : ReflectReference
---  The top-level UI text component.
--- 
---  Adding [`Text`] to an entity will pull in required components for setting up a UI text node.
--- 
---  The string in this component is the first 'text span' in a hierarchy of text spans that are collected into
---  a [`ComputedTextBlock`]. See [`TextSpan`](bevy_text::TextSpan) for the component used by children of entities with [`Text`].
--- 
--- 
---  ```
---  # use bevy_asset::Handle;
---  # use bevy_color::Color;
---  # use bevy_color::palettes::basic::BLUE;
---  # use bevy_ecs::world::World;
---  # use bevy_text::{Font, FontSize, Justify, TextLayout, TextFont, TextColor, TextSpan};
---  # use bevy_ui::prelude::Text;
---  #
---  # let font_handle: Handle<Font> = Default::default();
---  # let mut world = World::default();
---  #
---  // Basic usage.
---  world.spawn(Text::new("hello world!"));
--- 
---  // With non-default style.
---  world.spawn((
---      Text::new("hello world!"),
---      TextFont {
---          font: font_handle.clone().into(),
---          font_size: FontSize::Px(60.0),
---          ..Default::default()
---      },
---      TextColor(BLUE.into()),
---  ));
--- 
---  // With text justification.
---  world.spawn((
---      Text::new("hello world\nand bevy!"),
---      TextLayout::justify(Justify::Center)
---  ));
--- 
---  // With spans
---  world.spawn(Text::new("hello ")).with_children(|parent| {
---      parent.spawn(TextSpan::new("world"));
---      parent.spawn((TextSpan::new("!"), TextColor(BLUE.into())));
---  });
---  ```
---@field  [1] string
local Text = {}

---@param text string
---@return Text
function Text.new(text) end

---@param _self Text
---@param text string
---@return nil
function Text.set(_self,text) end

---@param _self Text
---@return string
function Text.as_str(_self) end



---@class Window : ReflectReference
---  The defining [`Component`] for window entities,
---  storing information about how it should appear and behave.
--- 
---  Each window corresponds to an entity, and is uniquely identified by the value of their [`Entity`].
---  When the [`Window`] component is added to an entity, a new window will be opened.
---  When it is removed or the entity is despawned, the window will close.
--- 
---  The primary window entity (and the corresponding window) is spawned by default
---  by [`WindowPlugin`](crate::WindowPlugin) and is marked with the [`PrimaryWindow`] component.
--- 
---  This component is synchronized with `winit` through `bevy_winit`:
---  it will reflect the current state of the window and can be modified to change this state.
--- 
---  # Example
--- 
---  Because this component is synchronized with `winit`, it can be used to perform
---  OS-integrated windowing operations. For example, here's a simple system
---  to change the window mode:
--- 
---  ```
---  # use bevy_ecs::query::With;
---  # use bevy_ecs::system::Query;
---  # use bevy_window::{WindowMode, PrimaryWindow, Window, MonitorSelection, VideoModeSelection};
---  fn change_window_mode(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
---      // Query returns one window typically.
---      for mut window in windows.iter_mut() {
---          window.mode =
---              WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
---      }
---  }
---  ```
---@field  present_mode? PresentMode
---@field  mode? WindowMode
---@field  position? WindowPosition
---@field  resolution? WindowResolution
---@field  title? string
---@field  name? string | nil
---@field  composite_alpha_mode? CompositeAlphaMode
---@field  resize_constraints? WindowResizeConstraints
---@field  resizable? boolean
---@field  enabled_buttons? EnabledButtons
---@field  decorations? boolean
---@field  transparent? boolean
---@field  focused? boolean
---@field  window_level? WindowLevel
---@field  canvas? string | nil
---@field  fit_canvas_to_parent? boolean
---@field  prevent_default_event_handling? boolean
---@field  internal? InternalWindowState
---@field  ime_enabled? boolean
---@field  ime_position? Vec2
---@field  window_theme? WindowTheme | nil
---@field  visible? boolean
---@field  skip_taskbar? boolean
---@field  clip_children? boolean
---@field  desired_maximum_frame_latency? NonZeroU32 | nil
---@field  recognize_pinch_gesture? boolean
---@field  recognize_rotation_gesture? boolean
---@field  recognize_doubletap_gesture? boolean
---@field  recognize_pan_gesture? [integer, integer] | nil
---@field  movable_by_window_background? boolean
---@field  fullsize_content_view? boolean
---@field  has_shadow? boolean
---@field  titlebar_shown? boolean
---@field  titlebar_transparent? boolean
---@field  titlebar_show_title? boolean
---@field  titlebar_show_buttons? boolean
---@field  borderless_game? boolean
---@field  prefers_home_indicator_hidden? boolean
---@field  prefers_status_bar_hidden? boolean
---@field  preferred_screen_edges_deferring_system_gestures? ScreenEdge
local Window = {}

---@param _self Window
---@return Vec2 | nil
function Window.cursor_position(_self) end

---@param _self Window
---@return integer
function Window.physical_height(_self) end

---@param _self Window
---@return Vec2
function Window.size(_self) end

---@param _self Window
---@return number
function Window.height(_self) end

---@param _self Window
---@return number
function Window.width(_self) end

---@return ReflectReference | nil
function Window.primary() end

---@param _self Window
---@return number
function Window.scale_factor(_self) end

---@return Entity | nil
function Window.primary_entity() end

---@param _self Window
---@param position? Vec2 | nil
---@return nil
function Window.set_cursor_position(_self,position) end

---@param _self Window
---@return integer
function Window.physical_width(_self) end

---@param payload Window.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Window
function Window.new(payload) end



---@class WindowResolution : ReflectReference
---  Controls the size of a [`Window`]
--- 
---  ## Physical, logical and requested sizes
--- 
---  There are three sizes associated with a window:
---  - the physical size,
---    which represents the actual height and width in physical pixels
---    the window occupies on the monitor,
---  - the logical size,
---    which represents the size that should be used to scale elements
---    inside the window, measured in logical pixels,
---  - the requested size,
---    measured in logical pixels, which is the value submitted
---    to the API when creating the window, or requesting that it be resized.
--- 
---  ## Scale factor
--- 
---  The reason logical size and physical size are separated and can be different
---  is to account for the cases where:
---  - several monitors have different pixel densities,
---  - the user has set up a pixel density preference in its operating system,
---  - the Bevy `App` has specified a specific scale factor between both.
--- 
---  The factor between physical size and logical size can be retrieved with
---  [`WindowResolution::scale_factor`].
--- 
---  For the first two cases, a scale factor is set automatically by the operating
---  system through the window backend. You can get it with
---  [`WindowResolution::base_scale_factor`].
--- 
---  For the third case, you can override this automatic scale factor with
---  [`WindowResolution::set_scale_factor_override`].
--- 
---  ## Requested and obtained sizes
--- 
---  The logical size should be equal to the requested size after creating/resizing,
---  when possible.
---  The reason the requested size and logical size might be different
---  is because the corresponding physical size might exceed limits (either the
---  size limits of the monitor, or limits defined in [`WindowResizeConstraints`]).
--- 
---  Note: The requested size is not kept in memory, for example requesting a size
---  too big for the screen, making the logical size different from the requested size,
---  and then setting a scale factor that makes the previous requested size within
---  the limits of the screen will not get back that previous requested size.
---@field  physical_width? integer
---@field  physical_height? integer
---@field  scale_factor_override? number | nil
---@field  scale_factor? number
local WindowResolution = {}

---@param _self WindowResolution
---@return number
function WindowResolution.height(_self) end

---@param _self WindowResolution
---@param width number
---@param height number
---@return nil
function WindowResolution.set(_self,width,height) end

---@param physical_width integer
---@param physical_height integer
---@return WindowResolution
function WindowResolution.new(physical_width,physical_height) end

---@param _self WindowResolution
---@return number
function WindowResolution.scale_factor(_self) end

---@param _self WindowResolution
---@return number
function WindowResolution.width(_self) end



---@class Duration : ReflectReference
---@operator mul(number): Duration
---@operator sub(Duration): Duration
---@operator div(number): Duration
---@operator add(Duration): Duration
local Duration = {}

---@param _self Duration
---@return number
function Duration.as_secs(_self) end

---@return Duration
function Duration.zero() end

---@param _self Duration
---@return boolean
function Duration.is_zero(_self) end

---@param micros integer
---@return Duration
function Duration.from_micros(micros) end

---@param _self Duration
---@return integer
function Duration.as_millis(_self) end

---@param _self Duration
---@param other Duration
---@return boolean
function Duration.__eq(_self,other) end

---@param _self Duration
---@param other Duration
---@return boolean
function Duration.eq(_self,other) end

---@param millis integer
---@return Duration
function Duration.from_millis(millis) end

---@param _self Duration
---@param factor number
---@return Duration
function Duration.mul(_self,factor) end

---@param _self Duration
---@param other Duration
---@return boolean
function Duration.__lt(_self,other) end

---@param _self Duration
---@param other Duration
---@return boolean
function Duration.lt(_self,other) end

---@param _self Duration
---@param rhs Duration
---@return Duration
function Duration.sub(_self,rhs) end

---@param secs number
---@return Duration
function Duration.from_secs(secs) end

---@param nanos integer
---@return Duration
function Duration.from_nanos(nanos) end

---@param _self Duration
---@return number
function Duration.as_secs_f64(_self) end

---@param _self Duration
---@param divisor number
---@return Duration
function Duration.div(_self,divisor) end

---@param _self Duration
---@param rhs Duration
---@return Duration
function Duration.add(_self,rhs) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Duration
function Duration.new(payload) end



---@class DVec2 : ReflectReference
---@field  x? number
---@field  y? number
local DVec2 = {}

---@param _self DVec2
---@return Vec2
function DVec2.as_vec2(_self) end

---@param x number
---@param y number
---@return DVec2
function DVec2.new(x,y) end



---@class DVec3 : ReflectReference
---@field  x? number
---@field  y? number
---@field  z? number
local DVec3 = {}

---@param x number
---@param y number
---@param z number
---@return DVec3
function DVec3.new(x,y,z) end

---@param _self DVec3
---@return Vec3
function DVec3.as_vec3(_self) end



---@class IVec2 : ReflectReference
---@field  x? integer
---@field  y? integer
---@operator div(IVec2 | integer): IVec2
---@operator add(IVec2 | integer): IVec2
---@operator mod(IVec2 | integer): IVec2
---@operator unm: IVec2
---@operator mul(IVec2 | integer): IVec2
---@operator sub(IVec2 | integer): IVec2
local IVec2 = {}

---@param _self IVec2
---@return IVec2
function IVec2.abs(_self) end

---@param _self IVec2
---@param other IVec2
---@return boolean
function IVec2.__eq(_self,other) end

---@param _self IVec2
---@param other IVec2
---@return boolean
function IVec2.eq(_self,other) end

---@param _self IVec2
---@param rhs IVec2 | integer
---@return IVec2
function IVec2.div(_self,rhs) end

---@param _self IVec2
---@return Vec2
function IVec2.as_vec2(_self) end

---@param _self IVec2
---@return integer
function IVec2.max_element(_self) end

---@param x integer
---@param y integer
---@return IVec2
function IVec2.new(x,y) end

---@param _self IVec2
---@param rhs IVec2 | integer
---@return IVec2
function IVec2.add(_self,rhs) end

---@param v integer
---@return IVec2
function IVec2.splat(v) end

---@param _self IVec2
---@return integer
function IVec2.min_element(_self) end

---@param _self IVec2
---@param rhs IVec2 | integer
---@return IVec2
function IVec2.rem(_self,rhs) end

---@param _self IVec2
---@param other IVec2
---@return IVec2
function IVec2.min(_self,other) end

---@param _self IVec2
---@param min IVec2
---@param max IVec2
---@return IVec2
function IVec2.clamp(_self,min,max) end

---@return IVec2
function IVec2.one() end

---@param _self IVec2
---@param other IVec2
---@return integer
function IVec2.dot(_self,other) end

---@return IVec2
function IVec2.unit_x() end

---@param _self IVec2
---@return IVec2
function IVec2.neg(_self) end

---@return IVec2
function IVec2.unit_y() end

---@param _self IVec2
---@param rhs IVec2 | integer
---@return IVec2
function IVec2.mul(_self,rhs) end

---@param _self IVec2
---@return integer[]
function IVec2.to_array(_self) end

---@return IVec2
function IVec2.zero() end

---@param _self IVec2
---@param other IVec2
---@return IVec2
function IVec2.max(_self,other) end

---@param _self IVec2
---@param rhs IVec2 | integer
---@return IVec2
function IVec2.sub(_self,rhs) end

---@param _self IVec2
---@return integer
function IVec2.length_squared(_self) end



---@class IVec3 : ReflectReference
---@field  x? integer
---@field  y? integer
---@field  z? integer
---@operator add(IVec3 | integer): IVec3
---@operator unm: IVec3
---@operator div(IVec3 | integer): IVec3
---@operator mul(IVec3 | integer): IVec3
---@operator mod(IVec3 | integer): IVec3
---@operator sub(IVec3 | integer): IVec3
local IVec3 = {}

---@param _self IVec3
---@return integer
function IVec3.max_element(_self) end

---@param _self IVec3
---@param rhs IVec3 | integer
---@return IVec3
function IVec3.add(_self,rhs) end

---@param v integer
---@return IVec3
function IVec3.splat(v) end

---@param _self IVec3
---@return IVec3
function IVec3.neg(_self) end

---@param _self IVec3
---@param rhs IVec3 | integer
---@return IVec3
function IVec3.div(_self,rhs) end

---@param _self IVec3
---@return integer
function IVec3.length_squared(_self) end

---@param _self IVec3
---@param rhs IVec3 | integer
---@return IVec3
function IVec3.mul(_self,rhs) end

---@param _self IVec3
---@param rhs IVec3 | integer
---@return IVec3
function IVec3.rem(_self,rhs) end

---@return IVec3
function IVec3.unit_x() end

---@param _self IVec3
---@return integer
function IVec3.min_element(_self) end

---@param _self IVec3
---@param other IVec3
---@return IVec3
function IVec3.max(_self,other) end

---@param _self IVec3
---@param other IVec3
---@return IVec3
function IVec3.min(_self,other) end

---@return IVec3
function IVec3.unit_z() end

---@param x integer
---@param y integer
---@param z integer
---@return IVec3
function IVec3.new(x,y,z) end

---@param _self IVec3
---@return IVec3
function IVec3.abs(_self) end

---@param _self IVec3
---@param other IVec3
---@return boolean
function IVec3.__eq(_self,other) end

---@param _self IVec3
---@param other IVec3
---@return boolean
function IVec3.eq(_self,other) end

---@return IVec3
function IVec3.zero() end

---@param _self IVec3
---@return Vec3
function IVec3.as_vec3(_self) end

---@return IVec3
function IVec3.one() end

---@return IVec3
function IVec3.unit_y() end

---@param _self IVec3
---@param min IVec3
---@param max IVec3
---@return IVec3
function IVec3.clamp(_self,min,max) end

---@param _self IVec3
---@param rhs IVec3 | integer
---@return IVec3
function IVec3.sub(_self,rhs) end

---@param _self IVec3
---@return integer[]
function IVec3.to_array(_self) end

---@param _self IVec3
---@param other IVec3
---@return integer
function IVec3.dot(_self,other) end



---@class Mat3 : ReflectReference
---@field  x_axis? Vec3
---@field  y_axis? Vec3
---@field  z_axis? Vec3
---@operator mul(Mat3 | Vec3 | number): Mat3 | Vec3
---@operator div(number): Mat3
---@operator add(Mat3): Mat3
---@operator sub(Mat3): Mat3
local Mat3 = {}

---@param _self Mat3
---@param rhs Mat3 | Vec3 | number
---@return Mat3 | Vec3
function Mat3.mul(_self,rhs) end

---@param x Vec3
---@param y Vec3
---@param z Vec3
---@return Mat3
function Mat3.from_cols(x,y,z) end

---@param _self Mat3
---@param index integer
---@return Vec3
function Mat3.row(_self,index) end

---@param _self Mat3
---@param point Vec2
---@return Vec2
function Mat3.transform_point2(_self,point) end

---@param diagonal Vec3
---@return Mat3
function Mat3.from_diagonal(diagonal) end

---@param _self Mat3
---@return Mat3
function Mat3.transpose(_self) end

---@param _self Mat3
---@param rhs number
---@return Mat3
function Mat3.div(_self,rhs) end

---@param _self Mat3
---@return number
function Mat3.determinant(_self) end

---@param _self Mat3
---@return Mat3
function Mat3.inverse(_self) end

---@param angle number
---@return Mat3
function Mat3.from_rotation_y(angle) end

---@param angle number
---@return Mat3
function Mat3.from_rotation_x(angle) end

---@param _self Mat3
---@param other Mat3
---@return boolean
function Mat3.__eq(_self,other) end

---@param _self Mat3
---@param other Mat3
---@return boolean
function Mat3.eq(_self,other) end

---@param angle number
---@return Mat3
function Mat3.from_rotation_z(angle) end

---@param _self Mat3
---@param other Mat3
---@return Mat3
function Mat3.add(_self,other) end

---@param _self Mat3
---@param other Mat3
---@return Mat3
function Mat3.sub(_self,other) end

---@param q Quat
---@return Mat3
function Mat3.from_quat(q) end

---@param _self Mat3
---@param index integer
---@return Vec3
function Mat3.col(_self,index) end

---@param _self Mat3
---@param v Vec3
---@return Vec3
function Mat3.mul_vec3(_self,v) end

---@return Mat3
function Mat3.identity() end

---@param _self Mat3
---@param vector Vec2
---@return Vec2
function Mat3.transform_vector2(_self,vector) end

---@param payload Mat3.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Mat3
function Mat3.new(payload) end



---@class Mat4 : ReflectReference
---@field  x_axis? Vec4
---@field  y_axis? Vec4
---@field  z_axis? Vec4
---@field  w_axis? Vec4
---@operator mul(Mat4 | Vec4 | number): Mat4 | Vec4
---@operator div(number): Mat4
---@operator sub(Mat4): Mat4
---@operator add(Mat4): Mat4
local Mat4 = {}

---@param _self Mat4
---@param vector Vec3
---@return Vec3
function Mat4.transform_vector3(_self,vector) end

---@param _self Mat4
---@param rhs Mat4 | Vec4 | number
---@return Mat4 | Vec4
function Mat4.mul(_self,rhs) end

---@param _self Mat4
---@param rhs number
---@return Mat4
function Mat4.div(_self,rhs) end

---@param _self Mat4
---@param other Mat4
---@return Mat4
function Mat4.sub(_self,other) end

---@param _self Mat4
---@param index integer
---@return Vec4
function Mat4.col(_self,index) end

---@param scale Vec3
---@param rotation Quat
---@param translation Vec3
---@return Mat4
function Mat4.from_scale_rotation_translation(scale,rotation,translation) end

---@param _self Mat4
---@param index integer
---@return Vec4
function Mat4.row(_self,index) end

---@param _self Mat4
---@param other Mat4
---@return boolean
function Mat4.__eq(_self,other) end

---@param _self Mat4
---@param other Mat4
---@return boolean
function Mat4.eq(_self,other) end

---@param scale Vec3
---@return Mat4
function Mat4.from_scale(scale) end

---@param rotation Quat
---@param translation Vec3
---@return Mat4
function Mat4.from_rotation_translation(rotation,translation) end

---@param x Vec4
---@param y Vec4
---@param z Vec4
---@param w Vec4
---@return Mat4
function Mat4.from_cols(x,y,z,w) end

---@param _self Mat4
---@param point Vec3
---@return Vec3
function Mat4.project_point3(_self,point) end

---@param _self Mat4
---@return Mat4
function Mat4.inverse(_self) end

---@param q Quat
---@return Mat4
function Mat4.from_quat(q) end

---@param _self Mat4
---@param point Vec3
---@return Vec3
function Mat4.transform_point3(_self,point) end

---@param translation Vec3
---@return Mat4
function Mat4.from_translation(translation) end

---@param _self Mat4
---@param other Mat4
---@return Mat4
function Mat4.add(_self,other) end

---@param _self Mat4
---@return number
function Mat4.determinant(_self) end

---@return Mat4
function Mat4.identity() end

---@param _self Mat4
---@return Mat4
function Mat4.transpose(_self) end

---@param payload Mat4.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Mat4
function Mat4.new(payload) end



---@class Quat : ReflectReference
---@field  x? number
---@field  y? number
---@field  z? number
---@field  w? number
---@operator unm: Quat
---@operator mul(Quat | Vec3): Quat | Vec3
local Quat = {}

---@param _self Quat
---@return Quat
function Quat.neg(_self) end

---@param _self Quat
---@return Quat
function Quat.normalize(_self) end

---@param from Vec3
---@param to Vec3
---@return Quat
function Quat.from_rotation_arc(from,to) end

---@param _self Quat
---@param other Quat
---@param t number
---@return Quat
function Quat.lerp(_self,other,t) end

---@param _self Quat
---@param other Quat
---@return number
function Quat.angle_between(_self,other) end

---@param _self Quat
---@return Vec3
function Quat.axis(_self) end

---@param _self Quat
---@return Quat
function Quat.conjugate(_self) end

---@param _self Quat
---@param other Quat
---@param t number
---@return Quat
function Quat.slerp(_self,other,t) end

---@param _self Quat
---@param other Quat
---@return number
function Quat.dot(_self,other) end

---@param _self Quat
---@param v Vec3
---@return Vec3
function Quat.mul_vec3(_self,v) end

---@param axis Vec3
---@param angle number
---@return Quat
function Quat.from_axis_angle(axis,angle) end

---@param _self Quat
---@return number
function Quat.angle(_self) end

---@param angle number
---@return Quat
function Quat.from_rotation_z(angle) end

---@param _self Quat
---@return Quat
function Quat.inverse(_self) end

---@param _self Quat
---@return boolean
function Quat.is_normalized(_self) end

---@return Quat
function Quat.identity() end

---@param _self Quat
---@return Vec3
function Quat.xyz(_self) end

---@param angle number
---@return Quat
function Quat.from_rotation_y(angle) end

---@param _self Quat
---@param order EulerRot
---@return number[]
function Quat.to_euler(_self,order) end

---@param _self Quat
---@param other Quat
---@return boolean
function Quat.__eq(_self,other) end

---@param _self Quat
---@param other Quat
---@return boolean
function Quat.eq(_self,other) end

---@param order EulerRot
---@param a number
---@param b number
---@param c number
---@return Quat
function Quat.from_euler(order,a,b,c) end

---@param _self Quat
---@param rhs Quat | Vec3
---@return Quat | Vec3
function Quat.mul(_self,rhs) end

---@param _self Quat
---@return number
function Quat.length(_self) end

---@param angle number
---@return Quat
function Quat.from_rotation_x(angle) end

---@param m Mat3
---@return Quat
function Quat.from_mat3(m) end

---@param payload Quat.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Quat
function Quat.new(payload) end



---@class UVec2 : ReflectReference
---@field  x? integer
---@field  y? integer
---@operator sub(UVec2 | integer): UVec2
---@operator mul(UVec2 | integer): UVec2
---@operator div(UVec2 | integer): UVec2
---@operator add(UVec2 | integer): UVec2
---@operator mod(UVec2 | integer): UVec2
local UVec2 = {}

---@param _self UVec2
---@param rhs UVec2 | integer
---@return UVec2
function UVec2.sub(_self,rhs) end

---@param _self UVec2
---@param other UVec2
---@return integer
function UVec2.dot(_self,other) end

---@return UVec2
function UVec2.one() end

---@param _self UVec2
---@param other UVec2
---@return UVec2
function UVec2.min(_self,other) end

---@param _self UVec2
---@param other UVec2
---@return boolean
function UVec2.__eq(_self,other) end

---@param _self UVec2
---@param other UVec2
---@return boolean
function UVec2.eq(_self,other) end

---@param _self UVec2
---@return integer
function UVec2.max_element(_self) end

---@return UVec2
function UVec2.zero() end

---@param _self UVec2
---@param rhs UVec2 | integer
---@return UVec2
function UVec2.mul(_self,rhs) end

---@return UVec2
function UVec2.unit_x() end

---@return UVec2
function UVec2.unit_y() end

---@param _self UVec2
---@return integer
function UVec2.min_element(_self) end

---@param _self UVec2
---@return Vec2
function UVec2.as_vec2(_self) end

---@param _self UVec2
---@param min UVec2
---@param max UVec2
---@return UVec2
function UVec2.clamp(_self,min,max) end

---@param _self UVec2
---@return integer
function UVec2.length_squared(_self) end

---@param v integer
---@return UVec2
function UVec2.splat(v) end

---@param x integer
---@param y integer
---@return UVec2
function UVec2.new(x,y) end

---@param _self UVec2
---@param other UVec2
---@return UVec2
function UVec2.max(_self,other) end

---@param _self UVec2
---@param rhs UVec2 | integer
---@return UVec2
function UVec2.div(_self,rhs) end

---@param _self UVec2
---@param rhs UVec2 | integer
---@return UVec2
function UVec2.add(_self,rhs) end

---@param _self UVec2
---@return integer[]
function UVec2.to_array(_self) end

---@param _self UVec2
---@param rhs UVec2 | integer
---@return UVec2
function UVec2.rem(_self,rhs) end



---@class UVec3 : ReflectReference
---@field  x? integer
---@field  y? integer
---@field  z? integer
---@operator mod(UVec3 | integer): UVec3
---@operator sub(UVec3 | integer): UVec3
---@operator div(UVec3 | integer): UVec3
---@operator mul(UVec3 | integer): UVec3
---@operator add(UVec3 | integer): UVec3
local UVec3 = {}

---@return UVec3
function UVec3.one() end

---@param _self UVec3
---@param rhs UVec3 | integer
---@return UVec3
function UVec3.rem(_self,rhs) end

---@param v integer
---@return UVec3
function UVec3.splat(v) end

---@param x integer
---@param y integer
---@param z integer
---@return UVec3
function UVec3.new(x,y,z) end

---@param _self UVec3
---@return integer
function UVec3.max_element(_self) end

---@param _self UVec3
---@param other UVec3
---@return integer
function UVec3.dot(_self,other) end

---@param _self UVec3
---@param rhs UVec3 | integer
---@return UVec3
function UVec3.sub(_self,rhs) end

---@return UVec3
function UVec3.unit_x() end

---@param _self UVec3
---@return integer
function UVec3.length_squared(_self) end

---@return UVec3
function UVec3.unit_z() end

---@param _self UVec3
---@param rhs UVec3 | integer
---@return UVec3
function UVec3.div(_self,rhs) end

---@param _self UVec3
---@return integer
function UVec3.min_element(_self) end

---@param _self UVec3
---@return integer[]
function UVec3.to_array(_self) end

---@param _self UVec3
---@param other UVec3
---@return boolean
function UVec3.__eq(_self,other) end

---@param _self UVec3
---@param other UVec3
---@return boolean
function UVec3.eq(_self,other) end

---@param _self UVec3
---@return Vec3
function UVec3.as_vec3(_self) end

---@param _self UVec3
---@param rhs UVec3 | integer
---@return UVec3
function UVec3.mul(_self,rhs) end

---@return UVec3
function UVec3.unit_y() end

---@param _self UVec3
---@param other UVec3
---@return UVec3
function UVec3.min(_self,other) end

---@param _self UVec3
---@param rhs UVec3 | integer
---@return UVec3
function UVec3.add(_self,rhs) end

---@param _self UVec3
---@param min UVec3
---@param max UVec3
---@return UVec3
function UVec3.clamp(_self,min,max) end

---@param _self UVec3
---@param other UVec3
---@return UVec3
function UVec3.max(_self,other) end

---@return UVec3
function UVec3.zero() end



---@class Vec2 : ReflectReference
---@field  x? number
---@field  y? number
---@operator div(Vec2 | number): Vec2
---@operator unm: Vec2
---@operator sub(Vec2 | number): Vec2
---@operator mul(Vec2 | number): Vec2
---@operator add(Vec2 | number): Vec2
---@operator mod(Vec2 | number): Vec2
local Vec2 = {}

---@param v number
---@return Vec2
function Vec2.splat(v) end

---@param x number
---@param y number
---@return Vec2
function Vec2.new(x,y) end

---@param _self Vec2
---@return Vec2
function Vec2.perp(_self) end

---@param _self Vec2
---@param z number
---@return Vec3
function Vec2.extend(_self,z) end

---@return Vec2
function Vec2.one() end

---@param _self Vec2
---@param other Vec2
---@return Vec2
function Vec2.rotate(_self,other) end

---@param _self Vec2
---@param rhs Vec2 | number
---@return Vec2
function Vec2.div(_self,rhs) end

---@param _self Vec2
---@return Vec2
function Vec2.neg(_self) end

---@param _self Vec2
---@return number
function Vec2.length_squared(_self) end

---@param _self Vec2
---@param normal Vec2
---@return Vec2
function Vec2.reflect(_self,normal) end

---@param _self Vec2
---@param rhs Vec2 | number
---@return Vec2
function Vec2.sub(_self,rhs) end

---@param _self Vec2
---@param other Vec2
---@return Vec2
function Vec2.max(_self,other) end

---@param angle number
---@return Vec2
function Vec2.from_angle(angle) end

---@param _self Vec2
---@return Vec2
function Vec2.abs(_self) end

---@param _self Vec2
---@param other Vec2
---@return number
function Vec2.distance_squared(_self,other) end

---@param _self Vec2
---@param min Vec2
---@param max Vec2
---@return Vec2
function Vec2.clamp(_self,min,max) end

---@param _self Vec2
---@return Vec2
function Vec2.round(_self) end

---@param _self Vec2
---@param min number
---@param max number
---@return Vec2
function Vec2.clamp_length(_self,min,max) end

---@param _self Vec2
---@param target Vec2
---@param max_distance number
---@return Vec2
function Vec2.move_towards(_self,target,max_distance) end

---@param _self Vec2
---@return number
function Vec2.length(_self) end

---@param _self Vec2
---@return number
function Vec2.min_element(_self) end

---@param _self Vec2
---@param rhs Vec2 | number
---@return Vec2
function Vec2.mul(_self,rhs) end

---@param _self Vec2
---@param rhs Vec2 | number
---@return Vec2
function Vec2.add(_self,rhs) end

---@param _self Vec2
---@return Vec2
function Vec2.normalize(_self) end

---@param _self Vec2
---@return UVec2
function Vec2.as_uvec2(_self) end

---@param _self Vec2
---@return IVec2
function Vec2.as_ivec2(_self) end

---@param _self Vec2
---@param other Vec2
---@return number
function Vec2.distance(_self,other) end

---@return Vec2
function Vec2.unit_y() end

---@param _self Vec2
---@param lhs number
---@return Vec2
function Vec2.rsub(_self,lhs) end

---@param _self Vec2
---@param other Vec2
---@return number
function Vec2.angle_to(_self,other) end

---@param _self Vec2
---@return Vec2
function Vec2.normalize_or_zero(_self) end

---@param _self Vec2
---@param lhs number
---@return Vec2
function Vec2.rdiv(_self,lhs) end

---@param _self Vec2
---@param other Vec2
---@return Vec2
function Vec2.reject_from(_self,other) end

---@param _self Vec2
---@return number
function Vec2.to_angle(_self) end

---@param _self Vec2
---@param other Vec2
---@return Vec2
function Vec2.project_onto(_self,other) end

---@param _self Vec2
---@param other Vec2
---@param t number
---@return Vec2
function Vec2.lerp(_self,other,t) end

---@param _self Vec2
---@return Vec2
function Vec2.ceil(_self) end

---@param _self Vec2
---@param other Vec2
---@return Vec2
function Vec2.min(_self,other) end

---@param _self Vec2
---@param other Vec2
---@return number
function Vec2.dot(_self,other) end

---@param _self Vec2
---@param max number
---@return Vec2
function Vec2.clamp_length_max(_self,max) end

---@param _self Vec2
---@return DVec2
function Vec2.as_dvec2(_self) end

---@param _self Vec2
---@return Vec2
function Vec2.floor(_self) end

---@param _self Vec2
---@return Vec2 | nil
function Vec2.try_normalize(_self) end

---@param _self Vec2
---@param other Vec2
---@return boolean
function Vec2.__eq(_self,other) end

---@param _self Vec2
---@param other Vec2
---@return boolean
function Vec2.eq(_self,other) end

---@param _self Vec2
---@return number[]
function Vec2.to_array(_self) end

---@param _self Vec2
---@return Vec2
function Vec2.fract(_self) end

---@param _self Vec2
---@param other Vec2
---@return number
function Vec2.perp_dot(_self,other) end

---@param _self Vec2
---@return number
function Vec2.max_element(_self) end

---@param _self Vec2
---@param rhs Vec2 | number
---@return Vec2
function Vec2.rem(_self,rhs) end

---@return Vec2
function Vec2.zero() end

---@return Vec2
function Vec2.unit_x() end

---@param _self Vec2
---@return Vec2
function Vec2.signum(_self) end



---@class Vec3 : ReflectReference
---@field  x? number
---@field  y? number
---@field  z? number
---@operator div(Vec3 | number): Vec3
---@operator mod(Vec3 | number): Vec3
---@operator unm: Vec3
---@operator sub(Vec3 | number): Vec3
---@operator mul(Vec3 | number): Vec3
---@operator add(Vec3 | number): Vec3
local Vec3 = {}

---@param _self Vec3
---@return number
function Vec3.length_squared(_self) end

---@param _self Vec3
---@return UVec3
function Vec3.as_uvec3(_self) end

---@param _self Vec3
---@param other Vec3
---@return number
function Vec3.angle_between(_self,other) end

---@param _self Vec3
---@return Vec2
function Vec3.truncate(_self) end

---@param _self Vec3
---@return Vec3
function Vec3.ceil(_self) end

---@param _self Vec3
---@return Vec3 | nil
function Vec3.try_normalize(_self) end

---@param _self Vec3
---@return Vec3
function Vec3.fract(_self) end

---@param _self Vec3
---@param other Vec3
---@return Vec3
function Vec3.max(_self,other) end

---@param _self Vec3
---@param max number
---@return Vec3
function Vec3.clamp_length_max(_self,max) end

---@param _self Vec3
---@param lhs number
---@return Vec3
function Vec3.rsub(_self,lhs) end

---@param _self Vec3
---@param other Vec3
---@return Vec3
function Vec3.min(_self,other) end

---@param _self Vec3
---@param min number
---@param max number
---@return Vec3
function Vec3.clamp_length(_self,min,max) end

---@param _self Vec3
---@return IVec3
function Vec3.as_ivec3(_self) end

---@param _self Vec3
---@param w number
---@return Vec4
function Vec3.extend(_self,w) end

---@param _self Vec3
---@param target Vec3
---@param max_distance number
---@return Vec3
function Vec3.move_towards(_self,target,max_distance) end

---@param _self Vec3
---@param rhs Vec3 | number
---@return Vec3
function Vec3.div(_self,rhs) end

---@param _self Vec3
---@param other Vec3
---@return Vec3
function Vec3.project_onto(_self,other) end

---@param _self Vec3
---@return Vec3
function Vec3.round(_self) end

---@param _self Vec3
---@param other Vec3
---@return number
function Vec3.distance(_self,other) end

---@param _self Vec3
---@return Vec3A
function Vec3.to_vec3a(_self) end

---@return Vec3
function Vec3.one() end

---@param _self Vec3
---@param other Vec3
---@return number
function Vec3.distance_squared(_self,other) end

---@param _self Vec3
---@return Vec3
function Vec3.normalize_or_zero(_self) end

---@param v number
---@return Vec3
function Vec3.splat(v) end

---@param _self Vec3
---@param normal Vec3
---@return Vec3
function Vec3.reflect(_self,normal) end

---@param _self Vec3
---@return number[]
function Vec3.to_array(_self) end

---@param _self Vec3
---@param other Vec3
---@return number
function Vec3.dot(_self,other) end

---@param _self Vec3
---@return Vec3
function Vec3.normalize(_self) end

---@param _self Vec3
---@return number
function Vec3.max_element(_self) end

---@param _self Vec3
---@return Vec3
function Vec3.signum(_self) end

---@param _self Vec3
---@param other Vec3
---@return boolean
function Vec3.__eq(_self,other) end

---@param _self Vec3
---@param other Vec3
---@return boolean
function Vec3.eq(_self,other) end

---@param _self Vec3
---@param min Vec3
---@param max Vec3
---@return Vec3
function Vec3.clamp(_self,min,max) end

---@param _self Vec3
---@param rhs Vec3 | number
---@return Vec3
function Vec3.rem(_self,rhs) end

---@param _self Vec3
---@return DVec3
function Vec3.as_dvec3(_self) end

---@param _self Vec3
---@return Vec3
function Vec3.neg(_self) end

---@return Vec3
function Vec3.zero() end

---@return Vec3
function Vec3.unit_z() end

---@param _self Vec3
---@param lhs number
---@return Vec3
function Vec3.rdiv(_self,lhs) end

---@param _self Vec3
---@return Vec3
function Vec3.floor(_self) end

---@param _self Vec3
---@param other Vec3
---@return Vec3
function Vec3.cross(_self,other) end

---@param _self Vec3
---@param rhs Vec3 | number
---@return Vec3
function Vec3.sub(_self,rhs) end

---@param _self Vec3
---@return Vec3
function Vec3.any_orthogonal_vector(_self) end

---@param x number
---@param y number
---@param z number
---@return Vec3
function Vec3.new(x,y,z) end

---@param _self Vec3
---@param other Vec3
---@return Vec3
function Vec3.reject_from(_self,other) end

---@param _self Vec3
---@param rhs Vec3 | number
---@return Vec3
function Vec3.mul(_self,rhs) end

---@param _self Vec3
---@return number
function Vec3.min_element(_self) end

---@return Vec3
function Vec3.unit_y() end

---@param _self Vec3
---@param other Vec3
---@param t number
---@return Vec3
function Vec3.lerp(_self,other,t) end

---@param _self Vec3
---@return Vec3
function Vec3.abs(_self) end

---@param _self Vec3
---@return number
function Vec3.length(_self) end

---@param _self Vec3
---@param rhs Vec3 | number
---@return Vec3
function Vec3.add(_self,rhs) end

---@return Vec3
function Vec3.unit_x() end



---@class Vec3A : ReflectReference
---@field  x? number
---@field  y? number
---@field  z? number
local Vec3A = {}

---@param x number
---@param y number
---@param z number
---@return Vec3A
function Vec3A.new(x,y,z) end

---@param _self Vec3A
---@return Vec3
function Vec3A.to_vec3(_self) end



---@class Vec4 : ReflectReference
---@field  x? number
---@field  y? number
---@field  z? number
---@field  w? number
---@operator div(Vec4 | number): Vec4
---@operator unm: Vec4
---@operator mod(Vec4 | number): Vec4
---@operator add(Vec4 | number): Vec4
---@operator mul(Vec4 | number): Vec4
---@operator sub(Vec4 | number): Vec4
local Vec4 = {}

---@param _self Vec4
---@param other Vec4
---@return Vec4
function Vec4.min(_self,other) end

---@param _self Vec4
---@param other Vec4
---@param t number
---@return Vec4
function Vec4.lerp(_self,other,t) end

---@param _self Vec4
---@param min number
---@param max number
---@return Vec4
function Vec4.clamp_length(_self,min,max) end

---@param x number
---@param y number
---@param z number
---@param w number
---@return Vec4
function Vec4.new(x,y,z,w) end

---@param _self Vec4
---@param other Vec4
---@return Vec4
function Vec4.reject_from(_self,other) end

---@return Vec4
function Vec4.one() end

---@param _self Vec4
---@return number
function Vec4.max_element(_self) end

---@param _self Vec4
---@return Vec4 | nil
function Vec4.try_normalize(_self) end

---@param _self Vec4
---@param other Vec4
---@return boolean
function Vec4.__eq(_self,other) end

---@param _self Vec4
---@param other Vec4
---@return boolean
function Vec4.eq(_self,other) end

---@param _self Vec4
---@param lhs number
---@return Vec4
function Vec4.rdiv(_self,lhs) end

---@param _self Vec4
---@return Vec4
function Vec4.signum(_self) end

---@param _self Vec4
---@return Vec4
function Vec4.normalize_or_zero(_self) end

---@param _self Vec4
---@param other Vec4
---@return number
function Vec4.distance(_self,other) end

---@param _self Vec4
---@param other Vec4
---@return number
function Vec4.dot(_self,other) end

---@param _self Vec4
---@return Vec3
function Vec4.truncate(_self) end

---@param _self Vec4
---@param rhs Vec4 | number
---@return Vec4
function Vec4.div(_self,rhs) end

---@param _self Vec4
---@return Vec4
function Vec4.neg(_self) end

---@return Vec4
function Vec4.zero() end

---@param _self Vec4
---@return number
function Vec4.length_squared(_self) end

---@param _self Vec4
---@return number
function Vec4.length(_self) end

---@param _self Vec4
---@return Vec4
function Vec4.abs(_self) end

---@param _self Vec4
---@param min Vec4
---@param max Vec4
---@return Vec4
function Vec4.clamp(_self,min,max) end

---@param _self Vec4
---@return number
function Vec4.min_element(_self) end

---@param _self Vec4
---@return Vec4
function Vec4.round(_self) end

---@param _self Vec4
---@param rhs Vec4 | number
---@return Vec4
function Vec4.rem(_self,rhs) end

---@param _self Vec4
---@return Vec4
function Vec4.normalize(_self) end

---@param _self Vec4
---@param lhs number
---@return Vec4
function Vec4.rsub(_self,lhs) end

---@param _self Vec4
---@param max number
---@return Vec4
function Vec4.clamp_length_max(_self,max) end

---@param _self Vec4
---@return Vec4
function Vec4.fract(_self) end

---@param _self Vec4
---@param target Vec4
---@param max_distance number
---@return Vec4
function Vec4.move_towards(_self,target,max_distance) end

---@param _self Vec4
---@param other Vec4
---@return Vec4
function Vec4.max(_self,other) end

---@param _self Vec4
---@param rhs Vec4 | number
---@return Vec4
function Vec4.add(_self,rhs) end

---@param _self Vec4
---@param other Vec4
---@return Vec4
function Vec4.project_onto(_self,other) end

---@param v number
---@return Vec4
function Vec4.splat(v) end

---@param _self Vec4
---@return Vec4
function Vec4.ceil(_self) end

---@param _self Vec4
---@param rhs Vec4 | number
---@return Vec4
function Vec4.mul(_self,rhs) end

---@param _self Vec4
---@param other Vec4
---@return number
function Vec4.distance_squared(_self,other) end

---@param _self Vec4
---@return number[]
function Vec4.to_array(_self) end

---@param _self Vec4
---@return Vec4
function Vec4.floor(_self) end

---@param _self Vec4
---@param normal Vec4
---@return Vec4
function Vec4.reflect(_self,normal) end

---@param _self Vec4
---@param rhs Vec4 | number
---@return Vec4
function Vec4.sub(_self,rhs) end



---@class NodeIndex : ReflectReference
local NodeIndex = {}

---@param index integer
---@return NodeIndex
function NodeIndex.new(index) end

---@param _self NodeIndex
---@return integer
function NodeIndex.index(_self) end



---@class Bool
--- A boolean value
local Bool = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Bool
function Bool.new(payload) end



---@class Char
--- An 8-bit character
local Char = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Char
function Char.new(payload) end



---@class DynamicFunction
--- A callable dynamic function
local DynamicFunction = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return DynamicFunction
function DynamicFunction.new(payload) end



---@class DynamicFunctionMut
--- A stateful and callable dynamic function
local DynamicFunctionMut = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return DynamicFunctionMut
function DynamicFunctionMut.new(payload) end



---@class F32
--- A 32-bit floating point number
local F32 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return F32
function F32.new(payload) end



---@class F64
--- A 64-bit floating point number
local F64 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return F64
function F64.new(payload) end



---@class FunctionCallContext
--- Function call context, if accepted by a function, means the function can access the world in arbitrary ways.
local FunctionCallContext = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FunctionCallContext
function FunctionCallContext.new(payload) end



---@class I128
--- A signed 128-bit integer
local I128 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return I128
function I128.new(payload) end



---@class I16
--- A signed 16-bit integer
local I16 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return I16
function I16.new(payload) end



---@class I32
--- A signed 32-bit integer
local I32 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return I32
function I32.new(payload) end



---@class I64
--- A signed 64-bit integer
local I64 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return I64
function I64.new(payload) end



---@class I8
--- A signed 8-bit integer
local I8 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return I8
function I8.new(payload) end



---@class Isize
--- A signed pointer-sized integer
local Isize = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Isize
function Isize.new(payload) end



---@class ReflectReference
--- A reference to a reflectable type
---@operator len: integer | nil
local ReflectReference = {}

---@param reference ReflectReference
--- The reference to insert the value into.
---@param key any
--- The index to insert the value at.
---@param value any
--- The value to insert.
---@return nil
function ReflectReference.insert(reference,key,value) end

---@param reference ReflectReference
--- The value to copy.
---@return any
function ReflectReference.clone(reference) end

---@param reference ReflectReference
--- The value to compare.
---@param other any
--- The value to compare with.
---@return boolean
function ReflectReference.__eq(reference,other) end

---@param reference ReflectReference
--- The value to compare.
---@param other any
--- The value to compare with.
---@return boolean
function ReflectReference.eq(reference,other) end

---@param reference ReflectReference
--- The reference to iterate over.
---@return function
function ReflectReference.__pairs(reference) end

---@param reference ReflectReference
--- The reference to iterate over.
---@return function
function ReflectReference.iter(reference) end

---@param reference ReflectReference
--- The reference to get the length of.
---@return integer | nil
function ReflectReference.len(reference) end

---@param reference ReflectReference
--- The reference to get the variant of.
---@return EnumVariantRef | nil
function ReflectReference.variant(reference) end

---@param reference ReflectReference
--- The reference to index into.
---@param key any
--- The key to index with.
---@return any | nil
function ReflectReference.map_get(reference,key) end

---@param reference ReflectReference
--- The reference to push the value into.
---@param value any
--- The value to push.
---@return nil
function ReflectReference.push(reference,value) end

---@param reference ReflectReference
--- The reference to remove the value from.
---@param key any
--- The key to remove the value at.
---@return any
function ReflectReference.remove(reference,key) end

---@param reference ReflectReference
--- The reference to get the variant name of.
---@return string | nil
function ReflectReference.variant_name(reference) end

---@param reference ReflectReference
--- The reference to display.
---@return string
function ReflectReference.__tostring(reference) end

---@param reference ReflectReference
--- The reference to display.
---@return string
function ReflectReference.debug(reference) end

---@param reference ReflectReference
--- The reference to pop the value from.
---@return any
function ReflectReference.pop(reference) end

---@param reference ReflectReference
--- The reference to clear.
---@return nil
function ReflectReference.clear(reference) end

---@param reference ReflectReference
--- The reference to list the functions of.
---@return FunctionInfo[]
function ReflectReference.functions(reference) end

---@param reference ReflectReference
--- The reference to display.
---@return string
function ReflectReference.__tostring(reference) end

---@param reference ReflectReference
--- The reference to display.
---@return string
function ReflectReference.display(reference) end

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReflectReference
function ReflectReference.new(payload) end



---@class ScriptValue : ReflectReference
---  An abstraction of values that can be passed to and from scripts.
---  This allows us to re-use logic between scripting languages.
local ScriptValue = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptValue
function ScriptValue.new(payload) end



---@class String
--- A heap allocated string
local String = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return String
function String.new(payload) end



---@class U128
--- An unsigned 128-bit integer
local U128 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return U128
function U128.new(payload) end



---@class U16
--- An unsigned 16-bit integer
local U16 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return U16
function U16.new(payload) end



---@class U32
--- An unsigned 32-bit integer
local U32 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return U32
function U32.new(payload) end



---@class U64
--- An unsigned 64-bit integer
local U64 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return U64
function U64.new(payload) end



---@class U8
--- An unsigned 8-bit integer
local U8 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return U8
function U8.new(payload) end



---@class Usize
--- An unsigned pointer-sized integer
local Usize = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Usize
function Usize.new(payload) end



---@class ActiveAnimation : ReflectReference
---  An animation that an [`AnimationPlayer`] is currently either playing or was
---  playing, but is presently paused.
--- 
---  A stopped animation is considered no longer active.
---@field  weight? number
---@field  repeat? RepeatAnimation
---@field  speed? number
---@field  elapsed? number
---@field  seek_time? number
---@field  last_seek_time? number | nil
---@field  completions? integer
---@field  just_completed? boolean
---@field  paused? boolean
local ActiveAnimation = {}

---@param payload ActiveAnimation.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ActiveAnimation
function ActiveAnimation.new(payload) end



---@class AnimationTargetId : ReflectReference
---  A component that identifies which parts of an [`AnimationClip`] asset can
---  be applied to an entity. Typically used alongside the
---  [`AnimatedBy`] component.
--- 
---  `AnimationTargetId` is implemented as a [UUID]. When importing an armature
---  or an animation clip, asset loaders typically use the full path name from
---  the armature to the bone to generate these UUIDs. The ID is unique to the
---  full path name and based only on the names. So, for example, any imported
---  armature with a bone at the root named `Hips` will assign the same
---  [`AnimationTargetId`] to its root bone. Likewise, any imported animation
---  clip that animates a root bone named `Hips` will reference the same
---  [`AnimationTargetId`]. Any animation is playable on any armature as long as
---  the bone names match, which allows for easy animation retargeting.
--- 
---  Note that asset loaders generally use the *full* path name to generate the
---  [`AnimationTargetId`]. Thus a bone named `Chest` directly connected to a
---  bone named `Hips` will have a different ID from a bone named `Chest` that's
---  connected to a bone named `Stomach`.
--- 
---  [UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier
---@field  [1] Uuid
local AnimationTargetId = {}

---@param payload AnimationTargetId.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AnimationTargetId
function AnimationTargetId.new(payload) end



---@class RepeatAnimation : ReflectReference
---  Repetition behavior of an animation.
---@field  Never RepeatAnimation.Never
---@field  Count RepeatAnimation.Count
---@field  Forever RepeatAnimation.Forever
local RepeatAnimation = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RepeatAnimation
function RepeatAnimation.new(payload) end



---@class RepeatAnimation.Never : EnumVariantRef
--- The `Never` variant of `RepeatAnimation`.
RepeatAnimation.Never = {}

---@param variant RepeatAnimation.Never
---@param payload NoFields
---@return RepeatAnimation
function RepeatAnimation.Never.new(variant,payload) end



---@class RepeatAnimation.Count : EnumVariantRef
--- The `Count` variant of `RepeatAnimation`.
RepeatAnimation.Count = {}

---@param variant RepeatAnimation.Count
---@param payload RepeatAnimation.Count.Fields
---@return RepeatAnimation
function RepeatAnimation.Count.new(variant,payload) end



---@class RepeatAnimation.Forever : EnumVariantRef
--- The `Forever` variant of `RepeatAnimation`.
RepeatAnimation.Forever = {}

---@param variant RepeatAnimation.Forever
---@param payload NoFields
---@return RepeatAnimation
function RepeatAnimation.Forever.new(variant,payload) end



---@class AssetIndex : ReflectReference
---  A generational runtime-only identifier for a specific [`Asset`] stored in [`Assets`]. This is optimized for efficient runtime
---  usage and is not suitable for identifying assets across app runs.
---@field  generation? integer
---@field  index? integer
local AssetIndex = {}

---@param payload AssetIndex.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AssetIndex
function AssetIndex.new(payload) end



---@class AssetPath : ReflectReference
---  Represents a path to an asset in a "virtual filesystem".
--- 
---  Asset paths consist of three main parts:
---  * [`AssetPath::source`]: The name of the [`AssetSource`](crate::io::AssetSource) to load the asset from.
---    This is optional. If one is not set the default source will be used (which is the `assets` folder by default).
---  * [`AssetPath::path`]: The "virtual filesystem path" pointing to an asset source file.
---  * [`AssetPath::label`]: An optional "named sub asset". When assets are loaded, they are
---    allowed to load "sub assets" of any type, which are identified by a named "label".
--- 
---  Asset paths are generally constructed (and visualized) as strings:
--- 
---  ```no_run
---  # use bevy_asset::{Asset, AssetServer, Handle};
---  # use bevy_reflect::TypePath;
---  #
---  # #[derive(Asset, TypePath, Default)]
---  # struct Mesh;
---  #
---  # #[derive(Asset, TypePath, Default)]
---  # struct Scene;
---  #
---  # let asset_server: AssetServer = panic!();
---  // This loads the `my_scene.scn` base asset from the default asset source.
---  let scene: Handle<Scene> = asset_server.load("my_scene.scn");
--- 
---  // This loads the `PlayerMesh` labeled asset from the `my_scene.scn` base asset in the default asset source.
---  let mesh: Handle<Mesh> = asset_server.load("my_scene.scn#PlayerMesh");
--- 
---  // This loads the `my_scene.scn` base asset from a custom 'remote' asset source.
---  let scene: Handle<Scene> = asset_server.load("remote://my_scene.scn");
---  ```
--- 
---  [`AssetPath`] implements [`From`] for `&'static str`, `&'static Path`, and `&'a String`,
---  which allows us to optimize the static cases.
---  This means that the common case of `asset_server.load("my_scene.scn")` when it creates and
---  clones internal owned [`AssetPaths`](AssetPath).
---  This also means that you should use [`AssetPath::parse`] in cases where `&str` is the explicit type.
local AssetPath = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AssetPath
function AssetPath.new(payload) end



---@class RenderAssetUsages : ReflectReference
---  Defines where the asset will be used.
--- 
---  If an asset is set to the `RENDER_WORLD` but not the `MAIN_WORLD`, the asset data (pixel data,
---  mesh vertex data, etc) will be removed from the cpu-side asset once it's been extracted and prepared
---  in the render world. The asset will remain in the assets collection, but with only metadata.
--- 
---  Unloading the asset data saves on memory, as for most cases it is no longer necessary to keep
---  it in RAM once it's been uploaded to the GPU's VRAM. However, this means you cannot access the
---  asset data from the CPU (via the `Assets<T>` resource) once unloaded (without re-loading it).
--- 
---  If you never need access to the asset from the CPU past the first frame it's loaded on,
---  or only need very infrequent access, then set this to `RENDER_WORLD`. Otherwise, set this to
---  `RENDER_WORLD | MAIN_WORLD`.
--- 
---  If you have an asset that doesn't actually need to end up in the render world, like an Image
---  that will be decoded into another Image asset, use `MAIN_WORLD` only.
--- 
---  ## Platform-specific
--- 
---  On Wasm, it is not possible for now to free reserved memory. To control memory usage, load assets
---  in sequence and unload one before loading the next. See this
---  [discussion about memory management](https://github.com/WebAssembly/design/issues/1397) for more
---  details.
local RenderAssetUsages = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RenderAssetUsages
function RenderAssetUsages.new(payload) end



---@class PlaybackMode : ReflectReference
---  The way Bevy manages the sound playback.
---@field  Once PlaybackMode.Once
---@field  Loop PlaybackMode.Loop
---@field  Despawn PlaybackMode.Despawn
---@field  Remove PlaybackMode.Remove
local PlaybackMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return PlaybackMode
function PlaybackMode.new(payload) end



---@class PlaybackMode.Once : EnumVariantRef
--- The `Once` variant of `PlaybackMode`.
PlaybackMode.Once = {}

---@param variant PlaybackMode.Once
---@param payload NoFields
---@return PlaybackMode
function PlaybackMode.Once.new(variant,payload) end



---@class PlaybackMode.Loop : EnumVariantRef
--- The `Loop` variant of `PlaybackMode`.
PlaybackMode.Loop = {}

---@param variant PlaybackMode.Loop
---@param payload NoFields
---@return PlaybackMode
function PlaybackMode.Loop.new(variant,payload) end



---@class PlaybackMode.Despawn : EnumVariantRef
--- The `Despawn` variant of `PlaybackMode`.
PlaybackMode.Despawn = {}

---@param variant PlaybackMode.Despawn
---@param payload NoFields
---@return PlaybackMode
function PlaybackMode.Despawn.new(variant,payload) end



---@class PlaybackMode.Remove : EnumVariantRef
--- The `Remove` variant of `PlaybackMode`.
PlaybackMode.Remove = {}

---@param variant PlaybackMode.Remove
---@param payload NoFields
---@return PlaybackMode
function PlaybackMode.Remove.new(variant,payload) end



---@class SpatialScale : ReflectReference
---  A scale factor applied to the positions of audio sources and listeners for
---  spatial audio.
--- 
---  Default is `Vec3::ONE`.
---@field  [1] Vec3
local SpatialScale = {}

---@param payload SpatialScale.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SpatialScale
function SpatialScale.new(payload) end



---@class Volume : ReflectReference
---  A [`Volume`] represents an audio source's volume level.
--- 
---  To create a new [`Volume`] from a linear scale value, use
---  [`Volume::Linear`].
--- 
---  To create a new [`Volume`] from decibels, use [`Volume::Decibels`].
---@field  Linear Volume.Linear
---@field  Decibels Volume.Decibels
local Volume = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Volume
function Volume.new(payload) end



---@class Volume.Linear : EnumVariantRef
--- The `Linear` variant of `Volume`.
Volume.Linear = {}

---@param variant Volume.Linear
---@param payload Volume.Linear.Fields
---@return Volume
function Volume.Linear.new(variant,payload) end



---@class Volume.Decibels : EnumVariantRef
--- The `Decibels` variant of `Volume`.
Volume.Decibels = {}

---@param variant Volume.Decibels
---@param payload Volume.Decibels.Fields
---@return Volume
function Volume.Decibels.new(variant,payload) end



---@class CameraOutputMode : ReflectReference
---  Control how this [`Camera`] outputs once rendering is completed.
---@field  Write CameraOutputMode.Write
---@field  Skip CameraOutputMode.Skip
local CameraOutputMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return CameraOutputMode
function CameraOutputMode.new(payload) end



---@class CameraOutputMode.Write : EnumVariantRef
--- The `Write` variant of `CameraOutputMode`.
CameraOutputMode.Write = {}

---@param variant CameraOutputMode.Write
---@param payload CameraOutputMode.Write.Fields
---@return CameraOutputMode
function CameraOutputMode.Write.new(variant,payload) end



---@class CameraOutputMode.Skip : EnumVariantRef
--- The `Skip` variant of `CameraOutputMode`.
CameraOutputMode.Skip = {}

---@param variant CameraOutputMode.Skip
---@param payload NoFields
---@return CameraOutputMode
function CameraOutputMode.Skip.new(variant,payload) end



---@class ComputedCameraValues : ReflectReference
---  Holds internally computed [`Camera`] values.
---@field  clip_from_view? Mat4
---@field  target_info? RenderTargetInfo | nil
---@field  old_viewport_size? UVec2 | nil
---@field  old_sub_camera_view? SubCameraView | nil
local ComputedCameraValues = {}

---@param payload ComputedCameraValues.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ComputedCameraValues
function ComputedCameraValues.new(payload) end



---@class ImageRenderTarget : ReflectReference
---  A render target that renders to an [`Image`].
---@field  handle? Handle<Image>
---@field  scale_factor? number
local ImageRenderTarget = {}

---@param payload ImageRenderTarget.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ImageRenderTarget
function ImageRenderTarget.new(payload) end



---@class ManualTextureViewHandle : ReflectReference
---  A unique id that corresponds to a specific `ManualTextureView` in the `ManualTextureViews` collection.
--- 
---  See `ManualTextureViews` in `bevy_camera` for more details.
---@field  [1] integer
local ManualTextureViewHandle = {}

---@param payload ManualTextureViewHandle.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ManualTextureViewHandle
function ManualTextureViewHandle.new(payload) end



---@class RenderTarget : ReflectReference
---  The "target" that a [`Camera`] will render to. For example, this could be a `Window`
---  swapchain or an [`Image`].
---@field  Window RenderTarget.Window
---@field  Image RenderTarget.Image
---@field  TextureView RenderTarget.TextureView
---@field  None RenderTarget.None
local RenderTarget = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RenderTarget
function RenderTarget.new(payload) end



---@class RenderTarget.Window : EnumVariantRef
--- The `Window` variant of `RenderTarget`.
RenderTarget.Window = {}

---@param variant RenderTarget.Window
---@param payload RenderTarget.Window.Fields
---@return RenderTarget
function RenderTarget.Window.new(variant,payload) end



---@class RenderTarget.Image : EnumVariantRef
--- The `Image` variant of `RenderTarget`.
RenderTarget.Image = {}

---@param variant RenderTarget.Image
---@param payload RenderTarget.Image.Fields
---@return RenderTarget
function RenderTarget.Image.new(variant,payload) end



---@class RenderTarget.TextureView : EnumVariantRef
--- The `TextureView` variant of `RenderTarget`.
RenderTarget.TextureView = {}

---@param variant RenderTarget.TextureView
---@param payload RenderTarget.TextureView.Fields
---@return RenderTarget
function RenderTarget.TextureView.new(variant,payload) end



---@class RenderTarget.None : EnumVariantRef
--- The `None` variant of `RenderTarget`.
RenderTarget.None = {}

---@param variant RenderTarget.None
---@param payload RenderTarget.None.Fields
---@return RenderTarget
function RenderTarget.None.new(variant,payload) end



---@class RenderTargetInfo : ReflectReference
---  Information about the current [`RenderTarget`].
---@field  physical_size? UVec2
---@field  scale_factor? number
local RenderTargetInfo = {}

---@param payload RenderTargetInfo.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RenderTargetInfo
function RenderTargetInfo.new(payload) end



---@class SubCameraView : ReflectReference
---  Settings to define a camera sub view.
--- 
---  When [`Camera::sub_camera_view`] is `Some`, only the sub-section of the
---  image defined by `size` and `offset` (relative to the `full_size` of the
---  whole image) is projected to the cameras viewport.
--- 
---  Take the example of the following multi-monitor setup:
---  ```css
---  ┌───┬───┐
---  │ A │ B │
---  ├───┼───┤
---  │ C │ D │
---  └───┴───┘
---  ```
---  If each monitor is 1920x1080, the whole image will have a resolution of
---  3840x2160. For each monitor we can use a single camera with a viewport of
---  the same size as the monitor it corresponds to. To ensure that the image is
---  cohesive, we can use a different sub view on each camera:
---  - Camera A: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 0,0
---  - Camera B: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 1920,0
---  - Camera C: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 0,1080
---  - Camera D: `full_size` = 3840x2160, `size` = 1920x1080, `offset` =
---    1920,1080
--- 
---  However since only the ratio between the values is important, they could all
---  be divided by 120 and still produce the same image. Camera D would for
---  example have the following values:
---  `full_size` = 32x18, `size` = 16x9, `offset` = 16,9
---@field  full_size? UVec2
---@field  offset? Vec2
---@field  size? UVec2
local SubCameraView = {}

---@param payload SubCameraView.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SubCameraView
function SubCameraView.new(payload) end



---@class Viewport : ReflectReference
---  Render viewport configuration for the [`Camera`] component.
--- 
---  The viewport defines the area on the render target to which the camera renders its image.
---  You can overlay multiple cameras in a single window using viewports to create effects like
---  split screen, minimaps, and character viewers.
--- 
---  <div class="warning">
--- 
---  Note that the physical position is in actual screen coordinates and not virtual pixels for window targets.  
---  You should use the scaling factor reported by the window, which on some OS's defaults to a value other than 1.
---  Please see the example code (which assumes a single camera and window)
--- 
---  ```no_run
---  # use bevy_camera::{Camera, Projection, Viewport};
---  # use bevy_transform::prelude::Transform;
---  # use bevy_ecs::prelude::*;
---  # use bevy_math::UVec2;
---  # use bevy_window::Window;
---  # use bevy_utils::default;
--- 
---  fn update_viewport(
---     mut camera_query: Query<(&mut Camera, &mut Transform, &mut Projection)>,
---     windows: Query<&Window>
---  ) {
---      let Ok((mut camera, _, _)) = camera_query.single_mut() else { return; };
--- 
---      let window = windows.single().expect("Window not found");
---      let scale = window.resolution.scale_factor();
--- 
---      camera.viewport = Some(Viewport {
---          physical_position: (UVec2::new(10, 10).as_vec2() * scale).as_uvec2(),
---          physical_size: (UVec2::new(100, 100).as_vec2() * scale).as_uvec2(),
---          ..default()
---      });
---  }
---  ```
--- 
---  </div>
---@field  physical_position? UVec2
---@field  physical_size? UVec2
---@field  depth? any
local Viewport = {}

---@param payload Viewport.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Viewport
function Viewport.new(payload) end



---@class ClearColorConfig : ReflectReference
---  For a camera, specifies the color used to clear the viewport
---  [before rendering](crate::camera::Camera::clear_color)
---  or when [writing to the final render target texture](crate::camera::Camera::output_mode).
---@field  Default ClearColorConfig.Default
---@field  Custom ClearColorConfig.Custom
---@field  None ClearColorConfig.None
local ClearColorConfig = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ClearColorConfig
function ClearColorConfig.new(payload) end



---@class ClearColorConfig.Default : EnumVariantRef
--- The `Default` variant of `ClearColorConfig`.
ClearColorConfig.Default = {}

---@param variant ClearColorConfig.Default
---@param payload NoFields
---@return ClearColorConfig
function ClearColorConfig.Default.new(variant,payload) end



---@class ClearColorConfig.Custom : EnumVariantRef
--- The `Custom` variant of `ClearColorConfig`.
ClearColorConfig.Custom = {}

---@param variant ClearColorConfig.Custom
---@param payload ClearColorConfig.Custom.Fields
---@return ClearColorConfig
function ClearColorConfig.Custom.new(variant,payload) end



---@class ClearColorConfig.None : EnumVariantRef
--- The `None` variant of `ClearColorConfig`.
ClearColorConfig.None = {}

---@param variant ClearColorConfig.None
---@param payload NoFields
---@return ClearColorConfig
function ClearColorConfig.None.new(variant,payload) end



---@class MsaaWriteback : ReflectReference
---  Controls when MSAA writeback occurs for a camera.
--- 
---  MSAA writeback copies the previous camera's output into the MSAA sampled texture before
---  rendering, allowing multiple cameras to layer their results when MSAA is enabled.
---@field  Off MsaaWriteback.Off
---@field  Auto MsaaWriteback.Auto
---@field  Always MsaaWriteback.Always
local MsaaWriteback = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MsaaWriteback
function MsaaWriteback.new(payload) end



---@class MsaaWriteback.Off : EnumVariantRef
--- The `Off` variant of `MsaaWriteback`.
MsaaWriteback.Off = {}

---@param variant MsaaWriteback.Off
---@param payload NoFields
---@return MsaaWriteback
function MsaaWriteback.Off.new(variant,payload) end



---@class MsaaWriteback.Auto : EnumVariantRef
--- The `Auto` variant of `MsaaWriteback`.
MsaaWriteback.Auto = {}

---@param variant MsaaWriteback.Auto
---@param payload NoFields
---@return MsaaWriteback
function MsaaWriteback.Auto.new(variant,payload) end



---@class MsaaWriteback.Always : EnumVariantRef
--- The `Always` variant of `MsaaWriteback`.
MsaaWriteback.Always = {}

---@param variant MsaaWriteback.Always
---@param payload NoFields
---@return MsaaWriteback
function MsaaWriteback.Always.new(variant,payload) end



---@class ScalingMode : ReflectReference
---  Scaling mode for [`OrthographicProjection`].
--- 
---  The effect of these scaling modes are combined with the [`OrthographicProjection::scale`] property.
--- 
---  For example, if the scaling mode is `ScalingMode::Fixed { width: 100.0, height: 300 }` and the scale is `2.0`,
---  the projection will be 200 world units wide and 600 world units tall.
--- 
---  # Examples
--- 
---  Configure the orthographic projection to two world units per window height:
--- 
---  ```
---  # use bevy_camera::{OrthographicProjection, Projection, ScalingMode};
---  let projection = Projection::Orthographic(OrthographicProjection {
---     scaling_mode: ScalingMode::FixedVertical { viewport_height: 2.0 },
---     ..OrthographicProjection::default_2d()
---  });
---  ```
---@field  WindowSize ScalingMode.WindowSize
---@field  Fixed ScalingMode.Fixed
---@field  AutoMin ScalingMode.AutoMin
---@field  AutoMax ScalingMode.AutoMax
---@field  FixedVertical ScalingMode.FixedVertical
---@field  FixedHorizontal ScalingMode.FixedHorizontal
local ScalingMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScalingMode
function ScalingMode.new(payload) end



---@class ScalingMode.WindowSize : EnumVariantRef
--- The `WindowSize` variant of `ScalingMode`.
ScalingMode.WindowSize = {}

---@param variant ScalingMode.WindowSize
---@param payload NoFields
---@return ScalingMode
function ScalingMode.WindowSize.new(variant,payload) end



---@class ScalingMode.Fixed : EnumVariantRef
--- The `Fixed` variant of `ScalingMode`.
ScalingMode.Fixed = {}

---@param variant ScalingMode.Fixed
---@param payload ScalingMode.Fixed.Fields
---@return ScalingMode
function ScalingMode.Fixed.new(variant,payload) end



---@class ScalingMode.AutoMin : EnumVariantRef
--- The `AutoMin` variant of `ScalingMode`.
ScalingMode.AutoMin = {}

---@param variant ScalingMode.AutoMin
---@param payload ScalingMode.AutoMin.Fields
---@return ScalingMode
function ScalingMode.AutoMin.new(variant,payload) end



---@class ScalingMode.AutoMax : EnumVariantRef
--- The `AutoMax` variant of `ScalingMode`.
ScalingMode.AutoMax = {}

---@param variant ScalingMode.AutoMax
---@param payload ScalingMode.AutoMax.Fields
---@return ScalingMode
function ScalingMode.AutoMax.new(variant,payload) end



---@class ScalingMode.FixedVertical : EnumVariantRef
--- The `FixedVertical` variant of `ScalingMode`.
ScalingMode.FixedVertical = {}

---@param variant ScalingMode.FixedVertical
---@param payload ScalingMode.FixedVertical.Fields
---@return ScalingMode
function ScalingMode.FixedVertical.new(variant,payload) end



---@class ScalingMode.FixedHorizontal : EnumVariantRef
--- The `FixedHorizontal` variant of `ScalingMode`.
ScalingMode.FixedHorizontal = {}

---@param variant ScalingMode.FixedHorizontal
---@param payload ScalingMode.FixedHorizontal.Fields
---@return ScalingMode
function ScalingMode.FixedHorizontal.new(variant,payload) end



---@class Okhsla : ReflectReference
---  Color in Okhsl color space with alpha.
---  Further information on this color model can be found on <https://bottosson.github.io/posts/colorpicker>.
--- 
---  Okhsl is defined relative to the sRGB (Rec. 709) gamut. Converting a wide-gamut or
---  HDR color clamps the lightness to `1.0` but can push saturation outside `[0.0, 1.0]`.
---  <div>
---  </div>
---@field  hue? number
---@field  saturation? number
---@field  lightness? number
---@field  alpha? number
local Okhsla = {}

---@param payload Okhsla.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Okhsla
function Okhsla.new(payload) end



---@class Okhsva : ReflectReference
---  Color in Okhsv color space with alpha.
---  Further information on this color model can be found on <https://bottosson.github.io/posts/colorpicker>.
--- 
---  Okhsv is defined relative to the sRGB (Rec. 709) gamut. Converting a wide-gamut or
---  HDR color can push saturation and value outside `[0.0, 1.0]`.
---  <div>
---  </div>
---@field  hue? number
---@field  saturation? number
---@field  value? number
---@field  alpha? number
local Okhsva = {}

---@param payload Okhsva.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Okhsva
function Okhsva.new(payload) end



---@class Okhwba : ReflectReference
---  Color in Okhwb color space with alpha.
---  Further information on this color model can be found on <https://bottosson.github.io/posts/colorpicker>.
---  <div>
---  </div>
---@field  hue? number
---@field  whiteness? number
---@field  blackness? number
---@field  alpha? number
local Okhwba = {}

---@param payload Okhwba.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Okhwba
function Okhwba.new(payload) end



---@class NoBackgroundMotionVectors : ReflectReference
---  When added to a camera with [`MotionVectorPrepass`], disables the automatic background motion
---  vector prepass.
--- 
---  By default, any camera with [`MotionVectorPrepass`] will automatically write camera-rotation
---  motion vectors for background pixels (those with no geometry, i.e. depth == 0 in reversed-Z).
---  Add this component to opt out, for example if you are writing custom background motion vectors
---  for your own effect.
local NoBackgroundMotionVectors = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NoBackgroundMotionVectors
function NoBackgroundMotionVectors.new(payload) end



---@class JumpAt : ReflectReference
---  Configuration options for the [`EaseFunction::Steps`] curves. This closely replicates the
---  [CSS step function specification].
--- 
---  [CSS step function specification]: https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description
---@field  Start JumpAt.Start
---@field  End JumpAt.End
---@field  None JumpAt.None
---@field  Both JumpAt.Both
local JumpAt = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JumpAt
function JumpAt.new(payload) end



---@class JumpAt.Start : EnumVariantRef
--- The `Start` variant of `JumpAt`.
JumpAt.Start = {}

---@param variant JumpAt.Start
---@param payload NoFields
---@return JumpAt
function JumpAt.Start.new(variant,payload) end



---@class JumpAt.End : EnumVariantRef
--- The `End` variant of `JumpAt`.
JumpAt.End = {}

---@param variant JumpAt.End
---@param payload NoFields
---@return JumpAt
function JumpAt.End.new(variant,payload) end



---@class JumpAt.None : EnumVariantRef
--- The `None` variant of `JumpAt`.
JumpAt.None = {}

---@param variant JumpAt.None
---@param payload NoFields
---@return JumpAt
function JumpAt.None.new(variant,payload) end



---@class JumpAt.Both : EnumVariantRef
--- The `Both` variant of `JumpAt`.
JumpAt.Both = {}

---@param variant JumpAt.Both
---@param payload NoFields
---@return JumpAt
function JumpAt.Both.new(variant,payload) end



---@class ComponentId : ReflectReference
---  A value which uniquely identifies the type of a [`Component`] or [`Resource`](crate::resource::Resource) within a
---  [`World`](crate::world::World).
--- 
---  Each time a new `Component` type is registered within a `World` using
---  e.g. [`World::register_component`](crate::world::World::register_component) or
---  [`World::register_component_with_descriptor`](crate::world::World::register_component_with_descriptor)
---  or a Resource with e.g. [`World::init_resource`](crate::world::World::init_resource),
---  a corresponding `ComponentId` is created to track it.
--- 
---  While the distinction between `ComponentId` and [`TypeId`] may seem superficial, breaking them
---  into two separate but related concepts allows components to exist outside of Rust's type system.
---  Each Rust type registered as a `Component` will have a corresponding `ComponentId`, but additional
---  `ComponentId`s may exist in a `World` to track components which cannot be
---  represented as Rust types for scripting or other advanced use-cases.
--- 
---  A `ComponentId` is tightly coupled to its parent `World`. Attempting to use a `ComponentId` from
---  one `World` to access the metadata of a `Component` in a different `World` is undefined behavior
---  and must not be attempted.
--- 
---  Given a type `T` which implements [`Component`] (including [`Resource`](crate::resource::Resource)), the `ComponentId` for `T` can be retrieved
---  from a `World` using [`World::component_id()`](crate::world::World::component_id) or via [`Components::component_id()`].
---@field  [1] integer
local ComponentId = {}

---@param payload ComponentId.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ComponentId
function ComponentId.new(payload) end



---@class AddEvent : ReflectReference
---  Trigger emitted when a component is inserted onto an entity that does not already have that
---  component. Runs before `Insert`.
---  See [`ComponentHooks::on_add`](`crate::lifecycle::ComponentHooks::on_add`) for more information.
---@field  entity? Entity
local AddEvent = {}

---@param payload AddEvent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AddEvent
function AddEvent.new(payload) end



---@class DespawnEvent : ReflectReference
---  [`EntityEvent`] emitted for each component on an entity when it is despawned.
---  See [`ComponentHooks::on_despawn`](`crate::lifecycle::ComponentHooks::on_despawn`) for more information.
---@field  entity? Entity
local DespawnEvent = {}

---@param payload DespawnEvent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return DespawnEvent
function DespawnEvent.new(payload) end



---@class DiscardEvent : ReflectReference
---  Trigger emitted when a component is removed from an entity, regardless
---  of whether or not it is later replaced.
--- 
---  Runs before the value is replaced, so you can still access the original component data.
---  See [`ComponentHooks::on_discard`](`crate::lifecycle::ComponentHooks::on_discard`) for more information.
---@field  entity? Entity
local DiscardEvent = {}

---@param payload DiscardEvent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return DiscardEvent
function DiscardEvent.new(payload) end



---@class InsertEvent : ReflectReference
---  Trigger emitted when a component is inserted, regardless of whether or not the entity already
---  had that component. Runs after `Add`, if it ran.
---  See [`ComponentHooks::on_insert`](`crate::lifecycle::ComponentHooks::on_insert`) for more information.
---@field  entity? Entity
local InsertEvent = {}

---@param payload InsertEvent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InsertEvent
function InsertEvent.new(payload) end



---@class RemoveEvent : ReflectReference
---  Trigger emitted when a component is removed from an entity, and runs before the component is
---  removed, so you can still access the component data.
---  See [`ComponentHooks::on_remove`](`crate::lifecycle::ComponentHooks::on_remove`) for more information.
---@field  entity? Entity
local RemoveEvent = {}

---@param payload RemoveEvent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RemoveEvent
function RemoveEvent.new(payload) end



---@class HashedStr : ReflectReference
---  A wrapper over Hashed. This exists to make Name("value".into()) possible, which plays nicely with contexts like the `bsn!` macro.
---@field  [1] any
local HashedStr = {}

---@param payload HashedStr.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return HashedStr
function HashedStr.new(payload) end



---@class TextureAtlas : ReflectReference
---  An index into a [`TextureAtlasLayout`], which corresponds to a specific section of a texture.
--- 
---  It stores a handle to [`TextureAtlasLayout`] and the index of the current section of the atlas.
---  The texture atlas contains various *sections* of a given texture, allowing users to have a single
---  image file for either sprite animation or global mapping.
---  You can change the texture [`index`](Self::index) of the atlas to animate the sprite or display only a *section* of the texture
---  for efficient rendering of related game objects.
--- 
---  Check the following examples for usage:
---  - [`animated sprite sheet example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
---  - [`sprite animation event example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
---  - [`texture atlas example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
---@field  layout? Handle<TextureAtlasLayout>
---@field  index? integer
local TextureAtlas = {}

---@param payload TextureAtlas.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TextureAtlas
function TextureAtlas.new(payload) end



---@class GamepadAxis : ReflectReference
---  Represents gamepad input types that are mapped in the range [-1.0, 1.0].
--- 
---  ## Usage
--- 
---  This is used to determine which axis has changed its value when receiving a
---  gamepad axis event. It is also used in the [`Gamepad`] component.
---@field  LeftStickX GamepadAxis.LeftStickX
---@field  LeftStickY GamepadAxis.LeftStickY
---@field  LeftZ GamepadAxis.LeftZ
---@field  RightStickX GamepadAxis.RightStickX
---@field  RightStickY GamepadAxis.RightStickY
---@field  RightZ GamepadAxis.RightZ
---@field  Other GamepadAxis.Other
local GamepadAxis = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GamepadAxis
function GamepadAxis.new(payload) end



---@class GamepadAxis.LeftStickX : EnumVariantRef
--- The `LeftStickX` variant of `GamepadAxis`.
GamepadAxis.LeftStickX = {}

---@param variant GamepadAxis.LeftStickX
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.LeftStickX.new(variant,payload) end



---@class GamepadAxis.LeftStickY : EnumVariantRef
--- The `LeftStickY` variant of `GamepadAxis`.
GamepadAxis.LeftStickY = {}

---@param variant GamepadAxis.LeftStickY
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.LeftStickY.new(variant,payload) end



---@class GamepadAxis.LeftZ : EnumVariantRef
--- The `LeftZ` variant of `GamepadAxis`.
GamepadAxis.LeftZ = {}

---@param variant GamepadAxis.LeftZ
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.LeftZ.new(variant,payload) end



---@class GamepadAxis.RightStickX : EnumVariantRef
--- The `RightStickX` variant of `GamepadAxis`.
GamepadAxis.RightStickX = {}

---@param variant GamepadAxis.RightStickX
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.RightStickX.new(variant,payload) end



---@class GamepadAxis.RightStickY : EnumVariantRef
--- The `RightStickY` variant of `GamepadAxis`.
GamepadAxis.RightStickY = {}

---@param variant GamepadAxis.RightStickY
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.RightStickY.new(variant,payload) end



---@class GamepadAxis.RightZ : EnumVariantRef
--- The `RightZ` variant of `GamepadAxis`.
GamepadAxis.RightZ = {}

---@param variant GamepadAxis.RightZ
---@param payload NoFields
---@return GamepadAxis
function GamepadAxis.RightZ.new(variant,payload) end



---@class GamepadAxis.Other : EnumVariantRef
--- The `Other` variant of `GamepadAxis`.
GamepadAxis.Other = {}

---@param variant GamepadAxis.Other
---@param payload GamepadAxis.Other.Fields
---@return GamepadAxis
function GamepadAxis.Other.new(variant,payload) end



---@class GamepadButton : ReflectReference
---  Represents gamepad input types that are mapped in the range [0.0, 1.0].
--- 
---  ## Usage
--- 
---  This is used to determine which button has changed its value when receiving gamepad button events.
---  It is also used in the [`Gamepad`] component.
---@field  South GamepadButton.South
---@field  East GamepadButton.East
---@field  North GamepadButton.North
---@field  West GamepadButton.West
---@field  C GamepadButton.C
---@field  Z GamepadButton.Z
---@field  LeftTrigger GamepadButton.LeftTrigger
---@field  LeftTrigger2 GamepadButton.LeftTrigger2
---@field  RightTrigger GamepadButton.RightTrigger
---@field  RightTrigger2 GamepadButton.RightTrigger2
---@field  Select GamepadButton.Select
---@field  Start GamepadButton.Start
---@field  Mode GamepadButton.Mode
---@field  LeftThumb GamepadButton.LeftThumb
---@field  RightThumb GamepadButton.RightThumb
---@field  DPadUp GamepadButton.DPadUp
---@field  DPadDown GamepadButton.DPadDown
---@field  DPadLeft GamepadButton.DPadLeft
---@field  DPadRight GamepadButton.DPadRight
---@field  Other GamepadButton.Other
local GamepadButton = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GamepadButton
function GamepadButton.new(payload) end



---@class GamepadButton.South : EnumVariantRef
--- The `South` variant of `GamepadButton`.
GamepadButton.South = {}

---@param variant GamepadButton.South
---@param payload NoFields
---@return GamepadButton
function GamepadButton.South.new(variant,payload) end



---@class GamepadButton.East : EnumVariantRef
--- The `East` variant of `GamepadButton`.
GamepadButton.East = {}

---@param variant GamepadButton.East
---@param payload NoFields
---@return GamepadButton
function GamepadButton.East.new(variant,payload) end



---@class GamepadButton.North : EnumVariantRef
--- The `North` variant of `GamepadButton`.
GamepadButton.North = {}

---@param variant GamepadButton.North
---@param payload NoFields
---@return GamepadButton
function GamepadButton.North.new(variant,payload) end



---@class GamepadButton.West : EnumVariantRef
--- The `West` variant of `GamepadButton`.
GamepadButton.West = {}

---@param variant GamepadButton.West
---@param payload NoFields
---@return GamepadButton
function GamepadButton.West.new(variant,payload) end



---@class GamepadButton.C : EnumVariantRef
--- The `C` variant of `GamepadButton`.
GamepadButton.C = {}

---@param variant GamepadButton.C
---@param payload NoFields
---@return GamepadButton
function GamepadButton.C.new(variant,payload) end



---@class GamepadButton.Z : EnumVariantRef
--- The `Z` variant of `GamepadButton`.
GamepadButton.Z = {}

---@param variant GamepadButton.Z
---@param payload NoFields
---@return GamepadButton
function GamepadButton.Z.new(variant,payload) end



---@class GamepadButton.LeftTrigger : EnumVariantRef
--- The `LeftTrigger` variant of `GamepadButton`.
GamepadButton.LeftTrigger = {}

---@param variant GamepadButton.LeftTrigger
---@param payload NoFields
---@return GamepadButton
function GamepadButton.LeftTrigger.new(variant,payload) end



---@class GamepadButton.LeftTrigger2 : EnumVariantRef
--- The `LeftTrigger2` variant of `GamepadButton`.
GamepadButton.LeftTrigger2 = {}

---@param variant GamepadButton.LeftTrigger2
---@param payload NoFields
---@return GamepadButton
function GamepadButton.LeftTrigger2.new(variant,payload) end



---@class GamepadButton.RightTrigger : EnumVariantRef
--- The `RightTrigger` variant of `GamepadButton`.
GamepadButton.RightTrigger = {}

---@param variant GamepadButton.RightTrigger
---@param payload NoFields
---@return GamepadButton
function GamepadButton.RightTrigger.new(variant,payload) end



---@class GamepadButton.RightTrigger2 : EnumVariantRef
--- The `RightTrigger2` variant of `GamepadButton`.
GamepadButton.RightTrigger2 = {}

---@param variant GamepadButton.RightTrigger2
---@param payload NoFields
---@return GamepadButton
function GamepadButton.RightTrigger2.new(variant,payload) end



---@class GamepadButton.Select : EnumVariantRef
--- The `Select` variant of `GamepadButton`.
GamepadButton.Select = {}

---@param variant GamepadButton.Select
---@param payload NoFields
---@return GamepadButton
function GamepadButton.Select.new(variant,payload) end



---@class GamepadButton.Start : EnumVariantRef
--- The `Start` variant of `GamepadButton`.
GamepadButton.Start = {}

---@param variant GamepadButton.Start
---@param payload NoFields
---@return GamepadButton
function GamepadButton.Start.new(variant,payload) end



---@class GamepadButton.Mode : EnumVariantRef
--- The `Mode` variant of `GamepadButton`.
GamepadButton.Mode = {}

---@param variant GamepadButton.Mode
---@param payload NoFields
---@return GamepadButton
function GamepadButton.Mode.new(variant,payload) end



---@class GamepadButton.LeftThumb : EnumVariantRef
--- The `LeftThumb` variant of `GamepadButton`.
GamepadButton.LeftThumb = {}

---@param variant GamepadButton.LeftThumb
---@param payload NoFields
---@return GamepadButton
function GamepadButton.LeftThumb.new(variant,payload) end



---@class GamepadButton.RightThumb : EnumVariantRef
--- The `RightThumb` variant of `GamepadButton`.
GamepadButton.RightThumb = {}

---@param variant GamepadButton.RightThumb
---@param payload NoFields
---@return GamepadButton
function GamepadButton.RightThumb.new(variant,payload) end



---@class GamepadButton.DPadUp : EnumVariantRef
--- The `DPadUp` variant of `GamepadButton`.
GamepadButton.DPadUp = {}

---@param variant GamepadButton.DPadUp
---@param payload NoFields
---@return GamepadButton
function GamepadButton.DPadUp.new(variant,payload) end



---@class GamepadButton.DPadDown : EnumVariantRef
--- The `DPadDown` variant of `GamepadButton`.
GamepadButton.DPadDown = {}

---@param variant GamepadButton.DPadDown
---@param payload NoFields
---@return GamepadButton
function GamepadButton.DPadDown.new(variant,payload) end



---@class GamepadButton.DPadLeft : EnumVariantRef
--- The `DPadLeft` variant of `GamepadButton`.
GamepadButton.DPadLeft = {}

---@param variant GamepadButton.DPadLeft
---@param payload NoFields
---@return GamepadButton
function GamepadButton.DPadLeft.new(variant,payload) end



---@class GamepadButton.DPadRight : EnumVariantRef
--- The `DPadRight` variant of `GamepadButton`.
GamepadButton.DPadRight = {}

---@param variant GamepadButton.DPadRight
---@param payload NoFields
---@return GamepadButton
function GamepadButton.DPadRight.new(variant,payload) end



---@class GamepadButton.Other : EnumVariantRef
--- The `Other` variant of `GamepadButton`.
GamepadButton.Other = {}

---@param variant GamepadButton.Other
---@param payload GamepadButton.Other.Fields
---@return GamepadButton
function GamepadButton.Other.new(variant,payload) end



---@class GamepadInput : ReflectReference
---  Encapsulation over [`GamepadAxis`] and [`GamepadButton`].
---@field  Axis GamepadInput.Axis
---@field  Button GamepadInput.Button
local GamepadInput = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GamepadInput
function GamepadInput.new(payload) end



---@class GamepadInput.Axis : EnumVariantRef
--- The `Axis` variant of `GamepadInput`.
GamepadInput.Axis = {}

---@param variant GamepadInput.Axis
---@param payload GamepadInput.Axis.Fields
---@return GamepadInput
function GamepadInput.Axis.new(variant,payload) end



---@class GamepadInput.Button : EnumVariantRef
--- The `Button` variant of `GamepadInput`.
GamepadInput.Button = {}

---@param variant GamepadInput.Button
---@param payload GamepadInput.Button.Fields
---@return GamepadInput
function GamepadInput.Button.new(variant,payload) end



---@class NativeKeyCode : ReflectReference
---  Contains the platform-native physical key identifier
--- 
---  The exact values vary from platform to platform (which is part of why this is a per-platform
---  enum), but the values are primarily tied to the key's physical location on the keyboard.
--- 
---  This enum is primarily used to store raw keycodes when Winit doesn't map a given native
---  physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
---  haven't mapped for you yet, this lets you use [`KeyCode`] to:
--- 
---  - Correctly match key press and release events.
---  - On non-web platforms, support assigning keybinds to virtually any key through a UI.
---@field  Unidentified NativeKeyCode.Unidentified
---@field  Android NativeKeyCode.Android
---@field  MacOS NativeKeyCode.MacOS
---@field  Windows NativeKeyCode.Windows
---@field  Xkb NativeKeyCode.Xkb
local NativeKeyCode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NativeKeyCode
function NativeKeyCode.new(payload) end



---@class NativeKeyCode.Unidentified : EnumVariantRef
--- The `Unidentified` variant of `NativeKeyCode`.
NativeKeyCode.Unidentified = {}

---@param variant NativeKeyCode.Unidentified
---@param payload NoFields
---@return NativeKeyCode
function NativeKeyCode.Unidentified.new(variant,payload) end



---@class NativeKeyCode.Android : EnumVariantRef
--- The `Android` variant of `NativeKeyCode`.
NativeKeyCode.Android = {}

---@param variant NativeKeyCode.Android
---@param payload NativeKeyCode.Android.Fields
---@return NativeKeyCode
function NativeKeyCode.Android.new(variant,payload) end



---@class NativeKeyCode.MacOS : EnumVariantRef
--- The `MacOS` variant of `NativeKeyCode`.
NativeKeyCode.MacOS = {}

---@param variant NativeKeyCode.MacOS
---@param payload NativeKeyCode.MacOS.Fields
---@return NativeKeyCode
function NativeKeyCode.MacOS.new(variant,payload) end



---@class NativeKeyCode.Windows : EnumVariantRef
--- The `Windows` variant of `NativeKeyCode`.
NativeKeyCode.Windows = {}

---@param variant NativeKeyCode.Windows
---@param payload NativeKeyCode.Windows.Fields
---@return NativeKeyCode
function NativeKeyCode.Windows.new(variant,payload) end



---@class NativeKeyCode.Xkb : EnumVariantRef
--- The `Xkb` variant of `NativeKeyCode`.
NativeKeyCode.Xkb = {}

---@param variant NativeKeyCode.Xkb
---@param payload NativeKeyCode.Xkb.Fields
---@return NativeKeyCode
function NativeKeyCode.Xkb.new(variant,payload) end



---@class FocusCause : ReflectReference
---  The cause for a [`FocusGained`]
--- 
---  Sometimes widgets would like to know how their focus was gained so they can act accordingly.
--- 
---  For example, a text input may want to select all text when navigated into, but not when pressed.
---@field  Navigated FocusCause.Navigated
---@field  Pressed FocusCause.Pressed
---@field  Auto FocusCause.Auto
local FocusCause = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FocusCause
function FocusCause.new(payload) end



---@class FocusCause.Navigated : EnumVariantRef
--- The `Navigated` variant of `FocusCause`.
FocusCause.Navigated = {}

---@param variant FocusCause.Navigated
---@param payload NoFields
---@return FocusCause
function FocusCause.Navigated.new(variant,payload) end



---@class FocusCause.Pressed : EnumVariantRef
--- The `Pressed` variant of `FocusCause`.
FocusCause.Pressed = {}

---@param variant FocusCause.Pressed
---@param payload NoFields
---@return FocusCause
function FocusCause.Pressed.new(variant,payload) end



---@class FocusCause.Auto : EnumVariantRef
--- The `Auto` variant of `FocusCause`.
FocusCause.Auto = {}

---@param variant FocusCause.Auto
---@param payload NoFields
---@return FocusCause
function FocusCause.Auto.new(variant,payload) end



---@class AlphaMode : ReflectReference
---  Sets how a material's base color alpha channel is used for transparency.
---@field  Opaque AlphaMode.Opaque
---@field  Mask AlphaMode.Mask
---@field  Blend AlphaMode.Blend
---@field  Premultiplied AlphaMode.Premultiplied
---@field  AlphaToCoverage AlphaMode.AlphaToCoverage
---@field  Add AlphaMode.Add
---@field  Multiply AlphaMode.Multiply
local AlphaMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AlphaMode
function AlphaMode.new(payload) end



---@class AlphaMode.Opaque : EnumVariantRef
--- The `Opaque` variant of `AlphaMode`.
AlphaMode.Opaque = {}

---@param variant AlphaMode.Opaque
---@param payload NoFields
---@return AlphaMode
function AlphaMode.Opaque.new(variant,payload) end



---@class AlphaMode.Mask : EnumVariantRef
--- The `Mask` variant of `AlphaMode`.
AlphaMode.Mask = {}

---@param variant AlphaMode.Mask
---@param payload AlphaMode.Mask.Fields
---@return AlphaMode
function AlphaMode.Mask.new(variant,payload) end



---@class AlphaMode.Blend : EnumVariantRef
--- The `Blend` variant of `AlphaMode`.
AlphaMode.Blend = {}

---@param variant AlphaMode.Blend
---@param payload NoFields
---@return AlphaMode
function AlphaMode.Blend.new(variant,payload) end



---@class AlphaMode.Premultiplied : EnumVariantRef
--- The `Premultiplied` variant of `AlphaMode`.
AlphaMode.Premultiplied = {}

---@param variant AlphaMode.Premultiplied
---@param payload NoFields
---@return AlphaMode
function AlphaMode.Premultiplied.new(variant,payload) end



---@class AlphaMode.AlphaToCoverage : EnumVariantRef
--- The `AlphaToCoverage` variant of `AlphaMode`.
AlphaMode.AlphaToCoverage = {}

---@param variant AlphaMode.AlphaToCoverage
---@param payload NoFields
---@return AlphaMode
function AlphaMode.AlphaToCoverage.new(variant,payload) end



---@class AlphaMode.Add : EnumVariantRef
--- The `Add` variant of `AlphaMode`.
AlphaMode.Add = {}

---@param variant AlphaMode.Add
---@param payload NoFields
---@return AlphaMode
function AlphaMode.Add.new(variant,payload) end



---@class AlphaMode.Multiply : EnumVariantRef
--- The `Multiply` variant of `AlphaMode`.
AlphaMode.Multiply = {}

---@param variant AlphaMode.Multiply
---@param payload NoFields
---@return AlphaMode
function AlphaMode.Multiply.new(variant,payload) end



---@class OpaqueRendererMethod : ReflectReference
---  Render method used for opaque materials.
--- 
---  The forward rendering main pass draws each mesh entity and shades it according to its
---  corresponding material and the lights that affect it. Some render features like Screen Space
---  Ambient Occlusion require running depth and normal prepasses, that are 'deferred'-like
---  prepasses over all mesh entities to populate depth and normal textures. This means that when
---  using render features that require running prepasses, multiple passes over all visible geometry
---  are required. This can be slow if there is a lot of geometry that cannot be batched into few
---  draws.
--- 
---  Deferred rendering runs a prepass to gather not only geometric information like depth and
---  normals, but also all the material properties like base color, emissive color, reflectance,
---  metalness, etc, and writes them into a deferred 'g-buffer' texture. The deferred main pass is
---  then a fullscreen pass that reads data from these textures and executes shading. This allows
---  for one pass over geometry, but is at the cost of not being able to use MSAA, and has heavier
---  bandwidth usage which can be unsuitable for low end mobile or other bandwidth-constrained devices.
--- 
---  If a material indicates `OpaqueRendererMethod::Auto`, `DefaultOpaqueRendererMethod` will be used.
---@field  Forward OpaqueRendererMethod.Forward
---@field  Deferred OpaqueRendererMethod.Deferred
---@field  Auto OpaqueRendererMethod.Auto
local OpaqueRendererMethod = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return OpaqueRendererMethod
function OpaqueRendererMethod.new(payload) end



---@class OpaqueRendererMethod.Forward : EnumVariantRef
--- The `Forward` variant of `OpaqueRendererMethod`.
OpaqueRendererMethod.Forward = {}

---@param variant OpaqueRendererMethod.Forward
---@param payload NoFields
---@return OpaqueRendererMethod
function OpaqueRendererMethod.Forward.new(variant,payload) end



---@class OpaqueRendererMethod.Deferred : EnumVariantRef
--- The `Deferred` variant of `OpaqueRendererMethod`.
OpaqueRendererMethod.Deferred = {}

---@param variant OpaqueRendererMethod.Deferred
---@param payload NoFields
---@return OpaqueRendererMethod
function OpaqueRendererMethod.Deferred.new(variant,payload) end



---@class OpaqueRendererMethod.Auto : EnumVariantRef
--- The `Auto` variant of `OpaqueRendererMethod`.
OpaqueRendererMethod.Auto = {}

---@param variant OpaqueRendererMethod.Auto
---@param payload NoFields
---@return OpaqueRendererMethod
function OpaqueRendererMethod.Auto.new(variant,payload) end



---@class CompassOctant : ReflectReference
---  A compass enum with 8 directions.
---  ```text
---           N (North)
---           ▲
---      NW   │   NE
---         ╲ │ ╱
---  W (West) ┼─────► E (East)
---         ╱ │ ╲
---      SW   │   SE
---           ▼
---           S (South)
---  ```
---@field  North CompassOctant.North
---@field  NorthEast CompassOctant.NorthEast
---@field  East CompassOctant.East
---@field  SouthEast CompassOctant.SouthEast
---@field  South CompassOctant.South
---@field  SouthWest CompassOctant.SouthWest
---@field  West CompassOctant.West
---@field  NorthWest CompassOctant.NorthWest
local CompassOctant = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return CompassOctant
function CompassOctant.new(payload) end



---@class CompassOctant.North : EnumVariantRef
--- The `North` variant of `CompassOctant`.
CompassOctant.North = {}

---@param variant CompassOctant.North
---@param payload NoFields
---@return CompassOctant
function CompassOctant.North.new(variant,payload) end



---@class CompassOctant.NorthEast : EnumVariantRef
--- The `NorthEast` variant of `CompassOctant`.
CompassOctant.NorthEast = {}

---@param variant CompassOctant.NorthEast
---@param payload NoFields
---@return CompassOctant
function CompassOctant.NorthEast.new(variant,payload) end



---@class CompassOctant.East : EnumVariantRef
--- The `East` variant of `CompassOctant`.
CompassOctant.East = {}

---@param variant CompassOctant.East
---@param payload NoFields
---@return CompassOctant
function CompassOctant.East.new(variant,payload) end



---@class CompassOctant.SouthEast : EnumVariantRef
--- The `SouthEast` variant of `CompassOctant`.
CompassOctant.SouthEast = {}

---@param variant CompassOctant.SouthEast
---@param payload NoFields
---@return CompassOctant
function CompassOctant.SouthEast.new(variant,payload) end



---@class CompassOctant.South : EnumVariantRef
--- The `South` variant of `CompassOctant`.
CompassOctant.South = {}

---@param variant CompassOctant.South
---@param payload NoFields
---@return CompassOctant
function CompassOctant.South.new(variant,payload) end



---@class CompassOctant.SouthWest : EnumVariantRef
--- The `SouthWest` variant of `CompassOctant`.
CompassOctant.SouthWest = {}

---@param variant CompassOctant.SouthWest
---@param payload NoFields
---@return CompassOctant
function CompassOctant.SouthWest.new(variant,payload) end



---@class CompassOctant.West : EnumVariantRef
--- The `West` variant of `CompassOctant`.
CompassOctant.West = {}

---@param variant CompassOctant.West
---@param payload NoFields
---@return CompassOctant
function CompassOctant.West.new(variant,payload) end



---@class CompassOctant.NorthWest : EnumVariantRef
--- The `NorthWest` variant of `CompassOctant`.
CompassOctant.NorthWest = {}

---@param variant CompassOctant.NorthWest
---@param payload NoFields
---@return CompassOctant
function CompassOctant.NorthWest.new(variant,payload) end



---@class URect : ReflectReference
---  A rectangle defined by two opposite corners.
--- 
---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
---  stored in `URect::min` and `URect::max`, respectively. The minimum/maximum invariant
---  must be upheld by the user when directly assigning the fields, otherwise some methods
---  produce invalid results. It is generally recommended to use one of the constructor
---  methods instead, which will ensure this invariant is met, unless you already have
---  the minimum and maximum corners.
---@field  min? UVec2
---@field  max? UVec2
local URect = {}

---@param payload URect.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return URect
function URect.new(payload) end



---@class Indices : ReflectReference
---  An array of indices into the [`VertexAttributeValues`](super::VertexAttributeValues) for a mesh.
--- 
---  It describes the order in which the vertex attributes should be joined into faces.
---@field  U16 Indices.U16
---@field  U32 Indices.U32
local Indices = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Indices
function Indices.new(payload) end



---@class Indices.U16 : EnumVariantRef
--- The `U16` variant of `Indices`.
Indices.U16 = {}

---@param variant Indices.U16
---@param payload Indices.U16.Fields
---@return Indices
function Indices.U16.new(variant,payload) end



---@class Indices.U32 : EnumVariantRef
--- The `U32` variant of `Indices`.
Indices.U32 = {}

---@param variant Indices.U32
---@param payload Indices.U32.Fields
---@return Indices
function Indices.U32.new(variant,payload) end



---@class MeshAttributeCompressionFlags : ReflectReference
---  If the corresponding attribute compression is enabled:
---  - Position will be Snorm16x4 relative to the mesh's AABB. The w component is unused.
---  - Normal and tangent will be Snorm16x2 with octahedral encoding, using [`octahedral_encode_signed`] and [`octahedral_encode_tangent`].
---  - UV0 and UV1 will be Unorm16x2. UVs are remapped based on their min/max values so they can go beyond [0, 1], though a larger range will reduce precision.
--- 
---  [`octahedral_encode_signed`]: crate::vertex::octahedral_encode_signed
---  [`octahedral_encode_tangent`]: crate::vertex::octahedral_encode_tangent
local MeshAttributeCompressionFlags = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MeshAttributeCompressionFlags
function MeshAttributeCompressionFlags.new(payload) end



---@class UvChannel : ReflectReference
---  An enum to define which UV attribute to use for a texture.
--- 
---  It only supports two UV attributes, [`Mesh::ATTRIBUTE_UV_0`] and
---  [`Mesh::ATTRIBUTE_UV_1`].
---  The default is [`UvChannel::Uv0`].
---@field  Uv0 UvChannel.Uv0
---@field  Uv1 UvChannel.Uv1
local UvChannel = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return UvChannel
function UvChannel.new(payload) end



---@class UvChannel.Uv0 : EnumVariantRef
--- The `Uv0` variant of `UvChannel`.
UvChannel.Uv0 = {}

---@param variant UvChannel.Uv0
---@param payload NoFields
---@return UvChannel
function UvChannel.Uv0.new(variant,payload) end



---@class UvChannel.Uv1 : EnumVariantRef
--- The `Uv1` variant of `UvChannel`.
UvChannel.Uv1 = {}

---@param variant UvChannel.Uv1
---@param payload NoFields
---@return UvChannel
function UvChannel.Uv1.new(variant,payload) end



---@class MorphAttributes : ReflectReference
---  Attributes **differences** used for morph targets.
---@field  position? Vec3
---@field  pad_a? number
---@field  normal? Vec3
---@field  pad_b? number
---@field  tangent? Vec3
---@field  pad_c? number
local MorphAttributes = {}

---@param payload MorphAttributes.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MorphAttributes
function MorphAttributes.new(payload) end



---@class JointAabb : ReflectReference
---@field  center? Vec3
---@field  half_size? Vec3
local JointAabb = {}

---@param payload JointAabb.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JointAabb
function JointAabb.new(payload) end



---@class JointIndex : ReflectReference
---@field  [1] integer
local JointIndex = {}

---@param payload JointIndex.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JointIndex
function JointIndex.new(payload) end



---@class SkinnedMeshBounds : ReflectReference
---  Data that can be used to calculate the AABB of a skinned mesh.
---@field  aabbs? JointAabb[]
---@field  aabb_index_to_joint_index? JointIndex[]
local SkinnedMeshBounds = {}

---@param payload SkinnedMeshBounds.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SkinnedMeshBounds
function SkinnedMeshBounds.new(payload) end



---@class Language : ReflectReference
---  Represents a scripting language. Languages which compile into another language should use the target language as their language.
---@field  Rhai Language.Rhai
---@field  Lua Language.Lua
---@field  Rune Language.Rune
---@field  External Language.External
---@field  Unknown Language.Unknown
local Language = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Language
function Language.new(payload) end



---@class Language.Rhai : EnumVariantRef
--- The `Rhai` variant of `Language`.
Language.Rhai = {}

---@param variant Language.Rhai
---@param payload NoFields
---@return Language
function Language.Rhai.new(variant,payload) end



---@class Language.Lua : EnumVariantRef
--- The `Lua` variant of `Language`.
Language.Lua = {}

---@param variant Language.Lua
---@param payload NoFields
---@return Language
function Language.Lua.new(variant,payload) end



---@class Language.Rune : EnumVariantRef
--- The `Rune` variant of `Language`.
Language.Rune = {}

---@param variant Language.Rune
---@param payload NoFields
---@return Language
function Language.Rune.new(variant,payload) end



---@class Language.External : EnumVariantRef
--- The `External` variant of `Language`.
Language.External = {}

---@param variant Language.External
---@param payload Language.External.Fields
---@return Language
function Language.External.new(variant,payload) end



---@class Language.Unknown : EnumVariantRef
--- The `Unknown` variant of `Language`.
Language.Unknown = {}

---@param variant Language.Unknown
---@param payload NoFields
---@return Language
function Language.Unknown.new(variant,payload) end



---@class ScriptAsset : ReflectReference
---  Represents a script loaded into memory as an asset
local ScriptAsset = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptAsset
function ScriptAsset.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<ScriptAsset>
function ScriptAsset.load(path) end

---@param value ScriptAsset
--- The asset value.
---@return Handle<ScriptAsset>
function ScriptAsset.add_asset(value) end

---@param handle Handle<ScriptAsset>
--- The handle to the asset.
---@return ScriptAsset | nil
function ScriptAsset.get_asset(handle) end



---@class FunctionArgInfo : ReflectReference
---  Information about a function argument.
---@field  name? Cow | nil
---@field  arg_index? integer
---@field  type_id? TypeId
local FunctionArgInfo = {}

---@param payload FunctionArgInfo.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FunctionArgInfo
function FunctionArgInfo.new(payload) end



---@class FunctionInfo : ReflectReference
---  Information about a function.
---@field  name? Cow
---@field  namespace? Namespace
---@field  arg_info? FunctionArgInfo[]
---@field  return_info? FunctionReturnInfo
---@field  docs? Cow | nil
local FunctionInfo = {}

---@param payload FunctionInfo.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FunctionInfo
function FunctionInfo.new(payload) end



---@class FunctionReturnInfo : ReflectReference
---  Information about a function return value.
---@field  type_id? TypeId
local FunctionReturnInfo = {}

---@param payload FunctionReturnInfo.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FunctionReturnInfo
function FunctionReturnInfo.new(payload) end



---@class InteropError : ReflectReference
---  An error occurring when converting between rust and a script context.
local InteropError = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InteropError
function InteropError.new(payload) end



---@class Namespace : ReflectReference
---  A namespace for functions
---@field  Global Namespace.Global
---@field  OnType Namespace.OnType
local Namespace = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Namespace
function Namespace.new(payload) end



---@class Namespace.Global : EnumVariantRef
--- The `Global` variant of `Namespace`.
Namespace.Global = {}

---@param variant Namespace.Global
---@param payload NoFields
---@return Namespace
function Namespace.Global.new(variant,payload) end



---@class Namespace.OnType : EnumVariantRef
--- The `OnType` variant of `Namespace`.
Namespace.OnType = {}

---@param variant Namespace.OnType
---@param payload Namespace.OnType.Fields
---@return Namespace
function Namespace.OnType.new(variant,payload) end



---@class DynamicComponent : ReflectReference
---  A dynamic script component
---@field  data? ScriptValue
local DynamicComponent = {}

---@param payload DynamicComponent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return DynamicComponent
function DynamicComponent.new(payload) end



---@class VariadicTuple : ReflectReference
---  A tuple variant of script value
local VariadicTuple = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return VariadicTuple
function VariadicTuple.new(payload) end



---@class StaticTypeRef : ReflectReference
---  A static reference to a type, what scripts see as the type's global.
--- 
---  Functions registered on this type are available on every type's static reference, i.e. `MyType.function(..)`.
---  When called this way, the first argument is the [`StaticTypeRef`] of the type, and is filled in automatically.
--- 
---  Functions registered on the type itself take priority over the ones registered here.
---@field  type_id? TypeId
local StaticTypeRef = {}

---@param type StaticTypeRef
--- The type to construct.
---@param payload table<string, any>
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReflectReference
function StaticTypeRef.new(type,payload) end

---@param asset_type StaticTypeRef
--- The asset type, filled in automatically.
---@param path string
--- The asset path, relative to the assets folder.
---@return ReflectReference
function StaticTypeRef.load(asset_type,path) end

---@param asset_type StaticTypeRef
--- The asset type, filled in automatically.
---@param value ReflectReference
--- The asset value.
---@return ReflectReference
function StaticTypeRef.add_asset(asset_type,value) end

---@param asset_type StaticTypeRef
--- The asset type, filled in automatically.
---@param handle ReflectReference
--- The handle to the asset.
---@return ReflectReference | nil
function StaticTypeRef.get_asset(asset_type,handle) end



---@class CallbackLabel : ReflectReference
---  A string which disallows common invalid characters in callback labels,
---  particularly at the start of the string
--- 
---  a valid callback label starts with a letter or underscore, and contains only ascii characters, as well as disallows some common keywords
---@field  [1] string
local CallbackLabel = {}

---@param payload CallbackLabel.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return CallbackLabel
function CallbackLabel.new(payload) end



---@class ScriptAssetModifiedMessage : ReflectReference
---  Emitted when a script asset is modified and all its attachments require re-loading
---@field  [1] any
local ScriptAssetModifiedMessage = {}

---@param payload ScriptAssetModifiedMessage.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptAssetModifiedMessage
function ScriptAssetModifiedMessage.new(payload) end



---@class ScriptAttachedMessage : ReflectReference
---  Emitted when a script is attached.
---@field  [1] ScriptAttachment
local ScriptAttachedMessage = {}

---@param payload ScriptAttachedMessage.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptAttachedMessage
function ScriptAttachedMessage.new(payload) end



---@class ScriptCallbackMessage : ReflectReference
---  A callback message meant to trigger a callback in a subset/set of scripts in the world with the given arguments
---@field  label? CallbackLabel
---@field  recipients? Recipients
---@field  language? Language | nil
---@field  args? ScriptValue[]
---@field  trigger_response? boolean
---@field  iteration? integer
local ScriptCallbackMessage = {}

---@param payload ScriptCallbackMessage.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptCallbackMessage
function ScriptCallbackMessage.new(payload) end



---@class ScriptDetachedMessage : ReflectReference
---  Emitted when a script is detached.
---@field  [1] ScriptAttachment
local ScriptDetachedMessage = {}

---@param payload ScriptDetachedMessage.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptDetachedMessage
function ScriptDetachedMessage.new(payload) end



---@class ScriptComponent : ReflectReference
---  A component which identifies the scripts existing on an entity.
--- 
---  Event handlers search for components with this component to figure out which scripts to run and on which entities.
---@field  [1] Handle<ScriptAsset>[]
local ScriptComponent = {}

---@param payload ScriptComponent.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScriptComponent
function ScriptComponent.new(payload) end



---@class ParallaxMappingMethod : ReflectReference
---  The [parallax mapping] method to use to compute depth based on the
---  material's [`depth_map`].
--- 
---  Parallax Mapping uses a depth map texture to give the illusion of depth
---  variation on a mesh surface that is geometrically flat.
--- 
---  See the `parallax_mapping.wesl` shader code for implementation details
---  and explanation of the methods used.
--- 
---  [`depth_map`]: crate::StandardMaterial::depth_map
---  [parallax mapping]: https://en.wikipedia.org/wiki/Parallax_mapping
---@field  Occlusion ParallaxMappingMethod.Occlusion
---@field  Relief ParallaxMappingMethod.Relief
local ParallaxMappingMethod = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ParallaxMappingMethod
function ParallaxMappingMethod.new(payload) end



---@class ParallaxMappingMethod.Occlusion : EnumVariantRef
--- The `Occlusion` variant of `ParallaxMappingMethod`.
ParallaxMappingMethod.Occlusion = {}

---@param variant ParallaxMappingMethod.Occlusion
---@param payload NoFields
---@return ParallaxMappingMethod
function ParallaxMappingMethod.Occlusion.new(variant,payload) end



---@class ParallaxMappingMethod.Relief : EnumVariantRef
--- The `Relief` variant of `ParallaxMappingMethod`.
ParallaxMappingMethod.Relief = {}

---@param variant ParallaxMappingMethod.Relief
---@param payload ParallaxMappingMethod.Relief.Fields
---@return ParallaxMappingMethod
function ParallaxMappingMethod.Relief.new(variant,payload) end



---@class ReadbackOnce : ReflectReference
---  Marker component that pairs with a [`Readback`] to indicate the entity should be
---  despawned after the first [`ReadbackComplete`] event fires for it.
--- 
---  ```ignore
---  commands.spawn(ReadbackOnce::buffer(handle));
---  ```
local ReadbackOnce = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReadbackOnce
function ReadbackOnce.new(payload) end



---@class ShaderBuffer : ReflectReference
---  A storage buffer that is prepared as a [`RenderAsset`] and uploaded to the GPU.
--- 
---  This buffer primarily exists in order to be embedded into a material that
---  implements the [`bevy_render_macros::AsBindGroup`] trait. Compared to
---  embedding a raw [`Buffer`], [`ShaderBuffer`] has the advantage that the
---  buffer can be resized without regenerating the materials that embed it.
local ShaderBuffer = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ShaderBuffer
function ShaderBuffer.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<ShaderBuffer>
function ShaderBuffer.load(path) end

---@param value ShaderBuffer
--- The asset value.
---@return Handle<ShaderBuffer>
function ShaderBuffer.add_asset(value) end

---@param handle Handle<ShaderBuffer>
--- The handle to the asset.
---@return ShaderBuffer | nil
function ShaderBuffer.get_asset(handle) end



---@class Screenshot : ReflectReference
---  A component that signals to the renderer to capture a screenshot this frame.
--- 
---  This component should be spawned on a new entity with an observer that will trigger
---  with [`ScreenshotCaptured`] when the screenshot is ready.
--- 
---  Screenshots are captured asynchronously and may not be available immediately after the frame
---  that the component is spawned on. The observer should be used to handle the screenshot when it
---  is ready.
--- 
---  Note that the screenshot entity will be despawned after the screenshot is captured and the
---  observer is triggered.
--- 
---  # Usage
--- 
---  ```
---  # use bevy_ecs::prelude::*;
---  # use bevy_render::view::screenshot::{save_to_disk, Screenshot};
--- 
---  fn take_screenshot(mut commands: Commands) {
---     commands.spawn(Screenshot::primary_window())
---        .observe(save_to_disk("screenshot.png"));
---  }
---  ```
---@field  [1] RenderTarget
local Screenshot = {}

---@param payload Screenshot.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Screenshot
function Screenshot.new(payload) end



---@class ScreenshotCaptured : ReflectReference
---@field  entity? Entity
---@field  image? Image
local ScreenshotCaptured = {}

---@param payload ScreenshotCaptured.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScreenshotCaptured
function ScreenshotCaptured.new(payload) end



---@class Aabb2d : ReflectReference
---  A 2D axis-aligned bounding box, or bounding rectangle
---@field  min? Vec2
---@field  max? Vec2
local Aabb2d = {}

---@param payload Aabb2d.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Aabb2d
function Aabb2d.new(payload) end



---@class Aabb3d : ReflectReference
---  A 3D axis-aligned bounding box
---@field  min? Vec3A
---@field  max? Vec3A
local Aabb3d = {}

---@param payload Aabb3d.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Aabb3d
function Aabb3d.new(payload) end



---@class SpriteAlphaMode : ReflectReference
---  An enum describing how a sprite's alpha channel will be handled.
---  The base color being modified is that of the texture after tinting.
---  The default is `Blend`.
---@field  Opaque SpriteAlphaMode.Opaque
---@field  Mask SpriteAlphaMode.Mask
---@field  Blend SpriteAlphaMode.Blend
local SpriteAlphaMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SpriteAlphaMode
function SpriteAlphaMode.new(payload) end



---@class SpriteAlphaMode.Opaque : EnumVariantRef
--- The `Opaque` variant of `SpriteAlphaMode`.
SpriteAlphaMode.Opaque = {}

---@param variant SpriteAlphaMode.Opaque
---@param payload NoFields
---@return SpriteAlphaMode
function SpriteAlphaMode.Opaque.new(variant,payload) end



---@class SpriteAlphaMode.Mask : EnumVariantRef
--- The `Mask` variant of `SpriteAlphaMode`.
SpriteAlphaMode.Mask = {}

---@param variant SpriteAlphaMode.Mask
---@param payload SpriteAlphaMode.Mask.Fields
---@return SpriteAlphaMode
function SpriteAlphaMode.Mask.new(variant,payload) end



---@class SpriteAlphaMode.Blend : EnumVariantRef
--- The `Blend` variant of `SpriteAlphaMode`.
SpriteAlphaMode.Blend = {}

---@param variant SpriteAlphaMode.Blend
---@param payload NoFields
---@return SpriteAlphaMode
function SpriteAlphaMode.Blend.new(variant,payload) end



---@class SpriteImageMode : ReflectReference
---  Controls how the image is altered when scaled.
---@field  Auto SpriteImageMode.Auto
---@field  Scale SpriteImageMode.Scale
---@field  Sliced SpriteImageMode.Sliced
---@field  Tiled SpriteImageMode.Tiled
local SpriteImageMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SpriteImageMode
function SpriteImageMode.new(payload) end



---@class SpriteImageMode.Auto : EnumVariantRef
--- The `Auto` variant of `SpriteImageMode`.
SpriteImageMode.Auto = {}

---@param variant SpriteImageMode.Auto
---@param payload NoFields
---@return SpriteImageMode
function SpriteImageMode.Auto.new(variant,payload) end



---@class SpriteImageMode.Scale : EnumVariantRef
--- The `Scale` variant of `SpriteImageMode`.
SpriteImageMode.Scale = {}

---@param variant SpriteImageMode.Scale
---@param payload SpriteImageMode.Scale.Fields
---@return SpriteImageMode
function SpriteImageMode.Scale.new(variant,payload) end



---@class SpriteImageMode.Sliced : EnumVariantRef
--- The `Sliced` variant of `SpriteImageMode`.
SpriteImageMode.Sliced = {}

---@param variant SpriteImageMode.Sliced
---@param payload SpriteImageMode.Sliced.Fields
---@return SpriteImageMode
function SpriteImageMode.Sliced.new(variant,payload) end



---@class SpriteImageMode.Tiled : EnumVariantRef
--- The `Tiled` variant of `SpriteImageMode`.
SpriteImageMode.Tiled = {}

---@param variant SpriteImageMode.Tiled
---@param payload SpriteImageMode.Tiled.Fields
---@return SpriteImageMode
function SpriteImageMode.Tiled.new(variant,payload) end



---@class SpriteScalingMode : ReflectReference
---  Represents various modes for proportional scaling of a texture.
--- 
---  Can be used in [`SpriteImageMode::Scale`].
---@field  FillCenter SpriteScalingMode.FillCenter
---@field  FillStart SpriteScalingMode.FillStart
---@field  FillEnd SpriteScalingMode.FillEnd
---@field  FitCenter SpriteScalingMode.FitCenter
---@field  FitStart SpriteScalingMode.FitStart
---@field  FitEnd SpriteScalingMode.FitEnd
local SpriteScalingMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SpriteScalingMode
function SpriteScalingMode.new(payload) end



---@class SpriteScalingMode.FillCenter : EnumVariantRef
--- The `FillCenter` variant of `SpriteScalingMode`.
SpriteScalingMode.FillCenter = {}

---@param variant SpriteScalingMode.FillCenter
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FillCenter.new(variant,payload) end



---@class SpriteScalingMode.FillStart : EnumVariantRef
--- The `FillStart` variant of `SpriteScalingMode`.
SpriteScalingMode.FillStart = {}

---@param variant SpriteScalingMode.FillStart
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FillStart.new(variant,payload) end



---@class SpriteScalingMode.FillEnd : EnumVariantRef
--- The `FillEnd` variant of `SpriteScalingMode`.
SpriteScalingMode.FillEnd = {}

---@param variant SpriteScalingMode.FillEnd
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FillEnd.new(variant,payload) end



---@class SpriteScalingMode.FitCenter : EnumVariantRef
--- The `FitCenter` variant of `SpriteScalingMode`.
SpriteScalingMode.FitCenter = {}

---@param variant SpriteScalingMode.FitCenter
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FitCenter.new(variant,payload) end



---@class SpriteScalingMode.FitStart : EnumVariantRef
--- The `FitStart` variant of `SpriteScalingMode`.
SpriteScalingMode.FitStart = {}

---@param variant SpriteScalingMode.FitStart
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FitStart.new(variant,payload) end



---@class SpriteScalingMode.FitEnd : EnumVariantRef
--- The `FitEnd` variant of `SpriteScalingMode`.
SpriteScalingMode.FitEnd = {}

---@param variant SpriteScalingMode.FitEnd
---@param payload NoFields
---@return SpriteScalingMode
function SpriteScalingMode.FitEnd.new(variant,payload) end



---@class BorderRect : ReflectReference
---  Defines border insets that shrink a rectangle from its minimum and maximum corners.
--- 
---  This struct is used to represent thickness or offsets from the four edges
---  of a rectangle, with values increasing inwards.
---@field  min_inset? Vec2
---@field  max_inset? Vec2
local BorderRect = {}

---@param payload BorderRect.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BorderRect
function BorderRect.new(payload) end



---@class SliceScaleMode : ReflectReference
---  Defines how a texture slice scales when resized
---@field  Stretch SliceScaleMode.Stretch
---@field  Tile SliceScaleMode.Tile
local SliceScaleMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SliceScaleMode
function SliceScaleMode.new(payload) end



---@class SliceScaleMode.Stretch : EnumVariantRef
--- The `Stretch` variant of `SliceScaleMode`.
SliceScaleMode.Stretch = {}

---@param variant SliceScaleMode.Stretch
---@param payload NoFields
---@return SliceScaleMode
function SliceScaleMode.Stretch.new(variant,payload) end



---@class SliceScaleMode.Tile : EnumVariantRef
--- The `Tile` variant of `SliceScaleMode`.
SliceScaleMode.Tile = {}

---@param variant SliceScaleMode.Tile
---@param payload SliceScaleMode.Tile.Fields
---@return SliceScaleMode
function SliceScaleMode.Tile.new(variant,payload) end



---@class TextureSlicer : ReflectReference
---  Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes
---  without needing to prepare multiple assets. The associated texture will be split into nine portions,
---  so that on resize the different portions scale or tile in different ways to keep the texture in proportion.
--- 
---  For example, when resizing a 9-sliced texture the corners will remain unscaled while the other
---  sections will be scaled or tiled.
--- 
---  See [9-sliced](https://en.wikipedia.org/wiki/9-slice_scaling) textures.
---@field  border? BorderRect
---@field  center_scale_mode? SliceScaleMode
---@field  sides_scale_mode? SliceScaleMode
---@field  max_corner_scale? number
local TextureSlicer = {}

---@param payload TextureSlicer.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TextureSlicer
function TextureSlicer.new(payload) end



---@class ColorMaterial : ReflectReference
---  A [2d material](Material2d) that renders [2d meshes](crate::Mesh2d) with a texture tinted by a uniform color
---@field  color? Color
---@field  alpha_mode? AlphaMode2d
---@field  uv_transform? Affine2
---@field  texture? Handle<Image> | nil
local ColorMaterial = {}

---@param payload ColorMaterial.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ColorMaterial
function ColorMaterial.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<ColorMaterial>
function ColorMaterial.load(path) end

---@param value ColorMaterial
--- The asset value.
---@return Handle<ColorMaterial>
function ColorMaterial.add_asset(value) end

---@param handle Handle<ColorMaterial>
--- The handle to the asset.
---@return ColorMaterial | nil
function ColorMaterial.get_asset(handle) end



---@class AlphaMode2d : ReflectReference
---  Sets how a 2d material's base color alpha channel is used for transparency.
---  Currently, this only works with [`Mesh2d`]. Sprites are always transparent.
--- 
---  This is very similar to [`AlphaMode`] but this only applies to 2d meshes.
---  We use a separate type because 2d doesn't support all the transparency modes that 3d does.
---@field  Opaque AlphaMode2d.Opaque
---@field  Mask AlphaMode2d.Mask
---@field  Blend AlphaMode2d.Blend
local AlphaMode2d = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AlphaMode2d
function AlphaMode2d.new(payload) end



---@class AlphaMode2d.Opaque : EnumVariantRef
--- The `Opaque` variant of `AlphaMode2d`.
AlphaMode2d.Opaque = {}

---@param variant AlphaMode2d.Opaque
---@param payload NoFields
---@return AlphaMode2d
function AlphaMode2d.Opaque.new(variant,payload) end



---@class AlphaMode2d.Mask : EnumVariantRef
--- The `Mask` variant of `AlphaMode2d`.
AlphaMode2d.Mask = {}

---@param variant AlphaMode2d.Mask
---@param payload AlphaMode2d.Mask.Fields
---@return AlphaMode2d
function AlphaMode2d.Mask.new(variant,payload) end



---@class AlphaMode2d.Blend : EnumVariantRef
--- The `Blend` variant of `AlphaMode2d`.
AlphaMode2d.Blend = {}

---@param variant AlphaMode2d.Blend
---@param payload NoFields
---@return AlphaMode2d
function AlphaMode2d.Blend.new(variant,payload) end



---@class SpriteMeshMaterial : ReflectReference
---@field  image? Handle<Image>
---@field  texture_atlas? TextureAtlas | nil
---@field  color? Color
---@field  flip_x? boolean
---@field  flip_y? boolean
---@field  custom_size? Vec2 | nil
---@field  rect? Rect | nil
---@field  image_mode? SpriteImageMode
---@field  alpha_mode? AlphaMode2d
---@field  anchor? Vec2
---@field  texture_atlas_layout? TextureAtlasLayout | nil
---@field  texture_atlas_index? integer
local SpriteMeshMaterial = {}

---@param payload SpriteMeshMaterial.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SpriteMeshMaterial
function SpriteMeshMaterial.new(payload) end

---@param path string
--- The asset path, relative to the assets folder.
---@return Handle<SpriteMeshMaterial>
function SpriteMeshMaterial.load(path) end

---@param value SpriteMeshMaterial
--- The asset value.
---@return Handle<SpriteMeshMaterial>
function SpriteMeshMaterial.add_asset(value) end

---@param handle Handle<SpriteMeshMaterial>
--- The handle to the asset.
---@return SpriteMeshMaterial | nil
function SpriteMeshMaterial.get_asset(handle) end



---@class ReflectableScheduleLabel : ReflectReference
local ReflectableScheduleLabel = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ReflectableScheduleLabel
function ReflectableScheduleLabel.new(payload) end



---@class EmSize : ReflectReference
---  The font size, in logical pixels, used to resolve `Val::Em` values in a UI node.
--- 
---  `Em` units are relative to the font size of the node they sit on, and this is that
---  font size made concrete. Required by `Node`, so every UI node has one.
--- 
---  If the node has a `TextFont`, the `EmSize` is derived from it whenever the `TextFont`,
---  `RemSize` or render-target info changes; until then any value you place on it persists.
---  If it does not have a `TextFont`, the value is yours to set, for example by an app-level
---  propagation system. `EmSize` defaults to [`DEFAULT_REM_SIZE_PX`], which matches the default
---  [`RemSize`] but does not track changes to it. Use `Val::Rem` instead for values that
---  should follow the root font size.
--- 
---  Removing a `TextFont` leaves the last derived value in place.
---@field  [1] number
local EmSize = {}

---@param payload EmSize.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return EmSize
function EmSize.new(payload) end



---@class FontFeatureTag : ReflectReference
---  An OpenType font feature tag.
---@field  [1] integer[]
local FontFeatureTag = {}

---@param payload FontFeatureTag.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontFeatureTag
function FontFeatureTag.new(payload) end



---@class FontFeatures : ReflectReference
---  OpenType features for .otf fonts that support them.
--- 
---  Examples features include ligatures, small-caps, and fractional number display. For the complete
---  list of OpenType features, see the spec at
---  `<https://learn.microsoft.com/en-us/typography/opentype/spec/featurelist>`.
--- 
---  # Usage:
---  ```
---  use bevy_text::{FontFeatureTag, FontFeatures};
--- 
---  // Create using the builder
---  let font_features = FontFeatures::builder()
---    .enable(FontFeatureTag::STANDARD_LIGATURES)
---    .set(FontFeatureTag::WEIGHT, 300)
---    .build();
--- 
---  // Create from a list
---  let more_font_features: FontFeatures = [
---    FontFeatureTag::STANDARD_LIGATURES,
---    FontFeatureTag::OLDSTYLE_FIGURES,
---    FontFeatureTag::TABULAR_FIGURES
---  ].into();
---  ```
---@field  features? [FontFeatureTag, integer][]
local FontFeatures = {}

---@param payload FontFeatures.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontFeatures
function FontFeatures.new(payload) end



---@class FontSize : ReflectReference
---  The vertical height of rasterized glyphs in the font atlas in pixels.
--- 
---  This is multiplied by the scale factor, but not the text entity
---  transform or camera projection.
--- 
---  The viewport variants are not supported by `Text2d`.
--- 
---  A new font atlas is generated for every combination of font handle and scaled font size
---  which can have a strong performance impact.
---@field  Px FontSize.Px
---@field  Vw FontSize.Vw
---@field  Vh FontSize.Vh
---@field  VMin FontSize.VMin
---@field  VMax FontSize.VMax
---@field  Rem FontSize.Rem
local FontSize = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontSize
function FontSize.new(payload) end



---@class FontSize.Px : EnumVariantRef
--- The `Px` variant of `FontSize`.
FontSize.Px = {}

---@param variant FontSize.Px
---@param payload FontSize.Px.Fields
---@return FontSize
function FontSize.Px.new(variant,payload) end



---@class FontSize.Vw : EnumVariantRef
--- The `Vw` variant of `FontSize`.
FontSize.Vw = {}

---@param variant FontSize.Vw
---@param payload FontSize.Vw.Fields
---@return FontSize
function FontSize.Vw.new(variant,payload) end



---@class FontSize.Vh : EnumVariantRef
--- The `Vh` variant of `FontSize`.
FontSize.Vh = {}

---@param variant FontSize.Vh
---@param payload FontSize.Vh.Fields
---@return FontSize
function FontSize.Vh.new(variant,payload) end



---@class FontSize.VMin : EnumVariantRef
--- The `VMin` variant of `FontSize`.
FontSize.VMin = {}

---@param variant FontSize.VMin
---@param payload FontSize.VMin.Fields
---@return FontSize
function FontSize.VMin.new(variant,payload) end



---@class FontSize.VMax : EnumVariantRef
--- The `VMax` variant of `FontSize`.
FontSize.VMax = {}

---@param variant FontSize.VMax
---@param payload FontSize.VMax.Fields
---@return FontSize
function FontSize.VMax.new(variant,payload) end



---@class FontSize.Rem : EnumVariantRef
--- The `Rem` variant of `FontSize`.
FontSize.Rem = {}

---@param variant FontSize.Rem
---@param payload FontSize.Rem.Fields
---@return FontSize
function FontSize.Rem.new(variant,payload) end



---@class FontSmoothing : ReflectReference
---  Determines which antialiasing method to use when rendering text. By default, text is
---  rendered with grayscale antialiasing, but this can be changed to achieve a pixelated look.
--- 
---  **Note:** Subpixel antialiasing is not currently supported.
---@field  None FontSmoothing.None
---@field  AntiAliased FontSmoothing.AntiAliased
local FontSmoothing = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontSmoothing
function FontSmoothing.new(payload) end



---@class FontSmoothing.None : EnumVariantRef
--- The `None` variant of `FontSmoothing`.
FontSmoothing.None = {}

---@param variant FontSmoothing.None
---@param payload NoFields
---@return FontSmoothing
function FontSmoothing.None.new(variant,payload) end



---@class FontSmoothing.AntiAliased : EnumVariantRef
--- The `AntiAliased` variant of `FontSmoothing`.
FontSmoothing.AntiAliased = {}

---@param variant FontSmoothing.AntiAliased
---@param payload NoFields
---@return FontSmoothing
function FontSmoothing.AntiAliased.new(variant,payload) end



---@class FontSource : ReflectReference
---  Determines how the font face for a text sections is selected.
--- 
---  A [`FontSource`] can be a handle to a font asset, a font family name,
---  a CSS font-family list, an ordered family list, or a generic font category
---  that is resolved using Parley's font database.
--- 
---  Font family fallback (selection of a font when the requested font is not found)
---  is automatically handled by [`parley::fontique`].
---  Be sure to enable the `parley/system` feature for automatic discovery of system fonts.
--- 
---  Generally speaking, these fallbacks are OS-specific,
---  and do not require manual configuration.
--- 
---  You can check which font family is used for a given [`FontSource`]
---  by calling [`FontCx::get_family`](crate::FontCx::get_family).
---@field  Handle FontSource.Handle
---@field  Family FontSource.Family
---@field  Families FontSource.Families
---@field  List FontSource.List
---@field  Generic FontSource.Generic
local FontSource = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontSource
function FontSource.new(payload) end



---@class FontSource.Handle : EnumVariantRef
--- The `Handle` variant of `FontSource`.
FontSource.Handle = {}

---@param variant FontSource.Handle
---@param payload FontSource.Handle.Fields
---@return FontSource
function FontSource.Handle.new(variant,payload) end



---@class FontSource.Family : EnumVariantRef
--- The `Family` variant of `FontSource`.
FontSource.Family = {}

---@param variant FontSource.Family
---@param payload FontSource.Family.Fields
---@return FontSource
function FontSource.Family.new(variant,payload) end



---@class FontSource.Families : EnumVariantRef
--- The `Families` variant of `FontSource`.
FontSource.Families = {}

---@param variant FontSource.Families
---@param payload FontSource.Families.Fields
---@return FontSource
function FontSource.Families.new(variant,payload) end



---@class FontSource.List : EnumVariantRef
--- The `List` variant of `FontSource`.
FontSource.List = {}

---@param variant FontSource.List
---@param payload FontSource.List.Fields
---@return FontSource
function FontSource.List.new(variant,payload) end



---@class FontSource.Generic : EnumVariantRef
--- The `Generic` variant of `FontSource`.
FontSource.Generic = {}

---@param variant FontSource.Generic
---@param payload FontSource.Generic.Fields
---@return FontSource
function FontSource.Generic.new(variant,payload) end



---@class FontStyle : ReflectReference
---  The slant style of a font face: normal, italic, or oblique.
---@field  Normal FontStyle.Normal
---@field  Italic FontStyle.Italic
---@field  Oblique FontStyle.Oblique
local FontStyle = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontStyle
function FontStyle.new(payload) end



---@class FontStyle.Normal : EnumVariantRef
--- The `Normal` variant of `FontStyle`.
FontStyle.Normal = {}

---@param variant FontStyle.Normal
---@param payload NoFields
---@return FontStyle
function FontStyle.Normal.new(variant,payload) end



---@class FontStyle.Italic : EnumVariantRef
--- The `Italic` variant of `FontStyle`.
FontStyle.Italic = {}

---@param variant FontStyle.Italic
---@param payload NoFields
---@return FontStyle
function FontStyle.Italic.new(variant,payload) end



---@class FontStyle.Oblique : EnumVariantRef
--- The `Oblique` variant of `FontStyle`.
FontStyle.Oblique = {}

---@param variant FontStyle.Oblique
---@param payload FontStyle.Oblique.Fields
---@return FontStyle
function FontStyle.Oblique.new(variant,payload) end



---@class FontVariationTag : ReflectReference
---  An OpenType font variation tag.
---@field  [1] integer[]
local FontVariationTag = {}

---@param payload FontVariationTag.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontVariationTag
function FontVariationTag.new(payload) end



---@class FontVariations : ReflectReference
---  OpenType font variations for variable fonts that support them.
--- 
---  Variable fonts expose named axes (e.g. `wght`, `FILL`) that accept continuous `f32` values.
---  This is distinct from [`FontFeatures`], which mainly controls on/off OpenType layout features.
--- 
---  # Usage
---  ```
---  use bevy_text::{FontVariationTag, FontVariations};
--- 
---  let variations = FontVariations::builder()
---      .set(FontVariationTag::WEIGHT, 400.0)
---      .build();
---  ```
---@field  variations? [FontVariationTag, number][]
local FontVariations = {}

---@param payload FontVariations.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontVariations
function FontVariations.new(payload) end



---@class FontWeight : ReflectReference
---  How thick or bold the strokes of a font appear.
--- 
---  Valid font weights range from 1 to 1000, inclusive.
---  Weights above 1000 are clamped to 1000.
---  A weight of 0 is treated as [`FontWeight::DEFAULT`].
--- 
---  Legacy names from when most fonts weren't variable fonts
---  are included as const values, but are misleading if
---  used in documentation and examples, as valid weights
---  for variable fonts are all of the numbers from 1-1000, and
---  not all fonts which are not variable fonts have those weights
---  supplied. If you use a custom font that supplies only specific
---  weights, that will be documented where you purchased the font.
--- 
---  `<https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-weight>`
---@field  [1] integer
local FontWeight = {}

---@param payload FontWeight.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontWeight
function FontWeight.new(payload) end



---@class FontWidth : ReflectReference
---  The visual width of a font as a ratio of its normal width, typically 0.5 to 2.0.
---  `<https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass>`
---@field  [1] number
local FontWidth = {}

---@param payload FontWidth.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FontWidth
function FontWidth.new(payload) end



---@class GenericFontFamily : ReflectReference
---  Generic font families that are resolved through Parley's font database.
---@field  Serif GenericFontFamily.Serif
---@field  SansSerif GenericFontFamily.SansSerif
---@field  Cursive GenericFontFamily.Cursive
---@field  Fantasy GenericFontFamily.Fantasy
---@field  Monospace GenericFontFamily.Monospace
---@field  SystemUi GenericFontFamily.SystemUi
---@field  UiSerif GenericFontFamily.UiSerif
---@field  UiSansSerif GenericFontFamily.UiSansSerif
---@field  UiMonospace GenericFontFamily.UiMonospace
---@field  UiRounded GenericFontFamily.UiRounded
---@field  Emoji GenericFontFamily.Emoji
---@field  Math GenericFontFamily.Math
---@field  FangSong GenericFontFamily.FangSong
local GenericFontFamily = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GenericFontFamily
function GenericFontFamily.new(payload) end



---@class GenericFontFamily.Serif : EnumVariantRef
--- The `Serif` variant of `GenericFontFamily`.
GenericFontFamily.Serif = {}

---@param variant GenericFontFamily.Serif
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Serif.new(variant,payload) end



---@class GenericFontFamily.SansSerif : EnumVariantRef
--- The `SansSerif` variant of `GenericFontFamily`.
GenericFontFamily.SansSerif = {}

---@param variant GenericFontFamily.SansSerif
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.SansSerif.new(variant,payload) end



---@class GenericFontFamily.Cursive : EnumVariantRef
--- The `Cursive` variant of `GenericFontFamily`.
GenericFontFamily.Cursive = {}

---@param variant GenericFontFamily.Cursive
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Cursive.new(variant,payload) end



---@class GenericFontFamily.Fantasy : EnumVariantRef
--- The `Fantasy` variant of `GenericFontFamily`.
GenericFontFamily.Fantasy = {}

---@param variant GenericFontFamily.Fantasy
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Fantasy.new(variant,payload) end



---@class GenericFontFamily.Monospace : EnumVariantRef
--- The `Monospace` variant of `GenericFontFamily`.
GenericFontFamily.Monospace = {}

---@param variant GenericFontFamily.Monospace
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Monospace.new(variant,payload) end



---@class GenericFontFamily.SystemUi : EnumVariantRef
--- The `SystemUi` variant of `GenericFontFamily`.
GenericFontFamily.SystemUi = {}

---@param variant GenericFontFamily.SystemUi
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.SystemUi.new(variant,payload) end



---@class GenericFontFamily.UiSerif : EnumVariantRef
--- The `UiSerif` variant of `GenericFontFamily`.
GenericFontFamily.UiSerif = {}

---@param variant GenericFontFamily.UiSerif
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.UiSerif.new(variant,payload) end



---@class GenericFontFamily.UiSansSerif : EnumVariantRef
--- The `UiSansSerif` variant of `GenericFontFamily`.
GenericFontFamily.UiSansSerif = {}

---@param variant GenericFontFamily.UiSansSerif
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.UiSansSerif.new(variant,payload) end



---@class GenericFontFamily.UiMonospace : EnumVariantRef
--- The `UiMonospace` variant of `GenericFontFamily`.
GenericFontFamily.UiMonospace = {}

---@param variant GenericFontFamily.UiMonospace
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.UiMonospace.new(variant,payload) end



---@class GenericFontFamily.UiRounded : EnumVariantRef
--- The `UiRounded` variant of `GenericFontFamily`.
GenericFontFamily.UiRounded = {}

---@param variant GenericFontFamily.UiRounded
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.UiRounded.new(variant,payload) end



---@class GenericFontFamily.Emoji : EnumVariantRef
--- The `Emoji` variant of `GenericFontFamily`.
GenericFontFamily.Emoji = {}

---@param variant GenericFontFamily.Emoji
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Emoji.new(variant,payload) end



---@class GenericFontFamily.Math : EnumVariantRef
--- The `Math` variant of `GenericFontFamily`.
GenericFontFamily.Math = {}

---@param variant GenericFontFamily.Math
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.Math.new(variant,payload) end



---@class GenericFontFamily.FangSong : EnumVariantRef
--- The `FangSong` variant of `GenericFontFamily`.
GenericFontFamily.FangSong = {}

---@param variant GenericFontFamily.FangSong
---@param payload NoFields
---@return GenericFontFamily
function GenericFontFamily.FangSong.new(variant,payload) end



---@class Justify : ReflectReference
---  Describes the horizontal alignment of multiple lines of text relative to each other.
--- 
---  This only affects the internal positioning of the lines of text within a text entity and
---  does not affect the text entity's position.
--- 
---  _Has no affect on a single line text entity_, unless used together with a
---  [`TextBounds`](super::bounds::TextBounds) component with an explicit `width` value.
---@field  Left Justify.Left
---@field  Center Justify.Center
---@field  Right Justify.Right
---@field  Justified Justify.Justified
---@field  Start Justify.Start
---@field  End Justify.End
local Justify = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Justify
function Justify.new(payload) end



---@class Justify.Left : EnumVariantRef
--- The `Left` variant of `Justify`.
Justify.Left = {}

---@param variant Justify.Left
---@param payload NoFields
---@return Justify
function Justify.Left.new(variant,payload) end



---@class Justify.Center : EnumVariantRef
--- The `Center` variant of `Justify`.
Justify.Center = {}

---@param variant Justify.Center
---@param payload NoFields
---@return Justify
function Justify.Center.new(variant,payload) end



---@class Justify.Right : EnumVariantRef
--- The `Right` variant of `Justify`.
Justify.Right = {}

---@param variant Justify.Right
---@param payload NoFields
---@return Justify
function Justify.Right.new(variant,payload) end



---@class Justify.Justified : EnumVariantRef
--- The `Justified` variant of `Justify`.
Justify.Justified = {}

---@param variant Justify.Justified
---@param payload NoFields
---@return Justify
function Justify.Justified.new(variant,payload) end



---@class Justify.Start : EnumVariantRef
--- The `Start` variant of `Justify`.
Justify.Start = {}

---@param variant Justify.Start
---@param payload NoFields
---@return Justify
function Justify.Start.new(variant,payload) end



---@class Justify.End : EnumVariantRef
--- The `End` variant of `Justify`.
Justify.End = {}

---@param variant Justify.End
---@param payload NoFields
---@return Justify
function Justify.End.new(variant,payload) end



---@class LineBreak : ReflectReference
---  Determines how lines will be broken when preventing text from running out of bounds.
---@field  WordBoundary LineBreak.WordBoundary
---@field  AnyCharacter LineBreak.AnyCharacter
---@field  WordOrCharacter LineBreak.WordOrCharacter
---@field  NoWrap LineBreak.NoWrap
local LineBreak = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return LineBreak
function LineBreak.new(payload) end



---@class LineBreak.WordBoundary : EnumVariantRef
--- The `WordBoundary` variant of `LineBreak`.
LineBreak.WordBoundary = {}

---@param variant LineBreak.WordBoundary
---@param payload NoFields
---@return LineBreak
function LineBreak.WordBoundary.new(variant,payload) end



---@class LineBreak.AnyCharacter : EnumVariantRef
--- The `AnyCharacter` variant of `LineBreak`.
LineBreak.AnyCharacter = {}

---@param variant LineBreak.AnyCharacter
---@param payload NoFields
---@return LineBreak
function LineBreak.AnyCharacter.new(variant,payload) end



---@class LineBreak.WordOrCharacter : EnumVariantRef
--- The `WordOrCharacter` variant of `LineBreak`.
LineBreak.WordOrCharacter = {}

---@param variant LineBreak.WordOrCharacter
---@param payload NoFields
---@return LineBreak
function LineBreak.WordOrCharacter.new(variant,payload) end



---@class LineBreak.NoWrap : EnumVariantRef
--- The `NoWrap` variant of `LineBreak`.
LineBreak.NoWrap = {}

---@param variant LineBreak.NoWrap
---@param payload NoFields
---@return LineBreak
function LineBreak.NoWrap.new(variant,payload) end



---@class RemSize : ReflectReference
---  The root font size, in logical pixels.
--- 
---  `Rem` units always resolve against this one global resource, including
---  `FontSize::Rem`, `LetterSpacing::Rem` and `Val::Rem`.
---@field  [1] number
local RemSize = {}

---@param payload RemSize.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RemSize
function RemSize.new(payload) end



---@class Fixed : ReflectReference
---  The fixed timestep game clock following virtual time.
--- 
---  A specialization of the [`Time`] structure. **For method documentation, see
---  [`Time<Fixed>#impl-Time<Fixed>`].**
--- 
---  It is automatically inserted as a resource by
---  [`TimePlugin`](crate::TimePlugin) and updated based on
---  [`Time<Virtual>`](Virtual). The fixed clock is automatically set as the
---  generic [`Time`] resource during [`FixedUpdate`](bevy_app::FixedUpdate)
---  schedule processing.
--- 
---  The fixed timestep clock advances in fixed-size increments, which is
---  extremely useful for writing logic (like physics) that should have
---  consistent behavior, regardless of framerate.
--- 
---  The default [`timestep()`](Time::timestep) is 64 hertz, or 15625
---  microseconds. This value was chosen because using 60 hertz has the potential
---  for a pathological interaction with the monitor refresh rate where the game
---  alternates between running two fixed timesteps and zero fixed timesteps per
---  frame (for example when running two fixed timesteps takes longer than a
---  frame). Additionally, the value is a power of two which losslessly converts
---  into [`f32`] and [`f64`].
--- 
---  To run a system on a fixed timestep, add it to one of the [`FixedMain`]
---  schedules, most commonly [`FixedUpdate`](bevy_app::FixedUpdate).
--- 
---  This schedule is run a number of times between
---  [`PreUpdate`](bevy_app::PreUpdate) and [`Update`](bevy_app::Update)
---  according to the accumulated [`overstep()`](Time::overstep) time divided by
---  the [`timestep()`](Time::timestep). This means the schedule may run 0, 1 or
---  more times during a single update (which typically corresponds to a rendered
---  frame).
--- 
---  `Time<Fixed>` and the generic [`Time`] resource will report a
---  [`delta()`](Time::delta) equal to [`timestep()`](Time::timestep) and always
---  grow [`elapsed()`](Time::elapsed) by one [`timestep()`](Time::timestep) per
---  iteration.
--- 
---  The fixed timestep clock follows the [`Time<Virtual>`](Virtual) clock, which
---  means it is affected by [`pause()`](Time::pause),
---  [`set_relative_speed()`](Time::set_relative_speed) and
---  [`set_max_delta()`](Time::set_max_delta) from virtual time. If the virtual
---  clock is paused, the [`FixedUpdate`](bevy_app::FixedUpdate) schedule will
---  not run. It is guaranteed that the [`elapsed()`](Time::elapsed) time in
---  `Time<Fixed>` is always between the previous `elapsed()` and the current
---  `elapsed()` value in `Time<Virtual>`, so the values are compatible.
--- 
---  Changing the timestep size while the game is running should not normally be
---  done, as having a regular interval is the point of this schedule, but it may
---  be necessary for effects like "bullet-time" if the normal granularity of the
---  fixed timestep is too big for the slowed down time. In this case,
---  [`set_timestep()`](Time::set_timestep) and be called to set a new value. The
---  new value will be used immediately for the next run of the
---  [`FixedUpdate`](bevy_app::FixedUpdate) schedule, meaning that it will affect
---  the [`delta()`](Time::delta) value for the very next
---  [`FixedUpdate`](bevy_app::FixedUpdate), even if it is still during the same
---  frame. Any [`overstep()`](Time::overstep) present in the accumulator will be
---  processed according to the new [`timestep()`](Time::timestep) value.
---@field  timestep? Duration
---@field  overstep? Duration
local Fixed = {}

---@param payload Fixed.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Fixed
function Fixed.new(payload) end



---@class Real : ReflectReference
---  Real time clock representing elapsed wall clock time.
--- 
---  A specialization of the [`Time`] structure. **For method documentation, see
---  [`Time<Real>#impl-Time<Real>`].**
--- 
---  It is automatically inserted as a resource by
---  [`TimePlugin`](crate::TimePlugin) and updated with time instants according
---  to [`TimeUpdateStrategy`](crate::TimeUpdateStrategy).[^disclaimer]
--- 
---  Note:
---  Using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration)
---  allows for mocking the wall clock for testing purposes.
---  Besides this use case, it is not recommended to do this, as it will no longer
---  represent "wall clock" time as intended.
--- 
---  The [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values of this
---  clock should be used for anything which deals specifically with real time
---  (wall clock time). It will not be affected by relative game speed
---  adjustments, pausing or other adjustments.[^disclaimer]
--- 
---  The clock does not count time from [`startup()`](Time::startup) to
---  [`first_update()`](Time::first_update()) into elapsed, but instead will
---  start counting time from the first update call. [`delta()`](Time::delta) and
---  [`elapsed()`](Time::elapsed) will report zero on the first update as there
---  is no previous update instant. This means that a [`delta()`](Time::delta) of
---  zero must be handled without errors in application logic, as it may
---  theoretically also happen at other times.
--- 
---  [`Instant`]s for [`startup()`](Time::startup),
---  [`first_update()`](Time::first_update) and
---  [`last_update()`](Time::last_update) are recorded and accessible.
--- 
---  [^disclaimer]: When using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration),
---      [`Time<Real>#impl-Time<Real>`] is only a *mock* of wall clock time.
--- 
---@field  startup? Instant
---@field  first_update? Instant | nil
---@field  last_update? Instant | nil
local Real = {}

---@param payload Real.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Real
function Real.new(payload) end



---@class TimerMode : ReflectReference
---  Specifies [`Timer`] behavior.
---@field  Once TimerMode.Once
---@field  Repeating TimerMode.Repeating
local TimerMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TimerMode
function TimerMode.new(payload) end



---@class TimerMode.Once : EnumVariantRef
--- The `Once` variant of `TimerMode`.
TimerMode.Once = {}

---@param variant TimerMode.Once
---@param payload NoFields
---@return TimerMode
function TimerMode.Once.new(variant,payload) end



---@class TimerMode.Repeating : EnumVariantRef
--- The `Repeating` variant of `TimerMode`.
TimerMode.Repeating = {}

---@param variant TimerMode.Repeating
---@param payload NoFields
---@return TimerMode
function TimerMode.Repeating.new(variant,payload) end



---@class Virtual : ReflectReference
---  The virtual game clock representing game time.
--- 
---  A specialization of the [`Time`] structure. **For method documentation, see
---  [`Time<Virtual>#impl-Time<Virtual>`].**
--- 
---  Normally used as `Time<Virtual>`. It is automatically inserted as a resource
---  by [`TimePlugin`](crate::TimePlugin) and updated based on
---  [`Time<Real>`](Real). The virtual clock is automatically set as the default
---  generic [`Time`] resource for the update.
--- 
---  The virtual clock differs from real time clock in that it can be paused, sped up
---  and slowed down. It also limits how much it can advance in a single update
---  in order to prevent unexpected behavior in cases where updates do not happen
---  at regular intervals (e.g. coming back after the program was suspended a long time).
--- 
---  The virtual clock can be paused by calling [`pause()`](Time::pause),
---  unpaused by calling [`unpause()`](Time::unpause), or toggled by calling
---  [`toggle()`](Time::toggle). When the game clock is
---  paused [`delta()`](Time::delta) will be zero on each update, and
---  [`elapsed()`](Time::elapsed) will not grow.
---  [`effective_speed()`](Time::effective_speed) will return `0.0`. Calling
---  [`pause()`](Time::pause) will not affect value the [`delta()`](Time::delta)
---  value for the update currently being processed.
--- 
---  The speed of the virtual clock can be changed by calling
---  [`set_relative_speed()`](Time::set_relative_speed). A value of `2.0` means
---  that virtual clock should advance twice as fast as real time, meaning that
---  [`delta()`](Time::delta) values will be double of what
---  [`Time<Real>::delta()`](Time::delta) reports and
---  [`elapsed()`](Time::elapsed) will go twice as fast as
---  [`Time<Real>::elapsed()`](Time::elapsed). Calling
---  [`set_relative_speed()`](Time::set_relative_speed) will not affect the
---  [`delta()`](Time::delta) value for the update currently being processed.
--- 
---  The maximum amount of delta time that can be added by a single update can be
---  set by [`set_max_delta()`](Time::set_max_delta). This value serves a dual
---  purpose in the virtual clock.
--- 
---  If the game temporarily freezes due to any reason, such as disk access, a
---  blocking system call, or operating system level suspend, reporting the full
---  elapsed delta time is likely to cause bugs in game logic. Usually if a
---  laptop is suspended for an hour, it doesn't make sense to try to simulate
---  the game logic for the elapsed hour when resuming. Instead it is better to
---  lose the extra time and pretend a shorter duration of time passed. Setting
---  [`max_delta()`](Time::max_delta) to a relatively short time means that the
---  impact on game logic will be minimal.
--- 
---  If the game lags for some reason, meaning that it will take a longer time to
---  compute a frame than the real time that passes during the computation, then
---  we would fall behind in processing virtual time. If this situation persists,
---  and computing a frame takes longer depending on how much virtual time has
---  passed, the game would enter a "death spiral" where computing each frame
---  takes longer and longer and the game will appear to freeze. By limiting the
---  maximum time that can be added at once, we also limit the amount of virtual
---  time the game needs to compute for each frame. This means that the game will
---  run slow, and it will run slower than real time, but it will not freeze and
---  it will recover as soon as computation becomes fast again.
--- 
---  You should set [`max_delta()`](Time::max_delta) to a value that is
---  approximately the minimum FPS your game should have even if heavily lagged
---  for a moment. The actual FPS when lagged will be somewhat lower than this,
---  depending on how much more time it takes to compute a frame compared to real
---  time. You should also consider how stable your FPS is, as the limit will
---  also dictate how big of an FPS drop you can accept without losing time and
---  falling behind real time.
---@field  max_delta? Duration
---@field  paused? boolean
---@field  relative_speed? number
---@field  effective_speed? number
local Virtual = {}

---@param payload Virtual.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Virtual
function Virtual.new(payload) end



---@class CornerRadius : ReflectReference
---  The horizontal and vertical radii of a rounded corner's elliptical arc.
---  If one field is auto, the resolved radius will be circular and clamped to half the length of the node's shortest side.
---  If either field is zero, or both are auto, the node will have square corners.
--- 
---  # Example
--- 
---  ```
---  # use bevy_math::{Vec2, Vec2Swizzles};
---  # use bevy_ui::{CornerRadius, Val};
---  # use bevy_text::{EmSize, RemSize};
---  let radius = Val::Px(10.0);
---  let size = Vec2::new(100.0, 50.0);
---  let viewport_size = Vec2::new(1920.0, 1080.0);
---  let em_size = EmSize(20.0);
---  let rem_size = RemSize(20.0);
--- 
---  let c1 = CornerRadius {
---      x: radius,
---      y: Val::ZERO,
---  };
---  let c2 = CornerRadius {
---      x: Val::ZERO,
---      y: radius,
---  };
---  let r = c1.resolve(1.0, size, viewport_size, em_size, rem_size);
---  assert_eq!(
---      r,
---      c2.resolve(1.0, size, viewport_size, em_size, rem_size).yx(),
---  );
---  assert_eq!(
---      r,
---      Vec2::new(10.0, 0.0),
---  );
---  ```
---@field  x? Val
---@field  y? Val
local CornerRadius = {}

---@param payload CornerRadius.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return CornerRadius
function CornerRadius.new(payload) end



---@class AlignContent : ReflectReference
---  Used to control how items are distributed.
---  - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
---  - For CSS Grid containers, controls alignment of grid rows.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
---@field  Default AlignContent.Default
---@field  Start AlignContent.Start
---@field  StartSafe AlignContent.StartSafe
---@field  End AlignContent.End
---@field  EndSafe AlignContent.EndSafe
---@field  FlexStart AlignContent.FlexStart
---@field  FlexStartSafe AlignContent.FlexStartSafe
---@field  FlexEnd AlignContent.FlexEnd
---@field  FlexEndSafe AlignContent.FlexEndSafe
---@field  Center AlignContent.Center
---@field  CenterSafe AlignContent.CenterSafe
---@field  Stretch AlignContent.Stretch
---@field  SpaceBetween AlignContent.SpaceBetween
---@field  SpaceEvenly AlignContent.SpaceEvenly
---@field  SpaceAround AlignContent.SpaceAround
local AlignContent = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AlignContent
function AlignContent.new(payload) end



---@class AlignContent.Default : EnumVariantRef
--- The `Default` variant of `AlignContent`.
AlignContent.Default = {}

---@param variant AlignContent.Default
---@param payload NoFields
---@return AlignContent
function AlignContent.Default.new(variant,payload) end



---@class AlignContent.Start : EnumVariantRef
--- The `Start` variant of `AlignContent`.
AlignContent.Start = {}

---@param variant AlignContent.Start
---@param payload NoFields
---@return AlignContent
function AlignContent.Start.new(variant,payload) end



---@class AlignContent.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `AlignContent`.
AlignContent.StartSafe = {}

---@param variant AlignContent.StartSafe
---@param payload NoFields
---@return AlignContent
function AlignContent.StartSafe.new(variant,payload) end



---@class AlignContent.End : EnumVariantRef
--- The `End` variant of `AlignContent`.
AlignContent.End = {}

---@param variant AlignContent.End
---@param payload NoFields
---@return AlignContent
function AlignContent.End.new(variant,payload) end



---@class AlignContent.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `AlignContent`.
AlignContent.EndSafe = {}

---@param variant AlignContent.EndSafe
---@param payload NoFields
---@return AlignContent
function AlignContent.EndSafe.new(variant,payload) end



---@class AlignContent.FlexStart : EnumVariantRef
--- The `FlexStart` variant of `AlignContent`.
AlignContent.FlexStart = {}

---@param variant AlignContent.FlexStart
---@param payload NoFields
---@return AlignContent
function AlignContent.FlexStart.new(variant,payload) end



---@class AlignContent.FlexStartSafe : EnumVariantRef
--- The `FlexStartSafe` variant of `AlignContent`.
AlignContent.FlexStartSafe = {}

---@param variant AlignContent.FlexStartSafe
---@param payload NoFields
---@return AlignContent
function AlignContent.FlexStartSafe.new(variant,payload) end



---@class AlignContent.FlexEnd : EnumVariantRef
--- The `FlexEnd` variant of `AlignContent`.
AlignContent.FlexEnd = {}

---@param variant AlignContent.FlexEnd
---@param payload NoFields
---@return AlignContent
function AlignContent.FlexEnd.new(variant,payload) end



---@class AlignContent.FlexEndSafe : EnumVariantRef
--- The `FlexEndSafe` variant of `AlignContent`.
AlignContent.FlexEndSafe = {}

---@param variant AlignContent.FlexEndSafe
---@param payload NoFields
---@return AlignContent
function AlignContent.FlexEndSafe.new(variant,payload) end



---@class AlignContent.Center : EnumVariantRef
--- The `Center` variant of `AlignContent`.
AlignContent.Center = {}

---@param variant AlignContent.Center
---@param payload NoFields
---@return AlignContent
function AlignContent.Center.new(variant,payload) end



---@class AlignContent.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `AlignContent`.
AlignContent.CenterSafe = {}

---@param variant AlignContent.CenterSafe
---@param payload NoFields
---@return AlignContent
function AlignContent.CenterSafe.new(variant,payload) end



---@class AlignContent.Stretch : EnumVariantRef
--- The `Stretch` variant of `AlignContent`.
AlignContent.Stretch = {}

---@param variant AlignContent.Stretch
---@param payload NoFields
---@return AlignContent
function AlignContent.Stretch.new(variant,payload) end



---@class AlignContent.SpaceBetween : EnumVariantRef
--- The `SpaceBetween` variant of `AlignContent`.
AlignContent.SpaceBetween = {}

---@param variant AlignContent.SpaceBetween
---@param payload NoFields
---@return AlignContent
function AlignContent.SpaceBetween.new(variant,payload) end



---@class AlignContent.SpaceEvenly : EnumVariantRef
--- The `SpaceEvenly` variant of `AlignContent`.
AlignContent.SpaceEvenly = {}

---@param variant AlignContent.SpaceEvenly
---@param payload NoFields
---@return AlignContent
function AlignContent.SpaceEvenly.new(variant,payload) end



---@class AlignContent.SpaceAround : EnumVariantRef
--- The `SpaceAround` variant of `AlignContent`.
AlignContent.SpaceAround = {}

---@param variant AlignContent.SpaceAround
---@param payload NoFields
---@return AlignContent
function AlignContent.SpaceAround.new(variant,payload) end



---@class AlignItems : ReflectReference
---  Used to control how each individual item is aligned by default within the space they're given.
---  - For Flexbox containers, sets default cross axis alignment of the child items.
---  - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
---@field  Default AlignItems.Default
---@field  Start AlignItems.Start
---@field  StartSafe AlignItems.StartSafe
---@field  End AlignItems.End
---@field  EndSafe AlignItems.EndSafe
---@field  FlexStart AlignItems.FlexStart
---@field  FlexStartSafe AlignItems.FlexStartSafe
---@field  FlexEnd AlignItems.FlexEnd
---@field  FlexEndSafe AlignItems.FlexEndSafe
---@field  Center AlignItems.Center
---@field  CenterSafe AlignItems.CenterSafe
---@field  Baseline AlignItems.Baseline
---@field  Stretch AlignItems.Stretch
local AlignItems = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AlignItems
function AlignItems.new(payload) end



---@class AlignItems.Default : EnumVariantRef
--- The `Default` variant of `AlignItems`.
AlignItems.Default = {}

---@param variant AlignItems.Default
---@param payload NoFields
---@return AlignItems
function AlignItems.Default.new(variant,payload) end



---@class AlignItems.Start : EnumVariantRef
--- The `Start` variant of `AlignItems`.
AlignItems.Start = {}

---@param variant AlignItems.Start
---@param payload NoFields
---@return AlignItems
function AlignItems.Start.new(variant,payload) end



---@class AlignItems.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `AlignItems`.
AlignItems.StartSafe = {}

---@param variant AlignItems.StartSafe
---@param payload NoFields
---@return AlignItems
function AlignItems.StartSafe.new(variant,payload) end



---@class AlignItems.End : EnumVariantRef
--- The `End` variant of `AlignItems`.
AlignItems.End = {}

---@param variant AlignItems.End
---@param payload NoFields
---@return AlignItems
function AlignItems.End.new(variant,payload) end



---@class AlignItems.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `AlignItems`.
AlignItems.EndSafe = {}

---@param variant AlignItems.EndSafe
---@param payload NoFields
---@return AlignItems
function AlignItems.EndSafe.new(variant,payload) end



---@class AlignItems.FlexStart : EnumVariantRef
--- The `FlexStart` variant of `AlignItems`.
AlignItems.FlexStart = {}

---@param variant AlignItems.FlexStart
---@param payload NoFields
---@return AlignItems
function AlignItems.FlexStart.new(variant,payload) end



---@class AlignItems.FlexStartSafe : EnumVariantRef
--- The `FlexStartSafe` variant of `AlignItems`.
AlignItems.FlexStartSafe = {}

---@param variant AlignItems.FlexStartSafe
---@param payload NoFields
---@return AlignItems
function AlignItems.FlexStartSafe.new(variant,payload) end



---@class AlignItems.FlexEnd : EnumVariantRef
--- The `FlexEnd` variant of `AlignItems`.
AlignItems.FlexEnd = {}

---@param variant AlignItems.FlexEnd
---@param payload NoFields
---@return AlignItems
function AlignItems.FlexEnd.new(variant,payload) end



---@class AlignItems.FlexEndSafe : EnumVariantRef
--- The `FlexEndSafe` variant of `AlignItems`.
AlignItems.FlexEndSafe = {}

---@param variant AlignItems.FlexEndSafe
---@param payload NoFields
---@return AlignItems
function AlignItems.FlexEndSafe.new(variant,payload) end



---@class AlignItems.Center : EnumVariantRef
--- The `Center` variant of `AlignItems`.
AlignItems.Center = {}

---@param variant AlignItems.Center
---@param payload NoFields
---@return AlignItems
function AlignItems.Center.new(variant,payload) end



---@class AlignItems.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `AlignItems`.
AlignItems.CenterSafe = {}

---@param variant AlignItems.CenterSafe
---@param payload NoFields
---@return AlignItems
function AlignItems.CenterSafe.new(variant,payload) end



---@class AlignItems.Baseline : EnumVariantRef
--- The `Baseline` variant of `AlignItems`.
AlignItems.Baseline = {}

---@param variant AlignItems.Baseline
---@param payload NoFields
---@return AlignItems
function AlignItems.Baseline.new(variant,payload) end



---@class AlignItems.Stretch : EnumVariantRef
--- The `Stretch` variant of `AlignItems`.
AlignItems.Stretch = {}

---@param variant AlignItems.Stretch
---@param payload NoFields
---@return AlignItems
function AlignItems.Stretch.new(variant,payload) end



---@class AlignSelf : ReflectReference
---  Used to control how the specified item is aligned within the space it's given.
---  - For Flexbox items, controls cross axis alignment of the item.
---  - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
---@field  Auto AlignSelf.Auto
---@field  Start AlignSelf.Start
---@field  StartSafe AlignSelf.StartSafe
---@field  End AlignSelf.End
---@field  EndSafe AlignSelf.EndSafe
---@field  FlexStart AlignSelf.FlexStart
---@field  FlexStartSafe AlignSelf.FlexStartSafe
---@field  FlexEnd AlignSelf.FlexEnd
---@field  FlexEndSafe AlignSelf.FlexEndSafe
---@field  Center AlignSelf.Center
---@field  CenterSafe AlignSelf.CenterSafe
---@field  Baseline AlignSelf.Baseline
---@field  Stretch AlignSelf.Stretch
local AlignSelf = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return AlignSelf
function AlignSelf.new(payload) end



---@class AlignSelf.Auto : EnumVariantRef
--- The `Auto` variant of `AlignSelf`.
AlignSelf.Auto = {}

---@param variant AlignSelf.Auto
---@param payload NoFields
---@return AlignSelf
function AlignSelf.Auto.new(variant,payload) end



---@class AlignSelf.Start : EnumVariantRef
--- The `Start` variant of `AlignSelf`.
AlignSelf.Start = {}

---@param variant AlignSelf.Start
---@param payload NoFields
---@return AlignSelf
function AlignSelf.Start.new(variant,payload) end



---@class AlignSelf.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `AlignSelf`.
AlignSelf.StartSafe = {}

---@param variant AlignSelf.StartSafe
---@param payload NoFields
---@return AlignSelf
function AlignSelf.StartSafe.new(variant,payload) end



---@class AlignSelf.End : EnumVariantRef
--- The `End` variant of `AlignSelf`.
AlignSelf.End = {}

---@param variant AlignSelf.End
---@param payload NoFields
---@return AlignSelf
function AlignSelf.End.new(variant,payload) end



---@class AlignSelf.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `AlignSelf`.
AlignSelf.EndSafe = {}

---@param variant AlignSelf.EndSafe
---@param payload NoFields
---@return AlignSelf
function AlignSelf.EndSafe.new(variant,payload) end



---@class AlignSelf.FlexStart : EnumVariantRef
--- The `FlexStart` variant of `AlignSelf`.
AlignSelf.FlexStart = {}

---@param variant AlignSelf.FlexStart
---@param payload NoFields
---@return AlignSelf
function AlignSelf.FlexStart.new(variant,payload) end



---@class AlignSelf.FlexStartSafe : EnumVariantRef
--- The `FlexStartSafe` variant of `AlignSelf`.
AlignSelf.FlexStartSafe = {}

---@param variant AlignSelf.FlexStartSafe
---@param payload NoFields
---@return AlignSelf
function AlignSelf.FlexStartSafe.new(variant,payload) end



---@class AlignSelf.FlexEnd : EnumVariantRef
--- The `FlexEnd` variant of `AlignSelf`.
AlignSelf.FlexEnd = {}

---@param variant AlignSelf.FlexEnd
---@param payload NoFields
---@return AlignSelf
function AlignSelf.FlexEnd.new(variant,payload) end



---@class AlignSelf.FlexEndSafe : EnumVariantRef
--- The `FlexEndSafe` variant of `AlignSelf`.
AlignSelf.FlexEndSafe = {}

---@param variant AlignSelf.FlexEndSafe
---@param payload NoFields
---@return AlignSelf
function AlignSelf.FlexEndSafe.new(variant,payload) end



---@class AlignSelf.Center : EnumVariantRef
--- The `Center` variant of `AlignSelf`.
AlignSelf.Center = {}

---@param variant AlignSelf.Center
---@param payload NoFields
---@return AlignSelf
function AlignSelf.Center.new(variant,payload) end



---@class AlignSelf.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `AlignSelf`.
AlignSelf.CenterSafe = {}

---@param variant AlignSelf.CenterSafe
---@param payload NoFields
---@return AlignSelf
function AlignSelf.CenterSafe.new(variant,payload) end



---@class AlignSelf.Baseline : EnumVariantRef
--- The `Baseline` variant of `AlignSelf`.
AlignSelf.Baseline = {}

---@param variant AlignSelf.Baseline
---@param payload NoFields
---@return AlignSelf
function AlignSelf.Baseline.new(variant,payload) end



---@class AlignSelf.Stretch : EnumVariantRef
--- The `Stretch` variant of `AlignSelf`.
AlignSelf.Stretch = {}

---@param variant AlignSelf.Stretch
---@param payload NoFields
---@return AlignSelf
function AlignSelf.Stretch.new(variant,payload) end



---@class BackgroundColor : ReflectReference
---  The background color of the node
--- 
---  This serves as the "fill" color.
---@field  [1] Color
local BackgroundColor = {}

---@param payload BackgroundColor.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BackgroundColor
function BackgroundColor.new(payload) end



---@class BoxSizing : ReflectReference
---  Which part of a Node's box length styles like width and height control
--- 
---  See: <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
---@field  BorderBox BoxSizing.BorderBox
---@field  ContentBox BoxSizing.ContentBox
local BoxSizing = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BoxSizing
function BoxSizing.new(payload) end



---@class BoxSizing.BorderBox : EnumVariantRef
--- The `BorderBox` variant of `BoxSizing`.
BoxSizing.BorderBox = {}

---@param variant BoxSizing.BorderBox
---@param payload NoFields
---@return BoxSizing
function BoxSizing.BorderBox.new(variant,payload) end



---@class BoxSizing.ContentBox : EnumVariantRef
--- The `ContentBox` variant of `BoxSizing`.
BoxSizing.ContentBox = {}

---@param variant BoxSizing.ContentBox
---@param payload NoFields
---@return BoxSizing
function BoxSizing.ContentBox.new(variant,payload) end



---@class Display : ReflectReference
---  Defines the layout model used by this node.
--- 
---  Part of the [`Node`] component.
---@field  Flex Display.Flex
---@field  Grid Display.Grid
---@field  Block Display.Block
---@field  None Display.None
local Display = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Display
function Display.new(payload) end



---@class Display.Flex : EnumVariantRef
--- The `Flex` variant of `Display`.
Display.Flex = {}

---@param variant Display.Flex
---@param payload NoFields
---@return Display
function Display.Flex.new(variant,payload) end



---@class Display.Grid : EnumVariantRef
--- The `Grid` variant of `Display`.
Display.Grid = {}

---@param variant Display.Grid
---@param payload NoFields
---@return Display
function Display.Grid.new(variant,payload) end



---@class Display.Block : EnumVariantRef
--- The `Block` variant of `Display`.
Display.Block = {}

---@param variant Display.Block
---@param payload NoFields
---@return Display
function Display.Block.new(variant,payload) end



---@class Display.None : EnumVariantRef
--- The `None` variant of `Display`.
Display.None = {}

---@param variant Display.None
---@param payload NoFields
---@return Display
function Display.None.new(variant,payload) end



---@class FlexDirection : ReflectReference
---  Defines how flexbox items are ordered within a flexbox
---@field  Row FlexDirection.Row
---@field  Column FlexDirection.Column
---@field  RowReverse FlexDirection.RowReverse
---@field  ColumnReverse FlexDirection.ColumnReverse
local FlexDirection = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FlexDirection
function FlexDirection.new(payload) end



---@class FlexDirection.Row : EnumVariantRef
--- The `Row` variant of `FlexDirection`.
FlexDirection.Row = {}

---@param variant FlexDirection.Row
---@param payload NoFields
---@return FlexDirection
function FlexDirection.Row.new(variant,payload) end



---@class FlexDirection.Column : EnumVariantRef
--- The `Column` variant of `FlexDirection`.
FlexDirection.Column = {}

---@param variant FlexDirection.Column
---@param payload NoFields
---@return FlexDirection
function FlexDirection.Column.new(variant,payload) end



---@class FlexDirection.RowReverse : EnumVariantRef
--- The `RowReverse` variant of `FlexDirection`.
FlexDirection.RowReverse = {}

---@param variant FlexDirection.RowReverse
---@param payload NoFields
---@return FlexDirection
function FlexDirection.RowReverse.new(variant,payload) end



---@class FlexDirection.ColumnReverse : EnumVariantRef
--- The `ColumnReverse` variant of `FlexDirection`.
FlexDirection.ColumnReverse = {}

---@param variant FlexDirection.ColumnReverse
---@param payload NoFields
---@return FlexDirection
function FlexDirection.ColumnReverse.new(variant,payload) end



---@class FlexWrap : ReflectReference
---  Defines if flexbox items appear on a single line or on multiple lines
---@field  NoWrap FlexWrap.NoWrap
---@field  Wrap FlexWrap.Wrap
---@field  WrapReverse FlexWrap.WrapReverse
local FlexWrap = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return FlexWrap
function FlexWrap.new(payload) end



---@class FlexWrap.NoWrap : EnumVariantRef
--- The `NoWrap` variant of `FlexWrap`.
FlexWrap.NoWrap = {}

---@param variant FlexWrap.NoWrap
---@param payload NoFields
---@return FlexWrap
function FlexWrap.NoWrap.new(variant,payload) end



---@class FlexWrap.Wrap : EnumVariantRef
--- The `Wrap` variant of `FlexWrap`.
FlexWrap.Wrap = {}

---@param variant FlexWrap.Wrap
---@param payload NoFields
---@return FlexWrap
function FlexWrap.Wrap.new(variant,payload) end



---@class FlexWrap.WrapReverse : EnumVariantRef
--- The `WrapReverse` variant of `FlexWrap`.
FlexWrap.WrapReverse = {}

---@param variant FlexWrap.WrapReverse
---@param payload NoFields
---@return FlexWrap
function FlexWrap.WrapReverse.new(variant,payload) end



---@class GlobalZIndex : ReflectReference
---  `GlobalZIndex` allows a [`Node`] entity anywhere in the UI hierarchy to escape the implicit draw ordering of the UI's layout tree and
---  be rendered above or below other UI nodes.
---  Root UI nodes without a `GlobalZIndex` component receive an implicit global z-index of `0`.
---  Nodes with a `GlobalZIndex` of greater than 0 will be drawn on top of nodes without a `GlobalZIndex` or nodes with a lower `GlobalZIndex`.
---  Nodes with a `GlobalZIndex` of less than 0 will be drawn below nodes without a `GlobalZIndex` or nodes with a greater `GlobalZIndex`.
---  The order of nodes with the same `GlobalZIndex` is stable between frames.
--- 
---  If two Nodes have the same `GlobalZIndex`, ties are decided in order by:
--- 
---  * The node with the higher `ZIndex`.
---  * The node that was newly added this frame.
---  * The node with a changed `GlobalZIndex` or `ZIndex`.
--- 
---  Otherwise the order is preserved from the previous frame.
---@field  [1] integer
local GlobalZIndex = {}

---@param payload GlobalZIndex.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GlobalZIndex
function GlobalZIndex.new(payload) end



---@class GridAutoFlow : ReflectReference
---  Controls whether grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.
--- 
---  The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later.
---  This may cause items to appear out-of-order when doing so would fill in holes left by larger items.
--- 
---  Defaults to [`GridAutoFlow::Row`].
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
---@field  Row GridAutoFlow.Row
---@field  Column GridAutoFlow.Column
---@field  RowDense GridAutoFlow.RowDense
---@field  ColumnDense GridAutoFlow.ColumnDense
local GridAutoFlow = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GridAutoFlow
function GridAutoFlow.new(payload) end



---@class GridAutoFlow.Row : EnumVariantRef
--- The `Row` variant of `GridAutoFlow`.
GridAutoFlow.Row = {}

---@param variant GridAutoFlow.Row
---@param payload NoFields
---@return GridAutoFlow
function GridAutoFlow.Row.new(variant,payload) end



---@class GridAutoFlow.Column : EnumVariantRef
--- The `Column` variant of `GridAutoFlow`.
GridAutoFlow.Column = {}

---@param variant GridAutoFlow.Column
---@param payload NoFields
---@return GridAutoFlow
function GridAutoFlow.Column.new(variant,payload) end



---@class GridAutoFlow.RowDense : EnumVariantRef
--- The `RowDense` variant of `GridAutoFlow`.
GridAutoFlow.RowDense = {}

---@param variant GridAutoFlow.RowDense
---@param payload NoFields
---@return GridAutoFlow
function GridAutoFlow.RowDense.new(variant,payload) end



---@class GridAutoFlow.ColumnDense : EnumVariantRef
--- The `ColumnDense` variant of `GridAutoFlow`.
GridAutoFlow.ColumnDense = {}

---@param variant GridAutoFlow.ColumnDense
---@param payload NoFields
---@return GridAutoFlow
function GridAutoFlow.ColumnDense.new(variant,payload) end



---@class GridPlacement : ReflectReference
---  Represents the position of a grid item in a single axis.
--- 
---  There are 3 fields which may be set:
---    - `start`: which grid line the item should start at
---    - `end`: which grid line the item should end at
---    - `span`: how many tracks the item should span
--- 
---  The default `span` is 1. If neither `start` or `end` is set then the item will be placed automatically.
--- 
---  Generally, at most two fields should be set. If all three fields are specified then `span` will be ignored. If `end` specifies an earlier
---  grid line than `start` then `end` will be ignored and the item will have a span of 1.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout/Line-based_Placement_with_CSS_Grid>
---@field  start? NonZeroI16 | nil
---@field  span? NonZeroU16 | nil
---@field  end? NonZeroI16 | nil
local GridPlacement = {}

---@param payload GridPlacement.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GridPlacement
function GridPlacement.new(payload) end



---@class GridTrack : ReflectReference
---  A [`GridTrack`] is a Row or Column of a CSS Grid. This struct specifies what size the track should be.
---  See below for the different "track sizing functions" you can specify.
---@field  min_sizing_function? MinTrackSizingFunction
---@field  max_sizing_function? MaxTrackSizingFunction
local GridTrack = {}

---@param payload GridTrack.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GridTrack
function GridTrack.new(payload) end



---@class GridTrackRepetition : ReflectReference
---  How many times to repeat a repeated grid track
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/repeat>
---@field  Count GridTrackRepetition.Count
---@field  AutoFill GridTrackRepetition.AutoFill
---@field  AutoFit GridTrackRepetition.AutoFit
local GridTrackRepetition = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return GridTrackRepetition
function GridTrackRepetition.new(payload) end



---@class GridTrackRepetition.Count : EnumVariantRef
--- The `Count` variant of `GridTrackRepetition`.
GridTrackRepetition.Count = {}

---@param variant GridTrackRepetition.Count
---@param payload GridTrackRepetition.Count.Fields
---@return GridTrackRepetition
function GridTrackRepetition.Count.new(variant,payload) end



---@class GridTrackRepetition.AutoFill : EnumVariantRef
--- The `AutoFill` variant of `GridTrackRepetition`.
GridTrackRepetition.AutoFill = {}

---@param variant GridTrackRepetition.AutoFill
---@param payload NoFields
---@return GridTrackRepetition
function GridTrackRepetition.AutoFill.new(variant,payload) end



---@class GridTrackRepetition.AutoFit : EnumVariantRef
--- The `AutoFit` variant of `GridTrackRepetition`.
GridTrackRepetition.AutoFit = {}

---@param variant GridTrackRepetition.AutoFit
---@param payload NoFields
---@return GridTrackRepetition
function GridTrackRepetition.AutoFit.new(variant,payload) end



---@class InlineDirection : ReflectReference
---  Sets the inline axis direction (LTR or RTL) used for layout.
---@field  Ltr InlineDirection.Ltr
---@field  Rtl InlineDirection.Rtl
local InlineDirection = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InlineDirection
function InlineDirection.new(payload) end



---@class InlineDirection.Ltr : EnumVariantRef
--- The `Ltr` variant of `InlineDirection`.
InlineDirection.Ltr = {}

---@param variant InlineDirection.Ltr
---@param payload NoFields
---@return InlineDirection
function InlineDirection.Ltr.new(variant,payload) end



---@class InlineDirection.Rtl : EnumVariantRef
--- The `Rtl` variant of `InlineDirection`.
InlineDirection.Rtl = {}

---@param variant InlineDirection.Rtl
---@param payload NoFields
---@return InlineDirection
function InlineDirection.Rtl.new(variant,payload) end



---@class JustifyContent : ReflectReference
---  Used to control how items are distributed.
---  - For Flexbox containers, controls alignment of items in the main axis.
---  - For CSS Grid containers, controls alignment of grid columns.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
---@field  Default JustifyContent.Default
---@field  Start JustifyContent.Start
---@field  StartSafe JustifyContent.StartSafe
---@field  End JustifyContent.End
---@field  EndSafe JustifyContent.EndSafe
---@field  FlexStart JustifyContent.FlexStart
---@field  FlexStartSafe JustifyContent.FlexStartSafe
---@field  FlexEnd JustifyContent.FlexEnd
---@field  FlexEndSafe JustifyContent.FlexEndSafe
---@field  Center JustifyContent.Center
---@field  CenterSafe JustifyContent.CenterSafe
---@field  Stretch JustifyContent.Stretch
---@field  SpaceBetween JustifyContent.SpaceBetween
---@field  SpaceEvenly JustifyContent.SpaceEvenly
---@field  SpaceAround JustifyContent.SpaceAround
local JustifyContent = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JustifyContent
function JustifyContent.new(payload) end



---@class JustifyContent.Default : EnumVariantRef
--- The `Default` variant of `JustifyContent`.
JustifyContent.Default = {}

---@param variant JustifyContent.Default
---@param payload NoFields
---@return JustifyContent
function JustifyContent.Default.new(variant,payload) end



---@class JustifyContent.Start : EnumVariantRef
--- The `Start` variant of `JustifyContent`.
JustifyContent.Start = {}

---@param variant JustifyContent.Start
---@param payload NoFields
---@return JustifyContent
function JustifyContent.Start.new(variant,payload) end



---@class JustifyContent.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `JustifyContent`.
JustifyContent.StartSafe = {}

---@param variant JustifyContent.StartSafe
---@param payload NoFields
---@return JustifyContent
function JustifyContent.StartSafe.new(variant,payload) end



---@class JustifyContent.End : EnumVariantRef
--- The `End` variant of `JustifyContent`.
JustifyContent.End = {}

---@param variant JustifyContent.End
---@param payload NoFields
---@return JustifyContent
function JustifyContent.End.new(variant,payload) end



---@class JustifyContent.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `JustifyContent`.
JustifyContent.EndSafe = {}

---@param variant JustifyContent.EndSafe
---@param payload NoFields
---@return JustifyContent
function JustifyContent.EndSafe.new(variant,payload) end



---@class JustifyContent.FlexStart : EnumVariantRef
--- The `FlexStart` variant of `JustifyContent`.
JustifyContent.FlexStart = {}

---@param variant JustifyContent.FlexStart
---@param payload NoFields
---@return JustifyContent
function JustifyContent.FlexStart.new(variant,payload) end



---@class JustifyContent.FlexStartSafe : EnumVariantRef
--- The `FlexStartSafe` variant of `JustifyContent`.
JustifyContent.FlexStartSafe = {}

---@param variant JustifyContent.FlexStartSafe
---@param payload NoFields
---@return JustifyContent
function JustifyContent.FlexStartSafe.new(variant,payload) end



---@class JustifyContent.FlexEnd : EnumVariantRef
--- The `FlexEnd` variant of `JustifyContent`.
JustifyContent.FlexEnd = {}

---@param variant JustifyContent.FlexEnd
---@param payload NoFields
---@return JustifyContent
function JustifyContent.FlexEnd.new(variant,payload) end



---@class JustifyContent.FlexEndSafe : EnumVariantRef
--- The `FlexEndSafe` variant of `JustifyContent`.
JustifyContent.FlexEndSafe = {}

---@param variant JustifyContent.FlexEndSafe
---@param payload NoFields
---@return JustifyContent
function JustifyContent.FlexEndSafe.new(variant,payload) end



---@class JustifyContent.Center : EnumVariantRef
--- The `Center` variant of `JustifyContent`.
JustifyContent.Center = {}

---@param variant JustifyContent.Center
---@param payload NoFields
---@return JustifyContent
function JustifyContent.Center.new(variant,payload) end



---@class JustifyContent.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `JustifyContent`.
JustifyContent.CenterSafe = {}

---@param variant JustifyContent.CenterSafe
---@param payload NoFields
---@return JustifyContent
function JustifyContent.CenterSafe.new(variant,payload) end



---@class JustifyContent.Stretch : EnumVariantRef
--- The `Stretch` variant of `JustifyContent`.
JustifyContent.Stretch = {}

---@param variant JustifyContent.Stretch
---@param payload NoFields
---@return JustifyContent
function JustifyContent.Stretch.new(variant,payload) end



---@class JustifyContent.SpaceBetween : EnumVariantRef
--- The `SpaceBetween` variant of `JustifyContent`.
JustifyContent.SpaceBetween = {}

---@param variant JustifyContent.SpaceBetween
---@param payload NoFields
---@return JustifyContent
function JustifyContent.SpaceBetween.new(variant,payload) end



---@class JustifyContent.SpaceEvenly : EnumVariantRef
--- The `SpaceEvenly` variant of `JustifyContent`.
JustifyContent.SpaceEvenly = {}

---@param variant JustifyContent.SpaceEvenly
---@param payload NoFields
---@return JustifyContent
function JustifyContent.SpaceEvenly.new(variant,payload) end



---@class JustifyContent.SpaceAround : EnumVariantRef
--- The `SpaceAround` variant of `JustifyContent`.
JustifyContent.SpaceAround = {}

---@param variant JustifyContent.SpaceAround
---@param payload NoFields
---@return JustifyContent
function JustifyContent.SpaceAround.new(variant,payload) end



---@class JustifyItems : ReflectReference
---  Used to control how each individual item is aligned by default within the space they're given.
---  - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
---  - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
---@field  Default JustifyItems.Default
---@field  Start JustifyItems.Start
---@field  StartSafe JustifyItems.StartSafe
---@field  End JustifyItems.End
---@field  EndSafe JustifyItems.EndSafe
---@field  Center JustifyItems.Center
---@field  CenterSafe JustifyItems.CenterSafe
---@field  Baseline JustifyItems.Baseline
---@field  Stretch JustifyItems.Stretch
local JustifyItems = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JustifyItems
function JustifyItems.new(payload) end



---@class JustifyItems.Default : EnumVariantRef
--- The `Default` variant of `JustifyItems`.
JustifyItems.Default = {}

---@param variant JustifyItems.Default
---@param payload NoFields
---@return JustifyItems
function JustifyItems.Default.new(variant,payload) end



---@class JustifyItems.Start : EnumVariantRef
--- The `Start` variant of `JustifyItems`.
JustifyItems.Start = {}

---@param variant JustifyItems.Start
---@param payload NoFields
---@return JustifyItems
function JustifyItems.Start.new(variant,payload) end



---@class JustifyItems.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `JustifyItems`.
JustifyItems.StartSafe = {}

---@param variant JustifyItems.StartSafe
---@param payload NoFields
---@return JustifyItems
function JustifyItems.StartSafe.new(variant,payload) end



---@class JustifyItems.End : EnumVariantRef
--- The `End` variant of `JustifyItems`.
JustifyItems.End = {}

---@param variant JustifyItems.End
---@param payload NoFields
---@return JustifyItems
function JustifyItems.End.new(variant,payload) end



---@class JustifyItems.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `JustifyItems`.
JustifyItems.EndSafe = {}

---@param variant JustifyItems.EndSafe
---@param payload NoFields
---@return JustifyItems
function JustifyItems.EndSafe.new(variant,payload) end



---@class JustifyItems.Center : EnumVariantRef
--- The `Center` variant of `JustifyItems`.
JustifyItems.Center = {}

---@param variant JustifyItems.Center
---@param payload NoFields
---@return JustifyItems
function JustifyItems.Center.new(variant,payload) end



---@class JustifyItems.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `JustifyItems`.
JustifyItems.CenterSafe = {}

---@param variant JustifyItems.CenterSafe
---@param payload NoFields
---@return JustifyItems
function JustifyItems.CenterSafe.new(variant,payload) end



---@class JustifyItems.Baseline : EnumVariantRef
--- The `Baseline` variant of `JustifyItems`.
JustifyItems.Baseline = {}

---@param variant JustifyItems.Baseline
---@param payload NoFields
---@return JustifyItems
function JustifyItems.Baseline.new(variant,payload) end



---@class JustifyItems.Stretch : EnumVariantRef
--- The `Stretch` variant of `JustifyItems`.
JustifyItems.Stretch = {}

---@param variant JustifyItems.Stretch
---@param payload NoFields
---@return JustifyItems
function JustifyItems.Stretch.new(variant,payload) end



---@class JustifySelf : ReflectReference
---  Used to control how the specified item is aligned within the space it's given.
---  - For children of flex nodes, this property has no effect. See `justify_content` for main axis alignment of flex items.
---  - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
--- 
---  # Safe alignments
--- 
---  Safe alignments will fall back to [`Self::Start`] if aligning the items would overflow the container, so
---  the start of the content stays visible.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
---@field  Auto JustifySelf.Auto
---@field  Start JustifySelf.Start
---@field  StartSafe JustifySelf.StartSafe
---@field  End JustifySelf.End
---@field  EndSafe JustifySelf.EndSafe
---@field  Center JustifySelf.Center
---@field  CenterSafe JustifySelf.CenterSafe
---@field  Baseline JustifySelf.Baseline
---@field  Stretch JustifySelf.Stretch
local JustifySelf = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return JustifySelf
function JustifySelf.new(payload) end



---@class JustifySelf.Auto : EnumVariantRef
--- The `Auto` variant of `JustifySelf`.
JustifySelf.Auto = {}

---@param variant JustifySelf.Auto
---@param payload NoFields
---@return JustifySelf
function JustifySelf.Auto.new(variant,payload) end



---@class JustifySelf.Start : EnumVariantRef
--- The `Start` variant of `JustifySelf`.
JustifySelf.Start = {}

---@param variant JustifySelf.Start
---@param payload NoFields
---@return JustifySelf
function JustifySelf.Start.new(variant,payload) end



---@class JustifySelf.StartSafe : EnumVariantRef
--- The `StartSafe` variant of `JustifySelf`.
JustifySelf.StartSafe = {}

---@param variant JustifySelf.StartSafe
---@param payload NoFields
---@return JustifySelf
function JustifySelf.StartSafe.new(variant,payload) end



---@class JustifySelf.End : EnumVariantRef
--- The `End` variant of `JustifySelf`.
JustifySelf.End = {}

---@param variant JustifySelf.End
---@param payload NoFields
---@return JustifySelf
function JustifySelf.End.new(variant,payload) end



---@class JustifySelf.EndSafe : EnumVariantRef
--- The `EndSafe` variant of `JustifySelf`.
JustifySelf.EndSafe = {}

---@param variant JustifySelf.EndSafe
---@param payload NoFields
---@return JustifySelf
function JustifySelf.EndSafe.new(variant,payload) end



---@class JustifySelf.Center : EnumVariantRef
--- The `Center` variant of `JustifySelf`.
JustifySelf.Center = {}

---@param variant JustifySelf.Center
---@param payload NoFields
---@return JustifySelf
function JustifySelf.Center.new(variant,payload) end



---@class JustifySelf.CenterSafe : EnumVariantRef
--- The `CenterSafe` variant of `JustifySelf`.
JustifySelf.CenterSafe = {}

---@param variant JustifySelf.CenterSafe
---@param payload NoFields
---@return JustifySelf
function JustifySelf.CenterSafe.new(variant,payload) end



---@class JustifySelf.Baseline : EnumVariantRef
--- The `Baseline` variant of `JustifySelf`.
JustifySelf.Baseline = {}

---@param variant JustifySelf.Baseline
---@param payload NoFields
---@return JustifySelf
function JustifySelf.Baseline.new(variant,payload) end



---@class JustifySelf.Stretch : EnumVariantRef
--- The `Stretch` variant of `JustifySelf`.
JustifySelf.Stretch = {}

---@param variant JustifySelf.Stretch
---@param payload NoFields
---@return JustifySelf
function JustifySelf.Stretch.new(variant,payload) end



---@class MaxTrackSizingFunction : ReflectReference
---@field  Px MaxTrackSizingFunction.Px
---@field  Percent MaxTrackSizingFunction.Percent
---@field  Em MaxTrackSizingFunction.Em
---@field  Rem MaxTrackSizingFunction.Rem
---@field  MinContent MaxTrackSizingFunction.MinContent
---@field  MaxContent MaxTrackSizingFunction.MaxContent
---@field  FitContentPx MaxTrackSizingFunction.FitContentPx
---@field  FitContentPercent MaxTrackSizingFunction.FitContentPercent
---@field  Auto MaxTrackSizingFunction.Auto
---@field  Fraction MaxTrackSizingFunction.Fraction
---@field  VMin MaxTrackSizingFunction.VMin
---@field  VMax MaxTrackSizingFunction.VMax
---@field  Vh MaxTrackSizingFunction.Vh
---@field  Vw MaxTrackSizingFunction.Vw
local MaxTrackSizingFunction = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.new(payload) end



---@class MaxTrackSizingFunction.Px : EnumVariantRef
--- The `Px` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Px = {}

---@param variant MaxTrackSizingFunction.Px
---@param payload MaxTrackSizingFunction.Px.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Px.new(variant,payload) end



---@class MaxTrackSizingFunction.Percent : EnumVariantRef
--- The `Percent` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Percent = {}

---@param variant MaxTrackSizingFunction.Percent
---@param payload MaxTrackSizingFunction.Percent.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Percent.new(variant,payload) end



---@class MaxTrackSizingFunction.Em : EnumVariantRef
--- The `Em` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Em = {}

---@param variant MaxTrackSizingFunction.Em
---@param payload MaxTrackSizingFunction.Em.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Em.new(variant,payload) end



---@class MaxTrackSizingFunction.Rem : EnumVariantRef
--- The `Rem` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Rem = {}

---@param variant MaxTrackSizingFunction.Rem
---@param payload MaxTrackSizingFunction.Rem.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Rem.new(variant,payload) end



---@class MaxTrackSizingFunction.MinContent : EnumVariantRef
--- The `MinContent` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.MinContent = {}

---@param variant MaxTrackSizingFunction.MinContent
---@param payload NoFields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.MinContent.new(variant,payload) end



---@class MaxTrackSizingFunction.MaxContent : EnumVariantRef
--- The `MaxContent` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.MaxContent = {}

---@param variant MaxTrackSizingFunction.MaxContent
---@param payload NoFields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.MaxContent.new(variant,payload) end



---@class MaxTrackSizingFunction.FitContentPx : EnumVariantRef
--- The `FitContentPx` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.FitContentPx = {}

---@param variant MaxTrackSizingFunction.FitContentPx
---@param payload MaxTrackSizingFunction.FitContentPx.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.FitContentPx.new(variant,payload) end



---@class MaxTrackSizingFunction.FitContentPercent : EnumVariantRef
--- The `FitContentPercent` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.FitContentPercent = {}

---@param variant MaxTrackSizingFunction.FitContentPercent
---@param payload MaxTrackSizingFunction.FitContentPercent.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.FitContentPercent.new(variant,payload) end



---@class MaxTrackSizingFunction.Auto : EnumVariantRef
--- The `Auto` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Auto = {}

---@param variant MaxTrackSizingFunction.Auto
---@param payload NoFields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Auto.new(variant,payload) end



---@class MaxTrackSizingFunction.Fraction : EnumVariantRef
--- The `Fraction` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Fraction = {}

---@param variant MaxTrackSizingFunction.Fraction
---@param payload MaxTrackSizingFunction.Fraction.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Fraction.new(variant,payload) end



---@class MaxTrackSizingFunction.VMin : EnumVariantRef
--- The `VMin` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.VMin = {}

---@param variant MaxTrackSizingFunction.VMin
---@param payload MaxTrackSizingFunction.VMin.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.VMin.new(variant,payload) end



---@class MaxTrackSizingFunction.VMax : EnumVariantRef
--- The `VMax` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.VMax = {}

---@param variant MaxTrackSizingFunction.VMax
---@param payload MaxTrackSizingFunction.VMax.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.VMax.new(variant,payload) end



---@class MaxTrackSizingFunction.Vh : EnumVariantRef
--- The `Vh` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Vh = {}

---@param variant MaxTrackSizingFunction.Vh
---@param payload MaxTrackSizingFunction.Vh.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Vh.new(variant,payload) end



---@class MaxTrackSizingFunction.Vw : EnumVariantRef
--- The `Vw` variant of `MaxTrackSizingFunction`.
MaxTrackSizingFunction.Vw = {}

---@param variant MaxTrackSizingFunction.Vw
---@param payload MaxTrackSizingFunction.Vw.Fields
---@return MaxTrackSizingFunction
function MaxTrackSizingFunction.Vw.new(variant,payload) end



---@class MinTrackSizingFunction : ReflectReference
---@field  Px MinTrackSizingFunction.Px
---@field  Percent MinTrackSizingFunction.Percent
---@field  Em MinTrackSizingFunction.Em
---@field  Rem MinTrackSizingFunction.Rem
---@field  MinContent MinTrackSizingFunction.MinContent
---@field  MaxContent MinTrackSizingFunction.MaxContent
---@field  Auto MinTrackSizingFunction.Auto
---@field  VMin MinTrackSizingFunction.VMin
---@field  VMax MinTrackSizingFunction.VMax
---@field  Vh MinTrackSizingFunction.Vh
---@field  Vw MinTrackSizingFunction.Vw
local MinTrackSizingFunction = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MinTrackSizingFunction
function MinTrackSizingFunction.new(payload) end



---@class MinTrackSizingFunction.Px : EnumVariantRef
--- The `Px` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Px = {}

---@param variant MinTrackSizingFunction.Px
---@param payload MinTrackSizingFunction.Px.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Px.new(variant,payload) end



---@class MinTrackSizingFunction.Percent : EnumVariantRef
--- The `Percent` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Percent = {}

---@param variant MinTrackSizingFunction.Percent
---@param payload MinTrackSizingFunction.Percent.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Percent.new(variant,payload) end



---@class MinTrackSizingFunction.Em : EnumVariantRef
--- The `Em` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Em = {}

---@param variant MinTrackSizingFunction.Em
---@param payload MinTrackSizingFunction.Em.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Em.new(variant,payload) end



---@class MinTrackSizingFunction.Rem : EnumVariantRef
--- The `Rem` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Rem = {}

---@param variant MinTrackSizingFunction.Rem
---@param payload MinTrackSizingFunction.Rem.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Rem.new(variant,payload) end



---@class MinTrackSizingFunction.MinContent : EnumVariantRef
--- The `MinContent` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.MinContent = {}

---@param variant MinTrackSizingFunction.MinContent
---@param payload NoFields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.MinContent.new(variant,payload) end



---@class MinTrackSizingFunction.MaxContent : EnumVariantRef
--- The `MaxContent` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.MaxContent = {}

---@param variant MinTrackSizingFunction.MaxContent
---@param payload NoFields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.MaxContent.new(variant,payload) end



---@class MinTrackSizingFunction.Auto : EnumVariantRef
--- The `Auto` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Auto = {}

---@param variant MinTrackSizingFunction.Auto
---@param payload NoFields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Auto.new(variant,payload) end



---@class MinTrackSizingFunction.VMin : EnumVariantRef
--- The `VMin` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.VMin = {}

---@param variant MinTrackSizingFunction.VMin
---@param payload MinTrackSizingFunction.VMin.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.VMin.new(variant,payload) end



---@class MinTrackSizingFunction.VMax : EnumVariantRef
--- The `VMax` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.VMax = {}

---@param variant MinTrackSizingFunction.VMax
---@param payload MinTrackSizingFunction.VMax.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.VMax.new(variant,payload) end



---@class MinTrackSizingFunction.Vh : EnumVariantRef
--- The `Vh` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Vh = {}

---@param variant MinTrackSizingFunction.Vh
---@param payload MinTrackSizingFunction.Vh.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Vh.new(variant,payload) end



---@class MinTrackSizingFunction.Vw : EnumVariantRef
--- The `Vw` variant of `MinTrackSizingFunction`.
MinTrackSizingFunction.Vw = {}

---@param variant MinTrackSizingFunction.Vw
---@param payload MinTrackSizingFunction.Vw.Fields
---@return MinTrackSizingFunction
function MinTrackSizingFunction.Vw.new(variant,payload) end



---@class Node : ReflectReference
---  The base component for UI entities. It describes UI layout and style properties.
--- 
---  When defining new types of UI entities, require [`Node`] to make them behave like UI nodes.
--- 
---  Nodes can be laid out using either Flexbox or CSS Grid Layout.
--- 
---  See below for general learning resources and for documentation on the individual style properties.
--- 
---  ### Flexbox
--- 
---  - [MDN: Basic Concepts of Flexbox](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox)
---  - [A Complete Guide To Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different Flexbox properties and how they work.
---  - [Flexbox Froggy](https://flexboxfroggy.com/). An interactive tutorial/game that teaches the essential parts of Flexbox in a fun engaging way.
--- 
---  ### CSS Grid
--- 
---  - [MDN: Basic Concepts of Grid Layout](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout/Basic_Concepts_of_Grid_Layout)
---  - [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different CSS Grid properties and how they work.
---  - [CSS Grid Garden](https://cssgridgarden.com/). An interactive tutorial/game that teaches the essential parts of CSS Grid in a fun engaging way.
--- 
---  # See also
--- 
---  - [`RelativeCursorPosition`](crate::RelativeCursorPosition) to obtain the cursor position relative to this node
---@field  display? Display
---@field  box_sizing? BoxSizing
---@field  position_type? PositionType
---@field  overflow? Overflow
---@field  scrollbar_width? number
---@field  overflow_clip_margin? OverflowClipMargin
---@field  left? Val
---@field  right? Val
---@field  top? Val
---@field  bottom? Val
---@field  width? Val
---@field  height? Val
---@field  min_width? Val
---@field  min_height? Val
---@field  max_width? Val
---@field  max_height? Val
---@field  aspect_ratio? number | nil
---@field  align_items? AlignItems
---@field  justify_items? JustifyItems
---@field  align_self? AlignSelf
---@field  justify_self? JustifySelf
---@field  align_content? AlignContent
---@field  justify_content? JustifyContent
---@field  direction? InlineDirection
---@field  margin? UiRect
---@field  padding? UiRect
---@field  border? UiRect
---@field  border_radius? BorderRadius
---@field  flex_direction? FlexDirection
---@field  flex_wrap? FlexWrap
---@field  flex_grow? number
---@field  flex_shrink? number
---@field  flex_basis? Val
---@field  row_gap? Val
---@field  column_gap? Val
---@field  grid_auto_flow? GridAutoFlow
---@field  grid_template_rows? RepeatedGridTrack[]
---@field  grid_template_columns? RepeatedGridTrack[]
---@field  grid_auto_rows? GridTrack[]
---@field  grid_auto_columns? GridTrack[]
---@field  grid_row? GridPlacement
---@field  grid_column? GridPlacement
local Node = {}

---@param payload Node.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Node
function Node.new(payload) end



---@class Outline : ReflectReference
---  The [`Outline`] component adds an outline outside the edge of a UI node.
---  Outlines do not take up space in the layout.
--- 
---  To add an [`Outline`] to a UI node you can spawn a `(Node, Outline)` tuple bundle:
---  ```
---  # use bevy_ecs::prelude::*;
---  # use bevy_ui::prelude::*;
---  # use bevy_color::palettes::basic::{RED, BLUE};
---  fn setup_ui(mut commands: Commands) {
---      commands.spawn((
---          Node {
---              width: Val::Px(100.),
---              height: Val::Px(100.),
---              ..Default::default()
---          },
---          BackgroundColor(BLUE.into()),
---          Outline::new(Val::Px(10.), Val::ZERO, RED.into())
---      ));
---  }
---  ```
--- 
---  [`Outline`] components can also be added later to existing UI nodes:
---  ```
---  # use bevy_ecs::prelude::*;
---  # use bevy_ui::prelude::*;
---  # use bevy_picking::hover::Hovered;
---  # use bevy_color::Color;
---  fn outline_hovered_button_system(
---      mut commands: Commands,
---      mut node_query: Query<(Entity, &Hovered, Option<&mut Outline>), Changed<Hovered>>,
---  ) {
---      for (entity, hovered, mut maybe_outline) in node_query.iter_mut() {
---          let outline_color =
---              if hovered.get() {
---                  Color::WHITE
---              } else {
---                  Color::NONE
---              };
---          if let Some(mut outline) = maybe_outline {
---              outline.color = outline_color;
---          } else {
---              commands.entity(entity).insert(Outline::new(Val::Px(10.), Val::ZERO, outline_color));
---          }
---      }
---  }
---  ```
---  Inserting and removing an [`Outline`] component repeatedly will result in table moves, so it is generally preferable to
---  set `Outline::color` to [`Color::NONE`] to hide an outline.
---@field  width? Val
---@field  offset? Val
---@field  color? Color
local Outline = {}

---@param payload Outline.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Outline
function Outline.new(payload) end



---@class OverflowAxis : ReflectReference
---  Whether to show or hide overflowing items
---@field  Visible OverflowAxis.Visible
---@field  Clip OverflowAxis.Clip
---@field  Hidden OverflowAxis.Hidden
---@field  Scroll OverflowAxis.Scroll
local OverflowAxis = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return OverflowAxis
function OverflowAxis.new(payload) end



---@class OverflowAxis.Visible : EnumVariantRef
--- The `Visible` variant of `OverflowAxis`.
OverflowAxis.Visible = {}

---@param variant OverflowAxis.Visible
---@param payload NoFields
---@return OverflowAxis
function OverflowAxis.Visible.new(variant,payload) end



---@class OverflowAxis.Clip : EnumVariantRef
--- The `Clip` variant of `OverflowAxis`.
OverflowAxis.Clip = {}

---@param variant OverflowAxis.Clip
---@param payload NoFields
---@return OverflowAxis
function OverflowAxis.Clip.new(variant,payload) end



---@class OverflowAxis.Hidden : EnumVariantRef
--- The `Hidden` variant of `OverflowAxis`.
OverflowAxis.Hidden = {}

---@param variant OverflowAxis.Hidden
---@param payload NoFields
---@return OverflowAxis
function OverflowAxis.Hidden.new(variant,payload) end



---@class OverflowAxis.Scroll : EnumVariantRef
--- The `Scroll` variant of `OverflowAxis`.
OverflowAxis.Scroll = {}

---@param variant OverflowAxis.Scroll
---@param payload NoFields
---@return OverflowAxis
function OverflowAxis.Scroll.new(variant,payload) end



---@class OverflowClipMargin : ReflectReference
---  The bounds of the visible area when a UI node is clipped.
---@field  visual_box? VisualBox
---@field  margin? number
local OverflowClipMargin = {}

---@param payload OverflowClipMargin.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return OverflowClipMargin
function OverflowClipMargin.new(payload) end



---@class PositionType : ReflectReference
---  The strategy used to position this node
---@field  Relative PositionType.Relative
---@field  Absolute PositionType.Absolute
local PositionType = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return PositionType
function PositionType.new(payload) end



---@class PositionType.Relative : EnumVariantRef
--- The `Relative` variant of `PositionType`.
PositionType.Relative = {}

---@param variant PositionType.Relative
---@param payload NoFields
---@return PositionType
function PositionType.Relative.new(variant,payload) end



---@class PositionType.Absolute : EnumVariantRef
--- The `Absolute` variant of `PositionType`.
PositionType.Absolute = {}

---@param variant PositionType.Absolute
---@param payload NoFields
---@return PositionType
function PositionType.Absolute.new(variant,payload) end



---@class RepeatedGridTrack : ReflectReference
---  Represents a *possibly* repeated [`GridTrack`].
--- 
---  The repetition parameter can either be:
---    - The integer `1`, in which case the track is non-repeated.
---    - a `u16` count to repeat the track N times.
---    - A `GridTrackRepetition::AutoFit` or `GridTrackRepetition::AutoFill`.
--- 
---  Note: that in the common case you want a non-repeating track (repetition count 1), you may use the constructor methods on [`GridTrack`]
---  to create a `RepeatedGridTrack`. i.e. `GridTrack::px(10.0)` is equivalent to `RepeatedGridTrack::px(1, 10.0)`.
--- 
---  You may only use one auto-repetition per track list. And if your track list contains an auto repetition
---  then all tracks (in and outside of the repetition) must be fixed size (px or percent). Integer repetitions are just shorthand for writing out
---  N tracks longhand and are not subject to the same limitations.
---@field  repetition? GridTrackRepetition
---@field  tracks? GridTrack[]
local RepeatedGridTrack = {}

---@param payload RepeatedGridTrack.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return RepeatedGridTrack
function RepeatedGridTrack.new(payload) end



---@class ResolvedBorderRadius : ReflectReference
---  Represents the resolved border radii values for a UI node.
--- 
---  The values are in physical pixels.
---@field  top_left? Vec2
---@field  top_right? Vec2
---@field  bottom_right? Vec2
---@field  bottom_left? Vec2
local ResolvedBorderRadius = {}

---@param payload ResolvedBorderRadius.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ResolvedBorderRadius
function ResolvedBorderRadius.new(payload) end



---@class VisualBox : ReflectReference
---  Used to determine which region of a UI node is used for visual bounds.
---@field  ContentBox VisualBox.ContentBox
---@field  PaddingBox VisualBox.PaddingBox
---@field  BorderBox VisualBox.BorderBox
local VisualBox = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return VisualBox
function VisualBox.new(payload) end



---@class VisualBox.ContentBox : EnumVariantRef
--- The `ContentBox` variant of `VisualBox`.
VisualBox.ContentBox = {}

---@param variant VisualBox.ContentBox
---@param payload NoFields
---@return VisualBox
function VisualBox.ContentBox.new(variant,payload) end



---@class VisualBox.PaddingBox : EnumVariantRef
--- The `PaddingBox` variant of `VisualBox`.
VisualBox.PaddingBox = {}

---@param variant VisualBox.PaddingBox
---@param payload NoFields
---@return VisualBox
function VisualBox.PaddingBox.new(variant,payload) end



---@class VisualBox.BorderBox : EnumVariantRef
--- The `BorderBox` variant of `VisualBox`.
VisualBox.BorderBox = {}

---@param variant VisualBox.BorderBox
---@param payload NoFields
---@return VisualBox
function VisualBox.BorderBox.new(variant,payload) end



---@class ZIndex : ReflectReference
---  Indicates that this [`Node`] entity's front-to-back ordering is not controlled solely
---  by its location in the UI hierarchy. A node with a higher z-index will appear on top
---  of sibling nodes with a lower z-index.
--- 
---  UI nodes that have the same z-index will appear according to the order in which they
---  appear in the UI hierarchy. In such a case, the last node to be added to its parent
---  will appear in front of its siblings.
--- 
---  Nodes without this component will be treated as if they had a value of
---  <code>[ZIndex][ZIndex]\(0\)</code>.
--- 
---  Use [`GlobalZIndex`] if you need to order separate UI hierarchies or nodes that are
---  not siblings in a given UI hierarchy.
---@field  [1] integer
local ZIndex = {}

---@param payload ZIndex.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ZIndex
function ZIndex.new(payload) end



---@class NodeImageMode : ReflectReference
---  Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image
---@field  Auto NodeImageMode.Auto
---@field  Stretch NodeImageMode.Stretch
---@field  Sliced NodeImageMode.Sliced
---@field  Tiled NodeImageMode.Tiled
local NodeImageMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NodeImageMode
function NodeImageMode.new(payload) end



---@class NodeImageMode.Auto : EnumVariantRef
--- The `Auto` variant of `NodeImageMode`.
NodeImageMode.Auto = {}

---@param variant NodeImageMode.Auto
---@param payload NoFields
---@return NodeImageMode
function NodeImageMode.Auto.new(variant,payload) end



---@class NodeImageMode.Stretch : EnumVariantRef
--- The `Stretch` variant of `NodeImageMode`.
NodeImageMode.Stretch = {}

---@param variant NodeImageMode.Stretch
---@param payload NoFields
---@return NodeImageMode
function NodeImageMode.Stretch.new(variant,payload) end



---@class NodeImageMode.Sliced : EnumVariantRef
--- The `Sliced` variant of `NodeImageMode`.
NodeImageMode.Sliced = {}

---@param variant NodeImageMode.Sliced
---@param payload NodeImageMode.Sliced.Fields
---@return NodeImageMode
function NodeImageMode.Sliced.new(variant,payload) end



---@class NodeImageMode.Tiled : EnumVariantRef
--- The `Tiled` variant of `NodeImageMode`.
NodeImageMode.Tiled = {}

---@param variant NodeImageMode.Tiled
---@param payload NodeImageMode.Tiled.Fields
---@return NodeImageMode
function NodeImageMode.Tiled.new(variant,payload) end



---@class VideoMode : ReflectReference
---  Represents a video mode that a monitor supports
---@field  physical_size? UVec2
---@field  bit_depth? integer
---@field  refresh_rate_millihertz? integer
local VideoMode = {}

---@param payload VideoMode.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return VideoMode
function VideoMode.new(payload) end



---@class CompositeAlphaMode : ReflectReference
---  Specifies how the alpha channel of the textures should be handled during compositing, for a [`Window`].
---@field  Auto CompositeAlphaMode.Auto
---@field  Opaque CompositeAlphaMode.Opaque
---@field  PreMultiplied CompositeAlphaMode.PreMultiplied
---@field  PostMultiplied CompositeAlphaMode.PostMultiplied
---@field  Inherit CompositeAlphaMode.Inherit
local CompositeAlphaMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return CompositeAlphaMode
function CompositeAlphaMode.new(payload) end



---@class CompositeAlphaMode.Auto : EnumVariantRef
--- The `Auto` variant of `CompositeAlphaMode`.
CompositeAlphaMode.Auto = {}

---@param variant CompositeAlphaMode.Auto
---@param payload NoFields
---@return CompositeAlphaMode
function CompositeAlphaMode.Auto.new(variant,payload) end



---@class CompositeAlphaMode.Opaque : EnumVariantRef
--- The `Opaque` variant of `CompositeAlphaMode`.
CompositeAlphaMode.Opaque = {}

---@param variant CompositeAlphaMode.Opaque
---@param payload NoFields
---@return CompositeAlphaMode
function CompositeAlphaMode.Opaque.new(variant,payload) end



---@class CompositeAlphaMode.PreMultiplied : EnumVariantRef
--- The `PreMultiplied` variant of `CompositeAlphaMode`.
CompositeAlphaMode.PreMultiplied = {}

---@param variant CompositeAlphaMode.PreMultiplied
---@param payload NoFields
---@return CompositeAlphaMode
function CompositeAlphaMode.PreMultiplied.new(variant,payload) end



---@class CompositeAlphaMode.PostMultiplied : EnumVariantRef
--- The `PostMultiplied` variant of `CompositeAlphaMode`.
CompositeAlphaMode.PostMultiplied = {}

---@param variant CompositeAlphaMode.PostMultiplied
---@param payload NoFields
---@return CompositeAlphaMode
function CompositeAlphaMode.PostMultiplied.new(variant,payload) end



---@class CompositeAlphaMode.Inherit : EnumVariantRef
--- The `Inherit` variant of `CompositeAlphaMode`.
CompositeAlphaMode.Inherit = {}

---@param variant CompositeAlphaMode.Inherit
---@param payload NoFields
---@return CompositeAlphaMode
function CompositeAlphaMode.Inherit.new(variant,payload) end



---@class EnabledButtons : ReflectReference
---  Specifies which [`Window`] control buttons should be enabled.
--- 
---  ## Platform-specific
--- 
---  **`iOS`**, **`Android`**, and the **`Web`** do not have window control buttons.
--- 
---  On some **`Linux`** environments these values have no effect.
---@field  minimize? boolean
---@field  maximize? boolean
---@field  close? boolean
local EnabledButtons = {}

---@param payload EnabledButtons.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return EnabledButtons
function EnabledButtons.new(payload) end



---@class InternalWindowState : ReflectReference
---  Stores internal [`Window`] state that isn't directly accessible.
---@field  minimize_request? boolean | nil
---@field  maximize_request? boolean | nil
---@field  drag_move_request? boolean
---@field  drag_resize_request? CompassOctant | nil
---@field  physical_cursor_position? DVec2 | nil
local InternalWindowState = {}

---@param payload InternalWindowState.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return InternalWindowState
function InternalWindowState.new(payload) end



---@class MonitorSelection : ReflectReference
---  References a screen monitor.
--- 
---  Used when centering a [`Window`] on a monitor.
---@field  Current MonitorSelection.Current
---@field  Primary MonitorSelection.Primary
---@field  Index MonitorSelection.Index
---@field  Entity MonitorSelection.Entity
local MonitorSelection = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return MonitorSelection
function MonitorSelection.new(payload) end



---@class MonitorSelection.Current : EnumVariantRef
--- The `Current` variant of `MonitorSelection`.
MonitorSelection.Current = {}

---@param variant MonitorSelection.Current
---@param payload NoFields
---@return MonitorSelection
function MonitorSelection.Current.new(variant,payload) end



---@class MonitorSelection.Primary : EnumVariantRef
--- The `Primary` variant of `MonitorSelection`.
MonitorSelection.Primary = {}

---@param variant MonitorSelection.Primary
---@param payload NoFields
---@return MonitorSelection
function MonitorSelection.Primary.new(variant,payload) end



---@class MonitorSelection.Index : EnumVariantRef
--- The `Index` variant of `MonitorSelection`.
MonitorSelection.Index = {}

---@param variant MonitorSelection.Index
---@param payload MonitorSelection.Index.Fields
---@return MonitorSelection
function MonitorSelection.Index.new(variant,payload) end



---@class MonitorSelection.Entity : EnumVariantRef
--- The `Entity` variant of `MonitorSelection`.
MonitorSelection.Entity = {}

---@param variant MonitorSelection.Entity
---@param payload MonitorSelection.Entity.Fields
---@return MonitorSelection
function MonitorSelection.Entity.new(variant,payload) end



---@class PresentMode : ReflectReference
---  Presentation mode for a [`Window`].
--- 
---  The presentation mode specifies when a frame is presented to the window. The [`Fifo`]
---  option corresponds to a traditional `VSync`, where the framerate is capped by the
---  display refresh rate. Both [`Immediate`] and [`Mailbox`] are low-latency and are not
---  capped by the refresh rate, but may not be available on all platforms. Tearing
---  may be observed with [`Immediate`] mode, but will not be observed with [`Mailbox`] or
---  [`Fifo`].
--- 
---  [`AutoVsync`] or [`AutoNoVsync`] will gracefully fallback to [`Fifo`] when unavailable.
--- 
---  [`Immediate`] or [`Mailbox`] will panic if not supported by the platform.
--- 
---  [`Fifo`]: PresentMode::Fifo
---  [`FifoRelaxed`]: PresentMode::FifoRelaxed
---  [`Immediate`]: PresentMode::Immediate
---  [`Mailbox`]: PresentMode::Mailbox
---  [`AutoVsync`]: PresentMode::AutoVsync
---  [`AutoNoVsync`]: PresentMode::AutoNoVsync
---@field  AutoVsync PresentMode.AutoVsync
---@field  AutoNoVsync PresentMode.AutoNoVsync
---@field  Fifo PresentMode.Fifo
---@field  FifoRelaxed PresentMode.FifoRelaxed
---@field  Immediate PresentMode.Immediate
---@field  Mailbox PresentMode.Mailbox
local PresentMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return PresentMode
function PresentMode.new(payload) end



---@class PresentMode.AutoVsync : EnumVariantRef
--- The `AutoVsync` variant of `PresentMode`.
PresentMode.AutoVsync = {}

---@param variant PresentMode.AutoVsync
---@param payload NoFields
---@return PresentMode
function PresentMode.AutoVsync.new(variant,payload) end



---@class PresentMode.AutoNoVsync : EnumVariantRef
--- The `AutoNoVsync` variant of `PresentMode`.
PresentMode.AutoNoVsync = {}

---@param variant PresentMode.AutoNoVsync
---@param payload NoFields
---@return PresentMode
function PresentMode.AutoNoVsync.new(variant,payload) end



---@class PresentMode.Fifo : EnumVariantRef
--- The `Fifo` variant of `PresentMode`.
PresentMode.Fifo = {}

---@param variant PresentMode.Fifo
---@param payload NoFields
---@return PresentMode
function PresentMode.Fifo.new(variant,payload) end



---@class PresentMode.FifoRelaxed : EnumVariantRef
--- The `FifoRelaxed` variant of `PresentMode`.
PresentMode.FifoRelaxed = {}

---@param variant PresentMode.FifoRelaxed
---@param payload NoFields
---@return PresentMode
function PresentMode.FifoRelaxed.new(variant,payload) end



---@class PresentMode.Immediate : EnumVariantRef
--- The `Immediate` variant of `PresentMode`.
PresentMode.Immediate = {}

---@param variant PresentMode.Immediate
---@param payload NoFields
---@return PresentMode
function PresentMode.Immediate.new(variant,payload) end



---@class PresentMode.Mailbox : EnumVariantRef
--- The `Mailbox` variant of `PresentMode`.
PresentMode.Mailbox = {}

---@param variant PresentMode.Mailbox
---@param payload NoFields
---@return PresentMode
function PresentMode.Mailbox.new(variant,payload) end



---@class ScreenEdge : ReflectReference
---  The edges of a screen. Corresponds to [`winit::platform::ios::ScreenEdge`].
--- 
---  # Platform-specific
--- 
---  - Only used on iOS.
--- 
---  [`winit::platform::ios::ScreenEdge`]: https://docs.rs/winit/latest/x86_64-apple-darwin/winit/platform/ios/struct.ScreenEdge.html
---@field  None ScreenEdge.None
---@field  Top ScreenEdge.Top
---@field  Left ScreenEdge.Left
---@field  Bottom ScreenEdge.Bottom
---@field  Right ScreenEdge.Right
---@field  All ScreenEdge.All
local ScreenEdge = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return ScreenEdge
function ScreenEdge.new(payload) end



---@class ScreenEdge.None : EnumVariantRef
--- The `None` variant of `ScreenEdge`.
ScreenEdge.None = {}

---@param variant ScreenEdge.None
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.None.new(variant,payload) end



---@class ScreenEdge.Top : EnumVariantRef
--- The `Top` variant of `ScreenEdge`.
ScreenEdge.Top = {}

---@param variant ScreenEdge.Top
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.Top.new(variant,payload) end



---@class ScreenEdge.Left : EnumVariantRef
--- The `Left` variant of `ScreenEdge`.
ScreenEdge.Left = {}

---@param variant ScreenEdge.Left
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.Left.new(variant,payload) end



---@class ScreenEdge.Bottom : EnumVariantRef
--- The `Bottom` variant of `ScreenEdge`.
ScreenEdge.Bottom = {}

---@param variant ScreenEdge.Bottom
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.Bottom.new(variant,payload) end



---@class ScreenEdge.Right : EnumVariantRef
--- The `Right` variant of `ScreenEdge`.
ScreenEdge.Right = {}

---@param variant ScreenEdge.Right
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.Right.new(variant,payload) end



---@class ScreenEdge.All : EnumVariantRef
--- The `All` variant of `ScreenEdge`.
ScreenEdge.All = {}

---@param variant ScreenEdge.All
---@param payload NoFields
---@return ScreenEdge
function ScreenEdge.All.new(variant,payload) end



---@class VideoModeSelection : ReflectReference
---  References an exclusive fullscreen video mode.
--- 
---  Used when setting [`WindowMode::Fullscreen`] on a window.
---@field  Current VideoModeSelection.Current
---@field  Specific VideoModeSelection.Specific
local VideoModeSelection = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return VideoModeSelection
function VideoModeSelection.new(payload) end



---@class VideoModeSelection.Current : EnumVariantRef
--- The `Current` variant of `VideoModeSelection`.
VideoModeSelection.Current = {}

---@param variant VideoModeSelection.Current
---@param payload NoFields
---@return VideoModeSelection
function VideoModeSelection.Current.new(variant,payload) end



---@class VideoModeSelection.Specific : EnumVariantRef
--- The `Specific` variant of `VideoModeSelection`.
VideoModeSelection.Specific = {}

---@param variant VideoModeSelection.Specific
---@param payload VideoModeSelection.Specific.Fields
---@return VideoModeSelection
function VideoModeSelection.Specific.new(variant,payload) end



---@class WindowLevel : ReflectReference
---  Specifies where a [`Window`] should appear relative to other overlapping windows (on top or under) .
--- 
---  Levels are groups of windows with respect to their z-position.
--- 
---  The relative ordering between windows in different window levels is fixed.
---  The z-order of windows within the same window level may change dynamically on user interaction.
--- 
---  ## Platform-specific
--- 
---  - **iOS / Android / Web / Wayland:** Unsupported.
---@field  AlwaysOnBottom WindowLevel.AlwaysOnBottom
---@field  Normal WindowLevel.Normal
---@field  AlwaysOnTop WindowLevel.AlwaysOnTop
local WindowLevel = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowLevel
function WindowLevel.new(payload) end



---@class WindowLevel.AlwaysOnBottom : EnumVariantRef
--- The `AlwaysOnBottom` variant of `WindowLevel`.
WindowLevel.AlwaysOnBottom = {}

---@param variant WindowLevel.AlwaysOnBottom
---@param payload NoFields
---@return WindowLevel
function WindowLevel.AlwaysOnBottom.new(variant,payload) end



---@class WindowLevel.Normal : EnumVariantRef
--- The `Normal` variant of `WindowLevel`.
WindowLevel.Normal = {}

---@param variant WindowLevel.Normal
---@param payload NoFields
---@return WindowLevel
function WindowLevel.Normal.new(variant,payload) end



---@class WindowLevel.AlwaysOnTop : EnumVariantRef
--- The `AlwaysOnTop` variant of `WindowLevel`.
WindowLevel.AlwaysOnTop = {}

---@param variant WindowLevel.AlwaysOnTop
---@param payload NoFields
---@return WindowLevel
function WindowLevel.AlwaysOnTop.new(variant,payload) end



---@class WindowMode : ReflectReference
---  Defines the way a [`Window`] is displayed.
---@field  Windowed WindowMode.Windowed
---@field  BorderlessFullscreen WindowMode.BorderlessFullscreen
---@field  Fullscreen WindowMode.Fullscreen
local WindowMode = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowMode
function WindowMode.new(payload) end



---@class WindowMode.Windowed : EnumVariantRef
--- The `Windowed` variant of `WindowMode`.
WindowMode.Windowed = {}

---@param variant WindowMode.Windowed
---@param payload NoFields
---@return WindowMode
function WindowMode.Windowed.new(variant,payload) end



---@class WindowMode.BorderlessFullscreen : EnumVariantRef
--- The `BorderlessFullscreen` variant of `WindowMode`.
WindowMode.BorderlessFullscreen = {}

---@param variant WindowMode.BorderlessFullscreen
---@param payload WindowMode.BorderlessFullscreen.Fields
---@return WindowMode
function WindowMode.BorderlessFullscreen.new(variant,payload) end



---@class WindowMode.Fullscreen : EnumVariantRef
--- The `Fullscreen` variant of `WindowMode`.
WindowMode.Fullscreen = {}

---@param variant WindowMode.Fullscreen
---@param payload WindowMode.Fullscreen.Fields
---@return WindowMode
function WindowMode.Fullscreen.new(variant,payload) end



---@class WindowPosition : ReflectReference
---  Defines where a [`Window`] should be placed on the screen.
---@field  Automatic WindowPosition.Automatic
---@field  Centered WindowPosition.Centered
---@field  At WindowPosition.At
local WindowPosition = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowPosition
function WindowPosition.new(payload) end



---@class WindowPosition.Automatic : EnumVariantRef
--- The `Automatic` variant of `WindowPosition`.
WindowPosition.Automatic = {}

---@param variant WindowPosition.Automatic
---@param payload NoFields
---@return WindowPosition
function WindowPosition.Automatic.new(variant,payload) end



---@class WindowPosition.Centered : EnumVariantRef
--- The `Centered` variant of `WindowPosition`.
WindowPosition.Centered = {}

---@param variant WindowPosition.Centered
---@param payload WindowPosition.Centered.Fields
---@return WindowPosition
function WindowPosition.Centered.new(variant,payload) end



---@class WindowPosition.At : EnumVariantRef
--- The `At` variant of `WindowPosition`.
WindowPosition.At = {}

---@param variant WindowPosition.At
---@param payload WindowPosition.At.Fields
---@return WindowPosition
function WindowPosition.At.new(variant,payload) end



---@class WindowRef : ReflectReference
---  Reference to a [`Window`], whether it be a direct link to a specific entity or
---  a more vague defaulting choice.
---@field  Primary WindowRef.Primary
---@field  Entity WindowRef.Entity
local WindowRef = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowRef
function WindowRef.new(payload) end



---@class WindowRef.Primary : EnumVariantRef
--- The `Primary` variant of `WindowRef`.
WindowRef.Primary = {}

---@param variant WindowRef.Primary
---@param payload NoFields
---@return WindowRef
function WindowRef.Primary.new(variant,payload) end



---@class WindowRef.Entity : EnumVariantRef
--- The `Entity` variant of `WindowRef`.
WindowRef.Entity = {}

---@param variant WindowRef.Entity
---@param payload WindowRef.Entity.Fields
---@return WindowRef
function WindowRef.Entity.new(variant,payload) end



---@class WindowResizeConstraints : ReflectReference
---  The size limits on a [`Window`].
--- 
---  These values are measured in logical pixels (see [`WindowResolution`]), so the user's
---  scale factor does affect the size limits on the window.
--- 
---  Please note that if the window is resizable, then when the window is
---  maximized it may have a size outside of these limits. The functionality
---  required to disable maximizing is not yet exposed by winit.
---@field  min_width? number
---@field  min_height? number
---@field  max_width? number
---@field  max_height? number
local WindowResizeConstraints = {}

---@param payload WindowResizeConstraints.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowResizeConstraints
function WindowResizeConstraints.new(payload) end



---@class WindowTheme : ReflectReference
---  The [`Window`] theme variant to use.
---@field  Light WindowTheme.Light
---@field  Dark WindowTheme.Dark
local WindowTheme = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return WindowTheme
function WindowTheme.new(payload) end



---@class WindowTheme.Light : EnumVariantRef
--- The `Light` variant of `WindowTheme`.
WindowTheme.Light = {}

---@param variant WindowTheme.Light
---@param payload NoFields
---@return WindowTheme
function WindowTheme.Light.new(variant,payload) end



---@class WindowTheme.Dark : EnumVariantRef
--- The `Dark` variant of `WindowTheme`.
WindowTheme.Dark = {}

---@param variant WindowTheme.Dark
---@param payload NoFields
---@return WindowTheme
function WindowTheme.Dark.new(variant,payload) end



---@class TypeId : ReflectReference
local TypeId = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return TypeId
function TypeId.new(payload) end



---@class NonZeroI16 : ReflectReference
local NonZeroI16 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NonZeroI16
function NonZeroI16.new(payload) end



---@class NonZeroU16 : ReflectReference
local NonZeroU16 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NonZeroU16
function NonZeroU16.new(payload) end



---@class NonZeroU32 : ReflectReference
local NonZeroU32 = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return NonZeroU32
function NonZeroU32.new(payload) end



---@class Affine2 : ReflectReference
---@field  matrix2? Mat2
---@field  translation? Vec2
local Affine2 = {}

---@param payload Affine2.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Affine2
function Affine2.new(payload) end



---@class Affine3A : ReflectReference
---@field  matrix3? Mat3A
---@field  translation? Vec3A
local Affine3A = {}

---@param payload Affine3A.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Affine3A
function Affine3A.new(payload) end



---@class EulerRot : ReflectReference
---@field  ZYX EulerRot.ZYX
---@field  ZXY EulerRot.ZXY
---@field  YXZ EulerRot.YXZ
---@field  YZX EulerRot.YZX
---@field  XYZ EulerRot.XYZ
---@field  XZY EulerRot.XZY
---@field  ZYZ EulerRot.ZYZ
---@field  ZXZ EulerRot.ZXZ
---@field  YXY EulerRot.YXY
---@field  YZY EulerRot.YZY
---@field  XYX EulerRot.XYX
---@field  XZX EulerRot.XZX
---@field  ZYXEx EulerRot.ZYXEx
---@field  ZXYEx EulerRot.ZXYEx
---@field  YXZEx EulerRot.YXZEx
---@field  YZXEx EulerRot.YZXEx
---@field  XYZEx EulerRot.XYZEx
---@field  XZYEx EulerRot.XZYEx
---@field  ZYZEx EulerRot.ZYZEx
---@field  ZXZEx EulerRot.ZXZEx
---@field  YXYEx EulerRot.YXYEx
---@field  YZYEx EulerRot.YZYEx
---@field  XYXEx EulerRot.XYXEx
---@field  XZXEx EulerRot.XZXEx
local EulerRot = {}

---@param payload table
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return EulerRot
function EulerRot.new(payload) end



---@class EulerRot.ZYX : EnumVariantRef
--- The `ZYX` variant of `EulerRot`.
EulerRot.ZYX = {}

---@param variant EulerRot.ZYX
---@param payload NoFields
---@return EulerRot
function EulerRot.ZYX.new(variant,payload) end



---@class EulerRot.ZXY : EnumVariantRef
--- The `ZXY` variant of `EulerRot`.
EulerRot.ZXY = {}

---@param variant EulerRot.ZXY
---@param payload NoFields
---@return EulerRot
function EulerRot.ZXY.new(variant,payload) end



---@class EulerRot.YXZ : EnumVariantRef
--- The `YXZ` variant of `EulerRot`.
EulerRot.YXZ = {}

---@param variant EulerRot.YXZ
---@param payload NoFields
---@return EulerRot
function EulerRot.YXZ.new(variant,payload) end



---@class EulerRot.YZX : EnumVariantRef
--- The `YZX` variant of `EulerRot`.
EulerRot.YZX = {}

---@param variant EulerRot.YZX
---@param payload NoFields
---@return EulerRot
function EulerRot.YZX.new(variant,payload) end



---@class EulerRot.XYZ : EnumVariantRef
--- The `XYZ` variant of `EulerRot`.
EulerRot.XYZ = {}

---@param variant EulerRot.XYZ
---@param payload NoFields
---@return EulerRot
function EulerRot.XYZ.new(variant,payload) end



---@class EulerRot.XZY : EnumVariantRef
--- The `XZY` variant of `EulerRot`.
EulerRot.XZY = {}

---@param variant EulerRot.XZY
---@param payload NoFields
---@return EulerRot
function EulerRot.XZY.new(variant,payload) end



---@class EulerRot.ZYZ : EnumVariantRef
--- The `ZYZ` variant of `EulerRot`.
EulerRot.ZYZ = {}

---@param variant EulerRot.ZYZ
---@param payload NoFields
---@return EulerRot
function EulerRot.ZYZ.new(variant,payload) end



---@class EulerRot.ZXZ : EnumVariantRef
--- The `ZXZ` variant of `EulerRot`.
EulerRot.ZXZ = {}

---@param variant EulerRot.ZXZ
---@param payload NoFields
---@return EulerRot
function EulerRot.ZXZ.new(variant,payload) end



---@class EulerRot.YXY : EnumVariantRef
--- The `YXY` variant of `EulerRot`.
EulerRot.YXY = {}

---@param variant EulerRot.YXY
---@param payload NoFields
---@return EulerRot
function EulerRot.YXY.new(variant,payload) end



---@class EulerRot.YZY : EnumVariantRef
--- The `YZY` variant of `EulerRot`.
EulerRot.YZY = {}

---@param variant EulerRot.YZY
---@param payload NoFields
---@return EulerRot
function EulerRot.YZY.new(variant,payload) end



---@class EulerRot.XYX : EnumVariantRef
--- The `XYX` variant of `EulerRot`.
EulerRot.XYX = {}

---@param variant EulerRot.XYX
---@param payload NoFields
---@return EulerRot
function EulerRot.XYX.new(variant,payload) end



---@class EulerRot.XZX : EnumVariantRef
--- The `XZX` variant of `EulerRot`.
EulerRot.XZX = {}

---@param variant EulerRot.XZX
---@param payload NoFields
---@return EulerRot
function EulerRot.XZX.new(variant,payload) end



---@class EulerRot.ZYXEx : EnumVariantRef
--- The `ZYXEx` variant of `EulerRot`.
EulerRot.ZYXEx = {}

---@param variant EulerRot.ZYXEx
---@param payload NoFields
---@return EulerRot
function EulerRot.ZYXEx.new(variant,payload) end



---@class EulerRot.ZXYEx : EnumVariantRef
--- The `ZXYEx` variant of `EulerRot`.
EulerRot.ZXYEx = {}

---@param variant EulerRot.ZXYEx
---@param payload NoFields
---@return EulerRot
function EulerRot.ZXYEx.new(variant,payload) end



---@class EulerRot.YXZEx : EnumVariantRef
--- The `YXZEx` variant of `EulerRot`.
EulerRot.YXZEx = {}

---@param variant EulerRot.YXZEx
---@param payload NoFields
---@return EulerRot
function EulerRot.YXZEx.new(variant,payload) end



---@class EulerRot.YZXEx : EnumVariantRef
--- The `YZXEx` variant of `EulerRot`.
EulerRot.YZXEx = {}

---@param variant EulerRot.YZXEx
---@param payload NoFields
---@return EulerRot
function EulerRot.YZXEx.new(variant,payload) end



---@class EulerRot.XYZEx : EnumVariantRef
--- The `XYZEx` variant of `EulerRot`.
EulerRot.XYZEx = {}

---@param variant EulerRot.XYZEx
---@param payload NoFields
---@return EulerRot
function EulerRot.XYZEx.new(variant,payload) end



---@class EulerRot.XZYEx : EnumVariantRef
--- The `XZYEx` variant of `EulerRot`.
EulerRot.XZYEx = {}

---@param variant EulerRot.XZYEx
---@param payload NoFields
---@return EulerRot
function EulerRot.XZYEx.new(variant,payload) end



---@class EulerRot.ZYZEx : EnumVariantRef
--- The `ZYZEx` variant of `EulerRot`.
EulerRot.ZYZEx = {}

---@param variant EulerRot.ZYZEx
---@param payload NoFields
---@return EulerRot
function EulerRot.ZYZEx.new(variant,payload) end



---@class EulerRot.ZXZEx : EnumVariantRef
--- The `ZXZEx` variant of `EulerRot`.
EulerRot.ZXZEx = {}

---@param variant EulerRot.ZXZEx
---@param payload NoFields
---@return EulerRot
function EulerRot.ZXZEx.new(variant,payload) end



---@class EulerRot.YXYEx : EnumVariantRef
--- The `YXYEx` variant of `EulerRot`.
EulerRot.YXYEx = {}

---@param variant EulerRot.YXYEx
---@param payload NoFields
---@return EulerRot
function EulerRot.YXYEx.new(variant,payload) end



---@class EulerRot.YZYEx : EnumVariantRef
--- The `YZYEx` variant of `EulerRot`.
EulerRot.YZYEx = {}

---@param variant EulerRot.YZYEx
---@param payload NoFields
---@return EulerRot
function EulerRot.YZYEx.new(variant,payload) end



---@class EulerRot.XYXEx : EnumVariantRef
--- The `XYXEx` variant of `EulerRot`.
EulerRot.XYXEx = {}

---@param variant EulerRot.XYXEx
---@param payload NoFields
---@return EulerRot
function EulerRot.XYXEx.new(variant,payload) end



---@class EulerRot.XZXEx : EnumVariantRef
--- The `XZXEx` variant of `EulerRot`.
EulerRot.XZXEx = {}

---@param variant EulerRot.XZXEx
---@param payload NoFields
---@return EulerRot
function EulerRot.XZXEx.new(variant,payload) end



---@class Mat2 : ReflectReference
---@field  x_axis? Vec2
---@field  y_axis? Vec2
local Mat2 = {}

---@param payload Mat2.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Mat2
function Mat2.new(payload) end



---@class Mat3A : ReflectReference
---@field  x_axis? Vec3A
---@field  y_axis? Vec3A
---@field  z_axis? Vec3A
local Mat3A = {}

---@param payload Mat3A.Fields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Mat3A
function Mat3A.new(payload) end



---@class SmolStr : ReflectReference
local SmolStr = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return SmolStr
function SmolStr.new(payload) end



---@class Uuid : ReflectReference
local Uuid = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Uuid
function Uuid.new(payload) end



---@class BlendState : ReflectReference
local BlendState = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return BlendState
function BlendState.new(payload) end



---@class Cow : ReflectReference
local Cow = {}

---@param payload NoFields
--- The fields of the type, keyed by name or index, or a list of the fields in order.
---@return Cow
function Cow.new(payload) end



---@class OsString
--- A heap allocated OS string
local OsString = {}



---@class PathBuf
--- A heap allocated file path
local PathBuf = {}



---@class Str
--- A string slice
local Str = {}



---@class Handle<A> : ReflectReference
--- A handle to an asset of type `A`.




---@class (exact) ActiveAnimation.Fields
--- The fields of `ActiveAnimation`, keyed by name. Optional fields are filled in from defaults.
---@field  weight? number
---@field  repeat? RepeatAnimation
---@field  speed? number
---@field  elapsed? number
---@field  seek_time? number
---@field  last_seek_time? number | nil
---@field  completions? integer
---@field  just_completed? boolean
---@field  paused? boolean
---@field  [any] 'never'




---@class (exact) AnimationPlayer.Fields
--- The fields of `AnimationPlayer`, keyed by name. Optional fields are filled in from defaults.
---@field  active_animations? table<NodeIndex, ActiveAnimation>
---@field  [any] 'never'




---@class (exact) AnimationTargetId.Fields
--- The fields of `AnimationTargetId`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Uuid
---@field  [any] 'never'




---@class (exact) RepeatAnimation.Count.Fields
--- The fields of the `Count` variant of `RepeatAnimation`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) AnimationGraph.Fields
--- The fields of `AnimationGraph`, keyed by name. Optional fields are filled in from defaults.
---@field  graph any
---@field  root? NodeIndex
---@field  mask_groups table<AnimationTargetId, integer>
---@field  [any] 'never'




---@class (exact) AssetIndex.Fields
--- The fields of `AssetIndex`, keyed by name. Optional fields are filled in from defaults.
---@field  generation? integer
---@field  index? integer
---@field  [any] 'never'




---@class (exact) PlaybackSettings.Fields
--- The fields of `PlaybackSettings`, keyed by name. Optional fields are filled in from defaults.
---@field  mode? PlaybackMode
---@field  volume? Volume
---@field  speed? number
---@field  paused? boolean
---@field  muted? boolean
---@field  spatial? boolean
---@field  spatial_scale? SpatialScale | nil
---@field  start_position? Duration | nil
---@field  duration? Duration | nil
---@field  [any] 'never'




---@class (exact) SpatialScale.Fields
--- The fields of `SpatialScale`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Vec3
---@field  [any] 'never'




---@class (exact) Volume.Decibels.Fields
--- The fields of the `Decibels` variant of `Volume`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Volume.Linear.Fields
--- The fields of the `Linear` variant of `Volume`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Camera.Fields
--- The fields of `Camera`, keyed by name. Optional fields are filled in from defaults.
---@field  viewport? Viewport | nil
---@field  order? integer
---@field  is_active? boolean
---@field  computed? ComputedCameraValues
---@field  output_mode? CameraOutputMode
---@field  msaa_writeback? MsaaWriteback
---@field  clear_color? ClearColorConfig
---@field  invert_culling? boolean
---@field  sub_camera_view? SubCameraView | nil
---@field  [any] 'never'




---@class (exact) CameraOutputMode.Write.Fields
--- The fields of the `Write` variant of `CameraOutputMode`, keyed by name. Optional fields are filled in from defaults.
---@field  blend_state BlendState | nil
---@field  clear_color? ClearColorConfig
---@field  [any] 'never'




---@class (exact) ComputedCameraValues.Fields
--- The fields of `ComputedCameraValues`, keyed by name. Optional fields are filled in from defaults.
---@field  clip_from_view? Mat4
---@field  target_info RenderTargetInfo | nil
---@field  old_viewport_size UVec2 | nil
---@field  old_sub_camera_view SubCameraView | nil
---@field  [any] 'never'




---@class (exact) ImageRenderTarget.Fields
--- The fields of `ImageRenderTarget`, keyed by name. Optional fields are filled in from defaults.
---@field  handle Handle<Image>
---@field  scale_factor? number
---@field  [any] 'never'




---@class (exact) ManualTextureViewHandle.Fields
--- The fields of `ManualTextureViewHandle`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) RenderTarget.Image.Fields
--- The fields of the `Image` variant of `RenderTarget`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] ImageRenderTarget
---@field  [any] 'never'




---@class (exact) RenderTarget.None.Fields
--- The fields of the `None` variant of `RenderTarget`, keyed by name. Optional fields are filled in from defaults.
---@field  size? UVec2
---@field  [any] 'never'




---@class (exact) RenderTarget.TextureView.Fields
--- The fields of the `TextureView` variant of `RenderTarget`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? ManualTextureViewHandle
---@field  [any] 'never'




---@class (exact) RenderTarget.Window.Fields
--- The fields of the `Window` variant of `RenderTarget`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? WindowRef
---@field  [any] 'never'




---@class (exact) RenderTargetInfo.Fields
--- The fields of `RenderTargetInfo`, keyed by name. Optional fields are filled in from defaults.
---@field  physical_size? UVec2
---@field  scale_factor? number
---@field  [any] 'never'




---@class (exact) SubCameraView.Fields
--- The fields of `SubCameraView`, keyed by name. Optional fields are filled in from defaults.
---@field  full_size? UVec2
---@field  offset? Vec2
---@field  size? UVec2
---@field  [any] 'never'




---@class (exact) Viewport.Fields
--- The fields of `Viewport`, keyed by name. Optional fields are filled in from defaults.
---@field  physical_position? UVec2
---@field  physical_size? UVec2
---@field  depth? any
---@field  [any] 'never'




---@class (exact) ClearColorConfig.Custom.Fields
--- The fields of the `Custom` variant of `ClearColorConfig`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Color
---@field  [any] 'never'




---@class (exact) OrthographicProjection.Fields
--- The fields of `OrthographicProjection`, keyed by name. Optional fields are filled in from defaults.
---@field  near? number
---@field  far? number
---@field  viewport_origin? Vec2
---@field  scaling_mode? ScalingMode
---@field  scale? number
---@field  area? Rect
---@field  [any] 'never'




---@class (exact) ScalingMode.AutoMax.Fields
--- The fields of the `AutoMax` variant of `ScalingMode`, keyed by name. Optional fields are filled in from defaults.
---@field  max_width? number
---@field  max_height? number
---@field  [any] 'never'




---@class (exact) ScalingMode.AutoMin.Fields
--- The fields of the `AutoMin` variant of `ScalingMode`, keyed by name. Optional fields are filled in from defaults.
---@field  min_width? number
---@field  min_height? number
---@field  [any] 'never'




---@class (exact) ScalingMode.Fixed.Fields
--- The fields of the `Fixed` variant of `ScalingMode`, keyed by name. Optional fields are filled in from defaults.
---@field  width? number
---@field  height? number
---@field  [any] 'never'




---@class (exact) ScalingMode.FixedHorizontal.Fields
--- The fields of the `FixedHorizontal` variant of `ScalingMode`, keyed by name. Optional fields are filled in from defaults.
---@field  viewport_width? number
---@field  [any] 'never'




---@class (exact) ScalingMode.FixedVertical.Fields
--- The fields of the `FixedVertical` variant of `ScalingMode`, keyed by name. Optional fields are filled in from defaults.
---@field  viewport_height? number
---@field  [any] 'never'




---@class (exact) InheritedVisibility.Fields
--- The fields of `InheritedVisibility`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? boolean
---@field  [any] 'never'




---@class (exact) ViewVisibility.Fields
--- The fields of `ViewVisibility`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) RenderLayers.Fields
--- The fields of `RenderLayers`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer[]
---@field  [any] 'never'




---@class (exact) Color.Hsla.Fields
--- The fields of the `Hsla` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Hsla
---@field  [any] 'never'




---@class (exact) Color.Hsva.Fields
--- The fields of the `Hsva` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Hsva
---@field  [any] 'never'




---@class (exact) Color.Hwba.Fields
--- The fields of the `Hwba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Hwba
---@field  [any] 'never'




---@class (exact) Color.Laba.Fields
--- The fields of the `Laba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Laba
---@field  [any] 'never'




---@class (exact) Color.Lcha.Fields
--- The fields of the `Lcha` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Lcha
---@field  [any] 'never'




---@class (exact) Color.LinearRgba.Fields
--- The fields of the `LinearRgba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? LinearRgba
---@field  [any] 'never'




---@class (exact) Color.Okhsla.Fields
--- The fields of the `Okhsla` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Okhsla
---@field  [any] 'never'




---@class (exact) Color.Okhsva.Fields
--- The fields of the `Okhsva` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Okhsva
---@field  [any] 'never'




---@class (exact) Color.Okhwba.Fields
--- The fields of the `Okhwba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Okhwba
---@field  [any] 'never'




---@class (exact) Color.Oklaba.Fields
--- The fields of the `Oklaba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Oklaba
---@field  [any] 'never'




---@class (exact) Color.Oklcha.Fields
--- The fields of the `Oklcha` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Oklcha
---@field  [any] 'never'




---@class (exact) Color.Srgba.Fields
--- The fields of the `Srgba` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Srgba
---@field  [any] 'never'




---@class (exact) Color.Xyza.Fields
--- The fields of the `Xyza` variant of `Color`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Xyza
---@field  [any] 'never'




---@class (exact) Okhsla.Fields
--- The fields of `Okhsla`, keyed by name. Optional fields are filled in from defaults.
---@field  hue? number
---@field  saturation? number
---@field  lightness? number
---@field  alpha? number
---@field  [any] 'never'




---@class (exact) Okhsva.Fields
--- The fields of `Okhsva`, keyed by name. Optional fields are filled in from defaults.
---@field  hue? number
---@field  saturation? number
---@field  value? number
---@field  alpha? number
---@field  [any] 'never'




---@class (exact) Okhwba.Fields
--- The fields of `Okhwba`, keyed by name. Optional fields are filled in from defaults.
---@field  hue? number
---@field  whiteness? number
---@field  blackness? number
---@field  alpha? number
---@field  [any] 'never'




---@class (exact) EaseFunction.Elastic.Fields
--- The fields of the `Elastic` variant of `EaseFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) EaseFunction.Steps.Fields
--- The fields of the `Steps` variant of `EaseFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [2]? JumpAt
---@field  [any] 'never'




---@class (exact) ComponentId.Fields
--- The fields of `ComponentId`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) ChildOf.Fields
--- The fields of `ChildOf`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Entity
---@field  [any] 'never'




---@class (exact) AddEvent.Fields
--- The fields of `AddEvent`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  [any] 'never'




---@class (exact) DespawnEvent.Fields
--- The fields of `DespawnEvent`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  [any] 'never'




---@class (exact) DiscardEvent.Fields
--- The fields of `DiscardEvent`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  [any] 'never'




---@class (exact) InsertEvent.Fields
--- The fields of `InsertEvent`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  [any] 'never'




---@class (exact) RemoveEvent.Fields
--- The fields of `RemoveEvent`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  [any] 'never'




---@class (exact) HashedStr.Fields
--- The fields of `HashedStr`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] any
---@field  [any] 'never'




---@class (exact) TextureAtlas.Fields
--- The fields of `TextureAtlas`, keyed by name. Optional fields are filled in from defaults.
---@field  layout? Handle<TextureAtlasLayout>
---@field  index? integer
---@field  [any] 'never'




---@class (exact) TextureAtlasLayout.Fields
--- The fields of `TextureAtlasLayout`, keyed by name. Optional fields are filled in from defaults.
---@field  size? UVec2
---@field  textures URect[]
---@field  [any] 'never'




---@class (exact) Gamepad.Fields
--- The fields of `Gamepad`, keyed by name. Optional fields are filled in from defaults.
---@field  vendor_id? integer | nil
---@field  product_id? integer | nil
---@field  digital? any
---@field  analog? any
---@field  [any] 'never'




---@class (exact) GamepadAxis.Other.Fields
--- The fields of the `Other` variant of `GamepadAxis`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) GamepadButton.Other.Fields
--- The fields of the `Other` variant of `GamepadButton`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) GamepadInput.Axis.Fields
--- The fields of the `Axis` variant of `GamepadInput`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] GamepadAxis
---@field  [any] 'never'




---@class (exact) GamepadInput.Button.Fields
--- The fields of the `Button` variant of `GamepadInput`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] GamepadButton
---@field  [any] 'never'




---@class (exact) KeyCode.Unidentified.Fields
--- The fields of the `Unidentified` variant of `KeyCode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] NativeKeyCode
---@field  [any] 'never'




---@class (exact) NativeKeyCode.Android.Fields
--- The fields of the `Android` variant of `NativeKeyCode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) NativeKeyCode.MacOS.Fields
--- The fields of the `MacOS` variant of `NativeKeyCode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) NativeKeyCode.Windows.Fields
--- The fields of the `Windows` variant of `NativeKeyCode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) NativeKeyCode.Xkb.Fields
--- The fields of the `Xkb` variant of `NativeKeyCode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) MouseButton.Other.Fields
--- The fields of the `Other` variant of `MouseButton`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) InputFocus.Fields
--- The fields of `InputFocus`, keyed by name. Optional fields are filled in from defaults.
---@field  current_focus? Entity | nil
---@field  recorded_changes? [Entity, FocusCause] | nil[]
---@field  original_focus? Entity | nil
---@field  [any] 'never'




---@class (exact) AlphaMode.Mask.Fields
--- The fields of the `Mask` variant of `AlphaMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) URect.Fields
--- The fields of `URect`, keyed by name. Optional fields are filled in from defaults.
---@field  min? UVec2
---@field  max? UVec2
---@field  [any] 'never'




---@class (exact) Rot2.Fields
--- The fields of `Rot2`, keyed by name. Optional fields are filled in from defaults.
---@field  cos? number
---@field  sin? number
---@field  [any] 'never'




---@class (exact) Indices.U16.Fields
--- The fields of the `U16` variant of `Indices`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] integer[]
---@field  [any] 'never'




---@class (exact) Indices.U32.Fields
--- The fields of the `U32` variant of `Indices`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] integer[]
---@field  [any] 'never'




---@class (exact) Mesh.Fields
--- The fields of `Mesh`, keyed by name. Optional fields are filled in from defaults.
---@field  indices any
---@field  morph_targets any
---@field  morph_target_names any
---@field  asset_usage RenderAssetUsages
---@field  enable_raytracing? boolean
---@field  attribute_compression MeshAttributeCompressionFlags
---@field  final_aabb Aabb3d | nil
---@field  final_uv_ranges Aabb2d | nil[]
---@field  skinned_mesh_bounds SkinnedMeshBounds | nil
---@field  [any] 'never'




---@class (exact) MorphAttributes.Fields
--- The fields of `MorphAttributes`, keyed by name. Optional fields are filled in from defaults.
---@field  position? Vec3
---@field  pad_a? number
---@field  normal? Vec3
---@field  pad_b? number
---@field  tangent? Vec3
---@field  pad_c? number
---@field  [any] 'never'




---@class (exact) JointAabb.Fields
--- The fields of `JointAabb`, keyed by name. Optional fields are filled in from defaults.
---@field  center? Vec3
---@field  half_size? Vec3
---@field  [any] 'never'




---@class (exact) JointIndex.Fields
--- The fields of `JointIndex`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) SkinnedMeshBounds.Fields
--- The fields of `SkinnedMeshBounds`, keyed by name. Optional fields are filled in from defaults.
---@field  aabbs JointAabb[]
---@field  aabb_index_to_joint_index JointIndex[]
---@field  [any] 'never'




---@class (exact) Language.External.Fields
--- The fields of the `External` variant of `Language`, keyed by name. Optional fields are filled in from defaults.
---@field  name Cow
---@field  one_indexed? boolean
---@field  [any] 'never'




---@class (exact) FunctionArgInfo.Fields
--- The fields of `FunctionArgInfo`, keyed by name. Optional fields are filled in from defaults.
---@field  name Cow | nil
---@field  arg_index? integer
---@field  type_id TypeId
---@field  [any] 'never'




---@class (exact) FunctionInfo.Fields
--- The fields of `FunctionInfo`, keyed by name. Optional fields are filled in from defaults.
---@field  name Cow
---@field  namespace Namespace
---@field  arg_info FunctionArgInfo[]
---@field  return_info FunctionReturnInfo
---@field  docs Cow | nil
---@field  [any] 'never'




---@class (exact) FunctionReturnInfo.Fields
--- The fields of `FunctionReturnInfo`, keyed by name. Optional fields are filled in from defaults.
---@field  type_id TypeId
---@field  [any] 'never'




---@class (exact) Namespace.OnType.Fields
--- The fields of the `OnType` variant of `Namespace`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] TypeId
---@field  [any] 'never'




---@class (exact) ScriptComponentRegistration.Fields
--- The fields of `ScriptComponentRegistration`, keyed by name. Optional fields are filled in from defaults.
---@field  registration ScriptTypeRegistration
---@field  component_id ComponentId
---@field  is_dynamic_script_component? boolean
---@field  script_namespace string | nil
---@field  [any] 'never'




---@class (exact) ScriptResourceRegistration.Fields
--- The fields of `ScriptResourceRegistration`, keyed by name. Optional fields are filled in from defaults.
---@field  registration ScriptTypeRegistration
---@field  resource_id ComponentId
---@field  [any] 'never'




---@class (exact) DynamicComponent.Fields
--- The fields of `DynamicComponent`, keyed by name. Optional fields are filled in from defaults.
---@field  data? ScriptValue
---@field  [any] 'never'




---@class (exact) CallbackLabel.Fields
--- The fields of `CallbackLabel`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? string
---@field  [any] 'never'




---@class (exact) ScriptAssetModifiedMessage.Fields
--- The fields of `ScriptAssetModifiedMessage`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? any
---@field  [any] 'never'




---@class (exact) ScriptAttachedMessage.Fields
--- The fields of `ScriptAttachedMessage`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] ScriptAttachment
---@field  [any] 'never'




---@class (exact) ScriptCallbackMessage.Fields
--- The fields of `ScriptCallbackMessage`, keyed by name. Optional fields are filled in from defaults.
---@field  label CallbackLabel
---@field  recipients Recipients
---@field  language? Language | nil
---@field  args ScriptValue[]
---@field  trigger_response? boolean
---@field  iteration? integer
---@field  [any] 'never'




---@class (exact) ScriptDetachedMessage.Fields
--- The fields of `ScriptDetachedMessage`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] ScriptAttachment
---@field  [any] 'never'




---@class (exact) ScriptComponent.Fields
--- The fields of `ScriptComponent`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Handle<ScriptAsset>[]
---@field  [any] 'never'




---@class (exact) Recipients.ScriptEntity.Fields
--- The fields of the `ScriptEntity` variant of `Recipients`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Handle<ScriptAsset>
---@field  [2] Entity
---@field  [any] 'never'




---@class (exact) Recipients.StaticScript.Fields
--- The fields of the `StaticScript` variant of `Recipients`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Handle<ScriptAsset>
---@field  [any] 'never'




---@class (exact) ScriptAttachment.EntityScript.Fields
--- The fields of the `EntityScript` variant of `ScriptAttachment`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Entity
---@field  [2] Handle<ScriptAsset>
---@field  [any] 'never'




---@class (exact) ScriptAttachment.StaticScript.Fields
--- The fields of the `StaticScript` variant of `ScriptAttachment`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Handle<ScriptAsset>
---@field  [any] 'never'




---@class (exact) ParallaxMappingMethod.Relief.Fields
--- The fields of the `Relief` variant of `ParallaxMappingMethod`, keyed by name. Optional fields are filled in from defaults.
---@field  max_steps? integer
---@field  [any] 'never'




---@class (exact) StandardMaterial.Fields
--- The fields of `StandardMaterial`, keyed by name. Optional fields are filled in from defaults.
---@field  base_color? Color
---@field  base_color_channel? UvChannel
---@field  base_color_texture? Handle<Image> | nil
---@field  emissive? LinearRgba
---@field  emissive_exposure_weight? number
---@field  emissive_channel? UvChannel
---@field  emissive_texture? Handle<Image> | nil
---@field  perceptual_roughness? number
---@field  metallic? number
---@field  metallic_roughness_channel? UvChannel
---@field  metallic_roughness_texture? Handle<Image> | nil
---@field  reflectance? number
---@field  specular_tint? Color
---@field  diffuse_transmission? number
---@field  specular_transmission? number
---@field  thickness? number
---@field  ior? number
---@field  attenuation_distance? number
---@field  attenuation_color? Color
---@field  normal_map_channel? UvChannel
---@field  normal_map_texture? Handle<Image> | nil
---@field  flip_normal_map_y? boolean
---@field  occlusion_channel? UvChannel
---@field  occlusion_texture? Handle<Image> | nil
---@field  clearcoat? number
---@field  clearcoat_perceptual_roughness? number
---@field  anisotropy_strength? number
---@field  anisotropy_rotation? number
---@field  double_sided? boolean
---@field  unlit? boolean
---@field  fog_enabled? boolean
---@field  alpha_mode? AlphaMode
---@field  depth_bias? number
---@field  depth_map? Handle<Image> | nil
---@field  parallax_depth_scale? number
---@field  parallax_mapping_method? ParallaxMappingMethod
---@field  max_parallax_layer_count? number
---@field  lightmap_exposure? number
---@field  opaque_render_method? OpaqueRendererMethod
---@field  deferred_lighting_pass_id? integer
---@field  uv_transform? Affine2
---@field  [any] 'never'




---@class (exact) Screenshot.Fields
--- The fields of `Screenshot`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] RenderTarget
---@field  [any] 'never'




---@class (exact) ScreenshotCaptured.Fields
--- The fields of `ScreenshotCaptured`, keyed by name. Optional fields are filled in from defaults.
---@field  entity Entity
---@field  image? Image
---@field  [any] 'never'




---@class (exact) Aabb2d.Fields
--- The fields of `Aabb2d`, keyed by name. Optional fields are filled in from defaults.
---@field  min? Vec2
---@field  max? Vec2
---@field  [any] 'never'




---@class (exact) Aabb3d.Fields
--- The fields of `Aabb3d`, keyed by name. Optional fields are filled in from defaults.
---@field  min? Vec3A
---@field  max? Vec3A
---@field  [any] 'never'




---@class (exact) Anchor.Fields
--- The fields of `Anchor`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Vec2
---@field  [any] 'never'




---@class (exact) Sprite.Fields
--- The fields of `Sprite`, keyed by name. Optional fields are filled in from defaults.
---@field  image? Handle<Image>
---@field  texture_atlas? TextureAtlas | nil
---@field  color? Color
---@field  flip_x? boolean
---@field  flip_y? boolean
---@field  custom_size? Vec2 | nil
---@field  rect? Rect | nil
---@field  image_mode? SpriteImageMode
---@field  alpha_mode? SpriteAlphaMode
---@field  [any] 'never'




---@class (exact) SpriteAlphaMode.Mask.Fields
--- The fields of the `Mask` variant of `SpriteAlphaMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) SpriteImageMode.Scale.Fields
--- The fields of the `Scale` variant of `SpriteImageMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? SpriteScalingMode
---@field  [any] 'never'




---@class (exact) SpriteImageMode.Sliced.Fields
--- The fields of the `Sliced` variant of `SpriteImageMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] TextureSlicer
---@field  [any] 'never'




---@class (exact) SpriteImageMode.Tiled.Fields
--- The fields of the `Tiled` variant of `SpriteImageMode`, keyed by name. Optional fields are filled in from defaults.
---@field  tile_x? boolean
---@field  tile_y? boolean
---@field  stretch_value? number
---@field  [any] 'never'




---@class (exact) BorderRect.Fields
--- The fields of `BorderRect`, keyed by name. Optional fields are filled in from defaults.
---@field  min_inset? Vec2
---@field  max_inset? Vec2
---@field  [any] 'never'




---@class (exact) SliceScaleMode.Tile.Fields
--- The fields of the `Tile` variant of `SliceScaleMode`, keyed by name. Optional fields are filled in from defaults.
---@field  stretch_value? number
---@field  [any] 'never'




---@class (exact) TextureSlicer.Fields
--- The fields of `TextureSlicer`, keyed by name. Optional fields are filled in from defaults.
---@field  border? BorderRect
---@field  center_scale_mode? SliceScaleMode
---@field  sides_scale_mode? SliceScaleMode
---@field  max_corner_scale? number
---@field  [any] 'never'




---@class (exact) ColorMaterial.Fields
--- The fields of `ColorMaterial`, keyed by name. Optional fields are filled in from defaults.
---@field  color? Color
---@field  alpha_mode? AlphaMode2d
---@field  uv_transform? Affine2
---@field  texture? Handle<Image> | nil
---@field  [any] 'never'




---@class (exact) AlphaMode2d.Mask.Fields
--- The fields of the `Mask` variant of `AlphaMode2d`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) SpriteMeshMaterial.Fields
--- The fields of `SpriteMeshMaterial`, keyed by name. Optional fields are filled in from defaults.
---@field  image Handle<Image>
---@field  texture_atlas TextureAtlas | nil
---@field  color? Color
---@field  flip_x? boolean
---@field  flip_y? boolean
---@field  custom_size Vec2 | nil
---@field  rect Rect | nil
---@field  image_mode? SpriteImageMode
---@field  alpha_mode? AlphaMode2d
---@field  anchor? Vec2
---@field  texture_atlas_layout TextureAtlasLayout | nil
---@field  texture_atlas_index? integer
---@field  [any] 'never'




---@class (exact) ReflectSchedule.Fields
--- The fields of `ReflectSchedule`, keyed by name. Optional fields are filled in from defaults.
---@field  type_path string
---@field  label ReflectableScheduleLabel
---@field  [any] 'never'




---@class (exact) EmSize.Fields
--- The fields of `EmSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontFeatureTag.Fields
--- The fields of `FontFeatureTag`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] integer[]
---@field  [any] 'never'




---@class (exact) FontFeatures.Fields
--- The fields of `FontFeatures`, keyed by name. Optional fields are filled in from defaults.
---@field  features [FontFeatureTag, integer][]
---@field  [any] 'never'




---@class (exact) FontSize.Px.Fields
--- The fields of the `Px` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSize.Rem.Fields
--- The fields of the `Rem` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSize.VMax.Fields
--- The fields of the `VMax` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSize.VMin.Fields
--- The fields of the `VMin` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSize.Vh.Fields
--- The fields of the `Vh` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSize.Vw.Fields
--- The fields of the `Vw` variant of `FontSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) FontSource.Families.Fields
--- The fields of the `Families` variant of `FontSource`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? SmolStr
---@field  [any] 'never'




---@class (exact) FontSource.Family.Fields
--- The fields of the `Family` variant of `FontSource`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? SmolStr
---@field  [any] 'never'




---@class (exact) FontSource.Generic.Fields
--- The fields of the `Generic` variant of `FontSource`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] GenericFontFamily
---@field  [any] 'never'




---@class (exact) FontSource.Handle.Fields
--- The fields of the `Handle` variant of `FontSource`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] bevy_asset::handle::Handle<bevy_text::font::Font>
---@field  [any] 'never'




---@class (exact) FontSource.List.Fields
--- The fields of the `List` variant of `FontSource`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] FontSource[]
---@field  [any] 'never'




---@class (exact) FontStyle.Oblique.Fields
--- The fields of the `Oblique` variant of `FontStyle`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] number | nil
---@field  [any] 'never'




---@class (exact) FontVariationTag.Fields
--- The fields of `FontVariationTag`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] integer[]
---@field  [any] 'never'




---@class (exact) FontVariations.Fields
--- The fields of `FontVariations`, keyed by name. Optional fields are filled in from defaults.
---@field  variations [FontVariationTag, number][]
---@field  [any] 'never'




---@class (exact) FontWeight.Fields
--- The fields of `FontWeight`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) FontWidth.Fields
--- The fields of `FontWidth`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) RemSize.Fields
--- The fields of `RemSize`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) TextFont.Fields
--- The fields of `TextFont`, keyed by name. Optional fields are filled in from defaults.
---@field  font? FontSource
---@field  font_size? FontSize
---@field  weight? FontWeight
---@field  width? FontWidth
---@field  style? FontStyle
---@field  font_smoothing? FontSmoothing
---@field  font_features? FontFeatures
---@field  font_variations? FontVariations
---@field  [any] 'never'




---@class (exact) TextLayout.Fields
--- The fields of `TextLayout`, keyed by name. Optional fields are filled in from defaults.
---@field  justify? Justify
---@field  linebreak? LineBreak
---@field  [any] 'never'




---@class (exact) Fixed.Fields
--- The fields of `Fixed`, keyed by name. Optional fields are filled in from defaults.
---@field  timestep? Duration
---@field  overstep? Duration
---@field  [any] 'never'




---@class (exact) Real.Fields
--- The fields of `Real`, keyed by name. Optional fields are filled in from defaults.
---@field  startup? Instant
---@field  first_update? Instant | nil
---@field  last_update? Instant | nil
---@field  [any] 'never'




---@class (exact) Virtual.Fields
--- The fields of `Virtual`, keyed by name. Optional fields are filled in from defaults.
---@field  max_delta? Duration
---@field  paused? boolean
---@field  relative_speed? number
---@field  effective_speed? number
---@field  [any] 'never'




---@class (exact) GlobalTransform.Fields
--- The fields of `GlobalTransform`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Affine3A
---@field  [any] 'never'




---@class (exact) Transform.Fields
--- The fields of `Transform`, keyed by name. Optional fields are filled in from defaults.
---@field  translation? Vec3
---@field  rotation? Quat
---@field  scale? Vec3
---@field  [any] 'never'




---@class (exact) CornerRadius.Fields
--- The fields of `CornerRadius`, keyed by name. Optional fields are filled in from defaults.
---@field  x? Val
---@field  y? Val
---@field  [any] 'never'




---@class (exact) Val.Em.Fields
--- The fields of the `Em` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.Percent.Fields
--- The fields of the `Percent` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.Px.Fields
--- The fields of the `Px` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.Rem.Fields
--- The fields of the `Rem` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.VMax.Fields
--- The fields of the `VMax` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.VMin.Fields
--- The fields of the `VMin` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.Vh.Fields
--- The fields of the `Vh` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Val.Vw.Fields
--- The fields of the `Vw` variant of `Val`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) BackgroundColor.Fields
--- The fields of `BackgroundColor`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? Color
---@field  [any] 'never'




---@class (exact) BorderColor.Fields
--- The fields of `BorderColor`, keyed by name. Optional fields are filled in from defaults.
---@field  top? Color
---@field  right? Color
---@field  bottom? Color
---@field  left? Color
---@field  [any] 'never'




---@class (exact) BorderRadius.Fields
--- The fields of `BorderRadius`, keyed by name. Optional fields are filled in from defaults.
---@field  top_left? CornerRadius
---@field  top_right? CornerRadius
---@field  bottom_right? CornerRadius
---@field  bottom_left? CornerRadius
---@field  [any] 'never'




---@class (exact) ComputedNode.Fields
--- The fields of `ComputedNode`, keyed by name. Optional fields are filled in from defaults.
---@field  size? Vec2
---@field  content_size? Vec2
---@field  scrollbar_size? Vec2
---@field  scroll_position? Vec2
---@field  outline_width? number
---@field  outline_offset? number
---@field  unrounded_size? Vec2
---@field  border? BorderRect
---@field  border_radius? ResolvedBorderRadius
---@field  padding? BorderRect
---@field  inverse_scale_factor? number
---@field  em_size? EmSize
---@field  rem_size? RemSize
---@field  [any] 'never'




---@class (exact) GlobalZIndex.Fields
--- The fields of `GlobalZIndex`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) GridPlacement.Fields
--- The fields of `GridPlacement`, keyed by name. Optional fields are filled in from defaults.
---@field  start? NonZeroI16 | nil
---@field  span? NonZeroU16 | nil
---@field  end? NonZeroI16 | nil
---@field  [any] 'never'




---@class (exact) GridTrack.Fields
--- The fields of `GridTrack`, keyed by name. Optional fields are filled in from defaults.
---@field  min_sizing_function? MinTrackSizingFunction
---@field  max_sizing_function? MaxTrackSizingFunction
---@field  [any] 'never'




---@class (exact) GridTrackRepetition.Count.Fields
--- The fields of the `Count` variant of `GridTrackRepetition`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Em.Fields
--- The fields of the `Em` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.FitContentPercent.Fields
--- The fields of the `FitContentPercent` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.FitContentPx.Fields
--- The fields of the `FitContentPx` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Fraction.Fields
--- The fields of the `Fraction` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Percent.Fields
--- The fields of the `Percent` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Px.Fields
--- The fields of the `Px` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Rem.Fields
--- The fields of the `Rem` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.VMax.Fields
--- The fields of the `VMax` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.VMin.Fields
--- The fields of the `VMin` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Vh.Fields
--- The fields of the `Vh` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MaxTrackSizingFunction.Vw.Fields
--- The fields of the `Vw` variant of `MaxTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Em.Fields
--- The fields of the `Em` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Percent.Fields
--- The fields of the `Percent` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Px.Fields
--- The fields of the `Px` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Rem.Fields
--- The fields of the `Rem` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.VMax.Fields
--- The fields of the `VMax` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.VMin.Fields
--- The fields of the `VMin` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Vh.Fields
--- The fields of the `Vh` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) MinTrackSizingFunction.Vw.Fields
--- The fields of the `Vw` variant of `MinTrackSizingFunction`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? number
---@field  [any] 'never'




---@class (exact) Node.Fields
--- The fields of `Node`, keyed by name. Optional fields are filled in from defaults.
---@field  display? Display
---@field  box_sizing? BoxSizing
---@field  position_type? PositionType
---@field  overflow? Overflow
---@field  scrollbar_width? number
---@field  overflow_clip_margin? OverflowClipMargin
---@field  left? Val
---@field  right? Val
---@field  top? Val
---@field  bottom? Val
---@field  width? Val
---@field  height? Val
---@field  min_width? Val
---@field  min_height? Val
---@field  max_width? Val
---@field  max_height? Val
---@field  aspect_ratio? number | nil
---@field  align_items? AlignItems
---@field  justify_items? JustifyItems
---@field  align_self? AlignSelf
---@field  justify_self? JustifySelf
---@field  align_content? AlignContent
---@field  justify_content? JustifyContent
---@field  direction? InlineDirection
---@field  margin? UiRect
---@field  padding? UiRect
---@field  border? UiRect
---@field  border_radius? BorderRadius
---@field  flex_direction? FlexDirection
---@field  flex_wrap? FlexWrap
---@field  flex_grow? number
---@field  flex_shrink? number
---@field  flex_basis? Val
---@field  row_gap? Val
---@field  column_gap? Val
---@field  grid_auto_flow? GridAutoFlow
---@field  grid_template_rows? RepeatedGridTrack[]
---@field  grid_template_columns? RepeatedGridTrack[]
---@field  grid_auto_rows? GridTrack[]
---@field  grid_auto_columns? GridTrack[]
---@field  grid_row? GridPlacement
---@field  grid_column? GridPlacement
---@field  [any] 'never'




---@class (exact) Outline.Fields
--- The fields of `Outline`, keyed by name. Optional fields are filled in from defaults.
---@field  width? Val
---@field  offset? Val
---@field  color? Color
---@field  [any] 'never'




---@class (exact) Overflow.Fields
--- The fields of `Overflow`, keyed by name. Optional fields are filled in from defaults.
---@field  x? OverflowAxis
---@field  y? OverflowAxis
---@field  [any] 'never'




---@class (exact) OverflowClipMargin.Fields
--- The fields of `OverflowClipMargin`, keyed by name. Optional fields are filled in from defaults.
---@field  visual_box? VisualBox
---@field  margin? number
---@field  [any] 'never'




---@class (exact) RepeatedGridTrack.Fields
--- The fields of `RepeatedGridTrack`, keyed by name. Optional fields are filled in from defaults.
---@field  repetition? GridTrackRepetition
---@field  tracks? GridTrack[]
---@field  [any] 'never'




---@class (exact) ResolvedBorderRadius.Fields
--- The fields of `ResolvedBorderRadius`, keyed by name. Optional fields are filled in from defaults.
---@field  top_left? Vec2
---@field  top_right? Vec2
---@field  bottom_right? Vec2
---@field  bottom_left? Vec2
---@field  [any] 'never'




---@class (exact) ZIndex.Fields
--- The fields of `ZIndex`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) NodeImageMode.Sliced.Fields
--- The fields of the `Sliced` variant of `NodeImageMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] TextureSlicer
---@field  [any] 'never'




---@class (exact) NodeImageMode.Tiled.Fields
--- The fields of the `Tiled` variant of `NodeImageMode`, keyed by name. Optional fields are filled in from defaults.
---@field  tile_x? boolean
---@field  tile_y? boolean
---@field  stretch_value? number
---@field  [any] 'never'




---@class (exact) VideoMode.Fields
--- The fields of `VideoMode`, keyed by name. Optional fields are filled in from defaults.
---@field  physical_size? UVec2
---@field  bit_depth? integer
---@field  refresh_rate_millihertz? integer
---@field  [any] 'never'




---@class (exact) EnabledButtons.Fields
--- The fields of `EnabledButtons`, keyed by name. Optional fields are filled in from defaults.
---@field  minimize? boolean
---@field  maximize? boolean
---@field  close? boolean
---@field  [any] 'never'




---@class (exact) InternalWindowState.Fields
--- The fields of `InternalWindowState`, keyed by name. Optional fields are filled in from defaults.
---@field  minimize_request? boolean | nil
---@field  maximize_request? boolean | nil
---@field  drag_move_request? boolean
---@field  drag_resize_request? CompassOctant | nil
---@field  physical_cursor_position? DVec2 | nil
---@field  [any] 'never'




---@class (exact) MonitorSelection.Entity.Fields
--- The fields of the `Entity` variant of `MonitorSelection`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Entity
---@field  [any] 'never'




---@class (exact) MonitorSelection.Index.Fields
--- The fields of the `Index` variant of `MonitorSelection`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? integer
---@field  [any] 'never'




---@class (exact) VideoModeSelection.Specific.Fields
--- The fields of the `Specific` variant of `VideoModeSelection`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] VideoMode
---@field  [any] 'never'




---@class (exact) Window.Fields
--- The fields of `Window`, keyed by name. Optional fields are filled in from defaults.
---@field  present_mode? PresentMode
---@field  mode? WindowMode
---@field  position? WindowPosition
---@field  resolution? WindowResolution
---@field  title? string
---@field  name? string | nil
---@field  composite_alpha_mode? CompositeAlphaMode
---@field  resize_constraints? WindowResizeConstraints
---@field  resizable? boolean
---@field  enabled_buttons? EnabledButtons
---@field  decorations? boolean
---@field  transparent? boolean
---@field  focused? boolean
---@field  window_level? WindowLevel
---@field  canvas? string | nil
---@field  fit_canvas_to_parent? boolean
---@field  prevent_default_event_handling? boolean
---@field  internal? InternalWindowState
---@field  ime_enabled? boolean
---@field  ime_position? Vec2
---@field  window_theme? WindowTheme | nil
---@field  visible? boolean
---@field  skip_taskbar? boolean
---@field  clip_children? boolean
---@field  desired_maximum_frame_latency? NonZeroU32 | nil
---@field  recognize_pinch_gesture? boolean
---@field  recognize_rotation_gesture? boolean
---@field  recognize_doubletap_gesture? boolean
---@field  recognize_pan_gesture? [integer, integer] | nil
---@field  movable_by_window_background? boolean
---@field  fullsize_content_view? boolean
---@field  has_shadow? boolean
---@field  titlebar_shown? boolean
---@field  titlebar_transparent? boolean
---@field  titlebar_show_title? boolean
---@field  titlebar_show_buttons? boolean
---@field  borderless_game? boolean
---@field  prefers_home_indicator_hidden? boolean
---@field  prefers_status_bar_hidden? boolean
---@field  preferred_screen_edges_deferring_system_gestures? ScreenEdge
---@field  [any] 'never'




---@class (exact) WindowMode.BorderlessFullscreen.Fields
--- The fields of the `BorderlessFullscreen` variant of `WindowMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] MonitorSelection
---@field  [any] 'never'




---@class (exact) WindowMode.Fullscreen.Fields
--- The fields of the `Fullscreen` variant of `WindowMode`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] MonitorSelection
---@field  [2] VideoModeSelection
---@field  [any] 'never'




---@class (exact) WindowPosition.At.Fields
--- The fields of the `At` variant of `WindowPosition`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1]? IVec2
---@field  [any] 'never'




---@class (exact) WindowPosition.Centered.Fields
--- The fields of the `Centered` variant of `WindowPosition`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] MonitorSelection
---@field  [any] 'never'




---@class (exact) WindowRef.Entity.Fields
--- The fields of the `Entity` variant of `WindowRef`, in order, i.e. `{ a, b }`. Optional fields are filled in from defaults.
---@field  [1] Entity
---@field  [any] 'never'




---@class (exact) WindowResizeConstraints.Fields
--- The fields of `WindowResizeConstraints`, keyed by name. Optional fields are filled in from defaults.
---@field  min_width? number
---@field  min_height? number
---@field  max_width? number
---@field  max_height? number
---@field  [any] 'never'




---@class (exact) Affine2.Fields
--- The fields of `Affine2`, keyed by name. Optional fields are filled in from defaults.
---@field  matrix2? Mat2
---@field  translation? Vec2
---@field  [any] 'never'




---@class (exact) Affine3A.Fields
--- The fields of `Affine3A`, keyed by name. Optional fields are filled in from defaults.
---@field  matrix3? Mat3A
---@field  translation? Vec3A
---@field  [any] 'never'




---@class (exact) Mat2.Fields
--- The fields of `Mat2`, keyed by name. Optional fields are filled in from defaults.
---@field  x_axis? Vec2
---@field  y_axis? Vec2
---@field  [any] 'never'




---@class (exact) Mat3.Fields
--- The fields of `Mat3`, keyed by name. Optional fields are filled in from defaults.
---@field  x_axis? Vec3
---@field  y_axis? Vec3
---@field  z_axis? Vec3
---@field  [any] 'never'




---@class (exact) Mat3A.Fields
--- The fields of `Mat3A`, keyed by name. Optional fields are filled in from defaults.
---@field  x_axis? Vec3A
---@field  y_axis? Vec3A
---@field  z_axis? Vec3A
---@field  [any] 'never'




---@class (exact) Mat4.Fields
--- The fields of `Mat4`, keyed by name. Optional fields are filled in from defaults.
---@field  x_axis? Vec4
---@field  y_axis? Vec4
---@field  z_axis? Vec4
---@field  w_axis? Vec4
---@field  [any] 'never'




---@class (exact) Quat.Fields
--- The fields of `Quat`, keyed by name. Optional fields are filled in from defaults.
---@field  x? number
---@field  y? number
---@field  z? number
---@field  w? number
---@field  [any] 'never'




---@class (exact) NoFields
--- The fields of a type or enum variant without fields, i.e. `{}`.
---@field  [any] 'never'




---@class (exact) namespace.animation
--- The types in the `animation` namespace, as returned by `import_namespace("animation")`.
---@field  AnimationPlayer AnimationPlayer
---@field  AnimationGraph AnimationGraph
---@field  NodeIndex NodeIndex
---@field  ActiveAnimation ActiveAnimation
---@field  AnimationTargetId AnimationTargetId
---@field  RepeatAnimation RepeatAnimation




---@class (exact) namespace.audio
--- The types in the `audio` namespace, as returned by `import_namespace("audio")`.
---@field  PlaybackSettings PlaybackSettings
---@field  PlaybackMode PlaybackMode
---@field  SpatialScale SpatialScale
---@field  Volume Volume




---@class (exact) namespace.camera
--- The types in the `camera` namespace, as returned by `import_namespace("camera")`.
---@field  Camera Camera
---@field  OrthographicProjection OrthographicProjection
---@field  InheritedVisibility InheritedVisibility
---@field  ViewVisibility ViewVisibility
---@field  Visibility Visibility
---@field  RenderLayers RenderLayers
---@field  CameraOutputMode CameraOutputMode
---@field  ComputedCameraValues ComputedCameraValues
---@field  ImageRenderTarget ImageRenderTarget
---@field  ManualTextureViewHandle ManualTextureViewHandle
---@field  RenderTarget RenderTarget
---@field  RenderTargetInfo RenderTargetInfo
---@field  SubCameraView SubCameraView
---@field  Viewport Viewport
---@field  ClearColorConfig ClearColorConfig
---@field  MsaaWriteback MsaaWriteback
---@field  ScalingMode ScalingMode




---@class (exact) namespace.color
--- The types in the `color` namespace, as returned by `import_namespace("color")`.
---@field  Color Color
---@field  Hsla Hsla
---@field  Hsva Hsva
---@field  Hwba Hwba
---@field  Laba Laba
---@field  Lcha Lcha
---@field  LinearRgba LinearRgba
---@field  Oklaba Oklaba
---@field  Oklcha Oklcha
---@field  Srgba Srgba
---@field  Xyza Xyza
---@field  Okhsla Okhsla
---@field  Okhsva Okhsva
---@field  Okhwba Okhwba




---@class (exact) namespace.math
--- The types in the `math` namespace, as returned by `import_namespace("math")`.
---@field  EaseFunction EaseFunction
---@field  Dir2 Dir2
---@field  Dir3 Dir3
---@field  Rect Rect
---@field  Rot2 Rot2
---@field  Annulus Annulus
---@field  Circle Circle
---@field  Rectangle Rectangle
---@field  RegularPolygon RegularPolygon
---@field  Capsule3d Capsule3d
---@field  Cuboid Cuboid
---@field  Cylinder Cylinder
---@field  Plane3d Plane3d
---@field  Sphere Sphere
---@field  Torus Torus
---@field  Ray2d Ray2d
---@field  Ray3d Ray3d
---@field  DVec2 DVec2
---@field  DVec3 DVec3
---@field  IVec2 IVec2
---@field  IVec3 IVec3
---@field  Mat3 Mat3
---@field  Mat4 Mat4
---@field  Quat Quat
---@field  UVec2 UVec2
---@field  UVec3 UVec3
---@field  Vec2 Vec2
---@field  Vec3 Vec3
---@field  Vec3A Vec3A
---@field  Vec4 Vec4
---@field  CompassOctant CompassOctant
---@field  URect URect
---@field  Affine2 Affine2
---@field  Affine3A Affine3A
---@field  EulerRot EulerRot
---@field  Mat2 Mat2
---@field  Mat3A Mat3A




---@class (exact) namespace.ecs
--- The types in the `ecs` namespace, as returned by `import_namespace("ecs")`.
---@field  Entity Entity
---@field  ChildOf ChildOf
---@field  Name Name
---@field  ComponentId ComponentId
---@field  AddEvent AddEvent
---@field  DespawnEvent DespawnEvent
---@field  DiscardEvent DiscardEvent
---@field  InsertEvent InsertEvent
---@field  RemoveEvent RemoveEvent
---@field  HashedStr HashedStr




---@class (exact) namespace.render
--- The types in the `render` namespace, as returned by `import_namespace("render")`.
---@field  Image Image
---@field  Mesh Mesh
---@field  StandardMaterial StandardMaterial
---@field  NoBackgroundMotionVectors NoBackgroundMotionVectors
---@field  TextureAtlas TextureAtlas
---@field  Indices Indices
---@field  MeshAttributeCompressionFlags MeshAttributeCompressionFlags
---@field  UvChannel UvChannel
---@field  MorphAttributes MorphAttributes
---@field  JointAabb JointAabb
---@field  JointIndex JointIndex
---@field  SkinnedMeshBounds SkinnedMeshBounds
---@field  ParallaxMappingMethod ParallaxMappingMethod
---@field  ReadbackOnce ReadbackOnce
---@field  ShaderBuffer ShaderBuffer
---@field  Screenshot Screenshot
---@field  ScreenshotCaptured ScreenshotCaptured




---@class (exact) namespace.sprite
--- The types in the `sprite` namespace, as returned by `import_namespace("sprite")`.
---@field  TextureAtlasLayout TextureAtlasLayout
---@field  Anchor Anchor
---@field  Sprite Sprite
---@field  SpriteAlphaMode SpriteAlphaMode
---@field  SpriteImageMode SpriteImageMode
---@field  SpriteScalingMode SpriteScalingMode
---@field  BorderRect BorderRect
---@field  SliceScaleMode SliceScaleMode
---@field  TextureSlicer TextureSlicer
---@field  ColorMaterial ColorMaterial
---@field  AlphaMode2d AlphaMode2d
---@field  SpriteMeshMaterial SpriteMeshMaterial




---@class (exact) namespace.input
--- The types in the `input` namespace, as returned by `import_namespace("input")`.
---@field  Gamepad Gamepad
---@field  KeyCode KeyCode
---@field  MouseButton MouseButton
---@field  InputFocus InputFocus
---@field  GamepadAxis GamepadAxis
---@field  GamepadButton GamepadButton
---@field  GamepadInput GamepadInput
---@field  NativeKeyCode NativeKeyCode
---@field  FocusCause FocusCause




---@class (exact) namespace.gizmos
--- The types in the `gizmos` namespace, as returned by `import_namespace("gizmos")`.
---@field  Gizmos Gizmos




---@class (exact) namespace.bms
--- The types in the `bms` namespace, as returned by `import_namespace("bms")`.
---@field  EnumVariantRef EnumVariantRef
---@field  ScriptBundleRegistration ScriptBundleRegistration
---@field  ScriptComponentRegistration ScriptComponentRegistration
---@field  ScriptQueryBuilder ScriptQueryBuilder
---@field  ScriptQueryResult ScriptQueryResult
---@field  ScriptResourceRegistration ScriptResourceRegistration
---@field  ScriptTypeRegistration ScriptTypeRegistration
---@field  ScriptEventRegistration ScriptEventRegistration
---@field  ScriptSystemBuilder ScriptSystemBuilder
---@field  Recipients Recipients
---@field  ScriptAttachment ScriptAttachment
---@field  ReflectSchedule ReflectSchedule
---@field  ReflectSystem ReflectSystem
---@field  ScriptValue ScriptValue
---@field  Language Language
---@field  ScriptAsset ScriptAsset
---@field  FunctionArgInfo FunctionArgInfo
---@field  FunctionInfo FunctionInfo
---@field  FunctionReturnInfo FunctionReturnInfo
---@field  InteropError InteropError
---@field  Namespace Namespace
---@field  DynamicComponent DynamicComponent
---@field  VariadicTuple VariadicTuple
---@field  StaticTypeRef StaticTypeRef
---@field  CallbackLabel CallbackLabel
---@field  ScriptAssetModifiedMessage ScriptAssetModifiedMessage
---@field  ScriptAttachedMessage ScriptAttachedMessage
---@field  ScriptCallbackMessage ScriptCallbackMessage
---@field  ScriptDetachedMessage ScriptDetachedMessage
---@field  ScriptComponent ScriptComponent
---@field  ReflectableScheduleLabel ReflectableScheduleLabel




---@class (exact) namespace.time
--- The types in the `time` namespace, as returned by `import_namespace("time")`.
---@field  Instant Instant
---@field  Stopwatch Stopwatch
---@field  Timer Timer
---@field  Duration Duration
---@field  Fixed Fixed
---@field  Real Real
---@field  TimerMode TimerMode
---@field  Virtual Virtual




---@class (exact) namespace.text
--- The types in the `text` namespace, as returned by `import_namespace("text")`.
---@field  Text2d Text2d
---@field  TextFont TextFont
---@field  TextLayout TextLayout
---@field  TextSpan TextSpan
---@field  Text Text
---@field  EmSize EmSize
---@field  FontFeatureTag FontFeatureTag
---@field  FontFeatures FontFeatures
---@field  FontSize FontSize
---@field  FontSmoothing FontSmoothing
---@field  FontSource FontSource
---@field  FontStyle FontStyle
---@field  FontVariationTag FontVariationTag
---@field  FontVariations FontVariations
---@field  FontWeight FontWeight
---@field  FontWidth FontWidth
---@field  GenericFontFamily GenericFontFamily
---@field  Justify Justify
---@field  LineBreak LineBreak
---@field  RemSize RemSize




---@class (exact) namespace.transform
--- The types in the `transform` namespace, as returned by `import_namespace("transform")`.
---@field  GlobalTransform GlobalTransform
---@field  Transform Transform




---@class (exact) namespace.ui
--- The types in the `ui` namespace, as returned by `import_namespace("ui")`.
---@field  UiRect UiRect
---@field  Val Val
---@field  BorderColor BorderColor
---@field  BorderRadius BorderRadius
---@field  ComputedNode ComputedNode
---@field  Overflow Overflow
---@field  ImageNode ImageNode
---@field  CornerRadius CornerRadius
---@field  AlignContent AlignContent
---@field  AlignItems AlignItems
---@field  AlignSelf AlignSelf
---@field  BackgroundColor BackgroundColor
---@field  BoxSizing BoxSizing
---@field  Display Display
---@field  FlexDirection FlexDirection
---@field  FlexWrap FlexWrap
---@field  GlobalZIndex GlobalZIndex
---@field  GridAutoFlow GridAutoFlow
---@field  GridPlacement GridPlacement
---@field  GridTrack GridTrack
---@field  GridTrackRepetition GridTrackRepetition
---@field  InlineDirection InlineDirection
---@field  JustifyContent JustifyContent
---@field  JustifyItems JustifyItems
---@field  JustifySelf JustifySelf
---@field  MaxTrackSizingFunction MaxTrackSizingFunction
---@field  MinTrackSizingFunction MinTrackSizingFunction
---@field  Node Node
---@field  Outline Outline
---@field  OverflowAxis OverflowAxis
---@field  OverflowClipMargin OverflowClipMargin
---@field  PositionType PositionType
---@field  RepeatedGridTrack RepeatedGridTrack
---@field  ResolvedBorderRadius ResolvedBorderRadius
---@field  VisualBox VisualBox
---@field  ZIndex ZIndex
---@field  NodeImageMode NodeImageMode




---@class (exact) namespace.window
--- The types in the `window` namespace, as returned by `import_namespace("window")`.
---@field  Window Window
---@field  WindowResolution WindowResolution
---@field  VideoMode VideoMode
---@field  CompositeAlphaMode CompositeAlphaMode
---@field  EnabledButtons EnabledButtons
---@field  InternalWindowState InternalWindowState
---@field  MonitorSelection MonitorSelection
---@field  PresentMode PresentMode
---@field  ScreenEdge ScreenEdge
---@field  VideoModeSelection VideoModeSelection
---@field  WindowLevel WindowLevel
---@field  WindowMode WindowMode
---@field  WindowPosition WindowPosition
---@field  WindowRef WindowRef
---@field  WindowResizeConstraints WindowResizeConstraints
---@field  WindowTheme WindowTheme




---@class (exact) namespace.asset
--- The types in the `asset` namespace, as returned by `import_namespace("asset")`.
---@field  AssetIndex AssetIndex
---@field  AssetPath AssetPath
---@field  RenderAssetUsages RenderAssetUsages




---@class (exact) namespace.bevy_curve
--- The types in the `bevy_curve` namespace, as returned by `import_namespace("bevy_curve")`.
---@field  JumpAt JumpAt




---@class (exact) namespace.bevy_material
--- The types in the `bevy_material` namespace, as returned by `import_namespace("bevy_material")`.
---@field  AlphaMode AlphaMode
---@field  OpaqueRendererMethod OpaqueRendererMethod




---@class (exact) namespace.bevy_shape
--- The types in the `bevy_shape` namespace, as returned by `import_namespace("bevy_shape")`.
---@field  Aabb2d Aabb2d
---@field  Aabb3d Aabb3d




---@class (exact) namespace.core
--- The types in the `core` namespace, as returned by `import_namespace("core")`.
---@field  TypeId TypeId
---@field  NonZeroI16 NonZeroI16
---@field  NonZeroU16 NonZeroU16
---@field  NonZeroU32 NonZeroU32




---@class (exact) namespace.smol_str
--- The types in the `smol_str` namespace, as returned by `import_namespace("smol_str")`.
---@field  SmolStr SmolStr




---@class (exact) namespace.uuid
--- The types in the `uuid` namespace, as returned by `import_namespace("uuid")`.
---@field  Uuid Uuid




---@class (exact) namespace.wgpu_types
--- The types in the `wgpu_types` namespace, as returned by `import_namespace("wgpu_types")`.
---@field  BlendState BlendState




---@class (exact) namespace.alloc
--- The types in the `alloc` namespace, as returned by `import_namespace("alloc")`.
---@field  Cow Cow





---@param p1 integer
---@return ReflectReference
function _entity_from_index(p1) end

---@return ReflectReference
function _make_invalid_entity() end

---@return ReflectReference
function _make_placeholder_entity() end

---@param p1 string
---@param p2 string
---@param p3? string | nil
---@return nil
function assert_str_eq(p1,p2,p3) end

---@param registration ScriptTypeRegistration | ScriptComponentRegistration | ScriptResourceRegistration | ScriptBundleRegistration
--- The type to construct.
---@param payload table<string, any>
--- The values to use to construct the type, keyed by field name or index, or a list of the fields in order.
---@return ReflectReference
function construct(registration,payload) end

---@param p1 integer
---@param p2 string
---@return ReflectReference
function create_test_asset(p1,p2) end

---@param namespace string
--- the script namespace to export into, it must start with `script.`.
---@param name string
--- the name of the function within the namespace.
---@param callback string
--- the label of the callback of this script to export.
---@return nil
function export_namespace(namespace,name,callback) end

---@return string
function global_hello_world() end

---@param path string
--- the name of the namespace to import.
---@overload fun(path: "animation"): namespace.animation
---@overload fun(path: "audio"): namespace.audio
---@overload fun(path: "camera"): namespace.camera
---@overload fun(path: "color"): namespace.color
---@overload fun(path: "math"): namespace.math
---@overload fun(path: "ecs"): namespace.ecs
---@overload fun(path: "render"): namespace.render
---@overload fun(path: "sprite"): namespace.sprite
---@overload fun(path: "input"): namespace.input
---@overload fun(path: "gizmos"): namespace.gizmos
---@overload fun(path: "bms"): namespace.bms
---@overload fun(path: "time"): namespace.time
---@overload fun(path: "text"): namespace.text
---@overload fun(path: "transform"): namespace.transform
---@overload fun(path: "ui"): namespace.ui
---@overload fun(path: "window"): namespace.window
---@overload fun(path: "asset"): namespace.asset
---@overload fun(path: "bevy_curve"): namespace.bevy_curve
---@overload fun(path: "bevy_material"): namespace.bevy_material
---@overload fun(path: "bevy_shape"): namespace.bevy_shape
---@overload fun(path: "core"): namespace.core
---@overload fun(path: "smol_str"): namespace.smol_str
---@overload fun(path: "uuid"): namespace.uuid
---@overload fun(path: "wgpu_types"): namespace.wgpu_types
---@overload fun(path: "alloc"): namespace.alloc
---@return any
function import_namespace(path) end

---@param p1 function
---@return function
function into_script_function(p1) end

---@param message string
---@return nil
function log_debug(message) end

---@param message string
---@return nil
function log_error(message) end

---@param message string
--- the message to log
---@return nil
function log_info(message) end

---@param message string
---@return nil
function log_trace(message) end

---@param message string
---@return nil
function log_warn(message) end

---@param p1 table<string, integer>
---@return table<string, integer>
function make_hashmap(p1) end

---@param p1 Entity[]
---@return Entity[]
function make_vec_entity(p1) end

---@param p1 integer[]
---@return integer[]
function make_vec_i64(p1) end

---@param p1 string[]
---@return string[]
function make_vec_string(p1) end

---@return nil
function noop() end

---@param p1 any
---@param p2 any
---@param p3 any
---@param p4 any
---@return nil
function noop_4_args(p1,p2,p3,p4) end

---@param tuple ... any
--- The tuple of values to curry
---@return any[]
function pack_args(... tuple) end

---@param label string
--- The name of the callback to run.
---@param target ScriptAttachment | Recipients
--- The script attachment or recipients to run the callback on.
---@param args ... any
--- The arguments to pass to the callback.
---@return nil
function queue_script_callback(label,target,... args) end

---@param p1? integer | nil
---@param p2? integer | nil
---@return integer
function random(p1,p2) end

---@param p1? integer | nil
---@param p2? integer | nil
---@return integer
function random_int(p1,p2) end

---@param callback string
--- the callback label to register this function against.
---@param _function function
--- the callback function which will be stored as a handler for this callback label.
---@return nil
function register_callback(callback,_function) end

---@return nil
function reseed() end

---@param label string
--- The name of the callback to run.
---@param target ScriptAttachment | Recipients
--- The script attachment or recipients to run the callback on.
---@param args ... any
--- The arguments to pass to the callback.
---@return any
function run_script_callback(label,target,... args) end

---@param callback string
--- The function name in the script this system should call when run.
---@param attachment ScriptAttachment
--- The script attachment to use for the system. This is the attachment that will be used for the system's callback.
---@return ScriptSystemBuilder
function system_builder(callback,attachment) end

---@param values any[]
--- The list of values to uncurry
---@return any ...
function unpack_args(values) end


---@type World
--- An global instance of this type
world = {}

---@type Handle<ScriptAsset>
--- An global instance of this type
script_asset = {}

---@type Entity
--- An global instance of this type
entity = {}

