// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction.html">Introduction</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="glossary.html">Glossary</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/installation.html">Installation</a></span></li><li class="chapter-item expanded "><li class="part-title">User Guide</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/managing-scripts.html">Managing Scripts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/running-scripts.html">Running Scripts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/callbacks.html">Callbacks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/contexts.html">Contexts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/customizing-script-contexts.html">Customizing Contexts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Summary/controlling-script-bindings.html">Controlling Script Bindings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptSystems/introduction.html">Script Systems</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptPipeline/pipeline.html">Script Pipeline</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptTooling/introduction.html">Editor Support</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Examples/introduction.html">Examples</a></span></li><li class="chapter-item expanded "><li class="part-title">Scripting Reference</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/introduction.html">Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/namespaces.html">Namespaces</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/references.html">References</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/core-callbacks.html">Core Callbacks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/constructing-arbitrary-types.html">Constructing Arbitrary Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/calling-other-scripts.html">Calling Other Scripts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/events-and-messages.html">Events and Messages</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ScriptingReference/bundles.html">Bundles</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="core_bindings.html">Core Bindings</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/builtins.html">Builtins</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types.html">Types</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_alloc.html">alloc</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cow.html">Cow</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_animation.html">animation</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/activeanimation.html">ActiveAnimation</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationclip.html">AnimationClip</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationeventdata.html">AnimationEventData</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationeventtarget.html">AnimationEventTarget</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationgraph.html">AnimationGraph</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationplayer.html">AnimationPlayer</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/animationtargetid.html">AnimationTargetId</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nodeindex.html">NodeIndex</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/repeatanimation.html">RepeatAnimation</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/timedanimationevent.html">TimedAnimationEvent</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_asset.html">asset</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/arcmutexvaluet.html">ArcMutexValue&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/assetida.html">AssetId&lt;A&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/assetindex.html">AssetIndex</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/assetpath.html">AssetPath</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/handlea.html">Handle&lt;A&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/handletemplatet.html">HandleTemplate&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/loadedfolder.html">LoadedFolder</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/loadeduntypedasset.html">LoadedUntypedAsset</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/renderassetusages.html">RenderAssetUsages</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_audio.html">audio</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/playbackmode.html">PlaybackMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/playbacksettings.html">PlaybackSettings</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/spatialscale.html">SpatialScale</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/volume.html">Volume</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_bevy_platform.html">bevy_platform</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/arctypeid0x948238dede8b5ae50d24942138ff6aed.html">Arc&lt;Unknown&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fixedhasher.html">FixedHasher</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/hashedv_s.html">Hashed&lt;V, S&gt;</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_bevy_shader.html">bevy_shader</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/shader.html">Shader</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_bms.html">bms</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/callbacklabel.html">CallbackLabel</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dynamiccomponent.html">DynamicComponent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/enumvariantref.html">EnumVariantRef</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/functionarginfo.html">FunctionArgInfo</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/functioninfo.html">FunctionInfo</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/functionreturninfo.html">FunctionReturnInfo</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/functionscope.html">FunctionScope</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/interoperror.html">InteropError</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/language.html">Language</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/recipients.html">Recipients</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/reflectableschedulelabel.html">ReflectableScheduleLabel</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/reflectschedule.html">ReflectSchedule</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/reflectsystem.html">ReflectSystem</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/reflectsystemset.html">ReflectSystemSet</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptasset.html">ScriptAsset</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptassetmodifiedmessage.html">ScriptAssetModifiedMessage</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptattachment.html">ScriptAttachment</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptbundleregistration.html">ScriptBundleRegistration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptcallbackmessage.html">ScriptCallbackMessage</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptcomponent.html">ScriptComponent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptcomponentregistration.html">ScriptComponentRegistration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scripteventregistration.html">ScriptEventRegistration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptlifecyclemessage.html">ScriptLifecycleMessage</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptquerybuilder.html">ScriptQueryBuilder</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptqueryresult.html">ScriptQueryResult</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptresourceregistration.html">ScriptResourceRegistration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptruncondition.html">ScriptRunCondition</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptsystembuilder.html">ScriptSystemBuilder</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scripttyperegistration.html">ScriptTypeRegistration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/variadictuple.html">VariadicTuple</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_camera.html">camera</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/camera.html">Camera</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cameraoutputmode.html">CameraOutputMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/clearcolorconfig.html">ClearColorConfig</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/computedcameravalues.html">ComputedCameraValues</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/imagerendertarget.html">ImageRenderTarget</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/inheritedvisibility.html">InheritedVisibility</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/manualtextureviewhandle.html">ManualTextureViewHandle</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/msaawriteback.html">MsaaWriteback</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/orthographicprojection.html">OrthographicProjection</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/renderlayers.html">RenderLayers</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/rendertarget.html">RenderTarget</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/rendertargetinfo.html">RenderTargetInfo</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scalingmode.html">ScalingMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/subcameraview.html">SubCameraView</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/viewport.html">Viewport</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/viewvisibility.html">ViewVisibility</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/visibility.html">Visibility</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_cameravisibilitysystems.html">camera.VisibilitySystems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_color.html">color</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/color.html">Color</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/hsla.html">Hsla</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/hsva.html">Hsva</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/hwba.html">Hwba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/laba.html">Laba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/lcha.html">Lcha</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/linearrgba.html">LinearRgba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/okhsla.html">Okhsla</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/okhsva.html">Okhsva</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/okhwba.html">Okhwba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/oklaba.html">Oklaba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/oklcha.html">Oklcha</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/srgba.html">Srgba</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/xyza.html">Xyza</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_core.html">core</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nonzeroi16.html">NonZeroI16</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nonzerou16.html">NonZeroU16</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nonzerou32.html">NonZeroU32</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/ranget.html">Range&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/typeid.html">TypeId</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_ecs.html">ecs</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/addevent.html">AddEvent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/childof.html">ChildOf</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/despawnevent.html">DespawnEvent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/discardevent.html">DiscardEvent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/entity.html">Entity</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/hashedstr.html">HashedStr</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/insertevent.html">InsertEvent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/name.html">Name</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/removeevent.html">RemoveEvent</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_gizmos.html">gizmos</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gizmos.html">Gizmos</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_input.html">input</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/axist.html">Axis&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/buttoninputt.html">ButtonInput&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/focuscause.html">FocusCause</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gamepad.html">Gamepad</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gamepadaxis.html">GamepadAxis</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gamepadbutton.html">GamepadButton</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gamepadinput.html">GamepadInput</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/inputfocus.html">InputFocus</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/keycode.html">KeyCode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mousebutton.html">MouseButton</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nativekeycode.html">NativeKeyCode</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_inputinputfocussystems.html">input.InputFocusSystems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_math.html">math</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/aabb2d.html">Aabb2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/aabb3d.html">Aabb3d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/affine2.html">Affine2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/affine3a.html">Affine3A</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/annulus.html">Annulus</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/capsule2d.html">Capsule2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/capsule3d.html">Capsule3d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/circle.html">Circle</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/compassoctant.html">CompassOctant</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cone.html">Cone</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cuboid.html">Cuboid</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cylinder.html">Cylinder</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dir2.html">Dir2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dir3.html">Dir3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dvec2.html">DVec2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dvec3.html">DVec3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/easefunction.html">EaseFunction</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/eulerrot.html">EulerRot</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/ivec2.html">IVec2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/ivec3.html">IVec3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/jumpat.html">JumpAt</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mat2.html">Mat2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mat3.html">Mat3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mat3a.html">Mat3A</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mat4.html">Mat4</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/plane3d.html">Plane3d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/quat.html">Quat</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/ray2d.html">Ray2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/ray3d.html">Ray3d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/rect.html">Rect</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/rectangle.html">Rectangle</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/regularpolygon.html">RegularPolygon</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/rot2.html">Rot2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/sphere.html">Sphere</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/torus.html">Torus</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/triangle2d.html">Triangle2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/urect.html">URect</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/uvec2.html">UVec2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/uvec3.html">UVec3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/vec2.html">Vec2</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/vec3.html">Vec3</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/vec3a.html">Vec3A</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/vec4.html">Vec4</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_petgraph.html">petgraph</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/digraphtypeid0xe40a2dc4f630ff71fab97263b8cd7b41_unit_u32.html">DiGraph&lt;Unknown, Unknown, u32&gt;</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_render.html">render</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/alphamode.html">AlphaMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/image.html">Image</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/indices.html">Indices</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/jointaabb.html">JointAabb</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/jointindex.html">JointIndex</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mesh.html">Mesh</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshattributecompressionflags.html">MeshAttributeCompressionFlags</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshextractabledataindices.html">MeshExtractableData&lt;Indices&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshextractabledataallocvecvecallocstringstring.html">MeshExtractableData&lt;Unknown&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshextractabledataallocvecvecbevy_meshmorphmorphattributes.html">MeshExtractableData&lt;Unknown&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/morphattributes.html">MorphAttributes</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nobackgroundmotionvectors.html">NoBackgroundMotionVectors</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/opaquerenderermethod.html">OpaqueRendererMethod</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/parallaxmappingmethod.html">ParallaxMappingMethod</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/readbackonce.html">ReadbackOnce</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/screenshot.html">Screenshot</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/screenshotcaptured.html">ScreenshotCaptured</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/shaderbuffer.html">ShaderBuffer</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/skinnedmeshbounds.html">SkinnedMeshBounds</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/skinnedmeshinversebindposes.html">SkinnedMeshInverseBindposes</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/standardmaterial.html">StandardMaterial</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textureatlas.html">TextureAtlas</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/uvchannel.html">UvChannel</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_smol_str.html">smol_str</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/smolstr.html">SmolStr</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_sprite.html">sprite</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/alphamode2d.html">AlphaMode2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/anchor.html">Anchor</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/borderrect.html">BorderRect</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/colormaterial.html">ColorMaterial</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshmaterial2dcolormaterial.html">MeshMaterial2d&lt;ColorMaterial&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/meshmaterial2dspritemeshmaterial.html">MeshMaterial2d&lt;SpriteMeshMaterial&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/slicescalemode.html">SliceScaleMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/sprite.html">Sprite</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/spritealphamode.html">SpriteAlphaMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/spriteimagemode.html">SpriteImageMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/spritemeshmaterial.html">SpriteMeshMaterial</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/spritescalingmode.html">SpriteScalingMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textureatlaslayout.html">TextureAtlasLayout</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textureslicer.html">TextureSlicer</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/tilemapchunkmaterial.html">TilemapChunkMaterial</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_text.html">text</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/emsize.html">EmSize</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/font.html">Font</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontfeatures.html">FontFeatures</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontfeaturetag.html">FontFeatureTag</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontsize.html">FontSize</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontsmoothing.html">FontSmoothing</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontsource.html">FontSource</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontstyle.html">FontStyle</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontvariations.html">FontVariations</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontvariationtag.html">FontVariationTag</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontweight.html">FontWeight</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fontwidth.html">FontWidth</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/genericfontfamily.html">GenericFontFamily</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/justify.html">Justify</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/linebreak.html">LineBreak</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/remsize.html">RemSize</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/text.html">Text</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/text2d.html">Text2d</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textfont.html">TextFont</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textlayout.html">TextLayout</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/textspan.html">TextSpan</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_time.html">time</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/duration.html">Duration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/fixed.html">Fixed</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/instant.html">Instant</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/real.html">Real</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/stopwatch.html">Stopwatch</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/timet.html">Time&lt;T&gt;</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/timer.html">Timer</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/timermode.html">TimerMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/virtual.html">Virtual</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_transform.html">transform</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/globaltransform.html">GlobalTransform</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/transform.html">Transform</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_transformtransformsystems.html">transform.TransformSystems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_ui.html">ui</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/aligncontent.html">AlignContent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/alignitems.html">AlignItems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/alignself.html">AlignSelf</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/backgroundcolor.html">BackgroundColor</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/bordercolor.html">BorderColor</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/borderradius.html">BorderRadius</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/boxsizing.html">BoxSizing</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/computednode.html">ComputedNode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/cornerradius.html">CornerRadius</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/display.html">Display</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/flexdirection.html">FlexDirection</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/flexwrap.html">FlexWrap</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/globalzindex.html">GlobalZIndex</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gridautoflow.html">GridAutoFlow</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gridplacement.html">GridPlacement</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gridtrack.html">GridTrack</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/gridtrackrepetition.html">GridTrackRepetition</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/imagenode.html">ImageNode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/inlinedirection.html">InlineDirection</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/justifycontent.html">JustifyContent</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/justifyitems.html">JustifyItems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/justifyself.html">JustifySelf</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/maxtracksizingfunction.html">MaxTrackSizingFunction</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/mintracksizingfunction.html">MinTrackSizingFunction</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/node.html">Node</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/nodeimagemode.html">NodeImageMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/outline.html">Outline</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/overflow.html">Overflow</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/overflowaxis.html">OverflowAxis</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/overflowclipmargin.html">OverflowClipMargin</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/positiontype.html">PositionType</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/repeatedgridtrack.html">RepeatedGridTrack</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/resolvedborderradius.html">ResolvedBorderRadius</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/uirect.html">UiRect</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/val.html">Val</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/visualbox.html">VisualBox</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/zindex.html">ZIndex</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_uiuisystems.html">ui.UiSystems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_uuid.html">uuid</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/uuid.html">Uuid</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_wgpu_types.html">wgpu_types</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/blendstate.html">BlendState</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_window.html">window</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/compositealphamode.html">CompositeAlphaMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/enabledbuttons.html">EnabledButtons</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/internalwindowstate.html">InternalWindowState</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/monitorselection.html">MonitorSelection</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/presentmode.html">PresentMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/screenedge.html">ScreenEdge</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/videomode.html">VideoMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/videomodeselection.html">VideoModeSelection</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/window.html">Window</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowlevel.html">WindowLevel</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowmode.html">WindowMode</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowposition.html">WindowPosition</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowref.html">WindowRef</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowresizeconstraints.html">WindowResizeConstraints</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowresolution.html">WindowResolution</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/windowtheme.html">WindowTheme</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_windowcursorsystems.html">window.CursorSystems</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/namespace_root.html">root</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/bool.html">bool</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/char.html">char</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dynamicfunction.html">DynamicFunction</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/dynamicfunctionmut.html">DynamicFunctionMut</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/f32.html">f32</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/f64.html">f64</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/functioncallcontext.html">FunctionCallContext</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/i128.html">i128</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/i16.html">i16</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/i32.html">i32</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/i64.html">i64</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/i8.html">i8</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/isize.html">isize</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/osstring.html">OsString</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/pathbuf.html">PathBuf</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/reflectreference.html">ReflectReference</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/scriptvalue.html">ScriptValue</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/statictyperef.html">StaticTypeRef</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/str.html">str</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/string.html">String</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/u128.html">u128</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/u16.html">u16</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/u32.html">u32</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/u64.html">u64</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/u8.html">u8</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/usize.html">usize</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="core_bindings/types/world.html">World</a></span></li></ol></li></ol></li></ol><li class="chapter-item expanded "><li class="part-title">Developing BMS</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development/introduction.html">Overview</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Development/setup.html">Setup</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Development/writing-bindings.html">Writing Bindings</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development/AddingLanguages/introduction.html">New Languages</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Development/AddingLanguages/evaluating-feasibility.html">Evaluating Feasibility</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="Development/AddingLanguages/necessary-features.html">Necessary Features</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="RequestsForComments/summary.html">RFCs</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="RequestsForComments/1-abandoning_codegen/1-abandoning_codegen.html">Abandoning Codegen</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Release Notes</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ReleaseNotes/guides.html">Release Notes</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.14-to-0.15.html">0.14-to-0.15</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.15.0.html">0.15.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.15-to-0.16.html">0.15-to-0.16</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.16.0.html">0.16.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.16-to-0.17.html">0.16-to-0.17</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.17.0.html">0.17.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.17-to-0.18.html">0.17-to-0.18</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.18.0.html">0.18.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.19.0.html">0.19.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.19-to-0.20.html">0.19-to-0.20</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.20.0.html">0.20.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.22-to-0.23.html">0.22-to-0.23</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.23.0.html">0.23.0</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.23-to-0.24.html">0.23-to-0.24</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="ReleaseNotes/0.24.0.html">0.24.0</a></span></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            // Check both with and without the '.html' suffix to be robust against pretty URLs
            if (link.href.replace(/\.html$/, '') === current_page.replace(/\.html$/, '')
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

