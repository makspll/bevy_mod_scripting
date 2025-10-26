---@meta
---@module "World"

---@class World
--- The ECS world containing all Components, Resources and Systems. Main point of interaction with a Bevy App.


---@class ScriptComponentRegistration
---  A reference to a component type's reflection registration.--- ---  In general think of this as a handle to a type.--- ---  Not to be confused with script registered dynamic components, although this can point to a script registered component.
---@field  registration ? ScriptTypeRegistration
---@field  component_id ? ComponentId
---@field  is_dynamic_script_component ? boolean


---@class ScriptQueryBuilder
---  The query builder is used to build ECS queries which retrieve spefific components filtered by specific conditions.--- ---  For example:---  ```rust,ignore---  builder.component(componentA)---      .component(componentB)---      .with(componentC)---      .without(componentD)  ---  ```--- ---  Will retrieve entities which:---  - Have componentA---  - Have componentB---  - Have componentC---  - Do not have componentD--- ---  As well as references to components:---  - componentA---  - componentB


---@class ScriptQueryResult
---  A result from a query.


---@class ScriptResourceRegistration
---  A reference to a resource type's reflection registration.--- ---  In general think of this as a handle to a type.
---@field  registration ? ScriptTypeRegistration
---@field  resource_id ? ComponentId


---@class ScriptTypeRegistration
---  A reference to a type which is not a `Resource` or `Component`.--- ---  In general think of this as a handle to a type.


---@class ScriptSystemBuilder
---  A builder for systems living in scripts


---@class ScriptAttachment
---  Specifies a unique attachment of a script. These attachments are mapped to [`ContextKey`]'s depending on the context policy used.


---@class ReflectSchedule
---  A reflectable schedule.
---@field  type_path ? string
---@field  label ? ReflectableScheduleLabel

---@class ReflectSystem
---  A reflectable system.


---@class Color
---  An enumerated type that can represent any of the color types in this crate.--- ---  This is useful when you need to store a color in a data structure that can't be generic over---  the color type.---  <div>---  </div>--- ---  # Operations--- ---  [`Color`] supports all the standard color operations, such as [mixing](Mix),---  [luminance](Luminance) and [hue](Hue) adjustment,---  and [diffing](EuclideanDistance). These operations delegate to the concrete color space contained---  by [`Color`], but will convert to [`Oklch`](Oklcha) for operations which aren't supported in the---  current space. After performing the operation, if a conversion was required, the result will be---  converted back into the original color space.--- ---  ```rust---  # use bevy_color::{Hue, Color};---  let red_hsv = Color::hsv(0., 1., 1.);---  let red_srgb = Color::srgb(1., 0., 0.);--- ---  // HSV has a definition of hue, so it will be returned.---  red_hsv.hue();--- ---  // SRGB doesn't have a native definition for hue.---  // Converts to Oklch and returns that result.---  red_srgb.hue();---  ```--- ---  [`Oklch`](Oklcha) has been chosen as the intermediary space in cases where conversion is required---  due to its perceptual uniformity and broad support for Bevy's color operations.---  To avoid the cost of repeated conversion, and ensure consistent results where that is desired,---  first convert this [`Color`] into your desired color space.


---@class Hsla
---  Color in Hue-Saturation-Lightness (HSL) color space with alpha.---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).---  <div>---  </div>
---@field  hue ? number
---@field  saturation ? number
---@field  lightness ? number
---@field  alpha ? number


---@class Hsva
---  Color in Hue-Saturation-Value (HSV) color space with alpha.---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).---  <div>---  </div>
---@field  hue ? number
---@field  saturation ? number
---@field  value ? number
---@field  alpha ? number


---@class Hwba
---  Color in Hue-Whiteness-Blackness (HWB) color space with alpha.---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).---  <div>---  </div>
---@field  hue ? number
---@field  whiteness ? number
---@field  blackness ? number
---@field  alpha ? number


---@class Laba
---  Color in LAB color space, with alpha---  <div>---  </div>
---@field  lightness ? number
---@field  a ? number
---@field  b ? number
---@field  alpha ? number


---@class Lcha
---  Color in LCH color space, with alpha---  <div>---  </div>
---@field  lightness ? number
---@field  chroma ? number
---@field  hue ? number
---@field  alpha ? number


---@class LinearRgba
---  Linear RGB color with alpha.---  <div>---  </div>
---@field  red ? number
---@field  green ? number
---@field  blue ? number
---@field  alpha ? number


---@class Oklaba
---  Color in Oklab color space, with alpha---  <div>---  </div>
---@field  lightness ? number
---@field  a ? number
---@field  b ? number
---@field  alpha ? number


---@class Oklcha
---  Color in Oklch color space, with alpha---  <div>---  </div>
---@field  lightness ? number
---@field  chroma ? number
---@field  hue ? number
---@field  alpha ? number


---@class Srgba
---  Non-linear standard RGB with alpha.---  <div>---  </div>
---@field  red ? number
---@field  green ? number
---@field  blue ? number
---@field  alpha ? number


