# Types

## Types

| Type | Summary |
| --- | --- |
| `DynamicFunctionMut` | [ A dynamic mutable script function\.](./types/dynamicfunctionmut.md) |
| `FunctionCallContext` | [ The caller context when calling a script function\.  Functions can choose to react to caller prefere](./types/functioncallcontext.md) |
| `ReflectReference` | [ An accessor to a \`dyn PartialReflect\` struct, stores a base ID of the type and a reflection path  s](./types/reflectreference.md) |
| `String` | [A heap allocated string](./types/string.md) |
| `Cow` | [No Documentation 🚧](./types/cow.md) |
| `Arc` | [No Documentation 🚧](./types/arc.md) |
| `Vec<Val<Entity>>` | [No Documentation 🚧](./types/vecvalentity.md) |
| `Vec<Val<ScriptQueryResult>>` | [No Documentation 🚧](./types/vecvalscriptqueryresult.md) |
| `Vec<Val<FunctionInfo>>` | [No Documentation 🚧](./types/vecvalfunctioninfo.md) |
| `Vec<ReflectReference>` | [No Documentation 🚧](./types/vecreflectreference.md) |
| `Vec<FunctionArgInfo>` | [No Documentation 🚧](./types/vecfunctionarginfo.md) |
| `AssetIndex` | [ A generational runtime\-only identifier for a specific \[\`Asset\`\] stored in \[\`Assets\`\]\. This is optim](./types/assetindex.md) |
| `Handle<()>` | [ A strong or weak handle to a specific \[\`Asset\`\]\. If a \[\`Handle\`\] is \[\`Handle::Strong\`\], the \[\`Asset](./types/handle().md) |
| `Handle<TypeId(0xe26d988202a4cf516258a1023452eff8)>` | [ A strong or weak handle to a specific \[\`Asset\`\]\. If a \[\`Handle\`\] is \[\`Handle::Strong\`\], the \[\`Asset](./types/handletypeid(0xe26d988202a4cf516258a1023452eff8).md) |
| `Handle<TypeId(0x3f1592ec7f459f2346f64544a4b444b3)>` | [ A strong or weak handle to a specific \[\`Asset\`\]\. If a \[\`Handle\`\] is \[\`Handle::Strong\`\], the \[\`Asset](./types/handletypeid(0x3f1592ec7f459f2346f64544a4b444b3).md) |
| `AssetId<()>` | [ A unique runtime\-only identifier for an \[\`Asset\`\]\. This is cheap to \[\`Copy\`\]/\[\`Clone\`\] and is not directly tied to the  lifetime of the Asset\. This means it \_can\_ point to an \[\`Asset\`\]](./types/assetid().md) |
| `AssetId<TypeId(0xe26d988202a4cf516258a1023452eff8)>` | [ A unique runtime\-only identifier for an \[\`Asset\`\]\. This is cheap to \[\`Copy\`\]/\[\`Clone\`\] and is not directly tied to the  lifetime of the Asset\. This means it \_can\_ point to an \[\`Asset\`\]](./types/assetidtypeid(0xe26d988202a4cf516258a1023452eff8).md) |
| `AssetId<TypeId(0x3f1592ec7f459f2346f64544a4b444b3)>` | [ A unique runtime\-only identifier for an \[\`Asset\`\]\. This is cheap to \[\`Copy\`\]/\[\`Clone\`\] and is not directly tied to the  lifetime of the Asset\. This means it \_can\_ point to an \[\`Asset\`\]](./types/assetidtypeid(0x3f1592ec7f459f2346f64544a4b444b3).md) |
| `AssetPath` | [ Represents a path to an asset in a "virtual filesystem"\.   Asset paths consist of three main parts:](./types/assetpath.md) |
| `Name` | [ Component used to identify an entity\. Stores a hash for faster comparisons\.   The hash is eagerly r](./types/name.md) |
| `ComponentId` | [ A value which uniquely identifies the type of a \[\`Component\`\] or \[\`Resource\`\] within a  \[\`World\`\]\.   Each time a new \`Component\` type is registered within a \`World\` using  e\.g\. \[\`World::register\_component\`\]](./types/componentid.md) |
| `ComponentTicks` | [ Records when a component or resource was added and when it was last mutably dereferenced \(or added\)](./types/componentticks.md) |
| `Tick` | [ A value that tracks when a system ran relative to other systems\.  This is used to power change dete](./types/tick.md) |
| `Entity` | [ Lightweight identifier of an \[entity\]\(crate::entity\)\.   The identifier is implemented using a \[gene](./types/entity.md) |
| `EntityHash` | [ A \[\`BuildHasher\`\] that results in a \[\`EntityHasher\`\]\.](./types/entityhash.md) |
| `Identifier` | [ A unified identifier for all entity and similar IDs\.   Has the same size as a \`u64\` integer, but th](./types/identifier.md) |
| `RemovedComponentEntity` | [ Wrapper around \[\`Entity\`\] for \[\`RemovedComponents\`\]\.  Internally, \`RemovedComponents\` uses these as](./types/removedcomponententity.md) |
| `SystemIdMarker` | [ Marker \[\`Component\`\]\(bevy\_ecs::component::Component\) for identifying \[\`SystemId\`\] \[\`Entity\`\]s\.](./types/systemidmarker.md) |
| `OnAdd` | [ Trigger emitted when a component is added to an entity\. See \[\`crate::component::ComponentHooks::on\_add\`\]  for more information\.](./types/onadd.md) |
| `OnInsert` | [ Trigger emitted when a component is inserted onto an entity\. See \[\`crate::component::ComponentHooks::on\_insert\`\]  for more information\.](./types/oninsert.md) |
| `OnRemove` | [ Trigger emitted when a component is removed from an entity\. See \[\`crate::component::ComponentHooks::on\_remove\`\]  for more information\.](./types/onremove.md) |
| `OnReplace` | [ Trigger emitted when a component is replaced on an entity\. See \[\`crate::component::ComponentHooks::on\_replace\`\]  for more information\.](./types/onreplace.md) |
| `Children` | [ Contains references to the child entities of this entity\.   Each child must contain a \[\`Parent\`\] component that points back to this entity\.  This component rarely needs to be created manually,  consider using higher level utilities like \[\`BuildChildren::with\_children\`\]](./types/children.md) |
| `Parent` | [ Holds a reference to the parent entity of this entity\.  This component should only be present on en](./types/parent.md) |
| `HierarchyEvent` | [ An \[\`Event\`\] that is fired whenever there is a change in the world's hierarchy\.   \[\`Event\`\]: bevy\_ecs::event::Event](./types/hierarchyevent.md) |
| `ButtonState` | [ The current "press" state of an element](./types/buttonstate.md) |
| `Axis<GamepadInput>` | [ Stores the position data of the input devices of type \`T\`\.   The values are stored as \`f32\`s, using](./types/axisgamepadinput.md) |
| `ButtonInput<GamepadButton>` | [ A "press\-able" input of type \`T\`\.   \#\# Usage   This type can be used as a resource to keep the curr](./types/buttoninputgamepadbutton.md) |
| `AxisSettings` | [ Settings for a \[\`GamepadAxis\`\]\.   It is used inside the \[\`GamepadSettings\`\] to define the sensitivi](./types/axissettings.md) |
| `ButtonAxisSettings` | [ Settings for a \[\`GamepadButton\`\]\.   It is used inside the \[\`GamepadSettings\`\] to define the sensiti](./types/buttonaxissettings.md) |
| `ButtonSettings` | [ Manages settings for gamepad buttons\.   It is used inside \[\`GamepadSettings\`\] to define the threshold for a \[\`GamepadButton\`\]](./types/buttonsettings.md) |
| `Gamepad` | [ Stores a connected gamepad's metadata such as the name and its \[\`GamepadButton\`\] and \[\`GamepadAxis\`](./types/gamepad.md) |
| `GamepadAxis` | [ Represents gamepad input types that are mapped in the range \[\-1\.0, 1\.0\]   \#\# Usage   This is used to determine which axis has changed its value when receiving a  gamepad axis event\. It is also used in the \[\`Gamepad\`\]](./types/gamepadaxis.md) |
| `GamepadAxisChangedEvent` | [ \[\`GamepadAxis\`\] event triggered by an analog state change](./types/gamepadaxischangedevent.md) |
| `GamepadButton` | [ Represents gamepad input types that are mapped in the range \[0\.0, 1\.0\]\.   \#\# Usage   This is used to determine which button has changed its value when receiving gamepad button events  It is also used in the \[\`Gamepad\`\]](./types/gamepadbutton.md) |
| `GamepadButtonChangedEvent` | [ \[\`GamepadButton\`\] event triggered by an analog state change](./types/gamepadbuttonchangedevent.md) |
| `GamepadButtonStateChangedEvent` | [ \[\`GamepadButton\`\] event triggered by a digital state change](./types/gamepadbuttonstatechangedevent.md) |
| `GamepadConnection` | [ The connection status of a gamepad\.](./types/gamepadconnection.md) |
| `GamepadConnectionEvent` | [ A Gamepad connection event\. Created when a connection to a gamepad  is established and when a gamep](./types/gamepadconnectionevent.md) |
| `GamepadEvent` | [ A gamepad event\.   This event type is used over the \[\`GamepadConnectionEvent\`\],  \[\`GamepadButtonChangedEvent\`](./types/gamepadevent.md) |
| `GamepadInput` | [ Encapsulation over \[\`GamepadAxis\`\] and \[\`GamepadButton\`\]](./types/gamepadinput.md) |
| `GamepadRumbleIntensity` | [ The intensity at which a gamepad's force\-feedback motors may rumble\.](./types/gamepadrumbleintensity.md) |
| `GamepadRumbleRequest` | [ An event that controls force\-feedback rumbling of a \[\`Gamepad\`\] \[\`entity\`\]\(Entity\)\.   \# Notes   Doe](./types/gamepadrumblerequest.md) |
| `GamepadSettings` | [ Gamepad settings component\.   \#\# Usage   It is used to create a \`bevy\` component that stores the se](./types/gamepadsettings.md) |
| `RawGamepadAxisChangedEvent` | [ \[\`GamepadAxis\`\] changed event unfiltered by \[\`GamepadSettings\`\]](./types/rawgamepadaxischangedevent.md) |
| `RawGamepadButtonChangedEvent` | [ \[\`GamepadButton\`\] changed event unfiltered by \[\`GamepadSettings\`\]](./types/rawgamepadbuttonchangedevent.md) |
| `RawGamepadEvent` | [ A raw gamepad event\.   This event type is used over the \[\`GamepadConnectionEvent\`\],  \[\`RawGamepadButtonChangedEvent\`](./types/rawgamepadevent.md) |
| `DoubleTapGesture` | [ Double tap gesture\.   \#\# Platform\-specific   \- Only available on \*\*\`macOS\`\*\* and \*\*\`iOS\`\*\*\.  \- On \*](./types/doubletapgesture.md) |
| `PanGesture` | [ Pan gesture\.   \#\# Platform\-specific   \- On \*\*\`iOS\`\*\*, must be enabled first](./types/pangesture.md) |
| `PinchGesture` | [ Two\-finger pinch gesture, often used for magnifications\.   Positive delta values indicate magnifica](./types/pinchgesture.md) |
| `RotationGesture` | [ Two\-finger rotation gesture\.   Positive delta values indicate rotation counterclockwise and  negati](./types/rotationgesture.md) |
| `Key` | [ The logical key code of a \[\`KeyboardInput\`\]\.   \#\# Technical   Its values map 1 to 1 to winit's Key\.](./types/key.md) |
| `KeyCode` | [ The key code of a \[\`KeyboardInput\`\]\.   \#\# Usage   It is used as the generic \`T\` value of an \[\`ButtonInput\`](./types/keycode.md) |
| `KeyboardFocusLost` | [ Gets generated from \`bevy\_winit::winit\_runner\`   Used for clearing all cached states to avoid havin](./types/keyboardfocuslost.md) |
| `KeyboardInput` | [ A keyboard input event\.   This event is the translated version of the \`WindowEvent::KeyboardInput\` ](./types/keyboardinput.md) |
| `NativeKey` | [ Contains the platform\-native logical key identifier, known as keysym\.   Exactly what that means dif](./types/nativekey.md) |
| `NativeKeyCode` | [ Contains the platform\-native physical key identifier   The exact values vary from platform to platf](./types/nativekeycode.md) |
| `AccumulatedMouseMotion` | [ Tracks how much the mouse has moved every frame\.   This resource is reset to zero every frame\.   Th](./types/accumulatedmousemotion.md) |
| `AccumulatedMouseScroll` | [ Tracks how much the mouse has scrolled every frame\.   This resource is reset to zero every frame\.  ](./types/accumulatedmousescroll.md) |
| `MouseButton` | [ A button on a mouse device\.   \#\# Usage   It is used as the generic \`T\` value of an \[\`ButtonInput\`\] to create a \`bevy\`  resource\.   \#\# Updating   The resource is updated inside of the \[\`mouse\_button\_input\_system\`\]](./types/mousebutton.md) |
| `MouseButtonInput` | [ A mouse button input event\.   This event is the translated version of the \`WindowEvent::MouseInput\`](./types/mousebuttoninput.md) |
| `MouseMotion` | [ An event reporting the change in physical position of a pointing device\.   This represents raw, unf](./types/mousemotion.md) |
| `MouseScrollUnit` | [ The scroll unit\.   Describes how a value of a \[\`MouseWheel\`\] event has to be interpreted\.   The value of the event can either be interpreted as the amount of lines or the amount of pixels  to scroll\.](./types/mousescrollunit.md) |
| `MouseWheel` | [ A mouse wheel event\.   This event is the translated version of the \`WindowEvent::MouseWheel\` from t](./types/mousewheel.md) |
| `ForceTouch` | [ A force description of a \[\`Touch\`\] input\.](./types/forcetouch.md) |
| `TouchInput` | [ A touch input event\.   \#\# Logic   Every time the user touches the screen, a new \[\`TouchPhase::Started\`\]](./types/touchinput.md) |
| `TouchPhase` | [ A phase of a \[\`TouchInput\`\]\.   \#\# Usage   It is used to describe the phase of the touch input that is currently active\.  This includes a phase that indicates that a touch input has started or ended,  or that a finger has moved\. There is also a canceled phase that indicates that  the system canceled the tracking of the finger\.](./types/touchphase.md) |
| `Affine3` | [ Reduced\-size version of \`glam::Affine3A\` for use when storage has  significant performance impact\. ](./types/affine3.md) |
| `AspectRatio` | [ An \`AspectRatio\` is the ratio of width to height\.](./types/aspectratio.md) |
| `Aabb2d` | [ A 2D axis\-aligned bounding box, or bounding rectangle](./types/aabb2d.md) |
| `BoundingCircle` | [ A bounding circle](./types/boundingcircle.md) |
| `Aabb3d` | [ A 3D axis\-aligned bounding box](./types/aabb3d.md) |
| `BoundingSphere` | [ A bounding sphere](./types/boundingsphere.md) |
| `AabbCast2d` | [ An intersection test that casts an \[\`Aabb2d\`\] along a ray\.](./types/aabbcast2d.md) |
| `BoundingCircleCast` | [ An intersection test that casts a \[\`BoundingCircle\`\] along a ray\.](./types/boundingcirclecast.md) |
| `RayCast2d` | [ A raycast intersection test for 2D bounding volumes](./types/raycast2d.md) |
| `AabbCast3d` | [ An intersection test that casts an \[\`Aabb3d\`\] along a ray\.](./types/aabbcast3d.md) |
| `BoundingSphereCast` | [ An intersection test that casts a \[\`BoundingSphere\`\] along a ray\.](./types/boundingspherecast.md) |
| `RayCast3d` | [ A raycast intersection test for 3D bounding volumes](./types/raycast3d.md) |
| `CompassOctant` | [ A compass enum with 8 directions\.  \`\`\`text           N \(North\)           ▲      NW   │   NE         ╲ │ ╱  W \(West\) ┼─────► E \(East\)         ╱ │ ╲      SW   │   SE           ▼           S \(South\)  \`](./types/compassoctant.md) |
| `CompassQuadrant` | [ A compass enum with 4 directions\.  \`\`\`text           N \(North\)           ▲           │           │  W \(West\) ┼─────► E \(East\)           │           │           ▼           S \(South\)  \`](./types/compassquadrant.md) |
| `EaseFunction` | [ Curve functions over the \[unit interval\], commonly used for easing transitions\.   \[unit interval\]: \`Interval::UNIT\`](./types/easefunction.md) |
| `Interval` | [ A nonempty closed interval, possibly unbounded in either direction\.   In other words, the interval ](./types/interval.md) |
| `Dir2` | [ A normalized vector pointing in a direction in 2D space](./types/dir2.md) |
| `Dir3` | [ A normalized vector pointing in a direction in 3D space](./types/dir3.md) |
| `Dir3A` | [ A normalized SIMD vector pointing in a direction in 3D space\.   This type stores a 16 byte aligned \[\`Vec3A\`\]](./types/dir3a.md) |
| `FloatOrd` | [ A wrapper for floats that implements \[\`Ord\`\], \[\`Eq\`\], and \[\`Hash\`\] traits\.   This is a work around for the fact that the IEEE 754\-2008 standard,  implemented by Rust's \[\`f32\`\]](./types/floatord.md) |
| `Isometry2d` | [ An isometry in two dimensions, representing a rotation followed by a translation\.  This can often b](./types/isometry2d.md) |
| `Isometry3d` | [ An isometry in three dimensions, representing a rotation followed by a translation\.  This can often](./types/isometry3d.md) |
| `Annulus` | [ A primitive shape formed by the region between two circles, also known as a ring\.](./types/annulus.md) |
| `Arc2d` | [ A primitive representing an arc between two points on a circle\.   An arc has no area\.  If you want ](./types/arc2d.md) |
| `Capsule2d` | [ A 2D capsule primitive, also known as a stadium or pill shape\.   A two\-dimensional capsule is defin](./types/capsule2d.md) |
| `Circle` | [ A circle primitive, representing the set of points some distance from the origin](./types/circle.md) |
| `CircularSector` | [ A primitive representing a circular sector: a pie slice of a circle\.   The segment is positioned so](./types/circularsector.md) |
| `CircularSegment` | [ A primitive representing a circular segment:  the area enclosed by the arc of a circle and its chor](./types/circularsegment.md) |
| `Ellipse` | [ An ellipse primitive, which is like a circle, but the width and height can be different](./types/ellipse.md) |
| `Line2d` | [ An infinite line going through the origin along a direction in 2D space\.   For a finite line: \[\`Segment2d\`\]](./types/line2d.md) |
| `Plane2d` | [ An unbounded plane in 2D space\. It forms a separating surface through the origin,  stretching infin](./types/plane2d.md) |
| `Rectangle` | [ A rectangle primitive, which is like a square, except that the width and height can be different](./types/rectangle.md) |
| `RegularPolygon` | [ A polygon centered on the origin where all vertices lie on a circle, equally far apart\.](./types/regularpolygon.md) |
| `Rhombus` | [ A rhombus primitive, also known as a diamond shape\.  A four sided polygon, centered on the origin, ](./types/rhombus.md) |
| `Segment2d` | [ A segment of a line going through the origin along a direction in 2D space\.](./types/segment2d.md) |
| `Triangle2d` | [ A triangle in 2D space](./types/triangle2d.md) |
| `Capsule3d` | [ A 3D capsule primitive centered on the origin  A three\-dimensional capsule is defined as a surface ](./types/capsule3d.md) |
| `Cone` | [ A cone primitive centered on the midpoint between the tip of the cone and the center of its base\.  ](./types/cone.md) |
| `ConicalFrustum` | [ A conical frustum primitive\.  A conical frustum can be created  by slicing off a section of a cone\.](./types/conicalfrustum.md) |
| `Cuboid` | [ A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not  required ](./types/cuboid.md) |
| `Cylinder` | [ A cylinder primitive centered on the origin](./types/cylinder.md) |
| `InfinitePlane3d` | [ An unbounded plane in 3D space\. It forms a separating surface through the origin,  stretching infin](./types/infiniteplane3d.md) |
| `Line3d` | [ An infinite line going through the origin along a direction in 3D space\.   For a finite line: \[\`Segment3d\`\]](./types/line3d.md) |
| `Plane3d` | [ A bounded plane in 3D space\. It forms a surface starting from the origin with a defined height and ](./types/plane3d.md) |
| `Segment3d` | [ A segment of a line going through the origin along a direction in 3D space\.](./types/segment3d.md) |
| `Sphere` | [ A sphere primitive, representing the set of all points some distance from the origin](./types/sphere.md) |
| `Tetrahedron` | [ A tetrahedron primitive\.](./types/tetrahedron.md) |
| `Torus` | [ A torus primitive, often representing a ring or donut shape  The set of points some distance from a](./types/torus.md) |
| `Triangle3d` | [ A 3D triangle primitive\.](./types/triangle3d.md) |
| `Ray2d` | [ An infinite half\-line starting at \`origin\` and going in \`direction\` in 2D space\.](./types/ray2d.md) |
| `Ray3d` | [ An infinite half\-line starting at \`origin\` and going in \`direction\` in 3D space\.](./types/ray3d.md) |
| `IRect` | [ A rectangle defined by two opposite corners\.   The rectangle is axis aligned, and defined by its mi](./types/irect.md) |
| `Rect` | [ A rectangle defined by two opposite corners\.   The rectangle is axis aligned, and defined by its mi](./types/rect.md) |
| `URect` | [ A rectangle defined by two opposite corners\.   The rectangle is axis aligned, and defined by its mi](./types/urect.md) |
| `Rot2` | [ A counterclockwise 2D rotation\.   \# Example   \`\`\`  \# use approx::assert\_relative\_eq;  \# use bevy\_math::\{Rot2, Vec2\};  use std::f32::consts::PI;   // Create rotations from radians or degrees  let rotation1 = Rot2::radians\(PI / 2\.0\);  let rotation2 = Rot2::degrees\(45\.0\);   // Get the angle back as radians or degrees  assert\_eq\!\(rotation1\.as\_degrees\(\), 90\.0\);  assert\_eq\!\(rotation2\.as\_radians\(\), PI / 4\.0\);   // "Add" rotations together using \`\*\`  assert\_relative\_eq\!\(rotation1 \* rotation2, Rot2::degrees\(135\.0\)\);   // Rotate vectors  assert\_](./types/rot2.md) |
| `Val<Entity>` | [ A wrapper around a value of type \`T\`\.   This can be used to retrieve a value out of a \[\`ScriptValue::Reference\`\]](./types/valentity.md) |
| `Val<ScriptQueryBuilder>` | [ A wrapper around a value of type \`T\`\.   This can be used to retrieve a value out of a \[\`ScriptValue::Reference\`\]](./types/valscriptquerybuilder.md) |
| `Val<ScriptQueryResult>` | [ A wrapper around a value of type \`T\`\.   This can be used to retrieve a value out of a \[\`ScriptValue::Reference\`\]](./types/valscriptqueryresult.md) |
| `Val<FunctionInfo>` | [ A wrapper around a value of type \`T\`\.   This can be used to retrieve a value out of a \[\`ScriptValue::Reference\`\]](./types/valfunctioninfo.md) |
| `Namespace` | [ A namespace for functions](./types/namespace.md) |
| `ScriptComponentRegistration` | [ A registration for a component type\.](./types/scriptcomponentregistration.md) |
| `ScriptQueryBuilder` | [ A builder for a query\.](./types/scriptquerybuilder.md) |
| `ScriptQueryResult` | [ A result from a query\.](./types/scriptqueryresult.md) |
| `ScriptResourceRegistration` | [ A registration for a resource type\.](./types/scriptresourceregistration.md) |
| `ScriptTypeRegistration` | [ A wrapper around a \`TypeRegistration\` that provides additional information about the type\.   This i](./types/scripttyperegistration.md) |
| `ScriptValue` | [ An abstraction of values that can be passed to and from scripts\.  This allows us to re\-use logic be](./types/scriptvalue.md) |
| `FunctionArgInfo` | [ Information about a function argument\.](./types/functionarginfo.md) |
| `FunctionInfo` | [ Information about a function\.](./types/functioninfo.md) |
| `FunctionReturnInfo` | [ Information about a function return value\.](./types/functionreturninfo.md) |
| `Fixed` | [ The fixed timestep game clock following virtual time\.   A specialization of the \[\`Time\`\] structure\. \*\*For method documentation, see  \[\`Time<Fixed>\#impl\-Time<Fixed>\`\]](./types/fixed.md) |
| `Real` | [ Real time clock representing elapsed wall clock time\.   A specialization of the \[\`Time\`\] structure\. \*\*For method documentation, see  \[\`Time<Real>\#impl\-Time<Real>\`\]](./types/real.md) |
| `Stopwatch` | [ A Stopwatch is a struct that tracks elapsed time when started\.   Note that in order to advance the ](./types/stopwatch.md) |
| `Time<()>` | [ A generic clock resource that tracks how much it has advanced since its  previous update and since ](./types/time().md) |
| `Time<Fixed>` | [ A generic clock resource that tracks how much it has advanced since its  previous update and since ](./types/timefixed.md) |
| `Time<Real>` | [ A generic clock resource that tracks how much it has advanced since its  previous update and since ](./types/timereal.md) |
| `Time<Virtual>` | [ A generic clock resource that tracks how much it has advanced since its  previous update and since ](./types/timevirtual.md) |
| `Timer` | [ Tracks elapsed time\. Enters the finished state once \`duration\` is reached\.   Non repeating timers w](./types/timer.md) |
| `TimerMode` | [ Specifies \[\`Timer\`\] behavior\.](./types/timermode.md) |
| `Virtual` | [ The virtual game clock representing game time\.   A specialization of the \[\`Time\`\] structure\. \*\*For method documentation, see  \[\`Time<Virtual>\#impl\-Time<Virtual>\`\]\.\*\*](./types/virtual.md) |
| `GlobalTransform` | [ \[\`GlobalTransform\`\] is an affine transformation from entity\-local coordinates to worldspace coordinates\.   You cannot directly mutate \[\`GlobalTransform\`\]](./types/globaltransform.md) |
| `Transform` | [ Describe the position of an entity\. If the entity has a parent, the position is relative  to its pa](./types/transform.md) |
| `Duration` | [No Documentation 🚧](./types/duration.md) |
| `Instant` | [No Documentation 🚧](./types/instant.md) |
| `HashMap<GamepadAxisAxisSettings, >` | [No Documentation 🚧](./types/hashmapgamepadaxisaxissettings,_.md) |
| `HashMap<GamepadButtonButtonAxisSettings, >` | [No Documentation 🚧](./types/hashmapgamepadbuttonbuttonaxissettings,_.md) |
| `HashMap<GamepadButtonButtonSettings, >` | [No Documentation 🚧](./types/hashmapgamepadbuttonbuttonsettings,_.md) |
| `HashMap<GamepadInputf32, >` | [No Documentation 🚧](./types/hashmapgamepadinputf32,_.md) |
| `HashSet<GamepadButton>` | [No Documentation 🚧](./types/hashsetgamepadbutton.md) |
| `bool` | [A boolean value](./types/bool.md) |
| `char` | [An 8\-bit character](./types/char.md) |
| `TypeId` | [No Documentation 🚧](./types/typeid.md) |
| `RangeFull` | [No Documentation 🚧](./types/rangefull.md) |
| `Option<[u8; 6]>` | [No Documentation 🚧](./types/option[u8;_6].md) |
| `Option<Cow>` | [No Documentation 🚧](./types/optioncow.md) |
| `Option<String>` | [No Documentation 🚧](./types/optionstring.md) |
| `Option<ForceTouch>` | [No Documentation 🚧](./types/optionforcetouch.md) |
| `Option<Val<Entity>>` | [No Documentation 🚧](./types/optionvalentity.md) |
| `Option<ReflectReference>` | [No Documentation 🚧](./types/optionreflectreference.md) |
| `Option<Instant>` | [No Documentation 🚧](./types/optioninstant.md) |
| `Option<char>` | [No Documentation 🚧](./types/optionchar.md) |
| `Option<f32>` | [No Documentation 🚧](./types/optionf32.md) |
| `Option<f64>` | [No Documentation 🚧](./types/optionf64.md) |
| `Option<u16>` | [No Documentation 🚧](./types/optionu16.md) |
| `Option<usize>` | [No Documentation 🚧](./types/optionusize.md) |
| `AtomicBool` | [No Documentation 🚧](./types/atomicbool.md) |
| `AtomicI16` | [No Documentation 🚧](./types/atomici16.md) |
| `AtomicI32` | [No Documentation 🚧](./types/atomici32.md) |
| `AtomicI64` | [No Documentation 🚧](./types/atomici64.md) |
| `AtomicI8` | [No Documentation 🚧](./types/atomici8.md) |
| `AtomicIsize` | [No Documentation 🚧](./types/atomicisize.md) |
| `AtomicU16` | [No Documentation 🚧](./types/atomicu16.md) |
| `AtomicU32` | [No Documentation 🚧](./types/atomicu32.md) |
| `AtomicU64` | [No Documentation 🚧](./types/atomicu64.md) |
| `AtomicU8` | [No Documentation 🚧](./types/atomicu8.md) |
| `AtomicUsize` | [No Documentation 🚧](./types/atomicusize.md) |
| `f32` | [A 32\-bit floating point number](./types/f32.md) |
| `f64` | [A 64\-bit floating point number](./types/f64.md) |
| `Affine2` | [No Documentation 🚧](./types/affine2.md) |
| `Affine3A` | [No Documentation 🚧](./types/affine3a.md) |
| `BVec2` | [No Documentation 🚧](./types/bvec2.md) |
| `BVec3` | [No Documentation 🚧](./types/bvec3.md) |
| `BVec3A` | [No Documentation 🚧](./types/bvec3a.md) |
| `BVec4` | [No Documentation 🚧](./types/bvec4.md) |
| `BVec4A` | [No Documentation 🚧](./types/bvec4a.md) |
| `DAffine2` | [No Documentation 🚧](./types/daffine2.md) |
| `DAffine3` | [No Documentation 🚧](./types/daffine3.md) |
| `DMat2` | [No Documentation 🚧](./types/dmat2.md) |
| `DMat3` | [No Documentation 🚧](./types/dmat3.md) |
| `DMat4` | [No Documentation 🚧](./types/dmat4.md) |
| `DQuat` | [No Documentation 🚧](./types/dquat.md) |
| `DVec2` | [No Documentation 🚧](./types/dvec2.md) |
| `DVec3` | [No Documentation 🚧](./types/dvec3.md) |
| `DVec4` | [No Documentation 🚧](./types/dvec4.md) |
| `EulerRot` | [No Documentation 🚧](./types/eulerrot.md) |
| `I64Vec2` | [No Documentation 🚧](./types/i64vec2.md) |
| `I64Vec3` | [No Documentation 🚧](./types/i64vec3.md) |
| `I64Vec4` | [No Documentation 🚧](./types/i64vec4.md) |
| `IVec2` | [No Documentation 🚧](./types/ivec2.md) |
| `IVec3` | [No Documentation 🚧](./types/ivec3.md) |
| `IVec4` | [No Documentation 🚧](./types/ivec4.md) |
| `Mat2` | [No Documentation 🚧](./types/mat2.md) |
| `Mat3` | [No Documentation 🚧](./types/mat3.md) |
| `Mat3A` | [No Documentation 🚧](./types/mat3a.md) |
| `Mat4` | [No Documentation 🚧](./types/mat4.md) |
| `Quat` | [No Documentation 🚧](./types/quat.md) |
| `U64Vec2` | [No Documentation 🚧](./types/u64vec2.md) |
| `U64Vec3` | [No Documentation 🚧](./types/u64vec3.md) |
| `U64Vec4` | [No Documentation 🚧](./types/u64vec4.md) |
| `UVec2` | [No Documentation 🚧](./types/uvec2.md) |
| `UVec3` | [No Documentation 🚧](./types/uvec3.md) |
| `UVec4` | [No Documentation 🚧](./types/uvec4.md) |
| `Vec2` | [No Documentation 🚧](./types/vec2.md) |
| `Vec3` | [No Documentation 🚧](./types/vec3.md) |
| `Vec3A` | [No Documentation 🚧](./types/vec3a.md) |
| `Vec4` | [No Documentation 🚧](./types/vec4.md) |
| `i128` | [A signed 128\-bit integer](./types/i128.md) |
| `i16` | [A signed 16\-bit integer](./types/i16.md) |
| `i32` | [A signed 32\-bit integer](./types/i32.md) |
| `i64` | [A signed 64\-bit integer](./types/i64.md) |
| `i8` | [A signed 8\-bit integer](./types/i8.md) |
| `isize` | [A signed pointer\-sized integer](./types/isize.md) |
| `SmallVec<TypeId(0x7183d4480fe425f1c7a17e25966b83b3)>` | [No Documentation 🚧](./types/smallvectypeid(0x7183d4480fe425f1c7a17e25966b83b3).md) |
| `SmolStr` | [No Documentation 🚧](./types/smolstr.md) |
| `HashMap<StringScriptValue, >` | [No Documentation 🚧](./types/hashmapstringscriptvalue,_.md) |
| `u128` | [An unsigned 128\-bit integer](./types/u128.md) |
| `u16` | [An unsigned 16\-bit integer](./types/u16.md) |
| `u32` | [An unsigned 32\-bit integer](./types/u32.md) |
| `u64` | [An unsigned 64\-bit integer](./types/u64.md) |
| `u8` | [An unsigned 8\-bit integer](./types/u8.md) |
| `usize` | [An unsigned pointer\-sized integer](./types/usize.md) |
| `Uuid` | [No Documentation 🚧](./types/uuid.md) |