---@class Xyza
---  [CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.---  <div>---  </div>
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  alpha ? number


---@class AutoExposureCompensationCurve
---  An auto exposure compensation curve.---  This curve is used to map the average log luminance of a scene to an---  exposure compensation value, to allow for fine control over the final exposure.
---@field  min_log_lum ? number
---@field  max_log_lum ? number
---@field  min_compensation ? number
---@field  max_compensation ? number
---@field  lut ? [u8; 256]


---@class AutoExposure
---  Component that enables auto exposure for an HDR-enabled 2d or 3d camera.--- ---  Auto exposure adjusts the exposure of the camera automatically to---  simulate the human eye's ability to adapt to different lighting conditions.--- ---  Bevy's implementation builds a 64 bin histogram of the scene's luminance,---  and then adjusts the exposure so that the average brightness of the final---  render will be middle gray. Because it's using a histogram, some details can---  be selectively ignored or emphasized. Outliers like shadows and specular---  highlights can be ignored, and certain areas can be given more (or less)---  weight based on a mask.--- ---  # Usage Notes--- ---  **Auto Exposure requires compute shaders and is not compatible with WebGL2.**
---@field  range ? RangeInclusive
---@field  filter ? RangeInclusive
---@field  speed_brighten ? number
---@field  speed_darken ? number
---@field  exponential_transition_distance ? number
---@field  metering_mask ? Handle
---@field  compensation_curve ? Handle


---@class Bloom
---  Applies a bloom effect to an HDR-enabled 2d or 3d camera.--- ---  Bloom emulates an effect found in real cameras and the human eye,---  causing halos to appear around very bright parts of the scene.--- ---  See also <https://en.wikipedia.org/wiki/Bloom_(shader_effect)>.--- ---  # Usage Notes--- ---  **Bloom is currently not compatible with WebGL2.**--- ---  Often used in conjunction with `bevy_pbr::StandardMaterial::emissive` for 3d meshes.--- ---  Bloom is best used alongside a tonemapping function that desaturates bright colors,---  such as [`crate::tonemapping::Tonemapping::TonyMcMapface`].--- ---  Bevy's implementation uses a parametric curve to blend between a set of---  blurred (lower frequency) images generated from the camera's view.---  See <https://starlederer.github.io/bloom/> for a visualization of the parametric curve---  used in Bevy as well as a visualization of the curve's respective scattering profile.
---@field  intensity ? number
---@field  low_frequency_boost ? number
---@field  low_frequency_boost_curvature ? number
---@field  high_pass_frequency ? number
---@field  prefilter ? BloomPrefilter
---@field  composite_mode ? BloomCompositeMode
---@field  max_mip_dimension ? integer
---@field  scale ? Vec2


---@class BloomCompositeMode



---@class BloomPrefilter
---  Applies a threshold filter to the input image to extract the brightest---  regions before blurring them and compositing back onto the original image.---  These settings are useful when emulating the 1990s-2000s game look.--- ---  # Considerations---  * Changing these settings creates a physically inaccurate image---  * Changing these settings makes it easy to make the final result look worse---  * Non-default prefilter settings should be used in conjunction with [`BloomCompositeMode::Additive`]
---@field  threshold ? number
---@field  threshold_softness ? number


---@class ContrastAdaptiveSharpening
---  Applies a contrast adaptive sharpening (CAS) filter to the camera.--- ---  CAS is usually used in combination with shader based anti-aliasing methods---  such as FXAA or TAA to regain some of the lost detail from the blurring that they introduce.--- ---  CAS is designed to adjust the amount of sharpening applied to different areas of an image---  based on the local contrast. This can help avoid over-sharpening areas with high contrast---  and under-sharpening areas with low contrast.--- ---  To use this, add the [`ContrastAdaptiveSharpening`] component to a 2D or 3D camera.
---@field  enabled ? boolean
---@field  sharpening_strength ? number
---@field  denoise ? boolean


---@class DenoiseCas

---@field  [1] ? boolean


---@class Camera2d
---  A 2D camera component. Enables the 2D render graph for a [`Camera`].


---@class Camera3d
---  A 3D camera component. Enables the main 3D render graph for a [`Camera`].--- ---  The camera coordinate space is right-handed X-right, Y-up, Z-back.---  This means "forward" is -Z.
---@field  depth_load_op ? Camera3dDepthLoadOp
---@field  depth_texture_usages ? Camera3dDepthTextureUsage
---@field  screen_space_specular_transmission_steps ? integer
---@field  screen_space_specular_transmission_quality ? ScreenSpaceTransmissionQuality


---@class Camera3dDepthLoadOp
---  The depth clear operation to perform for the main 3d pass.


---@class Camera3dDepthTextureUsage

---@field  [1] ? integer


---@class ScreenSpaceTransmissionQuality
---  The quality of the screen space transmission blur effect, applied to whatever's “behind” transmissive---  objects when their `roughness` is greater than `0.0`.--- ---  Higher qualities are more GPU-intensive.--- ---  **Note:** You can get better-looking results at any quality level by enabling TAA. See: [`TemporalAntiAliasPlugin`](crate::experimental::taa::TemporalAntiAliasPlugin).


---@class DepthOfField
---  A component that enables a [depth of field] postprocessing effect when attached to a [`Camera3d`],---  simulating the focus of a camera lens.--- ---  [depth of field]: https://en.wikipedia.org/wiki/Depth_of_field
---@field  mode ? DepthOfFieldMode
---@field  focal_distance ? number
---@field  sensor_height ? number
---@field  aperture_f_stops ? number
---@field  max_circle_of_confusion_diameter ? number
---@field  max_depth ? number


---@class DepthOfFieldMode
---  Controls the appearance of the effect.


---@class Fxaa
---  A component for enabling Fast Approximate Anti-Aliasing (FXAA)---  for a [`bevy_render::camera::Camera`].
---@field  enabled ? boolean
---@field  edge_threshold ? Sensitivity
---@field  edge_threshold_min ? Sensitivity


---@class Sensitivity



---@class MotionBlur
---  A component that enables and configures motion blur when added to a camera.--- ---  Motion blur is an effect that simulates how moving objects blur as they change position during---  the exposure of film, a sensor, or an eyeball.--- ---  Because rendering simulates discrete steps in time, we use per-pixel motion vectors to estimate---  the path of objects between frames. This kind of implementation has some artifacts:---  - Fast moving objects in front of a stationary object or when in front of empty space, will not---    have their edges blurred.---  - Transparent objects do not write to depth or motion vectors, so they cannot be blurred.--- ---  Other approaches, such as *A Reconstruction Filter for Plausible Motion Blur* produce more---  correct results, but are more expensive and complex, and have other kinds of artifacts. This---  implementation is relatively inexpensive and effective.--- ---  # Usage--- ---  Add the [`MotionBlur`] component to a camera to enable and configure motion blur for that---  camera.--- ---  ```---  # use bevy_core_pipeline::{core_3d::Camera3d, motion_blur::MotionBlur};---  # use bevy_ecs::prelude::*;---  # fn test(mut commands: Commands) {---  commands.spawn((---      Camera3d::default(),---      MotionBlur::default(),---  ));---  # }---  ````
---@field  shutter_angle ? number
---@field  samples ? integer


---@class OrderIndependentTransparencySettings
---  Used to identify which camera will use OIT to render transparent meshes---  and to configure OIT.
---@field  layer_count ? integer
---@field  alpha_threshold ? number


---@class ChromaticAberration
---  Adds colored fringes to the edges of objects in the scene.--- ---  [Chromatic aberration] simulates the effect when lenses fail to focus all---  colors of light toward a single point. It causes rainbow-colored streaks to---  appear, which are especially apparent on the edges of objects. Chromatic---  aberration is commonly used for collision effects, especially in horror---  games.--- ---  Bevy's implementation is based on that of *Inside* ([Gjøl & Svendsen 2016]).---  It's based on a customizable lookup texture, which allows for changing the---  color pattern. By default, the color pattern is simply a 3×1 pixel texture---  consisting of red, green, and blue, in that order, but you can change it to---  any image in order to achieve different effects.--- ---  [Chromatic aberration]: https://en.wikipedia.org/wiki/Chromatic_aberration--- ---  [Gjøl & Svendsen 2016]: https://github.com/playdeadgames/publications/blob/master/INSIDE/rendering_inside_gdc2016.pdf
---@field  color_lut ? Handle
---@field  intensity ? number
---@field  max_samples ? integer


---@class DepthPrepass
---  If added to a [`crate::prelude::Camera3d`] then depth values will be copied to a separate texture available to the main pass.


---@class MotionVectorPrepass
---  If added to a [`crate::prelude::Camera3d`] then screen space motion vectors will be copied to a separate texture available to the main pass.


---@class NormalPrepass
---  If added to a [`crate::prelude::Camera3d`] then vertex world normals will be copied to a separate texture available to the main pass.---  Normals will have normal map textures already applied.


---@class Skybox
---  Adds a skybox to a 3D camera, based on a cubemap texture.--- ---  Note that this component does not (currently) affect the scene's lighting.---  To do so, use `EnvironmentMapLight` alongside this component.--- ---  See also <https://en.wikipedia.org/wiki/Skybox_(video_games)>.
---@field  image ? Handle
---@field  brightness ? number
---@field  rotation ? Quat


---@class Smaa
---  A component for enabling Subpixel Morphological Anti-Aliasing (SMAA)---  for a [`bevy_render::camera::Camera`].
---@field  preset ? SmaaPreset


---@class SmaaPreset
---  A preset quality level for SMAA.--- ---  Higher values are slower but result in a higher-quality image.--- ---  The default value is *high*.


---@class TemporalAntiAliasing
---  Component to apply temporal anti-aliasing to a 3D perspective camera.--- ---  Temporal anti-aliasing (TAA) is a form of image smoothing/filtering, like---  multisample anti-aliasing (MSAA), or fast approximate anti-aliasing (FXAA).---  TAA works by blending (averaging) each frame with the past few frames.--- ---  # Tradeoffs--- ---  Pros:---  * Filters more types of aliasing than MSAA, such as textures and singular bright pixels (specular aliasing)---  * Cost scales with screen/view resolution, unlike MSAA which scales with number of triangles---  * Greatly increases the quality of stochastic rendering techniques such as SSAO, certain shadow map sampling methods, etc--- ---  Cons:---  * Chance of "ghosting" - ghostly trails left behind moving objects---  * Thin geometry, lighting detail, or texture lines may flicker noisily or disappear--- ---  Because TAA blends past frames with the current frame, when the frames differ too much---  (such as with fast moving objects or camera cuts), ghosting artifacts may occur.--- ---  Artifacts tend to be reduced at higher framerates and rendering resolution.--- ---  # Usage Notes--- ---  The [`TemporalAntiAliasPlugin`] must be added to your app.---  Any camera with this component must also disable [`Msaa`] by setting it to [`Msaa::Off`].--- ---  [Currently](https://github.com/bevyengine/bevy/issues/8423), TAA cannot be used with [`bevy_render::camera::OrthographicProjection`].--- ---  TAA also does not work well with alpha-blended meshes, as it requires depth writing to determine motion.--- ---  It is very important that correct motion vectors are written for everything on screen.---  Failure to do so will lead to ghosting artifacts. For instance, if particle effects---  are added using a third party library, the library must either:--- ---  1. Write particle motion vectors to the motion vectors prepass texture---  2. Render particles after TAA--- ---  If no [`MipBias`] component is attached to the camera, TAA will add a `MipBias(-1.0)` component.
---@field  reset ? boolean


---@class DebandDither
---  Enables a debanding shader that applies dithering to mitigate color banding in the final image for a given [`Camera`] entity.


---@class Tonemapping
---  Optionally enables a tonemapping shader that attempts to map linear input stimulus into a perceptually uniform image for a given [`Camera`] entity.


---@class ComponentId
---  A value which uniquely identifies the type of a [`Component`] or [`Resource`] within a---  [`World`].--- ---  Each time a new `Component` type is registered within a `World` using---  e.g. [`World::register_component`] or [`World::register_component_with_descriptor`]---  or a Resource with e.g. [`World::init_resource`],---  a corresponding `ComponentId` is created to track it.--- ---  While the distinction between `ComponentId` and [`TypeId`] may seem superficial, breaking them---  into two separate but related concepts allows components to exist outside of Rust's type system.---  Each Rust type registered as a `Component` will have a corresponding `ComponentId`, but additional---  `ComponentId`s may exist in a `World` to track components which cannot be---  represented as Rust types for scripting or other advanced use-cases.--- ---  A `ComponentId` is tightly coupled to its parent `World`. Attempting to use a `ComponentId` from---  one `World` to access the metadata of a `Component` in a different `World` is undefined behavior---  and must not be attempted.--- ---  Given a type `T` which implements [`Component`], the `ComponentId` for `T` can be retrieved---  from a `World` using [`World::component_id()`] or via [`Components::component_id()`]. Access---  to the `ComponentId` for a [`Resource`] is available via [`Components::resource_id()`].
---@field  [1] ? integer


---@class ComponentTicks
---  Records when a component or resource was added and when it was last mutably dereferenced (or added).
---@field  added ? Tick
---@field  changed ? Tick


---@class Tick
---  A value that tracks when a system ran relative to other systems.---  This is used to power change detection.--- ---  *Note* that a system that hasn't been run yet has a `Tick` of 0.
---@field  tick ? integer


---@class Entity
---  Lightweight identifier of an [entity](crate::entity).--- ---  The identifier is implemented using a [generational index]: a combination of an index and a generation.---  This allows fast insertion after data removal in an array while minimizing loss of spatial locality.--- ---  These identifiers are only valid on the [`World`] it's sourced from. Attempting to use an `Entity` to---  fetch entity components or metadata from a different world will either fail or return unexpected results.--- ---  [generational index]: https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594--- ---  # Stability warning---  For all intents and purposes, `Entity` should be treated as an opaque identifier. The internal bit---  representation is liable to change from release to release as are the behaviors or performance---  characteristics of any of its trait implementations (i.e. `Ord`, `Hash`, etc.). This means that changes in---  `Entity`'s representation, though made readable through various functions on the type, are not considered---  breaking changes under [SemVer].--- ---  In particular, directly serializing with `Serialize` and `Deserialize` make zero guarantee of long---  term wire format compatibility. Changes in behavior will cause serialized `Entity` values persisted---  to long term storage (i.e. disk, databases, etc.) will fail to deserialize upon being updated.--- ---  # Usage--- ---  This data type is returned by iterating a `Query` that has `Entity` as part of its query fetch type parameter ([learn more]).---  It can also be obtained by calling [`EntityCommands::id`] or [`EntityWorldMut::id`].--- ---  ```---  # use bevy_ecs::prelude::*;---  # #[derive(Component)]---  # struct SomeComponent;---  fn setup(mut commands: Commands) {---      // Calling `spawn` returns `EntityCommands`.---      let entity = commands.spawn(SomeComponent).id();---  }--- ---  fn exclusive_system(world: &mut World) {---      // Calling `spawn` returns `EntityWorldMut`.---      let entity = world.spawn(SomeComponent).id();---  }---  #---  # bevy_ecs::system::assert_is_system(setup);---  # bevy_ecs::system::assert_is_system(exclusive_system);---  ```--- ---  It can be used to refer to a specific entity to apply [`EntityCommands`], or to call [`Query::get`] (or similar methods) to access its components.--- ---  ```---  # use bevy_ecs::prelude::*;---  #---  # #[derive(Component)]---  # struct Expired;---  #---  fn dispose_expired_food(mut commands: Commands, query: Query<Entity, With<Expired>>) {---      for food_entity in &query {---          commands.entity(food_entity).despawn();---      }---  }---  #---  # bevy_ecs::system::assert_is_system(dispose_expired_food);---  ```--- ---  [learn more]: crate::system::Query#entity-id-access---  [`EntityCommands::id`]: crate::system::EntityCommands::id---  [`EntityWorldMut::id`]: crate::world::EntityWorldMut::id---  [`EntityCommands`]: crate::system::EntityCommands---  [`Query::get`]: crate::system::Query::get---  [`World`]: crate::world::World---  [SemVer]: https://semver.org/


---@class EntityHash
---  A [`BuildHasher`] that results in a [`EntityHasher`].


---@class EntityHashSet
---  A [`HashSet`] pre-configured to use [`EntityHash`] hashing.
---@field  [1] ? HashSet


---@class DefaultQueryFilters
---  Default query filters work by excluding entities with certain components from most queries.--- ---  If a query does not explicitly mention a given disabling component, it will not include entities with that component.---  To be more precise, this checks if the query's [`FilteredAccess`] contains the component,---  and if it does not, adds a [`Without`](crate::prelude::Without) filter for that component to the query.--- ---  This resource is initialized in the [`World`] whenever a new world is created,---  with the [`Disabled`] component as a disabling component.--- ---  Note that you can remove default query filters by overwriting the [`DefaultQueryFilters`] resource.---  This can be useful as a last resort escape hatch, but is liable to break compatibility with other libraries.--- ---  See the [module docs](crate::entity_disabling) for more info.--- --- ---  # Warning--- ---  Default query filters are a global setting that affects all queries in the [`World`],---  and incur a small performance cost for each query.--- ---  They can cause significant interoperability issues within the ecosystem,---  as users must be aware of each disabling component in use.--- ---  Think carefully about whether you need to use a new disabling component,---  and clearly communicate their presence in any libraries you publish.
---@field  disabling ? SmallVec


---@class Disabled
---  A marker component for disabled entities.--- ---  Semantically, this component is used to mark entities that are temporarily disabled (typically for gameplay reasons),---  but will likely be re-enabled at some point.--- ---  Like all disabling components, this only disables the entity itself,---  not its children or other entities that reference it.---  To disable an entire tree of entities, use [`EntityCommands::insert_recursive`](crate::prelude::EntityCommands::insert_recursive).--- ---  Every [`World`] has a default query filter that excludes entities with this component,---  registered in the [`DefaultQueryFilters`] resource.---  See [the module docs] for more info.--- ---  [the module docs]: crate::entity_disabling


---@class ChildOf
---  Stores the parent entity of this child entity with this component.--- ---  This is a [`Relationship`] component, and creates the canonical---  "parent / child" hierarchy. This is the "source of truth" component, and it pairs with---  the [`Children`] [`RelationshipTarget`](crate::relationship::RelationshipTarget).--- ---  This relationship should be used for things like:--- ---  1. Organizing entities in a scene---  2. Propagating configuration or data inherited from a parent, such as "visibility" or "world-space global transforms".---  3. Ensuring a hierarchy is despawned when an entity is despawned.--- ---  [`ChildOf`] contains a single "target" [`Entity`]. When [`ChildOf`] is inserted on a "source" entity,---  the "target" entity will automatically (and immediately, via a component hook) have a [`Children`]---  component inserted, and the "source" entity will be added to that [`Children`] instance.--- ---  If the [`ChildOf`] component is replaced with a different "target" entity, the old target's [`Children`]---  will be automatically (and immediately, via a component hook) be updated to reflect that change.--- ---  Likewise, when the [`ChildOf`] component is removed, the "source" entity will be removed from the old---  target's [`Children`]. If this results in [`Children`] being empty, [`Children`] will be automatically removed.--- ---  When a parent is despawned, all children (and their descendants) will _also_ be despawned.--- ---  You can create parent-child relationships in a variety of ways. The most direct way is to insert a [`ChildOf`] component:--- ---  ```---  # use bevy_ecs::prelude::*;---  # let mut world = World::new();---  let root = world.spawn_empty().id();---  let child1 = world.spawn(ChildOf(root)).id();---  let child2 = world.spawn(ChildOf(root)).id();---  let grandchild = world.spawn(ChildOf(child1)).id();--- ---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1, child2]);---  assert_eq!(&**world.entity(child1).get::<Children>().unwrap(), &[grandchild]);--- ---  world.entity_mut(child2).remove::<ChildOf>();---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1]);--- ---  world.entity_mut(root).despawn();---  assert!(world.get_entity(root).is_err());---  assert!(world.get_entity(child1).is_err());---  assert!(world.get_entity(grandchild).is_err());---  ```--- ---  However if you are spawning many children, you might want to use the [`EntityWorldMut::with_children`] helper instead:--- ---  ```---  # use bevy_ecs::prelude::*;---  # let mut world = World::new();---  let mut child1 = Entity::PLACEHOLDER;---  let mut child2 = Entity::PLACEHOLDER;---  let mut grandchild = Entity::PLACEHOLDER;---  let root = world.spawn_empty().with_children(|p| {---      child1 = p.spawn_empty().with_children(|p| {---          grandchild = p.spawn_empty().id();---      }).id();---      child2 = p.spawn_empty().id();---  }).id();--- ---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1, child2]);---  assert_eq!(&**world.entity(child1).get::<Children>().unwrap(), &[grandchild]);---  ```--- ---  [`Relationship`]: crate::relationship::Relationship
---@field  [1] ? Entity


---@class Children
---  Tracks which entities are children of this parent entity.--- ---  A [`RelationshipTarget`] collection component that is populated---  with entities that "target" this entity with the [`ChildOf`] [`Relationship`] component.--- ---  Together, these components form the "canonical parent-child hierarchy". See the [`ChildOf`] component for the full---  description of this relationship and instructions on how to use it.--- ---  # Usage--- ---  Like all [`RelationshipTarget`] components, this data should not be directly manipulated to avoid desynchronization.---  Instead, modify the [`ChildOf`] components on the "source" entities.--- ---  To access the children of an entity, you can iterate over the [`Children`] component,---  using the [`IntoIterator`] trait.---  For more complex access patterns, see the [`RelationshipTarget`] trait.--- ---  [`Relationship`]: crate::relationship::Relationship---  [`RelationshipTarget`]: crate::relationship::RelationshipTarget
---@field  [1] ? Vec


---@class Identifier
---  A unified identifier for all entity and similar IDs.--- ---  Has the same size as a `u64` integer, but the layout is split between a 32-bit low---  segment, a 31-bit high segment, and the significant bit reserved as type flags to denote---  entity kinds.


---@class Name
---  Component used to identify an entity. Stores a hash for faster comparisons.--- ---  The hash is eagerly re-computed upon each update to the name.--- ---  [`Name`] should not be treated as a globally unique identifier for entities,---  as multiple entities can have the same name.  [`Entity`] should be---  used instead as the default unique identifier.
---@field  hash ? integer
---@field  name ? Cow


---@class RemovedComponentEntity
---  Wrapper around [`Entity`] for [`RemovedComponents`].---  Internally, `RemovedComponents` uses these as an `Events<RemovedComponentEntity>`.
---@field  [1] ? Entity


---@class ButtonState
---  The current "press" state of an element


---@class AxisSettings
---  Settings for a [`GamepadAxis`].--- ---  It is used inside the [`GamepadSettings`] to define the sensitivity range and---  threshold for an axis.---  Values that are higher than `livezone_upperbound` will be rounded up to 1.0.---  Values that are lower than `livezone_lowerbound` will be rounded down to -1.0.---  Values that are in-between `deadzone_lowerbound` and `deadzone_upperbound` will be rounded to 0.0.---  Otherwise, values will be linearly rescaled to fit into the sensitivity range.---  For example, a value that is one fourth of the way from `deadzone_upperbound` to `livezone_upperbound` will be scaled to 0.25.--- ---  The valid range is `[-1.0, 1.0]`.
---@field  livezone_upperbound ? number
---@field  deadzone_upperbound ? number
---@field  deadzone_lowerbound ? number
---@field  livezone_lowerbound ? number
---@field  threshold ? number


---@class ButtonAxisSettings
---  Settings for a [`GamepadButton`].--- ---  It is used inside the [`GamepadSettings`] to define the sensitivity range and---  threshold for a button axis.--- ---  ## Logic--- ---  - Values that are higher than or equal to `high` will be rounded to 1.0.---  - Values that are lower than or equal to `low` will be rounded to 0.0.---  - Otherwise, values will not be rounded.--- ---  The valid range is from 0.0 to 1.0, inclusive.
---@field  high ? number
---@field  low ? number
---@field  threshold ? number


---@class ButtonSettings
---  Manages settings for gamepad buttons.--- ---  It is used inside [`GamepadSettings`] to define the threshold for a [`GamepadButton`]---  to be considered pressed or released. A button is considered pressed if the `press_threshold`---  value is surpassed and released if the `release_threshold` value is undercut.--- ---  Allowed values: `0.0 <= ``release_threshold`` <= ``press_threshold`` <= 1.0`
---@field  press_threshold ? number
---@field  release_threshold ? number


---@class Gamepad
---  Stores a connected gamepad's metadata such as the name and its [`GamepadButton`] and [`GamepadAxis`].--- ---  An entity with this component is spawned automatically after [`GamepadConnectionEvent`]---  and updated by [`gamepad_event_processing_system`].--- ---  See also [`GamepadSettings`] for configuration.--- ---  # Examples--- ---  ```---  # use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton};---  # use bevy_ecs::system::Query;---  # use bevy_ecs::name::Name;---  #---  fn gamepad_usage_system(gamepads: Query<(&Name, &Gamepad)>) {---      for (name, gamepad) in &gamepads {---          println!("{name}");--- ---          if gamepad.just_pressed(GamepadButton::North) {---              println!("{} just pressed North", name)---          }--- ---          if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)  {---              println!("left stick X: {}", left_stick_x)---          }---      }---  }---  ```
---@field  vendor_id ? Option
---@field  product_id ? Option
---@field  digital ? ButtonInput
---@field  analog ? Axis


---@class GamepadAxis
---  Represents gamepad input types that are mapped in the range [-1.0, 1.0].--- ---  ## Usage--- ---  This is used to determine which axis has changed its value when receiving a---  gamepad axis event. It is also used in the [`Gamepad`] component.


---@class GamepadAxisChangedEvent
---  [`GamepadAxis`] event triggered by an analog state change.
---@field  entity ? Entity
---@field  axis ? GamepadAxis
---@field  value ? number


---@class GamepadButton
---  Represents gamepad input types that are mapped in the range [0.0, 1.0].--- ---  ## Usage--- ---  This is used to determine which button has changed its value when receiving gamepad button events.---  It is also used in the [`Gamepad`] component.


---@class GamepadButtonChangedEvent
---  [`GamepadButton`] event triggered by an analog state change.
---@field  entity ? Entity
---@field  button ? GamepadButton
---@field  state ? ButtonState
---@field  value ? number


---@class GamepadButtonStateChangedEvent
---  [`GamepadButton`] event triggered by a digital state change.
---@field  entity ? Entity
---@field  button ? GamepadButton
---@field  state ? ButtonState


---@class GamepadConnection
---  The connection status of a gamepad.


---@class GamepadConnectionEvent
---  A Gamepad connection event. Created when a connection to a gamepad---  is established and when a gamepad is disconnected.
---@field  gamepad ? Entity
---@field  connection ? GamepadConnection


---@class GamepadEvent
---  A gamepad event.--- ---  This event type is used over the [`GamepadConnectionEvent`],---  [`GamepadButtonChangedEvent`] and [`GamepadAxisChangedEvent`] when---  the in-frame relative ordering of events is important.--- ---  This event is produced by `bevy_input`.


---@class GamepadInput
---  Encapsulation over [`GamepadAxis`] and [`GamepadButton`].


---@class GamepadRumbleIntensity
---  The intensity at which a gamepad's force-feedback motors may rumble.
---@field  strong_motor ? number
---@field  weak_motor ? number


---@class GamepadRumbleRequest
---  An event that controls force-feedback rumbling of a [`Gamepad`] [`entity`](Entity).--- ---  # Notes--- ---  Does nothing if the gamepad or platform does not support rumble.--- ---  # Example--- ---  ```---  # use bevy_input::gamepad::{Gamepad, GamepadRumbleRequest, GamepadRumbleIntensity};---  # use bevy_ecs::prelude::{EventWriter, Res, Query, Entity, With};---  # use core::time::Duration;---  fn rumble_gamepad_system(---      mut rumble_requests: EventWriter<GamepadRumbleRequest>,---      gamepads: Query<Entity, With<Gamepad>>,---  ) {---      for entity in gamepads.iter() {---          rumble_requests.write(GamepadRumbleRequest::Add {---              gamepad: entity,---              intensity: GamepadRumbleIntensity::MAX,---              duration: Duration::from_secs_f32(0.5),---          });---      }---  }---  ```


---@class GamepadSettings
---  Gamepad settings component.--- ---  ## Usage--- ---  It is used to create a `bevy` component that stores the settings of [`GamepadButton`] and [`GamepadAxis`] in [`Gamepad`].---  If no user defined [`ButtonSettings`], [`AxisSettings`], or [`ButtonAxisSettings`]---  are defined, the default settings of each are used as a fallback accordingly.--- ---  ## Note--- ---  The [`GamepadSettings`] are used to determine when raw gamepad events---  should register. Events that don't meet the change thresholds defined in [`GamepadSettings`]---  will not register. To modify these settings, mutate the corresponding component.
---@field  default_button_settings ? ButtonSettings
---@field  default_axis_settings ? AxisSettings
---@field  default_button_axis_settings ? ButtonAxisSettings
---@field  button_settings ? HashMap
---@field  axis_settings ? HashMap
---@field  button_axis_settings ? HashMap


---@class RawGamepadAxisChangedEvent
---  [`GamepadAxis`] changed event unfiltered by [`GamepadSettings`].
---@field  gamepad ? Entity
---@field  axis ? GamepadAxis
---@field  value ? number


---@class RawGamepadButtonChangedEvent
---  [`GamepadButton`] changed event unfiltered by [`GamepadSettings`].
---@field  gamepad ? Entity
---@field  button ? GamepadButton
---@field  value ? number


---@class RawGamepadEvent
---  A raw gamepad event.--- ---  This event type is used over the [`GamepadConnectionEvent`],---  [`RawGamepadButtonChangedEvent`] and [`RawGamepadAxisChangedEvent`] when---  the in-frame relative ordering of events is important.--- ---  This event type is used by `bevy_input` to feed its components.


---@class DoubleTapGesture
---  Double tap gesture.--- ---  ## Platform-specific--- ---  - Only available on **`macOS`** and **`iOS`**.---  - On **`iOS`**, must be enabled first


---@class PanGesture
---  Pan gesture.--- ---  ## Platform-specific--- ---  - On **`iOS`**, must be enabled first
---@field  [1] ? Vec2


---@class PinchGesture
---  Two-finger pinch gesture, often used for magnifications.--- ---  Positive delta values indicate magnification (zooming in) and---  negative delta values indicate shrinking (zooming out).--- ---  ## Platform-specific--- ---  - Only available on **`macOS`** and **`iOS`**.---  - On **`iOS`**, must be enabled first
---@field  [1] ? number


---@class RotationGesture
---  Two-finger rotation gesture.--- ---  Positive delta values indicate rotation counterclockwise and---  negative delta values indicate rotation clockwise.--- ---  ## Platform-specific--- ---  - Only available on **`macOS`** and **`iOS`**.---  - On **`iOS`**, must be enabled first
---@field  [1] ? number


---@class Key
---  The logical key code of a [`KeyboardInput`].--- ---  ## Technical--- ---  Its values map 1 to 1 to winit's Key.


---@class KeyCode
---  The key code of a [`KeyboardInput`].--- ---  ## Usage--- ---  It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<ButtonInput<KeyCode>>`.--- ---  Code representing the location of a physical key---  This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few---  exceptions:---  - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and---    `SuperRight` here.---  - The key that the specification calls "Super" is reported as `Unidentified` here.--- ---  [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables--- ---  ## Updating--- ---  The resource is updated inside of the [`keyboard_input_system`].


---@class KeyboardFocusLost
---  Gets generated from `bevy_winit::winit_runner`--- ---  Used for clearing all cached states to avoid having 'stuck' key presses---  when, for example, switching between windows with 'Alt-Tab' or using any other---  OS specific key combination that leads to Bevy window losing focus and not receiving any---  input events


---@class KeyboardInput
---  A keyboard input event.--- ---  This event is the translated version of the `WindowEvent::KeyboardInput` from the `winit` crate.---  It is available to the end user and can be used for game logic.--- ---  ## Usage--- ---  The event is consumed inside of the [`keyboard_input_system`]---  to update the [`ButtonInput<KeyCode>`](ButtonInput<KeyCode>) resource.
---@field  key_code ? KeyCode
---@field  logical_key ? Key
---@field  state ? ButtonState
---@field  text ? Option
---@field  repeat ? boolean
---@field  window ? Entity


---@class NativeKey
---  Contains the platform-native logical key identifier, known as keysym.--- ---  Exactly what that means differs from platform to platform, but the values are to some degree---  tied to the currently active keyboard layout. The same key on the same keyboard may also report---  different values on different platforms, which is one of the reasons this is a per-platform---  enum.--- ---  This enum is primarily used to store raw keysym when Winit doesn't map a given native logical---  key identifier to a meaningful [`Key`] variant. This lets you use [`Key`], and let the user---  define keybinds which work in the presence of identifiers we haven't mapped for you yet.


---@class NativeKeyCode
---  Contains the platform-native physical key identifier--- ---  The exact values vary from platform to platform (which is part of why this is a per-platform---  enum), but the values are primarily tied to the key's physical location on the keyboard.--- ---  This enum is primarily used to store raw keycodes when Winit doesn't map a given native---  physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we---  haven't mapped for you yet, this lets you use [`KeyCode`] to:--- ---  - Correctly match key press and release events.---  - On non-web platforms, support assigning keybinds to virtually any key through a UI.


---@class AccumulatedMouseMotion
---  Tracks how much the mouse has moved every frame.--- ---  This resource is reset to zero every frame.--- ---  This resource sums the total [`MouseMotion`] events received this frame.
---@field  delta ? Vec2


---@class AccumulatedMouseScroll
---  Tracks how much the mouse has scrolled every frame.--- ---  This resource is reset to zero every frame.--- ---  This resource sums the total [`MouseWheel`] events received this frame.
---@field  unit ? MouseScrollUnit
---@field  delta ? Vec2


---@class MouseButton
---  A button on a mouse device.--- ---  ## Usage--- ---  It is used as the generic `T` value of an [`ButtonInput`] to create a `bevy`---  resource.--- ---  ## Updating--- ---  The resource is updated inside of the [`mouse_button_input_system`].


---@class MouseButtonInput
---  A mouse button input event.--- ---  This event is the translated version of the `WindowEvent::MouseInput` from the `winit` crate.--- ---  ## Usage--- ---  The event is read inside of the [`mouse_button_input_system`]---  to update the [`ButtonInput<MouseButton>`] resource.
---@field  button ? MouseButton
---@field  state ? ButtonState
---@field  window ? Entity


---@class MouseMotion
---  An event reporting the change in physical position of a pointing device.--- ---  This represents raw, unfiltered physical motion.---  It is the translated version of [`DeviceEvent::MouseMotion`] from the `winit` crate.--- ---  All pointing devices connected to a single machine at the same time can emit the event independently.---  However, the event data does not make it possible to distinguish which device it is referring to.--- ---  [`DeviceEvent::MouseMotion`]: https://docs.rs/winit/latest/winit/event/enum.DeviceEvent.html#variant.MouseMotion
---@field  delta ? Vec2


---@class MouseScrollUnit
---  The scroll unit.--- ---  Describes how a value of a [`MouseWheel`] event has to be interpreted.--- ---  The value of the event can either be interpreted as the amount of lines or the amount of pixels---  to scroll.


---@class MouseWheel
---  A mouse wheel event.--- ---  This event is the translated version of the `WindowEvent::MouseWheel` from the `winit` crate.
---@field  unit ? MouseScrollUnit
---@field  x ? number
---@field  y ? number
---@field  window ? Entity


---@class ForceTouch
---  A force description of a [`Touch`] input.


---@class TouchInput
---  A touch input event.--- ---  ## Logic--- ---  Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique---  identifier for the finger is generated. When the finger is lifted, the [`TouchPhase::Ended`]---  event is generated with the same finger id.--- ---  After a [`TouchPhase::Started`] event has been emitted, there may be zero or more [`TouchPhase::Moved`]---  events when the finger is moved or the touch pressure changes.--- ---  The finger id may be reused by the system after an [`TouchPhase::Ended`] event. The user---  should assume that a new [`TouchPhase::Started`] event received with the same id has nothing---  to do with the old finger and is a new finger.--- ---  A [`TouchPhase::Canceled`] event is emitted when the system has canceled tracking this---  touch, such as when the window loses focus, or on iOS if the user moves the---  device against their face.--- ---  ## Note--- ---  This event is the translated version of the `WindowEvent::Touch` from the `winit` crate.---  It is available to the end user and can be used for game logic.
---@field  phase ? TouchPhase
---@field  position ? Vec2
---@field  window ? Entity
---@field  force ? Option
---@field  id ? integer


---@class TouchPhase
---  A phase of a [`TouchInput`].--- ---  ## Usage--- ---  It is used to describe the phase of the touch input that is currently active.---  This includes a phase that indicates that a touch input has started or ended,---  or that a finger has moved. There is also a canceled phase that indicates that---  the system canceled the tracking of the finger.


---@class AspectRatio
---  An `AspectRatio` is the ratio of width to height.
---@field  [1] ? number


---@class Aabb2d
---  A 2D axis-aligned bounding box, or bounding rectangle
---@field  min ? Vec2
---@field  max ? Vec2


---@class BoundingCircle
---  A bounding circle
---@field  center ? Vec2
---@field  circle ? Circle


---@class Aabb3d
---  A 3D axis-aligned bounding box
---@field  min ? Vec3A
---@field  max ? Vec3A


---@class BoundingSphere
---  A bounding sphere
---@field  center ? Vec3A
---@field  sphere ? Sphere


---@class AabbCast2d
---  An intersection test that casts an [`Aabb2d`] along a ray.
---@field  ray ? RayCast2d
---@field  aabb ? Aabb2d


---@class BoundingCircleCast
---  An intersection test that casts a [`BoundingCircle`] along a ray.
---@field  ray ? RayCast2d
---@field  circle ? BoundingCircle


---@class RayCast2d
---  A raycast intersection test for 2D bounding volumes
---@field  ray ? Ray2d
---@field  max ? number
---@field  direction_recip ? Vec2


---@class AabbCast3d
---  An intersection test that casts an [`Aabb3d`] along a ray.
---@field  ray ? RayCast3d
---@field  aabb ? Aabb3d


---@class BoundingSphereCast
---  An intersection test that casts a [`BoundingSphere`] along a ray.
---@field  ray ? RayCast3d
---@field  sphere ? BoundingSphere


---@class RayCast3d
---  A raycast intersection test for 3D bounding volumes
---@field  origin ? Vec3A
---@field  direction ? Dir3A
---@field  max ? number
---@field  direction_recip ? Vec3A


---@class CompassOctant
---  A compass enum with 8 directions.---  ```text---           N (North)---           ▲---      NW   │   NE---         ╲ │ ╱---  W (West) ┼─────► E (East)---         ╱ │ ╲---      SW   │   SE---           ▼---           S (South)---  ```


---@class CompassQuadrant
---  A compass enum with 4 directions.---  ```text---           N (North)---           ▲---           │---           │---  W (West) ┼─────► E (East)---           │---           │---           ▼---           S (South)---  ```


---@class EaseFunction
---  Curve functions over the [unit interval], commonly used for easing transitions.--- ---  `EaseFunction` can be used on its own to interpolate between `0.0` and `1.0`.---  It can also be combined with [`EasingCurve`] to interpolate between other---  intervals and types, including vectors and rotations.--- ---  # Example--- ---  [`sample`] the smoothstep function at various points. This will return `None`---  if the parameter is outside the unit interval.--- ---  ```---  # use bevy_math::prelude::*;---  let f = EaseFunction::SmoothStep;--- ---  assert_eq!(f.sample(-1.0), None);---  assert_eq!(f.sample(0.0), Some(0.0));---  assert_eq!(f.sample(0.5), Some(0.5));---  assert_eq!(f.sample(1.0), Some(1.0));---  assert_eq!(f.sample(2.0), None);---  ```--- ---  [`sample_clamped`] will clamp the parameter to the unit interval, so it---  always returns a value.--- ---  ```---  # use bevy_math::prelude::*;---  # let f = EaseFunction::SmoothStep;---  assert_eq!(f.sample_clamped(-1.0), 0.0);---  assert_eq!(f.sample_clamped(0.0), 0.0);---  assert_eq!(f.sample_clamped(0.5), 0.5);---  assert_eq!(f.sample_clamped(1.0), 1.0);---  assert_eq!(f.sample_clamped(2.0), 1.0);---  ```--- ---  [`sample`]: EaseFunction::sample---  [`sample_clamped`]: EaseFunction::sample_clamped---  [unit interval]: `Interval::UNIT`


---@class JumpAt
---  Configuration options for the [`EaseFunction::Steps`] curves. This closely replicates the---  [CSS step function specification].--- ---  [CSS step function specification]: https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description


---@class Interval
---  A nonempty closed interval, possibly unbounded in either direction.--- ---  In other words, the interval may stretch all the way to positive or negative infinity, but it---  will always have some nonempty interior.
---@field  start ? number
---@field  end ? number


---@class Dir2
---  A normalized vector pointing in a direction in 2D space
---@field  [1] ? Vec2


---@class Dir3
---  A normalized vector pointing in a direction in 3D space
---@field  [1] ? Vec3


---@class Dir3A
---  A normalized SIMD vector pointing in a direction in 3D space.--- ---  This type stores a 16 byte aligned [`Vec3A`].---  This may or may not be faster than [`Dir3`]: make sure to benchmark!
---@field  [1] ? Vec3A


---@class FloatOrd
---  A wrapper for floats that implements [`Ord`], [`Eq`], and [`Hash`] traits.--- ---  This is a work around for the fact that the IEEE 754-2008 standard,---  implemented by Rust's [`f32`] type,---  doesn't define an ordering for [`NaN`](f32::NAN),---  and `NaN` is not considered equal to any other `NaN`.--- ---  Wrapping a float with `FloatOrd` breaks conformance with the standard---  by sorting `NaN` as less than all other numbers and equal to any other `NaN`.
---@field  [1] ? number


---@class Isometry2d
---  An isometry in two dimensions, representing a rotation followed by a translation.---  This can often be useful for expressing relative positions and transformations from one position to another.--- ---  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,---  and belongs to the special [Euclidean group] SE(2). This includes translation and rotation, but excludes reflection.--- ---  For the three-dimensional version, see [`Isometry3d`].--- ---  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group--- ---  # Example--- ---  Isometries can be created from a given translation and rotation:--- ---  ```---  # use bevy_math::{Isometry2d, Rot2, Vec2};---  #---  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));---  ```--- ---  Or from separate parts:--- ---  ```---  # use bevy_math::{Isometry2d, Rot2, Vec2};---  #---  let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));---  let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));---  ```--- ---  The isometries can be used to transform points:--- ---  ```---  # use approx::assert_abs_diff_eq;---  # use bevy_math::{Isometry2d, Rot2, Vec2};---  #---  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));---  let point = Vec2::new(4.0, 4.0);--- ---  // These are equivalent---  let result = iso.transform_point(point);---  let result = iso * point;--- ---  assert_eq!(result, Vec2::new(-2.0, 5.0));---  ```--- ---  Isometries can also be composed together:--- ---  ```---  # use bevy_math::{Isometry2d, Rot2, Vec2};---  #---  # let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));---  # let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));---  # let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));---  #---  assert_eq!(iso1 * iso2, iso);---  ```--- ---  One common operation is to compute an isometry representing the relative positions of two objects---  for things like intersection tests. This can be done with an inverse transformation:--- ---  ```---  # use bevy_math::{Isometry2d, Rot2, Vec2};---  #---  let circle_iso = Isometry2d::from_translation(Vec2::new(2.0, 1.0));---  let rectangle_iso = Isometry2d::from_rotation(Rot2::degrees(90.0));--- ---  // Compute the relative position and orientation between the two shapes---  let relative_iso = circle_iso.inverse() * rectangle_iso;--- ---  // Or alternatively, to skip an extra rotation operation:---  let relative_iso = circle_iso.inverse_mul(rectangle_iso);---  ```
---@field  rotation ? Rot2
---@field  translation ? Vec2


---@class Isometry3d
---  An isometry in three dimensions, representing a rotation followed by a translation.---  This can often be useful for expressing relative positions and transformations from one position to another.--- ---  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,---  and belongs to the special [Euclidean group] SE(3). This includes translation and rotation, but excludes reflection.--- ---  For the two-dimensional version, see [`Isometry2d`].--- ---  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group--- ---  # Example--- ---  Isometries can be created from a given translation and rotation:--- ---  ```---  # use bevy_math::{Isometry3d, Quat, Vec3};---  # use std::f32::consts::FRAC_PI_2;---  #---  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));---  ```--- ---  Or from separate parts:--- ---  ```---  # use bevy_math::{Isometry3d, Quat, Vec3};---  # use std::f32::consts::FRAC_PI_2;---  #---  let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));---  let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));---  ```--- ---  The isometries can be used to transform points:--- ---  ```---  # use approx::assert_relative_eq;---  # use bevy_math::{Isometry3d, Quat, Vec3};---  # use std::f32::consts::FRAC_PI_2;---  #---  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));---  let point = Vec3::new(4.0, 4.0, 4.0);--- ---  // These are equivalent---  let result = iso.transform_point(point);---  let result = iso * point;--- ---  assert_relative_eq!(result, Vec3::new(-2.0, 5.0, 7.0));---  ```--- ---  Isometries can also be composed together:--- ---  ```---  # use bevy_math::{Isometry3d, Quat, Vec3};---  # use std::f32::consts::FRAC_PI_2;---  #---  # let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));---  # let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));---  # let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));---  #---  assert_eq!(iso1 * iso2, iso);---  ```--- ---  One common operation is to compute an isometry representing the relative positions of two objects---  for things like intersection tests. This can be done with an inverse transformation:--- ---  ```---  # use bevy_math::{Isometry3d, Quat, Vec3};---  # use std::f32::consts::FRAC_PI_2;---  #---  let sphere_iso = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));---  let cuboid_iso = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));--- ---  // Compute the relative position and orientation between the two shapes---  let relative_iso = sphere_iso.inverse() * cuboid_iso;--- ---  // Or alternatively, to skip an extra rotation operation:---  let relative_iso = sphere_iso.inverse_mul(cuboid_iso);---  ```
---@field  rotation ? Quat
---@field  translation ? Vec3A


---@class Annulus
---  A primitive shape formed by the region between two circles, also known as a ring.
---@field  inner_circle ? Circle
---@field  outer_circle ? Circle


---@class Arc2d
---  A primitive representing an arc between two points on a circle.--- ---  An arc has no area.---  If you want to include the portion of a circle's area swept out by the arc,---  use the pie-shaped [`CircularSector`].---  If you want to include only the space inside the convex hull of the arc,---  use the bowl-shaped [`CircularSegment`].--- ---  The arc is drawn starting from [`Vec2::Y`], extending by `half_angle` radians on---  either side. The center of the circle is the origin [`Vec2::ZERO`]. Note that this---  means that the origin may not be within the `Arc2d`'s convex hull.--- ---  **Warning:** Arcs with negative angle or radius, or with angle greater than an entire circle, are not officially supported.---  It is recommended to normalize arcs to have an angle in [0, 2π].
---@field  radius ? number
---@field  half_angle ? number


---@class Capsule2d
---  A 2D capsule primitive, also known as a stadium or pill shape.--- ---  A two-dimensional capsule is defined as a neighborhood of points at a distance (radius) from a line
---@field  radius ? number
---@field  half_length ? number


---@class Circle
---  A circle primitive, representing the set of points some distance from the origin
---@field  radius ? number


---@class CircularSector
---  A primitive representing a circular sector: a pie slice of a circle.--- ---  The segment is positioned so that it always includes [`Vec2::Y`] and is vertically symmetrical.---  To orient the sector differently, apply a rotation.---  The sector is drawn with the center of its circle at the origin [`Vec2::ZERO`].--- ---  **Warning:** Circular sectors with negative angle or radius, or with angle greater than an entire circle, are not officially supported.---  We recommend normalizing circular sectors to have an angle in [0, 2π].
---@field  arc ? Arc2d


---@class CircularSegment
---  A primitive representing a circular segment:---  the area enclosed by the arc of a circle and its chord (the line between its endpoints).--- ---  The segment is drawn starting from [`Vec2::Y`], extending equally on either side.---  To orient the segment differently, apply a rotation.---  The segment is drawn with the center of its circle at the origin [`Vec2::ZERO`].---  When positioning a segment, the [`apothem`](Self::apothem) function may be particularly useful.--- ---  **Warning:** Circular segments with negative angle or radius, or with angle greater than an entire circle, are not officially supported.---  We recommend normalizing circular segments to have an angle in [0, 2π].
---@field  arc ? Arc2d


---@class Ellipse
---  An ellipse primitive, which is like a circle, but the width and height can be different
---@field  half_size ? Vec2


---@class Line2d
---  An infinite line going through the origin along a direction in 2D space.--- ---  For a finite line: [`Segment2d`]
---@field  direction ? Dir2


---@class Plane2d
---  An unbounded plane in 2D space. It forms a separating surface through the origin,---  stretching infinitely far
---@field  normal ? Dir2


---@class Rectangle
---  A rectangle primitive, which is like a square, except that the width and height can be different
---@field  half_size ? Vec2


---@class RegularPolygon
---  A polygon centered on the origin where all vertices lie on a circle, equally far apart.
---@field  circumcircle ? Circle
---@field  sides ? integer


---@class Rhombus
---  A rhombus primitive, also known as a diamond shape.---  A four sided polygon, centered on the origin, where opposite sides are parallel but without---  requiring right angles.
---@field  half_diagonals ? Vec2


---@class Segment2d
---  A line segment defined by two endpoints in 2D space.
---@field  vertices ? [glam::Vec2; 2]


---@class Triangle2d
---  A triangle in 2D space
---@field  vertices ? [glam::Vec2; 3]


---@class Capsule3d
---  A 3D capsule primitive centered on the origin---  A three-dimensional capsule is defined as a surface at a distance (radius) from a line
---@field  radius ? number
---@field  half_length ? number


---@class Cone
---  A cone primitive centered on the midpoint between the tip of the cone and the center of its base.--- ---  The cone is oriented with its tip pointing towards the Y axis.
---@field  radius ? number
---@field  height ? number


---@class ConicalFrustum
---  A conical frustum primitive.---  A conical frustum can be created---  by slicing off a section of a cone.
---@field  radius_top ? number
---@field  radius_bottom ? number
---@field  height ? number


---@class Cuboid
---  A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not---  required to be the same.
---@field  half_size ? Vec3


---@class Cylinder
---  A cylinder primitive centered on the origin
---@field  radius ? number
---@field  half_height ? number


---@class InfinitePlane3d
---  An unbounded plane in 3D space. It forms a separating surface through the origin,---  stretching infinitely far
---@field  normal ? Dir3


---@class Line3d
---  An infinite line going through the origin along a direction in 3D space.--- ---  For a finite line: [`Segment3d`]
---@field  direction ? Dir3


---@class Plane3d
---  A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.
---@field  normal ? Dir3
---@field  half_size ? Vec2


---@class Segment3d
---  A line segment defined by two endpoints in 3D space.
---@field  vertices ? [glam::Vec3; 2]


---@class Sphere
---  A sphere primitive, representing the set of all points some distance from the origin
---@field  radius ? number


---@class Tetrahedron
---  A tetrahedron primitive.
---@field  vertices ? [glam::Vec3; 4]


---@class Torus
---  A torus primitive, often representing a ring or donut shape---  The set of points some distance from a circle centered at the origin
---@field  minor_radius ? number
---@field  major_radius ? number


---@class Triangle3d
---  A 3D triangle primitive.
---@field  vertices ? [glam::Vec3; 3]


---@class Ray2d
---  An infinite half-line starting at `origin` and going in `direction` in 2D space.
---@field  origin ? Vec2
---@field  direction ? Dir2


---@class Ray3d
---  An infinite half-line starting at `origin` and going in `direction` in 3D space.
---@field  origin ? Vec3
---@field  direction ? Dir3


---@class IRect
---  A rectangle defined by two opposite corners.--- ---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,---  stored in `IRect::min` and `IRect::max`, respectively. The minimum/maximum invariant---  must be upheld by the user when directly assigning the fields, otherwise some methods---  produce invalid results. It is generally recommended to use one of the constructor---  methods instead, which will ensure this invariant is met, unless you already have---  the minimum and maximum corners.
---@field  min ? IVec2
---@field  max ? IVec2


---@class Rect
---  A rectangle defined by two opposite corners.--- ---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,---  stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant---  must be upheld by the user when directly assigning the fields, otherwise some methods---  produce invalid results. It is generally recommended to use one of the constructor---  methods instead, which will ensure this invariant is met, unless you already have---  the minimum and maximum corners.
---@field  min ? Vec2
---@field  max ? Vec2


---@class URect
---  A rectangle defined by two opposite corners.--- ---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,---  stored in `URect::min` and `URect::max`, respectively. The minimum/maximum invariant---  must be upheld by the user when directly assigning the fields, otherwise some methods---  produce invalid results. It is generally recommended to use one of the constructor---  methods instead, which will ensure this invariant is met, unless you already have---  the minimum and maximum corners.
---@field  min ? UVec2
---@field  max ? UVec2


---@class Rot2
---  A counterclockwise 2D rotation.--- ---  # Example--- ---  ```---  # use approx::assert_relative_eq;---  # use bevy_math::{Rot2, Vec2};---  use std::f32::consts::PI;--- ---  // Create rotations from radians or degrees---  let rotation1 = Rot2::radians(PI / 2.0);---  let rotation2 = Rot2::degrees(45.0);--- ---  // Get the angle back as radians or degrees---  assert_eq!(rotation1.as_degrees(), 90.0);---  assert_eq!(rotation2.as_radians(), PI / 4.0);--- ---  // "Add" rotations together using `*`---  #[cfg(feature = "approx")]---  assert_relative_eq!(rotation1 * rotation2, Rot2::degrees(135.0));--- ---  // Rotate vectors---  #[cfg(feature = "approx")]---  assert_relative_eq!(rotation1 * Vec2::X, Vec2::Y);---  ```
---@field  cos ? number
---@field  sin ? number


---@class Instant



---@class Fixed
---  The fixed timestep game clock following virtual time.--- ---  A specialization of the [`Time`] structure. **For method documentation, see---  [`Time<Fixed>#impl-Time<Fixed>`].**---      ---  It is automatically inserted as a resource by---  [`TimePlugin`](crate::TimePlugin) and updated based on---  [`Time<Virtual>`](Virtual). The fixed clock is automatically set as the---  generic [`Time`] resource during [`FixedUpdate`](bevy_app::FixedUpdate)---  schedule processing.--- ---  The fixed timestep clock advances in fixed-size increments, which is---  extremely useful for writing logic (like physics) that should have---  consistent behavior, regardless of framerate.--- ---  The default [`timestep()`](Time::timestep) is 64 hertz, or 15625---  microseconds. This value was chosen because using 60 hertz has the potential---  for a pathological interaction with the monitor refresh rate where the game---  alternates between running two fixed timesteps and zero fixed timesteps per---  frame (for example when running two fixed timesteps takes longer than a---  frame). Additionally, the value is a power of two which losslessly converts---  into [`f32`] and [`f64`].--- ---  To run a system on a fixed timestep, add it to one of the [`FixedMain`]---  schedules, most commonly [`FixedUpdate`](bevy_app::FixedUpdate).--- ---  This schedule is run a number of times between---  [`PreUpdate`](bevy_app::PreUpdate) and [`Update`](bevy_app::Update)---  according to the accumulated [`overstep()`](Time::overstep) time divided by---  the [`timestep()`](Time::timestep). This means the schedule may run 0, 1 or---  more times during a single update (which typically corresponds to a rendered---  frame).--- ---  `Time<Fixed>` and the generic [`Time`] resource will report a---  [`delta()`](Time::delta) equal to [`timestep()`](Time::timestep) and always---  grow [`elapsed()`](Time::elapsed) by one [`timestep()`](Time::timestep) per---  iteration.--- ---  The fixed timestep clock follows the [`Time<Virtual>`](Virtual) clock, which---  means it is affected by [`pause()`](Time::pause),---  [`set_relative_speed()`](Time::set_relative_speed) and---  [`set_max_delta()`](Time::set_max_delta) from virtual time. If the virtual---  clock is paused, the [`FixedUpdate`](bevy_app::FixedUpdate) schedule will---  not run. It is guaranteed that the [`elapsed()`](Time::elapsed) time in---  `Time<Fixed>` is always between the previous `elapsed()` and the current---  `elapsed()` value in `Time<Virtual>`, so the values are compatible.--- ---  Changing the timestep size while the game is running should not normally be---  done, as having a regular interval is the point of this schedule, but it may---  be necessary for effects like "bullet-time" if the normal granularity of the---  fixed timestep is too big for the slowed down time. In this case,---  [`set_timestep()`](Time::set_timestep) and be called to set a new value. The---  new value will be used immediately for the next run of the---  [`FixedUpdate`](bevy_app::FixedUpdate) schedule, meaning that it will affect---  the [`delta()`](Time::delta) value for the very next---  [`FixedUpdate`](bevy_app::FixedUpdate), even if it is still during the same---  frame. Any [`overstep()`](Time::overstep) present in the accumulator will be---  processed according to the new [`timestep()`](Time::timestep) value.
---@field  timestep ? Duration
---@field  overstep ? Duration


---@class Real
---  Real time clock representing elapsed wall clock time.--- ---  A specialization of the [`Time`] structure. **For method documentation, see---  [`Time<Real>#impl-Time<Real>`].**--- ---  It is automatically inserted as a resource by---  [`TimePlugin`](crate::TimePlugin) and updated with time instants according---  to [`TimeUpdateStrategy`](crate::TimeUpdateStrategy).[^disclaimer]--- ---  Note:---  Using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration)---  allows for mocking the wall clock for testing purposes.---  Besides this use case, it is not recommended to do this, as it will no longer---  represent "wall clock" time as intended.--- ---  The [`delta()`](Time::delta) and [`elapsed()`](Time::elapsed) values of this---  clock should be used for anything which deals specifically with real time---  (wall clock time). It will not be affected by relative game speed---  adjustments, pausing or other adjustments.[^disclaimer]--- ---  The clock does not count time from [`startup()`](Time::startup) to---  [`first_update()`](Time::first_update()) into elapsed, but instead will---  start counting time from the first update call. [`delta()`](Time::delta) and---  [`elapsed()`](Time::elapsed) will report zero on the first update as there---  is no previous update instant. This means that a [`delta()`](Time::delta) of---  zero must be handled without errors in application logic, as it may---  theoretically also happen at other times.--- ---  [`Instant`]s for [`startup()`](Time::startup),---  [`first_update()`](Time::first_update) and---  [`last_update()`](Time::last_update) are recorded and accessible.--- ---  [^disclaimer]: When using [`TimeUpdateStrategy::ManualDuration`](crate::TimeUpdateStrategy::ManualDuration),---      [`Time<Real>#impl-Time<Real>`] is only a *mock* of wall clock time.---
---@field  startup ? Instant
---@field  first_update ? Option
---@field  last_update ? Option


---@class Stopwatch
---  A Stopwatch is a struct that tracks elapsed time when started.--- ---  Note that in order to advance the stopwatch [`tick`](Stopwatch::tick) **MUST** be called.---  # Examples--- ---  ```---  # use bevy_time::*;---  use std::time::Duration;---  let mut stopwatch = Stopwatch::new();---  assert_eq!(stopwatch.elapsed_secs(), 0.0);--- ---  stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second---  assert_eq!(stopwatch.elapsed_secs(), 1.0);--- ---  stopwatch.pause();---  stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick---  assert_eq!(stopwatch.elapsed_secs(), 1.0);--- ---  stopwatch.reset(); // reset the stopwatch---  assert!(stopwatch.is_paused());---  assert_eq!(stopwatch.elapsed_secs(), 0.0);---  ```
---@field  elapsed ? Duration
---@field  is_paused ? boolean


---@class Timer
---  Tracks elapsed time. Enters the finished state once `duration` is reached.--- ---  Non repeating timers will stop tracking and stay in the finished state until reset.---  Repeating timers will only be in the finished state on each tick `duration` is reached or---  exceeded, and can still be reset at any given point.--- ---  Paused timers will not have elapsed time increased.--- ---  Note that in order to advance the timer [`tick`](Timer::tick) **MUST** be called.
---@field  stopwatch ? Stopwatch
---@field  duration ? Duration
---@field  mode ? TimerMode
---@field  finished ? boolean
---@field  times_finished_this_tick ? integer


---@class TimerMode
---  Specifies [`Timer`] behavior.


---@class Virtual
---  The virtual game clock representing game time.--- ---  A specialization of the [`Time`] structure. **For method documentation, see---  [`Time<Virtual>#impl-Time<Virtual>`].**--- ---  Normally used as `Time<Virtual>`. It is automatically inserted as a resource---  by [`TimePlugin`](crate::TimePlugin) and updated based on---  [`Time<Real>`](Real). The virtual clock is automatically set as the default---  generic [`Time`] resource for the update.--- ---  The virtual clock differs from real time clock in that it can be paused, sped up---  and slowed down. It also limits how much it can advance in a single update---  in order to prevent unexpected behavior in cases where updates do not happen---  at regular intervals (e.g. coming back after the program was suspended a long time).--- ---  The virtual clock can be paused by calling [`pause()`](Time::pause) and---  unpaused by calling [`unpause()`](Time::unpause). When the game clock is---  paused [`delta()`](Time::delta) will be zero on each update, and---  [`elapsed()`](Time::elapsed) will not grow.---  [`effective_speed()`](Time::effective_speed) will return `0.0`. Calling---  [`pause()`](Time::pause) will not affect value the [`delta()`](Time::delta)---  value for the update currently being processed.--- ---  The speed of the virtual clock can be changed by calling---  [`set_relative_speed()`](Time::set_relative_speed). A value of `2.0` means---  that virtual clock should advance twice as fast as real time, meaning that---  [`delta()`](Time::delta) values will be double of what---  [`Time<Real>::delta()`](Time::delta) reports and---  [`elapsed()`](Time::elapsed) will go twice as fast as---  [`Time<Real>::elapsed()`](Time::elapsed). Calling---  [`set_relative_speed()`](Time::set_relative_speed) will not affect the---  [`delta()`](Time::delta) value for the update currently being processed.--- ---  The maximum amount of delta time that can be added by a single update can be---  set by [`set_max_delta()`](Time::set_max_delta). This value serves a dual---  purpose in the virtual clock.--- ---  If the game temporarily freezes due to any reason, such as disk access, a---  blocking system call, or operating system level suspend, reporting the full---  elapsed delta time is likely to cause bugs in game logic. Usually if a---  laptop is suspended for an hour, it doesn't make sense to try to simulate---  the game logic for the elapsed hour when resuming. Instead it is better to---  lose the extra time and pretend a shorter duration of time passed. Setting---  [`max_delta()`](Time::max_delta) to a relatively short time means that the---  impact on game logic will be minimal.--- ---  If the game lags for some reason, meaning that it will take a longer time to---  compute a frame than the real time that passes during the computation, then---  we would fall behind in processing virtual time. If this situation persists,---  and computing a frame takes longer depending on how much virtual time has---  passed, the game would enter a "death spiral" where computing each frame---  takes longer and longer and the game will appear to freeze. By limiting the---  maximum time that can be added at once, we also limit the amount of virtual---  time the game needs to compute for each frame. This means that the game will---  run slow, and it will run slower than real time, but it will not freeze and---  it will recover as soon as computation becomes fast again.--- ---  You should set [`max_delta()`](Time::max_delta) to a value that is---  approximately the minimum FPS your game should have even if heavily lagged---  for a moment. The actual FPS when lagged will be somewhat lower than this,---  depending on how much more time it takes to compute a frame compared to real---  time. You should also consider how stable your FPS is, as the limit will---  also dictate how big of an FPS drop you can accept without losing time and---  falling behind real time.
---@field  max_delta ? Duration
---@field  paused ? boolean
---@field  relative_speed ? number
---@field  effective_speed ? number


---@class GlobalTransform
---  [`GlobalTransform`] is an affine transformation from entity-local coordinates to worldspace coordinates.--- ---  You cannot directly mutate [`GlobalTransform`]; instead, you change an entity's transform by manipulating---  its [`Transform`], which indirectly causes Bevy to update its [`GlobalTransform`].--- ---  * To get the global transform of an entity, you should get its [`GlobalTransform`].---  * For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].---    [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.--- ---  ## [`Transform`] and [`GlobalTransform`]--- ---  [`Transform`] transforms an entity relative to its parent's reference frame, or relative to world space coordinates,---  if it doesn't have a [`ChildOf`](bevy_ecs::hierarchy::ChildOf) component.--- ---  [`GlobalTransform`] is managed by Bevy; it is computed by successively applying the [`Transform`] of each ancestor---  entity which has a Transform. This is done automatically by Bevy-internal systems in the system set---  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).--- ---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you---  update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag---  before the [`GlobalTransform`] is updated.--- ---  # Examples--- ---  - [`transform`][transform_example]--- ---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  [1] ? Affine3A


---@class Transform
---  Describe the position of an entity. If the entity has a parent, the position is relative---  to its parent position.--- ---  * To place or move an entity, you should set its [`Transform`].---  * To get the global transform of an entity, you should get its [`GlobalTransform`].---  * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].---    [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.--- ---  ## [`Transform`] and [`GlobalTransform`]--- ---  [`Transform`] is the position of an entity relative to its parent position, or the reference---  frame if it doesn't have a [`ChildOf`](bevy_ecs::hierarchy::ChildOf) component.--- ---  [`GlobalTransform`] is the position of an entity relative to the reference frame.--- ---  [`GlobalTransform`] is updated from [`Transform`] by systems in the system set---  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).--- ---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you---  update the [`Transform`] of an entity during this set or after, you will notice a 1 frame lag---  before the [`GlobalTransform`] is updated.--- ---  # Examples--- ---  - [`transform`][transform_example]--- ---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  translation ? Vec3
---@field  rotation ? Quat
---@field  scale ? Vec3


---@class TransformTreeChanged
---  An optimization for transform propagation. This ZST marker component uses change detection to---  mark all entities of the hierarchy as "dirty" if any of their descendants have a changed---  `Transform`. If this component is *not* marked `is_changed()`, propagation will halt.


---@class TypeId



---@class SocketAddr



---@class RangeFull



---@class AtomicBool



---@class AtomicI16



---@class AtomicI32



---@class AtomicI64



---@class AtomicI8



---@class AtomicIsize



---@class AtomicU16



---@class AtomicU32



---@class AtomicU64



---@class AtomicU8



---@class AtomicUsize



---@class Duration



---@class Affine2

---@field  matrix2 ? Mat2
---@field  translation ? Vec2


---@class Affine3A

---@field  matrix3 ? Mat3A
---@field  translation ? Vec3A


---@class BVec2

---@field  x ? boolean
---@field  y ? boolean


---@class BVec3

---@field  x ? boolean
---@field  y ? boolean
---@field  z ? boolean


---@class BVec3A



---@class BVec4

---@field  x ? boolean
---@field  y ? boolean
---@field  z ? boolean
---@field  w ? boolean


---@class BVec4A



---@class DAffine2

---@field  matrix2 ? DMat2
---@field  translation ? DVec2


---@class DAffine3

---@field  matrix3 ? DMat3
---@field  translation ? DVec3


---@class DMat2

---@field  x_axis ? DVec2
---@field  y_axis ? DVec2


---@class DMat3

---@field  x_axis ? DVec3
---@field  y_axis ? DVec3
---@field  z_axis ? DVec3


---@class DMat4

---@field  x_axis ? DVec4
---@field  y_axis ? DVec4
---@field  z_axis ? DVec4
---@field  w_axis ? DVec4


---@class DQuat

---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number


---@class DVec2

---@field  x ? number
---@field  y ? number


---@class DVec3

---@field  x ? number
---@field  y ? number
---@field  z ? number


---@class DVec4

---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number


---@class EulerRot



---@class I16Vec2

---@field  x ? integer
---@field  y ? integer


---@class I16Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class I16Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class I64Vec2

---@field  x ? integer
---@field  y ? integer


---@class I64Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class I64Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class I8Vec2

---@field  x ? integer
---@field  y ? integer


---@class I8Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class I8Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class IVec2

---@field  x ? integer
---@field  y ? integer


---@class IVec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class IVec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class Mat2

---@field  x_axis ? Vec2
---@field  y_axis ? Vec2


---@class Mat3

---@field  x_axis ? Vec3
---@field  y_axis ? Vec3
---@field  z_axis ? Vec3


---@class Mat3A

---@field  x_axis ? Vec3A
---@field  y_axis ? Vec3A
---@field  z_axis ? Vec3A


---@class Mat4

---@field  x_axis ? Vec4
---@field  y_axis ? Vec4
---@field  z_axis ? Vec4
---@field  w_axis ? Vec4


---@class Quat

---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number


---@class U16Vec2

---@field  x ? integer
---@field  y ? integer


---@class U16Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class U16Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class U64Vec2

---@field  x ? integer
---@field  y ? integer


---@class U64Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class U64Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class U8Vec2

---@field  x ? integer
---@field  y ? integer


---@class U8Vec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class U8Vec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class UVec2

---@field  x ? integer
---@field  y ? integer


---@class UVec3

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer


---@class UVec4

---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer


---@class Vec2

---@field  x ? number
---@field  y ? number


---@class Vec3

---@field  x ? number
---@field  y ? number
---@field  z ? number


---@class Vec3A

---@field  x ? number
---@field  y ? number
---@field  z ? number


---@class Vec4

---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number


---@class SmolStr



---@class Uuid



---@class AssetIndex
---  A generational runtime-only identifier for a specific [`Asset`] stored in [`Assets`]. This is optimized for efficient runtime---  usage and is not suitable for identifying assets across app runs.
---@field  generation ? integer
---@field  index ? integer


---@class AssetPath
---  Represents a path to an asset in a "virtual filesystem".--- ---  Asset paths consist of three main parts:---  * [`AssetPath::source`]: The name of the [`AssetSource`](crate::io::AssetSource) to load the asset from.---    This is optional. If one is not set the default source will be used (which is the `assets` folder by default).---  * [`AssetPath::path`]: The "virtual filesystem path" pointing to an asset source file.---  * [`AssetPath::label`]: An optional "named sub asset". When assets are loaded, they are---    allowed to load "sub assets" of any type, which are identified by a named "label".--- ---  Asset paths are generally constructed (and visualized) as strings:--- ---  ```no_run---  # use bevy_asset::{Asset, AssetServer, Handle};---  # use bevy_reflect::TypePath;---  #---  # #[derive(Asset, TypePath, Default)]---  # struct Mesh;---  #---  # #[derive(Asset, TypePath, Default)]---  # struct Scene;---  #---  # let asset_server: AssetServer = panic!();---  // This loads the `my_scene.scn` base asset from the default asset source.---  let scene: Handle<Scene> = asset_server.load("my_scene.scn");--- ---  // This loads the `PlayerMesh` labeled asset from the `my_scene.scn` base asset in the default asset source.---  let mesh: Handle<Mesh> = asset_server.load("my_scene.scn#PlayerMesh");--- ---  // This loads the `my_scene.scn` base asset from a custom 'remote' asset source.---  let scene: Handle<Scene> = asset_server.load("remote://my_scene.scn");---  ```--- ---  [`AssetPath`] implements [`From`] for `&'static str`, `&'static Path`, and `&'a String`,---  which allows us to optimize the static cases.---  This means that the common case of `asset_server.load("my_scene.scn")` when it creates and---  clones internal owned [`AssetPaths`](AssetPath).---  This also means that you should use [`AssetPath::parse`] in cases where `&str` is the explicit type.


---@class RenderAssetUsages
---  Defines where the asset will be used.--- ---  If an asset is set to the `RENDER_WORLD` but not the `MAIN_WORLD`, the asset will be---  unloaded from the asset server once it's been extracted and prepared in the render world.--- ---  Unloading the asset saves on memory, as for most cases it is no longer necessary to keep---  it in RAM once it's been uploaded to the GPU's VRAM. However, this means you can no longer---  access the asset from the CPU (via the `Assets<T>` resource) once unloaded (without re-loading it).--- ---  If you never need access to the asset from the CPU past the first frame it's loaded on,---  or only need very infrequent access, then set this to `RENDER_WORLD`. Otherwise, set this to---  `RENDER_WORLD | MAIN_WORLD`.--- ---  If you have an asset that doesn't actually need to end up in the render world, like an Image---  that will be decoded into another Image asset, use `MAIN_WORLD` only.--- ---  ## Platform-specific--- ---  On Wasm, it is not possible for now to free reserved memory. To control memory usage, load assets---  in sequence and unload one before loading the next. See this---  [discussion about memory management](https://github.com/WebAssembly/design/issues/1397) for more---  details.


---@class DeferredPrepass
---  If added to a [`crate::prelude::Camera3d`] then deferred materials will be rendered to the deferred gbuffer texture and will be available to subsequent passes.---  Note the default deferred lighting plugin also requires `DepthPrepass` to work correctly.


---@class SystemIdMarker
---  Marker [`Component`](bevy_ecs::component::Component) for identifying [`SystemId`] [`Entity`]s.


---@class OnAdd
---  Trigger emitted when a component is inserted onto an entity that does not already have that---  component. Runs before `OnInsert`.---  See [`crate::component::ComponentHooks::on_add`] for more information.


---@class OnDespawn
---  Trigger emitted for each component on an entity when it is despawned.---  See [`crate::component::ComponentHooks::on_despawn`] for more information.


---@class OnInsert
---  Trigger emitted when a component is inserted, regardless of whether or not the entity already---  had that component. Runs after `OnAdd`, if it ran.---  See [`crate::component::ComponentHooks::on_insert`] for more information.


---@class OnRemove
---  Trigger emitted when a component is removed from an entity, and runs before the component is---  removed, so you can still access the component data.---  See [`crate::component::ComponentHooks::on_remove`] for more information.


---@class OnReplace
---  Trigger emitted when a component is inserted onto an entity that already has that component.---  Runs before the value is replaced, so you can still access the original component data.---  See [`crate::component::ComponentHooks::on_replace`] for more information.


---@class Image



---@class TextureAtlas
---  An index into a [`TextureAtlasLayout`], which corresponds to a specific section of a texture.--- ---  It stores a handle to [`TextureAtlasLayout`] and the index of the current section of the atlas.---  The texture atlas contains various *sections* of a given texture, allowing users to have a single---  image file for either sprite animation or global mapping.---  You can change the texture [`index`](Self::index) of the atlas to animate the sprite or display only a *section* of the texture---  for efficient rendering of related game objects.--- ---  Check the following examples for usage:---  - [`animated sprite sheet example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)---  - [`sprite animation event example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)---  - [`texture atlas example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
---@field  layout ? Handle
---@field  index ? integer


---@class TextureAtlasLayout
---  Stores a map used to lookup the position of a texture in a [`TextureAtlas`].---  This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.--- ---  Optionally it can store a mapping from sub texture handles to the related area index (see---  [`TextureAtlasBuilder`]).--- ---  [Example usage animating sprite.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)---  [Example usage animating sprite in response to an event.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)---  [Example usage loading sprite sheet.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)--- ---  [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
---@field  size ? UVec2
---@field  textures ? Vec


---@class Affine3
---  Reduced-size version of `glam::Affine3A` for use when storage has---  significant performance impact. Convert to `glam::Affine3A` to do---  non-trivial calculations.
---@field  matrix3 ? Mat3
---@field  translation ? Vec3


---@class Indices
---  An array of indices into the [`VertexAttributeValues`](super::VertexAttributeValues) for a mesh.--- ---  It describes the order in which the vertex attributes should be joined into faces.


---@class Mesh
---  A 3D object made out of vertices representing triangles, lines, or points,---  with "attribute" values for each vertex.--- ---  Meshes can be automatically generated by a bevy `AssetLoader` (generally by loading a `Gltf` file),---  or by converting a [primitive](bevy_math::primitives) using [`into`](Into).---  It is also possible to create one manually. They can be edited after creation.--- ---  Meshes can be rendered with a `Mesh2d` and `MeshMaterial2d`---  or `Mesh3d` and `MeshMaterial3d` for 2D and 3D respectively.--- ---  A [`Mesh`] in Bevy is equivalent to a "primitive" in the glTF format, for a---  glTF Mesh representation, see `GltfMesh`.--- ---  ## Manual creation--- ---  The following function will construct a flat mesh, to be rendered with a---  `StandardMaterial` or `ColorMaterial`:--- ---  ```---  # use bevy_mesh::{Mesh, Indices, PrimitiveTopology};---  # use bevy_asset::RenderAssetUsages;---  fn create_simple_parallelogram() -> Mesh {---      // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.---      Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())---          // Add 4 vertices, each with its own position attribute (coordinate in---          // 3D space), for each of the corners of the parallelogram.---          .with_inserted_attribute(---              Mesh::ATTRIBUTE_POSITION,---              vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]---          )---          // Assign a UV coordinate to each vertex.---          .with_inserted_attribute(---              Mesh::ATTRIBUTE_UV_0,---              vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]---          )---          // Assign normals (everything points outwards)---          .with_inserted_attribute(---              Mesh::ATTRIBUTE_NORMAL,---              vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]---          )---          // After defining all the vertices and their attributes, build each triangle using the---          // indices of the vertices that make it up in a counter-clockwise order.---          .with_inserted_indices(Indices::U32(vec![---              // First triangle---              0, 3, 1,---              // Second triangle---              1, 3, 2---          ]))---  }---  ```--- ---  You can see how it looks like [here](https://github.com/bevyengine/bevy/blob/main/assets/docs/Mesh.png),---  used in a `Mesh3d` with a square bevy logo texture, with added axis, points,---  lines and text for clarity.--- ---  ## Other examples--- ---  For further visualization, explanation, and examples, see the built-in Bevy examples,---  and the [implementation of the built-in shapes](https://github.com/bevyengine/bevy/tree/main/crates/bevy_mesh/src/primitives).---  In particular, [generate_custom_mesh](https://github.com/bevyengine/bevy/blob/main/examples/3d/generate_custom_mesh.rs)---  teaches you to access and modify the attributes of a [`Mesh`] after creating it.--- ---  ## Common points of confusion--- ---  - UV maps in Bevy start at the top-left, see [`ATTRIBUTE_UV_0`](Mesh::ATTRIBUTE_UV_0),---    other APIs can have other conventions, `OpenGL` starts at bottom-left.---  - It is possible and sometimes useful for multiple vertices to have the same---    [position attribute](Mesh::ATTRIBUTE_POSITION) value,---    it's a common technique in 3D modeling for complex UV mapping or other calculations.---  - Bevy performs frustum culling based on the `Aabb` of meshes, which is calculated---    and added automatically for new meshes only. If a mesh is modified, the entity's `Aabb`---    needs to be updated manually or deleted so that it is re-calculated.--- ---  ## Use with `StandardMaterial`--- ---  To render correctly with `StandardMaterial`, a mesh needs to have properly defined:---  - [`UVs`](Mesh::ATTRIBUTE_UV_0): Bevy needs to know how to map a texture onto the mesh---    (also true for `ColorMaterial`).---  - [`Normals`](Mesh::ATTRIBUTE_NORMAL): Bevy needs to know how light interacts with your mesh.---    [0.0, 0.0, 1.0] is very common for simple flat meshes on the XY plane,---    because simple meshes are smooth and they don't require complex light calculations.---  - Vertex winding order: by default, `StandardMaterial.cull_mode` is `Some(Face::Back)`,---    which means that Bevy would *only* render the "front" of each triangle, which---    is the side of the triangle from where the vertices appear in a *counter-clockwise* order.
---@field  indices ? Option
---@field  morph_targets ? Option
---@field  morph_target_names ? Option
---@field  asset_usage ? RenderAssetUsages


---@class MeshMorphWeights
---  Control a specific [`Mesh`] instance's [morph targets]. These control the weights of---  specific "mesh primitives" in scene formats like GLTF. They can be set manually, but---  in most cases they should "automatically" synced by setting the [`MorphWeights`] component---  on a parent entity.--- ---  See [`MorphWeights`] for more details on Bevy's morph target implementation.--- ---  Add this to an [`Entity`] with a `Mesh3d` with a [`MorphAttributes`] set---  to control individual weights of each morph target.--- ---  [morph targets]: https://en.wikipedia.org/wiki/Morph_target_animation
---@field  weights ? Vec


---@class MorphWeights
---  Controls the [morph targets] for all child `Mesh3d` entities. In most cases, [`MorphWeights`] should be considered---  the "source of truth" when writing morph targets for meshes. However you can choose to write child [`MeshMorphWeights`]---  if your situation requires more granularity. Just note that if you set [`MorphWeights`], it will overwrite child---  [`MeshMorphWeights`] values.--- ---  This exists because Bevy's [`Mesh`] corresponds to a _single_ surface / material, whereas morph targets---  as defined in the GLTF spec exist on "multi-primitive meshes" (where each primitive is its own surface with its own material).---  Therefore in Bevy [`MorphWeights`] an a parent entity are the "canonical weights" from a GLTF perspective, which then---  synchronized to child `Mesh3d` / [`MeshMorphWeights`] (which correspond to "primitives" / "surfaces" from a GLTF perspective).--- ---  Add this to the parent of one or more [`Entities`](`Entity`) with a `Mesh3d` with a [`MeshMorphWeights`].--- ---  [morph targets]: https://en.wikipedia.org/wiki/Morph_target_animation
---@field  weights ? Vec
---@field  first_mesh ? Option


---@class AnnulusMeshBuilder
---  A builder for creating a [`Mesh`] with an [`Annulus`] shape.
---@field  annulus ? Annulus
---@field  resolution ? integer


---@class Capsule2dMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Capsule2d`] shape.
---@field  capsule ? Capsule2d
---@field  resolution ? integer


---@class CircleMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Circle`] shape.
---@field  circle ? Circle
---@field  resolution ? integer


---@class CircularMeshUvMode
---  Specifies how to generate UV-mappings for the [`CircularSector`] and [`CircularSegment`] shapes.--- ---  Currently the only variant is `Mask`, which is good for showing a portion of a texture that includes---  the entire circle, particularly the same texture will be displayed with different fractions of a---  complete circle.--- ---  It's expected that more will be added in the future, such as a variant that causes the texture to be---  scaled to fit the bounding box of the shape, which would be good for packed textures only including the---  portion of the circle that is needed to display.


---@class CircularSectorMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`CircularSector`] shape.--- ---  The resulting mesh will have a UV-map such that the center of the circle is---  at the center of the texture.
---@field  sector ? CircularSector
---@field  resolution ? integer
---@field  uv_mode ? CircularMeshUvMode


---@class CircularSegmentMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`CircularSegment`] shape.--- ---  The resulting mesh will have a UV-map such that the center of the circle is---  at the center of the texture.
---@field  segment ? CircularSegment
---@field  resolution ? integer
---@field  uv_mode ? CircularMeshUvMode


---@class EllipseMeshBuilder
---  A builder used for creating a [`Mesh`] with an [`Ellipse`] shape.
---@field  ellipse ? Ellipse
---@field  resolution ? integer


---@class RectangleMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Rectangle`] shape.
---@field  half_size ? Vec2


---@class RegularPolygonMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`RegularPolygon`] shape.
---@field  circumradius ? number
---@field  sides ? integer


---@class RhombusMeshBuilder
---  A builder for creating a [`Mesh`] with an [`Rhombus`] shape.
---@field  half_diagonals ? Vec2


---@class Triangle2dMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Triangle2d`] shape.
---@field  triangle ? Triangle2d


---@class Capsule3dMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Capsule3d`] shape.
---@field  capsule ? Capsule3d
---@field  rings ? integer
---@field  longitudes ? integer
---@field  latitudes ? integer
---@field  uv_profile ? CapsuleUvProfile


---@class CapsuleUvProfile
---  Manner in which UV coordinates are distributed vertically.


---@class ConeAnchor
---  Anchoring options for [`ConeMeshBuilder`]


---@class ConeMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Cone`] shape.
---@field  cone ? Cone
---@field  resolution ? integer
---@field  anchor ? ConeAnchor


---@class ConicalFrustumMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`ConicalFrustum`] shape.
---@field  frustum ? ConicalFrustum
---@field  resolution ? integer
---@field  segments ? integer


---@class CuboidMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Cuboid`] shape.
---@field  half_size ? Vec3


---@class CylinderAnchor
---  Anchoring options for [`CylinderMeshBuilder`]


---@class CylinderMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Cylinder`] shape.
---@field  cylinder ? Cylinder
---@field  resolution ? integer
---@field  segments ? integer
---@field  caps ? boolean
---@field  anchor ? CylinderAnchor


---@class PlaneMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Plane3d`] shape.
---@field  plane ? Plane3d
---@field  subdivisions ? integer


---@class SphereKind
---  A type of sphere mesh.


---@class SphereMeshBuilder
---  A builder used for creating a [`Mesh`] with an [`Sphere`] shape.
---@field  sphere ? Sphere
---@field  kind ? SphereKind


---@class TetrahedronMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Tetrahedron`] shape.
---@field  tetrahedron ? Tetrahedron


---@class TorusMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Torus`] shape.
---@field  torus ? Torus
---@field  minor_resolution ? integer
---@field  major_resolution ? integer
---@field  angle_range ? RangeInclusive


---@class Triangle3dMeshBuilder
---  A builder used for creating a [`Mesh`] with a [`Triangle3d`] shape.
---@field  triangle ? Triangle3d


---@class SkinnedMesh

---@field  inverse_bindposes ? bevy_asset::handle::Handle<bevy_mesh::skinning::SkinnedMeshInverseBindposes>
---@field  joints ? Vec


---@class ScriptAsset
---  Represents a script loaded into memory as an asset


---@class FunctionArgInfo
---  Information about a function argument.
---@field  name ? Option
---@field  arg_index ? integer
---@field  type_id ? TypeId


---@class FunctionInfo
---  Information about a function.
---@field  name ? Cow
---@field  namespace ? Namespace
---@field  arg_info ? Vec
---@field  return_info ? FunctionReturnInfo
---@field  docs ? Option


---@class FunctionReturnInfo
---  Information about a function return value.
---@field  type_id ? TypeId


---@class InteropError
---  An error occurring when converting between rust and a script context.


---@class Namespace
---  A namespace for functions


---@class DynamicComponent
---  A dynamic script component
---@field  data ? ScriptValue


---@class ScriptValue
---  An abstraction of values that can be passed to and from scripts.---  This allows us to re-use logic between scripting languages.


---@class AlphaMode
---  Sets how a material's base color alpha channel is used for transparency.


---@class Camera
---  The defining [`Component`] for camera entities,---  storing information about how and what to render through this camera.--- ---  The [`Camera`] component is added to an entity to define the properties of the viewpoint from---  which rendering occurs. It defines the position of the view to render, the projection method---  to transform the 3D objects into a 2D image, as well as the render target into which that image---  is produced.--- ---  Note that a [`Camera`] needs a [`CameraRenderGraph`] to render anything.---  This is typically provided by adding a [`Camera2d`] or [`Camera3d`] component,---  but custom render graphs can also be defined. Inserting a [`Camera`] with no render---  graph will emit an error at runtime.--- ---  [`Camera2d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/core_2d/struct.Camera2d.html---  [`Camera3d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/core_3d/struct.Camera3d.html
---@field  viewport ? Option
---@field  order ? integer
---@field  is_active ? boolean
---@field  target ? RenderTarget
---@field  hdr ? boolean
---@field  msaa_writeback ? boolean
---@field  clear_color ? ClearColorConfig
---@field  sub_camera_view ? Option


---@class CameraMainTextureUsages
---  This component lets you control the [`TextureUsages`] field of the main texture generated for the camera


---@class CameraRenderGraph
---  Configures the [`RenderGraph`](crate::render_graph::RenderGraph) name assigned to be run for a given [`Camera`] entity.


---@class Exposure
---  How much energy a `Camera3d` absorbs from incoming light.--- ---  <https://en.wikipedia.org/wiki/Exposure_(photography)>


---@class ImageRenderTarget
---  A render target that renders to an [`Image`].
---@field  handle ? Handle
---@field  scale_factor ? FloatOrd


---@class MipBias
---  Camera component specifying a mip bias to apply when sampling from material textures.--- ---  Often used in conjunction with antialiasing post-process effects to reduce textures blurriness.
---@field  [1] ? number


---@class RenderTarget
---  The "target" that a [`Camera`] will render to. For example, this could be a [`Window`]---  swapchain or an [`Image`].


---@class SubCameraView
---  Settings to define a camera sub view.--- ---  When [`Camera::sub_camera_view`] is `Some`, only the sub-section of the---  image defined by `size` and `offset` (relative to the `full_size` of the---  whole image) is projected to the cameras viewport.--- ---  Take the example of the following multi-monitor setup:---  ```css---  ┌───┬───┐---  │ A │ B │---  ├───┼───┤---  │ C │ D │---  └───┴───┘---  ```---  If each monitor is 1920x1080, the whole image will have a resolution of---  3840x2160. For each monitor we can use a single camera with a viewport of---  the same size as the monitor it corresponds to. To ensure that the image is---  cohesive, we can use a different sub view on each camera:---  - Camera A: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 0,0---  - Camera B: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 1920,0---  - Camera C: `full_size` = 3840x2160, `size` = 1920x1080, `offset` = 0,1080---  - Camera D: `full_size` = 3840x2160, `size` = 1920x1080, `offset` =---    1920,1080--- ---  However since only the ratio between the values is important, they could all---  be divided by 120 and still produce the same image. Camera D would for---  example have the following values:---  `full_size` = 32x18, `size` = 16x9, `offset` = 16,9
---@field  full_size ? UVec2
---@field  offset ? Vec2
---@field  size ? UVec2


---@class TemporalJitter
---  A subpixel offset to jitter a perspective camera's frustum by.--- ---  Useful for temporal rendering techniques.--- ---  Do not use with [`OrthographicProjection`].--- ---  [`OrthographicProjection`]: crate::camera::OrthographicProjection
---@field  offset ? Vec2


---@class Viewport
---  Render viewport configuration for the [`Camera`] component.--- ---  The viewport defines the area on the render target to which the camera renders its image.---  You can overlay multiple cameras in a single window using viewports to create effects like---  split screen, minimaps, and character viewers.
---@field  physical_position ? UVec2
---@field  physical_size ? UVec2
---@field  depth ? Range


---@class ClearColor
---  A [`Resource`] that stores the color that is used to clear the screen between frames.--- ---  This color appears as the "background" color for simple apps,---  when there are portions of the screen with nothing rendered.
---@field  [1] ? Color


---@class ClearColorConfig
---  For a camera, specifies the color used to clear the viewport before rendering.


---@class ManualTextureViewHandle
---  A unique id that corresponds to a specific [`ManualTextureView`] in the [`ManualTextureViews`] collection.
---@field  [1] ? integer


---@class CustomProjection
---  Holds a dynamic [`CameraProjection`] trait object. Use [`Projection::custom()`] to construct a---  custom projection.--- ---  The contained dynamic object can be downcast into a static type using [`CustomProjection::get`].


---@class OrthographicProjection
---  Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`],---  the size of objects remains the same regardless of their distance to the camera.--- ---  The volume contained in the projection is called the *view frustum*. Since the viewport is rectangular---  and projection lines are parallel, the view frustum takes the shape of a cuboid.--- ---  Note that the scale of the projection and the apparent size of objects are inversely proportional.---  As the size of the projection increases, the size of objects decreases.--- ---  # Examples--- ---  Configure the orthographic projection to one world unit per 100 window pixels:--- ---  ```---  # use bevy_render::camera::{OrthographicProjection, Projection, ScalingMode};---  let projection = Projection::Orthographic(OrthographicProjection {---      scaling_mode: ScalingMode::WindowSize,---      scale: 0.01,---      ..OrthographicProjection::default_2d()---  });---  ```
---@field  near ? number
---@field  far ? number
---@field  viewport_origin ? Vec2
---@field  scaling_mode ? ScalingMode
---@field  scale ? number
---@field  area ? Rect


---@class PerspectiveProjection
---  A 3D camera projection in which distant objects appear smaller than close objects.
---@field  fov ? number
---@field  aspect_ratio ? number
---@field  near ? number
---@field  far ? number


---@class Projection
---  Component that defines how to compute a [`Camera`]'s projection matrix.--- ---  Common projections, like perspective and orthographic, are provided out of the box to handle the---  majority of use cases. Custom projections can be added using the [`CameraProjection`] trait and---  the [`Projection::custom`] constructor.--- ---  ## What's a projection?--- ---  A camera projection essentially describes how 3d points from the point of view of a camera are---  projected onto a 2d screen. This is where properties like a camera's field of view are defined.---  More specifically, a projection is a 4x4 matrix that transforms points from view space (the---  point of view of the camera) into clip space. Clip space is almost, but not quite, equivalent to---  the rectangle that is rendered to your screen, with a depth axis. Any points that land outside---  the bounds of this cuboid are "clipped" and not rendered.--- ---  You can also think of the projection as the thing that describes the shape of a camera's---  frustum: the volume in 3d space that is visible to a camera.--- ---  [`Camera`]: crate::camera::Camera


---@class OcclusionCulling
---  Add this component to a view in order to enable experimental GPU occlusion---  culling.--- ---  *Bevy's occlusion culling is currently marked as experimental.* There are---  known issues whereby, in rare circumstances, occlusion culling can result in---  meshes being culled that shouldn't be (i.e. meshes that turn invisible).---  Please try it out and report issues.--- ---  *Occlusion culling* allows Bevy to avoid rendering objects that are fully---  behind other opaque or alpha tested objects. This is different from, and---  complements, depth fragment rejection as the `DepthPrepass` enables. While---  depth rejection allows Bevy to avoid rendering *pixels* that are behind---  other objects, the GPU still has to examine those pixels to reject them,---  which requires transforming the vertices of the objects and performing---  skinning if the objects were skinned. Occlusion culling allows the GPU to go---  a step further, avoiding even transforming the vertices of objects that it---  can quickly prove to be behind other objects.--- ---  Occlusion culling inherently has some overhead, because Bevy must examine---  the objects' bounding boxes, and create an acceleration structure---  (hierarchical Z-buffer) to perform the occlusion tests. Therefore, occlusion---  culling is disabled by default. Only enable it if you measure it to be a---  speedup on your scene. Note that, because Bevy's occlusion culling runs on---  the GPU and is quite efficient, it's rare for occlusion culling to result in---  a significant slowdown.--- ---  Occlusion culling currently requires a `DepthPrepass`. If no depth prepass---  is present on the view, the [`OcclusionCulling`] component will be ignored.---  Additionally, occlusion culling is currently incompatible with deferred---  shading; including both `DeferredPrepass` and [`OcclusionCulling`] results---  in unspecified behavior.--- ---  The algorithm that Bevy uses is known as [*two-phase occlusion culling*].---  When you enable occlusion culling, Bevy splits the depth prepass into two:---  an *early* depth prepass and a *late* depth prepass. The early depth prepass---  renders all the meshes that were visible last frame to produce a---  conservative approximation of the depth buffer. Then, after producing an---  acceleration structure known as a hierarchical Z-buffer or depth pyramid,---  Bevy tests the bounding boxes of all meshes against that depth buffer. Those---  that can be quickly proven to be behind the geometry rendered during the---  early depth prepass are skipped entirely. The other potentially-visible---  meshes are rendered during the late prepass, and finally all the visible---  meshes are rendered as usual during the opaque, transparent, etc. passes.--- ---  Unlike other occlusion culling systems you may be familiar with, Bevy's---  occlusion culling is fully dynamic and requires no baking step. The CPU---  overhead is minimal. Large skinned meshes and other dynamic objects can---  occlude other objects.--- ---  [*two-phase occlusion culling*]:---  https://medium.com/@mil_kru/two-pass-occlusion-culling-4100edcad501


---@class GlobalsUniform
---  Contains global values useful when writing shaders.---  Currently only contains values related to time.
---@field  time ? number
---@field  delta_time ? number
---@field  frame_count ? integer


---@class Mesh2d
---  A component for 2D meshes. Requires a [`MeshMaterial2d`] to be rendered, commonly using a [`ColorMaterial`].--- ---  [`MeshMaterial2d`]: <https://docs.rs/bevy/latest/bevy/sprite/struct.MeshMaterial2d.html>---  [`ColorMaterial`]: <https://docs.rs/bevy/latest/bevy/sprite/struct.ColorMaterial.html>--- ---  # Example--- ---  ```ignore---  # use bevy_sprite::{ColorMaterial, Mesh2d, MeshMaterial2d};---  # use bevy_ecs::prelude::*;---  # use bevy_render::mesh::Mesh;---  # use bevy_color::palettes::basic::RED;---  # use bevy_asset::Assets;---  # use bevy_math::primitives::Circle;---  #---  // Spawn an entity with a mesh using `ColorMaterial`.---  fn setup(---      mut commands: Commands,---      mut meshes: ResMut<Assets<Mesh>>,---      mut materials: ResMut<Assets<ColorMaterial>>,---  ) {---      commands.spawn((---          Mesh2d(meshes.add(Circle::new(50.0))),---          MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),---      ));---  }---  ```
---@field  [1] ? Handle


---@class Mesh3d
---  A component for 3D meshes. Requires a [`MeshMaterial3d`] to be rendered, commonly using a [`StandardMaterial`].--- ---  [`MeshMaterial3d`]: <https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html>---  [`StandardMaterial`]: <https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html>--- ---  # Example--- ---  ```ignore---  # use bevy_pbr::{Material, MeshMaterial3d, StandardMaterial};---  # use bevy_ecs::prelude::*;---  # use bevy_render::mesh::{Mesh, Mesh3d};---  # use bevy_color::palettes::basic::RED;---  # use bevy_asset::Assets;---  # use bevy_math::primitives::Capsule3d;---  #---  // Spawn an entity with a mesh using `StandardMaterial`.---  fn setup(---      mut commands: Commands,---      mut meshes: ResMut<Assets<Mesh>>,---      mut materials: ResMut<Assets<StandardMaterial>>,---  ) {---      commands.spawn((---          Mesh3d(meshes.add(Capsule3d::default())),---          MeshMaterial3d(materials.add(StandardMaterial {---              base_color: RED.into(),---              ..Default::default()---          })),---      ));---  }---  ```
---@field  [1] ? Handle


---@class Aabb
---  An axis-aligned bounding box, defined by:---  - a center,---  - the distances from the center to each faces along the axis,---    the faces are orthogonal to the axis.--- ---  It is typically used as a component on an entity to represent the local space---  occupied by this entity, with faces orthogonal to its local axis.--- ---  This component is notably used during "frustum culling", a process to determine---  if an entity should be rendered by a [`Camera`] if its bounding box intersects---  with the camera's [`Frustum`].--- ---  It will be added automatically by the systems in [`CalculateBounds`] to entities that:---  - could be subject to frustum culling, for example with a [`Mesh3d`]---    or `Sprite` component,---  - don't have the [`NoFrustumCulling`] component.--- ---  It won't be updated automatically if the space occupied by the entity changes,---  for example if the vertex positions of a [`Mesh3d`] are updated.--- ---  [`Camera`]: crate::camera::Camera---  [`NoFrustumCulling`]: crate::view::visibility::NoFrustumCulling---  [`CalculateBounds`]: crate::view::visibility::VisibilitySystems::CalculateBounds---  [`Mesh3d`]: crate::mesh::Mesh
---@field  center ? Vec3A
---@field  half_extents ? Vec3A


---@class CascadesFrusta



---@class CubemapFrusta



---@class Frustum
---  A region of 3D space defined by the intersection of 6 [`HalfSpace`]s.--- ---  Frustums are typically an apex-truncated square pyramid (a pyramid without the top) or a cuboid.--- ---  Half spaces are ordered left, right, top, bottom, near, far. The normal vectors---  of the half-spaces point towards the interior of the frustum.--- ---  A frustum component is used on an entity with a [`Camera`] component to---  determine which entities will be considered for rendering by this camera.---  All entities with an [`Aabb`] component that are not contained by (or crossing---  the boundary of) the frustum will not be rendered, and not be used in rendering computations.--- ---  This process is called frustum culling, and entities can opt out of it using---  the [`NoFrustumCulling`] component.--- ---  The frustum component is typically added automatically for cameras, either `Camera2d` or `Camera3d`.---  It is usually updated automatically by [`update_frusta`] from the---  [`CameraProjection`] component and [`GlobalTransform`] of the camera entity.--- ---  [`Camera`]: crate::camera::Camera---  [`NoFrustumCulling`]: crate::view::visibility::NoFrustumCulling---  [`update_frusta`]: crate::view::visibility::update_frusta---  [`CameraProjection`]: crate::camera::CameraProjection---  [`GlobalTransform`]: bevy_transform::components::GlobalTransform


---@class ShaderStorageBuffer
---  A storage buffer that is prepared as a [`RenderAsset`] and uploaded to the GPU.


---@class SyncToRenderWorld
---  Marker component that indicates that its entity needs to be synchronized to the render world.--- ---  This component is automatically added as a required component by [`ExtractComponentPlugin`] and [`SyncComponentPlugin`].---  For more information see [`SyncWorldPlugin`].--- ---  NOTE: This component should persist throughout the entity's entire lifecycle.---  If this component is removed from its entity, the entity will be despawned.--- ---  [`ExtractComponentPlugin`]: crate::extract_component::ExtractComponentPlugin---  [`SyncComponentPlugin`]: crate::sync_component::SyncComponentPlugin


---@class ColorGrading
---  Configures filmic color grading parameters to adjust the image appearance.--- ---  Color grading is applied just before tonemapping for a given---  [`Camera`](crate::camera::Camera) entity, with the sole exception of the---  `post_saturation` value in [`ColorGradingGlobal`], which is applied after---  tonemapping.
---@field  global ? ColorGradingGlobal
---@field  shadows ? ColorGradingSection
---@field  midtones ? ColorGradingSection
---@field  highlights ? ColorGradingSection


---@class ColorGradingGlobal
---  Filmic color grading values applied to the image as a whole (as opposed to---  individual sections, like shadows and highlights).
---@field  exposure ? number
---@field  temperature ? number
---@field  tint ? number
---@field  hue ? number
---@field  post_saturation ? number
---@field  midtones_range ? Range


---@class ColorGradingSection
---  A section of color grading values that can be selectively applied to---  shadows, midtones, and highlights.
---@field  saturation ? number
---@field  contrast ? number
---@field  gamma ? number
---@field  gain ? number
---@field  lift ? number


---@class Msaa
---  Component for configuring the number of samples for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing)---  for a [`Camera`](crate::camera::Camera).--- ---  Defaults to 4 samples. A higher number of samples results in smoother edges.--- ---  Some advanced rendering features may require that MSAA is disabled.--- ---  Note that the web currently only supports 1 or 4 samples.


---@class InheritedVisibility
---  Whether or not an entity is visible in the hierarchy.---  This will not be accurate until [`VisibilityPropagate`] runs in the [`PostUpdate`] schedule.--- ---  If this is false, then [`ViewVisibility`] should also be false.--- ---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate
---@field  [1] ? boolean


---@class NoFrustumCulling
---  Use this component to opt-out of built-in frustum culling for entities, see---  [`Frustum`].--- ---  It can be used for example:---  - when a [`Mesh`] is updated but its [`Aabb`] is not, which might happen with animations,---  - when using some light effects, like wanting a [`Mesh`] out of the [`Frustum`]---    to appear in the reflection of a [`Mesh`] within.


---@class ViewVisibility
---  Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.--- ---  Each frame, this will be reset to `false` during [`VisibilityPropagate`] systems in [`PostUpdate`].---  Later in the frame, systems in [`CheckVisibility`] will mark any visible entities using [`ViewVisibility::set`].---  Because of this, values of this type will be marked as changed every frame, even when they do not change.--- ---  If you wish to add custom visibility system that sets this value, make sure you add it to the [`CheckVisibility`] set.--- ---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate---  [`CheckVisibility`]: VisibilitySystems::CheckVisibility
---@field  [1] ? boolean


---@class Visibility
---  User indication of whether an entity is visible. Propagates down the entity hierarchy.--- ---  If an entity is hidden in this way, all [`Children`] (and all of their children and so on) who---  are set to [`Inherited`](Self::Inherited) will also be hidden.--- ---  This is done by the `visibility_propagate_system` which uses the entity hierarchy and---  `Visibility` to set the values of each entity's [`InheritedVisibility`] component.


---@class VisibilityClass
---  A bucket into which we group entities for the purposes of visibility.--- ---  Bevy's various rendering subsystems (3D, 2D, UI, etc.) want to be able to---  quickly winnow the set of entities to only those that the subsystem is---  tasked with rendering, to avoid spending time examining irrelevant entities.---  At the same time, Bevy wants the [`check_visibility`] system to determine---  all entities' visibilities at the same time, regardless of what rendering---  subsystem is responsible for drawing them. Additionally, your application---  may want to add more types of renderable objects that Bevy determines---  visibility for just as it does for Bevy's built-in objects.--- ---  The solution to this problem is *visibility classes*. A visibility class is---  a type, typically the type of a component, that represents the subsystem---  that renders it: for example, `Mesh3d`, `Mesh2d`, and `Sprite`. The---  [`VisibilityClass`] component stores the visibility class or classes that---  the entity belongs to. (Generally, an object will belong to only one---  visibility class, but in rare cases it may belong to multiple.)--- ---  When adding a new renderable component, you'll typically want to write an---  add-component hook that adds the type ID of that component to the---  [`VisibilityClass`] array. See `custom_phase_item` for an example.
---@field  [1] ? SmallVec


---@class VisibleEntities
---  Collection of entities visible from the current view.--- ---  This component contains all entities which are visible from the currently---  rendered view. The collection is updated automatically by the [`VisibilitySystems::CheckVisibility`]---  system set. Renderers can use the equivalent [`RenderVisibleEntities`] to optimize rendering of---  a particular view, to prevent drawing items not visible from that view.--- ---  This component is intended to be attached to the same entity as the [`Camera`] and---  the [`Frustum`] defining the view.


---@class VisibilityRange
---  Specifies the range of distances that this entity must be from the camera in---  order to be rendered.--- ---  This is also known as *hierarchical level of detail* or *HLOD*.--- ---  Use this component when you want to render a high-polygon mesh when the---  camera is close and a lower-polygon mesh when the camera is far away. This---  is a common technique for improving performance, because fine details are---  hard to see in a mesh at a distance. To avoid an artifact known as *popping*---  between levels, each level has a *margin*, within which the object---  transitions gradually from invisible to visible using a dithering effect.--- ---  You can also use this feature to replace multiple meshes with a single mesh---  when the camera is distant. This is the reason for the term "*hierarchical*---  level of detail". Reducing the number of meshes can be useful for reducing---  drawcall count. Note that you must place the [`VisibilityRange`] component---  on each entity you want to be part of a LOD group, as [`VisibilityRange`]---  isn't automatically propagated down to children.--- ---  A typical use of this feature might look like this:--- ---  | Entity                  | `start_margin` | `end_margin` |---  |-------------------------|----------------|--------------|---  | Root                    | N/A            | N/A          |---  | ├─ High-poly mesh       | [0, 0)         | [20, 25)     |---  | ├─ Low-poly mesh        | [20, 25)       | [70, 75)     |---  | └─ Billboard *imposter* | [70, 75)       | [150, 160)   |--- ---  With this setup, the user will see a high-poly mesh when the camera is---  closer than 20 units. As the camera zooms out, between 20 units to 25 units,---  the high-poly mesh will gradually fade to a low-poly mesh. When the camera---  is 70 to 75 units away, the low-poly mesh will fade to a single textured---  quad. And between 150 and 160 units, the object fades away entirely. Note---  that the `end_margin` of a higher LOD is always identical to the---  `start_margin` of the next lower LOD; this is important for the crossfade---  effect to function properly.
---@field  start_margin ? Range
---@field  end_margin ? Range
---@field  use_aabb ? boolean


---@class RenderLayers
---  Describes which rendering layers an entity belongs to.--- ---  Cameras with this component will only render entities with intersecting---  layers.--- ---  Entities may belong to one or more layers, or no layer at all.--- ---  The [`Default`] instance of `RenderLayers` contains layer `0`, the first layer.--- ---  An entity with this component without any layers is invisible.--- ---  Entities without this component belong to layer `0`.
---@field  [1] ? SmallVec


---@class Screenshot
---  A component that signals to the renderer to capture a screenshot this frame.--- ---  This component should be spawned on a new entity with an observer that will trigger---  with [`ScreenshotCaptured`] when the screenshot is ready.--- ---  Screenshots are captured asynchronously and may not be available immediately after the frame---  that the component is spawned on. The observer should be used to handle the screenshot when it---  is ready.--- ---  Note that the screenshot entity will be despawned after the screenshot is captured and the---  observer is triggered.--- ---  # Usage--- ---  ```---  # use bevy_ecs::prelude::*;---  # use bevy_render::view::screenshot::{save_to_disk, Screenshot};--- ---  fn take_screenshot(mut commands: Commands) {---     commands.spawn(Screenshot::primary_window())---        .observe(save_to_disk("screenshot.png"));---  }---  ```
---@field  [1] ? RenderTarget


---@class ScreenshotCaptured

---@field  [1] ? Image


---@class ColorMaterial
---  A [2d material](Material2d) that renders [2d meshes](crate::Mesh2d) with a texture tinted by a uniform color
---@field  color ? Color
---@field  alpha_mode ? AlphaMode2d
---@field  uv_transform ? Affine2
---@field  texture ? Option


---@class AlphaMode2d
---  Sets how a 2d material's base color alpha channel is used for transparency.---  Currently, this only works with [`Mesh2d`]. Sprites are always transparent.--- ---  This is very similar to [`AlphaMode`](bevy_render::alpha::AlphaMode) but this only applies to 2d meshes.---  We use a separate type because 2d doesn't support all the transparency modes that 3d does.


---@class Anchor
---  How a sprite is positioned relative to its [`Transform`].---  It defaults to `Anchor::Center`.


---@class Sprite
---  Describes a sprite to be rendered to a 2D camera
---@field  image ? Handle
---@field  texture_atlas ? Option
---@field  color ? Color
---@field  flip_x ? boolean
---@field  flip_y ? boolean
---@field  custom_size ? Option
---@field  rect ? Option
---@field  anchor ? Anchor
---@field  image_mode ? SpriteImageMode


---@class SpriteImageMode
---  Controls how the image is altered when scaled.


---@class BorderRect
---  Defines the extents of the border of a rectangle.--- ---  This struct is used to represent thickness or offsets from the edges---  of a rectangle (left, right, top, and bottom), with values increasing inwards.
---@field  left ? number
---@field  right ? number
---@field  top ? number
---@field  bottom ? number


---@class SliceScaleMode
---  Defines how a texture slice scales when resized


---@class TextureSlicer
---  Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes---  without needing to prepare multiple assets. The associated texture will be split into nine portions,---  so that on resize the different portions scale or tile in different ways to keep the texture in proportion.--- ---  For example, when resizing a 9-sliced texture the corners will remain unscaled while the other---  sections will be scaled or tiled.--- ---  See [9-sliced](https://en.wikipedia.org/wiki/9-slice_scaling) textures.
---@field  border ? BorderRect
---@field  center_scale_mode ? SliceScaleMode
---@field  sides_scale_mode ? SliceScaleMode
---@field  max_corner_scale ? number


---@class ReflectableScheduleLabel



---@class AppLifecycle
---  Application lifetime events


---@class CursorEntered
---  An event that is sent whenever the user's cursor enters a window.
---@field  window ? Entity


---@class CursorLeft
---  An event that is sent whenever the user's cursor leaves a window.
---@field  window ? Entity


---@class CursorMoved
---  An event reporting that the mouse cursor has moved inside a window.--- ---  The event is sent only if the cursor is over one of the application's windows.---  It is the translated version of [`WindowEvent::CursorMoved`] from the `winit` crate with the addition of `delta`.--- ---  Not to be confused with the `MouseMotion` event from `bevy_input`.--- ---  Because the range of data is limited by the window area and it may have been transformed by the OS to implement certain effects like acceleration,---  you should not use it for non-cursor-like behavior such as 3D camera control. Please see `MouseMotion` instead.--- ---  [`WindowEvent::CursorMoved`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
---@field  window ? Entity
---@field  position ? Vec2
---@field  delta ? Option


---@class FileDragAndDrop
---  Events related to files being dragged and dropped on a window.


---@class Ime
---  An Input Method Editor event.--- ---  This event is the translated version of the `WindowEvent::Ime` from the `winit` crate.--- ---  It is only sent if IME was enabled on the window with [`Window::ime_enabled`](crate::window::Window::ime_enabled).


---@class RequestRedraw
---  An event that indicates all of the application's windows should be redrawn,---  even if their control flow is set to `Wait` and there have been no window events.


---@class WindowBackendScaleFactorChanged
---  An event that indicates a window's OS-reported scale factor has changed.
---@field  window ? Entity
---@field  scale_factor ? number


---@class WindowCloseRequested
---  An event that is sent whenever the operating systems requests that a window---  be closed. This will be sent when the close button of the window is pressed.--- ---  If the default [`WindowPlugin`] is used, these events are handled---  by closing the corresponding [`Window`].---  To disable this behavior, set `close_when_requested` on the [`WindowPlugin`]---  to `false`.--- ---  [`WindowPlugin`]: crate::WindowPlugin---  [`Window`]: crate::Window
---@field  window ? Entity


---@class WindowClosed
---  An event that is sent whenever a window is closed. This will be sent when---  the window entity loses its [`Window`](crate::window::Window) component or is despawned.
---@field  window ? Entity


---@class WindowClosing
---  An event that is sent whenever a window is closing. This will be sent when---  after a [`WindowCloseRequested`] event is received and the window is in the process of closing.
---@field  window ? Entity


---@class WindowCreated
---  An event that is sent whenever a new window is created.--- ---  To create a new window, spawn an entity with a [`crate::Window`] on it.
---@field  window ? Entity


---@class WindowDestroyed
---  An event that is sent whenever a window is destroyed by the underlying window system.--- ---  Note that if your application only has a single window, this event may be your last chance to---  persist state before the application terminates.
---@field  window ? Entity


---@class WindowEvent
---  Wraps all `bevy_window` and `bevy_input` events in a common enum.--- ---  Read these events with `EventReader<WindowEvent>` if you need to---  access window events in the order they were received from the---  operating system. Otherwise, the event types are individually---  readable with `EventReader<E>` (e.g. `EventReader<KeyboardInput>`).


---@class WindowFocused
---  An event that indicates a window has received or lost focus.
---@field  window ? Entity
---@field  focused ? boolean


---@class WindowMoved
---  An event that is sent when a window is repositioned in physical pixels.
---@field  window ? Entity
---@field  position ? IVec2


---@class WindowOccluded
---  The window has been occluded (completely hidden from view).--- ---  This is different to window visibility as it depends on---  whether the window is closed, minimized, set invisible,---  or fully occluded by another window.--- ---  It is the translated version of [`WindowEvent::Occluded`] from the `winit` crate.--- ---  [`WindowEvent::Occluded`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.Occluded
---@field  window ? Entity
---@field  occluded ? boolean


---@class WindowResized
---  A window event that is sent whenever a window's logical size has changed.
---@field  window ? Entity
---@field  width ? number
---@field  height ? number


---@class WindowScaleFactorChanged
---  An event that indicates a window's scale factor has changed.
---@field  window ? Entity
---@field  scale_factor ? number


---@class WindowThemeChanged
---  An event sent when the system theme changes for a window.--- ---  This event is only sent when the window is relying on the system theme to control its appearance.---  i.e. It is only sent when [`Window::window_theme`](crate::window::Window::window_theme) is `None` and the system theme changes.
---@field  window ? Entity
---@field  theme ? WindowTheme


---@class Monitor
---  Represents an available monitor as reported by the user's operating system, which can be used---  to query information about the display, such as its size, position, and video modes.--- ---  Each monitor corresponds to an entity and can be used to position a monitor using---  [`crate::window::MonitorSelection::Entity`].--- ---  # Warning--- ---  This component is synchronized with `winit` through `bevy_winit`, but is effectively---  read-only as `winit` does not support changing monitor properties.
---@field  name ? Option
---@field  physical_height ? integer
---@field  physical_width ? integer
---@field  physical_position ? IVec2
---@field  refresh_rate_millihertz ? Option
---@field  scale_factor ? number
---@field  video_modes ? Vec


---@class VideoMode
---  Represents a video mode that a monitor supports
---@field  physical_size ? UVec2
---@field  bit_depth ? integer
---@field  refresh_rate_millihertz ? integer


---@class SystemCursorIcon
---  The icon to display for a window.--- ---  Examples of all of these cursors can be found [here](https://www.w3schools.com/cssref/playit.php?filename=playcss_cursor&preval=crosshair).---  This `enum` is simply a copy of a similar `enum` found in [`winit`](https://docs.rs/winit/latest/winit/window/enum.CursorIcon.html).---  `winit`, in turn, is based upon the [CSS3 UI spec](https://www.w3.org/TR/css-ui-3/#cursor).--- ---  See the [`window_settings`] example for usage.--- ---  [`window_settings`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs


---@class CompositeAlphaMode
---  Specifies how the alpha channel of the textures should be handled during compositing, for a [`Window`].


---@class CursorGrabMode
---  Defines if and how the cursor is grabbed by a [`Window`].--- ---  ## Platform-specific--- ---  - **`Windows`** doesn't support [`CursorGrabMode::Locked`]---  - **`macOS`** doesn't support [`CursorGrabMode::Confined`]---  - **`iOS/Android`** don't have cursors.--- ---  Since `Windows` and `macOS` have different [`CursorGrabMode`] support, we first try to set the grab mode that was asked for. If it doesn't work then use the alternate grab mode.


---@class CursorOptions
---  Cursor data for a [`Window`].
---@field  visible ? boolean
---@field  grab_mode ? CursorGrabMode
---@field  hit_test ? boolean


---@class EnabledButtons
---  Specifies which [`Window`] control buttons should be enabled.--- ---  ## Platform-specific--- ---  **`iOS`**, **`Android`**, and the **`Web`** do not have window control buttons.--- ---  On some **`Linux`** environments these values have no effect.
---@field  minimize ? boolean
---@field  maximize ? boolean
---@field  close ? boolean


---@class InternalWindowState
---  Stores internal [`Window`] state that isn't directly accessible.
---@field  minimize_request ? Option
---@field  maximize_request ? Option
---@field  drag_move_request ? boolean
---@field  drag_resize_request ? Option
---@field  physical_cursor_position ? Option


---@class MonitorSelection
---  References a screen monitor.--- ---  Used when centering a [`Window`] on a monitor.


---@class PresentMode
---  Presentation mode for a [`Window`].--- ---  The presentation mode specifies when a frame is presented to the window. The [`Fifo`]---  option corresponds to a traditional `VSync`, where the framerate is capped by the---  display refresh rate. Both [`Immediate`] and [`Mailbox`] are low-latency and are not---  capped by the refresh rate, but may not be available on all platforms. Tearing---  may be observed with [`Immediate`] mode, but will not be observed with [`Mailbox`] or---  [`Fifo`].--- ---  [`AutoVsync`] or [`AutoNoVsync`] will gracefully fallback to [`Fifo`] when unavailable.--- ---  [`Immediate`] or [`Mailbox`] will panic if not supported by the platform.--- ---  [`Fifo`]: PresentMode::Fifo---  [`FifoRelaxed`]: PresentMode::FifoRelaxed---  [`Immediate`]: PresentMode::Immediate---  [`Mailbox`]: PresentMode::Mailbox---  [`AutoVsync`]: PresentMode::AutoVsync---  [`AutoNoVsync`]: PresentMode::AutoNoVsync


---@class PrimaryWindow
---  Marker [`Component`] for the window considered the primary window.--- ---  Currently this is assumed to only exist on 1 entity at a time.--- ---  [`WindowPlugin`](crate::WindowPlugin) will spawn a [`Window`] entity---  with this component if [`primary_window`](crate::WindowPlugin::primary_window)---  is `Some`.


---@class VideoModeSelection
---  References an exclusive fullscreen video mode.--- ---  Used when setting [`WindowMode::Fullscreen`] on a window.


---@class Window
---  The defining [`Component`] for window entities,---  storing information about how it should appear and behave.--- ---  Each window corresponds to an entity, and is uniquely identified by the value of their [`Entity`].---  When the [`Window`] component is added to an entity, a new window will be opened.---  When it is removed or the entity is despawned, the window will close.--- ---  The primary window entity (and the corresponding window) is spawned by default---  by [`WindowPlugin`](crate::WindowPlugin) and is marked with the [`PrimaryWindow`] component.--- ---  This component is synchronized with `winit` through `bevy_winit`:---  it will reflect the current state of the window and can be modified to change this state.--- ---  # Example--- ---  Because this component is synchronized with `winit`, it can be used to perform---  OS-integrated windowing operations. For example, here's a simple system---  to change the window mode:--- ---  ```---  # use bevy_ecs::query::With;---  # use bevy_ecs::system::Query;---  # use bevy_window::{WindowMode, PrimaryWindow, Window, MonitorSelection, VideoModeSelection};---  fn change_window_mode(mut windows: Query<&mut Window, With<PrimaryWindow>>) {---      // Query returns one window typically.---      for mut window in windows.iter_mut() {---          window.mode =---              WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);---      }---  }---  ```
---@field  cursor_options ? CursorOptions
---@field  present_mode ? PresentMode
---@field  mode ? WindowMode
---@field  position ? WindowPosition
---@field  resolution ? WindowResolution
---@field  title ? string
---@field  name ? Option
---@field  composite_alpha_mode ? CompositeAlphaMode
---@field  resize_constraints ? WindowResizeConstraints
---@field  resizable ? boolean
---@field  enabled_buttons ? EnabledButtons
---@field  decorations ? boolean
---@field  transparent ? boolean
---@field  focused ? boolean
---@field  window_level ? WindowLevel
---@field  canvas ? Option
---@field  fit_canvas_to_parent ? boolean
---@field  prevent_default_event_handling ? boolean
---@field  internal ? InternalWindowState
---@field  ime_enabled ? boolean
---@field  ime_position ? Vec2
---@field  window_theme ? Option
---@field  visible ? boolean
---@field  skip_taskbar ? boolean
---@field  clip_children ? boolean
---@field  desired_maximum_frame_latency ? Option
---@field  recognize_pinch_gesture ? boolean
---@field  recognize_rotation_gesture ? boolean
---@field  recognize_doubletap_gesture ? boolean
---@field  recognize_pan_gesture ? Option
---@field  movable_by_window_background ? boolean
---@field  fullsize_content_view ? boolean
---@field  has_shadow ? boolean
---@field  titlebar_shown ? boolean
---@field  titlebar_transparent ? boolean
---@field  titlebar_show_title ? boolean
---@field  titlebar_show_buttons ? boolean
---@field  prefers_home_indicator_hidden ? boolean
---@field  prefers_status_bar_hidden ? boolean


---@class WindowLevel
---  Specifies where a [`Window`] should appear relative to other overlapping windows (on top or under) .--- ---  Levels are groups of windows with respect to their z-position.--- ---  The relative ordering between windows in different window levels is fixed.---  The z-order of windows within the same window level may change dynamically on user interaction.--- ---  ## Platform-specific--- ---  - **iOS / Android / Web / Wayland:** Unsupported.


---@class WindowMode
---  Defines the way a [`Window`] is displayed.


---@class WindowPosition
---  Defines where a [`Window`] should be placed on the screen.


---@class WindowRef
---  Reference to a [`Window`], whether it be a direct link to a specific entity or---  a more vague defaulting choice.


---@class WindowResizeConstraints
---  The size limits on a [`Window`].--- ---  These values are measured in logical pixels (see [`WindowResolution`]), so the user's---  scale factor does affect the size limits on the window.--- ---  Please note that if the window is resizable, then when the window is---  maximized it may have a size outside of these limits. The functionality---  required to disable maximizing is not yet exposed by winit.
---@field  min_width ? number
---@field  min_height ? number
---@field  max_width ? number
---@field  max_height ? number


---@class WindowResolution
---  Controls the size of a [`Window`]--- ---  ## Physical, logical and requested sizes--- ---  There are three sizes associated with a window:---  - the physical size,---    which represents the actual height and width in physical pixels---    the window occupies on the monitor,---  - the logical size,---    which represents the size that should be used to scale elements---    inside the window, measured in logical pixels,---  - the requested size,---    measured in logical pixels, which is the value submitted---    to the API when creating the window, or requesting that it be resized.--- ---  ## Scale factor--- ---  The reason logical size and physical size are separated and can be different---  is to account for the cases where:---  - several monitors have different pixel densities,---  - the user has set up a pixel density preference in its operating system,---  - the Bevy `App` has specified a specific scale factor between both.--- ---  The factor between physical size and logical size can be retrieved with---  [`WindowResolution::scale_factor`].--- ---  For the first two cases, a scale factor is set automatically by the operating---  system through the window backend. You can get it with---  [`WindowResolution::base_scale_factor`].--- ---  For the third case, you can override this automatic scale factor with---  [`WindowResolution::set_scale_factor_override`].--- ---  ## Requested and obtained sizes--- ---  The logical size should be equal to the requested size after creating/resizing,---  when possible.---  The reason the requested size and logical size might be different---  is because the corresponding physical size might exceed limits (either the---  size limits of the monitor, or limits defined in [`WindowResizeConstraints`]).--- ---  Note: The requested size is not kept in memory, for example requesting a size---  too big for the screen, making the logical size different from the requested size,---  and then setting a scale factor that makes the previous requested size within---  the limits of the screen will not get back that previous requested size.
---@field  physical_width ? integer
---@field  physical_height ? integer
---@field  scale_factor_override ? Option
---@field  scale_factor ? number


---@class WindowTheme
---  The [`Window`] theme variant to use.


---@class CursorIcon
---  Insert into a window entity to set the cursor for that window.


---@class NonZeroU32



---@class Cow



---@class Arc



---@class Range



---@class RangeInclusive
