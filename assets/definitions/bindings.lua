---@meta
---@module "World"

---@class World : ReflectReference
--- The ECS world containing all Components, Resources and Systems. Main point of interaction with a Bevy App.
World = {}

---@return ScriptQueryBuilder
function World.query() end

---@param e Entity 
---@return boolean
function World.has_entity(e) end

---@param entity Entity 
---@param registration ScriptComponentRegistration 
--- The resource to add.
---@return nil
function World.add_default_component(entity,registration) end

---@param handle_reference ReflectReference 
---@return boolean
function World.has_asset(handle_reference) end

---@param entity Entity 
--- The entity to retrieve the parent of.
---@return Entity | nil
function World.get_parent(entity) end

---@param registration ScriptResourceRegistration 
--- The registration of the resource to check for.
---@return boolean
function World.has_resource(registration) end

---@param entity Entity 
--- The entity to retrieve the component from.
---@param registration ScriptComponentRegistration 
--- The component to retrieve.
---@return ReflectReference | nil
function World.get_component(entity,registration) end

---@param entity Entity 
--- The entity to check.
---@param registration ScriptComponentRegistration 
--- The component to check for.
---@return boolean
function World.has_component(entity,registration) end

---@param entity Entity 
--- The entity to despawn.
---@return nil
function World.despawn(entity) end

---@param entity Entity 
--- The parent entity to receive children
---@param index integer 
--- The index to insert the children at
---@param children Entity[] 
--- The children entities to insert
---@return nil
function World.insert_children(entity,index,children) end

---@return nil
function World.exit() end

---@param entity Entity 
--- The entity to insert the component into.
---@param registration ScriptComponentRegistration 
--- The component registration of the component to insert.
---@param value ReflectReference 
--- The value of the component to insert. Can be constructed using `construct`
---@return nil
function World.insert_component(entity,registration,value) end

---@param name string 
--- The name of the component type
---@return ScriptComponentRegistration
function World.register_new_component(name) end

---@param registration ScriptResourceRegistration 
--- The resource to remove.
---@return nil
function World.remove_resource(registration) end

---@param entity Entity 
--- The entity to remove the component from.
---@param registration ScriptComponentRegistration 
--- The component to remove.
---@return nil
function World.remove_component(entity,registration) end

---@param entity Entity 
--- The entity to despawn the descendants of.
---@return nil
function World.despawn_descendants(entity) end

---@param handle_reference ReflectReference 
--- The handle to the asset (as a reflect reference).
---@param registration ScriptTypeRegistration 
--- The type registration of the asset type.
---@return ReflectReference | nil
function World.get_asset(handle_reference,registration) end

---@param registration ScriptResourceRegistration 
--- The registration of the resource to retrieve.
---@return ReflectReference | nil
function World.get_resource(registration) end

---@param entity Entity 
--- The entity to despawn recursively.
---@return nil
function World.despawn_recursive(entity) end

---@param entity Entity 
--- The parent entity to receive children
---@param children Entity[] 
--- The children entities to push
---@return nil
function World.push_children(entity,children) end

---@param name string 
--- The name of the schedule to retrieve.
---@return ReflectSchedule | nil
function World.get_schedule_by_name(name) end

---@param entity Entity 
--- The entity to retrieve the children of.
---@return Entity[]
function World.get_children(entity) end

---@param type_name string 
--- The name of the type to retrieve.
---@return ScriptTypeRegistration | ScriptComponentRegistration | ScriptResourceRegistration | nil
function World.get_type_by_name(type_name) end

---@param schedule ReflectSchedule 
--- The schedule to add the system to.
---@param builder ScriptSystemBuilder 
--- The system builder specifying the system and its dependencies.
---@return ReflectSystem
function World.add_system(schedule,builder) end

---@return Entity
function World.spawn() end


---@class ReflectReference : ReflectReference
---  A reference to an arbitrary reflected instance.
--- 
---  The reference can point to either the ECS, or to the allocator.
--- 
---  References are composed of two parts:
---  - The base kind, which specifies where the reference points to
---  - The path, which specifies how to access the value from the base.
--- 
---  Bindings defined on this type, apply to ALL references.
ReflectReference = {}

---@param reference ReflectReference 
--- The reference to remove the value from.
---@param key any 
--- The key to remove the value at.
---@return any
function ReflectReference.remove(reference,key) end

---@param reference ReflectReference 
--- The reference to clear.
---@return nil
function ReflectReference.clear(reference) end

---@param reference ReflectReference 
--- The reference to pop the value from.
---@return any
function ReflectReference.pop(reference) end

---@param reference ReflectReference 
--- The reference to index into.
---@param key any 
--- The key to index with.
---@return any | nil
function ReflectReference.map_get(reference,key) end

---@param reference ReflectReference 
--- The reference to get the variant name of.
---@return string | nil
function ReflectReference.variant_name(reference) end

---@param reference ReflectReference 
--- The reference to get the length of.
---@return integer | nil
function ReflectReference.len(reference) end

---@param reference ReflectReference 
--- The reference to insert the value into.
---@param key any 
--- The index to insert the value at.
---@param value any 
--- The value to insert.
---@return nil
function ReflectReference.insert(reference,key,value) end

---@param reference ReflectReference 
--- The reference to list the functions of.
---@return FunctionInfo[]
function ReflectReference.functions(reference) end

---@param reference ReflectReference 
--- The reference to iterate over.
---@return function
function ReflectReference.iter(reference) end

---@param reference ReflectReference 
--- The reference to display.
---@return string
function ReflectReference.debug(reference) end

---@param reference ReflectReference 
--- The reference to display.
---@return string
function ReflectReference.display(reference) end

---@param reference ReflectReference 
--- The reference to push the value into.
---@param value any 
--- The value to push.
---@return nil
function ReflectReference.push(reference,value) end


---@class ScriptComponentRegistration : ReflectReference
---  A reference to a component type's reflection registration.
--- 
---  In general think of this as a handle to a type.
--- 
---  Not to be confused with script registered dynamic components, although this can point to a script registered component.
---@field  registration ? ScriptTypeRegistration
---@field  component_id ? ComponentId
---@field  is_dynamic_script_component ? boolean
ScriptComponentRegistration = {}

---@param registration ScriptComponentRegistration 
--- The type registration.
---@return string
function ScriptComponentRegistration:type_name(registration) end

---@param registration ScriptComponentRegistration 
--- The type registration.
---@return string
function ScriptComponentRegistration:short_name(registration) end


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
ScriptQueryBuilder = {}

---@param query ScriptQueryBuilder 
--- The query to add the component to
---@param without ScriptComponentRegistration 
---@return ScriptQueryBuilder
function ScriptQueryBuilder:without(query,without) end

---@param query ScriptQueryBuilder 
--- The query to add the component to
---@param components ScriptComponentRegistration 
---@return ScriptQueryBuilder
function ScriptQueryBuilder:component(query,components) end

---@param query ScriptQueryBuilder 
--- The query to build.
---@return ScriptQueryResult[]
function ScriptQueryBuilder.build(query) end

---@param query ScriptQueryBuilder 
--- The query to add the component to
---@param with ScriptComponentRegistration 
---@return ScriptQueryBuilder
function ScriptQueryBuilder:with(query,with) end


---@class ScriptQueryResult : ReflectReference
---  A result from a query.
ScriptQueryResult = {}

---@param query ScriptQueryResult 
--- The query result to retrieve the entity from.
---@return Entity
function ScriptQueryResult:entity(query) end

---@param query ScriptQueryResult 
--- The query result to retrieve the components from.
---@return ReflectReference[]
function ScriptQueryResult:components(query) end


---@class ScriptResourceRegistration : ReflectReference
---  A reference to a resource type's reflection registration.
--- 
---  In general think of this as a handle to a type.
---@field  registration ? ScriptTypeRegistration
---@field  resource_id ? ComponentId
ScriptResourceRegistration = {}

---@param registration ScriptResourceRegistration 
--- The type registration.
---@return string
function ScriptResourceRegistration:short_name(registration) end

---@param registration ScriptResourceRegistration 
--- The type registration.
---@return string
function ScriptResourceRegistration:type_name(registration) end


---@class ScriptTypeRegistration : ReflectReference
---  A reference to a type which is not a `Resource` or `Component`.
--- 
---  In general think of this as a handle to a type.
ScriptTypeRegistration = {}

---@param registration ScriptTypeRegistration 
--- The type registration.
---@return string
function ScriptTypeRegistration:short_name(registration) end

---@param registration ScriptTypeRegistration 
--- The type registration.
---@return string
function ScriptTypeRegistration:type_name(registration) end


---@class ScriptSystemBuilder : ReflectReference
---  A builder for systems living in scripts
ScriptSystemBuilder = {}

---@param builder ScriptSystemBuilder 
--- The system builder to add the resource to.
---@param resource ScriptResourceRegistration 
--- The resource to add.
---@return ScriptSystemBuilder
function ScriptSystemBuilder:resource(builder,resource) end

---@param builder ScriptSystemBuilder 
--- The system builder to add the query to.
---@param query ScriptQueryBuilder 
--- The query to add.
---@return ScriptSystemBuilder
function ScriptSystemBuilder:query(builder,query) end

---@param builder ScriptSystemBuilder 
--- The system builder to add the dependency to.
---@param system ReflectSystem 
--- The system to run before.
---@return ScriptSystemBuilder
function ScriptSystemBuilder:before(builder,system) end

---@param builder ScriptSystemBuilder 
--- The system builder to add the dependency to.
---@param system ReflectSystem 
--- The system to run after.
---@return ScriptSystemBuilder
function ScriptSystemBuilder:after(builder,system) end

---@param builder ScriptSystemBuilder 
--- The system builder to make exclusive.
---@return ScriptSystemBuilder
function ScriptSystemBuilder:exclusive(builder) end


---@class ScriptAttachment : ReflectReference
---  Specifies a unique attachment of a script. These attachments are mapped to [`bevy_mod_scripting_core::ContextKey`]'s depending on the context policy used.
ScriptAttachment = {}

---@param script Handle 
--- The script asset to create the attachment from.
---@return ScriptAttachment
function ScriptAttachment.new_static_script(script) end

---@param entity Entity 
--- The entity to attach the script to.
---@param script Handle 
--- The script asset to attach to the entity.
---@return ScriptAttachment
function ScriptAttachment.new_entity_script(entity,script) end


---@class ReflectSchedule : ReflectReference
---  A reflectable schedule.
---@field  type_path ? string
---@field  label ? ReflectableScheduleLabel
ReflectSchedule = {}

---@param schedule ReflectSchedule 
--- The schedule to retrieve the systems from.
---@return ReflectSystem[]
function ReflectSchedule.systems(schedule) end

---@param schedule ReflectSchedule 
--- The schedule to retrieve the system from.
---@param name string 
--- The identifier or full path of the system to retrieve.
---@return ReflectSystem | nil
function ReflectSchedule.get_system_by_name(schedule,name) end

---@param schedule ReflectSchedule 
--- The schedule to render.
---@return string
function ReflectSchedule.render_dot(schedule) end


---@class ReflectSystem : ReflectReference
---  A reflectable system.
ReflectSystem = {}

---@param system ReflectSystem 
--- The system to retrieve the identifier from.
---@return string
function ReflectSystem:identifier(system) end

---@param system ReflectSystem 
--- The system to retrieve the path from.
---@return string
function ReflectSystem:path(system) end


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
Color = {}

---@param _self Color 
---@return LinearRgba
function Color:to_linear(_self) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param chroma number 
--- Chroma channel. [0.0, 1.5]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.lcha(lightness,chroma,hue,alpha) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param value number 
--- Value channel. [0.0, 1.0]
---@return Color
function Color.hsv(hue,saturation,value) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param chroma number 
--- Chroma channel. [0.0, 1.0]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.oklcha(lightness,chroma,hue,alpha) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param whiteness number 
--- Whiteness channel. [0.0, 1.0]
---@param blackness number 
--- Blackness channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.hwba(hue,whiteness,blackness,alpha) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param a number 
--- Green-red channel. [-1.0, 1.0]
---@param b number 
--- Blue-yellow channel. [-1.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.oklaba(lightness,a,b,alpha) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param a number 
--- a axis. [-1.5, 1.5]
---@param b number 
--- b axis. [-1.5, 1.5]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.laba(lightness,a,b,alpha) end

---@param _self Color 
---@return Srgba
function Color:to_srgba(_self) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.srgba(red,green,blue,alpha) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param a number 
--- Green-red channel. [-1.0, 1.0]
---@param b number 
--- Blue-yellow channel. [-1.0, 1.0]
---@return Color
function Color.oklab(lightness,a,b) end

---@param x number 
--- x-axis. [0.0, 1.0]
---@param y number 
--- y-axis. [0.0, 1.0]
---@param z number 
--- z-axis. [0.0, 1.0]
---@return Color
function Color.xyz(x,y,z) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@return Color
function Color.srgb(red,green,blue) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param chroma number 
--- Chroma channel. [0.0, 1.0]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@return Color
function Color.oklch(lightness,chroma,hue) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@return Color
function Color.linear_rgb(red,green,blue) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param whiteness number 
--- Whiteness channel. [0.0, 1.0]
---@param blackness number 
--- Blackness channel. [0.0, 1.0]
---@return Color
function Color.hwb(hue,whiteness,blackness) end

---@param red integer 
--- Red channel. [0, 255]
---@param green integer 
--- Green channel. [0, 255]
---@param blue integer 
--- Blue channel. [0, 255]
---@return Color
function Color.srgb_u8(red,green,blue) end

---@param _self Color 
---@return Color
function Color:clone(_self) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.hsla(hue,saturation,lightness,alpha) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param value number 
--- Value channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.hsva(hue,saturation,value,alpha) end

---@param _self Color 
---@param other Color 
---@return boolean
function Color:eq(_self,other) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param a number 
--- a axis. [-1.5, 1.5]
---@param b number 
--- b axis. [-1.5, 1.5]
---@return Color
function Color.lab(lightness,a,b) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param chroma number 
--- Chroma channel. [0.0, 1.5]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@return Color
function Color.lch(lightness,chroma,hue) end

---@param x number 
--- x-axis. [0.0, 1.0]
---@param y number 
--- y-axis. [0.0, 1.0]
---@param z number 
--- z-axis. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.xyza(x,y,z,alpha) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Color
function Color.linear_rgba(red,green,blue,alpha) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@return Color
function Color.hsl(hue,saturation,lightness) end

---@param red integer 
--- Red channel. [0, 255]
---@param green integer 
--- Green channel. [0, 255]
---@param blue integer 
--- Blue channel. [0, 255]
---@param alpha integer 
--- Alpha channel. [0, 255]
---@return Color
function Color.srgba_u8(red,green,blue,alpha) end

---@param array number[] 
--- Red, Green and Blue channels. Each channel is in the range [0.0, 1.0]
---@return Color
function Color.srgb_from_array(array) end


---@class Hsla : ReflectReference
---  Color in Hue-Saturation-Lightness (HSL) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).
---  <div>
---  </div>
---@field  hue ? number
---@field  saturation ? number
---@field  lightness ? number
---@field  alpha ? number
Hsla = {}

---@param _self Hsla 
---@param lightness number 
---@return Hsla
function Hsla:with_lightness(_self,lightness) end

---@param _self Hsla 
---@return Hsla
function Hsla:clone(_self) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Hsla
function Hsla.new(hue,saturation,lightness,alpha) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@return Hsla
function Hsla.hsl(hue,saturation,lightness) end

---@param _self Hsla 
---@param saturation number 
---@return Hsla
function Hsla:with_saturation(_self,saturation) end

---@param index integer 
---@return Hsla
function Hsla.sequential_dispersed(index) end

---@param _self Hsla 
---@param other Hsla 
---@return boolean
function Hsla:eq(_self,other) end


---@class Hsva : ReflectReference
---  Color in Hue-Saturation-Value (HSV) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).
---  <div>
---  </div>
---@field  hue ? number
---@field  saturation ? number
---@field  value ? number
---@field  alpha ? number
Hsva = {}

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param value number 
--- Value channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Hsva
function Hsva.new(hue,saturation,value,alpha) end

---@param _self Hsva 
---@param value number 
---@return Hsva
function Hsva:with_value(_self,value) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param saturation number 
--- Saturation channel. [0.0, 1.0]
---@param value number 
--- Value channel. [0.0, 1.0]
---@return Hsva
function Hsva.hsv(hue,saturation,value) end

---@param _self Hsva 
---@return Hsva
function Hsva:clone(_self) end

---@param _self Hsva 
---@param other Hsva 
---@return boolean
function Hsva:eq(_self,other) end

---@param _self Hsva 
---@param saturation number 
---@return Hsva
function Hsva:with_saturation(_self,saturation) end


---@class Hwba : ReflectReference
---  Color in Hue-Whiteness-Blackness (HWB) color space with alpha.
---  Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).
---  <div>
---  </div>
---@field  hue ? number
---@field  whiteness ? number
---@field  blackness ? number
---@field  alpha ? number
Hwba = {}

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param whiteness number 
--- Whiteness channel. [0.0, 1.0]
---@param blackness number 
--- Blackness channel. [0.0, 1.0]
---@return Hwba
function Hwba.hwb(hue,whiteness,blackness) end

---@param _self Hwba 
---@param blackness number 
---@return Hwba
function Hwba:with_blackness(_self,blackness) end

---@param _self Hwba 
---@return Hwba
function Hwba:clone(_self) end

---@param _self Hwba 
---@param other Hwba 
---@return boolean
function Hwba:eq(_self,other) end

---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param whiteness number 
--- Whiteness channel. [0.0, 1.0]
---@param blackness number 
--- Blackness channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Hwba
function Hwba.new(hue,whiteness,blackness,alpha) end

---@param _self Hwba 
---@param whiteness number 
---@return Hwba
function Hwba:with_whiteness(_self,whiteness) end


---@class Laba : ReflectReference
---  Color in LAB color space, with alpha
---  <div>
---  </div>
---@field  lightness ? number
---@field  a ? number
---@field  b ? number
---@field  alpha ? number
Laba = {}

---@param _self Laba 
---@return Laba
function Laba:neg(_self) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param a number 
--- a axis. [-1.5, 1.5]
---@param b number 
--- b axis. [-1.5, 1.5]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Laba
function Laba.new(lightness,a,b,alpha) end

---@param _self Laba 
---@param rhs number 
---@return Laba
function Laba:div(_self,rhs) end

---@param _self Laba 
---@param rhs Laba 
---@return Laba
function Laba:add(_self,rhs) end

---@param _self Laba 
---@param lightness number 
---@return Laba
function Laba:with_lightness(_self,lightness) end

---@param _self Laba 
---@param rhs number 
---@return Laba
function Laba:mul(_self,rhs) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param a number 
--- a axis. [-1.5, 1.5]
---@param b number 
--- b axis. [-1.5, 1.5]
---@return Laba
function Laba.lab(lightness,a,b) end

---@param _self Laba 
---@param other Laba 
---@return boolean
function Laba:eq(_self,other) end

---@param _self Laba 
---@param rhs Laba 
---@return Laba
function Laba:sub(_self,rhs) end

---@param _self Laba 
---@return Laba
function Laba:clone(_self) end


---@class Lcha : ReflectReference
---  Color in LCH color space, with alpha
---  <div>
---  </div>
---@field  lightness ? number
---@field  chroma ? number
---@field  hue ? number
---@field  alpha ? number
Lcha = {}

---@param _self Lcha 
---@param chroma number 
---@return Lcha
function Lcha:with_chroma(_self,chroma) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param chroma number 
--- Chroma channel. [0.0, 1.5]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Lcha
function Lcha.new(lightness,chroma,hue,alpha) end

---@param lightness number 
--- Lightness channel. [0.0, 1.5]
---@param chroma number 
--- Chroma channel. [0.0, 1.5]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@return Lcha
function Lcha.lch(lightness,chroma,hue) end

---@param _self Lcha 
---@param other Lcha 
---@return boolean
function Lcha:eq(_self,other) end

---@param index integer 
---@return Lcha
function Lcha.sequential_dispersed(index) end

---@param _self Lcha 
---@param lightness number 
---@return Lcha
function Lcha:with_lightness(_self,lightness) end

---@param _self Lcha 
---@return Lcha
function Lcha:clone(_self) end


---@class LinearRgba : ReflectReference
---  Linear RGB color with alpha.
---  <div>
---  </div>
---@field  red ? number
---@field  green ? number
---@field  blue ? number
---@field  alpha ? number
LinearRgba = {}

---@param _self LinearRgba 
---@return integer
function LinearRgba:as_u32(_self) end

---@param _self LinearRgba 
---@param other LinearRgba 
---@return boolean
function LinearRgba:eq(_self,other) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@return LinearRgba
function LinearRgba.rgb(red,green,blue) end

---@param _self LinearRgba 
---@param rhs number 
---@return LinearRgba
function LinearRgba:div(_self,rhs) end

---@param red number 
---@param green number 
---@param blue number 
---@param alpha number 
---@return LinearRgba
function LinearRgba.new(red,green,blue,alpha) end

---@param _self LinearRgba 
---@param rhs LinearRgba 
---@return LinearRgba
function LinearRgba:sub(_self,rhs) end

---@param _self LinearRgba 
---@return LinearRgba
function LinearRgba:clone(_self) end

---@param _self LinearRgba 
---@param rhs LinearRgba 
---@return LinearRgba
function LinearRgba:add(_self,rhs) end

---@param _self LinearRgba 
---@param rhs number 
---@return LinearRgba
function LinearRgba:mul(_self,rhs) end

---@param _self LinearRgba 
---@param red number 
---@return LinearRgba
function LinearRgba:with_red(_self,red) end

---@param _self LinearRgba 
---@param green number 
---@return LinearRgba
function LinearRgba:with_green(_self,green) end

---@param _self LinearRgba 
---@return LinearRgba
function LinearRgba:neg(_self) end

---@param _self LinearRgba 
---@param blue number 
---@return LinearRgba
function LinearRgba:with_blue(_self,blue) end


---@class Oklaba : ReflectReference
---  Color in Oklab color space, with alpha
---  <div>
---  </div>
---@field  lightness ? number
---@field  a ? number
---@field  b ? number
---@field  alpha ? number
Oklaba = {}

---@param _self Oklaba 
---@param rhs Oklaba 
---@return Oklaba
function Oklaba:sub(_self,rhs) end

---@param _self Oklaba 
---@return Oklaba
function Oklaba:neg(_self) end

---@param _self Oklaba 
---@param b number 
---@return Oklaba
function Oklaba:with_b(_self,b) end

---@param _self Oklaba 
---@param rhs number 
---@return Oklaba
function Oklaba:div(_self,rhs) end

---@param _self Oklaba 
---@return Oklaba
function Oklaba:clone(_self) end

---@param _self Oklaba 
---@param other Oklaba 
---@return boolean
function Oklaba:eq(_self,other) end

---@param _self Oklaba 
---@param rhs Oklaba 
---@return Oklaba
function Oklaba:add(_self,rhs) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param a number 
--- Green-red channel. [-1.0, 1.0]
---@param b number 
--- Blue-yellow channel. [-1.0, 1.0]
---@return Oklaba
function Oklaba.lab(lightness,a,b) end

---@param _self Oklaba 
---@param rhs number 
---@return Oklaba
function Oklaba:mul(_self,rhs) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param a number 
--- Green-red channel. [-1.0, 1.0]
---@param b number 
--- Blue-yellow channel. [-1.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Oklaba
function Oklaba.new(lightness,a,b,alpha) end

---@param _self Oklaba 
---@param a number 
---@return Oklaba
function Oklaba:with_a(_self,a) end

---@param _self Oklaba 
---@param lightness number 
---@return Oklaba
function Oklaba:with_lightness(_self,lightness) end


---@class Oklcha : ReflectReference
---  Color in Oklch color space, with alpha
---  <div>
---  </div>
---@field  lightness ? number
---@field  chroma ? number
---@field  hue ? number
---@field  alpha ? number
Oklcha = {}

---@param _self Oklcha 
---@param lightness number 
---@return Oklcha
function Oklcha:with_lightness(_self,lightness) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param chroma number 
--- Chroma channel. [0.0, 1.0]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Oklcha
function Oklcha.new(lightness,chroma,hue,alpha) end

---@param lightness number 
--- Lightness channel. [0.0, 1.0]
---@param chroma number 
--- Chroma channel. [0.0, 1.0]
---@param hue number 
--- Hue channel. [0.0, 360.0]
---@return Oklcha
function Oklcha.lch(lightness,chroma,hue) end

---@param _self Oklcha 
---@return Oklcha
function Oklcha:clone(_self) end

---@param index integer 
---@return Oklcha
function Oklcha.sequential_dispersed(index) end

---@param _self Oklcha 
---@param other Oklcha 
---@return boolean
function Oklcha:eq(_self,other) end

---@param _self Oklcha 
---@param chroma number 
---@return Oklcha
function Oklcha:with_chroma(_self,chroma) end


---@class Srgba : ReflectReference
---  Non-linear standard RGB with alpha.
---  <div>
---  </div>
---@field  red ? number
---@field  green ? number
---@field  blue ? number
---@field  alpha ? number
Srgba = {}

---@param _self Srgba 
---@return Srgba
function Srgba:neg(_self) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Srgba
function Srgba.new(red,green,blue,alpha) end

---@param _self Srgba 
---@param rhs number 
---@return Srgba
function Srgba:mul(_self,rhs) end

---@param _self Srgba 
---@param rhs Srgba 
---@return Srgba
function Srgba:add(_self,rhs) end

---@param r integer 
--- Red channel. [0, 255]
---@param g integer 
--- Green channel. [0, 255]
---@param b integer 
--- Blue channel. [0, 255]
---@return Srgba
function Srgba.rgb_u8(r,g,b) end

---@param _self Srgba 
---@param rhs Srgba 
---@return Srgba
function Srgba:sub(_self,rhs) end

---@param red number 
--- Red channel. [0.0, 1.0]
---@param green number 
--- Green channel. [0.0, 1.0]
---@param blue number 
--- Blue channel. [0.0, 1.0]
---@return Srgba
function Srgba.rgb(red,green,blue) end

---@param _self Srgba 
---@return Srgba
function Srgba:clone(_self) end

---@param _self Srgba 
---@param rhs number 
---@return Srgba
function Srgba:div(_self,rhs) end

---@param _self Srgba 
---@param other Srgba 
---@return boolean
function Srgba:eq(_self,other) end

---@param _self Srgba 
---@param green number 
---@return Srgba
function Srgba:with_green(_self,green) end

---@param r integer 
--- Red channel. [0, 255]
---@param g integer 
--- Green channel. [0, 255]
---@param b integer 
--- Blue channel. [0, 255]
---@param a integer 
--- Alpha channel. [0, 255]
---@return Srgba
function Srgba.rgba_u8(r,g,b,a) end

---@param _self Srgba 
---@param blue number 
---@return Srgba
function Srgba:with_blue(_self,blue) end

---@param _self Srgba 
---@return string
function Srgba:to_hex(_self) end

---@param value number 
---@return number
function Srgba.gamma_function(value) end

---@param _self Srgba 
---@param red number 
---@return Srgba
function Srgba:with_red(_self,red) end

---@param value number 
---@return number
function Srgba.gamma_function_inverse(value) end


---@class Xyza : ReflectReference
---  [CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.
---  <div>
---  </div>
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  alpha ? number
Xyza = {}

---@param _self Xyza 
---@param rhs number 
---@return Xyza
function Xyza:div(_self,rhs) end

---@param x number 
--- x-axis. [0.0, 1.0]
---@param y number 
--- y-axis. [0.0, 1.0]
---@param z number 
--- z-axis. [0.0, 1.0]
---@param alpha number 
--- Alpha channel. [0.0, 1.0]
---@return Xyza
function Xyza.new(x,y,z,alpha) end

---@param _self Xyza 
---@param rhs number 
---@return Xyza
function Xyza:mul(_self,rhs) end

---@param _self Xyza 
---@param x number 
---@return Xyza
function Xyza:with_x(_self,x) end

---@param _self Xyza 
---@param rhs Xyza 
---@return Xyza
function Xyza:sub(_self,rhs) end

---@param x number 
--- x-axis. [0.0, 1.0]
---@param y number 
--- y-axis. [0.0, 1.0]
---@param z number 
--- z-axis. [0.0, 1.0]
---@return Xyza
function Xyza.xyz(x,y,z) end

---@param _self Xyza 
---@return Xyza
function Xyza:neg(_self) end

---@param _self Xyza 
---@param other Xyza 
---@return boolean
function Xyza:eq(_self,other) end

---@param _self Xyza 
---@return Xyza
function Xyza:clone(_self) end

---@param _self Xyza 
---@param y number 
---@return Xyza
function Xyza:with_y(_self,y) end

---@param _self Xyza 
---@param z number 
---@return Xyza
function Xyza:with_z(_self,z) end

---@param _self Xyza 
---@param rhs Xyza 
---@return Xyza
function Xyza:add(_self,rhs) end


---@class AutoExposureCompensationCurve : ReflectReference
---  An auto exposure compensation curve.
---  This curve is used to map the average log luminance of a scene to an
---  exposure compensation value, to allow for fine control over the final exposure.
---@field  min_log_lum ? number
---@field  max_log_lum ? number
---@field  min_compensation ? number
---@field  max_compensation ? number
---@field  lut ? [u8; 256]
AutoExposureCompensationCurve = {}

---@param _self AutoExposureCompensationCurve 
---@return AutoExposureCompensationCurve
function AutoExposureCompensationCurve:clone(_self) end


---@class AutoExposure : ReflectReference
---  Component that enables auto exposure for an HDR-enabled 2d or 3d camera.
--- 
---  Auto exposure adjusts the exposure of the camera automatically to
---  simulate the human eye's ability to adapt to different lighting conditions.
--- 
---  Bevy's implementation builds a 64 bin histogram of the scene's luminance,
---  and then adjusts the exposure so that the average brightness of the final
---  render will be middle gray. Because it's using a histogram, some details can
---  be selectively ignored or emphasized. Outliers like shadows and specular
---  highlights can be ignored, and certain areas can be given more (or less)
---  weight based on a mask.
--- 
---  # Usage Notes
--- 
---  **Auto Exposure requires compute shaders and is not compatible with WebGL2.**
---@field  range ? RangeInclusive
---@field  filter ? RangeInclusive
---@field  speed_brighten ? number
---@field  speed_darken ? number
---@field  exponential_transition_distance ? number
---@field  metering_mask ? Handle
---@field  compensation_curve ? Handle
AutoExposure = {}

---@param _self AutoExposure 
---@return AutoExposure
function AutoExposure:clone(_self) end


---@class Bloom : ReflectReference
---  Applies a bloom effect to an HDR-enabled 2d or 3d camera.
--- 
---  Bloom emulates an effect found in real cameras and the human eye,
---  causing halos to appear around very bright parts of the scene.
--- 
---  See also <https://en.wikipedia.org/wiki/Bloom_(shader_effect)>.
--- 
---  # Usage Notes
--- 
---  **Bloom is currently not compatible with WebGL2.**
--- 
---  Often used in conjunction with `bevy_pbr::StandardMaterial::emissive` for 3d meshes.
--- 
---  Bloom is best used alongside a tonemapping function that desaturates bright colors,
---  such as [`crate::tonemapping::Tonemapping::TonyMcMapface`].
--- 
---  Bevy's implementation uses a parametric curve to blend between a set of
---  blurred (lower frequency) images generated from the camera's view.
---  See <https://starlederer.github.io/bloom/> for a visualization of the parametric curve
---  used in Bevy as well as a visualization of the curve's respective scattering profile.
---@field  intensity ? number
---@field  low_frequency_boost ? number
---@field  low_frequency_boost_curvature ? number
---@field  high_pass_frequency ? number
---@field  prefilter ? BloomPrefilter
---@field  composite_mode ? BloomCompositeMode
---@field  max_mip_dimension ? integer
---@field  scale ? Vec2
Bloom = {}

---@param _self Bloom 
---@return Bloom
function Bloom:clone(_self) end


---@class BloomCompositeMode : ReflectReference
BloomCompositeMode = {}

---@param _self BloomCompositeMode 
---@return nil
function BloomCompositeMode:assert_receiver_is_total_eq(_self) end

---@param _self BloomCompositeMode 
---@param other BloomCompositeMode 
---@return boolean
function BloomCompositeMode:eq(_self,other) end

---@param _self BloomCompositeMode 
---@return BloomCompositeMode
function BloomCompositeMode:clone(_self) end


---@class BloomPrefilter : ReflectReference
---  Applies a threshold filter to the input image to extract the brightest
---  regions before blurring them and compositing back onto the original image.
---  These settings are useful when emulating the 1990s-2000s game look.
--- 
---  # Considerations
---  * Changing these settings creates a physically inaccurate image
---  * Changing these settings makes it easy to make the final result look worse
---  * Non-default prefilter settings should be used in conjunction with [`BloomCompositeMode::Additive`]
---@field  threshold ? number
---@field  threshold_softness ? number
BloomPrefilter = {}

---@param _self BloomPrefilter 
---@return BloomPrefilter
function BloomPrefilter:clone(_self) end


---@class ContrastAdaptiveSharpening : ReflectReference
---  Applies a contrast adaptive sharpening (CAS) filter to the camera.
--- 
---  CAS is usually used in combination with shader based anti-aliasing methods
---  such as FXAA or TAA to regain some of the lost detail from the blurring that they introduce.
--- 
---  CAS is designed to adjust the amount of sharpening applied to different areas of an image
---  based on the local contrast. This can help avoid over-sharpening areas with high contrast
---  and under-sharpening areas with low contrast.
--- 
---  To use this, add the [`ContrastAdaptiveSharpening`] component to a 2D or 3D camera.
---@field  enabled ? boolean
---@field  sharpening_strength ? number
---@field  denoise ? boolean
ContrastAdaptiveSharpening = {}

---@param _self ContrastAdaptiveSharpening 
---@return ContrastAdaptiveSharpening
function ContrastAdaptiveSharpening:clone(_self) end


---@class DenoiseCas : ReflectReference
---@field  [1] ? boolean
DenoiseCas = {}

---@param _self DenoiseCas 
---@return DenoiseCas
function DenoiseCas:clone(_self) end


---@class Camera2d : ReflectReference
---  A 2D camera component. Enables the 2D render graph for a [`Camera`].
Camera2d = {}

---@param _self Camera2d 
---@return Camera2d
function Camera2d:clone(_self) end


---@class Camera3d : ReflectReference
---  A 3D camera component. Enables the main 3D render graph for a [`Camera`].
--- 
---  The camera coordinate space is right-handed X-right, Y-up, Z-back.
---  This means "forward" is -Z.
---@field  depth_load_op ? Camera3dDepthLoadOp
---@field  depth_texture_usages ? Camera3dDepthTextureUsage
---@field  screen_space_specular_transmission_steps ? integer
---@field  screen_space_specular_transmission_quality ? ScreenSpaceTransmissionQuality
Camera3d = {}

---@param _self Camera3d 
---@return Camera3d
function Camera3d:clone(_self) end


---@class Camera3dDepthLoadOp : ReflectReference
---  The depth clear operation to perform for the main 3d pass.
Camera3dDepthLoadOp = {}

---@param _self Camera3dDepthLoadOp 
---@return Camera3dDepthLoadOp
function Camera3dDepthLoadOp:clone(_self) end


---@class Camera3dDepthTextureUsage : ReflectReference
---@field  [1] ? integer
Camera3dDepthTextureUsage = {}

---@param _self Camera3dDepthTextureUsage 
---@return Camera3dDepthTextureUsage
function Camera3dDepthTextureUsage:clone(_self) end


---@class ScreenSpaceTransmissionQuality : ReflectReference
---  The quality of the screen space transmission blur effect, applied to whatever's “behind” transmissive
---  objects when their `roughness` is greater than `0.0`.
--- 
---  Higher qualities are more GPU-intensive.
--- 
---  **Note:** You can get better-looking results at any quality level by enabling TAA. See: [`TemporalAntiAliasPlugin`](crate::experimental::taa::TemporalAntiAliasPlugin).
ScreenSpaceTransmissionQuality = {}

---@param _self ScreenSpaceTransmissionQuality 
---@return ScreenSpaceTransmissionQuality
function ScreenSpaceTransmissionQuality:clone(_self) end

---@param _self ScreenSpaceTransmissionQuality 
---@param other ScreenSpaceTransmissionQuality 
---@return boolean
function ScreenSpaceTransmissionQuality:eq(_self,other) end


---@class DepthOfField : ReflectReference
---  A component that enables a [depth of field] postprocessing effect when attached to a [`Camera3d`],
---  simulating the focus of a camera lens.
--- 
---  [depth of field]: https://en.wikipedia.org/wiki/Depth_of_field
---@field  mode ? DepthOfFieldMode
---@field  focal_distance ? number
---@field  sensor_height ? number
---@field  aperture_f_stops ? number
---@field  max_circle_of_confusion_diameter ? number
---@field  max_depth ? number
DepthOfField = {}

---@param _self DepthOfField 
---@return DepthOfField
function DepthOfField:clone(_self) end


---@class DepthOfFieldMode : ReflectReference
---  Controls the appearance of the effect.
DepthOfFieldMode = {}

---@param _self DepthOfFieldMode 
---@return DepthOfFieldMode
function DepthOfFieldMode:clone(_self) end

---@param _self DepthOfFieldMode 
---@param other DepthOfFieldMode 
---@return boolean
function DepthOfFieldMode:eq(_self,other) end


---@class Fxaa : ReflectReference
---  A component for enabling Fast Approximate Anti-Aliasing (FXAA)
---  for a [`bevy_render::camera::Camera`].
---@field  enabled ? boolean
---@field  edge_threshold ? Sensitivity
---@field  edge_threshold_min ? Sensitivity
Fxaa = {}

---@param _self Fxaa 
---@return Fxaa
function Fxaa:clone(_self) end


---@class Sensitivity : ReflectReference
Sensitivity = {}

---@param _self Sensitivity 
---@return Sensitivity
function Sensitivity:clone(_self) end

---@param _self Sensitivity 
---@return nil
function Sensitivity:assert_receiver_is_total_eq(_self) end

---@param _self Sensitivity 
---@param other Sensitivity 
---@return boolean
function Sensitivity:eq(_self,other) end


---@class MotionBlur : ReflectReference
---  A component that enables and configures motion blur when added to a camera.
--- 
---  Motion blur is an effect that simulates how moving objects blur as they change position during
---  the exposure of film, a sensor, or an eyeball.
--- 
---  Because rendering simulates discrete steps in time, we use per-pixel motion vectors to estimate
---  the path of objects between frames. This kind of implementation has some artifacts:
---  - Fast moving objects in front of a stationary object or when in front of empty space, will not
---    have their edges blurred.
---  - Transparent objects do not write to depth or motion vectors, so they cannot be blurred.
--- 
---  Other approaches, such as *A Reconstruction Filter for Plausible Motion Blur* produce more
---  correct results, but are more expensive and complex, and have other kinds of artifacts. This
---  implementation is relatively inexpensive and effective.
--- 
---  # Usage
--- 
---  Add the [`MotionBlur`] component to a camera to enable and configure motion blur for that
---  camera.
--- 
---  ```
---  # use bevy_core_pipeline::{core_3d::Camera3d, motion_blur::MotionBlur};
---  # use bevy_ecs::prelude::*;
---  # fn test(mut commands: Commands) {
---  commands.spawn((
---      Camera3d::default(),
---      MotionBlur::default(),
---  ));
---  # }
---  ````
---@field  shutter_angle ? number
---@field  samples ? integer
MotionBlur = {}

---@param _self MotionBlur 
---@return MotionBlur
function MotionBlur:clone(_self) end


---@class OrderIndependentTransparencySettings : ReflectReference
---  Used to identify which camera will use OIT to render transparent meshes
---  and to configure OIT.
---@field  layer_count ? integer
---@field  alpha_threshold ? number
OrderIndependentTransparencySettings = {}

---@param _self OrderIndependentTransparencySettings 
---@return OrderIndependentTransparencySettings
function OrderIndependentTransparencySettings:clone(_self) end


---@class ChromaticAberration : ReflectReference
---  Adds colored fringes to the edges of objects in the scene.
--- 
---  [Chromatic aberration] simulates the effect when lenses fail to focus all
---  colors of light toward a single point. It causes rainbow-colored streaks to
---  appear, which are especially apparent on the edges of objects. Chromatic
---  aberration is commonly used for collision effects, especially in horror
---  games.
--- 
---  Bevy's implementation is based on that of *Inside* ([Gjøl & Svendsen 2016]).
---  It's based on a customizable lookup texture, which allows for changing the
---  color pattern. By default, the color pattern is simply a 3×1 pixel texture
---  consisting of red, green, and blue, in that order, but you can change it to
---  any image in order to achieve different effects.
--- 
---  [Chromatic aberration]: https://en.wikipedia.org/wiki/Chromatic_aberration
--- 
---  [Gjøl & Svendsen 2016]: https://github.com/playdeadgames/publications/blob/master/INSIDE/rendering_inside_gdc2016.pdf
---@field  color_lut ? Handle
---@field  intensity ? number
---@field  max_samples ? integer
ChromaticAberration = {}

---@param _self ChromaticAberration 
---@return ChromaticAberration
function ChromaticAberration:clone(_self) end


---@class DepthPrepass : ReflectReference
---  If added to a [`crate::prelude::Camera3d`] then depth values will be copied to a separate texture available to the main pass.
DepthPrepass = {}

---@param _self DepthPrepass 
---@return DepthPrepass
function DepthPrepass:clone(_self) end


---@class MotionVectorPrepass : ReflectReference
---  If added to a [`crate::prelude::Camera3d`] then screen space motion vectors will be copied to a separate texture available to the main pass.
MotionVectorPrepass = {}

---@param _self MotionVectorPrepass 
---@return MotionVectorPrepass
function MotionVectorPrepass:clone(_self) end


---@class NormalPrepass : ReflectReference
---  If added to a [`crate::prelude::Camera3d`] then vertex world normals will be copied to a separate texture available to the main pass.
---  Normals will have normal map textures already applied.
NormalPrepass = {}

---@param _self NormalPrepass 
---@return NormalPrepass
function NormalPrepass:clone(_self) end


---@class Skybox : ReflectReference
---  Adds a skybox to a 3D camera, based on a cubemap texture.
--- 
---  Note that this component does not (currently) affect the scene's lighting.
---  To do so, use `EnvironmentMapLight` alongside this component.
--- 
---  See also <https://en.wikipedia.org/wiki/Skybox_(video_games)>.
---@field  image ? Handle
---@field  brightness ? number
---@field  rotation ? Quat
Skybox = {}

---@param _self Skybox 
---@return Skybox
function Skybox:clone(_self) end


---@class Smaa : ReflectReference
---  A component for enabling Subpixel Morphological Anti-Aliasing (SMAA)
---  for a [`bevy_render::camera::Camera`].
---@field  preset ? SmaaPreset
Smaa = {}

---@param _self Smaa 
---@return Smaa
function Smaa:clone(_self) end


---@class SmaaPreset : ReflectReference
---  A preset quality level for SMAA.
--- 
---  Higher values are slower but result in a higher-quality image.
--- 
---  The default value is *high*.
SmaaPreset = {}

---@param _self SmaaPreset 
---@return SmaaPreset
function SmaaPreset:clone(_self) end

---@param _self SmaaPreset 
---@return nil
function SmaaPreset:assert_receiver_is_total_eq(_self) end

---@param _self SmaaPreset 
---@param other SmaaPreset 
---@return boolean
function SmaaPreset:eq(_self,other) end


---@class TemporalAntiAliasing : ReflectReference
---  Component to apply temporal anti-aliasing to a 3D perspective camera.
--- 
---  Temporal anti-aliasing (TAA) is a form of image smoothing/filtering, like
---  multisample anti-aliasing (MSAA), or fast approximate anti-aliasing (FXAA).
---  TAA works by blending (averaging) each frame with the past few frames.
--- 
---  # Tradeoffs
--- 
---  Pros:
---  * Filters more types of aliasing than MSAA, such as textures and singular bright pixels (specular aliasing)
---  * Cost scales with screen/view resolution, unlike MSAA which scales with number of triangles
---  * Greatly increases the quality of stochastic rendering techniques such as SSAO, certain shadow map sampling methods, etc
--- 
---  Cons:
---  * Chance of "ghosting" - ghostly trails left behind moving objects
---  * Thin geometry, lighting detail, or texture lines may flicker noisily or disappear
--- 
---  Because TAA blends past frames with the current frame, when the frames differ too much
---  (such as with fast moving objects or camera cuts), ghosting artifacts may occur.
--- 
---  Artifacts tend to be reduced at higher framerates and rendering resolution.
--- 
---  # Usage Notes
--- 
---  The [`TemporalAntiAliasPlugin`] must be added to your app.
---  Any camera with this component must also disable [`Msaa`] by setting it to [`Msaa::Off`].
--- 
---  [Currently](https://github.com/bevyengine/bevy/issues/8423), TAA cannot be used with [`bevy_render::camera::OrthographicProjection`].
--- 
---  TAA also does not work well with alpha-blended meshes, as it requires depth writing to determine motion.
--- 
---  It is very important that correct motion vectors are written for everything on screen.
---  Failure to do so will lead to ghosting artifacts. For instance, if particle effects
---  are added using a third party library, the library must either:
--- 
---  1. Write particle motion vectors to the motion vectors prepass texture
---  2. Render particles after TAA
--- 
---  If no [`MipBias`] component is attached to the camera, TAA will add a `MipBias(-1.0)` component.
---@field  reset ? boolean
TemporalAntiAliasing = {}

---@param _self TemporalAntiAliasing 
---@return TemporalAntiAliasing
function TemporalAntiAliasing:clone(_self) end


---@class DebandDither : ReflectReference
---  Enables a debanding shader that applies dithering to mitigate color banding in the final image for a given [`Camera`] entity.
DebandDither = {}

---@param _self DebandDither 
---@return DebandDither
function DebandDither:clone(_self) end

---@param _self DebandDither 
---@param other DebandDither 
---@return boolean
function DebandDither:eq(_self,other) end

---@param _self DebandDither 
---@return nil
function DebandDither:assert_receiver_is_total_eq(_self) end


---@class Tonemapping : ReflectReference
---  Optionally enables a tonemapping shader that attempts to map linear input stimulus into a perceptually uniform image for a given [`Camera`] entity.
Tonemapping = {}

---@param _self Tonemapping 
---@return nil
function Tonemapping:assert_receiver_is_total_eq(_self) end

---@param _self Tonemapping 
---@return boolean
function Tonemapping:is_enabled(_self) end

---@param _self Tonemapping 
---@return Tonemapping
function Tonemapping:clone(_self) end

---@param _self Tonemapping 
---@param other Tonemapping 
---@return boolean
function Tonemapping:eq(_self,other) end


---@class ComponentId : ReflectReference
---  A value which uniquely identifies the type of a [`Component`] or [`Resource`] within a
---  [`World`].
--- 
---  Each time a new `Component` type is registered within a `World` using
---  e.g. [`World::register_component`] or [`World::register_component_with_descriptor`]
---  or a Resource with e.g. [`World::init_resource`],
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
---  Given a type `T` which implements [`Component`], the `ComponentId` for `T` can be retrieved
---  from a `World` using [`World::component_id()`] or via [`Components::component_id()`]. Access
---  to the `ComponentId` for a [`Resource`] is available via [`Components::resource_id()`].
---@field  [1] ? integer
ComponentId = {}

---@param index integer 
---@return ComponentId
function ComponentId.new(index) end

---@param _self ComponentId 
---@return ComponentId
function ComponentId:clone(_self) end

---@param _self ComponentId 
---@return nil
function ComponentId:assert_receiver_is_total_eq(_self) end

---@param _self ComponentId 
---@return integer
function ComponentId:index(_self) end

---@param _self ComponentId 
---@param other ComponentId 
---@return boolean
function ComponentId:eq(_self,other) end


---@class ComponentTicks : ReflectReference
---  Records when a component or resource was added and when it was last mutably dereferenced (or added).
---@field  added ? Tick
---@field  changed ? Tick
ComponentTicks = {}

---@param _self ComponentTicks 
---@return ComponentTicks
function ComponentTicks:clone(_self) end

---@param _self ComponentTicks 
---@param last_run Tick 
---@param this_run Tick 
---@return boolean
function ComponentTicks:is_changed(_self,last_run,this_run) end

---@param change_tick Tick 
---@return ComponentTicks
function ComponentTicks.new(change_tick) end

---@param _self ComponentTicks 
---@param last_run Tick 
---@param this_run Tick 
---@return boolean
function ComponentTicks:is_added(_self,last_run,this_run) end

---@param _self ComponentTicks 
---@param change_tick Tick 
---@return nil
function ComponentTicks:set_changed(_self,change_tick) end


---@class Tick : ReflectReference
---  A value that tracks when a system ran relative to other systems.
---  This is used to power change detection.
--- 
---  *Note* that a system that hasn't been run yet has a `Tick` of 0.
---@field  tick ? integer
Tick = {}

---@param _self Tick 
---@return Tick
function Tick:clone(_self) end

---@param _self Tick 
---@return integer
function Tick:get(_self) end

---@param _self Tick 
---@param last_run Tick 
---@param this_run Tick 
---@return boolean
function Tick:is_newer_than(_self,last_run,this_run) end

---@param tick integer 
---@return Tick
function Tick.new(tick) end

---@param _self Tick 
---@param other Tick 
---@return boolean
function Tick:eq(_self,other) end

---@param _self Tick 
---@return nil
function Tick:assert_receiver_is_total_eq(_self) end

---@param _self Tick 
---@param tick integer 
---@return nil
function Tick:set(_self,tick) end


---@class Entity : ReflectReference
---  Lightweight identifier of an [entity](crate::entity).
--- 
---  The identifier is implemented using a [generational index]: a combination of an index and a generation.
---  This allows fast insertion after data removal in an array while minimizing loss of spatial locality.
--- 
---  These identifiers are only valid on the [`World`] it's sourced from. Attempting to use an `Entity` to
---  fetch entity components or metadata from a different world will either fail or return unexpected results.
--- 
---  [generational index]: https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
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
Entity = {}

---@param _self Entity 
---@return integer
function Entity:generation(_self) end

---@param index integer 
---@return Entity
function Entity.from_raw(index) end

---@param bits integer 
---@return Entity
function Entity.from_bits(bits) end

---@param _self Entity 
---@return integer
function Entity:to_bits(_self) end

---@param _self Entity 
---@param other Entity 
---@return boolean
function Entity:eq(_self,other) end

---@param _self Entity 
---@return Entity
function Entity:clone(_self) end

---@param _self Entity 
---@return integer
function Entity:index(_self) end


---@class EntityHash : ReflectReference
---  A [`BuildHasher`] that results in a [`EntityHasher`].
EntityHash = {}

---@param _self EntityHash 
---@return EntityHash
function EntityHash:clone(_self) end


---@class EntityHashSet : ReflectReference
---  A [`HashSet`] pre-configured to use [`EntityHash`] hashing.
---@field  [1] ? HashSet
EntityHashSet = {}

---@param _self EntityHashSet 
---@param other EntityHashSet 
---@return boolean
function EntityHashSet:eq(_self,other) end

---@param n integer 
---@return EntityHashSet
function EntityHashSet.with_capacity(n) end

---@return EntityHashSet
function EntityHashSet.new() end

---@param _self EntityHashSet 
---@return boolean
function EntityHashSet:is_empty(_self) end

---@param _self EntityHashSet 
---@return integer
function EntityHashSet:len(_self) end

---@param _self EntityHashSet 
---@return nil
function EntityHashSet:assert_receiver_is_total_eq(_self) end

---@param _self EntityHashSet 
---@return EntityHashSet
function EntityHashSet:clone(_self) end


---@class DefaultQueryFilters : ReflectReference
---  Default query filters work by excluding entities with certain components from most queries.
--- 
---  If a query does not explicitly mention a given disabling component, it will not include entities with that component.
---  To be more precise, this checks if the query's [`FilteredAccess`] contains the component,
---  and if it does not, adds a [`Without`](crate::prelude::Without) filter for that component to the query.
--- 
---  This resource is initialized in the [`World`] whenever a new world is created,
---  with the [`Disabled`] component as a disabling component.
--- 
---  Note that you can remove default query filters by overwriting the [`DefaultQueryFilters`] resource.
---  This can be useful as a last resort escape hatch, but is liable to break compatibility with other libraries.
--- 
---  See the [module docs](crate::entity_disabling) for more info.
--- 
--- 
---  # Warning
--- 
---  Default query filters are a global setting that affects all queries in the [`World`],
---  and incur a small performance cost for each query.
--- 
---  They can cause significant interoperability issues within the ecosystem,
---  as users must be aware of each disabling component in use.
--- 
---  Think carefully about whether you need to use a new disabling component,
---  and clearly communicate their presence in any libraries you publish.
---@field  disabling ? SmallVec
DefaultQueryFilters = {}

---@param _self DefaultQueryFilters 
---@param component_id ComponentId 
---@return nil
function DefaultQueryFilters:register_disabling_component(_self,component_id) end

---@return DefaultQueryFilters
function DefaultQueryFilters.empty() end


---@class Disabled : ReflectReference
---  A marker component for disabled entities.
--- 
---  Semantically, this component is used to mark entities that are temporarily disabled (typically for gameplay reasons),
---  but will likely be re-enabled at some point.
--- 
---  Like all disabling components, this only disables the entity itself,
---  not its children or other entities that reference it.
---  To disable an entire tree of entities, use [`EntityCommands::insert_recursive`](crate::prelude::EntityCommands::insert_recursive).
--- 
---  Every [`World`] has a default query filter that excludes entities with this component,
---  registered in the [`DefaultQueryFilters`] resource.
---  See [the module docs] for more info.
--- 
---  [the module docs]: crate::entity_disabling
Disabled = {}

---@param _self Disabled 
---@return Disabled
function Disabled:clone(_self) end


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
---  let mut child1 = Entity::PLACEHOLDER;
---  let mut child2 = Entity::PLACEHOLDER;
---  let mut grandchild = Entity::PLACEHOLDER;
---  let root = world.spawn_empty().with_children(|p| {
---      child1 = p.spawn_empty().with_children(|p| {
---          grandchild = p.spawn_empty().id();
---      }).id();
---      child2 = p.spawn_empty().id();
---  }).id();
--- 
---  assert_eq!(&**world.entity(root).get::<Children>().unwrap(), &[child1, child2]);
---  assert_eq!(&**world.entity(child1).get::<Children>().unwrap(), &[grandchild]);
---  ```
--- 
---  [`Relationship`]: crate::relationship::Relationship
---@field  [1] ? Entity
ChildOf = {}

---@param _self ChildOf 
---@return ChildOf
function ChildOf:clone(_self) end

---@param _self ChildOf 
---@return Entity
function ChildOf:get(_self) end

---@param _self ChildOf 
---@return nil
function ChildOf:assert_receiver_is_total_eq(_self) end

---@param _self ChildOf 
---@param other ChildOf 
---@return boolean
function ChildOf:eq(_self,other) end

---@param _self ChildOf 
---@return Entity
function ChildOf:parent(_self) end


---@class Children : ReflectReference
---  Tracks which entities are children of this parent entity.
--- 
---  A [`RelationshipTarget`] collection component that is populated
---  with entities that "target" this entity with the [`ChildOf`] [`Relationship`] component.
--- 
---  Together, these components form the "canonical parent-child hierarchy". See the [`ChildOf`] component for the full
---  description of this relationship and instructions on how to use it.
--- 
---  # Usage
--- 
---  Like all [`RelationshipTarget`] components, this data should not be directly manipulated to avoid desynchronization.
---  Instead, modify the [`ChildOf`] components on the "source" entities.
--- 
---  To access the children of an entity, you can iterate over the [`Children`] component,
---  using the [`IntoIterator`] trait.
---  For more complex access patterns, see the [`RelationshipTarget`] trait.
--- 
---  [`Relationship`]: crate::relationship::Relationship
---  [`RelationshipTarget`]: crate::relationship::RelationshipTarget
---@field  [1] ? Vec
Children = {}

---@param _self Children 
---@param other Children 
---@return boolean
function Children:eq(_self,other) end

---@param _self Children 
---@return nil
function Children:assert_receiver_is_total_eq(_self) end

---@param _self Children 
---@param a_index integer 
---@param b_index integer 
---@return nil
function Children:swap(_self,a_index,b_index) end


---@class Identifier : ReflectReference
---  A unified identifier for all entity and similar IDs.
--- 
---  Has the same size as a `u64` integer, but the layout is split between a 32-bit low
---  segment, a 31-bit high segment, and the significant bit reserved as type flags to denote
---  entity kinds.
Identifier = {}

---@param _self Identifier 
---@return integer
function Identifier:to_bits(_self) end

---@param _self Identifier 
---@return integer
function Identifier:masked_high(_self) end

---@param value integer 
---@return Identifier
function Identifier.from_bits(value) end

---@param _self Identifier 
---@return integer
function Identifier:low(_self) end

---@param _self Identifier 
---@param other Identifier 
---@return boolean
function Identifier:eq(_self,other) end

---@param _self Identifier 
---@return Identifier
function Identifier:clone(_self) end


---@class Name : ReflectReference
---  Component used to identify an entity. Stores a hash for faster comparisons.
--- 
---  The hash is eagerly re-computed upon each update to the name.
--- 
---  [`Name`] should not be treated as a globally unique identifier for entities,
---  as multiple entities can have the same name.  [`Entity`] should be
---  used instead as the default unique identifier.
---@field  hash ? integer
---@field  name ? Cow
Name = {}

---@param _self Name 
---@return Name
function Name:clone(_self) end

---@param _self Name 
---@param other Name 
---@return boolean
function Name:eq(_self,other) end


---@class RemovedComponentEntity : ReflectReference
---  Wrapper around [`Entity`] for [`RemovedComponents`].
---  Internally, `RemovedComponents` uses these as an `Events<RemovedComponentEntity>`.
---@field  [1] ? Entity
RemovedComponentEntity = {}

---@param _self RemovedComponentEntity 
---@return RemovedComponentEntity
function RemovedComponentEntity:clone(_self) end


---@class ButtonState : ReflectReference
---  The current "press" state of an element
ButtonState = {}

---@param _self ButtonState 
---@return ButtonState
function ButtonState:clone(_self) end

---@param _self ButtonState 
---@return boolean
function ButtonState:is_pressed(_self) end

---@param _self ButtonState 
---@param other ButtonState 
---@return boolean
function ButtonState:eq(_self,other) end

---@param _self ButtonState 
---@return nil
function ButtonState:assert_receiver_is_total_eq(_self) end


---@class AxisSettings : ReflectReference
---  Settings for a [`GamepadAxis`].
--- 
---  It is used inside the [`GamepadSettings`] to define the sensitivity range and
---  threshold for an axis.
---  Values that are higher than `livezone_upperbound` will be rounded up to 1.0.
---  Values that are lower than `livezone_lowerbound` will be rounded down to -1.0.
---  Values that are in-between `deadzone_lowerbound` and `deadzone_upperbound` will be rounded to 0.0.
---  Otherwise, values will be linearly rescaled to fit into the sensitivity range.
---  For example, a value that is one fourth of the way from `deadzone_upperbound` to `livezone_upperbound` will be scaled to 0.25.
--- 
---  The valid range is `[-1.0, 1.0]`.
---@field  livezone_upperbound ? number
---@field  deadzone_upperbound ? number
---@field  deadzone_lowerbound ? number
---@field  livezone_lowerbound ? number
---@field  threshold ? number
AxisSettings = {}

---@param _self AxisSettings 
---@return number
function AxisSettings:livezone_upperbound(_self) end

---@param _self AxisSettings 
---@param value number 
---@return number
function AxisSettings:set_livezone_upperbound(_self,value) end

---@param _self AxisSettings 
---@param value number 
---@return number
function AxisSettings:set_deadzone_lowerbound(_self,value) end

---@param _self AxisSettings 
---@return number
function AxisSettings:deadzone_upperbound(_self) end

---@param _self AxisSettings 
---@param other AxisSettings 
---@return boolean
function AxisSettings:eq(_self,other) end

---@param _self AxisSettings 
---@return number
function AxisSettings:deadzone_lowerbound(_self) end

---@param _self AxisSettings 
---@param value number 
---@return number
function AxisSettings:set_livezone_lowerbound(_self,value) end

---@param _self AxisSettings 
---@param raw_value number 
---@return number
function AxisSettings:clamp(_self,raw_value) end

---@param _self AxisSettings 
---@return number
function AxisSettings:livezone_lowerbound(_self) end

---@param _self AxisSettings 
---@return AxisSettings
function AxisSettings:clone(_self) end

---@param _self AxisSettings 
---@return number
function AxisSettings:threshold(_self) end

---@param _self AxisSettings 
---@param value number 
---@return number
function AxisSettings:set_deadzone_upperbound(_self,value) end

---@param _self AxisSettings 
---@param value number 
---@return number
function AxisSettings:set_threshold(_self,value) end


---@class ButtonAxisSettings : ReflectReference
---  Settings for a [`GamepadButton`].
--- 
---  It is used inside the [`GamepadSettings`] to define the sensitivity range and
---  threshold for a button axis.
--- 
---  ## Logic
--- 
---  - Values that are higher than or equal to `high` will be rounded to 1.0.
---  - Values that are lower than or equal to `low` will be rounded to 0.0.
---  - Otherwise, values will not be rounded.
--- 
---  The valid range is from 0.0 to 1.0, inclusive.
---@field  high ? number
---@field  low ? number
---@field  threshold ? number
ButtonAxisSettings = {}

---@param _self ButtonAxisSettings 
---@return ButtonAxisSettings
function ButtonAxisSettings:clone(_self) end


---@class ButtonSettings : ReflectReference
---  Manages settings for gamepad buttons.
--- 
---  It is used inside [`GamepadSettings`] to define the threshold for a [`GamepadButton`]
---  to be considered pressed or released. A button is considered pressed if the `press_threshold`
---  value is surpassed and released if the `release_threshold` value is undercut.
--- 
---  Allowed values: `0.0 <= ``release_threshold`` <= ``press_threshold`` <= 1.0`
---@field  press_threshold ? number
---@field  release_threshold ? number
ButtonSettings = {}

---@param _self ButtonSettings 
---@param value number 
---@return boolean
function ButtonSettings:is_released(_self,value) end

---@param _self ButtonSettings 
---@param value number 
---@return number
function ButtonSettings:set_press_threshold(_self,value) end

---@param _self ButtonSettings 
---@param other ButtonSettings 
---@return boolean
function ButtonSettings:eq(_self,other) end

---@param _self ButtonSettings 
---@param value number 
---@return boolean
function ButtonSettings:is_pressed(_self,value) end

---@param _self ButtonSettings 
---@return number
function ButtonSettings:release_threshold(_self) end

---@param _self ButtonSettings 
---@param value number 
---@return number
function ButtonSettings:set_release_threshold(_self,value) end

---@param _self ButtonSettings 
---@return ButtonSettings
function ButtonSettings:clone(_self) end

---@param _self ButtonSettings 
---@return number
function ButtonSettings:press_threshold(_self) end


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
---@field  vendor_id ? Option
---@field  product_id ? Option
---@field  digital ? ButtonInput
---@field  analog ? Axis
Gamepad = {}

---@param _self Gamepad 
---@return Vec2
function Gamepad:right_stick(_self) end

---@param _self Gamepad 
---@return Vec2
function Gamepad:dpad(_self) end

---@param _self Gamepad 
---@return integer | nil
function Gamepad:vendor_id(_self) end

---@param _self Gamepad 
---@param button_type GamepadButton 
---@return boolean
function Gamepad:just_released(_self,button_type) end

---@param _self Gamepad 
---@param button_type GamepadButton 
---@return boolean
function Gamepad:pressed(_self,button_type) end

---@param _self Gamepad 
---@return Vec2
function Gamepad:left_stick(_self) end

---@param _self Gamepad 
---@param button_type GamepadButton 
---@return boolean
function Gamepad:just_pressed(_self,button_type) end

---@param _self Gamepad 
---@return integer | nil
function Gamepad:product_id(_self) end


---@class GamepadAxis : ReflectReference
---  Represents gamepad input types that are mapped in the range [-1.0, 1.0].
--- 
---  ## Usage
--- 
---  This is used to determine which axis has changed its value when receiving a
---  gamepad axis event. It is also used in the [`Gamepad`] component.
GamepadAxis = {}

---@param _self GamepadAxis 
---@return nil
function GamepadAxis:assert_receiver_is_total_eq(_self) end

---@param _self GamepadAxis 
---@param other GamepadAxis 
---@return boolean
function GamepadAxis:eq(_self,other) end

---@param _self GamepadAxis 
---@return GamepadAxis
function GamepadAxis:clone(_self) end


---@class GamepadAxisChangedEvent : ReflectReference
---  [`GamepadAxis`] event triggered by an analog state change.
---@field  entity ? Entity
---@field  axis ? GamepadAxis
---@field  value ? number
GamepadAxisChangedEvent = {}

---@param entity Entity 
---@param axis GamepadAxis 
---@param value number 
---@return GamepadAxisChangedEvent
function GamepadAxisChangedEvent.new(entity,axis,value) end

---@param _self GamepadAxisChangedEvent 
---@return GamepadAxisChangedEvent
function GamepadAxisChangedEvent:clone(_self) end

---@param _self GamepadAxisChangedEvent 
---@param other GamepadAxisChangedEvent 
---@return boolean
function GamepadAxisChangedEvent:eq(_self,other) end


---@class GamepadButton : ReflectReference
---  Represents gamepad input types that are mapped in the range [0.0, 1.0].
--- 
---  ## Usage
--- 
---  This is used to determine which button has changed its value when receiving gamepad button events.
---  It is also used in the [`Gamepad`] component.
GamepadButton = {}

---@param _self GamepadButton 
---@return GamepadButton
function GamepadButton:clone(_self) end

---@param _self GamepadButton 
---@return nil
function GamepadButton:assert_receiver_is_total_eq(_self) end

---@param _self GamepadButton 
---@param other GamepadButton 
---@return boolean
function GamepadButton:eq(_self,other) end


---@class GamepadButtonChangedEvent : ReflectReference
---  [`GamepadButton`] event triggered by an analog state change.
---@field  entity ? Entity
---@field  button ? GamepadButton
---@field  state ? ButtonState
---@field  value ? number
GamepadButtonChangedEvent = {}

---@param entity Entity 
---@param button GamepadButton 
---@param state ButtonState 
---@param value number 
---@return GamepadButtonChangedEvent
function GamepadButtonChangedEvent.new(entity,button,state,value) end

---@param _self GamepadButtonChangedEvent 
---@param other GamepadButtonChangedEvent 
---@return boolean
function GamepadButtonChangedEvent:eq(_self,other) end

---@param _self GamepadButtonChangedEvent 
---@return GamepadButtonChangedEvent
function GamepadButtonChangedEvent:clone(_self) end


---@class GamepadButtonStateChangedEvent : ReflectReference
---  [`GamepadButton`] event triggered by a digital state change.
---@field  entity ? Entity
---@field  button ? GamepadButton
---@field  state ? ButtonState
GamepadButtonStateChangedEvent = {}

---@param _self GamepadButtonStateChangedEvent 
---@param other GamepadButtonStateChangedEvent 
---@return boolean
function GamepadButtonStateChangedEvent:eq(_self,other) end

---@param entity Entity 
---@param button GamepadButton 
---@param state ButtonState 
---@return GamepadButtonStateChangedEvent
function GamepadButtonStateChangedEvent.new(entity,button,state) end

---@param _self GamepadButtonStateChangedEvent 
---@return GamepadButtonStateChangedEvent
function GamepadButtonStateChangedEvent:clone(_self) end

---@param _self GamepadButtonStateChangedEvent 
---@return nil
function GamepadButtonStateChangedEvent:assert_receiver_is_total_eq(_self) end


---@class GamepadConnection : ReflectReference
---  The connection status of a gamepad.
GamepadConnection = {}

---@param _self GamepadConnection 
---@return GamepadConnection
function GamepadConnection:clone(_self) end

---@param _self GamepadConnection 
---@param other GamepadConnection 
---@return boolean
function GamepadConnection:eq(_self,other) end


---@class GamepadConnectionEvent : ReflectReference
---  A Gamepad connection event. Created when a connection to a gamepad
---  is established and when a gamepad is disconnected.
---@field  gamepad ? Entity
---@field  connection ? GamepadConnection
GamepadConnectionEvent = {}

---@param gamepad Entity 
---@param connection GamepadConnection 
---@return GamepadConnectionEvent
function GamepadConnectionEvent.new(gamepad,connection) end

---@param _self GamepadConnectionEvent 
---@return boolean
function GamepadConnectionEvent:disconnected(_self) end

---@param _self GamepadConnectionEvent 
---@return GamepadConnectionEvent
function GamepadConnectionEvent:clone(_self) end

---@param _self GamepadConnectionEvent 
---@param other GamepadConnectionEvent 
---@return boolean
function GamepadConnectionEvent:eq(_self,other) end

---@param _self GamepadConnectionEvent 
---@return boolean
function GamepadConnectionEvent:connected(_self) end


---@class GamepadEvent : ReflectReference
---  A gamepad event.
--- 
---  This event type is used over the [`GamepadConnectionEvent`],
---  [`GamepadButtonChangedEvent`] and [`GamepadAxisChangedEvent`] when
---  the in-frame relative ordering of events is important.
--- 
---  This event is produced by `bevy_input`.
GamepadEvent = {}

---@param _self GamepadEvent 
---@return GamepadEvent
function GamepadEvent:clone(_self) end

---@param _self GamepadEvent 
---@param other GamepadEvent 
---@return boolean
function GamepadEvent:eq(_self,other) end


---@class GamepadInput : ReflectReference
---  Encapsulation over [`GamepadAxis`] and [`GamepadButton`].
GamepadInput = {}

---@param _self GamepadInput 
---@return GamepadInput
function GamepadInput:clone(_self) end

---@param _self GamepadInput 
---@param other GamepadInput 
---@return boolean
function GamepadInput:eq(_self,other) end

---@param _self GamepadInput 
---@return nil
function GamepadInput:assert_receiver_is_total_eq(_self) end


---@class GamepadRumbleIntensity : ReflectReference
---  The intensity at which a gamepad's force-feedback motors may rumble.
---@field  strong_motor ? number
---@field  weak_motor ? number
GamepadRumbleIntensity = {}

---@param _self GamepadRumbleIntensity 
---@param other GamepadRumbleIntensity 
---@return boolean
function GamepadRumbleIntensity:eq(_self,other) end

---@param _self GamepadRumbleIntensity 
---@return GamepadRumbleIntensity
function GamepadRumbleIntensity:clone(_self) end

---@param intensity number 
---@return GamepadRumbleIntensity
function GamepadRumbleIntensity.strong_motor(intensity) end

---@param intensity number 
---@return GamepadRumbleIntensity
function GamepadRumbleIntensity.weak_motor(intensity) end


---@class GamepadRumbleRequest : ReflectReference
---  An event that controls force-feedback rumbling of a [`Gamepad`] [`entity`](Entity).
--- 
---  # Notes
--- 
---  Does nothing if the gamepad or platform does not support rumble.
--- 
---  # Example
--- 
---  ```
---  # use bevy_input::gamepad::{Gamepad, GamepadRumbleRequest, GamepadRumbleIntensity};
---  # use bevy_ecs::prelude::{EventWriter, Res, Query, Entity, With};
---  # use core::time::Duration;
---  fn rumble_gamepad_system(
---      mut rumble_requests: EventWriter<GamepadRumbleRequest>,
---      gamepads: Query<Entity, With<Gamepad>>,
---  ) {
---      for entity in gamepads.iter() {
---          rumble_requests.write(GamepadRumbleRequest::Add {
---              gamepad: entity,
---              intensity: GamepadRumbleIntensity::MAX,
---              duration: Duration::from_secs_f32(0.5),
---          });
---      }
---  }
---  ```
GamepadRumbleRequest = {}

---@param _self GamepadRumbleRequest 
---@return GamepadRumbleRequest
function GamepadRumbleRequest:clone(_self) end

---@param _self GamepadRumbleRequest 
---@return Entity
function GamepadRumbleRequest:gamepad(_self) end


---@class GamepadSettings : ReflectReference
---  Gamepad settings component.
--- 
---  ## Usage
--- 
---  It is used to create a `bevy` component that stores the settings of [`GamepadButton`] and [`GamepadAxis`] in [`Gamepad`].
---  If no user defined [`ButtonSettings`], [`AxisSettings`], or [`ButtonAxisSettings`]
---  are defined, the default settings of each are used as a fallback accordingly.
--- 
---  ## Note
--- 
---  The [`GamepadSettings`] are used to determine when raw gamepad events
---  should register. Events that don't meet the change thresholds defined in [`GamepadSettings`]
---  will not register. To modify these settings, mutate the corresponding component.
---@field  default_button_settings ? ButtonSettings
---@field  default_axis_settings ? AxisSettings
---@field  default_button_axis_settings ? ButtonAxisSettings
---@field  button_settings ? HashMap
---@field  axis_settings ? HashMap
---@field  button_axis_settings ? HashMap
GamepadSettings = {}

---@param _self GamepadSettings 
---@return GamepadSettings
function GamepadSettings:clone(_self) end


---@class RawGamepadAxisChangedEvent : ReflectReference
---  [`GamepadAxis`] changed event unfiltered by [`GamepadSettings`].
---@field  gamepad ? Entity
---@field  axis ? GamepadAxis
---@field  value ? number
RawGamepadAxisChangedEvent = {}

---@param gamepad Entity 
---@param axis_type GamepadAxis 
---@param value number 
---@return RawGamepadAxisChangedEvent
function RawGamepadAxisChangedEvent.new(gamepad,axis_type,value) end

---@param _self RawGamepadAxisChangedEvent 
---@param other RawGamepadAxisChangedEvent 
---@return boolean
function RawGamepadAxisChangedEvent:eq(_self,other) end

---@param _self RawGamepadAxisChangedEvent 
---@return RawGamepadAxisChangedEvent
function RawGamepadAxisChangedEvent:clone(_self) end


---@class RawGamepadButtonChangedEvent : ReflectReference
---  [`GamepadButton`] changed event unfiltered by [`GamepadSettings`].
---@field  gamepad ? Entity
---@field  button ? GamepadButton
---@field  value ? number
RawGamepadButtonChangedEvent = {}

---@param gamepad Entity 
---@param button_type GamepadButton 
---@param value number 
---@return RawGamepadButtonChangedEvent
function RawGamepadButtonChangedEvent.new(gamepad,button_type,value) end

---@param _self RawGamepadButtonChangedEvent 
---@return RawGamepadButtonChangedEvent
function RawGamepadButtonChangedEvent:clone(_self) end

---@param _self RawGamepadButtonChangedEvent 
---@param other RawGamepadButtonChangedEvent 
---@return boolean
function RawGamepadButtonChangedEvent:eq(_self,other) end


---@class RawGamepadEvent : ReflectReference
---  A raw gamepad event.
--- 
---  This event type is used over the [`GamepadConnectionEvent`],
---  [`RawGamepadButtonChangedEvent`] and [`RawGamepadAxisChangedEvent`] when
---  the in-frame relative ordering of events is important.
--- 
---  This event type is used by `bevy_input` to feed its components.
RawGamepadEvent = {}

---@param _self RawGamepadEvent 
---@return RawGamepadEvent
function RawGamepadEvent:clone(_self) end

---@param _self RawGamepadEvent 
---@param other RawGamepadEvent 
---@return boolean
function RawGamepadEvent:eq(_self,other) end


---@class DoubleTapGesture : ReflectReference
---  Double tap gesture.
--- 
---  ## Platform-specific
--- 
---  - Only available on **`macOS`** and **`iOS`**.
---  - On **`iOS`**, must be enabled first
DoubleTapGesture = {}

---@param _self DoubleTapGesture 
---@param other DoubleTapGesture 
---@return boolean
function DoubleTapGesture:eq(_self,other) end

---@param _self DoubleTapGesture 
---@return DoubleTapGesture
function DoubleTapGesture:clone(_self) end


---@class PanGesture : ReflectReference
---  Pan gesture.
--- 
---  ## Platform-specific
--- 
---  - On **`iOS`**, must be enabled first
---@field  [1] ? Vec2
PanGesture = {}

---@param _self PanGesture 
---@param other PanGesture 
---@return boolean
function PanGesture:eq(_self,other) end

---@param _self PanGesture 
---@return PanGesture
function PanGesture:clone(_self) end


---@class PinchGesture : ReflectReference
---  Two-finger pinch gesture, often used for magnifications.
--- 
---  Positive delta values indicate magnification (zooming in) and
---  negative delta values indicate shrinking (zooming out).
--- 
---  ## Platform-specific
--- 
---  - Only available on **`macOS`** and **`iOS`**.
---  - On **`iOS`**, must be enabled first
---@field  [1] ? number
PinchGesture = {}

---@param _self PinchGesture 
---@return PinchGesture
function PinchGesture:clone(_self) end

---@param _self PinchGesture 
---@param other PinchGesture 
---@return boolean
function PinchGesture:eq(_self,other) end


---@class RotationGesture : ReflectReference
---  Two-finger rotation gesture.
--- 
---  Positive delta values indicate rotation counterclockwise and
---  negative delta values indicate rotation clockwise.
--- 
---  ## Platform-specific
--- 
---  - Only available on **`macOS`** and **`iOS`**.
---  - On **`iOS`**, must be enabled first
---@field  [1] ? number
RotationGesture = {}

---@param _self RotationGesture 
---@return RotationGesture
function RotationGesture:clone(_self) end

---@param _self RotationGesture 
---@param other RotationGesture 
---@return boolean
function RotationGesture:eq(_self,other) end


---@class Key : ReflectReference
---  The logical key code of a [`KeyboardInput`].
--- 
---  ## Technical
--- 
---  Its values map 1 to 1 to winit's Key.
Key = {}

---@param _self Key 
---@param other Key 
---@return boolean
function Key:eq(_self,other) end

---@param _self Key 
---@return Key
function Key:clone(_self) end

---@param _self Key 
---@return nil
function Key:assert_receiver_is_total_eq(_self) end


---@class KeyCode : ReflectReference
---  The key code of a [`KeyboardInput`].
--- 
---  ## Usage
--- 
---  It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<ButtonInput<KeyCode>>`.
--- 
---  Code representing the location of a physical key
---  This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
---  exceptions:
---  - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and
---    `SuperRight` here.
---  - The key that the specification calls "Super" is reported as `Unidentified` here.
--- 
---  [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
--- 
---  ## Updating
--- 
---  The resource is updated inside of the [`keyboard_input_system`].
KeyCode = {}

---@param _self KeyCode 
---@return KeyCode
function KeyCode:clone(_self) end

---@param _self KeyCode 
---@return nil
function KeyCode:assert_receiver_is_total_eq(_self) end

---@param _self KeyCode 
---@param other KeyCode 
---@return boolean
function KeyCode:eq(_self,other) end


---@class KeyboardFocusLost : ReflectReference
---  Gets generated from `bevy_winit::winit_runner`
--- 
---  Used for clearing all cached states to avoid having 'stuck' key presses
---  when, for example, switching between windows with 'Alt-Tab' or using any other
---  OS specific key combination that leads to Bevy window losing focus and not receiving any
---  input events
KeyboardFocusLost = {}

---@param _self KeyboardFocusLost 
---@return KeyboardFocusLost
function KeyboardFocusLost:clone(_self) end

---@param _self KeyboardFocusLost 
---@param other KeyboardFocusLost 
---@return boolean
function KeyboardFocusLost:eq(_self,other) end

---@param _self KeyboardFocusLost 
---@return nil
function KeyboardFocusLost:assert_receiver_is_total_eq(_self) end


---@class KeyboardInput : ReflectReference
---  A keyboard input event.
--- 
---  This event is the translated version of the `WindowEvent::KeyboardInput` from the `winit` crate.
---  It is available to the end user and can be used for game logic.
--- 
---  ## Usage
--- 
---  The event is consumed inside of the [`keyboard_input_system`]
---  to update the [`ButtonInput<KeyCode>`](ButtonInput<KeyCode>) resource.
---@field  key_code ? KeyCode
---@field  logical_key ? Key
---@field  state ? ButtonState
---@field  text ? Option
---@field  repeat ? boolean
---@field  window ? Entity
KeyboardInput = {}

---@param _self KeyboardInput 
---@return KeyboardInput
function KeyboardInput:clone(_self) end

---@param _self KeyboardInput 
---@return nil
function KeyboardInput:assert_receiver_is_total_eq(_self) end

---@param _self KeyboardInput 
---@param other KeyboardInput 
---@return boolean
function KeyboardInput:eq(_self,other) end


---@class NativeKey : ReflectReference
---  Contains the platform-native logical key identifier, known as keysym.
--- 
---  Exactly what that means differs from platform to platform, but the values are to some degree
---  tied to the currently active keyboard layout. The same key on the same keyboard may also report
---  different values on different platforms, which is one of the reasons this is a per-platform
---  enum.
--- 
---  This enum is primarily used to store raw keysym when Winit doesn't map a given native logical
---  key identifier to a meaningful [`Key`] variant. This lets you use [`Key`], and let the user
---  define keybinds which work in the presence of identifiers we haven't mapped for you yet.
NativeKey = {}

---@param _self NativeKey 
---@return NativeKey
function NativeKey:clone(_self) end

---@param _self NativeKey 
---@return nil
function NativeKey:assert_receiver_is_total_eq(_self) end

---@param _self NativeKey 
---@param other NativeKey 
---@return boolean
function NativeKey:eq(_self,other) end


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
NativeKeyCode = {}

---@param _self NativeKeyCode 
---@return nil
function NativeKeyCode:assert_receiver_is_total_eq(_self) end

---@param _self NativeKeyCode 
---@param other NativeKeyCode 
---@return boolean
function NativeKeyCode:eq(_self,other) end

---@param _self NativeKeyCode 
---@return NativeKeyCode
function NativeKeyCode:clone(_self) end


---@class AccumulatedMouseMotion : ReflectReference
---  Tracks how much the mouse has moved every frame.
--- 
---  This resource is reset to zero every frame.
--- 
---  This resource sums the total [`MouseMotion`] events received this frame.
---@field  delta ? Vec2
AccumulatedMouseMotion = {}

---@param _self AccumulatedMouseMotion 
---@param other AccumulatedMouseMotion 
---@return boolean
function AccumulatedMouseMotion:eq(_self,other) end

---@param _self AccumulatedMouseMotion 
---@return AccumulatedMouseMotion
function AccumulatedMouseMotion:clone(_self) end


---@class AccumulatedMouseScroll : ReflectReference
---  Tracks how much the mouse has scrolled every frame.
--- 
---  This resource is reset to zero every frame.
--- 
---  This resource sums the total [`MouseWheel`] events received this frame.
---@field  unit ? MouseScrollUnit
---@field  delta ? Vec2
AccumulatedMouseScroll = {}

---@param _self AccumulatedMouseScroll 
---@param other AccumulatedMouseScroll 
---@return boolean
function AccumulatedMouseScroll:eq(_self,other) end

---@param _self AccumulatedMouseScroll 
---@return AccumulatedMouseScroll
function AccumulatedMouseScroll:clone(_self) end


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
MouseButton = {}

---@param _self MouseButton 
---@return MouseButton
function MouseButton:clone(_self) end

---@param _self MouseButton 
---@param other MouseButton 
---@return boolean
function MouseButton:eq(_self,other) end

---@param _self MouseButton 
---@return nil
function MouseButton:assert_receiver_is_total_eq(_self) end


---@class MouseButtonInput : ReflectReference
---  A mouse button input event.
--- 
---  This event is the translated version of the `WindowEvent::MouseInput` from the `winit` crate.
--- 
---  ## Usage
--- 
---  The event is read inside of the [`mouse_button_input_system`]
---  to update the [`ButtonInput<MouseButton>`] resource.
---@field  button ? MouseButton
---@field  state ? ButtonState
---@field  window ? Entity
MouseButtonInput = {}

---@param _self MouseButtonInput 
---@return nil
function MouseButtonInput:assert_receiver_is_total_eq(_self) end

---@param _self MouseButtonInput 
---@return MouseButtonInput
function MouseButtonInput:clone(_self) end

---@param _self MouseButtonInput 
---@param other MouseButtonInput 
---@return boolean
function MouseButtonInput:eq(_self,other) end


---@class MouseMotion : ReflectReference
---  An event reporting the change in physical position of a pointing device.
--- 
---  This represents raw, unfiltered physical motion.
---  It is the translated version of [`DeviceEvent::MouseMotion`] from the `winit` crate.
--- 
---  All pointing devices connected to a single machine at the same time can emit the event independently.
---  However, the event data does not make it possible to distinguish which device it is referring to.
--- 
---  [`DeviceEvent::MouseMotion`]: https://docs.rs/winit/latest/winit/event/enum.DeviceEvent.html#variant.MouseMotion
---@field  delta ? Vec2
MouseMotion = {}

---@param _self MouseMotion 
---@param other MouseMotion 
---@return boolean
function MouseMotion:eq(_self,other) end

---@param _self MouseMotion 
---@return MouseMotion
function MouseMotion:clone(_self) end


---@class MouseScrollUnit : ReflectReference
---  The scroll unit.
--- 
---  Describes how a value of a [`MouseWheel`] event has to be interpreted.
--- 
---  The value of the event can either be interpreted as the amount of lines or the amount of pixels
---  to scroll.
MouseScrollUnit = {}

---@param _self MouseScrollUnit 
---@param other MouseScrollUnit 
---@return boolean
function MouseScrollUnit:eq(_self,other) end

---@param _self MouseScrollUnit 
---@return nil
function MouseScrollUnit:assert_receiver_is_total_eq(_self) end

---@param _self MouseScrollUnit 
---@return MouseScrollUnit
function MouseScrollUnit:clone(_self) end


---@class MouseWheel : ReflectReference
---  A mouse wheel event.
--- 
---  This event is the translated version of the `WindowEvent::MouseWheel` from the `winit` crate.
---@field  unit ? MouseScrollUnit
---@field  x ? number
---@field  y ? number
---@field  window ? Entity
MouseWheel = {}

---@param _self MouseWheel 
---@param other MouseWheel 
---@return boolean
function MouseWheel:eq(_self,other) end

---@param _self MouseWheel 
---@return MouseWheel
function MouseWheel:clone(_self) end


---@class ForceTouch : ReflectReference
---  A force description of a [`Touch`] input.
ForceTouch = {}

---@param _self ForceTouch 
---@param other ForceTouch 
---@return boolean
function ForceTouch:eq(_self,other) end

---@param _self ForceTouch 
---@return ForceTouch
function ForceTouch:clone(_self) end


---@class TouchInput : ReflectReference
---  A touch input event.
--- 
---  ## Logic
--- 
---  Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique
---  identifier for the finger is generated. When the finger is lifted, the [`TouchPhase::Ended`]
---  event is generated with the same finger id.
--- 
---  After a [`TouchPhase::Started`] event has been emitted, there may be zero or more [`TouchPhase::Moved`]
---  events when the finger is moved or the touch pressure changes.
--- 
---  The finger id may be reused by the system after an [`TouchPhase::Ended`] event. The user
---  should assume that a new [`TouchPhase::Started`] event received with the same id has nothing
---  to do with the old finger and is a new finger.
--- 
---  A [`TouchPhase::Canceled`] event is emitted when the system has canceled tracking this
---  touch, such as when the window loses focus, or on iOS if the user moves the
---  device against their face.
--- 
---  ## Note
--- 
---  This event is the translated version of the `WindowEvent::Touch` from the `winit` crate.
---  It is available to the end user and can be used for game logic.
---@field  phase ? TouchPhase
---@field  position ? Vec2
---@field  window ? Entity
---@field  force ? Option
---@field  id ? integer
TouchInput = {}

---@param _self TouchInput 
---@param other TouchInput 
---@return boolean
function TouchInput:eq(_self,other) end

---@param _self TouchInput 
---@return TouchInput
function TouchInput:clone(_self) end


---@class TouchPhase : ReflectReference
---  A phase of a [`TouchInput`].
--- 
---  ## Usage
--- 
---  It is used to describe the phase of the touch input that is currently active.
---  This includes a phase that indicates that a touch input has started or ended,
---  or that a finger has moved. There is also a canceled phase that indicates that
---  the system canceled the tracking of the finger.
TouchPhase = {}

---@param _self TouchPhase 
---@param other TouchPhase 
---@return boolean
function TouchPhase:eq(_self,other) end

---@param _self TouchPhase 
---@return TouchPhase
function TouchPhase:clone(_self) end

---@param _self TouchPhase 
---@return nil
function TouchPhase:assert_receiver_is_total_eq(_self) end


---@class AspectRatio : ReflectReference
---  An `AspectRatio` is the ratio of width to height.
---@field  [1] ? number
AspectRatio = {}

---@param _self AspectRatio 
---@return boolean
function AspectRatio:is_landscape(_self) end

---@param _self AspectRatio 
---@param other AspectRatio 
---@return boolean
function AspectRatio:eq(_self,other) end

---@param _self AspectRatio 
---@return number
function AspectRatio:ratio(_self) end

---@param _self AspectRatio 
---@return boolean
function AspectRatio:is_square(_self) end

---@param _self AspectRatio 
---@return boolean
function AspectRatio:is_portrait(_self) end

---@param _self AspectRatio 
---@return AspectRatio
function AspectRatio:inverse(_self) end

---@param _self AspectRatio 
---@return AspectRatio
function AspectRatio:clone(_self) end


---@class Aabb2d : ReflectReference
---  A 2D axis-aligned bounding box, or bounding rectangle
---@field  min ? Vec2
---@field  max ? Vec2
Aabb2d = {}

---@param _self Aabb2d 
---@param other Aabb2d 
---@return boolean
function Aabb2d:eq(_self,other) end

---@param _self Aabb2d 
---@param point Vec2 
---@return Vec2
function Aabb2d:closest_point(_self,point) end

---@param center Vec2 
---@param half_size Vec2 
---@return Aabb2d
function Aabb2d.new(center,half_size) end

---@param _self Aabb2d 
---@return BoundingCircle
function Aabb2d:bounding_circle(_self) end

---@param _self Aabb2d 
---@return Aabb2d
function Aabb2d:clone(_self) end


---@class BoundingCircle : ReflectReference
---  A bounding circle
---@field  center ? Vec2
---@field  circle ? Circle
BoundingCircle = {}

---@param _self BoundingCircle 
---@return number
function BoundingCircle:radius(_self) end

---@param _self BoundingCircle 
---@return BoundingCircle
function BoundingCircle:clone(_self) end

---@param _self BoundingCircle 
---@param point Vec2 
---@return Vec2
function BoundingCircle:closest_point(_self,point) end

---@param center Vec2 
---@param radius number 
---@return BoundingCircle
function BoundingCircle.new(center,radius) end

---@param _self BoundingCircle 
---@return Aabb2d
function BoundingCircle:aabb_2d(_self) end

---@param _self BoundingCircle 
---@param other BoundingCircle 
---@return boolean
function BoundingCircle:eq(_self,other) end


---@class Aabb3d : ReflectReference
---  A 3D axis-aligned bounding box
---@field  min ? Vec3A
---@field  max ? Vec3A
Aabb3d = {}

---@param _self Aabb3d 
---@return Aabb3d
function Aabb3d:clone(_self) end

---@param _self Aabb3d 
---@return BoundingSphere
function Aabb3d:bounding_sphere(_self) end

---@param _self Aabb3d 
---@param other Aabb3d 
---@return boolean
function Aabb3d:eq(_self,other) end


---@class BoundingSphere : ReflectReference
---  A bounding sphere
---@field  center ? Vec3A
---@field  sphere ? Sphere
BoundingSphere = {}

---@param _self BoundingSphere 
---@param other BoundingSphere 
---@return boolean
function BoundingSphere:eq(_self,other) end

---@param _self BoundingSphere 
---@return number
function BoundingSphere:radius(_self) end

---@param _self BoundingSphere 
---@return BoundingSphere
function BoundingSphere:clone(_self) end

---@param _self BoundingSphere 
---@return Aabb3d
function BoundingSphere:aabb_3d(_self) end


---@class AabbCast2d : ReflectReference
---  An intersection test that casts an [`Aabb2d`] along a ray.
---@field  ray ? RayCast2d
---@field  aabb ? Aabb2d
AabbCast2d = {}

---@param aabb Aabb2d 
---@param origin Vec2 
---@param direction Dir2 
---@param max number 
---@return AabbCast2d
function AabbCast2d.new(aabb,origin,direction,max) end

---@param aabb Aabb2d 
---@param ray Ray2d 
---@param max number 
---@return AabbCast2d
function AabbCast2d.from_ray(aabb,ray,max) end

---@param _self AabbCast2d 
---@param aabb Aabb2d 
---@return number | nil
function AabbCast2d:aabb_collision_at(_self,aabb) end

---@param _self AabbCast2d 
---@return AabbCast2d
function AabbCast2d:clone(_self) end


---@class BoundingCircleCast : ReflectReference
---  An intersection test that casts a [`BoundingCircle`] along a ray.
---@field  ray ? RayCast2d
---@field  circle ? BoundingCircle
BoundingCircleCast = {}

---@param circle BoundingCircle 
---@param ray Ray2d 
---@param max number 
---@return BoundingCircleCast
function BoundingCircleCast.from_ray(circle,ray,max) end

---@param circle BoundingCircle 
---@param origin Vec2 
---@param direction Dir2 
---@param max number 
---@return BoundingCircleCast
function BoundingCircleCast.new(circle,origin,direction,max) end

---@param _self BoundingCircleCast 
---@param circle BoundingCircle 
---@return number | nil
function BoundingCircleCast:circle_collision_at(_self,circle) end

---@param _self BoundingCircleCast 
---@return BoundingCircleCast
function BoundingCircleCast:clone(_self) end


---@class RayCast2d : ReflectReference
---  A raycast intersection test for 2D bounding volumes
---@field  ray ? Ray2d
---@field  max ? number
---@field  direction_recip ? Vec2
RayCast2d = {}

---@param _self RayCast2d 
---@param aabb Aabb2d 
---@return number | nil
function RayCast2d:aabb_intersection_at(_self,aabb) end

---@param ray Ray2d 
---@param max number 
---@return RayCast2d
function RayCast2d.from_ray(ray,max) end

---@param _self RayCast2d 
---@return RayCast2d
function RayCast2d:clone(_self) end

---@param origin Vec2 
---@param direction Dir2 
---@param max number 
---@return RayCast2d
function RayCast2d.new(origin,direction,max) end

---@param _self RayCast2d 
---@return Vec2
function RayCast2d:direction_recip(_self) end

---@param _self RayCast2d 
---@param circle BoundingCircle 
---@return number | nil
function RayCast2d:circle_intersection_at(_self,circle) end


---@class AabbCast3d : ReflectReference
---  An intersection test that casts an [`Aabb3d`] along a ray.
---@field  ray ? RayCast3d
---@field  aabb ? Aabb3d
AabbCast3d = {}

---@param _self AabbCast3d 
---@param aabb Aabb3d 
---@return number | nil
function AabbCast3d:aabb_collision_at(_self,aabb) end

---@param _self AabbCast3d 
---@return AabbCast3d
function AabbCast3d:clone(_self) end

---@param aabb Aabb3d 
---@param ray Ray3d 
---@param max number 
---@return AabbCast3d
function AabbCast3d.from_ray(aabb,ray,max) end


---@class BoundingSphereCast : ReflectReference
---  An intersection test that casts a [`BoundingSphere`] along a ray.
---@field  ray ? RayCast3d
---@field  sphere ? BoundingSphere
BoundingSphereCast = {}

---@param sphere BoundingSphere 
---@param ray Ray3d 
---@param max number 
---@return BoundingSphereCast
function BoundingSphereCast.from_ray(sphere,ray,max) end

---@param _self BoundingSphereCast 
---@return BoundingSphereCast
function BoundingSphereCast:clone(_self) end

---@param _self BoundingSphereCast 
---@param sphere BoundingSphere 
---@return number | nil
function BoundingSphereCast:sphere_collision_at(_self,sphere) end


---@class RayCast3d : ReflectReference
---  A raycast intersection test for 3D bounding volumes
---@field  origin ? Vec3A
---@field  direction ? Dir3A
---@field  max ? number
---@field  direction_recip ? Vec3A
RayCast3d = {}

---@param ray Ray3d 
---@param max number 
---@return RayCast3d
function RayCast3d.from_ray(ray,max) end

---@param _self RayCast3d 
---@return Vec3A
function RayCast3d:direction_recip(_self) end

---@param _self RayCast3d 
---@param sphere BoundingSphere 
---@return number | nil
function RayCast3d:sphere_intersection_at(_self,sphere) end

---@param _self RayCast3d 
---@return RayCast3d
function RayCast3d:clone(_self) end

---@param _self RayCast3d 
---@param aabb Aabb3d 
---@return number | nil
function RayCast3d:aabb_intersection_at(_self,aabb) end


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
CompassOctant = {}

---@param _self CompassOctant 
---@return integer
function CompassOctant:to_index(_self) end

---@param _self CompassOctant 
---@return CompassOctant
function CompassOctant:opposite(_self) end

---@param _self CompassOctant 
---@return CompassOctant
function CompassOctant:clone(_self) end

---@param _self CompassOctant 
---@param other CompassOctant 
---@return boolean
function CompassOctant:eq(_self,other) end

---@param _self CompassOctant 
---@return CompassOctant
function CompassOctant:neg(_self) end

---@param _self CompassOctant 
---@return nil
function CompassOctant:assert_receiver_is_total_eq(_self) end


---@class CompassQuadrant : ReflectReference
---  A compass enum with 4 directions.
---  ```text
---           N (North)
---           ▲
---           │
---           │
---  W (West) ┼─────► E (East)
---           │
---           │
---           ▼
---           S (South)
---  ```
CompassQuadrant = {}

---@param _self CompassQuadrant 
---@return integer
function CompassQuadrant:to_index(_self) end

---@param _self CompassQuadrant 
---@param other CompassQuadrant 
---@return boolean
function CompassQuadrant:eq(_self,other) end

---@param _self CompassQuadrant 
---@return CompassQuadrant
function CompassQuadrant:opposite(_self) end

---@param _self CompassQuadrant 
---@return CompassQuadrant
function CompassQuadrant:clone(_self) end

---@param _self CompassQuadrant 
---@return CompassQuadrant
function CompassQuadrant:neg(_self) end

---@param _self CompassQuadrant 
---@return nil
function CompassQuadrant:assert_receiver_is_total_eq(_self) end


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
EaseFunction = {}

---@param _self EaseFunction 
---@return EaseFunction
function EaseFunction:clone(_self) end

---@param _self EaseFunction 
---@param other EaseFunction 
---@return boolean
function EaseFunction:eq(_self,other) end


---@class JumpAt : ReflectReference
---  Configuration options for the [`EaseFunction::Steps`] curves. This closely replicates the
---  [CSS step function specification].
--- 
---  [CSS step function specification]: https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description
JumpAt = {}

---@param _self JumpAt 
---@return nil
function JumpAt:assert_receiver_is_total_eq(_self) end

---@param _self JumpAt 
---@param other JumpAt 
---@return boolean
function JumpAt:eq(_self,other) end

---@param _self JumpAt 
---@return JumpAt
function JumpAt:clone(_self) end


---@class Interval : ReflectReference
---  A nonempty closed interval, possibly unbounded in either direction.
--- 
---  In other words, the interval may stretch all the way to positive or negative infinity, but it
---  will always have some nonempty interior.
---@field  start ? number
---@field  end ? number
Interval = {}

---@param _self Interval 
---@param other Interval 
---@return boolean
function Interval:eq(_self,other) end

---@param _self Interval 
---@return boolean
function Interval:has_finite_start(_self) end

---@param _self Interval 
---@param value number 
---@return number
function Interval:clamp(_self,value) end

---@param _self Interval 
---@return number
function Interval:length(_self) end

---@param _self Interval 
---@return boolean
function Interval:has_finite_end(_self) end

---@param _self Interval 
---@return boolean
function Interval:is_bounded(_self) end

---@param _self Interval 
---@param item number 
---@return boolean
function Interval:contains(_self,item) end


---@param _self Interval 
---@return Interval
function Interval:clone(_self) end

---@param _self Interval 
---@return number
function Interval:start(_self) end

---@param _self Interval 
---@param other Interval 
---@return boolean
function Interval:contains_interval(_self,other) end


---@class Dir2 : ReflectReference
---  A normalized vector pointing in a direction in 2D space
---@field  [1] ? Vec2
Dir2 = {}

---@param _self Dir2 
---@param rhs number 
---@return Vec2
function Dir2:mul(_self,rhs) end

---@param _self Dir2 
---@param rhs Dir2 
---@param s number 
---@return Dir2
function Dir2:slerp(_self,rhs,s) end

---@param _self Dir2 
---@return Dir2
function Dir2:fast_renormalize(_self) end

---@param _self Dir2 
---@return Vec2
function Dir2:as_vec2(_self) end

---@param _self Dir2 
---@return Rot2
function Dir2:rotation_to_x(_self) end

---@param _self Dir2 
---@return Dir2
function Dir2:clone(_self) end

---@param _self Dir2 
---@return Rot2
function Dir2:rotation_from_x(_self) end

---@param _self Dir2 
---@return Dir2
function Dir2:neg(_self) end

---@param x number 
---@param y number 
---@return Dir2
function Dir2.from_xy_unchecked(x,y) end

---@param _self Dir2 
---@param other Dir2 
---@return boolean
function Dir2:eq(_self,other) end

---@param value Vec2 
---@return Dir2
function Dir2.new_unchecked(value) end

---@param _self Dir2 
---@return Rot2
function Dir2:rotation_from_y(_self) end

---@param _self Dir2 
---@return Rot2
function Dir2:rotation_to_y(_self) end

---@param _self Dir2 
---@param other Dir2 
---@return Rot2
function Dir2:rotation_from(_self,other) end

---@param _self Dir2 
---@param other Dir2 
---@return Rot2
function Dir2:rotation_to(_self,other) end


---@class Dir3 : ReflectReference
---  A normalized vector pointing in a direction in 3D space
---@field  [1] ? Vec3
Dir3 = {}

---@param value Vec3 
---@return Dir3
function Dir3.new_unchecked(value) end

---@param _self Dir3 
---@return Dir3
function Dir3:neg(_self) end

---@param _self Dir3 
---@param rhs Dir3 
---@param s number 
---@return Dir3
function Dir3:slerp(_self,rhs,s) end

---@param _self Dir3 
---@param rhs number 
---@return Vec3
function Dir3:mul(_self,rhs) end

---@param _self Dir3 
---@return Dir3
function Dir3:clone(_self) end

---@param _self Dir3 
---@return Vec3
function Dir3:as_vec3(_self) end

---@param _self Dir3 
---@return Dir3
function Dir3:fast_renormalize(_self) end

---@param _self Dir3 
---@param other Dir3 
---@return boolean
function Dir3:eq(_self,other) end

---@param x number 
---@param y number 
---@param z number 
---@return Dir3
function Dir3.from_xyz_unchecked(x,y,z) end


---@class Dir3A : ReflectReference
---  A normalized SIMD vector pointing in a direction in 3D space.
--- 
---  This type stores a 16 byte aligned [`Vec3A`].
---  This may or may not be faster than [`Dir3`]: make sure to benchmark!
---@field  [1] ? Vec3A
Dir3A = {}

---@param _self Dir3A 
---@return Dir3A
function Dir3A:fast_renormalize(_self) end

---@param _self Dir3A 
---@return Vec3A
function Dir3A:as_vec3a(_self) end

---@param _self Dir3A 
---@param rhs Dir3A 
---@param s number 
---@return Dir3A
function Dir3A:slerp(_self,rhs,s) end

---@param value Vec3A 
---@return Dir3A
function Dir3A.new_unchecked(value) end

---@param _self Dir3A 
---@return Dir3A
function Dir3A:clone(_self) end

---@param _self Dir3A 
---@param other Dir3A 
---@return boolean
function Dir3A:eq(_self,other) end

---@param x number 
---@param y number 
---@param z number 
---@return Dir3A
function Dir3A.from_xyz_unchecked(x,y,z) end

---@param _self Dir3A 
---@param rhs number 
---@return Vec3A
function Dir3A:mul(_self,rhs) end

---@param _self Dir3A 
---@return Dir3A
function Dir3A:neg(_self) end


---@class FloatOrd : ReflectReference
---  A wrapper for floats that implements [`Ord`], [`Eq`], and [`Hash`] traits.
--- 
---  This is a work around for the fact that the IEEE 754-2008 standard,
---  implemented by Rust's [`f32`] type,
---  doesn't define an ordering for [`NaN`](f32::NAN),
---  and `NaN` is not considered equal to any other `NaN`.
--- 
---  Wrapping a float with `FloatOrd` breaks conformance with the standard
---  by sorting `NaN` as less than all other numbers and equal to any other `NaN`.
---@field  [1] ? number
FloatOrd = {}

---@param _self FloatOrd 
---@param other FloatOrd 
---@return boolean
function FloatOrd:eq(_self,other) end

---@param _self FloatOrd 
---@param other FloatOrd 
---@return boolean
function FloatOrd:lt(_self,other) end

---@param _self FloatOrd 
---@return FloatOrd
function FloatOrd:neg(_self) end

---@param _self FloatOrd 
---@param other FloatOrd 
---@return boolean
function FloatOrd:ge(_self,other) end

---@param _self FloatOrd 
---@param other FloatOrd 
---@return boolean
function FloatOrd:le(_self,other) end

---@param _self FloatOrd 
---@return FloatOrd
function FloatOrd:clone(_self) end

---@param _self FloatOrd 
---@param other FloatOrd 
---@return boolean
function FloatOrd:gt(_self,other) end


---@class Isometry2d : ReflectReference
---  An isometry in two dimensions, representing a rotation followed by a translation.
---  This can often be useful for expressing relative positions and transformations from one position to another.
--- 
---  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,
---  and belongs to the special [Euclidean group] SE(2). This includes translation and rotation, but excludes reflection.
--- 
---  For the three-dimensional version, see [`Isometry3d`].
--- 
---  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group
--- 
---  # Example
--- 
---  Isometries can be created from a given translation and rotation:
--- 
---  ```
---  # use bevy_math::{Isometry2d, Rot2, Vec2};
---  #
---  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
---  ```
--- 
---  Or from separate parts:
--- 
---  ```
---  # use bevy_math::{Isometry2d, Rot2, Vec2};
---  #
---  let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
---  let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));
---  ```
--- 
---  The isometries can be used to transform points:
--- 
---  ```
---  # use approx::assert_abs_diff_eq;
---  # use bevy_math::{Isometry2d, Rot2, Vec2};
---  #
---  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
---  let point = Vec2::new(4.0, 4.0);
--- 
---  // These are equivalent
---  let result = iso.transform_point(point);
---  let result = iso * point;
--- 
---  assert_eq!(result, Vec2::new(-2.0, 5.0));
---  ```
--- 
---  Isometries can also be composed together:
--- 
---  ```
---  # use bevy_math::{Isometry2d, Rot2, Vec2};
---  #
---  # let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
---  # let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
---  # let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));
---  #
---  assert_eq!(iso1 * iso2, iso);
---  ```
--- 
---  One common operation is to compute an isometry representing the relative positions of two objects
---  for things like intersection tests. This can be done with an inverse transformation:
--- 
---  ```
---  # use bevy_math::{Isometry2d, Rot2, Vec2};
---  #
---  let circle_iso = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
---  let rectangle_iso = Isometry2d::from_rotation(Rot2::degrees(90.0));
--- 
---  // Compute the relative position and orientation between the two shapes
---  let relative_iso = circle_iso.inverse() * rectangle_iso;
--- 
---  // Or alternatively, to skip an extra rotation operation:
---  let relative_iso = circle_iso.inverse_mul(rectangle_iso);
---  ```
---@field  rotation ? Rot2
---@field  translation ? Vec2
Isometry2d = {}

---@param translation Vec2 
---@param rotation Rot2 
---@return Isometry2d
function Isometry2d.new(translation,rotation) end

---@param rotation Rot2 
---@return Isometry2d
function Isometry2d.from_rotation(rotation) end

---@param x number 
---@param y number 
---@return Isometry2d
function Isometry2d.from_xy(x,y) end

---@param _self Isometry2d 
---@return Isometry2d
function Isometry2d:clone(_self) end

---@param _self Isometry2d 
---@param point Vec2 
---@return Vec2
function Isometry2d:transform_point(_self,point) end

---@param _self Isometry2d 
---@return Isometry2d
function Isometry2d:inverse(_self) end

---@param translation Vec2 
---@return Isometry2d
function Isometry2d.from_translation(translation) end

---@param p1 Isometry2d 
---@param p2 Dir2 
---@return Dir2
function Isometry2d:mul(p1,p2) end

---@param _self Isometry2d 
---@param point Vec2 
---@return Vec2
function Isometry2d:inverse_transform_point(_self,point) end

---@param _self Isometry2d 
---@param rhs Isometry2d 
---@return Isometry2d
function Isometry2d:mul(_self,rhs) end

---@param _self Isometry2d 
---@param rhs Isometry2d 
---@return Isometry2d
function Isometry2d:inverse_mul(_self,rhs) end

---@param _self Isometry2d 
---@param other Isometry2d 
---@return boolean
function Isometry2d:eq(_self,other) end

---@param p1 Isometry2d 
---@param p2 Vec2 
---@return Vec2
function Isometry2d:mul(p1,p2) end


---@class Isometry3d : ReflectReference
---  An isometry in three dimensions, representing a rotation followed by a translation.
---  This can often be useful for expressing relative positions and transformations from one position to another.
--- 
---  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,
---  and belongs to the special [Euclidean group] SE(3). This includes translation and rotation, but excludes reflection.
--- 
---  For the two-dimensional version, see [`Isometry2d`].
--- 
---  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group
--- 
---  # Example
--- 
---  Isometries can be created from a given translation and rotation:
--- 
---  ```
---  # use bevy_math::{Isometry3d, Quat, Vec3};
---  # use std::f32::consts::FRAC_PI_2;
---  #
---  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
---  ```
--- 
---  Or from separate parts:
--- 
---  ```
---  # use bevy_math::{Isometry3d, Quat, Vec3};
---  # use std::f32::consts::FRAC_PI_2;
---  #
---  let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
---  let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
---  ```
--- 
---  The isometries can be used to transform points:
--- 
---  ```
---  # use approx::assert_relative_eq;
---  # use bevy_math::{Isometry3d, Quat, Vec3};
---  # use std::f32::consts::FRAC_PI_2;
---  #
---  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
---  let point = Vec3::new(4.0, 4.0, 4.0);
--- 
---  // These are equivalent
---  let result = iso.transform_point(point);
---  let result = iso * point;
--- 
---  assert_relative_eq!(result, Vec3::new(-2.0, 5.0, 7.0));
---  ```
--- 
---  Isometries can also be composed together:
--- 
---  ```
---  # use bevy_math::{Isometry3d, Quat, Vec3};
---  # use std::f32::consts::FRAC_PI_2;
---  #
---  # let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
---  # let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
---  # let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
---  #
---  assert_eq!(iso1 * iso2, iso);
---  ```
--- 
---  One common operation is to compute an isometry representing the relative positions of two objects
---  for things like intersection tests. This can be done with an inverse transformation:
--- 
---  ```
---  # use bevy_math::{Isometry3d, Quat, Vec3};
---  # use std::f32::consts::FRAC_PI_2;
---  #
---  let sphere_iso = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
---  let cuboid_iso = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
--- 
---  // Compute the relative position and orientation between the two shapes
---  let relative_iso = sphere_iso.inverse() * cuboid_iso;
--- 
---  // Or alternatively, to skip an extra rotation operation:
---  let relative_iso = sphere_iso.inverse_mul(cuboid_iso);
---  ```
---@field  rotation ? Quat
---@field  translation ? Vec3A
Isometry3d = {}

---@param x number 
---@param y number 
---@param z number 
---@return Isometry3d
function Isometry3d.from_xyz(x,y,z) end

---@param rotation Quat 
---@return Isometry3d
function Isometry3d.from_rotation(rotation) end

---@param _self Isometry3d 
---@param other Isometry3d 
---@return boolean
function Isometry3d:eq(_self,other) end

---@param _self Isometry3d 
---@param rhs Isometry3d 
---@return Isometry3d
function Isometry3d:mul(_self,rhs) end

---@param _self Isometry3d 
---@param rhs Isometry3d 
---@return Isometry3d
function Isometry3d:inverse_mul(_self,rhs) end

---@param _self Isometry3d 
---@return Isometry3d
function Isometry3d:inverse(_self) end

---@param p1 Isometry3d 
---@param p2 Vec3A 
---@return Vec3A
function Isometry3d:mul(p1,p2) end

---@param p1 Isometry3d 
---@param p2 Vec3 
---@return Vec3
function Isometry3d:mul(p1,p2) end

---@param _self Isometry3d 
---@return Isometry3d
function Isometry3d:clone(_self) end

---@param p1 Isometry3d 
---@param p2 Dir3 
---@return Dir3
function Isometry3d:mul(p1,p2) end


---@class Annulus : ReflectReference
---  A primitive shape formed by the region between two circles, also known as a ring.
---@field  inner_circle ? Circle
---@field  outer_circle ? Circle
Annulus = {}

---@param _self Annulus 
---@return number
function Annulus:thickness(_self) end

---@param _self Annulus 
---@return number
function Annulus:diameter(_self) end

---@param inner_radius number 
---@param outer_radius number 
---@return Annulus
function Annulus.new(inner_radius,outer_radius) end

---@param _self Annulus 
---@param point Vec2 
---@return Vec2
function Annulus:closest_point(_self,point) end

---@param _self Annulus 
---@return Annulus
function Annulus:clone(_self) end

---@param _self Annulus 
---@param other Annulus 
---@return boolean
function Annulus:eq(_self,other) end


---@class Arc2d : ReflectReference
---  A primitive representing an arc between two points on a circle.
--- 
---  An arc has no area.
---  If you want to include the portion of a circle's area swept out by the arc,
---  use the pie-shaped [`CircularSector`].
---  If you want to include only the space inside the convex hull of the arc,
---  use the bowl-shaped [`CircularSegment`].
--- 
---  The arc is drawn starting from [`Vec2::Y`], extending by `half_angle` radians on
---  either side. The center of the circle is the origin [`Vec2::ZERO`]. Note that this
---  means that the origin may not be within the `Arc2d`'s convex hull.
--- 
---  **Warning:** Arcs with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
---  It is recommended to normalize arcs to have an angle in [0, 2π].
---@field  radius ? number
---@field  half_angle ? number
Arc2d = {}

---@param _self Arc2d 
---@return Arc2d
function Arc2d:clone(_self) end

---@param _self Arc2d 
---@return boolean
function Arc2d:is_major(_self) end

---@param _self Arc2d 
---@return number
function Arc2d:angle(_self) end

---@param _self Arc2d 
---@return number
function Arc2d:length(_self) end

---@param _self Arc2d 
---@param other Arc2d 
---@return boolean
function Arc2d:eq(_self,other) end

---@param _self Arc2d 
---@return boolean
function Arc2d:is_minor(_self) end

---@param radius number 
---@param angle number 
---@return Arc2d
function Arc2d.from_degrees(radius,angle) end

---@param _self Arc2d 
---@return Vec2
function Arc2d:chord_midpoint(_self) end

---@param _self Arc2d 
---@return Vec2
function Arc2d:midpoint(_self) end

---@param _self Arc2d 
---@return number
function Arc2d:apothem(_self) end

---@param _self Arc2d 
---@return number
function Arc2d:chord_length(_self) end

---@param _self Arc2d 
---@return Vec2
function Arc2d:right_endpoint(_self) end

---@param _self Arc2d 
---@return Vec2
function Arc2d:left_endpoint(_self) end

---@param _self Arc2d 
---@return number
function Arc2d:sagitta(_self) end

---@param radius number 
---@param angle number 
---@return Arc2d
function Arc2d.from_radians(radius,angle) end

---@param radius number 
---@param half_angle number 
---@return Arc2d
function Arc2d.new(radius,half_angle) end

---@param _self Arc2d 
---@return number
function Arc2d:half_chord_length(_self) end

---@param radius number 
---@param fraction number 
---@return Arc2d
function Arc2d.from_turns(radius,fraction) end


---@class Capsule2d : ReflectReference
---  A 2D capsule primitive, also known as a stadium or pill shape.
--- 
---  A two-dimensional capsule is defined as a neighborhood of points at a distance (radius) from a line
---@field  radius ? number
---@field  half_length ? number
Capsule2d = {}

---@param _self Capsule2d 
---@return Rectangle
function Capsule2d:to_inner_rectangle(_self) end

---@param radius number 
---@param length number 
---@return Capsule2d
function Capsule2d.new(radius,length) end

---@param _self Capsule2d 
---@param other Capsule2d 
---@return boolean
function Capsule2d:eq(_self,other) end

---@param _self Capsule2d 
---@return Capsule2d
function Capsule2d:clone(_self) end


---@class Circle : ReflectReference
---  A circle primitive, representing the set of points some distance from the origin
---@field  radius ? number
Circle = {}

---@param _self Circle 
---@param other Circle 
---@return boolean
function Circle:eq(_self,other) end

---@param _self Circle 
---@return number
function Circle:diameter(_self) end

---@param _self Circle 
---@param point Vec2 
---@return Vec2
function Circle:closest_point(_self,point) end

---@param radius number 
---@return Circle
function Circle.new(radius) end

---@param _self Circle 
---@return Circle
function Circle:clone(_self) end


---@class CircularSector : ReflectReference
---  A primitive representing a circular sector: a pie slice of a circle.
--- 
---  The segment is positioned so that it always includes [`Vec2::Y`] and is vertically symmetrical.
---  To orient the sector differently, apply a rotation.
---  The sector is drawn with the center of its circle at the origin [`Vec2::ZERO`].
--- 
---  **Warning:** Circular sectors with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
---  We recommend normalizing circular sectors to have an angle in [0, 2π].
---@field  arc ? Arc2d
CircularSector = {}

---@param _self CircularSector 
---@return number
function CircularSector:sagitta(_self) end

---@param _self CircularSector 
---@return number
function CircularSector:angle(_self) end

---@param _self CircularSector 
---@param other CircularSector 
---@return boolean
function CircularSector:eq(_self,other) end

---@param _self CircularSector 
---@return number
function CircularSector:apothem(_self) end

---@param _self CircularSector 
---@return Vec2
function CircularSector:chord_midpoint(_self) end

---@param _self CircularSector 
---@return CircularSector
function CircularSector:clone(_self) end

---@param radius number 
---@param angle number 
---@return CircularSector
function CircularSector.new(radius,angle) end

---@param _self CircularSector 
---@return number
function CircularSector:half_angle(_self) end

---@param _self CircularSector 
---@return number
function CircularSector:arc_length(_self) end

---@param _self CircularSector 
---@return number
function CircularSector:chord_length(_self) end

---@param _self CircularSector 
---@return number
function CircularSector:radius(_self) end

---@param radius number 
---@param angle number 
---@return CircularSector
function CircularSector.from_radians(radius,angle) end

---@param radius number 
---@param angle number 
---@return CircularSector
function CircularSector.from_degrees(radius,angle) end

---@param _self CircularSector 
---@return number
function CircularSector:half_chord_length(_self) end

---@param radius number 
---@param fraction number 
---@return CircularSector
function CircularSector.from_turns(radius,fraction) end


---@class CircularSegment : ReflectReference
---  A primitive representing a circular segment:
---  the area enclosed by the arc of a circle and its chord (the line between its endpoints).
--- 
---  The segment is drawn starting from [`Vec2::Y`], extending equally on either side.
---  To orient the segment differently, apply a rotation.
---  The segment is drawn with the center of its circle at the origin [`Vec2::ZERO`].
---  When positioning a segment, the [`apothem`](Self::apothem) function may be particularly useful.
--- 
---  **Warning:** Circular segments with negative angle or radius, or with angle greater than an entire circle, are not officially supported.
---  We recommend normalizing circular segments to have an angle in [0, 2π].
---@field  arc ? Arc2d
CircularSegment = {}

---@param radius number 
---@param angle number 
---@return CircularSegment
function CircularSegment.from_degrees(radius,angle) end

---@param _self CircularSegment 
---@return Vec2
function CircularSegment:chord_midpoint(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:chord_length(_self) end

---@param _self CircularSegment 
---@param other CircularSegment 
---@return boolean
function CircularSegment:eq(_self,other) end

---@param _self CircularSegment 
---@return number
function CircularSegment:radius(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:half_angle(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:apothem(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:angle(_self) end

---@param _self CircularSegment 
---@return CircularSegment
function CircularSegment:clone(_self) end

---@param radius number 
---@param fraction number 
---@return CircularSegment
function CircularSegment.from_turns(radius,fraction) end

---@param radius number 
---@param angle number 
---@return CircularSegment
function CircularSegment.from_radians(radius,angle) end

---@param _self CircularSegment 
---@return number
function CircularSegment:arc_length(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:half_chord_length(_self) end

---@param _self CircularSegment 
---@return number
function CircularSegment:sagitta(_self) end

---@param radius number 
---@param angle number 
---@return CircularSegment
function CircularSegment.new(radius,angle) end


---@class Ellipse : ReflectReference
---  An ellipse primitive, which is like a circle, but the width and height can be different
---@field  half_size ? Vec2
Ellipse = {}

---@param _self Ellipse 
---@return number
function Ellipse:focal_length(_self) end

---@param _self Ellipse 
---@param other Ellipse 
---@return boolean
function Ellipse:eq(_self,other) end

---@param half_width number 
---@param half_height number 
---@return Ellipse
function Ellipse.new(half_width,half_height) end

---@param size Vec2 
---@return Ellipse
function Ellipse.from_size(size) end

---@param _self Ellipse 
---@return number
function Ellipse:semi_major(_self) end

---@param _self Ellipse 
---@return number
function Ellipse:eccentricity(_self) end

---@param _self Ellipse 
---@return number
function Ellipse:semi_minor(_self) end

---@param _self Ellipse 
---@return Ellipse
function Ellipse:clone(_self) end


---@class Line2d : ReflectReference
---  An infinite line going through the origin along a direction in 2D space.
--- 
---  For a finite line: [`Segment2d`]
---@field  direction ? Dir2
Line2d = {}

---@param _self Line2d 
---@param other Line2d 
---@return boolean
function Line2d:eq(_self,other) end

---@param _self Line2d 
---@return Line2d
function Line2d:clone(_self) end


---@class Plane2d : ReflectReference
---  An unbounded plane in 2D space. It forms a separating surface through the origin,
---  stretching infinitely far
---@field  normal ? Dir2
Plane2d = {}

---@param _self Plane2d 
---@param other Plane2d 
---@return boolean
function Plane2d:eq(_self,other) end

---@param normal Vec2 
---@return Plane2d
function Plane2d.new(normal) end

---@param _self Plane2d 
---@return Plane2d
function Plane2d:clone(_self) end


---@class Rectangle : ReflectReference
---  A rectangle primitive, which is like a square, except that the width and height can be different
---@field  half_size ? Vec2
Rectangle = {}

---@param _self Rectangle 
---@param other Rectangle 
---@return boolean
function Rectangle:eq(_self,other) end

---@param _self Rectangle 
---@param point Vec2 
---@return Vec2
function Rectangle:closest_point(_self,point) end

---@param length number 
---@return Rectangle
function Rectangle.from_length(length) end

---@param point1 Vec2 
---@param point2 Vec2 
---@return Rectangle
function Rectangle.from_corners(point1,point2) end

---@param _self Rectangle 
---@return Rectangle
function Rectangle:clone(_self) end

---@param _self Rectangle 
---@return Vec2
function Rectangle:size(_self) end

---@param size Vec2 
---@return Rectangle
function Rectangle.from_size(size) end

---@param width number 
---@param height number 
---@return Rectangle
function Rectangle.new(width,height) end


---@class RegularPolygon : ReflectReference
---  A polygon centered on the origin where all vertices lie on a circle, equally far apart.
---@field  circumcircle ? Circle
---@field  sides ? integer
RegularPolygon = {}

---@param _self RegularPolygon 
---@return number
function RegularPolygon:external_angle_degrees(_self) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:side_length(_self) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:circumradius(_self) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:external_angle_radians(_self) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:internal_angle_degrees(_self) end

---@param _self RegularPolygon 
---@param other RegularPolygon 
---@return boolean
function RegularPolygon:eq(_self,other) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:internal_angle_radians(_self) end

---@param circumradius number 
---@param sides integer 
---@return RegularPolygon
function RegularPolygon.new(circumradius,sides) end

---@param _self RegularPolygon 
---@return RegularPolygon
function RegularPolygon:clone(_self) end

---@param _self RegularPolygon 
---@return number
function RegularPolygon:inradius(_self) end


---@class Rhombus : ReflectReference
---  A rhombus primitive, also known as a diamond shape.
---  A four sided polygon, centered on the origin, where opposite sides are parallel but without
---  requiring right angles.
---@field  half_diagonals ? Vec2
Rhombus = {}

---@param _self Rhombus 
---@return Rhombus
function Rhombus:clone(_self) end

---@param _self Rhombus 
---@return number
function Rhombus:inradius(_self) end

---@param horizontal_diagonal number 
---@param vertical_diagonal number 
---@return Rhombus
function Rhombus.new(horizontal_diagonal,vertical_diagonal) end

---@param side number 
---@return Rhombus
function Rhombus.from_side(side) end

---@param _self Rhombus 
---@param point Vec2 
---@return Vec2
function Rhombus:closest_point(_self,point) end

---@param _self Rhombus 
---@return number
function Rhombus:circumradius(_self) end

---@param _self Rhombus 
---@param other Rhombus 
---@return boolean
function Rhombus:eq(_self,other) end

---@param _self Rhombus 
---@return number
function Rhombus:side(_self) end

---@param inradius number 
---@return Rhombus
function Rhombus.from_inradius(inradius) end


---@class Segment2d : ReflectReference
---  A line segment defined by two endpoints in 2D space.
---@field  vertices ? [glam::Vec2; 2]
Segment2d = {}

---@param _self Segment2d 
---@return number
function Segment2d:length_squared(_self) end

---@param ray Ray2d 
---@param length number 
---@return Segment2d
function Segment2d.from_ray_and_length(ray,length) end

---@param _self Segment2d 
---@return Dir2
function Segment2d:left_normal(_self) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:point2(_self) end

---@param point1 Vec2 
---@param point2 Vec2 
---@return Segment2d
function Segment2d.new(point1,point2) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:center(_self) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:point1(_self) end

---@param _self Segment2d 
---@param rotation Rot2 
---@return Segment2d
function Segment2d:rotated(_self,rotation) end

---@param _self Segment2d 
---@param rotation Rot2 
---@return Segment2d
function Segment2d:rotated_around_center(_self,rotation) end

---@param _self Segment2d 
---@param rotation Rot2 
---@param point Vec2 
---@return Segment2d
function Segment2d:rotated_around(_self,rotation,point) end

---@param direction Dir2 
---@param length number 
---@return Segment2d
function Segment2d.from_direction_and_length(direction,length) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:scaled_left_normal(_self) end

---@param _self Segment2d 
---@return Segment2d
function Segment2d:centered(_self) end

---@param _self Segment2d 
---@return Dir2
function Segment2d:right_normal(_self) end

---@param scaled_direction Vec2 
---@return Segment2d
function Segment2d.from_scaled_direction(scaled_direction) end

---@param _self Segment2d 
---@return Dir2
function Segment2d:direction(_self) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:scaled_right_normal(_self) end

---@param _self Segment2d 
---@return Segment2d
function Segment2d:reversed(_self) end

---@param _self Segment2d 
---@param length number 
---@return Segment2d
function Segment2d:resized(_self,length) end

---@param _self Segment2d 
---@return Segment2d
function Segment2d:clone(_self) end

---@param _self Segment2d 
---@return nil
function Segment2d:reverse(_self) end

---@param _self Segment2d 
---@param other Segment2d 
---@return boolean
function Segment2d:eq(_self,other) end

---@param _self Segment2d 
---@return number
function Segment2d:length(_self) end

---@param _self Segment2d 
---@param translation Vec2 
---@return Segment2d
function Segment2d:translated(_self,translation) end

---@param _self Segment2d 
---@return Vec2
function Segment2d:scaled_direction(_self) end


---@class Triangle2d : ReflectReference
---  A triangle in 2D space
---@field  vertices ? [glam::Vec2; 3]
Triangle2d = {}

---@param _self Triangle2d 
---@return nil
function Triangle2d:reverse(_self) end

---@param _self Triangle2d 
---@return Triangle2d
function Triangle2d:reversed(_self) end

---@param _self Triangle2d 
---@return boolean
function Triangle2d:is_acute(_self) end

---@param _self Triangle2d 
---@return boolean
function Triangle2d:is_obtuse(_self) end

---@param _self Triangle2d 
---@param other Triangle2d 
---@return boolean
function Triangle2d:eq(_self,other) end

---@param a Vec2 
---@param b Vec2 
---@param c Vec2 
---@return Triangle2d
function Triangle2d.new(a,b,c) end

---@param _self Triangle2d 
---@return Triangle2d
function Triangle2d:clone(_self) end

---@param _self Triangle2d 
---@return boolean
function Triangle2d:is_degenerate(_self) end


---@class Capsule3d : ReflectReference
---  A 3D capsule primitive centered on the origin
---  A three-dimensional capsule is defined as a surface at a distance (radius) from a line
---@field  radius ? number
---@field  half_length ? number
Capsule3d = {}

---@param _self Capsule3d 
---@return Cylinder
function Capsule3d:to_cylinder(_self) end

---@param _self Capsule3d 
---@return Capsule3d
function Capsule3d:clone(_self) end

---@param _self Capsule3d 
---@param other Capsule3d 
---@return boolean
function Capsule3d:eq(_self,other) end

---@param radius number 
---@param length number 
---@return Capsule3d
function Capsule3d.new(radius,length) end


---@class Cone : ReflectReference
---  A cone primitive centered on the midpoint between the tip of the cone and the center of its base.
--- 
---  The cone is oriented with its tip pointing towards the Y axis.
---@field  radius ? number
---@field  height ? number
Cone = {}

---@param _self Cone 
---@return number
function Cone:slant_height(_self) end

---@param _self Cone 
---@return number
function Cone:base_area(_self) end

---@param radius number 
---@param height number 
---@return Cone
function Cone.new(radius,height) end

---@param _self Cone 
---@return Cone
function Cone:clone(_self) end

---@param _self Cone 
---@param other Cone 
---@return boolean
function Cone:eq(_self,other) end

---@param _self Cone 
---@return number
function Cone:lateral_area(_self) end

---@param _self Cone 
---@return Circle
function Cone:base(_self) end


---@class ConicalFrustum : ReflectReference
---  A conical frustum primitive.
---  A conical frustum can be created
---  by slicing off a section of a cone.
---@field  radius_top ? number
---@field  radius_bottom ? number
---@field  height ? number
ConicalFrustum = {}

---@param _self ConicalFrustum 
---@param other ConicalFrustum 
---@return boolean
function ConicalFrustum:eq(_self,other) end

---@param _self ConicalFrustum 
---@return ConicalFrustum
function ConicalFrustum:clone(_self) end


---@class Cuboid : ReflectReference
---  A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not
---  required to be the same.
---@field  half_size ? Vec3
Cuboid = {}

---@param _self Cuboid 
---@param point Vec3 
---@return Vec3
function Cuboid:closest_point(_self,point) end

---@param length number 
---@return Cuboid
function Cuboid.from_length(length) end

---@param _self Cuboid 
---@return Cuboid
function Cuboid:clone(_self) end

---@param _self Cuboid 
---@param other Cuboid 
---@return boolean
function Cuboid:eq(_self,other) end

---@param size Vec3 
---@return Cuboid
function Cuboid.from_size(size) end

---@param _self Cuboid 
---@return Vec3
function Cuboid:size(_self) end

---@param x_length number 
---@param y_length number 
---@param z_length number 
---@return Cuboid
function Cuboid.new(x_length,y_length,z_length) end

---@param point1 Vec3 
---@param point2 Vec3 
---@return Cuboid
function Cuboid.from_corners(point1,point2) end


---@class Cylinder : ReflectReference
---  A cylinder primitive centered on the origin
---@field  radius ? number
---@field  half_height ? number
Cylinder = {}

---@param _self Cylinder 
---@param other Cylinder 
---@return boolean
function Cylinder:eq(_self,other) end

---@param _self Cylinder 
---@return number
function Cylinder:lateral_area(_self) end

---@param radius number 
---@param height number 
---@return Cylinder
function Cylinder.new(radius,height) end

---@param _self Cylinder 
---@return Circle
function Cylinder:base(_self) end

---@param _self Cylinder 
---@return Cylinder
function Cylinder:clone(_self) end

---@param _self Cylinder 
---@return number
function Cylinder:base_area(_self) end


---@class InfinitePlane3d : ReflectReference
---  An unbounded plane in 3D space. It forms a separating surface through the origin,
---  stretching infinitely far
---@field  normal ? Dir3
InfinitePlane3d = {}

---@param _self InfinitePlane3d 
---@param other InfinitePlane3d 
---@return boolean
function InfinitePlane3d:eq(_self,other) end

---@param _self InfinitePlane3d 
---@return InfinitePlane3d
function InfinitePlane3d:clone(_self) end

---@param _self InfinitePlane3d 
---@param origin Vec3 
---@return Isometry3d
function InfinitePlane3d:isometry_from_xy(_self,origin) end

---@param _self InfinitePlane3d 
---@param origin Vec3 
---@return Isometry3d
function InfinitePlane3d:isometry_into_xy(_self,origin) end


---@class Line3d : ReflectReference
---  An infinite line going through the origin along a direction in 3D space.
--- 
---  For a finite line: [`Segment3d`]
---@field  direction ? Dir3
Line3d = {}

---@param _self Line3d 
---@return Line3d
function Line3d:clone(_self) end

---@param _self Line3d 
---@param other Line3d 
---@return boolean
function Line3d:eq(_self,other) end


---@class Plane3d : ReflectReference
---  A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.
---@field  normal ? Dir3
---@field  half_size ? Vec2
Plane3d = {}

---@param _self Plane3d 
---@return Plane3d
function Plane3d:clone(_self) end

---@param normal Vec3 
---@param half_size Vec2 
---@return Plane3d
function Plane3d.new(normal,half_size) end

---@param _self Plane3d 
---@param other Plane3d 
---@return boolean
function Plane3d:eq(_self,other) end


---@class Segment3d : ReflectReference
---  A line segment defined by two endpoints in 3D space.
---@field  vertices ? [glam::Vec3; 2]
Segment3d = {}

---@param _self Segment3d 
---@return Vec3
function Segment3d:scaled_direction(_self) end

---@param scaled_direction Vec3 
---@return Segment3d
function Segment3d.from_scaled_direction(scaled_direction) end

---@param direction Dir3 
---@param length number 
---@return Segment3d
function Segment3d.from_direction_and_length(direction,length) end

---@param _self Segment3d 
---@param rotation Quat 
---@return Segment3d
function Segment3d:rotated(_self,rotation) end

---@param _self Segment3d 
---@return number
function Segment3d:length(_self) end

---@param _self Segment3d 
---@param translation Vec3 
---@return Segment3d
function Segment3d:translated(_self,translation) end

---@param _self Segment3d 
---@return nil
function Segment3d:reverse(_self) end

---@param _self Segment3d 
---@return Vec3
function Segment3d:center(_self) end

---@param _self Segment3d 
---@return Dir3
function Segment3d:direction(_self) end

---@param _self Segment3d 
---@return number
function Segment3d:length_squared(_self) end

---@param _self Segment3d 
---@return Vec3
function Segment3d:point1(_self) end

---@param _self Segment3d 
---@param rotation Quat 
---@param point Vec3 
---@return Segment3d
function Segment3d:rotated_around(_self,rotation,point) end

---@param _self Segment3d 
---@param rotation Quat 
---@return Segment3d
function Segment3d:rotated_around_center(_self,rotation) end

---@param _self Segment3d 
---@param other Segment3d 
---@return boolean
function Segment3d:eq(_self,other) end

---@param _self Segment3d 
---@return Segment3d
function Segment3d:clone(_self) end

---@param _self Segment3d 
---@return Vec3
function Segment3d:point2(_self) end

---@param _self Segment3d 
---@return Segment3d
function Segment3d:reversed(_self) end

---@param ray Ray3d 
---@param length number 
---@return Segment3d
function Segment3d.from_ray_and_length(ray,length) end

---@param _self Segment3d 
---@param length number 
---@return Segment3d
function Segment3d:resized(_self,length) end

---@param _self Segment3d 
---@return Segment3d
function Segment3d:centered(_self) end

---@param point1 Vec3 
---@param point2 Vec3 
---@return Segment3d
function Segment3d.new(point1,point2) end


---@class Sphere : ReflectReference
---  A sphere primitive, representing the set of all points some distance from the origin
---@field  radius ? number
Sphere = {}

---@param _self Sphere 
---@param other Sphere 
---@return boolean
function Sphere:eq(_self,other) end

---@param _self Sphere 
---@return number
function Sphere:diameter(_self) end

---@param _self Sphere 
---@param point Vec3 
---@return Vec3
function Sphere:closest_point(_self,point) end

---@param _self Sphere 
---@return Sphere
function Sphere:clone(_self) end

---@param radius number 
---@return Sphere
function Sphere.new(radius) end


---@class Tetrahedron : ReflectReference
---  A tetrahedron primitive.
---@field  vertices ? [glam::Vec3; 4]
Tetrahedron = {}

---@param _self Tetrahedron 
---@return Vec3
function Tetrahedron:centroid(_self) end

---@param _self Tetrahedron 
---@return Tetrahedron
function Tetrahedron:clone(_self) end

---@param _self Tetrahedron 
---@return number
function Tetrahedron:signed_volume(_self) end

---@param _self Tetrahedron 
---@param other Tetrahedron 
---@return boolean
function Tetrahedron:eq(_self,other) end

---@param a Vec3 
---@param b Vec3 
---@param c Vec3 
---@param d Vec3 
---@return Tetrahedron
function Tetrahedron.new(a,b,c,d) end


---@class Torus : ReflectReference
---  A torus primitive, often representing a ring or donut shape
---  The set of points some distance from a circle centered at the origin
---@field  minor_radius ? number
---@field  major_radius ? number
Torus = {}

---@param _self Torus 
---@return number
function Torus:inner_radius(_self) end

---@param _self Torus 
---@return Torus
function Torus:clone(_self) end

---@param inner_radius number 
---@param outer_radius number 
---@return Torus
function Torus.new(inner_radius,outer_radius) end

---@param _self Torus 
---@return number
function Torus:outer_radius(_self) end

---@param _self Torus 
---@param other Torus 
---@return boolean
function Torus:eq(_self,other) end


---@class Triangle3d : ReflectReference
---  A 3D triangle primitive.
---@field  vertices ? [glam::Vec3; 3]
Triangle3d = {}

---@param _self Triangle3d 
---@param other Triangle3d 
---@return boolean
function Triangle3d:eq(_self,other) end

---@param _self Triangle3d 
---@return nil
function Triangle3d:reverse(_self) end

---@param a Vec3 
---@param b Vec3 
---@param c Vec3 
---@return Triangle3d
function Triangle3d.new(a,b,c) end

---@param _self Triangle3d 
---@return boolean
function Triangle3d:is_obtuse(_self) end

---@param _self Triangle3d 
---@return Triangle3d
function Triangle3d:clone(_self) end

---@param _self Triangle3d 
---@return boolean
function Triangle3d:is_degenerate(_self) end

---@param _self Triangle3d 
---@return Vec3
function Triangle3d:circumcenter(_self) end

---@param _self Triangle3d 
---@return Vec3
function Triangle3d:centroid(_self) end

---@param _self Triangle3d 
---@return Triangle3d
function Triangle3d:reversed(_self) end

---@param _self Triangle3d 
---@return boolean
function Triangle3d:is_acute(_self) end


---@class Ray2d : ReflectReference
---  An infinite half-line starting at `origin` and going in `direction` in 2D space.
---@field  origin ? Vec2
---@field  direction ? Dir2
Ray2d = {}

---@param _self Ray2d 
---@param distance number 
---@return Vec2
function Ray2d:get_point(_self,distance) end

---@param _self Ray2d 
---@param plane_origin Vec2 
---@param plane Plane2d 
---@return number | nil
function Ray2d:intersect_plane(_self,plane_origin,plane) end

---@param _self Ray2d 
---@return Ray2d
function Ray2d:clone(_self) end

---@param _self Ray2d 
---@param other Ray2d 
---@return boolean
function Ray2d:eq(_self,other) end

---@param origin Vec2 
---@param direction Dir2 
---@return Ray2d
function Ray2d.new(origin,direction) end


---@class Ray3d : ReflectReference
---  An infinite half-line starting at `origin` and going in `direction` in 3D space.
---@field  origin ? Vec3
---@field  direction ? Dir3
Ray3d = {}

---@param origin Vec3 
---@param direction Dir3 
---@return Ray3d
function Ray3d.new(origin,direction) end

---@param _self Ray3d 
---@param other Ray3d 
---@return boolean
function Ray3d:eq(_self,other) end

---@param _self Ray3d 
---@return Ray3d
function Ray3d:clone(_self) end

---@param _self Ray3d 
---@param distance number 
---@return Vec3
function Ray3d:get_point(_self,distance) end

---@param _self Ray3d 
---@param plane_origin Vec3 
---@param plane InfinitePlane3d 
---@return number | nil
function Ray3d:intersect_plane(_self,plane_origin,plane) end


---@class IRect : ReflectReference
---  A rectangle defined by two opposite corners.
--- 
---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
---  stored in `IRect::min` and `IRect::max`, respectively. The minimum/maximum invariant
---  must be upheld by the user when directly assigning the fields, otherwise some methods
---  produce invalid results. It is generally recommended to use one of the constructor
---  methods instead, which will ensure this invariant is met, unless you already have
---  the minimum and maximum corners.
---@field  min ? IVec2
---@field  max ? IVec2
IRect = {}

---@param _self IRect 
---@return IVec2
function IRect:size(_self) end

---@param _self IRect 
---@return IRect
function IRect:clone(_self) end

---@param _self IRect 
---@return nil
function IRect:assert_receiver_is_total_eq(_self) end

---@param _self IRect 
---@return Rect
function IRect:as_rect(_self) end

---@param _self IRect 
---@return IVec2
function IRect:half_size(_self) end

---@param _self IRect 
---@return boolean
function IRect:is_empty(_self) end

---@param _self IRect 
---@return integer
function IRect:height(_self) end

---@param _self IRect 
---@param other IRect 
---@return boolean
function IRect:eq(_self,other) end

---@param _self IRect 
---@param other IRect 
---@return IRect
function IRect:union(_self,other) end

---@param _self IRect 
---@param point IVec2 
---@return boolean
function IRect:contains(_self,point) end

---@param x0 integer 
---@param y0 integer 
---@param x1 integer 
---@param y1 integer 
---@return IRect
function IRect.new(x0,y0,x1,y1) end

---@param _self IRect 
---@param other IVec2 
---@return IRect
function IRect:union_point(_self,other) end

---@param _self IRect 
---@param expansion integer 
---@return IRect
function IRect:inflate(_self,expansion) end

---@param _self IRect 
---@return URect
function IRect:as_urect(_self) end

---@param _self IRect 
---@return IVec2
function IRect:center(_self) end

---@param _self IRect 
---@param other IRect 
---@return IRect
function IRect:intersect(_self,other) end

---@param _self IRect 
---@return integer
function IRect:width(_self) end

---@param p0 IVec2 
---@param p1 IVec2 
---@return IRect
function IRect.from_corners(p0,p1) end

---@param origin IVec2 
---@param size IVec2 
---@return IRect
function IRect.from_center_size(origin,size) end

---@param origin IVec2 
---@param half_size IVec2 
---@return IRect
function IRect.from_center_half_size(origin,half_size) end


---@class Rect : ReflectReference
---  A rectangle defined by two opposite corners.
--- 
---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
---  stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant
---  must be upheld by the user when directly assigning the fields, otherwise some methods
---  produce invalid results. It is generally recommended to use one of the constructor
---  methods instead, which will ensure this invariant is met, unless you already have
---  the minimum and maximum corners.
---@field  min ? Vec2
---@field  max ? Vec2
Rect = {}

---@param _self Rect 
---@return IRect
function Rect:as_irect(_self) end

---@param _self Rect 
---@return number
function Rect:height(_self) end

---@param origin Vec2 
---@param size Vec2 
---@return Rect
function Rect.from_center_size(origin,size) end

---@param _self Rect 
---@return Vec2
function Rect:half_size(_self) end

---@param _self Rect 
---@param other Rect 
---@return Rect
function Rect:normalize(_self,other) end

---@param _self Rect 
---@return number
function Rect:width(_self) end

---@param _self Rect 
---@param expansion number 
---@return Rect
function Rect:inflate(_self,expansion) end

---@param _self Rect 
---@param other Rect 
---@return boolean
function Rect:eq(_self,other) end

---@param _self Rect 
---@return Rect
function Rect:clone(_self) end

---@param _self Rect 
---@param other Rect 
---@return Rect
function Rect:union(_self,other) end

---@param _self Rect 
---@return Vec2
function Rect:center(_self) end

---@param _self Rect 
---@return URect
function Rect:as_urect(_self) end

---@param p0 Vec2 
---@param p1 Vec2 
---@return Rect
function Rect.from_corners(p0,p1) end

---@param x0 number 
---@param y0 number 
---@param x1 number 
---@param y1 number 
---@return Rect
function Rect.new(x0,y0,x1,y1) end

---@param _self Rect 
---@return Vec2
function Rect:size(_self) end

---@param _self Rect 
---@param other Rect 
---@return Rect
function Rect:intersect(_self,other) end

---@param origin Vec2 
---@param half_size Vec2 
---@return Rect
function Rect.from_center_half_size(origin,half_size) end

---@param _self Rect 
---@param other Vec2 
---@return Rect
function Rect:union_point(_self,other) end

---@param _self Rect 
---@return boolean
function Rect:is_empty(_self) end

---@param _self Rect 
---@param point Vec2 
---@return boolean
function Rect:contains(_self,point) end


---@class URect : ReflectReference
---  A rectangle defined by two opposite corners.
--- 
---  The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
---  stored in `URect::min` and `URect::max`, respectively. The minimum/maximum invariant
---  must be upheld by the user when directly assigning the fields, otherwise some methods
---  produce invalid results. It is generally recommended to use one of the constructor
---  methods instead, which will ensure this invariant is met, unless you already have
---  the minimum and maximum corners.
---@field  min ? UVec2
---@field  max ? UVec2
URect = {}

---@param origin UVec2 
---@param half_size UVec2 
---@return URect
function URect.from_center_half_size(origin,half_size) end

---@param _self URect 
---@return boolean
function URect:is_empty(_self) end

---@param _self URect 
---@return nil
function URect:assert_receiver_is_total_eq(_self) end

---@param _self URect 
---@param expansion integer 
---@return URect
function URect:inflate(_self,expansion) end

---@param _self URect 
---@return UVec2
function URect:center(_self) end

---@param _self URect 
---@return URect
function URect:clone(_self) end

---@param _self URect 
---@param point UVec2 
---@return boolean
function URect:contains(_self,point) end

---@param _self URect 
---@param other URect 
---@return boolean
function URect:eq(_self,other) end

---@param _self URect 
---@return integer
function URect:height(_self) end

---@param _self URect 
---@return UVec2
function URect:size(_self) end

---@param x0 integer 
---@param y0 integer 
---@param x1 integer 
---@param y1 integer 
---@return URect
function URect.new(x0,y0,x1,y1) end

---@param _self URect 
---@return UVec2
function URect:half_size(_self) end

---@param _self URect 
---@param other UVec2 
---@return URect
function URect:union_point(_self,other) end

---@param _self URect 
---@return IRect
function URect:as_irect(_self) end

---@param _self URect 
---@return integer
function URect:width(_self) end

---@param _self URect 
---@return Rect
function URect:as_rect(_self) end

---@param _self URect 
---@param other URect 
---@return URect
function URect:union(_self,other) end

---@param p0 UVec2 
---@param p1 UVec2 
---@return URect
function URect.from_corners(p0,p1) end

---@param origin UVec2 
---@param size UVec2 
---@return URect
function URect.from_center_size(origin,size) end

---@param _self URect 
---@param other URect 
---@return URect
function URect:intersect(_self,other) end


---@class Rot2 : ReflectReference
---  A counterclockwise 2D rotation.
--- 
---  # Example
--- 
---  ```
---  # use approx::assert_relative_eq;
---  # use bevy_math::{Rot2, Vec2};
---  use std::f32::consts::PI;
--- 
---  // Create rotations from radians or degrees
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
---@field  cos ? number
---@field  sin ? number
Rot2 = {}

---@param _self Rot2 
---@param other Rot2 
---@return number
function Rot2:angle_to(_self,other) end

---@param degrees number 
---@return Rot2
function Rot2.degrees(degrees) end

---@param _self Rot2 
---@return number
function Rot2:as_radians(_self) end

---@param p1 Rot2 
---@param p2 Vec2 
---@return Vec2
function Rot2:mul(p1,p2) end

---@param _self Rot2 
---@return boolean
function Rot2:is_finite(_self) end

---@param _self Rot2 
---@return number
function Rot2:length_recip(_self) end

---@param _self Rot2 
---@return number
function Rot2:as_turn_fraction(_self) end

---@param _self Rot2 
---@param _end Rot2 
---@param s number 
---@return Rot2
function Rot2:slerp(_self,_end,s) end

---@param _self Rot2 
---@return number
function Rot2:length_squared(_self) end

---@param _self Rot2 
---@param rhs Rot2 
---@return Rot2
function Rot2:mul(_self,rhs) end

---@param _self Rot2 
---@return Rot2
function Rot2:inverse(_self) end

---@param _self Rot2 
---@return Rot2
function Rot2:normalize(_self) end

---@param _self Rot2 
---@param other Rot2 
---@return boolean
function Rot2:eq(_self,other) end

---@param radians number 
---@return Rot2
function Rot2.radians(radians) end

---@param _self Rot2 
---@return Rot2
function Rot2:fast_renormalize(_self) end

---@param _self Rot2 
---@return Rot2
function Rot2:clone(_self) end

---@param _self Rot2 
---@return boolean
function Rot2:is_near_identity(_self) end

---@param _self Rot2 
---@return boolean
function Rot2:is_nan(_self) end

---@param _self Rot2 
---@return [number, number]
function Rot2:sin_cos(_self) end

---@param p1 Rot2 
---@param p2 Dir2 
---@return Dir2
function Rot2:mul(p1,p2) end

---@param sin number 
---@param cos number 
---@return Rot2
function Rot2.from_sin_cos(sin,cos) end

---@param _self Rot2 
---@param _end Rot2 
---@param s number 
---@return Rot2
function Rot2:nlerp(_self,_end,s) end

---@param _self Rot2 
---@return boolean
function Rot2:is_normalized(_self) end

---@param _self Rot2 
---@return number
function Rot2:as_degrees(_self) end

---@param _self Rot2 
---@return number
function Rot2:length(_self) end

---@param fraction number 
---@return Rot2
function Rot2.turn_fraction(fraction) end


---@class Instant : ReflectReference
Instant = {}

---@param _self Instant 
---@param other Duration 
---@return Instant
function Instant:add(_self,other) end

---@param _self Instant 
---@param earlier Instant 
---@return Duration
function Instant:saturating_duration_since(_self,earlier) end

---@param p1 Instant 
---@param p2 Instant 
---@return Duration
function Instant:sub(p1,p2) end

---@param _self Instant 
---@param earlier Instant 
---@return Duration
function Instant:duration_since(_self,earlier) end

---@param _self Instant 
---@param other Duration 
---@return Instant
function Instant:sub(_self,other) end

---@param _self Instant 
---@param other Instant 
---@return boolean
function Instant:eq(_self,other) end

---@param _self Instant 
---@return Instant
function Instant:clone(_self) end

---@param _self Instant 
---@return nil
function Instant:assert_receiver_is_total_eq(_self) end

---@param _self Instant 
---@return Duration
function Instant:elapsed(_self) end

---@return Instant
function Instant.now() end


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
---@field  timestep ? Duration
---@field  overstep ? Duration
Fixed = {}

---@param _self Fixed 
---@return Fixed
function Fixed:clone(_self) end


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
---@field  startup ? Instant
---@field  first_update ? Option
---@field  last_update ? Option
Real = {}

---@param _self Real 
---@return Real
function Real:clone(_self) end


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
---@field  elapsed ? Duration
---@field  is_paused ? boolean
Stopwatch = {}

---@param _self Stopwatch 
---@return nil
function Stopwatch:pause(_self) end

---@return Stopwatch
function Stopwatch.new() end

---@param _self Stopwatch 
---@return Stopwatch
function Stopwatch:clone(_self) end

---@param _self Stopwatch 
---@return number
function Stopwatch:elapsed_secs(_self) end

---@param _self Stopwatch 
---@param time Duration 
---@return nil
function Stopwatch:set_elapsed(_self,time) end

---@param _self Stopwatch 
---@param other Stopwatch 
---@return boolean
function Stopwatch:eq(_self,other) end

---@param _self Stopwatch 
---@return nil
function Stopwatch:reset(_self) end

---@param _self Stopwatch 
---@return Duration
function Stopwatch:elapsed(_self) end

---@param _self Stopwatch 
---@return nil
function Stopwatch:unpause(_self) end

---@param _self Stopwatch 
---@return boolean
function Stopwatch:is_paused(_self) end

---@param _self Stopwatch 
---@return number
function Stopwatch:elapsed_secs_f64(_self) end

---@param _self Stopwatch 
---@return nil
function Stopwatch:assert_receiver_is_total_eq(_self) end


---@class Timer : ReflectReference
---  Tracks elapsed time. Enters the finished state once `duration` is reached.
--- 
---  Non repeating timers will stop tracking and stay in the finished state until reset.
---  Repeating timers will only be in the finished state on each tick `duration` is reached or
---  exceeded, and can still be reset at any given point.
--- 
---  Paused timers will not have elapsed time increased.
--- 
---  Note that in order to advance the timer [`tick`](Timer::tick) **MUST** be called.
---@field  stopwatch ? Stopwatch
---@field  duration ? Duration
---@field  mode ? TimerMode
---@field  finished ? boolean
---@field  times_finished_this_tick ? integer
Timer = {}

---@param duration Duration 
---@param mode TimerMode 
---@return Timer
function Timer.new(duration,mode) end

---@param _self Timer 
---@param other Timer 
---@return boolean
function Timer:eq(_self,other) end

---@param _self Timer 
---@return nil
function Timer:unpause(_self) end

---@param _self Timer 
---@return boolean
function Timer:just_finished(_self) end

---@param _self Timer 
---@param duration Duration 
---@return nil
function Timer:set_duration(_self,duration) end

---@param _self Timer 
---@return boolean
function Timer:paused(_self) end

---@param _self Timer 
---@return TimerMode
function Timer:mode(_self) end

---@param _self Timer 
---@return Duration
function Timer:duration(_self) end

---@param _self Timer 
---@param time Duration 
---@return nil
function Timer:set_elapsed(_self,time) end

---@param duration number 
---@param mode TimerMode 
---@return Timer
function Timer.from_seconds(duration,mode) end

---@param _self Timer 
---@return nil
function Timer:reset(_self) end

---@param _self Timer 
---@return boolean
function Timer:finished(_self) end

---@param _self Timer 
---@return Duration
function Timer:remaining(_self) end

---@param _self Timer 
---@return number
function Timer:elapsed_secs(_self) end

---@param _self Timer 
---@return number
function Timer:remaining_secs(_self) end

---@param _self Timer 
---@return nil
function Timer:assert_receiver_is_total_eq(_self) end

---@param _self Timer 
---@return number
function Timer:fraction_remaining(_self) end

---@param _self Timer 
---@return number
function Timer:fraction(_self) end

---@param _self Timer 
---@return number
function Timer:elapsed_secs_f64(_self) end

---@param _self Timer 
---@return integer
function Timer:times_finished_this_tick(_self) end

---@param _self Timer 
---@return Duration
function Timer:elapsed(_self) end

---@param _self Timer 
---@param mode TimerMode 
---@return nil
function Timer:set_mode(_self,mode) end

---@param _self Timer 
---@return Timer
function Timer:clone(_self) end

---@param _self Timer 
---@return nil
function Timer:pause(_self) end


---@class TimerMode : ReflectReference
---  Specifies [`Timer`] behavior.
TimerMode = {}

---@param _self TimerMode 
---@return TimerMode
function TimerMode:clone(_self) end

---@param _self TimerMode 
---@param other TimerMode 
---@return boolean
function TimerMode:eq(_self,other) end

---@param _self TimerMode 
---@return nil
function TimerMode:assert_receiver_is_total_eq(_self) end


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
---  The virtual clock can be paused by calling [`pause()`](Time::pause) and
---  unpaused by calling [`unpause()`](Time::unpause). When the game clock is
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
---@field  max_delta ? Duration
---@field  paused ? boolean
---@field  relative_speed ? number
---@field  effective_speed ? number
Virtual = {}

---@param _self Virtual 
---@return Virtual
function Virtual:clone(_self) end


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
---  entity which has a Transform. This is done automatically by Bevy-internal systems in the system set
---  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
--- 
---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
---  update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag
---  before the [`GlobalTransform`] is updated.
--- 
---  # Examples
--- 
---  - [`transform`][transform_example]
--- 
---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  [1] ? Affine3A
GlobalTransform = {}

---@param scale Vec3 
---@return GlobalTransform
function GlobalTransform.from_scale(scale) end

---@param x number 
---@param y number 
---@param z number 
---@return GlobalTransform
function GlobalTransform.from_xyz(x,y,z) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:left(_self) end

---@param _self GlobalTransform 
---@param value Vec3 
---@return Vec3
function GlobalTransform:mul(_self,value) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:up(_self) end

---@param _self GlobalTransform 
---@return Vec3A
function GlobalTransform:translation_vec3a(_self) end

---@param _self GlobalTransform 
---@param point Vec3 
---@return Vec3
function GlobalTransform:transform_point(_self,point) end

---@param rotation Quat 
---@return GlobalTransform
function GlobalTransform.from_rotation(rotation) end

---@param _self GlobalTransform 
---@return Quat
function GlobalTransform:rotation(_self) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:forward(_self) end

---@param _self GlobalTransform 
---@return Vec3
function GlobalTransform:scale(_self) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:down(_self) end

---@param p1 GlobalTransform 
---@param p2 GlobalTransform 
---@return GlobalTransform
function GlobalTransform:mul(p1,p2) end

---@param iso Isometry3d 
---@return GlobalTransform
function GlobalTransform.from_isometry(iso) end

---@param _self GlobalTransform 
---@return GlobalTransform
function GlobalTransform:clone(_self) end

---@param _self GlobalTransform 
---@return Transform
function GlobalTransform:compute_transform(_self) end

---@param _self GlobalTransform 
---@return Affine3A
function GlobalTransform:affine(_self) end

---@param _self GlobalTransform 
---@param parent GlobalTransform 
---@return Transform
function GlobalTransform:reparented_to(_self,parent) end

---@param p1 GlobalTransform 
---@param p2 Transform 
---@return GlobalTransform
function GlobalTransform:mul(p1,p2) end

---@param _self GlobalTransform 
---@return Vec3
function GlobalTransform:translation(_self) end

---@param _self GlobalTransform 
---@return Mat4
function GlobalTransform:compute_matrix(_self) end

---@param _self GlobalTransform 
---@param extents Vec3A 
---@return number
function GlobalTransform:radius_vec3a(_self,extents) end

---@param _self GlobalTransform 
---@param transform Transform 
---@return GlobalTransform
function GlobalTransform:mul_transform(_self,transform) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:back(_self) end

---@param _self GlobalTransform 
---@return Dir3
function GlobalTransform:right(_self) end

---@param translation Vec3 
---@return GlobalTransform
function GlobalTransform.from_translation(translation) end

---@param _self GlobalTransform 
---@return Isometry3d
function GlobalTransform:to_isometry(_self) end

---@param _self GlobalTransform 
---@param other GlobalTransform 
---@return boolean
function GlobalTransform:eq(_self,other) end


---@class Transform : ReflectReference
---  Describe the position of an entity. If the entity has a parent, the position is relative
---  to its parent position.
--- 
---  * To place or move an entity, you should set its [`Transform`].
---  * To get the global transform of an entity, you should get its [`GlobalTransform`].
---  * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
---    [`GlobalTransform`] is automatically inserted whenever [`Transform`] is inserted.
--- 
---  ## [`Transform`] and [`GlobalTransform`]
--- 
---  [`Transform`] is the position of an entity relative to its parent position, or the reference
---  frame if it doesn't have a [`ChildOf`](bevy_ecs::hierarchy::ChildOf) component.
--- 
---  [`GlobalTransform`] is the position of an entity relative to the reference frame.
--- 
---  [`GlobalTransform`] is updated from [`Transform`] by systems in the system set
---  [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
--- 
---  This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
---  update the [`Transform`] of an entity during this set or after, you will notice a 1 frame lag
---  before the [`GlobalTransform`] is updated.
--- 
---  # Examples
--- 
---  - [`transform`][transform_example]
--- 
---  [transform_example]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
---@field  translation ? Vec3
---@field  rotation ? Quat
---@field  scale ? Vec3
Transform = {}

---@param _self Transform 
---@return boolean
function Transform:is_finite(_self) end

---@param _self Transform 
---@param point Vec3 
---@param rotation Quat 
---@return nil
function Transform:rotate_around(_self,point,rotation) end

---@param translation Vec3 
---@return Transform
function Transform.from_translation(translation) end

---@param _self Transform 
---@return Dir3
function Transform:right(_self) end

---@param p1 Transform 
---@param p2 GlobalTransform 
---@return GlobalTransform
function Transform:mul(p1,p2) end

---@param _self Transform 
---@param value Vec3 
---@return Vec3
function Transform:mul(_self,value) end

---@param _self Transform 
---@return Dir3
function Transform:left(_self) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_local_z(_self,angle) end

---@param _self Transform 
---@return Dir3
function Transform:local_y(_self) end

---@param _self Transform 
---@param point Vec3 
---@param rotation Quat 
---@return nil
function Transform:translate_around(_self,point,rotation) end

---@param _self Transform 
---@return Affine3A
function Transform:compute_affine(_self) end

---@param _self Transform 
---@param translation Vec3 
---@return Transform
function Transform:with_translation(_self,translation) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_z(_self,angle) end

---@param _self Transform 
---@param rotation Quat 
---@return nil
function Transform:rotate(_self,rotation) end

---@param _self Transform 
---@return Dir3
function Transform:forward(_self) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_x(_self,angle) end

---@param _self Transform 
---@param rotation Quat 
---@return Transform
function Transform:with_rotation(_self,rotation) end

---@param _self Transform 
---@return Dir3
function Transform:up(_self) end

---@param _self Transform 
---@return Dir3
function Transform:local_x(_self) end

---@param _self Transform 
---@param point Vec3 
---@return Vec3
function Transform:transform_point(_self,point) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_y(_self,angle) end

---@param _self Transform 
---@param transform Transform 
---@return Transform
function Transform:mul_transform(_self,transform) end

---@param _self Transform 
---@return Dir3
function Transform:down(_self) end

---@param _self Transform 
---@param other Transform 
---@return boolean
function Transform:eq(_self,other) end

---@param x number 
---@param y number 
---@param z number 
---@return Transform
function Transform.from_xyz(x,y,z) end

---@param rotation Quat 
---@return Transform
function Transform.from_rotation(rotation) end

---@param _self Transform 
---@param axis Dir3 
---@param angle number 
---@return nil
function Transform:rotate_axis(_self,axis,angle) end

---@param _self Transform 
---@param scale Vec3 
---@return Transform
function Transform:with_scale(_self,scale) end

---@param _self Transform 
---@return Transform
function Transform:clone(_self) end

---@param iso Isometry3d 
---@return Transform
function Transform.from_isometry(iso) end

---@param _self Transform 
---@return Dir3
function Transform:back(_self) end

---@param _self Transform 
---@return Mat4
function Transform:compute_matrix(_self) end

---@param p1 Transform 
---@param p2 Transform 
---@return Transform
function Transform:mul(p1,p2) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_local_y(_self,angle) end

---@param _self Transform 
---@return Dir3
function Transform:local_z(_self) end

---@param world_from_local Mat4 
---@return Transform
function Transform.from_matrix(world_from_local) end

---@param scale Vec3 
---@return Transform
function Transform.from_scale(scale) end

---@param _self Transform 
---@param angle number 
---@return nil
function Transform:rotate_local_x(_self,angle) end

---@param _self Transform 
---@param axis Dir3 
---@param angle number 
---@return nil
function Transform:rotate_local_axis(_self,axis,angle) end

---@param _self Transform 
---@param rotation Quat 
---@return nil
function Transform:rotate_local(_self,rotation) end

---@param _self Transform 
---@return Isometry3d
function Transform:to_isometry(_self) end


---@class TransformTreeChanged : ReflectReference
---  An optimization for transform propagation. This ZST marker component uses change detection to
---  mark all entities of the hierarchy as "dirty" if any of their descendants have a changed
---  `Transform`. If this component is *not* marked `is_changed()`, propagation will halt.
TransformTreeChanged = {}

---@param _self TransformTreeChanged 
---@return TransformTreeChanged
function TransformTreeChanged:clone(_self) end

---@param _self TransformTreeChanged 
---@param other TransformTreeChanged 
---@return boolean
function TransformTreeChanged:eq(_self,other) end


---@class TypeId : ReflectReference
TypeId = {}

---@param _self TypeId 
---@param other TypeId 
---@return boolean
function TypeId:eq(_self,other) end

---@param _self TypeId 
---@return nil
function TypeId:assert_receiver_is_total_eq(_self) end

---@param _self TypeId 
---@return TypeId
function TypeId:clone(_self) end


---@class SocketAddr : ReflectReference
SocketAddr = {}

---@param _self SocketAddr 
---@param new_port integer 
---@return nil
function SocketAddr:set_port(_self,new_port) end

---@param _self SocketAddr 
---@return boolean
function SocketAddr:is_ipv4(_self) end

---@param _self SocketAddr 
---@param other SocketAddr 
---@return boolean
function SocketAddr:eq(_self,other) end

---@param _self SocketAddr 
---@return nil
function SocketAddr:assert_receiver_is_total_eq(_self) end

---@param _self SocketAddr 
---@return SocketAddr
function SocketAddr:clone(_self) end

---@param _self SocketAddr 
---@return integer
function SocketAddr:port(_self) end

---@param _self SocketAddr 
---@return boolean
function SocketAddr:is_ipv6(_self) end


---@class RangeFull : ReflectReference
RangeFull = {}

---@param _self RangeFull 
---@param other RangeFull 
---@return boolean
function RangeFull:eq(_self,other) end

---@param _self RangeFull 
---@return RangeFull
function RangeFull:clone(_self) end

---@param _self RangeFull 
---@return nil
function RangeFull:assert_receiver_is_total_eq(_self) end


---@class AtomicBool : ReflectReference
AtomicBool = {}

---@param _self AtomicBool 
---@return boolean
function AtomicBool:into_inner(_self) end

---@param v boolean 
---@return AtomicBool
function AtomicBool.new(v) end


---@class AtomicI16 : ReflectReference
AtomicI16 = {}

---@param v integer 
---@return AtomicI16
function AtomicI16.new(v) end

---@param _self AtomicI16 
---@return integer
function AtomicI16:into_inner(_self) end


---@class AtomicI32 : ReflectReference
AtomicI32 = {}

---@param _self AtomicI32 
---@return integer
function AtomicI32:into_inner(_self) end

---@param v integer 
---@return AtomicI32
function AtomicI32.new(v) end


---@class AtomicI64 : ReflectReference
AtomicI64 = {}

---@param _self AtomicI64 
---@return integer
function AtomicI64:into_inner(_self) end

---@param v integer 
---@return AtomicI64
function AtomicI64.new(v) end


---@class AtomicI8 : ReflectReference
AtomicI8 = {}

---@param v integer 
---@return AtomicI8
function AtomicI8.new(v) end

---@param _self AtomicI8 
---@return integer
function AtomicI8:into_inner(_self) end


---@class AtomicIsize : ReflectReference
AtomicIsize = {}

---@param v integer 
---@return AtomicIsize
function AtomicIsize.new(v) end

---@param _self AtomicIsize 
---@return integer
function AtomicIsize:into_inner(_self) end


---@class AtomicU16 : ReflectReference
AtomicU16 = {}

---@param _self AtomicU16 
---@return integer
function AtomicU16:into_inner(_self) end

---@param v integer 
---@return AtomicU16
function AtomicU16.new(v) end


---@class AtomicU32 : ReflectReference
AtomicU32 = {}

---@param _self AtomicU32 
---@return integer
function AtomicU32:into_inner(_self) end

---@param v integer 
---@return AtomicU32
function AtomicU32.new(v) end


---@class AtomicU64 : ReflectReference
AtomicU64 = {}

---@param _self AtomicU64 
---@return integer
function AtomicU64:into_inner(_self) end

---@param v integer 
---@return AtomicU64
function AtomicU64.new(v) end


---@class AtomicU8 : ReflectReference
AtomicU8 = {}

---@param v integer 
---@return AtomicU8
function AtomicU8.new(v) end

---@param _self AtomicU8 
---@return integer
function AtomicU8:into_inner(_self) end


---@class AtomicUsize : ReflectReference
AtomicUsize = {}

---@param v integer 
---@return AtomicUsize
function AtomicUsize.new(v) end

---@param _self AtomicUsize 
---@return integer
function AtomicUsize:into_inner(_self) end


---@class Duration : ReflectReference
Duration = {}

---@param _self Duration 
---@return integer
function Duration:as_secs(_self) end

---@param _self Duration 
---@param other Duration 
---@return Duration
function Duration:abs_diff(_self,other) end

---@param _self Duration 
---@param rhs Duration 
---@return Duration
function Duration:sub(_self,rhs) end

---@param _self Duration 
---@param rhs integer 
---@return Duration
function Duration:saturating_mul(_self,rhs) end

---@param _self Duration 
---@param rhs number 
---@return Duration
function Duration:div_f64(_self,rhs) end

---@param _self Duration 
---@param rhs Duration 
---@return Duration
function Duration:saturating_sub(_self,rhs) end

---@param _self Duration 
---@return integer
function Duration:as_nanos(_self) end

---@param _self Duration 
---@return integer
function Duration:subsec_nanos(_self) end

---@param secs integer 
---@return Duration
function Duration.from_secs(secs) end

---@param _self Duration 
---@return integer
function Duration:as_micros(_self) end

---@param _self Duration 
---@return number
function Duration:as_secs_f64(_self) end

---@param _self Duration 
---@param rhs integer 
---@return Duration
function Duration:div(_self,rhs) end

---@param _self Duration 
---@return integer
function Duration:subsec_micros(_self) end

---@param secs number 
---@return Duration
function Duration.from_secs_f32(secs) end

---@param _self Duration 
---@param rhs number 
---@return Duration
function Duration:mul_f32(_self,rhs) end

---@param _self Duration 
---@param rhs integer 
---@return Duration
function Duration:mul(_self,rhs) end

---@param nanos integer 
---@return Duration
function Duration.from_nanos(nanos) end

---@param _self Duration 
---@return integer
function Duration:as_millis(_self) end

---@param _self Duration 
---@return number
function Duration:as_secs_f32(_self) end

---@param _self Duration 
---@param rhs Duration 
---@return number
function Duration:div_duration_f32(_self,rhs) end

---@param _self Duration 
---@return nil
function Duration:assert_receiver_is_total_eq(_self) end

---@param secs number 
---@return Duration
function Duration.from_secs_f64(secs) end

---@param _self Duration 
---@param other Duration 
---@return boolean
function Duration:eq(_self,other) end

---@param _self Duration 
---@param rhs number 
---@return Duration
function Duration:mul_f64(_self,rhs) end

---@param _self Duration 
---@param rhs number 
---@return Duration
function Duration:div_f32(_self,rhs) end

---@param micros integer 
---@return Duration
function Duration.from_micros(micros) end

---@param _self Duration 
---@param rhs Duration 
---@return number
function Duration:div_duration_f64(_self,rhs) end

---@param _self Duration 
---@return integer
function Duration:subsec_millis(_self) end

---@param _self Duration 
---@param rhs Duration 
---@return Duration
function Duration:saturating_add(_self,rhs) end

---@param _self Duration 
---@return boolean
function Duration:is_zero(_self) end

---@param _self Duration 
---@param rhs Duration 
---@return Duration
function Duration:add(_self,rhs) end

---@param secs integer 
---@param nanos integer 
---@return Duration
function Duration.new(secs,nanos) end

---@param millis integer 
---@return Duration
function Duration.from_millis(millis) end

---@param _self Duration 
---@return Duration
function Duration:clone(_self) end


---@class Affine2 : ReflectReference
---@field  matrix2 ? Mat2
---@field  translation ? Vec2
Affine2 = {}

---@param matrix2 Mat2 
---@param translation Vec2 
---@return Affine2
function Affine2.from_mat2_translation(matrix2,translation) end

---@param scale Vec2 
---@return Affine2
function Affine2.from_scale(scale) end

---@param matrix2 Mat2 
---@return Affine2
function Affine2.from_mat2(matrix2) end

---@param _self Affine2 
---@param rhs Affine2 
---@return boolean
function Affine2:eq(_self,rhs) end

---@param _self Affine2 
---@return number[][]
function Affine2:to_cols_array_2d(_self) end

---@param _self Affine2 
---@return boolean
function Affine2:is_finite(_self) end

---@param translation Vec2 
---@return Affine2
function Affine2.from_translation(translation) end

---@param _self Affine2 
---@param rhs Affine2 
---@return Affine2
function Affine2:mul(_self,rhs) end

---@param _self Affine2 
---@return Affine2
function Affine2:clone(_self) end

---@param m Mat3 
---@return Affine2
function Affine2.from_mat3(m) end

---@param p1 Affine2 
---@param p2 Mat3A 
---@return Mat3A
function Affine2:mul(p1,p2) end

---@param _self Affine2 
---@param rhs Affine2 
---@param max_abs_diff number 
---@return boolean
function Affine2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param angle number 
---@return Affine2
function Affine2.from_angle(angle) end

---@param _self Affine2 
---@return Affine2
function Affine2:inverse(_self) end

---@param p1 Affine2 
---@param p2 Mat3 
---@return Mat3
function Affine2:mul(p1,p2) end

---@param x_axis Vec2 
---@param y_axis Vec2 
---@param z_axis Vec2 
---@return Affine2
function Affine2.from_cols(x_axis,y_axis,z_axis) end

---@param m Mat3A 
---@return Affine2
function Affine2.from_mat3a(m) end

---@param _self Affine2 
---@param rhs Vec2 
---@return Vec2
function Affine2:transform_point2(_self,rhs) end

---@param _self Affine2 
---@return boolean
function Affine2:is_nan(_self) end

---@param _self Affine2 
---@return number[]
function Affine2:to_cols_array(_self) end

---@param _self Affine2 
---@param rhs Vec2 
---@return Vec2
function Affine2:transform_vector2(_self,rhs) end

---@param scale Vec2 
---@param angle number 
---@param translation Vec2 
---@return Affine2
function Affine2.from_scale_angle_translation(scale,angle,translation) end

---@param angle number 
---@param translation Vec2 
---@return Affine2
function Affine2.from_angle_translation(angle,translation) end


---@class Affine3A : ReflectReference
---@field  matrix3 ? Mat3A
---@field  translation ? Vec3A
Affine3A = {}

---@param axis Vec3 
---@param angle number 
---@return Affine3A
function Affine3A.from_axis_angle(axis,angle) end

---@param angle number 
---@return Affine3A
function Affine3A.from_rotation_x(angle) end

---@param eye Vec3 
---@param dir Vec3 
---@param up Vec3 
---@return Affine3A
function Affine3A.look_to_lh(eye,dir,up) end

---@param _self Affine3A 
---@param rhs Vec3 
---@return Vec3
function Affine3A:transform_point3(_self,rhs) end

---@param _self Affine3A 
---@return number[][]
function Affine3A:to_cols_array_2d(_self) end

---@param mat3 Mat3 
---@return Affine3A
function Affine3A.from_mat3(mat3) end

---@param scale Vec3 
---@return Affine3A
function Affine3A.from_scale(scale) end

---@param p1 Affine3A 
---@param p2 Mat4 
---@return Mat4
function Affine3A:mul(p1,p2) end

---@param scale Vec3 
---@param rotation Quat 
---@param translation Vec3 
---@return Affine3A
function Affine3A.from_scale_rotation_translation(scale,rotation,translation) end

---@param _self Affine3A 
---@param rhs Affine3A 
---@return boolean
function Affine3A:eq(_self,rhs) end

---@param eye Vec3 
---@param center Vec3 
---@param up Vec3 
---@return Affine3A
function Affine3A.look_at_rh(eye,center,up) end

---@param angle number 
---@return Affine3A
function Affine3A.from_rotation_z(angle) end

---@param rotation Quat 
---@param translation Vec3 
---@return Affine3A
function Affine3A.from_rotation_translation(rotation,translation) end

---@param eye Vec3 
---@param center Vec3 
---@param up Vec3 
---@return Affine3A
function Affine3A.look_at_lh(eye,center,up) end

---@param _self Affine3A 
---@return Affine3A
function Affine3A:inverse(_self) end

---@param _self Affine3A 
---@param rhs Affine3A 
---@param max_abs_diff number 
---@return boolean
function Affine3A:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param mat3 Mat3 
---@param translation Vec3 
---@return Affine3A
function Affine3A.from_mat3_translation(mat3,translation) end

---@param _self Affine3A 
---@param rhs Affine3A 
---@return Affine3A
function Affine3A:mul(_self,rhs) end

---@param translation Vec3 
---@return Affine3A
function Affine3A.from_translation(translation) end

---@param _self Affine3A 
---@param rhs Vec3A 
---@return Vec3A
function Affine3A:transform_point3a(_self,rhs) end

---@param rotation Quat 
---@return Affine3A
function Affine3A.from_quat(rotation) end

---@param _self Affine3A 
---@param rhs Vec3 
---@return Vec3
function Affine3A:transform_vector3(_self,rhs) end

---@param x_axis Vec3A 
---@param y_axis Vec3A 
---@param z_axis Vec3A 
---@param w_axis Vec3A 
---@return Affine3A
function Affine3A.from_cols(x_axis,y_axis,z_axis,w_axis) end

---@param _self Affine3A 
---@return boolean
function Affine3A:is_nan(_self) end

---@param _self Affine3A 
---@return number[]
function Affine3A:to_cols_array(_self) end

---@param angle number 
---@return Affine3A
function Affine3A.from_rotation_y(angle) end

---@param _self Affine3A 
---@return Affine3A
function Affine3A:clone(_self) end

---@param _self Affine3A 
---@return boolean
function Affine3A:is_finite(_self) end

---@param eye Vec3 
---@param dir Vec3 
---@param up Vec3 
---@return Affine3A
function Affine3A.look_to_rh(eye,dir,up) end

---@param m Mat4 
---@return Affine3A
function Affine3A.from_mat4(m) end

---@param _self Affine3A 
---@param rhs Vec3A 
---@return Vec3A
function Affine3A:transform_vector3a(_self,rhs) end


---@class BVec2 : ReflectReference
---@field  x ? boolean
---@field  y ? boolean
BVec2 = {}

---@param _self BVec2 
---@param index integer 
---@param value boolean 
---@return nil
function BVec2:set(_self,index,value) end

---@param _self BVec2 
---@return BVec2
function BVec2:clone(_self) end

---@param v boolean 
---@return BVec2
function BVec2.splat(v) end

---@param x boolean 
---@param y boolean 
---@return BVec2
function BVec2.new(x,y) end

---@param _self BVec2 
---@param index integer 
---@return boolean
function BVec2:test(_self,index) end

---@param _self BVec2 
---@param other BVec2 
---@return boolean
function BVec2:eq(_self,other) end

---@param _self BVec2 
---@return integer
function BVec2:bitmask(_self) end

---@param _self BVec2 
---@return nil
function BVec2:assert_receiver_is_total_eq(_self) end

---@param _self BVec2 
---@return boolean
function BVec2:all(_self) end

---@param _self BVec2 
---@return boolean
function BVec2:any(_self) end

---@param a boolean[] 
---@return BVec2
function BVec2.from_array(a) end


---@class BVec3 : ReflectReference
---@field  x ? boolean
---@field  y ? boolean
---@field  z ? boolean
BVec3 = {}

---@param v boolean 
---@return BVec3
function BVec3.splat(v) end

---@param _self BVec3 
---@return boolean
function BVec3:any(_self) end

---@param a boolean[] 
---@return BVec3
function BVec3.from_array(a) end

---@param _self BVec3 
---@param index integer 
---@param value boolean 
---@return nil
function BVec3:set(_self,index,value) end

---@param _self BVec3 
---@param index integer 
---@return boolean
function BVec3:test(_self,index) end

---@param _self BVec3 
---@param other BVec3 
---@return boolean
function BVec3:eq(_self,other) end

---@param _self BVec3 
---@return BVec3
function BVec3:clone(_self) end

---@param _self BVec3 
---@return nil
function BVec3:assert_receiver_is_total_eq(_self) end

---@param _self BVec3 
---@return boolean
function BVec3:all(_self) end

---@param _self BVec3 
---@return integer
function BVec3:bitmask(_self) end

---@param x boolean 
---@param y boolean 
---@param z boolean 
---@return BVec3
function BVec3.new(x,y,z) end


---@class BVec3A : ReflectReference
BVec3A = {}

---@param x boolean 
---@param y boolean 
---@param z boolean 
---@return BVec3A
function BVec3A.new(x,y,z) end

---@param _self BVec3A 
---@return integer
function BVec3A:bitmask(_self) end

---@param _self BVec3A 
---@return BVec3A
function BVec3A:clone(_self) end

---@param _self BVec3A 
---@return boolean
function BVec3A:any(_self) end

---@param _self BVec3A 
---@param index integer 
---@param value boolean 
---@return nil
function BVec3A:set(_self,index,value) end

---@param a boolean[] 
---@return BVec3A
function BVec3A.from_array(a) end

---@param _self BVec3A 
---@param rhs BVec3A 
---@return boolean
function BVec3A:eq(_self,rhs) end

---@param _self BVec3A 
---@param index integer 
---@return boolean
function BVec3A:test(_self,index) end

---@param v boolean 
---@return BVec3A
function BVec3A.splat(v) end

---@param _self BVec3A 
---@return boolean
function BVec3A:all(_self) end


---@class BVec4 : ReflectReference
---@field  x ? boolean
---@field  y ? boolean
---@field  z ? boolean
---@field  w ? boolean
BVec4 = {}

---@param x boolean 
---@param y boolean 
---@param z boolean 
---@param w boolean 
---@return BVec4
function BVec4.new(x,y,z,w) end

---@param _self BVec4 
---@param index integer 
---@param value boolean 
---@return nil
function BVec4:set(_self,index,value) end

---@param _self BVec4 
---@return BVec4
function BVec4:clone(_self) end

---@param _self BVec4 
---@return nil
function BVec4:assert_receiver_is_total_eq(_self) end

---@param _self BVec4 
---@param other BVec4 
---@return boolean
function BVec4:eq(_self,other) end

---@param _self BVec4 
---@return integer
function BVec4:bitmask(_self) end

---@param _self BVec4 
---@return boolean
function BVec4:all(_self) end

---@param a boolean[] 
---@return BVec4
function BVec4.from_array(a) end

---@param _self BVec4 
---@return boolean
function BVec4:any(_self) end

---@param v boolean 
---@return BVec4
function BVec4.splat(v) end

---@param _self BVec4 
---@param index integer 
---@return boolean
function BVec4:test(_self,index) end


---@class BVec4A : ReflectReference
BVec4A = {}

---@param _self BVec4A 
---@param index integer 
---@return boolean
function BVec4A:test(_self,index) end

---@param _self BVec4A 
---@return BVec4A
function BVec4A:clone(_self) end

---@param _self BVec4A 
---@return boolean
function BVec4A:any(_self) end

---@param _self BVec4A 
---@param index integer 
---@param value boolean 
---@return nil
function BVec4A:set(_self,index,value) end

---@param a boolean[] 
---@return BVec4A
function BVec4A.from_array(a) end

---@param _self BVec4A 
---@return boolean
function BVec4A:all(_self) end

---@param _self BVec4A 
---@param rhs BVec4A 
---@return boolean
function BVec4A:eq(_self,rhs) end

---@param _self BVec4A 
---@return integer
function BVec4A:bitmask(_self) end

---@param x boolean 
---@param y boolean 
---@param z boolean 
---@param w boolean 
---@return BVec4A
function BVec4A.new(x,y,z,w) end

---@param v boolean 
---@return BVec4A
function BVec4A.splat(v) end


---@class DAffine2 : ReflectReference
---@field  matrix2 ? DMat2
---@field  translation ? DVec2
DAffine2 = {}

---@param angle number 
---@param translation DVec2 
---@return DAffine2
function DAffine2.from_angle_translation(angle,translation) end

---@param _self DAffine2 
---@param rhs DVec2 
---@return DVec2
function DAffine2:transform_point2(_self,rhs) end

---@param scale DVec2 
---@param angle number 
---@param translation DVec2 
---@return DAffine2
function DAffine2.from_scale_angle_translation(scale,angle,translation) end

---@param x_axis DVec2 
---@param y_axis DVec2 
---@param z_axis DVec2 
---@return DAffine2
function DAffine2.from_cols(x_axis,y_axis,z_axis) end

---@param translation DVec2 
---@return DAffine2
function DAffine2.from_translation(translation) end

---@param m DMat3 
---@return DAffine2
function DAffine2.from_mat3(m) end

---@param _self DAffine2 
---@param rhs DAffine2 
---@return boolean
function DAffine2:eq(_self,rhs) end

---@param matrix2 DMat2 
---@return DAffine2
function DAffine2.from_mat2(matrix2) end

---@param _self DAffine2 
---@return boolean
function DAffine2:is_finite(_self) end

---@param p1 DAffine2 
---@param p2 DMat3 
---@return DMat3
function DAffine2:mul(p1,p2) end

---@param matrix2 DMat2 
---@param translation DVec2 
---@return DAffine2
function DAffine2.from_mat2_translation(matrix2,translation) end

---@param _self DAffine2 
---@return number[][]
function DAffine2:to_cols_array_2d(_self) end

---@param _self DAffine2 
---@return number[]
function DAffine2:to_cols_array(_self) end

---@param _self DAffine2 
---@param rhs DAffine2 
---@return DAffine2
function DAffine2:mul(_self,rhs) end

---@param _self DAffine2 
---@param rhs DAffine2 
---@param max_abs_diff number 
---@return boolean
function DAffine2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param angle number 
---@return DAffine2
function DAffine2.from_angle(angle) end

---@param _self DAffine2 
---@param rhs DVec2 
---@return DVec2
function DAffine2:transform_vector2(_self,rhs) end

---@param scale DVec2 
---@return DAffine2
function DAffine2.from_scale(scale) end

---@param _self DAffine2 
---@return DAffine2
function DAffine2:inverse(_self) end

---@param _self DAffine2 
---@return DAffine2
function DAffine2:clone(_self) end

---@param _self DAffine2 
---@return boolean
function DAffine2:is_nan(_self) end


---@class DAffine3 : ReflectReference
---@field  matrix3 ? DMat3
---@field  translation ? DVec3
DAffine3 = {}

---@param eye DVec3 
---@param dir DVec3 
---@param up DVec3 
---@return DAffine3
function DAffine3.look_to_lh(eye,dir,up) end

---@param mat3 DMat3 
---@return DAffine3
function DAffine3.from_mat3(mat3) end

---@param axis DVec3 
---@param angle number 
---@return DAffine3
function DAffine3.from_axis_angle(axis,angle) end

---@param angle number 
---@return DAffine3
function DAffine3.from_rotation_y(angle) end

---@param angle number 
---@return DAffine3
function DAffine3.from_rotation_x(angle) end

---@param eye DVec3 
---@param center DVec3 
---@param up DVec3 
---@return DAffine3
function DAffine3.look_at_lh(eye,center,up) end

---@param _self DAffine3 
---@param rhs DVec3 
---@return DVec3
function DAffine3:transform_point3(_self,rhs) end

---@param angle number 
---@return DAffine3
function DAffine3.from_rotation_z(angle) end

---@param rotation DQuat 
---@param translation DVec3 
---@return DAffine3
function DAffine3.from_rotation_translation(rotation,translation) end

---@param rotation DQuat 
---@return DAffine3
function DAffine3.from_quat(rotation) end

---@param scale DVec3 
---@param rotation DQuat 
---@param translation DVec3 
---@return DAffine3
function DAffine3.from_scale_rotation_translation(scale,rotation,translation) end

---@param _self DAffine3 
---@param rhs DAffine3 
---@return DAffine3
function DAffine3:mul(_self,rhs) end

---@param _self DAffine3 
---@return DAffine3
function DAffine3:clone(_self) end

---@param x_axis DVec3 
---@param y_axis DVec3 
---@param z_axis DVec3 
---@param w_axis DVec3 
---@return DAffine3
function DAffine3.from_cols(x_axis,y_axis,z_axis,w_axis) end

---@param _self DAffine3 
---@return boolean
function DAffine3:is_nan(_self) end

---@param _self DAffine3 
---@param rhs DVec3 
---@return DVec3
function DAffine3:transform_vector3(_self,rhs) end

---@param _self DAffine3 
---@return DAffine3
function DAffine3:inverse(_self) end

---@param m DMat4 
---@return DAffine3
function DAffine3.from_mat4(m) end

---@param eye DVec3 
---@param center DVec3 
---@param up DVec3 
---@return DAffine3
function DAffine3.look_at_rh(eye,center,up) end

---@param translation DVec3 
---@return DAffine3
function DAffine3.from_translation(translation) end

---@param mat3 DMat3 
---@param translation DVec3 
---@return DAffine3
function DAffine3.from_mat3_translation(mat3,translation) end

---@param scale DVec3 
---@return DAffine3
function DAffine3.from_scale(scale) end

---@param _self DAffine3 
---@param rhs DAffine3 
---@param max_abs_diff number 
---@return boolean
function DAffine3:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DAffine3 
---@return boolean
function DAffine3:is_finite(_self) end

---@param p1 DAffine3 
---@param p2 DMat4 
---@return DMat4
function DAffine3:mul(p1,p2) end

---@param _self DAffine3 
---@return number[][]
function DAffine3:to_cols_array_2d(_self) end

---@param _self DAffine3 
---@param rhs DAffine3 
---@return boolean
function DAffine3:eq(_self,rhs) end

---@param eye DVec3 
---@param dir DVec3 
---@param up DVec3 
---@return DAffine3
function DAffine3.look_to_rh(eye,dir,up) end

---@param _self DAffine3 
---@return number[]
function DAffine3:to_cols_array(_self) end


---@class DMat2 : ReflectReference
---@field  x_axis ? DVec2
---@field  y_axis ? DVec2
DMat2 = {}

---@param _self DMat2 
---@return number[][]
function DMat2:to_cols_array_2d(_self) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:add(_self,rhs) end

---@param scale DVec2 
---@param angle number 
---@return DMat2
function DMat2.from_scale_angle(scale,angle) end

---@param _self DMat2 
---@param rhs number 
---@return DMat2
function DMat2:mul_scalar(_self,rhs) end

---@param _self DMat2 
---@return DMat2
function DMat2:inverse(_self) end

---@param x_axis DVec2 
---@param y_axis DVec2 
---@return DMat2
function DMat2.from_cols(x_axis,y_axis) end

---@param _self DMat2 
---@return DMat2
function DMat2:neg(_self) end

---@param _self DMat2 
---@return DMat2
function DMat2:transpose(_self) end

---@param diagonal DVec2 
---@return DMat2
function DMat2.from_diagonal(diagonal) end

---@param _self DMat2 
---@param rhs number 
---@return DMat2
function DMat2:div_scalar(_self,rhs) end

---@param _self DMat2 
---@return DMat2
function DMat2:abs(_self) end

---@param _self DMat2 
---@return Mat2
function DMat2:as_mat2(_self) end

---@param _self DMat2 
---@param rhs DMat2 
---@param max_abs_diff number 
---@return boolean
function DMat2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DMat2 
---@param rhs DMat2 
---@return boolean
function DMat2:eq(_self,rhs) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:sub(_self,rhs) end

---@param m DMat3 
---@param i integer 
---@param j integer 
---@return DMat2
function DMat2.from_mat3_minor(m,i,j) end

---@param _self DMat2 
---@param rhs DVec2 
---@return DVec2
function DMat2:mul_vec2(_self,rhs) end

---@param angle number 
---@return DMat2
function DMat2.from_angle(angle) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:mul(_self,rhs) end

---@param _self DMat2 
---@return boolean
function DMat2:is_finite(_self) end

---@param m DMat3 
---@return DMat2
function DMat2.from_mat3(m) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:sub_mat2(_self,rhs) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:mul_mat2(_self,rhs) end

---@param _self DMat2 
---@param rhs number 
---@return DMat2
function DMat2:div(_self,rhs) end

---@param _self DMat2 
---@return number
function DMat2:determinant(_self) end

---@param _self DMat2 
---@return number[]
function DMat2:to_cols_array(_self) end

---@param p1 DMat2 
---@param p2 DVec2 
---@return DVec2
function DMat2:mul(p1,p2) end

---@param _self DMat2 
---@param index integer 
---@return DVec2
function DMat2:row(_self,index) end

---@param _self DMat2 
---@return DMat2
function DMat2:clone(_self) end

---@param _self DMat2 
---@return boolean
function DMat2:is_nan(_self) end

---@param _self DMat2 
---@param index integer 
---@return DVec2
function DMat2:col(_self,index) end

---@param p1 DMat2 
---@param p2 number 
---@return DMat2
function DMat2:mul(p1,p2) end

---@param _self DMat2 
---@param rhs DMat2 
---@return DMat2
function DMat2:add_mat2(_self,rhs) end


---@class DMat3 : ReflectReference
---@field  x_axis ? DVec3
---@field  y_axis ? DVec3
---@field  z_axis ? DVec3
DMat3 = {}

---@param _self DMat3 
---@param rhs DMat3 
---@param max_abs_diff number 
---@return boolean
function DMat3:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param axis DVec3 
---@param angle number 
---@return DMat3
function DMat3.from_axis_angle(axis,angle) end

---@param _self DMat3 
---@param index integer 
---@return DVec3
function DMat3:col(_self,index) end

---@param _self DMat3 
---@param rhs DMat3 
---@return DMat3
function DMat3:add(_self,rhs) end

---@param _self DMat3 
---@param rhs number 
---@return DMat3
function DMat3:div(_self,rhs) end

---@param angle number 
---@return DMat3
function DMat3.from_rotation_x(angle) end

---@param angle number 
---@return DMat3
function DMat3.from_angle(angle) end

---@param _self DMat3 
---@param rhs number 
---@return DMat3
function DMat3:div_scalar(_self,rhs) end

---@param rotation DQuat 
---@return DMat3
function DMat3.from_quat(rotation) end

---@param _self DMat3 
---@param rhs DVec3 
---@return DVec3
function DMat3:mul_vec3(_self,rhs) end

---@param _self DMat3 
---@return DMat3
function DMat3:neg(_self) end

---@param m DMat4 
---@param i integer 
---@param j integer 
---@return DMat3
function DMat3.from_mat4_minor(m,i,j) end

---@param _self DMat3 
---@param rhs DVec2 
---@return DVec2
function DMat3:transform_point2(_self,rhs) end

---@param _self DMat3 
---@return boolean
function DMat3:is_nan(_self) end

---@param _self DMat3 
---@param rhs DAffine2 
---@return DMat3
function DMat3:mul(_self,rhs) end

---@param _self DMat3 
---@param rhs DMat3 
---@return boolean
function DMat3:eq(_self,rhs) end

---@param _self DMat3 
---@return boolean
function DMat3:is_finite(_self) end

---@param _self DMat3 
---@param rhs DMat3 
---@return DMat3
function DMat3:sub_mat3(_self,rhs) end

---@param angle number 
---@return DMat3
function DMat3.from_rotation_y(angle) end

---@param diagonal DVec3 
---@return DMat3
function DMat3.from_diagonal(diagonal) end

---@param angle number 
---@return DMat3
function DMat3.from_rotation_z(angle) end

---@param _self DMat3 
---@param index integer 
---@return DVec3
function DMat3:row(_self,index) end

---@param m DMat2 
---@return DMat3
function DMat3.from_mat2(m) end

---@param _self DMat3 
---@param rhs number 
---@return DMat3
function DMat3:mul_scalar(_self,rhs) end

---@param p1 DMat3 
---@param p2 number 
---@return DMat3
function DMat3:mul(p1,p2) end

---@param _self DMat3 
---@param rhs DVec2 
---@return DVec2
function DMat3:transform_vector2(_self,rhs) end

---@param order EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return DMat3
function DMat3.from_euler(order,a,b,c) end

---@param _self DMat3 
---@return DMat3
function DMat3:clone(_self) end

---@param _self DMat3 
---@param rhs DMat3 
---@return DMat3
function DMat3:add_mat3(_self,rhs) end

---@param scale DVec2 
---@return DMat3
function DMat3.from_scale(scale) end

---@param _self DMat3 
---@return DMat3
function DMat3:transpose(_self) end

---@param scale DVec2 
---@param angle number 
---@param translation DVec2 
---@return DMat3
function DMat3.from_scale_angle_translation(scale,angle,translation) end

---@param m DMat4 
---@return DMat3
function DMat3.from_mat4(m) end

---@param _self DMat3 
---@return number[]
function DMat3:to_cols_array(_self) end

---@param _self DMat3 
---@param rhs DMat3 
---@return DMat3
function DMat3:sub(_self,rhs) end

---@param _self DMat3 
---@return number[][]
function DMat3:to_cols_array_2d(_self) end

---@param p1 DMat3 
---@param p2 DVec3 
---@return DVec3
function DMat3:mul(p1,p2) end

---@param p1 DMat3 
---@param p2 DMat3 
---@return DMat3
function DMat3:mul(p1,p2) end

---@param _self DMat3 
---@return DMat3
function DMat3:inverse(_self) end

---@param translation DVec2 
---@return DMat3
function DMat3.from_translation(translation) end

---@param _self DMat3 
---@return Mat3
function DMat3:as_mat3(_self) end

---@param _self DMat3 
---@param order EulerRot 
---@return [number, number, number]
function DMat3:to_euler(_self,order) end

---@param _self DMat3 
---@return number
function DMat3:determinant(_self) end

---@param _self DMat3 
---@param rhs DMat3 
---@return DMat3
function DMat3:mul_mat3(_self,rhs) end

---@param x_axis DVec3 
---@param y_axis DVec3 
---@param z_axis DVec3 
---@return DMat3
function DMat3.from_cols(x_axis,y_axis,z_axis) end

---@param _self DMat3 
---@return DMat3
function DMat3:abs(_self) end


---@class DMat4 : ReflectReference
---@field  x_axis ? DVec4
---@field  y_axis ? DVec4
---@field  z_axis ? DVec4
---@field  w_axis ? DVec4
DMat4 = {}

---@param _self DMat4 
---@param rhs DAffine3 
---@return DMat4
function DMat4:mul(_self,rhs) end

---@param angle number 
---@return DMat4
function DMat4.from_rotation_z(angle) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return DMat4
function DMat4.orthographic_rh_gl(left,right,bottom,top,near,far) end

---@param _self DMat4 
---@param rhs DVec4 
---@return DVec4
function DMat4:mul_vec4(_self,rhs) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return DMat4
function DMat4.perspective_infinite_reverse_rh(fov_y_radians,aspect_ratio,z_near) end

---@param _self DMat4 
---@param rhs number 
---@return DMat4
function DMat4:div_scalar(_self,rhs) end

---@param _self DMat4 
---@return DMat4
function DMat4:neg(_self) end

---@param _self DMat4 
---@param rhs DVec3 
---@return DVec3
function DMat4:transform_point3(_self,rhs) end

---@param angle number 
---@return DMat4
function DMat4.from_rotation_x(angle) end

---@param _self DMat4 
---@return DMat4
function DMat4:abs(_self) end

---@param scale DVec3 
---@return DMat4
function DMat4.from_scale(scale) end

---@param _self DMat4 
---@param rhs DMat4 
---@param max_abs_diff number 
---@return boolean
function DMat4:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param rotation DQuat 
---@param translation DVec3 
---@return DMat4
function DMat4.from_rotation_translation(rotation,translation) end

---@param _self DMat4 
---@param rhs DMat4 
---@return DMat4
function DMat4:add(_self,rhs) end

---@param eye DVec3 
---@param dir DVec3 
---@param up DVec3 
---@return DMat4
function DMat4.look_to_lh(eye,dir,up) end

---@param order EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return DMat4
function DMat4.from_euler(order,a,b,c) end

---@param eye DVec3 
---@param center DVec3 
---@param up DVec3 
---@return DMat4
function DMat4.look_at_rh(eye,center,up) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return DMat4
function DMat4.perspective_infinite_rh(fov_y_radians,aspect_ratio,z_near) end

---@param angle number 
---@return DMat4
function DMat4.from_rotation_y(angle) end

---@param p1 DMat4 
---@param p2 DVec4 
---@return DVec4
function DMat4:mul(p1,p2) end

---@param scale DVec3 
---@param rotation DQuat 
---@param translation DVec3 
---@return DMat4
function DMat4.from_scale_rotation_translation(scale,rotation,translation) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return DMat4
function DMat4.perspective_infinite_lh(fov_y_radians,aspect_ratio,z_near) end

---@param x_axis DVec4 
---@param y_axis DVec4 
---@param z_axis DVec4 
---@param w_axis DVec4 
---@return DMat4
function DMat4.from_cols(x_axis,y_axis,z_axis,w_axis) end

---@param eye DVec3 
---@param center DVec3 
---@param up DVec3 
---@return DMat4
function DMat4.look_at_lh(eye,center,up) end

---@param _self DMat4 
---@param order EulerRot 
---@return [number, number, number]
function DMat4:to_euler(_self,order) end

---@param _self DMat4 
---@param rhs DMat4 
---@return DMat4
function DMat4:sub(_self,rhs) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return DMat4
function DMat4.perspective_rh(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return DMat4
function DMat4.perspective_rh_gl(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param _self DMat4 
---@return Mat4
function DMat4:as_mat4(_self) end

---@param p1 DMat4 
---@param p2 DMat4 
---@return DMat4
function DMat4:mul(p1,p2) end

---@param _self DMat4 
---@param rhs DMat4 
---@return DMat4
function DMat4:mul_mat4(_self,rhs) end

---@param diagonal DVec4 
---@return DMat4
function DMat4.from_diagonal(diagonal) end

---@param _self DMat4 
---@param rhs DMat4 
---@return DMat4
function DMat4:sub_mat4(_self,rhs) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return DMat4
function DMat4.orthographic_rh(left,right,bottom,top,near,far) end

---@param _self DMat4 
---@param rhs DVec3 
---@return DVec3
function DMat4:project_point3(_self,rhs) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return DMat4
function DMat4.orthographic_lh(left,right,bottom,top,near,far) end

---@param _self DMat4 
---@return number
function DMat4:determinant(_self) end

---@param axis DVec3 
---@param angle number 
---@return DMat4
function DMat4.from_axis_angle(axis,angle) end

---@param _self DMat4 
---@param rhs DVec3 
---@return DVec3
function DMat4:transform_vector3(_self,rhs) end

---@param _self DMat4 
---@param rhs DMat4 
---@return DMat4
function DMat4:add_mat4(_self,rhs) end

---@param _self DMat4 
---@return DMat4
function DMat4:inverse(_self) end

---@param _self DMat4 
---@return boolean
function DMat4:is_nan(_self) end

---@param rotation DQuat 
---@return DMat4
function DMat4.from_quat(rotation) end

---@param eye DVec3 
---@param dir DVec3 
---@param up DVec3 
---@return DMat4
function DMat4.look_to_rh(eye,dir,up) end

---@param _self DMat4 
---@return boolean
function DMat4:is_finite(_self) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return DMat4
function DMat4.perspective_infinite_reverse_lh(fov_y_radians,aspect_ratio,z_near) end

---@param _self DMat4 
---@param index integer 
---@return DVec4
function DMat4:row(_self,index) end

---@param _self DMat4 
---@return number[][]
function DMat4:to_cols_array_2d(_self) end

---@param m DMat3 
---@return DMat4
function DMat4.from_mat3(m) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return DMat4
function DMat4.perspective_lh(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param _self DMat4 
---@param rhs DMat4 
---@return boolean
function DMat4:eq(_self,rhs) end

---@param _self DMat4 
---@param rhs number 
---@return DMat4
function DMat4:div(_self,rhs) end

---@param _self DMat4 
---@return DMat4
function DMat4:transpose(_self) end

---@param _self DMat4 
---@param rhs number 
---@return DMat4
function DMat4:mul_scalar(_self,rhs) end

---@param _self DMat4 
---@param index integer 
---@return DVec4
function DMat4:col(_self,index) end

---@param _self DMat4 
---@return number[]
function DMat4:to_cols_array(_self) end

---@param _self DMat4 
---@return DMat4
function DMat4:clone(_self) end

---@param translation DVec3 
---@return DMat4
function DMat4.from_translation(translation) end

---@param p1 DMat4 
---@param p2 number 
---@return DMat4
function DMat4:mul(p1,p2) end


---@class DQuat : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number
DQuat = {}

---@param v DVec4 
---@return DQuat
function DQuat.from_vec4(v) end

---@param from DVec2 
---@param to DVec2 
---@return DQuat
function DQuat.from_rotation_arc_2d(from,to) end

---@param _self DQuat 
---@return DQuat
function DQuat:normalize(_self) end

---@param p1 DQuat 
---@param p2 number 
---@return DQuat
function DQuat:mul(p1,p2) end

---@param _self DQuat 
---@param rhs DQuat 
---@return boolean
function DQuat:eq(_self,rhs) end

---@param _self DQuat 
---@return DQuat
function DQuat:conjugate(_self) end

---@param _self DQuat 
---@param rhs DQuat 
---@return DQuat
function DQuat:mul_quat(_self,rhs) end

---@param _self DQuat 
---@return DVec3
function DQuat:to_scaled_axis(_self) end

---@param mat DMat3 
---@return DQuat
function DQuat.from_mat3(mat) end

---@param _self DQuat 
---@param order EulerRot 
---@return [number, number, number]
function DQuat:to_euler(_self,order) end

---@param _self DQuat 
---@param rhs DQuat 
---@param max_angle number 
---@return DQuat
function DQuat:rotate_towards(_self,rhs,max_angle) end

---@param _self DQuat 
---@param _end DQuat 
---@param s number 
---@return DQuat
function DQuat:lerp(_self,_end,s) end

---@param _self DQuat 
---@return DQuat
function DQuat:inverse(_self) end

---@param _self DQuat 
---@return boolean
function DQuat:is_nan(_self) end

---@param _self DQuat 
---@return number[]
function DQuat:to_array(_self) end

---@param _self DQuat 
---@return DQuat
function DQuat:clone(_self) end

---@param x number 
---@param y number 
---@param z number 
---@param w number 
---@return DQuat
function DQuat.from_xyzw(x,y,z,w) end

---@param angle number 
---@return DQuat
function DQuat.from_rotation_z(angle) end

---@param _self DQuat 
---@param rhs DQuat 
---@return number
function DQuat:angle_between(_self,rhs) end

---@param _self DQuat 
---@param rhs DQuat 
---@return DQuat
function DQuat:sub(_self,rhs) end

---@param _self DQuat 
---@param rhs DQuat 
---@return number
function DQuat:dot(_self,rhs) end

---@param angle number 
---@return DQuat
function DQuat.from_rotation_y(angle) end

---@param _self DQuat 
---@param rhs DQuat 
---@return DQuat
function DQuat:add(_self,rhs) end

---@param _self DQuat 
---@param _end DQuat 
---@param s number 
---@return DQuat
function DQuat:slerp(_self,_end,s) end

---@param _self DQuat 
---@param rhs DQuat 
---@param max_abs_diff number 
---@return boolean
function DQuat:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DQuat 
---@return number
function DQuat:length(_self) end

---@param from DVec3 
---@param to DVec3 
---@return DQuat
function DQuat.from_rotation_arc(from,to) end

---@param mat DMat4 
---@return DQuat
function DQuat.from_mat4(mat) end

---@param _self DQuat 
---@return DVec3
function DQuat:xyz(_self) end

---@param _self DQuat 
---@param rhs number 
---@return DQuat
function DQuat:div(_self,rhs) end

---@param v DVec3 
---@return DQuat
function DQuat.from_scaled_axis(v) end

---@param _self DQuat 
---@return boolean
function DQuat:is_near_identity(_self) end

---@param _self DQuat 
---@return DQuat
function DQuat:neg(_self) end

---@param _self DQuat 
---@return Quat
function DQuat:as_quat(_self) end

---@param _self DQuat 
---@return boolean
function DQuat:is_finite(_self) end

---@param p1 DQuat 
---@param p2 DVec3 
---@return DVec3
function DQuat:mul(p1,p2) end

---@param _self DQuat 
---@param rhs DQuat 
---@return DQuat
function DQuat:mul(_self,rhs) end

---@param a number[] 
---@return DQuat
function DQuat.from_array(a) end

---@param _self DQuat 
---@return number
function DQuat:length_recip(_self) end

---@param axis DVec3 
---@param angle number 
---@return DQuat
function DQuat.from_axis_angle(axis,angle) end

---@param from DVec3 
---@param to DVec3 
---@return DQuat
function DQuat.from_rotation_arc_colinear(from,to) end

---@param angle number 
---@return DQuat
function DQuat.from_rotation_x(angle) end

---@param euler EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return DQuat
function DQuat.from_euler(euler,a,b,c) end

---@param a DAffine3 
---@return DQuat
function DQuat.from_affine3(a) end

---@param _self DQuat 
---@param rhs DVec3 
---@return DVec3
function DQuat:mul_vec3(_self,rhs) end

---@param _self DQuat 
---@return boolean
function DQuat:is_normalized(_self) end

---@param _self DQuat 
---@return number
function DQuat:length_squared(_self) end


---@class DVec2 : ReflectReference
---@field  x ? number
---@field  y ? number
DVec2 = {}

---@param _self DVec2 
---@param rhs DVec2 
---@param max_angle number 
---@return DVec2
function DVec2:rotate_towards(_self,rhs,max_angle) end

---@param _self DVec2 
---@return DVec2
function DVec2:normalize_or_zero(_self) end

---@param p1 DVec2 
---@param p2 DVec2 
---@return DVec2
function DVec2:div(p1,p2) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmplt(_self,rhs) end

---@param p1 DVec2 
---@param p2 number 
---@return DVec2
function DVec2:mul(p1,p2) end

---@param _self DVec2 
---@param y number 
---@return DVec2
function DVec2:with_y(_self,y) end

---@param mask BVec2 
---@param if_true DVec2 
---@param if_false DVec2 
---@return DVec2
function DVec2.select(mask,if_true,if_false) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmpeq(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:rotate(_self,rhs) end

---@param _self DVec2 
---@return integer
function DVec2:is_negative_bitmask(_self) end

---@param _self DVec2 
---@return boolean
function DVec2:is_normalized(_self) end

---@param _self DVec2 
---@return BVec2
function DVec2:is_nan_mask(_self) end

---@param _self DVec2 
---@return number
function DVec2:element_sum(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmpgt(_self,rhs) end

---@param _self DVec2 
---@return number
function DVec2:to_angle(_self) end

---@param _self DVec2 
---@param max number 
---@return DVec2
function DVec2:clamp_length_max(_self,max) end

---@param _self DVec2 
---@param min DVec2 
---@param max DVec2 
---@return DVec2
function DVec2:clamp(_self,min,max) end

---@param _self DVec2 
---@return number
function DVec2:length_recip(_self) end

---@param p1 DVec2 
---@param p2 DVec2 
---@return DVec2
function DVec2:add(p1,p2) end

---@param _self DVec2 
---@return U16Vec2
function DVec2:as_u16vec2(_self) end

---@param _self DVec2 
---@param normal DVec2 
---@return DVec2
function DVec2:reflect(_self,normal) end

---@param _self DVec2 
---@return UVec2
function DVec2:as_uvec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:midpoint(_self,rhs) end

---@param p1 DVec2 
---@param p2 DVec2 
---@return DVec2
function DVec2:mul(p1,p2) end

---@param _self DVec2 
---@param min number 
---@param max number 
---@return DVec2
function DVec2:clamp_length(_self,min,max) end

---@param x number 
---@param y number 
---@return DVec2
function DVec2.new(x,y) end

---@param _self DVec2 
---@return I16Vec2
function DVec2:as_i16vec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:perp_dot(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:distance_squared(_self,rhs) end

---@param _self DVec2 
---@return number[]
function DVec2:to_array(_self) end

---@param _self DVec2 
---@return number
function DVec2:max_element(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:copysign(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:sub(_self,rhs) end

---@param _self DVec2 
---@return Vec2
function DVec2:as_vec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:add(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:exp(_self) end

---@param _self DVec2 
---@return DVec2
function DVec2:floor(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:angle_to(_self,rhs) end

---@param _self DVec2 
---@param x number 
---@return DVec2
function DVec2:with_x(_self,x) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:reject_from_normalized(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:fract_gl(_self) end

---@param _self DVec2 
---@return number
function DVec2:length_squared(_self) end

---@param _self DVec2 
---@param a DVec2 
---@param b DVec2 
---@return DVec2
function DVec2:mul_add(_self,a,b) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:dot(_self,rhs) end

---@param angle number 
---@return DVec2
function DVec2.from_angle(angle) end

---@param _self DVec2 
---@return DVec2
function DVec2:recip(_self) end

---@param _self DVec2 
---@return U64Vec2
function DVec2:as_u64vec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@param d number 
---@return DVec2
function DVec2:move_towards(_self,rhs,d) end

---@param _self DVec2 
---@return DVec2
function DVec2:abs(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:rem_euclid(_self,rhs) end

---@param p1 DVec2 
---@param p2 number 
---@return DVec2
function DVec2:sub(p1,p2) end

---@param _self DVec2 
---@return IVec2
function DVec2:as_ivec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@param max_abs_diff number 
---@return boolean
function DVec2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:reject_from(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:signum(_self) end

---@param p1 DVec2 
---@param p2 number 
---@return DVec2
function DVec2:rem(p1,p2) end

---@param _self DVec2 
---@return number
function DVec2:length(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:max(_self,rhs) end

---@param _self DVec2 
---@param normal DVec2 
---@param eta number 
---@return DVec2
function DVec2:refract(_self,normal,eta) end

---@param p1 DVec2 
---@param p2 DVec2 
---@return DVec2
function DVec2:rem(p1,p2) end

---@param _self DVec2 
---@return DVec2
function DVec2:round(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:project_onto(_self,rhs) end

---@param _self DVec2 
---@return boolean
function DVec2:is_nan(_self) end

---@param _self DVec2 
---@param other DVec2 
---@return boolean
function DVec2:eq(_self,other) end

---@param _self DVec2 
---@return boolean
function DVec2:is_finite(_self) end

---@param _self DVec2 
---@param fallback DVec2 
---@return DVec2
function DVec2:normalize_or(_self,fallback) end

---@param _self DVec2 
---@return I8Vec2
function DVec2:as_i8vec2(_self) end

---@param p1 DVec2 
---@param p2 number 
---@return DVec2
function DVec2:div(p1,p2) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:distance(_self,rhs) end

---@param _self DVec2 
---@return number
function DVec2:min_element(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:mul(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:neg(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmpne(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:fract(_self) end

---@param _self DVec2 
---@return DVec2
function DVec2:trunc(_self) end

---@param _self DVec2 
---@param n number 
---@return DVec2
function DVec2:powf(_self,n) end

---@param a number[] 
---@return DVec2
function DVec2.from_array(a) end

---@param _self DVec2 
---@return number
function DVec2:element_product(_self) end

---@param _self DVec2 
---@param min number 
---@return DVec2
function DVec2:clamp_length_min(_self,min) end

---@param _self DVec2 
---@return U8Vec2
function DVec2:as_u8vec2(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return number
function DVec2:angle_between(_self,rhs) end

---@param _self DVec2 
---@return I64Vec2
function DVec2:as_i64vec2(_self) end

---@param p1 DVec2 
---@param p2 number 
---@return DVec2
function DVec2:add(p1,p2) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:div_euclid(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:ceil(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmple(_self,rhs) end

---@param v number 
---@return DVec2
function DVec2.splat(v) end

---@param _self DVec2 
---@param rhs DVec2 
---@param s number 
---@return DVec2
function DVec2:lerp(_self,rhs,s) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:rem(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:perp(_self) end

---@param _self DVec2 
---@param z number 
---@return DVec3
function DVec2:extend(_self,z) end

---@param _self DVec2 
---@return DVec2
function DVec2:normalize(_self) end

---@param p1 DVec2 
---@param p2 DVec2 
---@return DVec2
function DVec2:sub(p1,p2) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:min(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:div(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:project_onto_normalized(_self,rhs) end

---@param _self DVec2 
---@param rhs DVec2 
---@return BVec2
function DVec2:cmpge(_self,rhs) end

---@param _self DVec2 
---@return DVec2
function DVec2:clone(_self) end

---@param _self DVec2 
---@param rhs DVec2 
---@return DVec2
function DVec2:dot_into_vec(_self,rhs) end

---@param _self DVec2 
---@return BVec2
function DVec2:is_finite_mask(_self) end


---@class DVec3 : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
DVec3 = {}

---@param _self DVec3 
---@param rhs DVec3 
---@return number
function DVec3:distance(_self,rhs) end

---@param _self DVec3 
---@return number
function DVec3:length(_self) end

---@param p1 DVec3 
---@param p2 DVec3 
---@return DVec3
function DVec3:div(p1,p2) end

---@param _self DVec3 
---@return UVec3
function DVec3:as_uvec3(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@param max_abs_diff number 
---@return boolean
function DVec3:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:add(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmpne(_self,rhs) end

---@param _self DVec3 
---@return number
function DVec3:element_sum(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:copysign(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:sub(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:clone(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:normalize(_self) end

---@param x number 
---@param y number 
---@param z number 
---@return DVec3
function DVec3.new(x,y,z) end

---@param p1 DVec3 
---@param p2 DVec3 
---@return DVec3
function DVec3:mul(p1,p2) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:rem_euclid(_self,rhs) end

---@param p1 DVec3 
---@param p2 DVec3 
---@return DVec3
function DVec3:rem(p1,p2) end

---@param _self DVec3 
---@param fallback DVec3 
---@return DVec3
function DVec3:normalize_or(_self,fallback) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:midpoint(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:normalize_or_zero(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmpeq(_self,rhs) end

---@param _self DVec3 
---@return number
function DVec3:length_recip(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:signum(_self) end

---@param _self DVec3 
---@param x number 
---@return DVec3
function DVec3:with_x(_self,x) end

---@param _self DVec3 
---@param rhs DVec3 
---@param d number 
---@return DVec3
function DVec3:move_towards(_self,rhs,d) end

---@param _self DVec3 
---@return I16Vec3
function DVec3:as_i16vec3(_self) end

---@param _self DVec3 
---@return number
function DVec3:length_squared(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:div_euclid(_self,rhs) end

---@param _self DVec3 
---@param w number 
---@return DVec4
function DVec3:extend(_self,w) end

---@param p1 DVec3 
---@param p2 number 
---@return DVec3
function DVec3:rem(p1,p2) end

---@param _self DVec3 
---@return Vec3A
function DVec3:as_vec3a(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmpgt(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:floor(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:div(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@param s number 
---@return DVec3
function DVec3:lerp(_self,rhs,s) end

---@param _self DVec3 
---@return I8Vec3
function DVec3:as_i8vec3(_self) end

---@param _self DVec3 
---@return boolean
function DVec3:is_finite(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:ceil(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:dot_into_vec(_self,rhs) end

---@param _self DVec3 
---@return boolean
function DVec3:is_nan(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmple(_self,rhs) end

---@param _self DVec3 
---@return number
function DVec3:min_element(_self) end

---@param mask BVec3 
---@param if_true DVec3 
---@param if_false DVec3 
---@return DVec3
function DVec3.select(mask,if_true,if_false) end

---@param _self DVec3 
---@param min number 
---@param max number 
---@return DVec3
function DVec3:clamp_length(_self,min,max) end

---@param p1 DVec3 
---@param p2 number 
---@return DVec3
function DVec3:div(p1,p2) end

---@param _self DVec3 
---@param n number 
---@return DVec3
function DVec3:powf(_self,n) end

---@param _self DVec3 
---@param rhs DVec3 
---@return number
function DVec3:distance_squared(_self,rhs) end

---@param _self DVec3 
---@return number[]
function DVec3:to_array(_self) end

---@param p1 DVec3 
---@param p2 number 
---@return DVec3
function DVec3:sub(p1,p2) end

---@param _self DVec3 
---@return U64Vec3
function DVec3:as_u64vec3(_self) end

---@param _self DVec3 
---@param z number 
---@return DVec3
function DVec3:with_z(_self,z) end

---@param p1 DVec3 
---@param p2 number 
---@return DVec3
function DVec3:add(p1,p2) end

---@param _self DVec3 
---@return boolean
function DVec3:is_normalized(_self) end

---@param _self DVec3 
---@param y number 
---@return DVec3
function DVec3:with_y(_self,y) end

---@param _self DVec3 
---@return DVec3
function DVec3:exp(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:project_onto_normalized(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:fract_gl(_self) end

---@param _self DVec3 
---@return BVec3
function DVec3:is_finite_mask(_self) end

---@param p1 DVec3 
---@param p2 DVec3 
---@return DVec3
function DVec3:sub(p1,p2) end

---@param _self DVec3 
---@return DVec3
function DVec3:fract(_self) end

---@param _self DVec3 
---@param a DVec3 
---@param b DVec3 
---@return DVec3
function DVec3:mul_add(_self,a,b) end

---@param _self DVec3 
---@param normal DVec3 
---@return DVec3
function DVec3:reflect(_self,normal) end

---@param _self DVec3 
---@param min number 
---@return DVec3
function DVec3:clamp_length_min(_self,min) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:cross(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:max(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:reject_from_normalized(_self,rhs) end

---@param _self DVec3 
---@param other DVec3 
---@return boolean
function DVec3:eq(_self,other) end

---@param _self DVec3 
---@return number
function DVec3:element_product(_self) end

---@param p1 DVec3 
---@param p2 DVec3 
---@return DVec3
function DVec3:add(p1,p2) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:min(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:any_orthogonal_vector(_self) end

---@param _self DVec3 
---@param normal DVec3 
---@param eta number 
---@return DVec3
function DVec3:refract(_self,normal,eta) end

---@param _self DVec3 
---@return I64Vec3
function DVec3:as_i64vec3(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:mul(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:recip(_self) end

---@param a number[] 
---@return DVec3
function DVec3.from_array(a) end

---@param v number 
---@return DVec3
function DVec3.splat(v) end

---@param _self DVec3 
---@return IVec3
function DVec3:as_ivec3(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:round(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:trunc(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:rem(_self,rhs) end

---@param _self DVec3 
---@return BVec3
function DVec3:is_nan_mask(_self) end

---@param _self DVec3 
---@return U16Vec3
function DVec3:as_u16vec3(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return number
function DVec3:angle_between(_self,rhs) end

---@param _self DVec3 
---@return Vec3
function DVec3:as_vec3(_self) end

---@param p1 DVec3 
---@param p2 number 
---@return DVec3
function DVec3:mul(p1,p2) end

---@param _self DVec3 
---@param min DVec3 
---@param max DVec3 
---@return DVec3
function DVec3:clamp(_self,min,max) end

---@param _self DVec3 
---@return number
function DVec3:max_element(_self) end

---@param _self DVec3 
---@return DVec2
function DVec3:truncate(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:neg(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmpge(_self,rhs) end

---@param _self DVec3 
---@return integer
function DVec3:is_negative_bitmask(_self) end

---@param _self DVec3 
---@return DVec3
function DVec3:any_orthonormal_vector(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:reject_from(_self,rhs) end

---@param _self DVec3 
---@param rhs DVec3 
---@return BVec3
function DVec3:cmplt(_self,rhs) end

---@param _self DVec3 
---@param max number 
---@return DVec3
function DVec3:clamp_length_max(_self,max) end

---@param _self DVec3 
---@param rhs DVec3 
---@return DVec3
function DVec3:project_onto(_self,rhs) end

---@param _self DVec3 
---@return U8Vec3
function DVec3:as_u8vec3(_self) end

---@param _self DVec3 
---@param rhs DVec3 
---@return number
function DVec3:dot(_self,rhs) end

---@param _self DVec3 
---@return DVec3
function DVec3:abs(_self) end


---@class DVec4 : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number
DVec4 = {}

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:div_euclid(_self,rhs) end

---@param _self DVec4 
---@param normal DVec4 
---@param eta number 
---@return DVec4
function DVec4:refract(_self,normal,eta) end

---@param _self DVec4 
---@param n number 
---@return DVec4
function DVec4:powf(_self,n) end

---@param p1 DVec4 
---@param p2 number 
---@return DVec4
function DVec4:sub(p1,p2) end

---@param _self DVec4 
---@return I16Vec4
function DVec4:as_i16vec4(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:ceil(_self) end

---@param _self DVec4 
---@return number
function DVec4:min_element(_self) end

---@param p1 DVec4 
---@param p2 number 
---@return DVec4
function DVec4:rem(p1,p2) end

---@param _self DVec4 
---@param min DVec4 
---@param max DVec4 
---@return DVec4
function DVec4:clamp(_self,min,max) end

---@param _self DVec4 
---@param rhs DVec4 
---@param d number 
---@return DVec4
function DVec4:move_towards(_self,rhs,d) end

---@param _self DVec4 
---@return boolean
function DVec4:is_nan(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:max(_self,rhs) end

---@param _self DVec4 
---@return DVec4
function DVec4:normalize_or_zero(_self) end

---@param p1 DVec4 
---@param p2 number 
---@return DVec4
function DVec4:mul(p1,p2) end

---@param p1 DVec4 
---@param p2 DVec4 
---@return DVec4
function DVec4:div(p1,p2) end

---@param _self DVec4 
---@return boolean
function DVec4:is_normalized(_self) end

---@param _self DVec4 
---@param a DVec4 
---@param b DVec4 
---@return DVec4
function DVec4:mul_add(_self,a,b) end

---@param _self DVec4 
---@return UVec4
function DVec4:as_uvec4(_self) end

---@param _self DVec4 
---@param min number 
---@return DVec4
function DVec4:clamp_length_min(_self,min) end

---@param _self DVec4 
---@return DVec4
function DVec4:floor(_self) end

---@param v number 
---@return DVec4
function DVec4.splat(v) end

---@param _self DVec4 
---@return DVec4
function DVec4:fract(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:exp(_self) end

---@param _self DVec4 
---@param normal DVec4 
---@return DVec4
function DVec4:reflect(_self,normal) end

---@param _self DVec4 
---@return DVec4
function DVec4:recip(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return number
function DVec4:distance(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@param s number 
---@return DVec4
function DVec4:lerp(_self,rhs,s) end

---@param _self DVec4 
---@return DVec4
function DVec4:neg(_self) end

---@param _self DVec4 
---@param w number 
---@return DVec4
function DVec4:with_w(_self,w) end

---@param p1 DVec4 
---@param p2 DVec4 
---@return DVec4
function DVec4:rem(p1,p2) end

---@param _self DVec4 
---@return number
function DVec4:element_product(_self) end

---@param _self DVec4 
---@param z number 
---@return DVec4
function DVec4:with_z(_self,z) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmple(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:dot_into_vec(_self,rhs) end

---@param _self DVec4 
---@return integer
function DVec4:is_negative_bitmask(_self) end

---@param _self DVec4 
---@return number
function DVec4:element_sum(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:clone(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:reject_from_normalized(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:midpoint(_self,rhs) end

---@param _self DVec4 
---@param min number 
---@param max number 
---@return DVec4
function DVec4:clamp_length(_self,min,max) end

---@param a number[] 
---@return DVec4
function DVec4.from_array(a) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmpne(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmpge(_self,rhs) end

---@param _self DVec4 
---@return DVec4
function DVec4:round(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:reject_from(_self,rhs) end

---@param p1 DVec4 
---@param p2 number 
---@return DVec4
function DVec4:div(p1,p2) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:project_onto_normalized(_self,rhs) end

---@param _self DVec4 
---@return BVec4
function DVec4:is_nan_mask(_self) end

---@param _self DVec4 
---@return U64Vec4
function DVec4:as_u64vec4(_self) end

---@param _self DVec4 
---@return I64Vec4
function DVec4:as_i64vec4(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:rem_euclid(_self,rhs) end

---@param _self DVec4 
---@param x number 
---@return DVec4
function DVec4:with_x(_self,x) end

---@param _self DVec4 
---@return DVec4
function DVec4:trunc(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:fract_gl(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:signum(_self) end

---@param _self DVec4 
---@param fallback DVec4 
---@return DVec4
function DVec4:normalize_or(_self,fallback) end

---@param _self DVec4 
---@return IVec4
function DVec4:as_ivec4(_self) end

---@param p1 DVec4 
---@param p2 DVec4 
---@return DVec4
function DVec4:mul(p1,p2) end

---@param _self DVec4 
---@param max number 
---@return DVec4
function DVec4:clamp_length_max(_self,max) end

---@param _self DVec4 
---@return boolean
function DVec4:is_finite(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:sub(_self,rhs) end

---@param _self DVec4 
---@return number
function DVec4:length_squared(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:add(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:mul(_self,rhs) end

---@param p1 DVec4 
---@param p2 DVec4 
---@return DVec4
function DVec4:add(p1,p2) end

---@param _self DVec4 
---@return U8Vec4
function DVec4:as_u8vec4(_self) end

---@param x number 
---@param y number 
---@param z number 
---@param w number 
---@return DVec4
function DVec4.new(x,y,z,w) end

---@param _self DVec4 
---@param other DVec4 
---@return boolean
function DVec4:eq(_self,other) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:project_onto(_self,rhs) end

---@param p1 DVec4 
---@param p2 number 
---@return DVec4
function DVec4:add(p1,p2) end

---@param _self DVec4 
---@return number[]
function DVec4:to_array(_self) end

---@param p1 DVec4 
---@param p2 DVec4 
---@return DVec4
function DVec4:sub(p1,p2) end

---@param mask BVec4 
---@param if_true DVec4 
---@param if_false DVec4 
---@return DVec4
function DVec4.select(mask,if_true,if_false) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:copysign(_self,rhs) end

---@param _self DVec4 
---@return I8Vec4
function DVec4:as_i8vec4(_self) end

---@param _self DVec4 
---@param y number 
---@return DVec4
function DVec4:with_y(_self,y) end

---@param _self DVec4 
---@return U16Vec4
function DVec4:as_u16vec4(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmpeq(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return number
function DVec4:distance_squared(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:rem(_self,rhs) end

---@param _self DVec4 
---@return number
function DVec4:length_recip(_self) end

---@param _self DVec4 
---@return DVec4
function DVec4:normalize(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:div(_self,rhs) end

---@param _self DVec4 
---@return DVec4
function DVec4:abs(_self) end

---@param _self DVec4 
---@return DVec3
function DVec4:truncate(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@param max_abs_diff number 
---@return boolean
function DVec4:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmpgt(_self,rhs) end

---@param _self DVec4 
---@return BVec4
function DVec4:is_finite_mask(_self) end

---@param _self DVec4 
---@return number
function DVec4:length(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return BVec4
function DVec4:cmplt(_self,rhs) end

---@param _self DVec4 
---@return Vec4
function DVec4:as_vec4(_self) end

---@param _self DVec4 
---@param rhs DVec4 
---@return number
function DVec4:dot(_self,rhs) end

---@param _self DVec4 
---@param rhs DVec4 
---@return DVec4
function DVec4:min(_self,rhs) end

---@param _self DVec4 
---@return number
function DVec4:max_element(_self) end


---@class EulerRot : ReflectReference
EulerRot = {}

---@param _self EulerRot 
---@param other EulerRot 
---@return boolean
function EulerRot:eq(_self,other) end

---@param _self EulerRot 
---@return EulerRot
function EulerRot:clone(_self) end

---@param _self EulerRot 
---@return nil
function EulerRot:assert_receiver_is_total_eq(_self) end


---@class I16Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
I16Vec2 = {}

---@param _self I16Vec2 
---@param rhs U16Vec2 
---@return I16Vec2
function I16Vec2:saturating_sub_unsigned(_self,rhs) end

---@param x integer 
---@param y integer 
---@return I16Vec2
function I16Vec2.new(x,y) end

---@param p1 I16Vec2 
---@param p2 I16Vec2 
---@return I16Vec2
function I16Vec2:mul(p1,p2) end

---@param _self I16Vec2 
---@return UVec2
function I16Vec2:as_uvec2(_self) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:length_squared(_self) end

---@param p1 I16Vec2 
---@param p2 I16Vec2 
---@return I16Vec2
function I16Vec2:sub(p1,p2) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:sub(_self,rhs) end

---@param _self I16Vec2 
---@return I8Vec2
function I16Vec2:as_i8vec2(_self) end

---@param p1 I16Vec2 
---@param p2 I16Vec2 
---@return I16Vec2
function I16Vec2:add(p1,p2) end

---@param _self I16Vec2 
---@param rhs U16Vec2 
---@return I16Vec2
function I16Vec2:saturating_add_unsigned(_self,rhs) end

---@param _self I16Vec2 
---@return I64Vec2
function I16Vec2:as_i64vec2(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmplt(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return integer
function I16Vec2:distance_squared(_self,rhs) end

---@param _self I16Vec2 
---@return I16Vec2
function I16Vec2:neg(_self) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:min_element(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:saturating_add(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:add(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_div(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmpge(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:mul(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmple(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:saturating_div(_self,rhs) end

---@param _self I16Vec2 
---@param rhs U16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_add_unsigned(_self,rhs) end

---@param p1 I16Vec2 
---@param p2 integer 
---@return I16Vec2
function I16Vec2:add(p1,p2) end

---@param _self I16Vec2 
---@param other I16Vec2 
---@return boolean
function I16Vec2:eq(_self,other) end

---@param _self I16Vec2 
---@return DVec2
function I16Vec2:as_dvec2(_self) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:max_element(_self) end

---@param _self I16Vec2 
---@return I16Vec2
function I16Vec2:clone(_self) end

---@param _self I16Vec2 
---@param min I16Vec2 
---@param max I16Vec2 
---@return I16Vec2
function I16Vec2:clamp(_self,min,max) end

---@param _self I16Vec2 
---@return integer[]
function I16Vec2:to_array(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:saturating_mul(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:min(_self,rhs) end

---@param _self I16Vec2 
---@param z integer 
---@return I16Vec3
function I16Vec2:extend(_self,z) end

---@param _self I16Vec2 
---@return I16Vec2
function I16Vec2:signum(_self) end

---@param _self I16Vec2 
---@return Vec2
function I16Vec2:as_vec2(_self) end

---@param _self I16Vec2 
---@return U64Vec2
function I16Vec2:as_u64vec2(_self) end

---@param v integer 
---@return I16Vec2
function I16Vec2.splat(v) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:is_negative_bitmask(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:rem_euclid(_self,rhs) end

---@param _self I16Vec2 
---@return IVec2
function I16Vec2:as_ivec2(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:div(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:rotate(_self,rhs) end

---@param p1 I16Vec2 
---@param p2 integer 
---@return I16Vec2
function I16Vec2:sub(p1,p2) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return integer
function I16Vec2:perp_dot(_self,rhs) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:element_sum(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:max(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:rem(_self,rhs) end

---@param _self I16Vec2 
---@return I16Vec2
function I16Vec2:abs(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:saturating_sub(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmpgt(_self,rhs) end

---@param p1 I16Vec2 
---@param p2 I16Vec2 
---@return I16Vec2
function I16Vec2:div(p1,p2) end

---@param p1 I16Vec2 
---@param p2 integer 
---@return I16Vec2
function I16Vec2:rem(p1,p2) end

---@param p1 I16Vec2 
---@param p2 integer 
---@return I16Vec2
function I16Vec2:mul(p1,p2) end

---@param _self I16Vec2 
---@return U16Vec2
function I16Vec2:as_u16vec2(_self) end

---@param _self I16Vec2 
---@param y integer 
---@return I16Vec2
function I16Vec2:with_y(_self,y) end

---@param mask BVec2 
---@param if_true I16Vec2 
---@param if_false I16Vec2 
---@return I16Vec2
function I16Vec2.select(mask,if_true,if_false) end

---@param _self I16Vec2 
---@return integer
function I16Vec2:element_product(_self) end

---@param _self I16Vec2 
---@return nil
function I16Vec2:assert_receiver_is_total_eq(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_mul(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmpeq(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:div_euclid(_self,rhs) end

---@param _self I16Vec2 
---@param rhs U16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_sub_unsigned(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return integer
function I16Vec2:dot(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_add(_self,rhs) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:wrapping_sub(_self,rhs) end

---@param _self I16Vec2 
---@return U8Vec2
function I16Vec2:as_u8vec2(_self) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return I16Vec2
function I16Vec2:dot_into_vec(_self,rhs) end

---@param p1 I16Vec2 
---@param p2 I16Vec2 
---@return I16Vec2
function I16Vec2:rem(p1,p2) end

---@param _self I16Vec2 
---@param rhs I16Vec2 
---@return BVec2
function I16Vec2:cmpne(_self,rhs) end

---@param _self I16Vec2 
---@param x integer 
---@return I16Vec2
function I16Vec2:with_x(_self,x) end

---@param p1 I16Vec2 
---@param p2 integer 
---@return I16Vec2
function I16Vec2:div(p1,p2) end

---@param _self I16Vec2 
---@return I16Vec2
function I16Vec2:perp(_self) end

---@param a integer[] 
---@return I16Vec2
function I16Vec2.from_array(a) end


---@class I16Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
I16Vec3 = {}

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmplt(_self,rhs) end

---@param _self I16Vec3 
---@return integer[]
function I16Vec3:to_array(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmple(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:mul(_self,rhs) end

---@param p1 I16Vec3 
---@param p2 I16Vec3 
---@return I16Vec3
function I16Vec3:add(p1,p2) end

---@param _self I16Vec3 
---@param rhs U16Vec3 
---@return I16Vec3
function I16Vec3:saturating_sub_unsigned(_self,rhs) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:element_sum(_self) end

---@param p1 I16Vec3 
---@param p2 integer 
---@return I16Vec3
function I16Vec3:add(p1,p2) end

---@param p1 I16Vec3 
---@param p2 integer 
---@return I16Vec3
function I16Vec3:rem(p1,p2) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:element_product(_self) end

---@param _self I16Vec3 
---@param rhs U16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_sub_unsigned(_self,rhs) end

---@param _self I16Vec3 
---@return I16Vec2
function I16Vec3:truncate(_self) end

---@param _self I16Vec3 
---@return I16Vec3
function I16Vec3:neg(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmpne(_self,rhs) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:length_squared(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmpge(_self,rhs) end

---@param p1 I16Vec3 
---@param p2 integer 
---@return I16Vec3
function I16Vec3:div(p1,p2) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:add(_self,rhs) end

---@param _self I16Vec3 
---@return Vec3
function I16Vec3:as_vec3(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:saturating_mul(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:saturating_add(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:cross(_self,rhs) end

---@param _self I16Vec3 
---@param min I16Vec3 
---@param max I16Vec3 
---@return I16Vec3
function I16Vec3:clamp(_self,min,max) end

---@param p1 I16Vec3 
---@param p2 integer 
---@return I16Vec3
function I16Vec3:mul(p1,p2) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_div(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmpeq(_self,rhs) end

---@param p1 I16Vec3 
---@param p2 I16Vec3 
---@return I16Vec3
function I16Vec3:rem(p1,p2) end

---@param _self I16Vec3 
---@return Vec3A
function I16Vec3:as_vec3a(_self) end

---@param _self I16Vec3 
---@return I16Vec3
function I16Vec3:clone(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:saturating_sub(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:min(_self,rhs) end

---@param _self I16Vec3 
---@param w integer 
---@return I16Vec4
function I16Vec3:extend(_self,w) end

---@param _self I16Vec3 
---@return I64Vec3
function I16Vec3:as_i64vec3(_self) end

---@param _self I16Vec3 
---@param y integer 
---@return I16Vec3
function I16Vec3:with_y(_self,y) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:max(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return BVec3
function I16Vec3:cmpgt(_self,rhs) end

---@param _self I16Vec3 
---@return U16Vec3
function I16Vec3:as_u16vec3(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_mul(_self,rhs) end

---@param p1 I16Vec3 
---@param p2 I16Vec3 
---@return I16Vec3
function I16Vec3:mul(p1,p2) end

---@param _self I16Vec3 
---@return I16Vec3
function I16Vec3:signum(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:sub(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return integer
function I16Vec3:dot(_self,rhs) end

---@param mask BVec3 
---@param if_true I16Vec3 
---@param if_false I16Vec3 
---@return I16Vec3
function I16Vec3.select(mask,if_true,if_false) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_sub(_self,rhs) end

---@param _self I16Vec3 
---@return I8Vec3
function I16Vec3:as_i8vec3(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:div(_self,rhs) end

---@param _self I16Vec3 
---@param z integer 
---@return I16Vec3
function I16Vec3:with_z(_self,z) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:max_element(_self) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:min_element(_self) end

---@param a integer[] 
---@return I16Vec3
function I16Vec3.from_array(a) end

---@param _self I16Vec3 
---@return nil
function I16Vec3:assert_receiver_is_total_eq(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return integer
function I16Vec3:distance_squared(_self,rhs) end

---@param _self I16Vec3 
---@return DVec3
function I16Vec3:as_dvec3(_self) end

---@param _self I16Vec3 
---@param rhs U16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_add_unsigned(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:div_euclid(_self,rhs) end

---@param _self I16Vec3 
---@return integer
function I16Vec3:is_negative_bitmask(_self) end

---@param _self I16Vec3 
---@return UVec3
function I16Vec3:as_uvec3(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:rem_euclid(_self,rhs) end

---@param _self I16Vec3 
---@return U64Vec3
function I16Vec3:as_u64vec3(_self) end

---@param _self I16Vec3 
---@param other I16Vec3 
---@return boolean
function I16Vec3:eq(_self,other) end

---@param _self I16Vec3 
---@return U8Vec3
function I16Vec3:as_u8vec3(_self) end

---@param _self I16Vec3 
---@return IVec3
function I16Vec3:as_ivec3(_self) end

---@param p1 I16Vec3 
---@param p2 I16Vec3 
---@return I16Vec3
function I16Vec3:sub(p1,p2) end

---@param _self I16Vec3 
---@param x integer 
---@return I16Vec3
function I16Vec3:with_x(_self,x) end

---@param p1 I16Vec3 
---@param p2 integer 
---@return I16Vec3
function I16Vec3:sub(p1,p2) end

---@param v integer 
---@return I16Vec3
function I16Vec3.splat(v) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:dot_into_vec(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:wrapping_add(_self,rhs) end

---@param _self I16Vec3 
---@return I16Vec3
function I16Vec3:abs(_self) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:rem(_self,rhs) end

---@param _self I16Vec3 
---@param rhs I16Vec3 
---@return I16Vec3
function I16Vec3:saturating_div(_self,rhs) end

---@param _self I16Vec3 
---@param rhs U16Vec3 
---@return I16Vec3
function I16Vec3:saturating_add_unsigned(_self,rhs) end

---@param p1 I16Vec3 
---@param p2 I16Vec3 
---@return I16Vec3
function I16Vec3:div(p1,p2) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return I16Vec3
function I16Vec3.new(x,y,z) end


---@class I16Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
I16Vec4 = {}

---@param _self I16Vec4 
---@param rhs U16Vec4 
---@return I16Vec4
function I16Vec4:saturating_add_unsigned(_self,rhs) end

---@param _self I16Vec4 
---@return nil
function I16Vec4:assert_receiver_is_total_eq(_self) end

---@param v integer 
---@return I16Vec4
function I16Vec4.splat(v) end

---@param _self I16Vec4 
---@param w integer 
---@return I16Vec4
function I16Vec4:with_w(_self,w) end

---@param a integer[] 
---@return I16Vec4
function I16Vec4.from_array(a) end

---@param p1 I16Vec4 
---@param p2 I16Vec4 
---@return I16Vec4
function I16Vec4:add(p1,p2) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:saturating_sub(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return I16Vec4
function I16Vec4.new(x,y,z,w) end

---@param _self I16Vec4 
---@return I8Vec4
function I16Vec4:as_i8vec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:rem(_self,rhs) end

---@param _self I16Vec4 
---@return I16Vec4
function I16Vec4:neg(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmpgt(_self,rhs) end

---@param _self I16Vec4 
---@return Vec4
function I16Vec4:as_vec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_mul(_self,rhs) end

---@param _self I16Vec4 
---@param rhs U16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_add_unsigned(_self,rhs) end

---@param _self I16Vec4 
---@param min I16Vec4 
---@param max I16Vec4 
---@return I16Vec4
function I16Vec4:clamp(_self,min,max) end

---@param p1 I16Vec4 
---@param p2 I16Vec4 
---@return I16Vec4
function I16Vec4:mul(p1,p2) end

---@param _self I16Vec4 
---@return integer[]
function I16Vec4:to_array(_self) end

---@param _self I16Vec4 
---@param other I16Vec4 
---@return boolean
function I16Vec4:eq(_self,other) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:length_squared(_self) end

---@param _self I16Vec4 
---@return I16Vec3
function I16Vec4:truncate(_self) end

---@param _self I16Vec4 
---@param y integer 
---@return I16Vec4
function I16Vec4:with_y(_self,y) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmple(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmpne(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:saturating_add(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:div_euclid(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmpge(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:saturating_mul(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_add(_self,rhs) end

---@param mask BVec4 
---@param if_true I16Vec4 
---@param if_false I16Vec4 
---@return I16Vec4
function I16Vec4.select(mask,if_true,if_false) end

---@param _self I16Vec4 
---@return DVec4
function I16Vec4:as_dvec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_sub(_self,rhs) end

---@param _self I16Vec4 
---@return U64Vec4
function I16Vec4:as_u64vec4(_self) end

---@param _self I16Vec4 
---@param rhs U16Vec4 
---@return I16Vec4
function I16Vec4:saturating_sub_unsigned(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:max(_self,rhs) end

---@param _self I16Vec4 
---@return IVec4
function I16Vec4:as_ivec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmpeq(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:div(_self,rhs) end

---@param p1 I16Vec4 
---@param p2 integer 
---@return I16Vec4
function I16Vec4:rem(p1,p2) end

---@param _self I16Vec4 
---@param x integer 
---@return I16Vec4
function I16Vec4:with_x(_self,x) end

---@param _self I16Vec4 
---@return I64Vec4
function I16Vec4:as_i64vec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return BVec4
function I16Vec4:cmplt(_self,rhs) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:is_negative_bitmask(_self) end

---@param _self I16Vec4 
---@return I16Vec4
function I16Vec4:abs(_self) end

---@param _self I16Vec4 
---@return I16Vec4
function I16Vec4:signum(_self) end

---@param p1 I16Vec4 
---@param p2 integer 
---@return I16Vec4
function I16Vec4:mul(p1,p2) end

---@param _self I16Vec4 
---@return U16Vec4
function I16Vec4:as_u16vec4(_self) end

---@param _self I16Vec4 
---@return U8Vec4
function I16Vec4:as_u8vec4(_self) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:min_element(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:sub(_self,rhs) end

---@param p1 I16Vec4 
---@param p2 integer 
---@return I16Vec4
function I16Vec4:sub(p1,p2) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_div(_self,rhs) end

---@param p1 I16Vec4 
---@param p2 integer 
---@return I16Vec4
function I16Vec4:div(p1,p2) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:min(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:mul(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:add(_self,rhs) end

---@param p1 I16Vec4 
---@param p2 integer 
---@return I16Vec4
function I16Vec4:add(p1,p2) end

---@param _self I16Vec4 
---@param rhs U16Vec4 
---@return I16Vec4
function I16Vec4:wrapping_sub_unsigned(_self,rhs) end

---@param _self I16Vec4 
---@param z integer 
---@return I16Vec4
function I16Vec4:with_z(_self,z) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:rem_euclid(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return integer
function I16Vec4:dot(_self,rhs) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:element_product(_self) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:max_element(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return integer
function I16Vec4:distance_squared(_self,rhs) end

---@param p1 I16Vec4 
---@param p2 I16Vec4 
---@return I16Vec4
function I16Vec4:sub(p1,p2) end

---@param p1 I16Vec4 
---@param p2 I16Vec4 
---@return I16Vec4
function I16Vec4:div(p1,p2) end

---@param _self I16Vec4 
---@return UVec4
function I16Vec4:as_uvec4(_self) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:saturating_div(_self,rhs) end

---@param _self I16Vec4 
---@param rhs I16Vec4 
---@return I16Vec4
function I16Vec4:dot_into_vec(_self,rhs) end

---@param _self I16Vec4 
---@return I16Vec4
function I16Vec4:clone(_self) end

---@param p1 I16Vec4 
---@param p2 I16Vec4 
---@return I16Vec4
function I16Vec4:rem(p1,p2) end

---@param _self I16Vec4 
---@return integer
function I16Vec4:element_sum(_self) end


---@class I64Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
I64Vec2 = {}

---@param _self I64Vec2 
---@return U8Vec2
function I64Vec2:as_u8vec2(_self) end

---@param _self I64Vec2 
---@param rhs U64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_add_unsigned(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 I64Vec2 
---@return I64Vec2
function I64Vec2:mul(p1,p2) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:div(_self,rhs) end

---@param _self I64Vec2 
---@return I64Vec2
function I64Vec2:abs(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmpeq(_self,rhs) end

---@param _self I64Vec2 
---@return I64Vec2
function I64Vec2:signum(_self) end

---@param _self I64Vec2 
---@return I64Vec2
function I64Vec2:neg(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmpgt(_self,rhs) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:element_product(_self) end

---@param _self I64Vec2 
---@param z integer 
---@return I64Vec3
function I64Vec2:extend(_self,z) end

---@param p1 I64Vec2 
---@param p2 I64Vec2 
---@return I64Vec2
function I64Vec2:rem(p1,p2) end

---@param _self I64Vec2 
---@return nil
function I64Vec2:assert_receiver_is_total_eq(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_add(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_sub(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return integer
function I64Vec2:distance_squared(_self,rhs) end

---@param _self I64Vec2 
---@param other I64Vec2 
---@return boolean
function I64Vec2:eq(_self,other) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmpne(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 I64Vec2 
---@return I64Vec2
function I64Vec2:add(p1,p2) end

---@param p1 I64Vec2 
---@param p2 integer 
---@return I64Vec2
function I64Vec2:mul(p1,p2) end

---@param _self I64Vec2 
---@param rhs U64Vec2 
---@return I64Vec2
function I64Vec2:saturating_add_unsigned(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 integer 
---@return I64Vec2
function I64Vec2:rem(p1,p2) end

---@param p1 I64Vec2 
---@param p2 I64Vec2 
---@return I64Vec2
function I64Vec2:div(p1,p2) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:saturating_mul(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:saturating_sub(_self,rhs) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:element_sum(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:min(_self,rhs) end

---@param _self I64Vec2 
---@return Vec2
function I64Vec2:as_vec2(_self) end

---@param p1 I64Vec2 
---@param p2 integer 
---@return I64Vec2
function I64Vec2:add(p1,p2) end

---@param _self I64Vec2 
---@return IVec2
function I64Vec2:as_ivec2(_self) end

---@param _self I64Vec2 
---@return integer[]
function I64Vec2:to_array(_self) end

---@param x integer 
---@param y integer 
---@return I64Vec2
function I64Vec2.new(x,y) end

---@param _self I64Vec2 
---@param y integer 
---@return I64Vec2
function I64Vec2:with_y(_self,y) end

---@param _self I64Vec2 
---@return DVec2
function I64Vec2:as_dvec2(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:dot_into_vec(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 integer 
---@return I64Vec2
function I64Vec2:div(p1,p2) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:saturating_add(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:rem_euclid(_self,rhs) end

---@param _self I64Vec2 
---@param rhs U64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_sub_unsigned(_self,rhs) end

---@param _self I64Vec2 
---@param min I64Vec2 
---@param max I64Vec2 
---@return I64Vec2
function I64Vec2:clamp(_self,min,max) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:rotate(_self,rhs) end

---@param _self I64Vec2 
---@return UVec2
function I64Vec2:as_uvec2(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmpge(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmplt(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return integer
function I64Vec2:dot(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 integer 
---@return I64Vec2
function I64Vec2:sub(p1,p2) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:add(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return BVec2
function I64Vec2:cmple(_self,rhs) end

---@param _self I64Vec2 
---@return U16Vec2
function I64Vec2:as_u16vec2(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:max(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_div(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:saturating_div(_self,rhs) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:min_element(_self) end

---@param _self I64Vec2 
---@param x integer 
---@return I64Vec2
function I64Vec2:with_x(_self,x) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:rem(_self,rhs) end

---@param mask BVec2 
---@param if_true I64Vec2 
---@param if_false I64Vec2 
---@return I64Vec2
function I64Vec2.select(mask,if_true,if_false) end

---@param _self I64Vec2 
---@return I16Vec2
function I64Vec2:as_i16vec2(_self) end

---@param v integer 
---@return I64Vec2
function I64Vec2.splat(v) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:length_squared(_self) end

---@param _self I64Vec2 
---@return I8Vec2
function I64Vec2:as_i8vec2(_self) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:max_element(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:mul(_self,rhs) end

---@param _self I64Vec2 
---@return U64Vec2
function I64Vec2:as_u64vec2(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:wrapping_mul(_self,rhs) end

---@param p1 I64Vec2 
---@param p2 I64Vec2 
---@return I64Vec2
function I64Vec2:sub(p1,p2) end

---@param _self I64Vec2 
---@return integer
function I64Vec2:is_negative_bitmask(_self) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:div_euclid(_self,rhs) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return I64Vec2
function I64Vec2:sub(_self,rhs) end

---@param _self I64Vec2 
---@return I64Vec2
function I64Vec2:perp(_self) end

---@param _self I64Vec2 
---@param rhs U64Vec2 
---@return I64Vec2
function I64Vec2:saturating_sub_unsigned(_self,rhs) end

---@param a integer[] 
---@return I64Vec2
function I64Vec2.from_array(a) end

---@param _self I64Vec2 
---@param rhs I64Vec2 
---@return integer
function I64Vec2:perp_dot(_self,rhs) end

---@param _self I64Vec2 
---@return I64Vec2
function I64Vec2:clone(_self) end


---@class I64Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
I64Vec3 = {}

---@param _self I64Vec3 
---@return integer
function I64Vec3:element_sum(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:div_euclid(_self,rhs) end

---@param _self I64Vec3 
---@param min I64Vec3 
---@param max I64Vec3 
---@return I64Vec3
function I64Vec3:clamp(_self,min,max) end

---@param p1 I64Vec3 
---@param p2 integer 
---@return I64Vec3
function I64Vec3:rem(p1,p2) end

---@param p1 I64Vec3 
---@param p2 I64Vec3 
---@return I64Vec3
function I64Vec3:sub(p1,p2) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmpne(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:cross(_self,rhs) end

---@param p1 I64Vec3 
---@param p2 integer 
---@return I64Vec3
function I64Vec3:sub(p1,p2) end

---@param _self I64Vec3 
---@return I64Vec2
function I64Vec3:truncate(_self) end

---@param p1 I64Vec3 
---@param p2 I64Vec3 
---@return I64Vec3
function I64Vec3:rem(p1,p2) end

---@param _self I64Vec3 
---@param rhs U64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_add_unsigned(_self,rhs) end

---@param _self I64Vec3 
---@return I64Vec3
function I64Vec3:neg(_self) end

---@param _self I64Vec3 
---@return U16Vec3
function I64Vec3:as_u16vec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:saturating_div(_self,rhs) end

---@param _self I64Vec3 
---@param other I64Vec3 
---@return boolean
function I64Vec3:eq(_self,other) end

---@param p1 I64Vec3 
---@param p2 I64Vec3 
---@return I64Vec3
function I64Vec3:div(p1,p2) end

---@param _self I64Vec3 
---@return integer
function I64Vec3:is_negative_bitmask(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:saturating_mul(_self,rhs) end

---@param _self I64Vec3 
---@param rhs U64Vec3 
---@return I64Vec3
function I64Vec3:saturating_add_unsigned(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:max(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:saturating_add(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return integer
function I64Vec3:dot(_self,rhs) end

---@param v integer 
---@return I64Vec3
function I64Vec3.splat(v) end

---@param a integer[] 
---@return I64Vec3
function I64Vec3.from_array(a) end

---@param _self I64Vec3 
---@param w integer 
---@return I64Vec4
function I64Vec3:extend(_self,w) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmpgt(_self,rhs) end

---@param _self I64Vec3 
---@return I64Vec3
function I64Vec3:clone(_self) end

---@param p1 I64Vec3 
---@param p2 I64Vec3 
---@return I64Vec3
function I64Vec3:mul(p1,p2) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:rem(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:div(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return I64Vec3
function I64Vec3.new(x,y,z) end

---@param _self I64Vec3 
---@return nil
function I64Vec3:assert_receiver_is_total_eq(_self) end

---@param _self I64Vec3 
---@return U64Vec3
function I64Vec3:as_u64vec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:rem_euclid(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_mul(_self,rhs) end

---@param _self I64Vec3 
---@return I8Vec3
function I64Vec3:as_i8vec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:mul(_self,rhs) end

---@param _self I64Vec3 
---@return I64Vec3
function I64Vec3:abs(_self) end

---@param mask BVec3 
---@param if_true I64Vec3 
---@param if_false I64Vec3 
---@return I64Vec3
function I64Vec3.select(mask,if_true,if_false) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:add(_self,rhs) end

---@param _self I64Vec3 
---@param z integer 
---@return I64Vec3
function I64Vec3:with_z(_self,z) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return integer
function I64Vec3:distance_squared(_self,rhs) end

---@param _self I64Vec3 
---@return integer
function I64Vec3:min_element(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:min(_self,rhs) end

---@param _self I64Vec3 
---@return I64Vec3
function I64Vec3:signum(_self) end

---@param _self I64Vec3 
---@return integer[]
function I64Vec3:to_array(_self) end

---@param p1 I64Vec3 
---@param p2 integer 
---@return I64Vec3
function I64Vec3:div(p1,p2) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:dot_into_vec(_self,rhs) end

---@param _self I64Vec3 
---@return integer
function I64Vec3:element_product(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_div(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmpeq(_self,rhs) end

---@param p1 I64Vec3 
---@param p2 integer 
---@return I64Vec3
function I64Vec3:mul(p1,p2) end

---@param _self I64Vec3 
---@return DVec3
function I64Vec3:as_dvec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_sub(_self,rhs) end

---@param _self I64Vec3 
---@return integer
function I64Vec3:length_squared(_self) end

---@param _self I64Vec3 
---@return Vec3A
function I64Vec3:as_vec3a(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:sub(_self,rhs) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmple(_self,rhs) end

---@param _self I64Vec3 
---@return U8Vec3
function I64Vec3:as_u8vec3(_self) end

---@param p1 I64Vec3 
---@param p2 integer 
---@return I64Vec3
function I64Vec3:add(p1,p2) end

---@param _self I64Vec3 
---@return I16Vec3
function I64Vec3:as_i16vec3(_self) end

---@param _self I64Vec3 
---@param rhs U64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_sub_unsigned(_self,rhs) end

---@param _self I64Vec3 
---@return IVec3
function I64Vec3:as_ivec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:wrapping_add(_self,rhs) end

---@param _self I64Vec3 
---@return Vec3
function I64Vec3:as_vec3(_self) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmpge(_self,rhs) end

---@param _self I64Vec3 
---@return integer
function I64Vec3:max_element(_self) end

---@param _self I64Vec3 
---@param rhs U64Vec3 
---@return I64Vec3
function I64Vec3:saturating_sub_unsigned(_self,rhs) end

---@param p1 I64Vec3 
---@param p2 I64Vec3 
---@return I64Vec3
function I64Vec3:add(p1,p2) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return BVec3
function I64Vec3:cmplt(_self,rhs) end

---@param _self I64Vec3 
---@return UVec3
function I64Vec3:as_uvec3(_self) end

---@param _self I64Vec3 
---@param x integer 
---@return I64Vec3
function I64Vec3:with_x(_self,x) end

---@param _self I64Vec3 
---@param rhs I64Vec3 
---@return I64Vec3
function I64Vec3:saturating_sub(_self,rhs) end

---@param _self I64Vec3 
---@param y integer 
---@return I64Vec3
function I64Vec3:with_y(_self,y) end


---@class I64Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
I64Vec4 = {}

---@param p1 I64Vec4 
---@param p2 I64Vec4 
---@return I64Vec4
function I64Vec4:add(p1,p2) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmple(_self,rhs) end

---@param _self I64Vec4 
---@return I64Vec4
function I64Vec4:signum(_self) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:min_element(_self) end

---@param _self I64Vec4 
---@return I64Vec4
function I64Vec4:neg(_self) end

---@param _self I64Vec4 
---@param w integer 
---@return I64Vec4
function I64Vec4:with_w(_self,w) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_add(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return integer
function I64Vec4:dot(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 integer 
---@return I64Vec4
function I64Vec4:sub(p1,p2) end

---@param _self I64Vec4 
---@return I16Vec4
function I64Vec4:as_i16vec4(_self) end

---@param _self I64Vec4 
---@return I64Vec3
function I64Vec4:truncate(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmplt(_self,rhs) end

---@param _self I64Vec4 
---@param rhs U64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_add_unsigned(_self,rhs) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:length_squared(_self) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:max_element(_self) end

---@param _self I64Vec4 
---@param z integer 
---@return I64Vec4
function I64Vec4:with_z(_self,z) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:add(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 integer 
---@return I64Vec4
function I64Vec4:add(p1,p2) end

---@param _self I64Vec4 
---@return UVec4
function I64Vec4:as_uvec4(_self) end

---@param _self I64Vec4 
---@param min I64Vec4 
---@param max I64Vec4 
---@return I64Vec4
function I64Vec4:clamp(_self,min,max) end

---@param _self I64Vec4 
---@param y integer 
---@return I64Vec4
function I64Vec4:with_y(_self,y) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:sub(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:mul(_self,rhs) end

---@param mask BVec4 
---@param if_true I64Vec4 
---@param if_false I64Vec4 
---@return I64Vec4
function I64Vec4.select(mask,if_true,if_false) end

---@param p1 I64Vec4 
---@param p2 integer 
---@return I64Vec4
function I64Vec4:rem(p1,p2) end

---@param p1 I64Vec4 
---@param p2 integer 
---@return I64Vec4
function I64Vec4:mul(p1,p2) end

---@param _self I64Vec4 
---@param rhs U64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_sub_unsigned(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmpgt(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:rem_euclid(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return I64Vec4
function I64Vec4.new(x,y,z,w) end

---@param _self I64Vec4 
---@param x integer 
---@return I64Vec4
function I64Vec4:with_x(_self,x) end

---@param _self I64Vec4 
---@return integer[]
function I64Vec4:to_array(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmpge(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 I64Vec4 
---@return I64Vec4
function I64Vec4:div(p1,p2) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:div(_self,rhs) end

---@param _self I64Vec4 
---@return Vec4
function I64Vec4:as_vec4(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:saturating_sub(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_mul(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:div_euclid(_self,rhs) end

---@param _self I64Vec4 
---@param rhs U64Vec4 
---@return I64Vec4
function I64Vec4:saturating_add_unsigned(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:saturating_add(_self,rhs) end

---@param _self I64Vec4 
---@param rhs U64Vec4 
---@return I64Vec4
function I64Vec4:saturating_sub_unsigned(_self,rhs) end

---@param _self I64Vec4 
---@return IVec4
function I64Vec4:as_ivec4(_self) end

---@param _self I64Vec4 
---@param other I64Vec4 
---@return boolean
function I64Vec4:eq(_self,other) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:saturating_div(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_div(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 I64Vec4 
---@return I64Vec4
function I64Vec4:rem(p1,p2) end

---@param _self I64Vec4 
---@return nil
function I64Vec4:assert_receiver_is_total_eq(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:rem(_self,rhs) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:element_product(_self) end

---@param p1 I64Vec4 
---@param p2 integer 
---@return I64Vec4
function I64Vec4:div(p1,p2) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:min(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 I64Vec4 
---@return I64Vec4
function I64Vec4:mul(p1,p2) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:element_sum(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmpne(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:wrapping_sub(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:dot_into_vec(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return BVec4
function I64Vec4:cmpeq(_self,rhs) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:saturating_mul(_self,rhs) end

---@param _self I64Vec4 
---@return I64Vec4
function I64Vec4:clone(_self) end

---@param _self I64Vec4 
---@return I64Vec4
function I64Vec4:abs(_self) end

---@param _self I64Vec4 
---@return U8Vec4
function I64Vec4:as_u8vec4(_self) end

---@param _self I64Vec4 
---@return U16Vec4
function I64Vec4:as_u16vec4(_self) end

---@param _self I64Vec4 
---@return U64Vec4
function I64Vec4:as_u64vec4(_self) end

---@param _self I64Vec4 
---@return DVec4
function I64Vec4:as_dvec4(_self) end

---@param a integer[] 
---@return I64Vec4
function I64Vec4.from_array(a) end

---@param _self I64Vec4 
---@return I8Vec4
function I64Vec4:as_i8vec4(_self) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return I64Vec4
function I64Vec4:max(_self,rhs) end

---@param p1 I64Vec4 
---@param p2 I64Vec4 
---@return I64Vec4
function I64Vec4:sub(p1,p2) end

---@param _self I64Vec4 
---@return integer
function I64Vec4:is_negative_bitmask(_self) end

---@param v integer 
---@return I64Vec4
function I64Vec4.splat(v) end

---@param _self I64Vec4 
---@param rhs I64Vec4 
---@return integer
function I64Vec4:distance_squared(_self,rhs) end


---@class I8Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
I8Vec2 = {}

---@param v integer 
---@return I8Vec2
function I8Vec2.splat(v) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:mul(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:sub(_self,rhs) end

---@param _self I8Vec2 
---@return DVec2
function I8Vec2:as_dvec2(_self) end

---@param p1 I8Vec2 
---@param p2 I8Vec2 
---@return I8Vec2
function I8Vec2:div(p1,p2) end

---@param p1 I8Vec2 
---@param p2 integer 
---@return I8Vec2
function I8Vec2:mul(p1,p2) end

---@param _self I8Vec2 
---@param rhs U8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_sub_unsigned(_self,rhs) end

---@param _self I8Vec2 
---@param other I8Vec2 
---@return boolean
function I8Vec2:eq(_self,other) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:add(_self,rhs) end

---@param mask BVec2 
---@param if_true I8Vec2 
---@param if_false I8Vec2 
---@return I8Vec2
function I8Vec2.select(mask,if_true,if_false) end

---@param p1 I8Vec2 
---@param p2 I8Vec2 
---@return I8Vec2
function I8Vec2:mul(p1,p2) end

---@param _self I8Vec2 
---@return integer[]
function I8Vec2:to_array(_self) end

---@param _self I8Vec2 
---@return U8Vec2
function I8Vec2:as_u8vec2(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_add(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmpgt(_self,rhs) end

---@param _self I8Vec2 
---@return U16Vec2
function I8Vec2:as_u16vec2(_self) end

---@param _self I8Vec2 
---@return Vec2
function I8Vec2:as_vec2(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:saturating_div(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmplt(_self,rhs) end

---@param p1 I8Vec2 
---@param p2 integer 
---@return I8Vec2
function I8Vec2:add(p1,p2) end

---@param x integer 
---@param y integer 
---@return I8Vec2
function I8Vec2.new(x,y) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:div(_self,rhs) end

---@param _self I8Vec2 
---@param y integer 
---@return I8Vec2
function I8Vec2:with_y(_self,y) end

---@param p1 I8Vec2 
---@param p2 I8Vec2 
---@return I8Vec2
function I8Vec2:rem(p1,p2) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:is_negative_bitmask(_self) end

---@param a integer[] 
---@return I8Vec2
function I8Vec2.from_array(a) end

---@param _self I8Vec2 
---@param x integer 
---@return I8Vec2
function I8Vec2:with_x(_self,x) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_sub(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:saturating_mul(_self,rhs) end

---@param _self I8Vec2 
---@return I8Vec2
function I8Vec2:perp(_self) end

---@param p1 I8Vec2 
---@param p2 I8Vec2 
---@return I8Vec2
function I8Vec2:add(p1,p2) end

---@param p1 I8Vec2 
---@param p2 integer 
---@return I8Vec2
function I8Vec2:div(p1,p2) end

---@param _self I8Vec2 
---@return I64Vec2
function I8Vec2:as_i64vec2(_self) end

---@param p1 I8Vec2 
---@param p2 I8Vec2 
---@return I8Vec2
function I8Vec2:sub(p1,p2) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:element_product(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:max(_self,rhs) end

---@param _self I8Vec2 
---@return U64Vec2
function I8Vec2:as_u64vec2(_self) end

---@param p1 I8Vec2 
---@param p2 integer 
---@return I8Vec2
function I8Vec2:rem(p1,p2) end

---@param _self I8Vec2 
---@return I16Vec2
function I8Vec2:as_i16vec2(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:min(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:rem(_self,rhs) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:max_element(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmpeq(_self,rhs) end

---@param _self I8Vec2 
---@return IVec2
function I8Vec2:as_ivec2(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmpge(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:rotate(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmpne(_self,rhs) end

---@param _self I8Vec2 
---@param rhs U8Vec2 
---@return I8Vec2
function I8Vec2:saturating_sub_unsigned(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:rem_euclid(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return integer
function I8Vec2:distance_squared(_self,rhs) end

---@param _self I8Vec2 
---@param z integer 
---@return I8Vec3
function I8Vec2:extend(_self,z) end

---@param _self I8Vec2 
---@return I8Vec2
function I8Vec2:neg(_self) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:min_element(_self) end

---@param _self I8Vec2 
---@return I8Vec2
function I8Vec2:signum(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return integer
function I8Vec2:perp_dot(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return BVec2
function I8Vec2:cmple(_self,rhs) end

---@param p1 I8Vec2 
---@param p2 integer 
---@return I8Vec2
function I8Vec2:sub(p1,p2) end

---@param _self I8Vec2 
---@param min I8Vec2 
---@param max I8Vec2 
---@return I8Vec2
function I8Vec2:clamp(_self,min,max) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_div(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return integer
function I8Vec2:dot(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:dot_into_vec(_self,rhs) end

---@param _self I8Vec2 
---@return I8Vec2
function I8Vec2:clone(_self) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_mul(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:saturating_add(_self,rhs) end

---@param _self I8Vec2 
---@return I8Vec2
function I8Vec2:abs(_self) end

---@param _self I8Vec2 
---@param rhs U8Vec2 
---@return I8Vec2
function I8Vec2:wrapping_add_unsigned(_self,rhs) end

---@param _self I8Vec2 
---@return nil
function I8Vec2:assert_receiver_is_total_eq(_self) end

---@param _self I8Vec2 
---@param rhs U8Vec2 
---@return I8Vec2
function I8Vec2:saturating_add_unsigned(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:div_euclid(_self,rhs) end

---@param _self I8Vec2 
---@param rhs I8Vec2 
---@return I8Vec2
function I8Vec2:saturating_sub(_self,rhs) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:length_squared(_self) end

---@param _self I8Vec2 
---@return UVec2
function I8Vec2:as_uvec2(_self) end

---@param _self I8Vec2 
---@return integer
function I8Vec2:element_sum(_self) end


---@class I8Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
I8Vec3 = {}

---@param _self I8Vec3 
---@return Vec3
function I8Vec3:as_vec3(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:rem_euclid(_self,rhs) end

---@param _self I8Vec3 
---@param w integer 
---@return I8Vec4
function I8Vec3:extend(_self,w) end

---@param _self I8Vec3 
---@return U16Vec3
function I8Vec3:as_u16vec3(_self) end

---@param p1 I8Vec3 
---@param p2 integer 
---@return I8Vec3
function I8Vec3:add(p1,p2) end

---@param _self I8Vec3 
---@return I8Vec3
function I8Vec3:signum(_self) end

---@param _self I8Vec3 
---@return DVec3
function I8Vec3:as_dvec3(_self) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:element_product(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmpgt(_self,rhs) end

---@param _self I8Vec3 
---@param rhs U8Vec3 
---@return I8Vec3
function I8Vec3:saturating_sub_unsigned(_self,rhs) end

---@param _self I8Vec3 
---@param rhs U8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_add_unsigned(_self,rhs) end

---@param _self I8Vec3 
---@param rhs U8Vec3 
---@return I8Vec3
function I8Vec3:saturating_add_unsigned(_self,rhs) end

---@param _self I8Vec3 
---@return UVec3
function I8Vec3:as_uvec3(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:add(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:max(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:cross(_self,rhs) end

---@param p1 I8Vec3 
---@param p2 I8Vec3 
---@return I8Vec3
function I8Vec3:div(p1,p2) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:div_euclid(_self,rhs) end

---@param _self I8Vec3 
---@param min I8Vec3 
---@param max I8Vec3 
---@return I8Vec3
function I8Vec3:clamp(_self,min,max) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:max_element(_self) end

---@param p1 I8Vec3 
---@param p2 integer 
---@return I8Vec3
function I8Vec3:sub(p1,p2) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_sub(_self,rhs) end

---@param _self I8Vec3 
---@return Vec3A
function I8Vec3:as_vec3a(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmpeq(_self,rhs) end

---@param _self I8Vec3 
---@return I8Vec3
function I8Vec3:neg(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmpne(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_div(_self,rhs) end

---@param _self I8Vec3 
---@return U64Vec3
function I8Vec3:as_u64vec3(_self) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return I8Vec3
function I8Vec3.new(x,y,z) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:saturating_add(_self,rhs) end

---@param _self I8Vec3 
---@return I8Vec2
function I8Vec3:truncate(_self) end

---@param p1 I8Vec3 
---@param p2 I8Vec3 
---@return I8Vec3
function I8Vec3:sub(p1,p2) end

---@param p1 I8Vec3 
---@param p2 I8Vec3 
---@return I8Vec3
function I8Vec3:add(p1,p2) end

---@param _self I8Vec3 
---@return nil
function I8Vec3:assert_receiver_is_total_eq(_self) end

---@param v integer 
---@return I8Vec3
function I8Vec3.splat(v) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmplt(_self,rhs) end

---@param _self I8Vec3 
---@return I64Vec3
function I8Vec3:as_i64vec3(_self) end

---@param mask BVec3 
---@param if_true I8Vec3 
---@param if_false I8Vec3 
---@return I8Vec3
function I8Vec3.select(mask,if_true,if_false) end

---@param p1 I8Vec3 
---@param p2 integer 
---@return I8Vec3
function I8Vec3:rem(p1,p2) end

---@param a integer[] 
---@return I8Vec3
function I8Vec3.from_array(a) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:is_negative_bitmask(_self) end

---@param _self I8Vec3 
---@param other I8Vec3 
---@return boolean
function I8Vec3:eq(_self,other) end

---@param _self I8Vec3 
---@param y integer 
---@return I8Vec3
function I8Vec3:with_y(_self,y) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:mul(_self,rhs) end

---@param _self I8Vec3 
---@return integer[]
function I8Vec3:to_array(_self) end

---@param _self I8Vec3 
---@return I8Vec3
function I8Vec3:abs(_self) end

---@param _self I8Vec3 
---@param z integer 
---@return I8Vec3
function I8Vec3:with_z(_self,z) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:sub(_self,rhs) end

---@param p1 I8Vec3 
---@param p2 I8Vec3 
---@return I8Vec3
function I8Vec3:mul(p1,p2) end

---@param _self I8Vec3 
---@param x integer 
---@return I8Vec3
function I8Vec3:with_x(_self,x) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return integer
function I8Vec3:distance_squared(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:div(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:dot_into_vec(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return integer
function I8Vec3:dot(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_add(_self,rhs) end

---@param p1 I8Vec3 
---@param p2 I8Vec3 
---@return I8Vec3
function I8Vec3:rem(p1,p2) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:min_element(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:saturating_sub(_self,rhs) end

---@param _self I8Vec3 
---@param rhs U8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_sub_unsigned(_self,rhs) end

---@param _self I8Vec3 
---@return U8Vec3
function I8Vec3:as_u8vec3(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmple(_self,rhs) end

---@param _self I8Vec3 
---@return I8Vec3
function I8Vec3:clone(_self) end

---@param _self I8Vec3 
---@return I16Vec3
function I8Vec3:as_i16vec3(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:saturating_mul(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:saturating_div(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return BVec3
function I8Vec3:cmpge(_self,rhs) end

---@param p1 I8Vec3 
---@param p2 integer 
---@return I8Vec3
function I8Vec3:div(p1,p2) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:length_squared(_self) end

---@param _self I8Vec3 
---@return IVec3
function I8Vec3:as_ivec3(_self) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:min(_self,rhs) end

---@param p1 I8Vec3 
---@param p2 integer 
---@return I8Vec3
function I8Vec3:mul(p1,p2) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:wrapping_mul(_self,rhs) end

---@param _self I8Vec3 
---@param rhs I8Vec3 
---@return I8Vec3
function I8Vec3:rem(_self,rhs) end

---@param _self I8Vec3 
---@return integer
function I8Vec3:element_sum(_self) end


---@class I8Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
I8Vec4 = {}

---@param _self I8Vec4 
---@return IVec4
function I8Vec4:as_ivec4(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmple(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:max(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return integer
function I8Vec4:dot(_self,rhs) end

---@param _self I8Vec4 
---@param w integer 
---@return I8Vec4
function I8Vec4:with_w(_self,w) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:length_squared(_self) end

---@param _self I8Vec4 
---@param rhs U8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_add_unsigned(_self,rhs) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:max_element(_self) end

---@param p1 I8Vec4 
---@param p2 I8Vec4 
---@return I8Vec4
function I8Vec4:add(p1,p2) end

---@param _self I8Vec4 
---@return I8Vec3
function I8Vec4:truncate(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:sub(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:rem_euclid(_self,rhs) end

---@param _self I8Vec4 
---@return integer[]
function I8Vec4:to_array(_self) end

---@param _self I8Vec4 
---@param other I8Vec4 
---@return boolean
function I8Vec4:eq(_self,other) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:add(_self,rhs) end

---@param _self I8Vec4 
---@return U8Vec4
function I8Vec4:as_u8vec4(_self) end

---@param _self I8Vec4 
---@return nil
function I8Vec4:assert_receiver_is_total_eq(_self) end

---@param _self I8Vec4 
---@return U64Vec4
function I8Vec4:as_u64vec4(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:mul(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmpge(_self,rhs) end

---@param _self I8Vec4 
---@return I16Vec4
function I8Vec4:as_i16vec4(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:saturating_mul(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:min(_self,rhs) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:min_element(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:saturating_add(_self,rhs) end

---@param _self I8Vec4 
---@return I64Vec4
function I8Vec4:as_i64vec4(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmplt(_self,rhs) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:is_negative_bitmask(_self) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return I8Vec4
function I8Vec4.new(x,y,z,w) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:saturating_div(_self,rhs) end

---@param _self I8Vec4 
---@return DVec4
function I8Vec4:as_dvec4(_self) end

---@param v integer 
---@return I8Vec4
function I8Vec4.splat(v) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:div_euclid(_self,rhs) end

---@param _self I8Vec4 
---@return I8Vec4
function I8Vec4:signum(_self) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:element_product(_self) end

---@param _self I8Vec4 
---@param x integer 
---@return I8Vec4
function I8Vec4:with_x(_self,x) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:rem(_self,rhs) end

---@param _self I8Vec4 
---@return UVec4
function I8Vec4:as_uvec4(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_div(_self,rhs) end

---@param p1 I8Vec4 
---@param p2 integer 
---@return I8Vec4
function I8Vec4:sub(p1,p2) end

---@param _self I8Vec4 
---@param min I8Vec4 
---@param max I8Vec4 
---@return I8Vec4
function I8Vec4:clamp(_self,min,max) end

---@param p1 I8Vec4 
---@param p2 I8Vec4 
---@return I8Vec4
function I8Vec4:sub(p1,p2) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_mul(_self,rhs) end

---@param _self I8Vec4 
---@return I8Vec4
function I8Vec4:clone(_self) end

---@param _self I8Vec4 
---@param rhs U8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_sub_unsigned(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmpeq(_self,rhs) end

---@param _self I8Vec4 
---@param z integer 
---@return I8Vec4
function I8Vec4:with_z(_self,z) end

---@param p1 I8Vec4 
---@param p2 integer 
---@return I8Vec4
function I8Vec4:add(p1,p2) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmpne(_self,rhs) end

---@param _self I8Vec4 
---@return I8Vec4
function I8Vec4:neg(_self) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_sub(_self,rhs) end

---@param _self I8Vec4 
---@param rhs U8Vec4 
---@return I8Vec4
function I8Vec4:saturating_add_unsigned(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:dot_into_vec(_self,rhs) end

---@param p1 I8Vec4 
---@param p2 I8Vec4 
---@return I8Vec4
function I8Vec4:rem(p1,p2) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return BVec4
function I8Vec4:cmpgt(_self,rhs) end

---@param p1 I8Vec4 
---@param p2 I8Vec4 
---@return I8Vec4
function I8Vec4:div(p1,p2) end

---@param p1 I8Vec4 
---@param p2 I8Vec4 
---@return I8Vec4
function I8Vec4:mul(p1,p2) end

---@param _self I8Vec4 
---@return U16Vec4
function I8Vec4:as_u16vec4(_self) end

---@param _self I8Vec4 
---@param rhs U8Vec4 
---@return I8Vec4
function I8Vec4:saturating_sub_unsigned(_self,rhs) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:div(_self,rhs) end

---@param _self I8Vec4 
---@return I8Vec4
function I8Vec4:abs(_self) end

---@param _self I8Vec4 
---@param y integer 
---@return I8Vec4
function I8Vec4:with_y(_self,y) end

---@param _self I8Vec4 
---@return integer
function I8Vec4:element_sum(_self) end

---@param a integer[] 
---@return I8Vec4
function I8Vec4.from_array(a) end

---@param p1 I8Vec4 
---@param p2 integer 
---@return I8Vec4
function I8Vec4:rem(p1,p2) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:saturating_sub(_self,rhs) end

---@param mask BVec4 
---@param if_true I8Vec4 
---@param if_false I8Vec4 
---@return I8Vec4
function I8Vec4.select(mask,if_true,if_false) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return I8Vec4
function I8Vec4:wrapping_add(_self,rhs) end

---@param _self I8Vec4 
---@return Vec4
function I8Vec4:as_vec4(_self) end

---@param p1 I8Vec4 
---@param p2 integer 
---@return I8Vec4
function I8Vec4:mul(p1,p2) end

---@param p1 I8Vec4 
---@param p2 integer 
---@return I8Vec4
function I8Vec4:div(p1,p2) end

---@param _self I8Vec4 
---@param rhs I8Vec4 
---@return integer
function I8Vec4:distance_squared(_self,rhs) end


---@class IVec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
IVec2 = {}

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:rotate(_self,rhs) end

---@param _self IVec2 
---@return DVec2
function IVec2:as_dvec2(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:add(_self,rhs) end

---@param _self IVec2 
---@return I8Vec2
function IVec2:as_i8vec2(_self) end

---@param _self IVec2 
---@param min IVec2 
---@param max IVec2 
---@return IVec2
function IVec2:clamp(_self,min,max) end

---@param _self IVec2 
---@param rhs IVec2 
---@return integer
function IVec2:distance_squared(_self,rhs) end

---@param p1 IVec2 
---@param p2 IVec2 
---@return IVec2
function IVec2:sub(p1,p2) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:saturating_mul(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:div(_self,rhs) end

---@param _self IVec2 
---@param rhs UVec2 
---@return IVec2
function IVec2:wrapping_add_unsigned(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmpne(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:mul(_self,rhs) end

---@param p1 IVec2 
---@param p2 IVec2 
---@return IVec2
function IVec2:div(p1,p2) end

---@param p1 IVec2 
---@param p2 integer 
---@return IVec2
function IVec2:rem(p1,p2) end

---@param _self IVec2 
---@return UVec2
function IVec2:as_uvec2(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:wrapping_add(_self,rhs) end

---@param p1 IVec2 
---@param p2 IVec2 
---@return IVec2
function IVec2:add(p1,p2) end

---@param p1 IVec2 
---@param p2 integer 
---@return IVec2
function IVec2:div(p1,p2) end

---@param _self IVec2 
---@return integer
function IVec2:element_sum(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmpeq(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:rem(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:div_euclid(_self,rhs) end

---@param _self IVec2 
---@return integer[]
function IVec2:to_array(_self) end

---@param _self IVec2 
---@return nil
function IVec2:assert_receiver_is_total_eq(_self) end

---@param _self IVec2 
---@return integer
function IVec2:max_element(_self) end

---@param _self IVec2 
---@param other IVec2 
---@return boolean
function IVec2:eq(_self,other) end

---@param _self IVec2 
---@param rhs UVec2 
---@return IVec2
function IVec2:wrapping_sub_unsigned(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmple(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:saturating_div(_self,rhs) end

---@param p1 IVec2 
---@param p2 integer 
---@return IVec2
function IVec2:mul(p1,p2) end

---@param _self IVec2 
---@param rhs UVec2 
---@return IVec2
function IVec2:saturating_sub_unsigned(_self,rhs) end

---@param a integer[] 
---@return IVec2
function IVec2.from_array(a) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmpgt(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmplt(_self,rhs) end

---@param _self IVec2 
---@param x integer 
---@return IVec2
function IVec2:with_x(_self,x) end

---@param v integer 
---@return IVec2
function IVec2.splat(v) end

---@param _self IVec2 
---@return integer
function IVec2:is_negative_bitmask(_self) end

---@param p1 IVec2 
---@param p2 IVec2 
---@return IVec2
function IVec2:rem(p1,p2) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:wrapping_mul(_self,rhs) end

---@param _self IVec2 
---@return IVec2
function IVec2:perp(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return integer
function IVec2:perp_dot(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:dot_into_vec(_self,rhs) end

---@param p1 IVec2 
---@param p2 IVec2 
---@return IVec2
function IVec2:mul(p1,p2) end

---@param mask BVec2 
---@param if_true IVec2 
---@param if_false IVec2 
---@return IVec2
function IVec2.select(mask,if_true,if_false) end

---@param _self IVec2 
---@return U64Vec2
function IVec2:as_u64vec2(_self) end

---@param p1 IVec2 
---@param p2 integer 
---@return IVec2
function IVec2:add(p1,p2) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:rem_euclid(_self,rhs) end

---@param _self IVec2 
---@return integer
function IVec2:length_squared(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return integer
function IVec2:dot(_self,rhs) end

---@param _self IVec2 
---@return U8Vec2
function IVec2:as_u8vec2(_self) end

---@param _self IVec2 
---@return Vec2
function IVec2:as_vec2(_self) end

---@param _self IVec2 
---@return U16Vec2
function IVec2:as_u16vec2(_self) end

---@param p1 IVec2 
---@param p2 integer 
---@return IVec2
function IVec2:sub(p1,p2) end

---@param _self IVec2 
---@param rhs IVec2 
---@return BVec2
function IVec2:cmpge(_self,rhs) end

---@param _self IVec2 
---@return IVec2
function IVec2:neg(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:wrapping_sub(_self,rhs) end

---@param _self IVec2 
---@param z integer 
---@return IVec3
function IVec2:extend(_self,z) end

---@param _self IVec2 
---@param y integer 
---@return IVec2
function IVec2:with_y(_self,y) end

---@param _self IVec2 
---@return IVec2
function IVec2:clone(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:wrapping_div(_self,rhs) end

---@param _self IVec2 
---@return I64Vec2
function IVec2:as_i64vec2(_self) end

---@param x integer 
---@param y integer 
---@return IVec2
function IVec2.new(x,y) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:saturating_sub(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:saturating_add(_self,rhs) end

---@param _self IVec2 
---@return integer
function IVec2:element_product(_self) end

---@param _self IVec2 
---@return IVec2
function IVec2:signum(_self) end

---@param _self IVec2 
---@return I16Vec2
function IVec2:as_i16vec2(_self) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:min(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:max(_self,rhs) end

---@param _self IVec2 
---@param rhs IVec2 
---@return IVec2
function IVec2:sub(_self,rhs) end

---@param _self IVec2 
---@return integer
function IVec2:min_element(_self) end

---@param _self IVec2 
---@return IVec2
function IVec2:abs(_self) end

---@param _self IVec2 
---@param rhs UVec2 
---@return IVec2
function IVec2:saturating_add_unsigned(_self,rhs) end


---@class IVec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
IVec3 = {}

---@param _self IVec3 
---@param other IVec3 
---@return boolean
function IVec3:eq(_self,other) end

---@param a integer[] 
---@return IVec3
function IVec3.from_array(a) end

---@param p1 IVec3 
---@param p2 integer 
---@return IVec3
function IVec3:rem(p1,p2) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:saturating_sub(_self,rhs) end

---@param p1 IVec3 
---@param p2 IVec3 
---@return IVec3
function IVec3:mul(p1,p2) end

---@param _self IVec3 
---@return U64Vec3
function IVec3:as_u64vec3(_self) end

---@param _self IVec3 
---@return integer
function IVec3:min_element(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmpne(_self,rhs) end

---@param _self IVec3 
---@param y integer 
---@return IVec3
function IVec3:with_y(_self,y) end

---@param p1 IVec3 
---@param p2 integer 
---@return IVec3
function IVec3:div(p1,p2) end

---@param _self IVec3 
---@return IVec2
function IVec3:truncate(_self) end

---@param _self IVec3 
---@return Vec3
function IVec3:as_vec3(_self) end

---@param mask BVec3 
---@param if_true IVec3 
---@param if_false IVec3 
---@return IVec3
function IVec3.select(mask,if_true,if_false) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:mul(_self,rhs) end

---@param _self IVec3 
---@param min IVec3 
---@param max IVec3 
---@return IVec3
function IVec3:clamp(_self,min,max) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:wrapping_div(_self,rhs) end

---@param _self IVec3 
---@param x integer 
---@return IVec3
function IVec3:with_x(_self,x) end

---@param p1 IVec3 
---@param p2 integer 
---@return IVec3
function IVec3:add(p1,p2) end

---@param _self IVec3 
---@param rhs UVec3 
---@return IVec3
function IVec3:wrapping_sub_unsigned(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:wrapping_add(_self,rhs) end

---@param _self IVec3 
---@return I16Vec3
function IVec3:as_i16vec3(_self) end

---@param _self IVec3 
---@return I64Vec3
function IVec3:as_i64vec3(_self) end

---@param _self IVec3 
---@return integer
function IVec3:max_element(_self) end

---@param _self IVec3 
---@param rhs UVec3 
---@return IVec3
function IVec3:saturating_sub_unsigned(_self,rhs) end

---@param _self IVec3 
---@return integer
function IVec3:element_sum(_self) end

---@param p1 IVec3 
---@param p2 integer 
---@return IVec3
function IVec3:sub(p1,p2) end

---@param _self IVec3 
---@param rhs UVec3 
---@return IVec3
function IVec3:wrapping_add_unsigned(_self,rhs) end

---@param _self IVec3 
---@return U8Vec3
function IVec3:as_u8vec3(_self) end

---@param _self IVec3 
---@return IVec3
function IVec3:clone(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:saturating_div(_self,rhs) end

---@param _self IVec3 
---@param w integer 
---@return IVec4
function IVec3:extend(_self,w) end

---@param p1 IVec3 
---@param p2 integer 
---@return IVec3
function IVec3:mul(p1,p2) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:div_euclid(_self,rhs) end

---@param p1 IVec3 
---@param p2 IVec3 
---@return IVec3
function IVec3:add(p1,p2) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:sub(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:max(_self,rhs) end

---@param _self IVec3 
---@return integer
function IVec3:length_squared(_self) end

---@param _self IVec3 
---@return IVec3
function IVec3:signum(_self) end

---@param _self IVec3 
---@return U16Vec3
function IVec3:as_u16vec3(_self) end

---@param _self IVec3 
---@return UVec3
function IVec3:as_uvec3(_self) end

---@param v integer 
---@return IVec3
function IVec3.splat(v) end

---@param _self IVec3 
---@return IVec3
function IVec3:abs(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:cross(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmpgt(_self,rhs) end

---@param p1 IVec3 
---@param p2 IVec3 
---@return IVec3
function IVec3:div(p1,p2) end

---@param _self IVec3 
---@return Vec3A
function IVec3:as_vec3a(_self) end

---@param _self IVec3 
---@return nil
function IVec3:assert_receiver_is_total_eq(_self) end

---@param _self IVec3 
---@param z integer 
---@return IVec3
function IVec3:with_z(_self,z) end

---@param _self IVec3 
---@return integer
function IVec3:is_negative_bitmask(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return integer
function IVec3:distance_squared(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:wrapping_sub(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return IVec3
function IVec3.new(x,y,z) end

---@param _self IVec3 
---@param rhs UVec3 
---@return IVec3
function IVec3:saturating_add_unsigned(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:saturating_add(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:rem_euclid(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:min(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return integer
function IVec3:dot(_self,rhs) end

---@param _self IVec3 
---@return IVec3
function IVec3:neg(_self) end

---@param p1 IVec3 
---@param p2 IVec3 
---@return IVec3
function IVec3:rem(p1,p2) end

---@param _self IVec3 
---@return integer[]
function IVec3:to_array(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:dot_into_vec(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:div(_self,rhs) end

---@param _self IVec3 
---@return I8Vec3
function IVec3:as_i8vec3(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:rem(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:saturating_mul(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmpge(_self,rhs) end

---@param p1 IVec3 
---@param p2 IVec3 
---@return IVec3
function IVec3:sub(p1,p2) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmple(_self,rhs) end

---@param _self IVec3 
---@return DVec3
function IVec3:as_dvec3(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmpeq(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:add(_self,rhs) end

---@param _self IVec3 
---@param rhs IVec3 
---@return BVec3
function IVec3:cmplt(_self,rhs) end

---@param _self IVec3 
---@return integer
function IVec3:element_product(_self) end

---@param _self IVec3 
---@param rhs IVec3 
---@return IVec3
function IVec3:wrapping_mul(_self,rhs) end


---@class IVec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
IVec4 = {}

---@param _self IVec4 
---@return nil
function IVec4:assert_receiver_is_total_eq(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:saturating_add(_self,rhs) end

---@param _self IVec4 
---@param min IVec4 
---@param max IVec4 
---@return IVec4
function IVec4:clamp(_self,min,max) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmpeq(_self,rhs) end

---@param v integer 
---@return IVec4
function IVec4.splat(v) end

---@param _self IVec4 
---@param rhs UVec4 
---@return IVec4
function IVec4:saturating_sub_unsigned(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return integer
function IVec4:distance_squared(_self,rhs) end

---@param _self IVec4 
---@return integer
function IVec4:length_squared(_self) end

---@param p1 IVec4 
---@param p2 IVec4 
---@return IVec4
function IVec4:rem(p1,p2) end

---@param _self IVec4 
---@param z integer 
---@return IVec4
function IVec4:with_z(_self,z) end

---@param p1 IVec4 
---@param p2 IVec4 
---@return IVec4
function IVec4:sub(p1,p2) end

---@param _self IVec4 
---@return IVec4
function IVec4:neg(_self) end

---@param _self IVec4 
---@return integer
function IVec4:min_element(_self) end

---@param _self IVec4 
---@return I16Vec4
function IVec4:as_i16vec4(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:wrapping_div(_self,rhs) end

---@param p1 IVec4 
---@param p2 integer 
---@return IVec4
function IVec4:rem(p1,p2) end

---@param _self IVec4 
---@return integer
function IVec4:max_element(_self) end

---@param _self IVec4 
---@return UVec4
function IVec4:as_uvec4(_self) end

---@param mask BVec4 
---@param if_true IVec4 
---@param if_false IVec4 
---@return IVec4
function IVec4.select(mask,if_true,if_false) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:dot_into_vec(_self,rhs) end

---@param _self IVec4 
---@return U8Vec4
function IVec4:as_u8vec4(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:saturating_div(_self,rhs) end

---@param _self IVec4 
---@return integer
function IVec4:element_product(_self) end

---@param _self IVec4 
---@return integer[]
function IVec4:to_array(_self) end

---@param _self IVec4 
---@param rhs UVec4 
---@return IVec4
function IVec4:wrapping_add_unsigned(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:rem_euclid(_self,rhs) end

---@param _self IVec4 
---@return DVec4
function IVec4:as_dvec4(_self) end

---@param _self IVec4 
---@return IVec4
function IVec4:abs(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:add(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:max(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:div(_self,rhs) end

---@param _self IVec4 
---@return integer
function IVec4:is_negative_bitmask(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:mul(_self,rhs) end

---@param _self IVec4 
---@return U16Vec4
function IVec4:as_u16vec4(_self) end

---@param _self IVec4 
---@return U64Vec4
function IVec4:as_u64vec4(_self) end

---@param _self IVec4 
---@param rhs UVec4 
---@return IVec4
function IVec4:wrapping_sub_unsigned(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:saturating_sub(_self,rhs) end

---@param p1 IVec4 
---@param p2 integer 
---@return IVec4
function IVec4:div(p1,p2) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:min(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmpge(_self,rhs) end

---@param _self IVec4 
---@return IVec4
function IVec4:clone(_self) end

---@param _self IVec4 
---@return Vec4
function IVec4:as_vec4(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmpgt(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:sub(_self,rhs) end

---@param p1 IVec4 
---@param p2 integer 
---@return IVec4
function IVec4:sub(p1,p2) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:rem(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmpne(_self,rhs) end

---@param _self IVec4 
---@return I8Vec4
function IVec4:as_i8vec4(_self) end

---@param _self IVec4 
---@param y integer 
---@return IVec4
function IVec4:with_y(_self,y) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:div_euclid(_self,rhs) end

---@param a integer[] 
---@return IVec4
function IVec4.from_array(a) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:saturating_mul(_self,rhs) end

---@param p1 IVec4 
---@param p2 IVec4 
---@return IVec4
function IVec4:add(p1,p2) end

---@param _self IVec4 
---@return integer
function IVec4:element_sum(_self) end

---@param p1 IVec4 
---@param p2 IVec4 
---@return IVec4
function IVec4:div(p1,p2) end

---@param _self IVec4 
---@param other IVec4 
---@return boolean
function IVec4:eq(_self,other) end

---@param _self IVec4 
---@param w integer 
---@return IVec4
function IVec4:with_w(_self,w) end

---@param _self IVec4 
---@return IVec4
function IVec4:signum(_self) end

---@param p1 IVec4 
---@param p2 integer 
---@return IVec4
function IVec4:mul(p1,p2) end

---@param p1 IVec4 
---@param p2 integer 
---@return IVec4
function IVec4:add(p1,p2) end

---@param _self IVec4 
---@param x integer 
---@return IVec4
function IVec4:with_x(_self,x) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:wrapping_mul(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmplt(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:wrapping_add(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return IVec4
function IVec4.new(x,y,z,w) end

---@param _self IVec4 
---@return I64Vec4
function IVec4:as_i64vec4(_self) end

---@param _self IVec4 
---@param rhs IVec4 
---@return IVec4
function IVec4:wrapping_sub(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return BVec4
function IVec4:cmple(_self,rhs) end

---@param _self IVec4 
---@param rhs UVec4 
---@return IVec4
function IVec4:saturating_add_unsigned(_self,rhs) end

---@param _self IVec4 
---@param rhs IVec4 
---@return integer
function IVec4:dot(_self,rhs) end

---@param _self IVec4 
---@return IVec3
function IVec4:truncate(_self) end

---@param p1 IVec4 
---@param p2 IVec4 
---@return IVec4
function IVec4:mul(p1,p2) end


---@class Mat2 : ReflectReference
---@field  x_axis ? Vec2
---@field  y_axis ? Vec2
Mat2 = {}

---@param _self Mat2 
---@return Mat2
function Mat2:clone(_self) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:mul(_self,rhs) end

---@param _self Mat2 
---@param index integer 
---@return Vec2
function Mat2:row(_self,index) end

---@param m Mat3A 
---@return Mat2
function Mat2.from_mat3a(m) end

---@param _self Mat2 
---@param rhs number 
---@return Mat2
function Mat2:div(_self,rhs) end

---@param _self Mat2 
---@return boolean
function Mat2:is_nan(_self) end

---@param _self Mat2 
---@param rhs Mat2 
---@param max_abs_diff number 
---@return boolean
function Mat2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Mat2 
---@param rhs number 
---@return Mat2
function Mat2:mul_scalar(_self,rhs) end

---@param p1 Mat2 
---@param p2 number 
---@return Mat2
function Mat2:mul(p1,p2) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:add_mat2(_self,rhs) end

---@param _self Mat2 
---@return number[][]
function Mat2:to_cols_array_2d(_self) end

---@param x_axis Vec2 
---@param y_axis Vec2 
---@return Mat2
function Mat2.from_cols(x_axis,y_axis) end

---@param _self Mat2 
---@param rhs number 
---@return Mat2
function Mat2:div_scalar(_self,rhs) end

---@param _self Mat2 
---@return number[]
function Mat2:to_cols_array(_self) end

---@param _self Mat2 
---@return Mat2
function Mat2:transpose(_self) end

---@param _self Mat2 
---@return boolean
function Mat2:is_finite(_self) end

---@param _self Mat2 
---@return Mat2
function Mat2:inverse(_self) end

---@param _self Mat2 
---@param rhs Mat2 
---@return boolean
function Mat2:eq(_self,rhs) end

---@param _self Mat2 
---@return DMat2
function Mat2:as_dmat2(_self) end

---@param _self Mat2 
---@return Mat2
function Mat2:abs(_self) end

---@param m Mat3 
---@return Mat2
function Mat2.from_mat3(m) end

---@param m Mat3A 
---@param i integer 
---@param j integer 
---@return Mat2
function Mat2.from_mat3a_minor(m,i,j) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:mul_mat2(_self,rhs) end

---@param angle number 
---@return Mat2
function Mat2.from_angle(angle) end

---@param diagonal Vec2 
---@return Mat2
function Mat2.from_diagonal(diagonal) end

---@param _self Mat2 
---@param index integer 
---@return Vec2
function Mat2:col(_self,index) end

---@param _self Mat2 
---@param rhs Vec2 
---@return Vec2
function Mat2:mul_vec2(_self,rhs) end

---@param _self Mat2 
---@return number
function Mat2:determinant(_self) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:add(_self,rhs) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:sub_mat2(_self,rhs) end

---@param _self Mat2 
---@param rhs Mat2 
---@return Mat2
function Mat2:sub(_self,rhs) end

---@param _self Mat2 
---@return Mat2
function Mat2:neg(_self) end

---@param m Mat3 
---@param i integer 
---@param j integer 
---@return Mat2
function Mat2.from_mat3_minor(m,i,j) end

---@param scale Vec2 
---@param angle number 
---@return Mat2
function Mat2.from_scale_angle(scale,angle) end

---@param p1 Mat2 
---@param p2 Vec2 
---@return Vec2
function Mat2:mul(p1,p2) end


---@class Mat3 : ReflectReference
---@field  x_axis ? Vec3
---@field  y_axis ? Vec3
---@field  z_axis ? Vec3
Mat3 = {}

---@param translation Vec2 
---@return Mat3
function Mat3.from_translation(translation) end

---@param x_axis Vec3 
---@param y_axis Vec3 
---@param z_axis Vec3 
---@return Mat3
function Mat3.from_cols(x_axis,y_axis,z_axis) end

---@param m Mat2 
---@return Mat3
function Mat3.from_mat2(m) end

---@param _self Mat3 
---@return boolean
function Mat3:is_finite(_self) end

---@param p1 Mat3 
---@param p2 Mat3 
---@return Mat3
function Mat3:mul(p1,p2) end

---@param m Mat4 
---@return Mat3
function Mat3.from_mat4(m) end

---@param _self Mat3 
---@param rhs Mat3 
---@return Mat3
function Mat3:add(_self,rhs) end

---@param diagonal Vec3 
---@return Mat3
function Mat3.from_diagonal(diagonal) end

---@param _self Mat3 
---@return Mat3
function Mat3:clone(_self) end

---@param _self Mat3 
---@param index integer 
---@return Vec3
function Mat3:col(_self,index) end

---@param order EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return Mat3
function Mat3.from_euler(order,a,b,c) end

---@param p1 Mat3 
---@param p2 Vec3 
---@return Vec3
function Mat3:mul(p1,p2) end

---@param _self Mat3 
---@param rhs Mat3 
---@param max_abs_diff number 
---@return boolean
function Mat3:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param scale Vec2 
---@param angle number 
---@param translation Vec2 
---@return Mat3
function Mat3.from_scale_angle_translation(scale,angle,translation) end

---@param _self Mat3 
---@param rhs Mat3 
---@return Mat3
function Mat3:sub(_self,rhs) end

---@param scale Vec2 
---@return Mat3
function Mat3.from_scale(scale) end

---@param p1 Mat3 
---@param p2 Vec3A 
---@return Vec3A
function Mat3:mul(p1,p2) end

---@param angle number 
---@return Mat3
function Mat3.from_rotation_y(angle) end

---@param angle number 
---@return Mat3
function Mat3.from_angle(angle) end

---@param rotation Quat 
---@return Mat3
function Mat3.from_quat(rotation) end

---@param _self Mat3 
---@return number[]
function Mat3:to_cols_array(_self) end

---@param _self Mat3 
---@return Mat3
function Mat3:abs(_self) end

---@param _self Mat3 
---@param rhs Mat3 
---@return Mat3
function Mat3:add_mat3(_self,rhs) end

---@param _self Mat3 
---@param order EulerRot 
---@return [number, number, number]
function Mat3:to_euler(_self,order) end

---@param _self Mat3 
---@return Mat3
function Mat3:transpose(_self) end

---@param _self Mat3 
---@param rhs Vec3A 
---@return Vec3A
function Mat3:mul_vec3a(_self,rhs) end

---@param axis Vec3 
---@param angle number 
---@return Mat3
function Mat3.from_axis_angle(axis,angle) end

---@param angle number 
---@return Mat3
function Mat3.from_rotation_x(angle) end

---@param _self Mat3 
---@return boolean
function Mat3:is_nan(_self) end

---@param _self Mat3 
---@return number
function Mat3:determinant(_self) end

---@param _self Mat3 
---@return Mat3
function Mat3:inverse(_self) end

---@param _self Mat3 
---@param index integer 
---@return Vec3
function Mat3:row(_self,index) end

---@param _self Mat3 
---@param rhs Vec2 
---@return Vec2
function Mat3:transform_vector2(_self,rhs) end

---@param _self Mat3 
---@return DMat3
function Mat3:as_dmat3(_self) end

---@param _self Mat3 
---@return number[][]
function Mat3:to_cols_array_2d(_self) end

---@param _self Mat3 
---@param rhs Vec2 
---@return Vec2
function Mat3:transform_point2(_self,rhs) end

---@param _self Mat3 
---@return Mat3
function Mat3:neg(_self) end

---@param angle number 
---@return Mat3
function Mat3.from_rotation_z(angle) end

---@param m Mat4 
---@param i integer 
---@param j integer 
---@return Mat3
function Mat3.from_mat4_minor(m,i,j) end

---@param _self Mat3 
---@param rhs number 
---@return Mat3
function Mat3:div_scalar(_self,rhs) end

---@param _self Mat3 
---@param rhs number 
---@return Mat3
function Mat3:div(_self,rhs) end

---@param _self Mat3 
---@param rhs number 
---@return Mat3
function Mat3:mul_scalar(_self,rhs) end

---@param p1 Mat3 
---@param p2 number 
---@return Mat3
function Mat3:mul(p1,p2) end

---@param _self Mat3 
---@param rhs Mat3 
---@return Mat3
function Mat3:mul_mat3(_self,rhs) end

---@param _self Mat3 
---@param rhs Vec3 
---@return Vec3
function Mat3:mul_vec3(_self,rhs) end

---@param _self Mat3 
---@param rhs Affine2 
---@return Mat3
function Mat3:mul(_self,rhs) end

---@param _self Mat3 
---@param rhs Mat3 
---@return boolean
function Mat3:eq(_self,rhs) end

---@param _self Mat3 
---@param rhs Mat3 
---@return Mat3
function Mat3:sub_mat3(_self,rhs) end


---@class Mat3A : ReflectReference
---@field  x_axis ? Vec3A
---@field  y_axis ? Vec3A
---@field  z_axis ? Vec3A
Mat3A = {}

---@param _self Mat3A 
---@return number[]
function Mat3A:to_cols_array(_self) end

---@param p1 Mat3A 
---@param p2 Vec3 
---@return Vec3
function Mat3A:mul(p1,p2) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return Mat3A
function Mat3A:sub(_self,rhs) end

---@param _self Mat3A 
---@param order EulerRot 
---@return [number, number, number]
function Mat3A:to_euler(_self,order) end

---@param _self Mat3A 
---@return Mat3A
function Mat3A:inverse(_self) end

---@param angle number 
---@return Mat3A
function Mat3A.from_angle(angle) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return Mat3A
function Mat3A:add(_self,rhs) end

---@param rotation Quat 
---@return Mat3A
function Mat3A.from_quat(rotation) end

---@param _self Mat3A 
---@return DMat3
function Mat3A:as_dmat3(_self) end

---@param _self Mat3A 
---@return boolean
function Mat3A:is_finite(_self) end

---@param _self Mat3A 
---@param rhs Affine2 
---@return Mat3A
function Mat3A:mul(_self,rhs) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return boolean
function Mat3A:eq(_self,rhs) end

---@param axis Vec3 
---@param angle number 
---@return Mat3A
function Mat3A.from_axis_angle(axis,angle) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return Mat3A
function Mat3A:add_mat3(_self,rhs) end

---@param _self Mat3A 
---@param rhs number 
---@return Mat3A
function Mat3A:div(_self,rhs) end

---@param p1 Mat3A 
---@param p2 number 
---@return Mat3A
function Mat3A:mul(p1,p2) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@param max_abs_diff number 
---@return boolean
function Mat3A:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return Mat3A
function Mat3A:mul_mat3(_self,rhs) end

---@param _self Mat3A 
---@return Mat3A
function Mat3A:neg(_self) end

---@param scale Vec2 
---@return Mat3A
function Mat3A.from_scale(scale) end

---@param _self Mat3A 
---@return boolean
function Mat3A:is_nan(_self) end

---@param _self Mat3A 
---@return number[][]
function Mat3A:to_cols_array_2d(_self) end

---@param angle number 
---@return Mat3A
function Mat3A.from_rotation_x(angle) end

---@param _self Mat3A 
---@return number
function Mat3A:determinant(_self) end

---@param angle number 
---@return Mat3A
function Mat3A.from_rotation_z(angle) end

---@param p1 Mat3A 
---@param p2 Mat3A 
---@return Mat3A
function Mat3A:mul(p1,p2) end

---@param p1 Mat3A 
---@param p2 Vec3A 
---@return Vec3A
function Mat3A:mul(p1,p2) end

---@param m Mat4 
---@return Mat3A
function Mat3A.from_mat4(m) end

---@param _self Mat3A 
---@param index integer 
---@return Vec3A
function Mat3A:row(_self,index) end

---@param _self Mat3A 
---@param rhs Vec2 
---@return Vec2
function Mat3A:transform_point2(_self,rhs) end

---@param scale Vec2 
---@param angle number 
---@param translation Vec2 
---@return Mat3A
function Mat3A.from_scale_angle_translation(scale,angle,translation) end

---@param _self Mat3A 
---@param rhs number 
---@return Mat3A
function Mat3A:mul_scalar(_self,rhs) end

---@param _self Mat3A 
---@param rhs Vec3A 
---@return Vec3A
function Mat3A:mul_vec3a(_self,rhs) end

---@param diagonal Vec3 
---@return Mat3A
function Mat3A.from_diagonal(diagonal) end

---@param _self Mat3A 
---@param rhs Vec2 
---@return Vec2
function Mat3A:transform_vector2(_self,rhs) end

---@param _self Mat3A 
---@return Mat3A
function Mat3A:transpose(_self) end

---@param _self Mat3A 
---@param rhs Vec3 
---@return Vec3
function Mat3A:mul_vec3(_self,rhs) end

---@param x_axis Vec3A 
---@param y_axis Vec3A 
---@param z_axis Vec3A 
---@return Mat3A
function Mat3A.from_cols(x_axis,y_axis,z_axis) end

---@param order EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return Mat3A
function Mat3A.from_euler(order,a,b,c) end

---@param _self Mat3A 
---@param rhs Mat3A 
---@return Mat3A
function Mat3A:sub_mat3(_self,rhs) end

---@param angle number 
---@return Mat3A
function Mat3A.from_rotation_y(angle) end

---@param _self Mat3A 
---@return Mat3A
function Mat3A:clone(_self) end

---@param _self Mat3A 
---@param index integer 
---@return Vec3A
function Mat3A:col(_self,index) end

---@param m Mat2 
---@return Mat3A
function Mat3A.from_mat2(m) end

---@param _self Mat3A 
---@param rhs number 
---@return Mat3A
function Mat3A:div_scalar(_self,rhs) end

---@param _self Mat3A 
---@return Mat3A
function Mat3A:abs(_self) end

---@param translation Vec2 
---@return Mat3A
function Mat3A.from_translation(translation) end

---@param m Mat4 
---@param i integer 
---@param j integer 
---@return Mat3A
function Mat3A.from_mat4_minor(m,i,j) end


---@class Mat4 : ReflectReference
---@field  x_axis ? Vec4
---@field  y_axis ? Vec4
---@field  z_axis ? Vec4
---@field  w_axis ? Vec4
Mat4 = {}

---@param eye Vec3 
---@param dir Vec3 
---@param up Vec3 
---@return Mat4
function Mat4.look_to_rh(eye,dir,up) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return Mat4
function Mat4.orthographic_rh(left,right,bottom,top,near,far) end

---@param diagonal Vec4 
---@return Mat4
function Mat4.from_diagonal(diagonal) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return Mat4
function Mat4.perspective_rh(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param angle number 
---@return Mat4
function Mat4.from_rotation_x(angle) end

---@param eye Vec3 
---@param dir Vec3 
---@param up Vec3 
---@return Mat4
function Mat4.look_to_lh(eye,dir,up) end

---@param _self Mat4 
---@param order EulerRot 
---@return [number, number, number]
function Mat4:to_euler(_self,order) end

---@param _self Mat4 
---@param rhs Mat4 
---@return Mat4
function Mat4:add(_self,rhs) end

---@param _self Mat4 
---@param index integer 
---@return Vec4
function Mat4:col(_self,index) end

---@param p1 Mat4 
---@param p2 number 
---@return Mat4
function Mat4:mul(p1,p2) end

---@param _self Mat4 
---@param rhs Vec3A 
---@return Vec3A
function Mat4:transform_vector3a(_self,rhs) end

---@param _self Mat4 
---@param rhs Mat4 
---@return Mat4
function Mat4:sub_mat4(_self,rhs) end

---@param rotation Quat 
---@param translation Vec3 
---@return Mat4
function Mat4.from_rotation_translation(rotation,translation) end

---@param _self Mat4 
---@param rhs Vec3 
---@return Vec3
function Mat4:project_point3(_self,rhs) end

---@param _self Mat4 
---@param rhs number 
---@return Mat4
function Mat4:div_scalar(_self,rhs) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return Mat4
function Mat4.perspective_infinite_lh(fov_y_radians,aspect_ratio,z_near) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return Mat4
function Mat4.orthographic_lh(left,right,bottom,top,near,far) end

---@param left number 
---@param right number 
---@param bottom number 
---@param top number 
---@param near number 
---@param far number 
---@return Mat4
function Mat4.orthographic_rh_gl(left,right,bottom,top,near,far) end

---@param _self Mat4 
---@return Mat4
function Mat4:neg(_self) end

---@param scale Vec3 
---@return Mat4
function Mat4.from_scale(scale) end

---@param _self Mat4 
---@return Mat4
function Mat4:transpose(_self) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return Mat4
function Mat4.perspective_rh_gl(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param _self Mat4 
---@param rhs Vec3A 
---@return Vec3A
function Mat4:project_point3a(_self,rhs) end

---@param translation Vec3 
---@return Mat4
function Mat4.from_translation(translation) end

---@param _self Mat4 
---@param rhs Mat4 
---@return boolean
function Mat4:eq(_self,rhs) end

---@param m Mat3 
---@return Mat4
function Mat4.from_mat3(m) end

---@param _self Mat4 
---@param rhs Vec3A 
---@return Vec3A
function Mat4:transform_point3a(_self,rhs) end

---@param m Mat3A 
---@return Mat4
function Mat4.from_mat3a(m) end

---@param order EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return Mat4
function Mat4.from_euler(order,a,b,c) end

---@param _self Mat4 
---@param rhs number 
---@return Mat4
function Mat4:mul_scalar(_self,rhs) end

---@param _self Mat4 
---@param rhs Mat4 
---@return Mat4
function Mat4:add_mat4(_self,rhs) end

---@param p1 Mat4 
---@param p2 Vec4 
---@return Vec4
function Mat4:mul(p1,p2) end

---@param _self Mat4 
---@return boolean
function Mat4:is_nan(_self) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return Mat4
function Mat4.perspective_infinite_reverse_rh(fov_y_radians,aspect_ratio,z_near) end

---@param _self Mat4 
---@param rhs Affine3A 
---@return Mat4
function Mat4:mul(_self,rhs) end

---@param _self Mat4 
---@param rhs Mat4 
---@return Mat4
function Mat4:sub(_self,rhs) end

---@param _self Mat4 
---@return Mat4
function Mat4:clone(_self) end

---@param _self Mat4 
---@param index integer 
---@return Vec4
function Mat4:row(_self,index) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@param z_far number 
---@return Mat4
function Mat4.perspective_lh(fov_y_radians,aspect_ratio,z_near,z_far) end

---@param _self Mat4 
---@param rhs Vec4 
---@return Vec4
function Mat4:mul_vec4(_self,rhs) end

---@param rotation Quat 
---@return Mat4
function Mat4.from_quat(rotation) end

---@param _self Mat4 
---@return boolean
function Mat4:is_finite(_self) end

---@param _self Mat4 
---@return Mat4
function Mat4:inverse(_self) end

---@param angle number 
---@return Mat4
function Mat4.from_rotation_z(angle) end

---@param _self Mat4 
---@return number
function Mat4:determinant(_self) end

---@param eye Vec3 
---@param center Vec3 
---@param up Vec3 
---@return Mat4
function Mat4.look_at_rh(eye,center,up) end

---@param _self Mat4 
---@return number[]
function Mat4:to_cols_array(_self) end

---@param p1 Mat4 
---@param p2 Mat4 
---@return Mat4
function Mat4:mul(p1,p2) end

---@param _self Mat4 
---@param rhs Mat4 
---@param max_abs_diff number 
---@return boolean
function Mat4:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Mat4 
---@param rhs Vec3 
---@return Vec3
function Mat4:transform_vector3(_self,rhs) end

---@param x_axis Vec4 
---@param y_axis Vec4 
---@param z_axis Vec4 
---@param w_axis Vec4 
---@return Mat4
function Mat4.from_cols(x_axis,y_axis,z_axis,w_axis) end

---@param scale Vec3 
---@param rotation Quat 
---@param translation Vec3 
---@return Mat4
function Mat4.from_scale_rotation_translation(scale,rotation,translation) end

---@param _self Mat4 
---@return number[][]
function Mat4:to_cols_array_2d(_self) end

---@param _self Mat4 
---@return DMat4
function Mat4:as_dmat4(_self) end

---@param _self Mat4 
---@param rhs Vec3 
---@return Vec3
function Mat4:transform_point3(_self,rhs) end

---@param axis Vec3 
---@param angle number 
---@return Mat4
function Mat4.from_axis_angle(axis,angle) end

---@param eye Vec3 
---@param center Vec3 
---@param up Vec3 
---@return Mat4
function Mat4.look_at_lh(eye,center,up) end

---@param _self Mat4 
---@param rhs Mat4 
---@return Mat4
function Mat4:mul_mat4(_self,rhs) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return Mat4
function Mat4.perspective_infinite_rh(fov_y_radians,aspect_ratio,z_near) end

---@param fov_y_radians number 
---@param aspect_ratio number 
---@param z_near number 
---@return Mat4
function Mat4.perspective_infinite_reverse_lh(fov_y_radians,aspect_ratio,z_near) end

---@param _self Mat4 
---@param rhs number 
---@return Mat4
function Mat4:div(_self,rhs) end

---@param _self Mat4 
---@return Mat4
function Mat4:abs(_self) end

---@param angle number 
---@return Mat4
function Mat4.from_rotation_y(angle) end


---@class Quat : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number
Quat = {}

---@param _self Quat 
---@param rhs Quat 
---@return Quat
function Quat:sub(_self,rhs) end

---@param p1 Quat 
---@param p2 number 
---@return Quat
function Quat:mul(p1,p2) end

---@param _self Quat 
---@return boolean
function Quat:is_normalized(_self) end

---@param _self Quat 
---@return Quat
function Quat:conjugate(_self) end

---@param euler EulerRot 
---@param a number 
---@param b number 
---@param c number 
---@return Quat
function Quat.from_euler(euler,a,b,c) end

---@param _self Quat 
---@return Quat
function Quat:inverse(_self) end

---@param _self Quat 
---@return DQuat
function Quat:as_dquat(_self) end

---@param angle number 
---@return Quat
function Quat.from_rotation_y(angle) end

---@param _self Quat 
---@return number[]
function Quat:to_array(_self) end

---@param _self Quat 
---@return boolean
function Quat:is_nan(_self) end

---@param _self Quat 
---@param rhs Quat 
---@return Quat
function Quat:add(_self,rhs) end

---@param x number 
---@param y number 
---@param z number 
---@param w number 
---@return Quat
function Quat.from_xyzw(x,y,z,w) end

---@param angle number 
---@return Quat
function Quat.from_rotation_z(angle) end

---@param _self Quat 
---@param rhs Quat 
---@return number
function Quat:angle_between(_self,rhs) end

---@param _self Quat 
---@param _end Quat 
---@param s number 
---@return Quat
function Quat:slerp(_self,_end,s) end

---@param from Vec3 
---@param to Vec3 
---@return Quat
function Quat.from_rotation_arc_colinear(from,to) end

---@param _self Quat 
---@return Quat
function Quat:neg(_self) end

---@param _self Quat 
---@param rhs Quat 
---@param max_angle number 
---@return Quat
function Quat:rotate_towards(_self,rhs,max_angle) end

---@param angle number 
---@return Quat
function Quat.from_rotation_x(angle) end

---@param p1 Quat 
---@param p2 Vec3 
---@return Vec3
function Quat:mul(p1,p2) end

---@param _self Quat 
---@param rhs Vec3 
---@return Vec3
function Quat:mul_vec3(_self,rhs) end

---@param mat Mat3A 
---@return Quat
function Quat.from_mat3a(mat) end

---@param _self Quat 
---@return Vec3
function Quat:to_scaled_axis(_self) end

---@param p1 Quat 
---@param p2 Vec3A 
---@return Vec3A
function Quat:mul(p1,p2) end

---@param from Vec2 
---@param to Vec2 
---@return Quat
function Quat.from_rotation_arc_2d(from,to) end

---@param _self Quat 
---@param rhs Quat 
---@return Quat
function Quat:mul(_self,rhs) end

---@param _self Quat 
---@param rhs number 
---@return Quat
function Quat:div(_self,rhs) end

---@param _self Quat 
---@param rhs Quat 
---@return number
function Quat:dot(_self,rhs) end

---@param from Vec3 
---@param to Vec3 
---@return Quat
function Quat.from_rotation_arc(from,to) end

---@param _self Quat 
---@param rhs Quat 
---@return boolean
function Quat:eq(_self,rhs) end

---@param _self Quat 
---@param order EulerRot 
---@return [number, number, number]
function Quat:to_euler(_self,order) end

---@param _self Quat 
---@param _end Quat 
---@param s number 
---@return Quat
function Quat:lerp(_self,_end,s) end

---@param _self Quat 
---@param rhs Quat 
---@param max_abs_diff number 
---@return boolean
function Quat:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Quat 
---@return number
function Quat:length_squared(_self) end

---@param _self Quat 
---@param rhs Vec3A 
---@return Vec3A
function Quat:mul_vec3a(_self,rhs) end

---@param a number[] 
---@return Quat
function Quat.from_array(a) end

---@param v Vec4 
---@return Quat
function Quat.from_vec4(v) end

---@param a Affine3A 
---@return Quat
function Quat.from_affine3(a) end

---@param _self Quat 
---@return Quat
function Quat:clone(_self) end

---@param _self Quat 
---@return Vec3
function Quat:xyz(_self) end

---@param mat Mat4 
---@return Quat
function Quat.from_mat4(mat) end

---@param axis Vec3 
---@param angle number 
---@return Quat
function Quat.from_axis_angle(axis,angle) end

---@param _self Quat 
---@return Quat
function Quat:normalize(_self) end

---@param _self Quat 
---@return boolean
function Quat:is_finite(_self) end

---@param _self Quat 
---@return boolean
function Quat:is_near_identity(_self) end

---@param mat Mat3 
---@return Quat
function Quat.from_mat3(mat) end

---@param _self Quat 
---@return number
function Quat:length(_self) end

---@param v Vec3 
---@return Quat
function Quat.from_scaled_axis(v) end

---@param _self Quat 
---@param rhs Quat 
---@return Quat
function Quat:mul_quat(_self,rhs) end

---@param _self Quat 
---@return number
function Quat:length_recip(_self) end


---@class U16Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
U16Vec2 = {}

---@param _self U16Vec2 
---@return integer[]
function U16Vec2:to_array(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:min(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmplt(_self,rhs) end

---@param p1 U16Vec2 
---@param p2 U16Vec2 
---@return U16Vec2
function U16Vec2:div(p1,p2) end

---@param _self U16Vec2 
---@return integer
function U16Vec2:min_element(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:wrapping_mul(_self,rhs) end

---@param mask BVec2 
---@param if_true U16Vec2 
---@param if_false U16Vec2 
---@return U16Vec2
function U16Vec2.select(mask,if_true,if_false) end

---@param p1 U16Vec2 
---@param p2 integer 
---@return U16Vec2
function U16Vec2:rem(p1,p2) end

---@param p1 U16Vec2 
---@param p2 U16Vec2 
---@return U16Vec2
function U16Vec2:rem(p1,p2) end

---@param p1 U16Vec2 
---@param p2 integer 
---@return U16Vec2
function U16Vec2:div(p1,p2) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:max(_self,rhs) end

---@param _self U16Vec2 
---@return nil
function U16Vec2:assert_receiver_is_total_eq(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:saturating_add(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:dot_into_vec(_self,rhs) end

---@param p1 U16Vec2 
---@param p2 integer 
---@return U16Vec2
function U16Vec2:mul(p1,p2) end

---@param _self U16Vec2 
---@return U64Vec2
function U16Vec2:as_u64vec2(_self) end

---@param _self U16Vec2 
---@return DVec2
function U16Vec2:as_dvec2(_self) end

---@param _self U16Vec2 
---@return I64Vec2
function U16Vec2:as_i64vec2(_self) end

---@param p1 U16Vec2 
---@param p2 U16Vec2 
---@return U16Vec2
function U16Vec2:sub(p1,p2) end

---@param _self U16Vec2 
---@return U8Vec2
function U16Vec2:as_u8vec2(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmple(_self,rhs) end

---@param _self U16Vec2 
---@return U16Vec2
function U16Vec2:clone(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:div(_self,rhs) end

---@param _self U16Vec2 
---@return UVec2
function U16Vec2:as_uvec2(_self) end

---@param _self U16Vec2 
---@param rhs I16Vec2 
---@return U16Vec2
function U16Vec2:wrapping_add_signed(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmpeq(_self,rhs) end

---@param _self U16Vec2 
---@param x integer 
---@return U16Vec2
function U16Vec2:with_x(_self,x) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmpne(_self,rhs) end

---@param v integer 
---@return U16Vec2
function U16Vec2.splat(v) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmpgt(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return integer
function U16Vec2:dot(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:wrapping_sub(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:wrapping_div(_self,rhs) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:saturating_sub(_self,rhs) end

---@param p1 U16Vec2 
---@param p2 U16Vec2 
---@return U16Vec2
function U16Vec2:mul(p1,p2) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:mul(_self,rhs) end

---@param _self U16Vec2 
---@param other U16Vec2 
---@return boolean
function U16Vec2:eq(_self,other) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return BVec2
function U16Vec2:cmpge(_self,rhs) end

---@param _self U16Vec2 
---@return integer
function U16Vec2:element_product(_self) end

---@param p1 U16Vec2 
---@param p2 U16Vec2 
---@return U16Vec2
function U16Vec2:add(p1,p2) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:saturating_mul(_self,rhs) end

---@param _self U16Vec2 
---@return IVec2
function U16Vec2:as_ivec2(_self) end

---@param _self U16Vec2 
---@param rhs I16Vec2 
---@return U16Vec2
function U16Vec2:saturating_add_signed(_self,rhs) end

---@param _self U16Vec2 
---@return integer
function U16Vec2:length_squared(_self) end

---@param _self U16Vec2 
---@param z integer 
---@return U16Vec3
function U16Vec2:extend(_self,z) end

---@param _self U16Vec2 
---@return integer
function U16Vec2:element_sum(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:saturating_div(_self,rhs) end

---@param _self U16Vec2 
---@return integer
function U16Vec2:max_element(_self) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:rem(_self,rhs) end

---@param p1 U16Vec2 
---@param p2 integer 
---@return U16Vec2
function U16Vec2:add(p1,p2) end

---@param _self U16Vec2 
---@param y integer 
---@return U16Vec2
function U16Vec2:with_y(_self,y) end

---@param x integer 
---@param y integer 
---@return U16Vec2
function U16Vec2.new(x,y) end

---@param p1 U16Vec2 
---@param p2 integer 
---@return U16Vec2
function U16Vec2:sub(p1,p2) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:sub(_self,rhs) end

---@param _self U16Vec2 
---@return I8Vec2
function U16Vec2:as_i8vec2(_self) end

---@param _self U16Vec2 
---@param min U16Vec2 
---@param max U16Vec2 
---@return U16Vec2
function U16Vec2:clamp(_self,min,max) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:wrapping_add(_self,rhs) end

---@param _self U16Vec2 
---@return Vec2
function U16Vec2:as_vec2(_self) end

---@param _self U16Vec2 
---@return I16Vec2
function U16Vec2:as_i16vec2(_self) end

---@param a integer[] 
---@return U16Vec2
function U16Vec2.from_array(a) end

---@param _self U16Vec2 
---@param rhs U16Vec2 
---@return U16Vec2
function U16Vec2:add(_self,rhs) end


---@class U16Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
U16Vec3 = {}

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:add(_self,rhs) end

---@param p1 U16Vec3 
---@param p2 U16Vec3 
---@return U16Vec3
function U16Vec3:mul(p1,p2) end

---@param _self U16Vec3 
---@return nil
function U16Vec3:assert_receiver_is_total_eq(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:saturating_mul(_self,rhs) end

---@param _self U16Vec3 
---@return integer
function U16Vec3:element_product(_self) end

---@param _self U16Vec3 
---@return Vec3A
function U16Vec3:as_vec3a(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:wrapping_add(_self,rhs) end

---@param _self U16Vec3 
---@return integer
function U16Vec3:element_sum(_self) end

---@param _self U16Vec3 
---@return U8Vec3
function U16Vec3:as_u8vec3(_self) end

---@param p1 U16Vec3 
---@param p2 integer 
---@return U16Vec3
function U16Vec3:sub(p1,p2) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmpge(_self,rhs) end

---@param _self U16Vec3 
---@return integer
function U16Vec3:min_element(_self) end

---@param _self U16Vec3 
---@param w integer 
---@return U16Vec4
function U16Vec3:extend(_self,w) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:div(_self,rhs) end

---@param p1 U16Vec3 
---@param p2 U16Vec3 
---@return U16Vec3
function U16Vec3:rem(p1,p2) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:max(_self,rhs) end

---@param _self U16Vec3 
---@return I16Vec3
function U16Vec3:as_i16vec3(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:dot_into_vec(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return integer
function U16Vec3:dot(_self,rhs) end

---@param p1 U16Vec3 
---@param p2 integer 
---@return U16Vec3
function U16Vec3:mul(p1,p2) end

---@param _self U16Vec3 
---@return integer
function U16Vec3:length_squared(_self) end

---@param _self U16Vec3 
---@return U64Vec3
function U16Vec3:as_u64vec3(_self) end

---@param _self U16Vec3 
---@return U16Vec3
function U16Vec3:clone(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:saturating_add(_self,rhs) end

---@param _self U16Vec3 
---@return U16Vec2
function U16Vec3:truncate(_self) end

---@param p1 U16Vec3 
---@param p2 U16Vec3 
---@return U16Vec3
function U16Vec3:sub(p1,p2) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmpeq(_self,rhs) end

---@param a integer[] 
---@return U16Vec3
function U16Vec3.from_array(a) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:wrapping_div(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:saturating_div(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmple(_self,rhs) end

---@param _self U16Vec3 
---@param rhs I16Vec3 
---@return U16Vec3
function U16Vec3:saturating_add_signed(_self,rhs) end

---@param p1 U16Vec3 
---@param p2 integer 
---@return U16Vec3
function U16Vec3:rem(p1,p2) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:mul(_self,rhs) end

---@param _self U16Vec3 
---@return IVec3
function U16Vec3:as_ivec3(_self) end

---@param _self U16Vec3 
---@return I8Vec3
function U16Vec3:as_i8vec3(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:sub(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmplt(_self,rhs) end

---@param _self U16Vec3 
---@param other U16Vec3 
---@return boolean
function U16Vec3:eq(_self,other) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:rem(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:min(_self,rhs) end

---@param _self U16Vec3 
---@return DVec3
function U16Vec3:as_dvec3(_self) end

---@param p1 U16Vec3 
---@param p2 integer 
---@return U16Vec3
function U16Vec3:div(p1,p2) end

---@param _self U16Vec3 
---@return integer
function U16Vec3:max_element(_self) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return U16Vec3
function U16Vec3.new(x,y,z) end

---@param _self U16Vec3 
---@param y integer 
---@return U16Vec3
function U16Vec3:with_y(_self,y) end

---@param p1 U16Vec3 
---@param p2 U16Vec3 
---@return U16Vec3
function U16Vec3:add(p1,p2) end

---@param v integer 
---@return U16Vec3
function U16Vec3.splat(v) end

---@param p1 U16Vec3 
---@param p2 U16Vec3 
---@return U16Vec3
function U16Vec3:div(p1,p2) end

---@param mask BVec3 
---@param if_true U16Vec3 
---@param if_false U16Vec3 
---@return U16Vec3
function U16Vec3.select(mask,if_true,if_false) end

---@param _self U16Vec3 
---@param x integer 
---@return U16Vec3
function U16Vec3:with_x(_self,x) end

---@param _self U16Vec3 
---@param min U16Vec3 
---@param max U16Vec3 
---@return U16Vec3
function U16Vec3:clamp(_self,min,max) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:wrapping_sub(_self,rhs) end

---@param _self U16Vec3 
---@return integer[]
function U16Vec3:to_array(_self) end

---@param _self U16Vec3 
---@return Vec3
function U16Vec3:as_vec3(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmpgt(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:cross(_self,rhs) end

---@param p1 U16Vec3 
---@param p2 integer 
---@return U16Vec3
function U16Vec3:add(p1,p2) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:saturating_sub(_self,rhs) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return BVec3
function U16Vec3:cmpne(_self,rhs) end

---@param _self U16Vec3 
---@return UVec3
function U16Vec3:as_uvec3(_self) end

---@param _self U16Vec3 
---@param rhs I16Vec3 
---@return U16Vec3
function U16Vec3:wrapping_add_signed(_self,rhs) end

---@param _self U16Vec3 
---@return I64Vec3
function U16Vec3:as_i64vec3(_self) end

---@param _self U16Vec3 
---@param rhs U16Vec3 
---@return U16Vec3
function U16Vec3:wrapping_mul(_self,rhs) end

---@param _self U16Vec3 
---@param z integer 
---@return U16Vec3
function U16Vec3:with_z(_self,z) end


---@class U16Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
U16Vec4 = {}

---@param p1 U16Vec4 
---@param p2 U16Vec4 
---@return U16Vec4
function U16Vec4:sub(p1,p2) end

---@param _self U16Vec4 
---@param y integer 
---@return U16Vec4
function U16Vec4:with_y(_self,y) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:add(_self,rhs) end

---@param _self U16Vec4 
---@return integer
function U16Vec4:element_product(_self) end

---@param _self U16Vec4 
---@param w integer 
---@return U16Vec4
function U16Vec4:with_w(_self,w) end

---@param _self U16Vec4 
---@return I8Vec4
function U16Vec4:as_i8vec4(_self) end

---@param _self U16Vec4 
---@param rhs I16Vec4 
---@return U16Vec4
function U16Vec4:saturating_add_signed(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmpeq(_self,rhs) end

---@param _self U16Vec4 
---@return integer
function U16Vec4:min_element(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmplt(_self,rhs) end

---@param _self U16Vec4 
---@return integer[]
function U16Vec4:to_array(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:saturating_mul(_self,rhs) end

---@param _self U16Vec4 
---@return U64Vec4
function U16Vec4:as_u64vec4(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:wrapping_mul(_self,rhs) end

---@param mask BVec4 
---@param if_true U16Vec4 
---@param if_false U16Vec4 
---@return U16Vec4
function U16Vec4.select(mask,if_true,if_false) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmpge(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:wrapping_sub(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:saturating_sub(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:wrapping_div(_self,rhs) end

---@param _self U16Vec4 
---@return integer
function U16Vec4:length_squared(_self) end

---@param _self U16Vec4 
---@return U8Vec4
function U16Vec4:as_u8vec4(_self) end

---@param p1 U16Vec4 
---@param p2 integer 
---@return U16Vec4
function U16Vec4:div(p1,p2) end

---@param _self U16Vec4 
---@return I16Vec4
function U16Vec4:as_i16vec4(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return integer
function U16Vec4:dot(_self,rhs) end

---@param _self U16Vec4 
---@return U16Vec3
function U16Vec4:truncate(_self) end

---@param p1 U16Vec4 
---@param p2 U16Vec4 
---@return U16Vec4
function U16Vec4:mul(p1,p2) end

---@param p1 U16Vec4 
---@param p2 integer 
---@return U16Vec4
function U16Vec4:mul(p1,p2) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmpne(_self,rhs) end

---@param _self U16Vec4 
---@param min U16Vec4 
---@param max U16Vec4 
---@return U16Vec4
function U16Vec4:clamp(_self,min,max) end

---@param v integer 
---@return U16Vec4
function U16Vec4.splat(v) end

---@param _self U16Vec4 
---@param rhs I16Vec4 
---@return U16Vec4
function U16Vec4:wrapping_add_signed(_self,rhs) end

---@param _self U16Vec4 
---@return IVec4
function U16Vec4:as_ivec4(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:dot_into_vec(_self,rhs) end

---@param _self U16Vec4 
---@return integer
function U16Vec4:element_sum(_self) end

---@param _self U16Vec4 
---@param x integer 
---@return U16Vec4
function U16Vec4:with_x(_self,x) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:div(_self,rhs) end

---@param _self U16Vec4 
---@return UVec4
function U16Vec4:as_uvec4(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmple(_self,rhs) end

---@param _self U16Vec4 
---@return Vec4
function U16Vec4:as_vec4(_self) end

---@param _self U16Vec4 
---@return U16Vec4
function U16Vec4:clone(_self) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:mul(_self,rhs) end

---@param _self U16Vec4 
---@param z integer 
---@return U16Vec4
function U16Vec4:with_z(_self,z) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:saturating_div(_self,rhs) end

---@param _self U16Vec4 
---@return I64Vec4
function U16Vec4:as_i64vec4(_self) end

---@param p1 U16Vec4 
---@param p2 U16Vec4 
---@return U16Vec4
function U16Vec4:div(p1,p2) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:rem(_self,rhs) end

---@param p1 U16Vec4 
---@param p2 U16Vec4 
---@return U16Vec4
function U16Vec4:add(p1,p2) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return BVec4
function U16Vec4:cmpgt(_self,rhs) end

---@param p1 U16Vec4 
---@param p2 integer 
---@return U16Vec4
function U16Vec4:add(p1,p2) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return U16Vec4
function U16Vec4.new(x,y,z,w) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:saturating_add(_self,rhs) end

---@param p1 U16Vec4 
---@param p2 integer 
---@return U16Vec4
function U16Vec4:sub(p1,p2) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:max(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:min(_self,rhs) end

---@param a integer[] 
---@return U16Vec4
function U16Vec4.from_array(a) end

---@param p1 U16Vec4 
---@param p2 U16Vec4 
---@return U16Vec4
function U16Vec4:rem(p1,p2) end

---@param _self U16Vec4 
---@return nil
function U16Vec4:assert_receiver_is_total_eq(_self) end

---@param _self U16Vec4 
---@return integer
function U16Vec4:max_element(_self) end

---@param _self U16Vec4 
---@param other U16Vec4 
---@return boolean
function U16Vec4:eq(_self,other) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:sub(_self,rhs) end

---@param _self U16Vec4 
---@param rhs U16Vec4 
---@return U16Vec4
function U16Vec4:wrapping_add(_self,rhs) end

---@param _self U16Vec4 
---@return DVec4
function U16Vec4:as_dvec4(_self) end

---@param p1 U16Vec4 
---@param p2 integer 
---@return U16Vec4
function U16Vec4:rem(p1,p2) end


---@class U64Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
U64Vec2 = {}

---@param _self U64Vec2 
---@return U64Vec2
function U64Vec2:clone(_self) end

---@param _self U64Vec2 
---@param z integer 
---@return U64Vec3
function U64Vec2:extend(_self,z) end

---@param p1 U64Vec2 
---@param p2 U64Vec2 
---@return U64Vec2
function U64Vec2:add(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:wrapping_sub(_self,rhs) end

---@param p1 U64Vec2 
---@param p2 integer 
---@return U64Vec2
function U64Vec2:add(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:sub(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:min(_self,rhs) end

---@param p1 U64Vec2 
---@param p2 integer 
---@return U64Vec2
function U64Vec2:sub(p1,p2) end

---@param p1 U64Vec2 
---@param p2 U64Vec2 
---@return U64Vec2
function U64Vec2:div(p1,p2) end

---@param p1 U64Vec2 
---@param p2 integer 
---@return U64Vec2
function U64Vec2:div(p1,p2) end

---@param a integer[] 
---@return U64Vec2
function U64Vec2.from_array(a) end

---@param _self U64Vec2 
---@return I8Vec2
function U64Vec2:as_i8vec2(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmple(_self,rhs) end

---@param _self U64Vec2 
---@return DVec2
function U64Vec2:as_dvec2(_self) end

---@param p1 U64Vec2 
---@param p2 U64Vec2 
---@return U64Vec2
function U64Vec2:sub(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:rem(_self,rhs) end

---@param p1 U64Vec2 
---@param p2 U64Vec2 
---@return U64Vec2
function U64Vec2:mul(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:wrapping_mul(_self,rhs) end

---@param _self U64Vec2 
---@param rhs I64Vec2 
---@return U64Vec2
function U64Vec2:wrapping_add_signed(_self,rhs) end

---@param _self U64Vec2 
---@return nil
function U64Vec2:assert_receiver_is_total_eq(_self) end

---@param _self U64Vec2 
---@param x integer 
---@return U64Vec2
function U64Vec2:with_x(_self,x) end

---@param _self U64Vec2 
---@return integer[]
function U64Vec2:to_array(_self) end

---@param _self U64Vec2 
---@return integer
function U64Vec2:length_squared(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:max(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:div(_self,rhs) end

---@param _self U64Vec2 
---@param other U64Vec2 
---@return boolean
function U64Vec2:eq(_self,other) end

---@param _self U64Vec2 
---@param min U64Vec2 
---@param max U64Vec2 
---@return U64Vec2
function U64Vec2:clamp(_self,min,max) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmplt(_self,rhs) end

---@param _self U64Vec2 
---@return U8Vec2
function U64Vec2:as_u8vec2(_self) end

---@param p1 U64Vec2 
---@param p2 integer 
---@return U64Vec2
function U64Vec2:rem(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:wrapping_div(_self,rhs) end

---@param _self U64Vec2 
---@return Vec2
function U64Vec2:as_vec2(_self) end

---@param _self U64Vec2 
---@return integer
function U64Vec2:element_sum(_self) end

---@param _self U64Vec2 
---@return I64Vec2
function U64Vec2:as_i64vec2(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmpeq(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmpge(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:dot_into_vec(_self,rhs) end

---@param _self U64Vec2 
---@return integer
function U64Vec2:element_product(_self) end

---@param p1 U64Vec2 
---@param p2 U64Vec2 
---@return U64Vec2
function U64Vec2:rem(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:add(_self,rhs) end

---@param _self U64Vec2 
---@return UVec2
function U64Vec2:as_uvec2(_self) end

---@param _self U64Vec2 
---@return integer
function U64Vec2:min_element(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:saturating_div(_self,rhs) end

---@param _self U64Vec2 
---@return I16Vec2
function U64Vec2:as_i16vec2(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:mul(_self,rhs) end

---@param p1 U64Vec2 
---@param p2 integer 
---@return U64Vec2
function U64Vec2:mul(p1,p2) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:saturating_add(_self,rhs) end

---@param _self U64Vec2 
---@param rhs I64Vec2 
---@return U64Vec2
function U64Vec2:saturating_add_signed(_self,rhs) end

---@param mask BVec2 
---@param if_true U64Vec2 
---@param if_false U64Vec2 
---@return U64Vec2
function U64Vec2.select(mask,if_true,if_false) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmpne(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:wrapping_add(_self,rhs) end

---@param _self U64Vec2 
---@return U16Vec2
function U64Vec2:as_u16vec2(_self) end

---@param v integer 
---@return U64Vec2
function U64Vec2.splat(v) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return integer
function U64Vec2:dot(_self,rhs) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return BVec2
function U64Vec2:cmpgt(_self,rhs) end

---@param x integer 
---@param y integer 
---@return U64Vec2
function U64Vec2.new(x,y) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:saturating_sub(_self,rhs) end

---@param _self U64Vec2 
---@param y integer 
---@return U64Vec2
function U64Vec2:with_y(_self,y) end

---@param _self U64Vec2 
---@return IVec2
function U64Vec2:as_ivec2(_self) end

---@param _self U64Vec2 
---@return integer
function U64Vec2:max_element(_self) end

---@param _self U64Vec2 
---@param rhs U64Vec2 
---@return U64Vec2
function U64Vec2:saturating_mul(_self,rhs) end


---@class U64Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
U64Vec3 = {}

---@param _self U64Vec3 
---@return IVec3
function U64Vec3:as_ivec3(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:wrapping_div(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmpge(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 U64Vec3 
---@return U64Vec3
function U64Vec3:mul(p1,p2) end

---@param a integer[] 
---@return U64Vec3
function U64Vec3.from_array(a) end

---@param _self U64Vec3 
---@return I8Vec3
function U64Vec3:as_i8vec3(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:saturating_div(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:min(_self,rhs) end

---@param _self U64Vec3 
---@param w integer 
---@return U64Vec4
function U64Vec3:extend(_self,w) end

---@param _self U64Vec3 
---@return U64Vec2
function U64Vec3:truncate(_self) end

---@param _self U64Vec3 
---@return Vec3
function U64Vec3:as_vec3(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:wrapping_mul(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:add(_self,rhs) end

---@param _self U64Vec3 
---@param z integer 
---@return U64Vec3
function U64Vec3:with_z(_self,z) end

---@param p1 U64Vec3 
---@param p2 integer 
---@return U64Vec3
function U64Vec3:add(p1,p2) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:rem(_self,rhs) end

---@param _self U64Vec3 
---@param rhs I64Vec3 
---@return U64Vec3
function U64Vec3:saturating_add_signed(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:saturating_add(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:saturating_sub(_self,rhs) end

---@param _self U64Vec3 
---@return integer
function U64Vec3:max_element(_self) end

---@param p1 U64Vec3 
---@param p2 integer 
---@return U64Vec3
function U64Vec3:rem(p1,p2) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:saturating_mul(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 integer 
---@return U64Vec3
function U64Vec3:div(p1,p2) end

---@param _self U64Vec3 
---@return I16Vec3
function U64Vec3:as_i16vec3(_self) end

---@param v integer 
---@return U64Vec3
function U64Vec3.splat(v) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:wrapping_sub(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:cross(_self,rhs) end

---@param _self U64Vec3 
---@return U64Vec3
function U64Vec3:clone(_self) end

---@param _self U64Vec3 
---@return DVec3
function U64Vec3:as_dvec3(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:mul(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 integer 
---@return U64Vec3
function U64Vec3:sub(p1,p2) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return integer
function U64Vec3:dot(_self,rhs) end

---@param _self U64Vec3 
---@return U8Vec3
function U64Vec3:as_u8vec3(_self) end

---@param _self U64Vec3 
---@return I64Vec3
function U64Vec3:as_i64vec3(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmpgt(_self,rhs) end

---@param _self U64Vec3 
---@return nil
function U64Vec3:assert_receiver_is_total_eq(_self) end

---@param p1 U64Vec3 
---@param p2 U64Vec3 
---@return U64Vec3
function U64Vec3:rem(p1,p2) end

---@param _self U64Vec3 
---@return U16Vec3
function U64Vec3:as_u16vec3(_self) end

---@param _self U64Vec3 
---@param min U64Vec3 
---@param max U64Vec3 
---@return U64Vec3
function U64Vec3:clamp(_self,min,max) end

---@param _self U64Vec3 
---@param y integer 
---@return U64Vec3
function U64Vec3:with_y(_self,y) end

---@param _self U64Vec3 
---@param x integer 
---@return U64Vec3
function U64Vec3:with_x(_self,x) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmpeq(_self,rhs) end

---@param _self U64Vec3 
---@param other U64Vec3 
---@return boolean
function U64Vec3:eq(_self,other) end

---@param _self U64Vec3 
---@return integer
function U64Vec3:length_squared(_self) end

---@param _self U64Vec3 
---@return Vec3A
function U64Vec3:as_vec3a(_self) end

---@param p1 U64Vec3 
---@param p2 U64Vec3 
---@return U64Vec3
function U64Vec3:div(p1,p2) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmplt(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:div(_self,rhs) end

---@param _self U64Vec3 
---@return integer
function U64Vec3:element_product(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:dot_into_vec(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:sub(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 U64Vec3 
---@return U64Vec3
function U64Vec3:sub(p1,p2) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmpne(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 integer 
---@return U64Vec3
function U64Vec3:mul(p1,p2) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return U64Vec3
function U64Vec3.new(x,y,z) end

---@param _self U64Vec3 
---@return integer
function U64Vec3:min_element(_self) end

---@param _self U64Vec3 
---@return UVec3
function U64Vec3:as_uvec3(_self) end

---@param _self U64Vec3 
---@param rhs I64Vec3 
---@return U64Vec3
function U64Vec3:wrapping_add_signed(_self,rhs) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:max(_self,rhs) end

---@param mask BVec3 
---@param if_true U64Vec3 
---@param if_false U64Vec3 
---@return U64Vec3
function U64Vec3.select(mask,if_true,if_false) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return BVec3
function U64Vec3:cmple(_self,rhs) end

---@param p1 U64Vec3 
---@param p2 U64Vec3 
---@return U64Vec3
function U64Vec3:add(p1,p2) end

---@param _self U64Vec3 
---@return integer
function U64Vec3:element_sum(_self) end

---@param _self U64Vec3 
---@param rhs U64Vec3 
---@return U64Vec3
function U64Vec3:wrapping_add(_self,rhs) end

---@param _self U64Vec3 
---@return integer[]
function U64Vec3:to_array(_self) end


---@class U64Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
U64Vec4 = {}

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:rem(_self,rhs) end

---@param _self U64Vec4 
---@return nil
function U64Vec4:assert_receiver_is_total_eq(_self) end

---@param _self U64Vec4 
---@return UVec4
function U64Vec4:as_uvec4(_self) end

---@param _self U64Vec4 
---@return integer
function U64Vec4:max_element(_self) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:saturating_mul(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:saturating_div(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:wrapping_sub(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmpeq(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:sub(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmpge(_self,rhs) end

---@param _self U64Vec4 
---@return integer
function U64Vec4:length_squared(_self) end

---@param _self U64Vec4 
---@param z integer 
---@return U64Vec4
function U64Vec4:with_z(_self,z) end

---@param _self U64Vec4 
---@return integer
function U64Vec4:element_product(_self) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmpgt(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return U64Vec4
function U64Vec4.new(x,y,z,w) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:dot_into_vec(_self,rhs) end

---@param _self U64Vec4 
---@return DVec4
function U64Vec4:as_dvec4(_self) end

---@param p1 U64Vec4 
---@param p2 integer 
---@return U64Vec4
function U64Vec4:sub(p1,p2) end

---@param _self U64Vec4 
---@return I8Vec4
function U64Vec4:as_i8vec4(_self) end

---@param _self U64Vec4 
---@param rhs I64Vec4 
---@return U64Vec4
function U64Vec4:wrapping_add_signed(_self,rhs) end

---@param p1 U64Vec4 
---@param p2 U64Vec4 
---@return U64Vec4
function U64Vec4:sub(p1,p2) end

---@param _self U64Vec4 
---@return U64Vec3
function U64Vec4:truncate(_self) end

---@param _self U64Vec4 
---@return I64Vec4
function U64Vec4:as_i64vec4(_self) end

---@param mask BVec4 
---@param if_true U64Vec4 
---@param if_false U64Vec4 
---@return U64Vec4
function U64Vec4.select(mask,if_true,if_false) end

---@param _self U64Vec4 
---@return integer
function U64Vec4:min_element(_self) end

---@param p1 U64Vec4 
---@param p2 U64Vec4 
---@return U64Vec4
function U64Vec4:rem(p1,p2) end

---@param p1 U64Vec4 
---@param p2 U64Vec4 
---@return U64Vec4
function U64Vec4:mul(p1,p2) end

---@param _self U64Vec4 
---@return integer
function U64Vec4:element_sum(_self) end

---@param _self U64Vec4 
---@param x integer 
---@return U64Vec4
function U64Vec4:with_x(_self,x) end

---@param p1 U64Vec4 
---@param p2 integer 
---@return U64Vec4
function U64Vec4:div(p1,p2) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmple(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:saturating_add(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:div(_self,rhs) end

---@param _self U64Vec4 
---@return Vec4
function U64Vec4:as_vec4(_self) end

---@param _self U64Vec4 
---@return IVec4
function U64Vec4:as_ivec4(_self) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:wrapping_div(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:saturating_sub(_self,rhs) end

---@param _self U64Vec4 
---@return integer[]
function U64Vec4:to_array(_self) end

---@param _self U64Vec4 
---@param rhs I64Vec4 
---@return U64Vec4
function U64Vec4:saturating_add_signed(_self,rhs) end

---@param _self U64Vec4 
---@param w integer 
---@return U64Vec4
function U64Vec4:with_w(_self,w) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:min(_self,rhs) end

---@param p1 U64Vec4 
---@param p2 U64Vec4 
---@return U64Vec4
function U64Vec4:add(p1,p2) end

---@param _self U64Vec4 
---@param y integer 
---@return U64Vec4
function U64Vec4:with_y(_self,y) end

---@param a integer[] 
---@return U64Vec4
function U64Vec4.from_array(a) end

---@param _self U64Vec4 
---@return U64Vec4
function U64Vec4:clone(_self) end

---@param v integer 
---@return U64Vec4
function U64Vec4.splat(v) end

---@param _self U64Vec4 
---@return U8Vec4
function U64Vec4:as_u8vec4(_self) end

---@param p1 U64Vec4 
---@param p2 integer 
---@return U64Vec4
function U64Vec4:add(p1,p2) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:add(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmplt(_self,rhs) end

---@param _self U64Vec4 
---@param other U64Vec4 
---@return boolean
function U64Vec4:eq(_self,other) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:wrapping_mul(_self,rhs) end

---@param p1 U64Vec4 
---@param p2 U64Vec4 
---@return U64Vec4
function U64Vec4:div(p1,p2) end

---@param _self U64Vec4 
---@return I16Vec4
function U64Vec4:as_i16vec4(_self) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:wrapping_add(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return integer
function U64Vec4:dot(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:max(_self,rhs) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return BVec4
function U64Vec4:cmpne(_self,rhs) end

---@param _self U64Vec4 
---@return U16Vec4
function U64Vec4:as_u16vec4(_self) end

---@param _self U64Vec4 
---@param min U64Vec4 
---@param max U64Vec4 
---@return U64Vec4
function U64Vec4:clamp(_self,min,max) end

---@param p1 U64Vec4 
---@param p2 integer 
---@return U64Vec4
function U64Vec4:rem(p1,p2) end

---@param p1 U64Vec4 
---@param p2 integer 
---@return U64Vec4
function U64Vec4:mul(p1,p2) end

---@param _self U64Vec4 
---@param rhs U64Vec4 
---@return U64Vec4
function U64Vec4:mul(_self,rhs) end


---@class U8Vec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
U8Vec2 = {}

---@param p1 U8Vec2 
---@param p2 integer 
---@return U8Vec2
function U8Vec2:sub(p1,p2) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:rem(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 integer 
---@return U8Vec2
function U8Vec2:rem(p1,p2) end

---@param _self U8Vec2 
---@param y integer 
---@return U8Vec2
function U8Vec2:with_y(_self,y) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:min(_self,rhs) end

---@param v integer 
---@return U8Vec2
function U8Vec2.splat(v) end

---@param _self U8Vec2 
---@param x integer 
---@return U8Vec2
function U8Vec2:with_x(_self,x) end

---@param _self U8Vec2 
---@param rhs I8Vec2 
---@return U8Vec2
function U8Vec2:saturating_add_signed(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:saturating_sub(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 U8Vec2 
---@return U8Vec2
function U8Vec2:add(p1,p2) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmpne(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 U8Vec2 
---@return U8Vec2
function U8Vec2:mul(p1,p2) end

---@param _self U8Vec2 
---@param min U8Vec2 
---@param max U8Vec2 
---@return U8Vec2
function U8Vec2:clamp(_self,min,max) end

---@param _self U8Vec2 
---@param other U8Vec2 
---@return boolean
function U8Vec2:eq(_self,other) end

---@param _self U8Vec2 
---@return U8Vec2
function U8Vec2:clone(_self) end

---@param _self U8Vec2 
---@return UVec2
function U8Vec2:as_uvec2(_self) end

---@param _self U8Vec2 
---@return integer
function U8Vec2:min_element(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:saturating_mul(_self,rhs) end

---@param a integer[] 
---@return U8Vec2
function U8Vec2.from_array(a) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:wrapping_sub(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmpgt(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 integer 
---@return U8Vec2
function U8Vec2:add(p1,p2) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:saturating_add(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 integer 
---@return U8Vec2
function U8Vec2:mul(p1,p2) end

---@param _self U8Vec2 
---@return nil
function U8Vec2:assert_receiver_is_total_eq(_self) end

---@param _self U8Vec2 
---@param z integer 
---@return U8Vec3
function U8Vec2:extend(_self,z) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmpge(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:saturating_div(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:wrapping_mul(_self,rhs) end

---@param _self U8Vec2 
---@return integer
function U8Vec2:element_product(_self) end

---@param _self U8Vec2 
---@return I16Vec2
function U8Vec2:as_i16vec2(_self) end

---@param p1 U8Vec2 
---@param p2 integer 
---@return U8Vec2
function U8Vec2:div(p1,p2) end

---@param _self U8Vec2 
---@return IVec2
function U8Vec2:as_ivec2(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return integer
function U8Vec2:dot(_self,rhs) end

---@param x integer 
---@param y integer 
---@return U8Vec2
function U8Vec2.new(x,y) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:sub(_self,rhs) end

---@param mask BVec2 
---@param if_true U8Vec2 
---@param if_false U8Vec2 
---@return U8Vec2
function U8Vec2.select(mask,if_true,if_false) end

---@param _self U8Vec2 
---@return integer[]
function U8Vec2:to_array(_self) end

---@param _self U8Vec2 
---@return integer
function U8Vec2:max_element(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:wrapping_add(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:mul(_self,rhs) end

---@param _self U8Vec2 
---@return Vec2
function U8Vec2:as_vec2(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:wrapping_div(_self,rhs) end

---@param _self U8Vec2 
---@return I64Vec2
function U8Vec2:as_i64vec2(_self) end

---@param _self U8Vec2 
---@return DVec2
function U8Vec2:as_dvec2(_self) end

---@param _self U8Vec2 
---@return integer
function U8Vec2:length_squared(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:max(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmpeq(_self,rhs) end

---@param _self U8Vec2 
---@return U16Vec2
function U8Vec2:as_u16vec2(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:div(_self,rhs) end

---@param p1 U8Vec2 
---@param p2 U8Vec2 
---@return U8Vec2
function U8Vec2:rem(p1,p2) end

---@param p1 U8Vec2 
---@param p2 U8Vec2 
---@return U8Vec2
function U8Vec2:div(p1,p2) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:add(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmple(_self,rhs) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return BVec2
function U8Vec2:cmplt(_self,rhs) end

---@param _self U8Vec2 
---@return integer
function U8Vec2:element_sum(_self) end

---@param _self U8Vec2 
---@return I8Vec2
function U8Vec2:as_i8vec2(_self) end

---@param _self U8Vec2 
---@param rhs U8Vec2 
---@return U8Vec2
function U8Vec2:dot_into_vec(_self,rhs) end

---@param _self U8Vec2 
---@param rhs I8Vec2 
---@return U8Vec2
function U8Vec2:wrapping_add_signed(_self,rhs) end

---@param _self U8Vec2 
---@return U64Vec2
function U8Vec2:as_u64vec2(_self) end

---@param p1 U8Vec2 
---@param p2 U8Vec2 
---@return U8Vec2
function U8Vec2:sub(p1,p2) end


---@class U8Vec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
U8Vec3 = {}

---@param p1 U8Vec3 
---@param p2 integer 
---@return U8Vec3
function U8Vec3:mul(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmpne(_self,rhs) end

---@param _self U8Vec3 
---@return IVec3
function U8Vec3:as_ivec3(_self) end

---@param _self U8Vec3 
---@return integer
function U8Vec3:element_product(_self) end

---@param _self U8Vec3 
---@return integer[]
function U8Vec3:to_array(_self) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmpgt(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:saturating_sub(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:saturating_mul(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:mul(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return integer
function U8Vec3:dot(_self,rhs) end

---@param _self U8Vec3 
---@return UVec3
function U8Vec3:as_uvec3(_self) end

---@param p1 U8Vec3 
---@param p2 integer 
---@return U8Vec3
function U8Vec3:add(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:add(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:rem(_self,rhs) end

---@param _self U8Vec3 
---@return integer
function U8Vec3:element_sum(_self) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:saturating_add(_self,rhs) end

---@param _self U8Vec3 
---@param y integer 
---@return U8Vec3
function U8Vec3:with_y(_self,y) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:cross(_self,rhs) end

---@param _self U8Vec3 
---@return Vec3A
function U8Vec3:as_vec3a(_self) end

---@param _self U8Vec3 
---@param x integer 
---@return U8Vec3
function U8Vec3:with_x(_self,x) end

---@param p1 U8Vec3 
---@param p2 U8Vec3 
---@return U8Vec3
function U8Vec3:mul(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:div(_self,rhs) end

---@param _self U8Vec3 
---@return U8Vec3
function U8Vec3:clone(_self) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:wrapping_add(_self,rhs) end

---@param _self U8Vec3 
---@return integer
function U8Vec3:min_element(_self) end

---@param _self U8Vec3 
---@return DVec3
function U8Vec3:as_dvec3(_self) end

---@param _self U8Vec3 
---@param z integer 
---@return U8Vec3
function U8Vec3:with_z(_self,z) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return U8Vec3
function U8Vec3.new(x,y,z) end

---@param p1 U8Vec3 
---@param p2 integer 
---@return U8Vec3
function U8Vec3:rem(p1,p2) end

---@param _self U8Vec3 
---@return I8Vec3
function U8Vec3:as_i8vec3(_self) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:wrapping_mul(_self,rhs) end

---@param _self U8Vec3 
---@return Vec3
function U8Vec3:as_vec3(_self) end

---@param _self U8Vec3 
---@return U16Vec3
function U8Vec3:as_u16vec3(_self) end

---@param _self U8Vec3 
---@return integer
function U8Vec3:max_element(_self) end

---@param _self U8Vec3 
---@param rhs I8Vec3 
---@return U8Vec3
function U8Vec3:wrapping_add_signed(_self,rhs) end

---@param _self U8Vec3 
---@return U8Vec2
function U8Vec3:truncate(_self) end

---@param mask BVec3 
---@param if_true U8Vec3 
---@param if_false U8Vec3 
---@return U8Vec3
function U8Vec3.select(mask,if_true,if_false) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:wrapping_div(_self,rhs) end

---@param p1 U8Vec3 
---@param p2 integer 
---@return U8Vec3
function U8Vec3:sub(p1,p2) end

---@param _self U8Vec3 
---@return U64Vec3
function U8Vec3:as_u64vec3(_self) end

---@param p1 U8Vec3 
---@param p2 U8Vec3 
---@return U8Vec3
function U8Vec3:add(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:saturating_div(_self,rhs) end

---@param _self U8Vec3 
---@param w integer 
---@return U8Vec4
function U8Vec3:extend(_self,w) end

---@param _self U8Vec3 
---@return integer
function U8Vec3:length_squared(_self) end

---@param _self U8Vec3 
---@return I64Vec3
function U8Vec3:as_i64vec3(_self) end

---@param _self U8Vec3 
---@return nil
function U8Vec3:assert_receiver_is_total_eq(_self) end

---@param _self U8Vec3 
---@param rhs I8Vec3 
---@return U8Vec3
function U8Vec3:saturating_add_signed(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:dot_into_vec(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmplt(_self,rhs) end

---@param _self U8Vec3 
---@return I16Vec3
function U8Vec3:as_i16vec3(_self) end

---@param p1 U8Vec3 
---@param p2 U8Vec3 
---@return U8Vec3
function U8Vec3:rem(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:wrapping_sub(_self,rhs) end

---@param _self U8Vec3 
---@param other U8Vec3 
---@return boolean
function U8Vec3:eq(_self,other) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmpge(_self,rhs) end

---@param p1 U8Vec3 
---@param p2 integer 
---@return U8Vec3
function U8Vec3:div(p1,p2) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:max(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmple(_self,rhs) end

---@param v integer 
---@return U8Vec3
function U8Vec3.splat(v) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return BVec3
function U8Vec3:cmpeq(_self,rhs) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:min(_self,rhs) end

---@param a integer[] 
---@return U8Vec3
function U8Vec3.from_array(a) end

---@param _self U8Vec3 
---@param rhs U8Vec3 
---@return U8Vec3
function U8Vec3:sub(_self,rhs) end

---@param _self U8Vec3 
---@param min U8Vec3 
---@param max U8Vec3 
---@return U8Vec3
function U8Vec3:clamp(_self,min,max) end

---@param p1 U8Vec3 
---@param p2 U8Vec3 
---@return U8Vec3
function U8Vec3:div(p1,p2) end

---@param p1 U8Vec3 
---@param p2 U8Vec3 
---@return U8Vec3
function U8Vec3:sub(p1,p2) end


---@class U8Vec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
U8Vec4 = {}

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:max(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:saturating_mul(_self,rhs) end

---@param _self U8Vec4 
---@return U16Vec4
function U8Vec4:as_u16vec4(_self) end

---@param _self U8Vec4 
---@param other U8Vec4 
---@return boolean
function U8Vec4:eq(_self,other) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:add(_self,rhs) end

---@param _self U8Vec4 
---@param rhs I8Vec4 
---@return U8Vec4
function U8Vec4:wrapping_add_signed(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:dot_into_vec(_self,rhs) end

---@param p1 U8Vec4 
---@param p2 U8Vec4 
---@return U8Vec4
function U8Vec4:sub(p1,p2) end

---@param _self U8Vec4 
---@return I8Vec4
function U8Vec4:as_i8vec4(_self) end

---@param _self U8Vec4 
---@return U8Vec3
function U8Vec4:truncate(_self) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:div(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return U8Vec4
function U8Vec4.new(x,y,z,w) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmpeq(_self,rhs) end

---@param _self U8Vec4 
---@return integer[]
function U8Vec4:to_array(_self) end

---@param p1 U8Vec4 
---@param p2 U8Vec4 
---@return U8Vec4
function U8Vec4:add(p1,p2) end

---@param _self U8Vec4 
---@param x integer 
---@return U8Vec4
function U8Vec4:with_x(_self,x) end

---@param _self U8Vec4 
---@param rhs I8Vec4 
---@return U8Vec4
function U8Vec4:saturating_add_signed(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmpgt(_self,rhs) end

---@param a integer[] 
---@return U8Vec4
function U8Vec4.from_array(a) end

---@param p1 U8Vec4 
---@param p2 integer 
---@return U8Vec4
function U8Vec4:sub(p1,p2) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:saturating_sub(_self,rhs) end

---@param _self U8Vec4 
---@return IVec4
function U8Vec4:as_ivec4(_self) end

---@param p1 U8Vec4 
---@param p2 integer 
---@return U8Vec4
function U8Vec4:add(p1,p2) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:wrapping_add(_self,rhs) end

---@param _self U8Vec4 
---@param min U8Vec4 
---@param max U8Vec4 
---@return U8Vec4
function U8Vec4:clamp(_self,min,max) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:wrapping_mul(_self,rhs) end

---@param _self U8Vec4 
---@param z integer 
---@return U8Vec4
function U8Vec4:with_z(_self,z) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:wrapping_sub(_self,rhs) end

---@param _self U8Vec4 
---@return integer
function U8Vec4:element_product(_self) end

---@param p1 U8Vec4 
---@param p2 integer 
---@return U8Vec4
function U8Vec4:div(p1,p2) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return integer
function U8Vec4:dot(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:wrapping_div(_self,rhs) end

---@param _self U8Vec4 
---@return UVec4
function U8Vec4:as_uvec4(_self) end

---@param _self U8Vec4 
---@return integer
function U8Vec4:element_sum(_self) end

---@param v integer 
---@return U8Vec4
function U8Vec4.splat(v) end

---@param mask BVec4 
---@param if_true U8Vec4 
---@param if_false U8Vec4 
---@return U8Vec4
function U8Vec4.select(mask,if_true,if_false) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:sub(_self,rhs) end

---@param _self U8Vec4 
---@return integer
function U8Vec4:max_element(_self) end

---@param _self U8Vec4 
---@param w integer 
---@return U8Vec4
function U8Vec4:with_w(_self,w) end

---@param _self U8Vec4 
---@return integer
function U8Vec4:min_element(_self) end

---@param _self U8Vec4 
---@return nil
function U8Vec4:assert_receiver_is_total_eq(_self) end

---@param p1 U8Vec4 
---@param p2 integer 
---@return U8Vec4
function U8Vec4:rem(p1,p2) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmpge(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:mul(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmpne(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmplt(_self,rhs) end

---@param _self U8Vec4 
---@return integer
function U8Vec4:length_squared(_self) end

---@param _self U8Vec4 
---@return I64Vec4
function U8Vec4:as_i64vec4(_self) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:saturating_add(_self,rhs) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:rem(_self,rhs) end

---@param _self U8Vec4 
---@return U8Vec4
function U8Vec4:clone(_self) end

---@param p1 U8Vec4 
---@param p2 integer 
---@return U8Vec4
function U8Vec4:mul(p1,p2) end

---@param p1 U8Vec4 
---@param p2 U8Vec4 
---@return U8Vec4
function U8Vec4:rem(p1,p2) end

---@param _self U8Vec4 
---@param y integer 
---@return U8Vec4
function U8Vec4:with_y(_self,y) end

---@param p1 U8Vec4 
---@param p2 U8Vec4 
---@return U8Vec4
function U8Vec4:div(p1,p2) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:min(_self,rhs) end

---@param _self U8Vec4 
---@return DVec4
function U8Vec4:as_dvec4(_self) end

---@param p1 U8Vec4 
---@param p2 U8Vec4 
---@return U8Vec4
function U8Vec4:mul(p1,p2) end

---@param _self U8Vec4 
---@return Vec4
function U8Vec4:as_vec4(_self) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return U8Vec4
function U8Vec4:saturating_div(_self,rhs) end

---@param _self U8Vec4 
---@return I16Vec4
function U8Vec4:as_i16vec4(_self) end

---@param _self U8Vec4 
---@param rhs U8Vec4 
---@return BVec4
function U8Vec4:cmple(_self,rhs) end

---@param _self U8Vec4 
---@return U64Vec4
function U8Vec4:as_u64vec4(_self) end


---@class UVec2 : ReflectReference
---@field  x ? integer
---@field  y ? integer
UVec2 = {}

---@param p1 UVec2 
---@param p2 UVec2 
---@return UVec2
function UVec2:add(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmpeq(_self,rhs) end

---@param _self UVec2 
---@param rhs IVec2 
---@return UVec2
function UVec2:saturating_add_signed(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:mul(_self,rhs) end

---@param _self UVec2 
---@return U64Vec2
function UVec2:as_u64vec2(_self) end

---@param _self UVec2 
---@return I16Vec2
function UVec2:as_i16vec2(_self) end

---@param _self UVec2 
---@return integer
function UVec2:element_sum(_self) end

---@param _self UVec2 
---@return integer
function UVec2:length_squared(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:wrapping_add(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmplt(_self,rhs) end

---@param _self UVec2 
---@return U16Vec2
function UVec2:as_u16vec2(_self) end

---@param p1 UVec2 
---@param p2 integer 
---@return UVec2
function UVec2:add(p1,p2) end

---@param p1 UVec2 
---@param p2 integer 
---@return UVec2
function UVec2:div(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:saturating_div(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmple(_self,rhs) end

---@param v integer 
---@return UVec2
function UVec2.splat(v) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:sub(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmpgt(_self,rhs) end

---@param _self UVec2 
---@return integer[]
function UVec2:to_array(_self) end

---@param p1 UVec2 
---@param p2 integer 
---@return UVec2
function UVec2:mul(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:max(_self,rhs) end

---@param _self UVec2 
---@return integer
function UVec2:max_element(_self) end

---@param _self UVec2 
---@param rhs IVec2 
---@return UVec2
function UVec2:wrapping_add_signed(_self,rhs) end

---@param _self UVec2 
---@param y integer 
---@return UVec2
function UVec2:with_y(_self,y) end

---@param _self UVec2 
---@return IVec2
function UVec2:as_ivec2(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmpge(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:wrapping_sub(_self,rhs) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:min(_self,rhs) end

---@param p1 UVec2 
---@param p2 UVec2 
---@return UVec2
function UVec2:sub(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:div(_self,rhs) end

---@param p1 UVec2 
---@param p2 UVec2 
---@return UVec2
function UVec2:rem(p1,p2) end

---@param p1 UVec2 
---@param p2 integer 
---@return UVec2
function UVec2:sub(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:add(_self,rhs) end

---@param p1 UVec2 
---@param p2 UVec2 
---@return UVec2
function UVec2:div(p1,p2) end

---@param _self UVec2 
---@return U8Vec2
function UVec2:as_u8vec2(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:saturating_add(_self,rhs) end

---@param _self UVec2 
---@return Vec2
function UVec2:as_vec2(_self) end

---@param _self UVec2 
---@return DVec2
function UVec2:as_dvec2(_self) end

---@param _self UVec2 
---@return integer
function UVec2:element_product(_self) end

---@param p1 UVec2 
---@param p2 integer 
---@return UVec2
function UVec2:rem(p1,p2) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:wrapping_div(_self,rhs) end

---@param a integer[] 
---@return UVec2
function UVec2.from_array(a) end

---@param _self UVec2 
---@param rhs UVec2 
---@return BVec2
function UVec2:cmpne(_self,rhs) end

---@param p1 UVec2 
---@param p2 UVec2 
---@return UVec2
function UVec2:mul(p1,p2) end

---@param _self UVec2 
---@return I8Vec2
function UVec2:as_i8vec2(_self) end

---@param _self UVec2 
---@return integer
function UVec2:min_element(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:dot_into_vec(_self,rhs) end

---@param _self UVec2 
---@param other UVec2 
---@return boolean
function UVec2:eq(_self,other) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:saturating_mul(_self,rhs) end

---@param _self UVec2 
---@param z integer 
---@return UVec3
function UVec2:extend(_self,z) end

---@param _self UVec2 
---@return I64Vec2
function UVec2:as_i64vec2(_self) end

---@param mask BVec2 
---@param if_true UVec2 
---@param if_false UVec2 
---@return UVec2
function UVec2.select(mask,if_true,if_false) end

---@param _self UVec2 
---@param x integer 
---@return UVec2
function UVec2:with_x(_self,x) end

---@param _self UVec2 
---@param rhs UVec2 
---@return integer
function UVec2:dot(_self,rhs) end

---@param x integer 
---@param y integer 
---@return UVec2
function UVec2.new(x,y) end

---@param _self UVec2 
---@return nil
function UVec2:assert_receiver_is_total_eq(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:rem(_self,rhs) end

---@param _self UVec2 
---@param min UVec2 
---@param max UVec2 
---@return UVec2
function UVec2:clamp(_self,min,max) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:wrapping_mul(_self,rhs) end

---@param _self UVec2 
---@return UVec2
function UVec2:clone(_self) end

---@param _self UVec2 
---@param rhs UVec2 
---@return UVec2
function UVec2:saturating_sub(_self,rhs) end


---@class UVec3 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
UVec3 = {}

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:rem(_self,rhs) end

---@param _self UVec3 
---@param z integer 
---@return UVec3
function UVec3:with_z(_self,z) end

---@param _self UVec3 
---@return Vec3
function UVec3:as_vec3(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:saturating_div(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:saturating_sub(_self,rhs) end

---@param _self UVec3 
---@return I64Vec3
function UVec3:as_i64vec3(_self) end

---@param _self UVec3 
---@return integer
function UVec3:min_element(_self) end

---@param _self UVec3 
---@return Vec3A
function UVec3:as_vec3a(_self) end

---@param _self UVec3 
---@param min UVec3 
---@param max UVec3 
---@return UVec3
function UVec3:clamp(_self,min,max) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:saturating_mul(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmple(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:dot_into_vec(_self,rhs) end

---@param _self UVec3 
---@return DVec3
function UVec3:as_dvec3(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmplt(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:min(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:sub(_self,rhs) end

---@param _self UVec3 
---@return IVec3
function UVec3:as_ivec3(_self) end

---@param p1 UVec3 
---@param p2 integer 
---@return UVec3
function UVec3:div(p1,p2) end

---@param _self UVec3 
---@return UVec2
function UVec3:truncate(_self) end

---@param _self UVec3 
---@return I8Vec3
function UVec3:as_i8vec3(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmpgt(_self,rhs) end

---@param p1 UVec3 
---@param p2 UVec3 
---@return UVec3
function UVec3:rem(p1,p2) end

---@param _self UVec3 
---@return integer[]
function UVec3:to_array(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmpne(_self,rhs) end

---@param x integer 
---@param y integer 
---@param z integer 
---@return UVec3
function UVec3.new(x,y,z) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:max(_self,rhs) end

---@param _self UVec3 
---@return integer
function UVec3:length_squared(_self) end

---@param _self UVec3 
---@param rhs IVec3 
---@return UVec3
function UVec3:saturating_add_signed(_self,rhs) end

---@param p1 UVec3 
---@param p2 UVec3 
---@return UVec3
function UVec3:sub(p1,p2) end

---@param a integer[] 
---@return UVec3
function UVec3.from_array(a) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:cross(_self,rhs) end

---@param _self UVec3 
---@param other UVec3 
---@return boolean
function UVec3:eq(_self,other) end

---@param p1 UVec3 
---@param p2 UVec3 
---@return UVec3
function UVec3:div(p1,p2) end

---@param _self UVec3 
---@param w integer 
---@return UVec4
function UVec3:extend(_self,w) end

---@param p1 UVec3 
---@param p2 integer 
---@return UVec3
function UVec3:rem(p1,p2) end

---@param _self UVec3 
---@param x integer 
---@return UVec3
function UVec3:with_x(_self,x) end

---@param p1 UVec3 
---@param p2 integer 
---@return UVec3
function UVec3:sub(p1,p2) end

---@param _self UVec3 
---@return integer
function UVec3:max_element(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:wrapping_div(_self,rhs) end

---@param _self UVec3 
---@return U64Vec3
function UVec3:as_u64vec3(_self) end

---@param p1 UVec3 
---@param p2 integer 
---@return UVec3
function UVec3:mul(p1,p2) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:wrapping_add(_self,rhs) end

---@param v integer 
---@return UVec3
function UVec3.splat(v) end

---@param _self UVec3 
---@return U16Vec3
function UVec3:as_u16vec3(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:wrapping_sub(_self,rhs) end

---@param _self UVec3 
---@param y integer 
---@return UVec3
function UVec3:with_y(_self,y) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmpeq(_self,rhs) end

---@param _self UVec3 
---@return nil
function UVec3:assert_receiver_is_total_eq(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return integer
function UVec3:dot(_self,rhs) end

---@param mask BVec3 
---@param if_true UVec3 
---@param if_false UVec3 
---@return UVec3
function UVec3.select(mask,if_true,if_false) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:mul(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:saturating_add(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return BVec3
function UVec3:cmpge(_self,rhs) end

---@param _self UVec3 
---@return integer
function UVec3:element_sum(_self) end

---@param p1 UVec3 
---@param p2 UVec3 
---@return UVec3
function UVec3:mul(p1,p2) end

---@param _self UVec3 
---@param rhs IVec3 
---@return UVec3
function UVec3:wrapping_add_signed(_self,rhs) end

---@param p1 UVec3 
---@param p2 UVec3 
---@return UVec3
function UVec3:add(p1,p2) end

---@param _self UVec3 
---@return I16Vec3
function UVec3:as_i16vec3(_self) end

---@param _self UVec3 
---@return U8Vec3
function UVec3:as_u8vec3(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:add(_self,rhs) end

---@param p1 UVec3 
---@param p2 integer 
---@return UVec3
function UVec3:add(p1,p2) end

---@param _self UVec3 
---@return UVec3
function UVec3:clone(_self) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:div(_self,rhs) end

---@param _self UVec3 
---@param rhs UVec3 
---@return UVec3
function UVec3:wrapping_mul(_self,rhs) end

---@param _self UVec3 
---@return integer
function UVec3:element_product(_self) end


---@class UVec4 : ReflectReference
---@field  x ? integer
---@field  y ? integer
---@field  z ? integer
---@field  w ? integer
UVec4 = {}

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmpne(_self,rhs) end

---@param _self UVec4 
---@return integer
function UVec4:element_product(_self) end

---@param p1 UVec4 
---@param p2 integer 
---@return UVec4
function UVec4:rem(p1,p2) end

---@param x integer 
---@param y integer 
---@param z integer 
---@param w integer 
---@return UVec4
function UVec4.new(x,y,z,w) end

---@param _self UVec4 
---@return UVec4
function UVec4:clone(_self) end

---@param _self UVec4 
---@param rhs IVec4 
---@return UVec4
function UVec4:wrapping_add_signed(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmple(_self,rhs) end

---@param p1 UVec4 
---@param p2 UVec4 
---@return UVec4
function UVec4:add(p1,p2) end

---@param _self UVec4 
---@param x integer 
---@return UVec4
function UVec4:with_x(_self,x) end

---@param p1 UVec4 
---@param p2 integer 
---@return UVec4
function UVec4:sub(p1,p2) end

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmpge(_self,rhs) end

---@param _self UVec4 
---@param other UVec4 
---@return boolean
function UVec4:eq(_self,other) end

---@param _self UVec4 
---@return IVec4
function UVec4:as_ivec4(_self) end

---@param _self UVec4 
---@return integer
function UVec4:length_squared(_self) end

---@param _self UVec4 
---@return U16Vec4
function UVec4:as_u16vec4(_self) end

---@param _self UVec4 
---@param min UVec4 
---@param max UVec4 
---@return UVec4
function UVec4:clamp(_self,min,max) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:min(_self,rhs) end

---@param _self UVec4 
---@return DVec4
function UVec4:as_dvec4(_self) end

---@param _self UVec4 
---@return nil
function UVec4:assert_receiver_is_total_eq(_self) end

---@param _self UVec4 
---@return U64Vec4
function UVec4:as_u64vec4(_self) end

---@param _self UVec4 
---@return I8Vec4
function UVec4:as_i8vec4(_self) end

---@param v integer 
---@return UVec4
function UVec4.splat(v) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:saturating_mul(_self,rhs) end

---@param _self UVec4 
---@param w integer 
---@return UVec4
function UVec4:with_w(_self,w) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:sub(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:wrapping_div(_self,rhs) end

---@param a integer[] 
---@return UVec4
function UVec4.from_array(a) end

---@param _self UVec4 
---@return integer
function UVec4:element_sum(_self) end

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmpgt(_self,rhs) end

---@param _self UVec4 
---@param z integer 
---@return UVec4
function UVec4:with_z(_self,z) end

---@param _self UVec4 
---@return integer
function UVec4:max_element(_self) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:max(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:rem(_self,rhs) end

---@param p1 UVec4 
---@param p2 UVec4 
---@return UVec4
function UVec4:rem(p1,p2) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:mul(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmpeq(_self,rhs) end

---@param _self UVec4 
---@return I64Vec4
function UVec4:as_i64vec4(_self) end

---@param p1 UVec4 
---@param p2 UVec4 
---@return UVec4
function UVec4:div(p1,p2) end

---@param _self UVec4 
---@return integer[]
function UVec4:to_array(_self) end

---@param _self UVec4 
---@return I16Vec4
function UVec4:as_i16vec4(_self) end

---@param _self UVec4 
---@return integer
function UVec4:min_element(_self) end

---@param _self UVec4 
---@return Vec4
function UVec4:as_vec4(_self) end

---@param _self UVec4 
---@param rhs UVec4 
---@return integer
function UVec4:dot(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:wrapping_mul(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:saturating_sub(_self,rhs) end

---@param _self UVec4 
---@param rhs IVec4 
---@return UVec4
function UVec4:saturating_add_signed(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:dot_into_vec(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:div(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:saturating_div(_self,rhs) end

---@param _self UVec4 
---@param y integer 
---@return UVec4
function UVec4:with_y(_self,y) end

---@param mask BVec4 
---@param if_true UVec4 
---@param if_false UVec4 
---@return UVec4
function UVec4.select(mask,if_true,if_false) end

---@param _self UVec4 
---@param rhs UVec4 
---@return BVec4
function UVec4:cmplt(_self,rhs) end

---@param p1 UVec4 
---@param p2 integer 
---@return UVec4
function UVec4:div(p1,p2) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:saturating_add(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:wrapping_sub(_self,rhs) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:add(_self,rhs) end

---@param p1 UVec4 
---@param p2 integer 
---@return UVec4
function UVec4:add(p1,p2) end

---@param _self UVec4 
---@return UVec3
function UVec4:truncate(_self) end

---@param p1 UVec4 
---@param p2 UVec4 
---@return UVec4
function UVec4:mul(p1,p2) end

---@param _self UVec4 
---@return U8Vec4
function UVec4:as_u8vec4(_self) end

---@param p1 UVec4 
---@param p2 UVec4 
---@return UVec4
function UVec4:sub(p1,p2) end

---@param _self UVec4 
---@param rhs UVec4 
---@return UVec4
function UVec4:wrapping_add(_self,rhs) end

---@param p1 UVec4 
---@param p2 integer 
---@return UVec4
function UVec4:mul(p1,p2) end


---@class Vec2 : ReflectReference
---@field  x ? number
---@field  y ? number
Vec2 = {}

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmpeq(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:fract(_self) end

---@param _self Vec2 
---@param max number 
---@return Vec2
function Vec2:clamp_length_max(_self,max) end

---@param a number[] 
---@return Vec2
function Vec2.from_array(a) end

---@param _self Vec2 
---@return integer
function Vec2:is_negative_bitmask(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:clone(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:project_onto_normalized(_self,rhs) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:div(_self,rhs) end

---@param _self Vec2 
---@param min Vec2 
---@param max Vec2 
---@return Vec2
function Vec2:clamp(_self,min,max) end

---@param _self Vec2 
---@param rhs Vec2 
---@param s number 
---@return Vec2
function Vec2:lerp(_self,rhs,s) end

---@param _self Vec2 
---@return number
function Vec2:element_sum(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:div_euclid(_self,rhs) end

---@param angle number 
---@return Vec2
function Vec2.from_angle(angle) end

---@param _self Vec2 
---@param normal Vec2 
---@param eta number 
---@return Vec2
function Vec2:refract(_self,normal,eta) end

---@param _self Vec2 
---@param rhs Vec2 
---@param max_angle number 
---@return Vec2
function Vec2:rotate_towards(_self,rhs,max_angle) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:mul(_self,rhs) end

---@param _self Vec2 
---@return U64Vec2
function Vec2:as_u64vec2(_self) end

---@param _self Vec2 
---@return number
function Vec2:max_element(_self) end

---@param _self Vec2 
---@param other Vec2 
---@return boolean
function Vec2:eq(_self,other) end

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmpgt(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:trunc(_self) end

---@param _self Vec2 
---@return BVec2
function Vec2:is_nan_mask(_self) end

---@param _self Vec2 
---@param y number 
---@return Vec2
function Vec2:with_y(_self,y) end

---@param _self Vec2 
---@param x number 
---@return Vec2
function Vec2:with_x(_self,x) end

---@param _self Vec2 
---@return Vec2
function Vec2:recip(_self) end

---@param _self Vec2 
---@return boolean
function Vec2:is_nan(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:signum(_self) end

---@param p1 Vec2 
---@param p2 number 
---@return Vec2
function Vec2:rem(p1,p2) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:project_onto(_self,rhs) end

---@param _self Vec2 
---@param z number 
---@return Vec3
function Vec2:extend(_self,z) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:midpoint(_self,rhs) end

---@param _self Vec2 
---@return U8Vec2
function Vec2:as_u8vec2(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:dot(_self,rhs) end

---@param _self Vec2 
---@return UVec2
function Vec2:as_uvec2(_self) end

---@param _self Vec2 
---@return BVec2
function Vec2:is_finite_mask(_self) end

---@param _self Vec2 
---@param n number 
---@return Vec2
function Vec2:powf(_self,n) end

---@param _self Vec2 
---@return Vec2
function Vec2:normalize_or_zero(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:round(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:perp(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmplt(_self,rhs) end

---@param _self Vec2 
---@param min number 
---@param max number 
---@return Vec2
function Vec2:clamp_length(_self,min,max) end

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmpge(_self,rhs) end

---@param _self Vec2 
---@return number
function Vec2:to_angle(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:rotate(_self,rhs) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:dot_into_vec(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:exp(_self) end

---@param p1 Vec2 
---@param p2 Vec2 
---@return Vec2
function Vec2:rem(p1,p2) end

---@param _self Vec2 
---@param normal Vec2 
---@return Vec2
function Vec2:reflect(_self,normal) end

---@param p1 Vec2 
---@param p2 number 
---@return Vec2
function Vec2:div(p1,p2) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:add(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:neg(_self) end

---@param p1 Vec2 
---@param p2 Vec2 
---@return Vec2
function Vec2:sub(p1,p2) end

---@param mask BVec2 
---@param if_true Vec2 
---@param if_false Vec2 
---@return Vec2
function Vec2.select(mask,if_true,if_false) end

---@param _self Vec2 
---@return U16Vec2
function Vec2:as_u16vec2(_self) end

---@param p1 Vec2 
---@param p2 number 
---@return Vec2
function Vec2:sub(p1,p2) end

---@param p1 Vec2 
---@param p2 number 
---@return Vec2
function Vec2:mul(p1,p2) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:rem_euclid(_self,rhs) end

---@param p1 Vec2 
---@param p2 Vec2 
---@return Vec2
function Vec2:mul(p1,p2) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:distance_squared(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:ceil(_self) end

---@param _self Vec2 
---@return number
function Vec2:length_squared(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:floor(_self) end

---@param _self Vec2 
---@return DVec2
function Vec2:as_dvec2(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:max(_self,rhs) end

---@param _self Vec2 
---@return boolean
function Vec2:is_finite(_self) end

---@param _self Vec2 
---@return Vec2
function Vec2:abs(_self) end

---@param _self Vec2 
---@param a Vec2 
---@param b Vec2 
---@return Vec2
function Vec2:mul_add(_self,a,b) end

---@param _self Vec2 
---@param fallback Vec2 
---@return Vec2
function Vec2:normalize_or(_self,fallback) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:reject_from(_self,rhs) end

---@param p1 Vec2 
---@param p2 number 
---@return Vec2
function Vec2:add(p1,p2) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:angle_to(_self,rhs) end

---@param p1 Vec2 
---@param p2 Vec2 
---@return Vec2
function Vec2:div(p1,p2) end

---@param _self Vec2 
---@return I16Vec2
function Vec2:as_i16vec2(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:distance(_self,rhs) end

---@param _self Vec2 
---@return Vec2
function Vec2:fract_gl(_self) end

---@param _self Vec2 
---@return number
function Vec2:length_recip(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:sub(_self,rhs) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:angle_between(_self,rhs) end

---@param _self Vec2 
---@return number
function Vec2:min_element(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:copysign(_self,rhs) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:rem(_self,rhs) end

---@param _self Vec2 
---@return IVec2
function Vec2:as_ivec2(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmpne(_self,rhs) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:reject_from_normalized(_self,rhs) end

---@param _self Vec2 
---@return boolean
function Vec2:is_normalized(_self) end

---@param _self Vec2 
---@return number
function Vec2:element_product(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@param d number 
---@return Vec2
function Vec2:move_towards(_self,rhs,d) end

---@param v number 
---@return Vec2
function Vec2.splat(v) end

---@param _self Vec2 
---@param rhs Vec2 
---@return Vec2
function Vec2:min(_self,rhs) end

---@param p1 Vec2 
---@param p2 Vec2 
---@return Vec2
function Vec2:add(p1,p2) end

---@param _self Vec2 
---@return I8Vec2
function Vec2:as_i8vec2(_self) end

---@param _self Vec2 
---@return I64Vec2
function Vec2:as_i64vec2(_self) end

---@param _self Vec2 
---@return number
function Vec2:length(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return number
function Vec2:perp_dot(_self,rhs) end

---@param _self Vec2 
---@param min number 
---@return Vec2
function Vec2:clamp_length_min(_self,min) end

---@param x number 
---@param y number 
---@return Vec2
function Vec2.new(x,y) end

---@param _self Vec2 
---@param rhs Vec2 
---@param max_abs_diff number 
---@return boolean
function Vec2:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Vec2 
---@return Vec2
function Vec2:normalize(_self) end

---@param _self Vec2 
---@param rhs Vec2 
---@return BVec2
function Vec2:cmple(_self,rhs) end

---@param _self Vec2 
---@return number[]
function Vec2:to_array(_self) end


---@class Vec3 : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
Vec3 = {}

---@param _self Vec3 
---@return integer
function Vec3:is_negative_bitmask(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:clone(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:ceil(_self) end

---@param _self Vec3 
---@return number
function Vec3:max_element(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:min(_self,rhs) end

---@param _self Vec3 
---@param a Vec3 
---@param b Vec3 
---@return Vec3
function Vec3:mul_add(_self,a,b) end

---@param v number 
---@return Vec3
function Vec3.splat(v) end

---@param _self Vec3 
---@return Vec3
function Vec3:any_orthogonal_vector(_self) end

---@param _self Vec3 
---@return I64Vec3
function Vec3:as_i64vec3(_self) end

---@param _self Vec3 
---@return boolean
function Vec3:is_finite(_self) end

---@param _self Vec3 
---@return number[]
function Vec3:to_array(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:copysign(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@param s number 
---@return Vec3
function Vec3:lerp(_self,rhs,s) end

---@param p1 Vec3 
---@param p2 Vec3 
---@return Vec3
function Vec3:rem(p1,p2) end

---@param _self Vec3 
---@param fallback Vec3 
---@return Vec3
function Vec3:normalize_or(_self,fallback) end

---@param _self Vec3 
---@return number
function Vec3:length(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:floor(_self) end

---@param _self Vec3 
---@return number
function Vec3:element_product(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:normalize_or_zero(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@param d number 
---@return Vec3
function Vec3:move_towards(_self,rhs,d) end

---@param _self Vec3 
---@param rhs Vec3 
---@return number
function Vec3:angle_between(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmplt(_self,rhs) end

---@param p1 Vec3 
---@param p2 number 
---@return Vec3
function Vec3:sub(p1,p2) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:mul(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@param max_abs_diff number 
---@return boolean
function Vec3:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmpge(_self,rhs) end

---@param _self Vec3 
---@param min number 
---@param max number 
---@return Vec3
function Vec3:clamp_length(_self,min,max) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:div(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:midpoint(_self,rhs) end

---@param _self Vec3 
---@return boolean
function Vec3:is_nan(_self) end

---@param p1 Vec3 
---@param p2 number 
---@return Vec3
function Vec3:mul(p1,p2) end

---@param _self Vec3 
---@return BVec3
function Vec3:is_nan_mask(_self) end

---@param _self Vec3 
---@return U64Vec3
function Vec3:as_u64vec3(_self) end

---@param _self Vec3 
---@return BVec3
function Vec3:is_finite_mask(_self) end

---@param _self Vec3 
---@param w number 
---@return Vec4
function Vec3:extend(_self,w) end

---@param _self Vec3 
---@param normal Vec3 
---@param eta number 
---@return Vec3
function Vec3:refract(_self,normal,eta) end

---@param _self Vec3 
---@param min number 
---@return Vec3
function Vec3:clamp_length_min(_self,min) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:rem_euclid(_self,rhs) end

---@param _self Vec3 
---@return Vec3
function Vec3:neg(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:fract(_self) end

---@param p1 Vec3 
---@param p2 Vec3 
---@return Vec3
function Vec3:mul(p1,p2) end

---@param _self Vec3 
---@param rhs Vec3 
---@return number
function Vec3:distance(_self,rhs) end

---@param _self Vec3 
---@return UVec3
function Vec3:as_uvec3(_self) end

---@param _self Vec3 
---@return boolean
function Vec3:is_normalized(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:cross(_self,rhs) end

---@param _self Vec3 
---@return Vec3
function Vec3:signum(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:fract_gl(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:recip(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:sub(_self,rhs) end

---@param p1 Vec3 
---@param p2 Vec3 
---@return Vec3
function Vec3:sub(p1,p2) end

---@param p1 Vec3 
---@param p2 number 
---@return Vec3
function Vec3:rem(p1,p2) end

---@param a number[] 
---@return Vec3
function Vec3.from_array(a) end

---@param _self Vec3 
---@return Vec3
function Vec3:round(_self) end

---@param x number 
---@param y number 
---@param z number 
---@return Vec3
function Vec3.new(x,y,z) end

---@param _self Vec3 
---@return U16Vec3
function Vec3:as_u16vec3(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:add(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmpne(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmpgt(_self,rhs) end

---@param p1 Vec3 
---@param p2 number 
---@return Vec3
function Vec3:add(p1,p2) end

---@param _self Vec3 
---@return number
function Vec3:length_squared(_self) end

---@param _self Vec3 
---@return number
function Vec3:element_sum(_self) end

---@param _self Vec3 
---@return IVec3
function Vec3:as_ivec3(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:any_orthonormal_vector(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmple(_self,rhs) end

---@param p1 Vec3 
---@param p2 Vec3 
---@return Vec3
function Vec3:add(p1,p2) end

---@param mask BVec3 
---@param if_true Vec3 
---@param if_false Vec3 
---@return Vec3
function Vec3.select(mask,if_true,if_false) end

---@param _self Vec3 
---@param max number 
---@return Vec3
function Vec3:clamp_length_max(_self,max) end

---@param _self Vec3 
---@return Vec3
function Vec3:trunc(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:max(_self,rhs) end

---@param _self Vec3 
---@return Vec3
function Vec3:abs(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:exp(_self) end

---@param _self Vec3 
---@return number
function Vec3:length_recip(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:rem(_self,rhs) end

---@param _self Vec3 
---@param normal Vec3 
---@return Vec3
function Vec3:reflect(_self,normal) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:project_onto_normalized(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return BVec3
function Vec3:cmpeq(_self,rhs) end

---@param _self Vec3 
---@return I16Vec3
function Vec3:as_i16vec3(_self) end

---@param p1 Vec3 
---@param p2 number 
---@return Vec3
function Vec3:div(p1,p2) end

---@param _self Vec3 
---@param other Vec3 
---@return boolean
function Vec3:eq(_self,other) end

---@param _self Vec3 
---@return I8Vec3
function Vec3:as_i8vec3(_self) end

---@param p1 Vec3 
---@param p2 Vec3 
---@return Vec3
function Vec3:div(p1,p2) end

---@param _self Vec3 
---@param rhs Vec3 
---@return number
function Vec3:dot(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:reject_from_normalized(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:div_euclid(_self,rhs) end

---@param _self Vec3 
---@param y number 
---@return Vec3
function Vec3:with_y(_self,y) end

---@param _self Vec3 
---@return U8Vec3
function Vec3:as_u8vec3(_self) end

---@param _self Vec3 
---@return number
function Vec3:min_element(_self) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:project_onto(_self,rhs) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:reject_from(_self,rhs) end

---@param _self Vec3 
---@param z number 
---@return Vec3
function Vec3:with_z(_self,z) end

---@param _self Vec3 
---@return DVec3
function Vec3:as_dvec3(_self) end

---@param _self Vec3 
---@return Vec3
function Vec3:normalize(_self) end

---@param _self Vec3 
---@param n number 
---@return Vec3
function Vec3:powf(_self,n) end

---@param _self Vec3 
---@param rhs Vec3 
---@return number
function Vec3:distance_squared(_self,rhs) end

---@param _self Vec3 
---@param x number 
---@return Vec3
function Vec3:with_x(_self,x) end

---@param _self Vec3 
---@param rhs Vec3 
---@return Vec3
function Vec3:dot_into_vec(_self,rhs) end

---@param _self Vec3 
---@param min Vec3 
---@param max Vec3 
---@return Vec3
function Vec3:clamp(_self,min,max) end

---@param _self Vec3 
---@return Vec2
function Vec3:truncate(_self) end


---@class Vec3A : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
Vec3A = {}

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:copysign(_self,rhs) end

---@param _self Vec3A 
---@return boolean
function Vec3A:is_finite(_self) end

---@param p1 Vec3A 
---@param p2 number 
---@return Vec3A
function Vec3A:rem(p1,p2) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return number
function Vec3A:distance(_self,rhs) end

---@param _self Vec3A 
---@param min number 
---@return Vec3A
function Vec3A:clamp_length_min(_self,min) end

---@param _self Vec3A 
---@return boolean
function Vec3A:is_normalized(_self) end

---@param _self Vec3A 
---@return I64Vec3
function Vec3A:as_i64vec3(_self) end

---@param _self Vec3A 
---@return number[]
function Vec3A:to_array(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:reject_from_normalized(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:round(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:max(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:midpoint(_self,rhs) end

---@param _self Vec3A 
---@return BVec3A
function Vec3A:is_finite_mask(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:project_onto_normalized(_self,rhs) end

---@param _self Vec3A 
---@param y number 
---@return Vec3A
function Vec3A:with_y(_self,y) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:div_euclid(_self,rhs) end

---@param _self Vec3A 
---@param fallback Vec3A 
---@return Vec3A
function Vec3A:normalize_or(_self,fallback) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:fract(_self) end

---@param _self Vec3A 
---@return U16Vec3
function Vec3A:as_u16vec3(_self) end

---@param p1 Vec3A 
---@param p2 Vec3A 
---@return Vec3A
function Vec3A:mul(p1,p2) end

---@param a number[] 
---@return Vec3A
function Vec3A.from_array(a) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@param d number 
---@return Vec3A
function Vec3A:move_towards(_self,rhs,d) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:cross(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmpne(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmpeq(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:neg(_self) end

---@param _self Vec3A 
---@param normal Vec3A 
---@param eta number 
---@return Vec3A
function Vec3A:refract(_self,normal,eta) end

---@param _self Vec3A 
---@return DVec3
function Vec3A:as_dvec3(_self) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:signum(_self) end

---@param _self Vec3A 
---@param n number 
---@return Vec3A
function Vec3A:powf(_self,n) end

---@param _self Vec3A 
---@return number
function Vec3A:min_element(_self) end

---@param p1 Vec3A 
---@param p2 Vec3A 
---@return Vec3A
function Vec3A:div(p1,p2) end

---@param _self Vec3A 
---@return boolean
function Vec3A:is_nan(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:rem_euclid(_self,rhs) end

---@param _self Vec3A 
---@return number
function Vec3A:element_product(_self) end

---@param x number 
---@param y number 
---@param z number 
---@return Vec3A
function Vec3A.new(x,y,z) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:ceil(_self) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:abs(_self) end

---@param v Vec4 
---@return Vec3A
function Vec3A.from_vec4(v) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:trunc(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:project_onto(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:reject_from(_self,rhs) end

---@param p1 Vec3A 
---@param p2 Vec3A 
---@return Vec3A
function Vec3A:sub(p1,p2) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmpgt(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:normalize(_self) end

---@param _self Vec3A 
---@return number
function Vec3A:length(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return boolean
function Vec3A:eq(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@param max_abs_diff number 
---@return boolean
function Vec3A:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Vec3A 
---@return number
function Vec3A:max_element(_self) end

---@param _self Vec3A 
---@return I8Vec3
function Vec3A:as_i8vec3(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:sub(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return number
function Vec3A:angle_between(_self,rhs) end

---@param _self Vec3A 
---@return I16Vec3
function Vec3A:as_i16vec3(_self) end

---@param _self Vec3A 
---@param a Vec3A 
---@param b Vec3A 
---@return Vec3A
function Vec3A:mul_add(_self,a,b) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:min(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:rem(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:dot_into_vec(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return number
function Vec3A:dot(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmple(_self,rhs) end

---@param _self Vec3A 
---@return IVec3
function Vec3A:as_ivec3(_self) end

---@param p1 Vec3A 
---@param p2 number 
---@return Vec3A
function Vec3A:add(p1,p2) end

---@param _self Vec3A 
---@return number
function Vec3A:length_squared(_self) end

---@param p1 Vec3A 
---@param p2 Vec3A 
---@return Vec3A
function Vec3A:add(p1,p2) end

---@param _self Vec3A 
---@return number
function Vec3A:element_sum(_self) end

---@param _self Vec3A 
---@param max number 
---@return Vec3A
function Vec3A:clamp_length_max(_self,max) end

---@param _self Vec3A 
---@return BVec3A
function Vec3A:is_nan_mask(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:div(_self,rhs) end

---@param _self Vec3A 
---@return integer
function Vec3A:is_negative_bitmask(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:mul(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:recip(_self) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:any_orthogonal_vector(_self) end

---@param _self Vec3A 
---@param min Vec3A 
---@param max Vec3A 
---@return Vec3A
function Vec3A:clamp(_self,min,max) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return number
function Vec3A:distance_squared(_self,rhs) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmpge(_self,rhs) end

---@param _self Vec3A 
---@param w number 
---@return Vec4
function Vec3A:extend(_self,w) end

---@param _self Vec3A 
---@param x number 
---@return Vec3A
function Vec3A:with_x(_self,x) end

---@param v number 
---@return Vec3A
function Vec3A.splat(v) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:any_orthonormal_vector(_self) end

---@param _self Vec3A 
---@return UVec3
function Vec3A:as_uvec3(_self) end

---@param p1 Vec3A 
---@param p2 number 
---@return Vec3A
function Vec3A:mul(p1,p2) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@param s number 
---@return Vec3A
function Vec3A:lerp(_self,rhs,s) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:clone(_self) end

---@param _self Vec3A 
---@return number
function Vec3A:length_recip(_self) end

---@param p1 Vec3A 
---@param p2 number 
---@return Vec3A
function Vec3A:sub(p1,p2) end

---@param _self Vec3A 
---@return U64Vec3
function Vec3A:as_u64vec3(_self) end

---@param _self Vec3A 
---@return Vec2
function Vec3A:truncate(_self) end

---@param p1 Vec3A 
---@param p2 number 
---@return Vec3A
function Vec3A:div(p1,p2) end

---@param _self Vec3A 
---@param normal Vec3A 
---@return Vec3A
function Vec3A:reflect(_self,normal) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return BVec3A
function Vec3A:cmplt(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:floor(_self) end

---@param _self Vec3A 
---@param z number 
---@return Vec3A
function Vec3A:with_z(_self,z) end

---@param _self Vec3A 
---@param min number 
---@param max number 
---@return Vec3A
function Vec3A:clamp_length(_self,min,max) end

---@param _self Vec3A 
---@return U8Vec3
function Vec3A:as_u8vec3(_self) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:fract_gl(_self) end

---@param _self Vec3A 
---@param rhs Vec3A 
---@return Vec3A
function Vec3A:add(_self,rhs) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:normalize_or_zero(_self) end

---@param _self Vec3A 
---@return Vec3A
function Vec3A:exp(_self) end

---@param mask BVec3A 
---@param if_true Vec3A 
---@param if_false Vec3A 
---@return Vec3A
function Vec3A.select(mask,if_true,if_false) end

---@param p1 Vec3A 
---@param p2 Vec3A 
---@return Vec3A
function Vec3A:rem(p1,p2) end


---@class Vec4 : ReflectReference
---@field  x ? number
---@field  y ? number
---@field  z ? number
---@field  w ? number
Vec4 = {}

---@param a number[] 
---@return Vec4
function Vec4.from_array(a) end

---@param _self Vec4 
---@param a Vec4 
---@param b Vec4 
---@return Vec4
function Vec4:mul_add(_self,a,b) end

---@param _self Vec4 
---@return Vec4
function Vec4:abs(_self) end

---@param _self Vec4 
---@return boolean
function Vec4:is_normalized(_self) end

---@param x number 
---@param y number 
---@param z number 
---@param w number 
---@return Vec4
function Vec4.new(x,y,z,w) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:reject_from_normalized(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:midpoint(_self,rhs) end

---@param _self Vec4 
---@return Vec4
function Vec4:fract(_self) end

---@param _self Vec4 
---@return Vec4
function Vec4:signum(_self) end

---@param mask BVec4A 
---@param if_true Vec4 
---@param if_false Vec4 
---@return Vec4
function Vec4.select(mask,if_true,if_false) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmpgt(_self,rhs) end

---@param _self Vec4 
---@return Vec4
function Vec4:ceil(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:div(_self,rhs) end

---@param _self Vec4 
---@return number
function Vec4:element_sum(_self) end

---@param p1 Vec4 
---@param p2 number 
---@return Vec4
function Vec4:sub(p1,p2) end

---@param p1 Vec4 
---@param p2 number 
---@return Vec4
function Vec4:div(p1,p2) end

---@param _self Vec4 
---@return U16Vec4
function Vec4:as_u16vec4(_self) end

---@param _self Vec4 
---@return Vec4
function Vec4:normalize_or_zero(_self) end

---@param _self Vec4 
---@return U8Vec4
function Vec4:as_u8vec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:rem(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return number
function Vec4:dot(_self,rhs) end

---@param _self Vec4 
---@param min number 
---@return Vec4
function Vec4:clamp_length_min(_self,min) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmpne(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@param d number 
---@return Vec4
function Vec4:move_towards(_self,rhs,d) end

---@param _self Vec4 
---@return Vec4
function Vec4:exp(_self) end

---@param _self Vec4 
---@return Vec4
function Vec4:floor(_self) end

---@param _self Vec4 
---@param fallback Vec4 
---@return Vec4
function Vec4:normalize_or(_self,fallback) end

---@param p1 Vec4 
---@param p2 Vec4 
---@return Vec4
function Vec4:rem(p1,p2) end

---@param p1 Vec4 
---@param p2 number 
---@return Vec4
function Vec4:mul(p1,p2) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:reject_from(_self,rhs) end

---@param _self Vec4 
---@return I16Vec4
function Vec4:as_i16vec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:copysign(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return number
function Vec4:distance_squared(_self,rhs) end

---@param _self Vec4 
---@return boolean
function Vec4:is_nan(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:add(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:mul(_self,rhs) end

---@param _self Vec4 
---@return number
function Vec4:length_recip(_self) end

---@param _self Vec4 
---@return number
function Vec4:length_squared(_self) end

---@param _self Vec4 
---@return Vec4
function Vec4:trunc(_self) end

---@param _self Vec4 
---@return number[]
function Vec4:to_array(_self) end

---@param p1 Vec4 
---@param p2 number 
---@return Vec4
function Vec4:rem(p1,p2) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:project_onto(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:sub(_self,rhs) end

---@param _self Vec4 
---@return DVec4
function Vec4:as_dvec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmple(_self,rhs) end

---@param _self Vec4 
---@param max number 
---@return Vec4
function Vec4:clamp_length_max(_self,max) end

---@param _self Vec4 
---@return I64Vec4
function Vec4:as_i64vec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:max(_self,rhs) end

---@param _self Vec4 
---@param normal Vec4 
---@return Vec4
function Vec4:reflect(_self,normal) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:dot_into_vec(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:rem_euclid(_self,rhs) end

---@param _self Vec4 
---@param rhs Vec4 
---@return boolean
function Vec4:eq(_self,rhs) end

---@param _self Vec4 
---@param n number 
---@return Vec4
function Vec4:powf(_self,n) end

---@param _self Vec4 
---@return Vec4
function Vec4:clone(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmpge(_self,rhs) end

---@param _self Vec4 
---@return integer
function Vec4:is_negative_bitmask(_self) end

---@param _self Vec4 
---@return number
function Vec4:max_element(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@param s number 
---@return Vec4
function Vec4:lerp(_self,rhs,s) end

---@param _self Vec4 
---@param min number 
---@param max number 
---@return Vec4
function Vec4:clamp_length(_self,min,max) end

---@param _self Vec4 
---@param rhs Vec4 
---@param max_abs_diff number 
---@return boolean
function Vec4:abs_diff_eq(_self,rhs,max_abs_diff) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:project_onto_normalized(_self,rhs) end

---@param p1 Vec4 
---@param p2 Vec4 
---@return Vec4
function Vec4:div(p1,p2) end

---@param _self Vec4 
---@return BVec4A
function Vec4:is_nan_mask(_self) end

---@param _self Vec4 
---@return number
function Vec4:length(_self) end

---@param _self Vec4 
---@param y number 
---@return Vec4
function Vec4:with_y(_self,y) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:min(_self,rhs) end

---@param _self Vec4 
---@return Vec4
function Vec4:recip(_self) end

---@param p1 Vec4 
---@param p2 Vec4 
---@return Vec4
function Vec4:sub(p1,p2) end

---@param p1 Vec4 
---@param p2 Vec4 
---@return Vec4
function Vec4:mul(p1,p2) end

---@param _self Vec4 
---@return Vec4
function Vec4:fract_gl(_self) end

---@param _self Vec4 
---@param w number 
---@return Vec4
function Vec4:with_w(_self,w) end

---@param _self Vec4 
---@return Vec3
function Vec4:truncate(_self) end

---@param _self Vec4 
---@return number
function Vec4:min_element(_self) end

---@param _self Vec4 
---@return I8Vec4
function Vec4:as_i8vec4(_self) end

---@param _self Vec4 
---@return boolean
function Vec4:is_finite(_self) end

---@param _self Vec4 
---@param z number 
---@return Vec4
function Vec4:with_z(_self,z) end

---@param _self Vec4 
---@return Vec4
function Vec4:neg(_self) end

---@param _self Vec4 
---@return BVec4A
function Vec4:is_finite_mask(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return number
function Vec4:distance(_self,rhs) end

---@param _self Vec4 
---@return UVec4
function Vec4:as_uvec4(_self) end

---@param _self Vec4 
---@param x number 
---@return Vec4
function Vec4:with_x(_self,x) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmplt(_self,rhs) end

---@param _self Vec4 
---@param min Vec4 
---@param max Vec4 
---@return Vec4
function Vec4:clamp(_self,min,max) end

---@param _self Vec4 
---@return IVec4
function Vec4:as_ivec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return BVec4A
function Vec4:cmpeq(_self,rhs) end

---@param _self Vec4 
---@return number
function Vec4:element_product(_self) end

---@param _self Vec4 
---@return Vec4
function Vec4:round(_self) end

---@param _self Vec4 
---@param normal Vec4 
---@param eta number 
---@return Vec4
function Vec4:refract(_self,normal,eta) end

---@param v number 
---@return Vec4
function Vec4.splat(v) end

---@param p1 Vec4 
---@param p2 number 
---@return Vec4
function Vec4:add(p1,p2) end

---@param _self Vec4 
---@return Vec4
function Vec4:normalize(_self) end

---@param p1 Vec4 
---@param p2 Vec4 
---@return Vec4
function Vec4:add(p1,p2) end

---@param _self Vec4 
---@return U64Vec4
function Vec4:as_u64vec4(_self) end

---@param _self Vec4 
---@param rhs Vec4 
---@return Vec4
function Vec4:div_euclid(_self,rhs) end


---@class SmolStr : ReflectReference
SmolStr = {}

---@param _self SmolStr 
---@return string
function SmolStr:to_string(_self) end

---@param _self SmolStr 
---@return integer
function SmolStr:len(_self) end

---@param _self SmolStr 
---@return boolean
function SmolStr:is_empty(_self) end

---@param _self SmolStr 
---@return boolean
function SmolStr:is_heap_allocated(_self) end

---@param _self SmolStr 
---@param other SmolStr 
---@return boolean
function SmolStr:eq(_self,other) end

---@param _self SmolStr 
---@return SmolStr
function SmolStr:clone(_self) end


---@class Uuid : ReflectReference
Uuid = {}

---@param _self Uuid 
---@param other Uuid 
---@return boolean
function Uuid:eq(_self,other) end

---@return integer[]
function Uuid.encode_buffer() end


---@param _self Uuid 
---@return boolean
function Uuid:is_max(_self) end

---@param _self Uuid 
---@return Uuid
function Uuid:clone(_self) end

---@return Uuid
function Uuid.new_v4() end

---@param _self Uuid 
---@return [integer, integer]
function Uuid:as_u64_pair(_self) end

---@param _self Uuid 
---@return integer[]
function Uuid:into_bytes(_self) end

---@param _self Uuid 
---@return integer[] | nil
function Uuid:get_node_id(_self) end

---@param bytes integer[] 
---@return Uuid
function Uuid.from_bytes(bytes) end

---@param _self Uuid 
---@return integer
function Uuid:as_u128(_self) end

---@param _self Uuid 
---@return integer
function Uuid:get_version_num(_self) end

---@param _self Uuid 
---@return integer[]
function Uuid:to_bytes_le(_self) end

---@param v integer 
---@return Uuid
function Uuid.from_u128_le(v) end

---@param v integer 
---@return Uuid
function Uuid.from_u128(v) end

---@param _self Uuid 
---@return nil
function Uuid:assert_receiver_is_total_eq(_self) end

---@param _self Uuid 
---@return integer
function Uuid:to_u128_le(_self) end

---@return Uuid
function Uuid.max() end

---@param b integer[] 
---@return Uuid
function Uuid.from_bytes_le(b) end

---@param _self Uuid 
---@return boolean
function Uuid:is_nil(_self) end

---@param high_bits integer 
---@param low_bits integer 
---@return Uuid
function Uuid.from_u64_pair(high_bits,low_bits) end


---@class DynamicFunction : ReflectReference
---  A dynamic script function.
DynamicFunction = {}


---@class DynamicFunctionMut : ReflectReference
---  A dynamic mutable script function.
DynamicFunctionMut = {}


---@class FunctionCallContext : ReflectReference
---  The caller context when calling a script function.
---  Functions can choose to react to caller preferences such as converting 1-indexed numbers to 0-indexed numbers
FunctionCallContext = {}


---@class PathBuf : ReflectReference
--- A heap allocated file path
PathBuf = {}


---@class String : ReflectReference
--- A heap allocated string
String = {}


---@class AssetIndex : ReflectReference
---  A generational runtime-only identifier for a specific [`Asset`] stored in [`Assets`]. This is optimized for efficient runtime
---  usage and is not suitable for identifying assets across app runs.
---@field  generation ? integer
---@field  index ? integer
AssetIndex = {}


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
AssetPath = {}


---@class RenderAssetUsages : ReflectReference
---  Defines where the asset will be used.
--- 
---  If an asset is set to the `RENDER_WORLD` but not the `MAIN_WORLD`, the asset will be
---  unloaded from the asset server once it's been extracted and prepared in the render world.
--- 
---  Unloading the asset saves on memory, as for most cases it is no longer necessary to keep
---  it in RAM once it's been uploaded to the GPU's VRAM. However, this means you can no longer
---  access the asset from the CPU (via the `Assets<T>` resource) once unloaded (without re-loading it).
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
RenderAssetUsages = {}


---@class DeferredPrepass : ReflectReference
---  If added to a [`crate::prelude::Camera3d`] then deferred materials will be rendered to the deferred gbuffer texture and will be available to subsequent passes.
---  Note the default deferred lighting plugin also requires `DepthPrepass` to work correctly.
DeferredPrepass = {}


---@class SystemIdMarker : ReflectReference
---  Marker [`Component`](bevy_ecs::component::Component) for identifying [`SystemId`] [`Entity`]s.
SystemIdMarker = {}


---@class OnAdd : ReflectReference
---  Trigger emitted when a component is inserted onto an entity that does not already have that
---  component. Runs before `OnInsert`.
---  See [`crate::component::ComponentHooks::on_add`] for more information.
OnAdd = {}


---@class OnDespawn : ReflectReference
---  Trigger emitted for each component on an entity when it is despawned.
---  See [`crate::component::ComponentHooks::on_despawn`] for more information.
OnDespawn = {}


---@class OnInsert : ReflectReference
---  Trigger emitted when a component is inserted, regardless of whether or not the entity already
---  had that component. Runs after `OnAdd`, if it ran.
---  See [`crate::component::ComponentHooks::on_insert`] for more information.
OnInsert = {}


---@class OnRemove : ReflectReference
---  Trigger emitted when a component is removed from an entity, and runs before the component is
---  removed, so you can still access the component data.
---  See [`crate::component::ComponentHooks::on_remove`] for more information.
OnRemove = {}


---@class OnReplace : ReflectReference
---  Trigger emitted when a component is inserted onto an entity that already has that component.
---  Runs before the value is replaced, so you can still access the original component data.
---  See [`crate::component::ComponentHooks::on_replace`] for more information.
OnReplace = {}


---@class Image : ReflectReference
Image = {}


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
---@field  layout ? Handle
---@field  index ? integer
TextureAtlas = {}


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
---@field  size ? UVec2
---@field  textures ? Vec
TextureAtlasLayout = {}


---@class Affine3 : ReflectReference
---  Reduced-size version of `glam::Affine3A` for use when storage has
---  significant performance impact. Convert to `glam::Affine3A` to do
---  non-trivial calculations.
---@field  matrix3 ? Mat3
---@field  translation ? Vec3
Affine3 = {}


---@class Indices : ReflectReference
---  An array of indices into the [`VertexAttributeValues`](super::VertexAttributeValues) for a mesh.
--- 
---  It describes the order in which the vertex attributes should be joined into faces.
Indices = {}


---@class Mesh : ReflectReference
---  A 3D object made out of vertices representing triangles, lines, or points,
---  with "attribute" values for each vertex.
--- 
---  Meshes can be automatically generated by a bevy `AssetLoader` (generally by loading a `Gltf` file),
---  or by converting a [primitive](bevy_math::primitives) using [`into`](Into).
---  It is also possible to create one manually. They can be edited after creation.
--- 
---  Meshes can be rendered with a `Mesh2d` and `MeshMaterial2d`
---  or `Mesh3d` and `MeshMaterial3d` for 2D and 3D respectively.
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
---  used in a `Mesh3d` with a square bevy logo texture, with added axis, points,
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
---@field  indices ? Option
---@field  morph_targets ? Option
---@field  morph_target_names ? Option
---@field  asset_usage ? RenderAssetUsages
Mesh = {}


---@class MeshMorphWeights : ReflectReference
---  Control a specific [`Mesh`] instance's [morph targets]. These control the weights of
---  specific "mesh primitives" in scene formats like GLTF. They can be set manually, but
---  in most cases they should "automatically" synced by setting the [`MorphWeights`] component
---  on a parent entity.
--- 
---  See [`MorphWeights`] for more details on Bevy's morph target implementation.
--- 
---  Add this to an [`Entity`] with a `Mesh3d` with a [`MorphAttributes`] set
---  to control individual weights of each morph target.
--- 
---  [morph targets]: https://en.wikipedia.org/wiki/Morph_target_animation
---@field  weights ? Vec
MeshMorphWeights = {}


---@class MorphWeights : ReflectReference
---  Controls the [morph targets] for all child `Mesh3d` entities. In most cases, [`MorphWeights`] should be considered
---  the "source of truth" when writing morph targets for meshes. However you can choose to write child [`MeshMorphWeights`]
---  if your situation requires more granularity. Just note that if you set [`MorphWeights`], it will overwrite child
---  [`MeshMorphWeights`] values.
--- 
---  This exists because Bevy's [`Mesh`] corresponds to a _single_ surface / material, whereas morph targets
---  as defined in the GLTF spec exist on "multi-primitive meshes" (where each primitive is its own surface with its own material).
---  Therefore in Bevy [`MorphWeights`] an a parent entity are the "canonical weights" from a GLTF perspective, which then
---  synchronized to child `Mesh3d` / [`MeshMorphWeights`] (which correspond to "primitives" / "surfaces" from a GLTF perspective).
--- 
---  Add this to the parent of one or more [`Entities`](`Entity`) with a `Mesh3d` with a [`MeshMorphWeights`].
--- 
---  [morph targets]: https://en.wikipedia.org/wiki/Morph_target_animation
---@field  weights ? Vec
---@field  first_mesh ? Option
MorphWeights = {}


---@class AnnulusMeshBuilder : ReflectReference
---  A builder for creating a [`Mesh`] with an [`Annulus`] shape.
---@field  annulus ? Annulus
---@field  resolution ? integer
AnnulusMeshBuilder = {}


---@class Capsule2dMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Capsule2d`] shape.
---@field  capsule ? Capsule2d
---@field  resolution ? integer
Capsule2dMeshBuilder = {}


---@class CircleMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Circle`] shape.
---@field  circle ? Circle
---@field  resolution ? integer
CircleMeshBuilder = {}


---@class CircularMeshUvMode : ReflectReference
---  Specifies how to generate UV-mappings for the [`CircularSector`] and [`CircularSegment`] shapes.
--- 
---  Currently the only variant is `Mask`, which is good for showing a portion of a texture that includes
---  the entire circle, particularly the same texture will be displayed with different fractions of a
---  complete circle.
--- 
---  It's expected that more will be added in the future, such as a variant that causes the texture to be
---  scaled to fit the bounding box of the shape, which would be good for packed textures only including the
---  portion of the circle that is needed to display.
CircularMeshUvMode = {}


---@class CircularSectorMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`CircularSector`] shape.
--- 
---  The resulting mesh will have a UV-map such that the center of the circle is
---  at the center of the texture.
---@field  sector ? CircularSector
---@field  resolution ? integer
---@field  uv_mode ? CircularMeshUvMode
CircularSectorMeshBuilder = {}


---@class CircularSegmentMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`CircularSegment`] shape.
--- 
---  The resulting mesh will have a UV-map such that the center of the circle is
---  at the center of the texture.
---@field  segment ? CircularSegment
---@field  resolution ? integer
---@field  uv_mode ? CircularMeshUvMode
CircularSegmentMeshBuilder = {}


---@class EllipseMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with an [`Ellipse`] shape.
---@field  ellipse ? Ellipse
---@field  resolution ? integer
EllipseMeshBuilder = {}


---@class RectangleMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Rectangle`] shape.
---@field  half_size ? Vec2
RectangleMeshBuilder = {}


---@class RegularPolygonMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`RegularPolygon`] shape.
---@field  circumradius ? number
---@field  sides ? integer
RegularPolygonMeshBuilder = {}


---@class RhombusMeshBuilder : ReflectReference
---  A builder for creating a [`Mesh`] with an [`Rhombus`] shape.
---@field  half_diagonals ? Vec2
RhombusMeshBuilder = {}


---@class Triangle2dMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Triangle2d`] shape.
---@field  triangle ? Triangle2d
Triangle2dMeshBuilder = {}


---@class Capsule3dMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Capsule3d`] shape.
---@field  capsule ? Capsule3d
---@field  rings ? integer
---@field  longitudes ? integer
---@field  latitudes ? integer
---@field  uv_profile ? CapsuleUvProfile
Capsule3dMeshBuilder = {}


---@class CapsuleUvProfile : ReflectReference
---  Manner in which UV coordinates are distributed vertically.
CapsuleUvProfile = {}


---@class ConeAnchor : ReflectReference
---  Anchoring options for [`ConeMeshBuilder`]
ConeAnchor = {}


---@class ConeMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Cone`] shape.
---@field  cone ? Cone
---@field  resolution ? integer
---@field  anchor ? ConeAnchor
ConeMeshBuilder = {}


---@class ConicalFrustumMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`ConicalFrustum`] shape.
---@field  frustum ? ConicalFrustum
---@field  resolution ? integer
---@field  segments ? integer
ConicalFrustumMeshBuilder = {}


---@class CuboidMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Cuboid`] shape.
---@field  half_size ? Vec3
CuboidMeshBuilder = {}


---@class CylinderAnchor : ReflectReference
---  Anchoring options for [`CylinderMeshBuilder`]
CylinderAnchor = {}


---@class CylinderMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Cylinder`] shape.
---@field  cylinder ? Cylinder
---@field  resolution ? integer
---@field  segments ? integer
---@field  caps ? boolean
---@field  anchor ? CylinderAnchor
CylinderMeshBuilder = {}


---@class PlaneMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Plane3d`] shape.
---@field  plane ? Plane3d
---@field  subdivisions ? integer
PlaneMeshBuilder = {}


---@class SphereKind : ReflectReference
---  A type of sphere mesh.
SphereKind = {}


---@class SphereMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with an [`Sphere`] shape.
---@field  sphere ? Sphere
---@field  kind ? SphereKind
SphereMeshBuilder = {}


---@class TetrahedronMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Tetrahedron`] shape.
---@field  tetrahedron ? Tetrahedron
TetrahedronMeshBuilder = {}


---@class TorusMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Torus`] shape.
---@field  torus ? Torus
---@field  minor_resolution ? integer
---@field  major_resolution ? integer
---@field  angle_range ? RangeInclusive
TorusMeshBuilder = {}


---@class Triangle3dMeshBuilder : ReflectReference
---  A builder used for creating a [`Mesh`] with a [`Triangle3d`] shape.
---@field  triangle ? Triangle3d
Triangle3dMeshBuilder = {}


---@class SkinnedMesh : ReflectReference
---@field  inverse_bindposes ? bevy_asset::handle::Handle<bevy_mesh::skinning::SkinnedMeshInverseBindposes>
---@field  joints ? Vec
SkinnedMesh = {}


---@class ScriptAsset : ReflectReference
---  Represents a script loaded into memory as an asset
ScriptAsset = {}


---@class FunctionArgInfo : ReflectReference
---  Information about a function argument.
---@field  name ? Option
---@field  arg_index ? integer
---@field  type_id ? TypeId
FunctionArgInfo = {}


---@class FunctionInfo : ReflectReference
---  Information about a function.
---@field  name ? Cow
---@field  namespace ? Namespace
---@field  arg_info ? Vec
---@field  return_info ? FunctionReturnInfo
---@field  docs ? Option
FunctionInfo = {}


---@class FunctionReturnInfo : ReflectReference
---  Information about a function return value.
---@field  type_id ? TypeId
FunctionReturnInfo = {}


---@class InteropError : ReflectReference
---  An error occurring when converting between rust and a script context.
InteropError = {}


---@class Namespace : ReflectReference
---  A namespace for functions
Namespace = {}


---@class DynamicComponent : ReflectReference
---  A dynamic script component
---@field  data ? ScriptValue
DynamicComponent = {}


---@class ScriptValue : ReflectReference
---  An abstraction of values that can be passed to and from scripts.
---  This allows us to re-use logic between scripting languages.
ScriptValue = {}


---@class AlphaMode : ReflectReference
---  Sets how a material's base color alpha channel is used for transparency.
AlphaMode = {}


---@class Camera : ReflectReference
---  The defining [`Component`] for camera entities,
---  storing information about how and what to render through this camera.
--- 
---  The [`Camera`] component is added to an entity to define the properties of the viewpoint from
---  which rendering occurs. It defines the position of the view to render, the projection method
---  to transform the 3D objects into a 2D image, as well as the render target into which that image
---  is produced.
--- 
---  Note that a [`Camera`] needs a [`CameraRenderGraph`] to render anything.
---  This is typically provided by adding a [`Camera2d`] or [`Camera3d`] component,
---  but custom render graphs can also be defined. Inserting a [`Camera`] with no render
---  graph will emit an error at runtime.
--- 
---  [`Camera2d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/core_2d/struct.Camera2d.html
---  [`Camera3d`]: https://docs.rs/bevy/latest/bevy/core_pipeline/core_3d/struct.Camera3d.html
---@field  viewport ? Option
---@field  order ? integer
---@field  is_active ? boolean
---@field  target ? RenderTarget
---@field  hdr ? boolean
---@field  msaa_writeback ? boolean
---@field  clear_color ? ClearColorConfig
---@field  sub_camera_view ? Option
Camera = {}


---@class CameraMainTextureUsages : ReflectReference
---  This component lets you control the [`TextureUsages`] field of the main texture generated for the camera
CameraMainTextureUsages = {}


---@class CameraRenderGraph : ReflectReference
---  Configures the [`RenderGraph`](crate::render_graph::RenderGraph) name assigned to be run for a given [`Camera`] entity.
CameraRenderGraph = {}


---@class Exposure : ReflectReference
---  How much energy a `Camera3d` absorbs from incoming light.
--- 
---  <https://en.wikipedia.org/wiki/Exposure_(photography)>
Exposure = {}


---@class ImageRenderTarget : ReflectReference
---  A render target that renders to an [`Image`].
---@field  handle ? Handle
---@field  scale_factor ? FloatOrd
ImageRenderTarget = {}


---@class MipBias : ReflectReference
---  Camera component specifying a mip bias to apply when sampling from material textures.
--- 
---  Often used in conjunction with antialiasing post-process effects to reduce textures blurriness.
---@field  [1] ? number
MipBias = {}


---@class RenderTarget : ReflectReference
---  The "target" that a [`Camera`] will render to. For example, this could be a [`Window`]
---  swapchain or an [`Image`].
RenderTarget = {}


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
---@field  full_size ? UVec2
---@field  offset ? Vec2
---@field  size ? UVec2
SubCameraView = {}


---@class TemporalJitter : ReflectReference
---  A subpixel offset to jitter a perspective camera's frustum by.
--- 
---  Useful for temporal rendering techniques.
--- 
---  Do not use with [`OrthographicProjection`].
--- 
---  [`OrthographicProjection`]: crate::camera::OrthographicProjection
---@field  offset ? Vec2
TemporalJitter = {}


---@class Viewport : ReflectReference
---  Render viewport configuration for the [`Camera`] component.
--- 
---  The viewport defines the area on the render target to which the camera renders its image.
---  You can overlay multiple cameras in a single window using viewports to create effects like
---  split screen, minimaps, and character viewers.
---@field  physical_position ? UVec2
---@field  physical_size ? UVec2
---@field  depth ? Range
Viewport = {}


---@class ClearColor : ReflectReference
---  A [`Resource`] that stores the color that is used to clear the screen between frames.
--- 
---  This color appears as the "background" color for simple apps,
---  when there are portions of the screen with nothing rendered.
---@field  [1] ? Color
ClearColor = {}


---@class ClearColorConfig : ReflectReference
---  For a camera, specifies the color used to clear the viewport before rendering.
ClearColorConfig = {}


---@class ManualTextureViewHandle : ReflectReference
---  A unique id that corresponds to a specific [`ManualTextureView`] in the [`ManualTextureViews`] collection.
---@field  [1] ? integer
ManualTextureViewHandle = {}


---@class CustomProjection : ReflectReference
---  Holds a dynamic [`CameraProjection`] trait object. Use [`Projection::custom()`] to construct a
---  custom projection.
--- 
---  The contained dynamic object can be downcast into a static type using [`CustomProjection::get`].
CustomProjection = {}


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
---  # use bevy_render::camera::{OrthographicProjection, Projection, ScalingMode};
---  let projection = Projection::Orthographic(OrthographicProjection {
---      scaling_mode: ScalingMode::WindowSize,
---      scale: 0.01,
---      ..OrthographicProjection::default_2d()
---  });
---  ```
---@field  near ? number
---@field  far ? number
---@field  viewport_origin ? Vec2
---@field  scaling_mode ? ScalingMode
---@field  scale ? number
---@field  area ? Rect
OrthographicProjection = {}


---@class PerspectiveProjection : ReflectReference
---  A 3D camera projection in which distant objects appear smaller than close objects.
---@field  fov ? number
---@field  aspect_ratio ? number
---@field  near ? number
---@field  far ? number
PerspectiveProjection = {}


---@class Projection : ReflectReference
---  Component that defines how to compute a [`Camera`]'s projection matrix.
--- 
---  Common projections, like perspective and orthographic, are provided out of the box to handle the
---  majority of use cases. Custom projections can be added using the [`CameraProjection`] trait and
---  the [`Projection::custom`] constructor.
--- 
---  ## What's a projection?
--- 
---  A camera projection essentially describes how 3d points from the point of view of a camera are
---  projected onto a 2d screen. This is where properties like a camera's field of view are defined.
---  More specifically, a projection is a 4x4 matrix that transforms points from view space (the
---  point of view of the camera) into clip space. Clip space is almost, but not quite, equivalent to
---  the rectangle that is rendered to your screen, with a depth axis. Any points that land outside
---  the bounds of this cuboid are "clipped" and not rendered.
--- 
---  You can also think of the projection as the thing that describes the shape of a camera's
---  frustum: the volume in 3d space that is visible to a camera.
--- 
---  [`Camera`]: crate::camera::Camera
Projection = {}


---@class OcclusionCulling : ReflectReference
---  Add this component to a view in order to enable experimental GPU occlusion
---  culling.
--- 
---  *Bevy's occlusion culling is currently marked as experimental.* There are
---  known issues whereby, in rare circumstances, occlusion culling can result in
---  meshes being culled that shouldn't be (i.e. meshes that turn invisible).
---  Please try it out and report issues.
--- 
---  *Occlusion culling* allows Bevy to avoid rendering objects that are fully
---  behind other opaque or alpha tested objects. This is different from, and
---  complements, depth fragment rejection as the `DepthPrepass` enables. While
---  depth rejection allows Bevy to avoid rendering *pixels* that are behind
---  other objects, the GPU still has to examine those pixels to reject them,
---  which requires transforming the vertices of the objects and performing
---  skinning if the objects were skinned. Occlusion culling allows the GPU to go
---  a step further, avoiding even transforming the vertices of objects that it
---  can quickly prove to be behind other objects.
--- 
---  Occlusion culling inherently has some overhead, because Bevy must examine
---  the objects' bounding boxes, and create an acceleration structure
---  (hierarchical Z-buffer) to perform the occlusion tests. Therefore, occlusion
---  culling is disabled by default. Only enable it if you measure it to be a
---  speedup on your scene. Note that, because Bevy's occlusion culling runs on
---  the GPU and is quite efficient, it's rare for occlusion culling to result in
---  a significant slowdown.
--- 
---  Occlusion culling currently requires a `DepthPrepass`. If no depth prepass
---  is present on the view, the [`OcclusionCulling`] component will be ignored.
---  Additionally, occlusion culling is currently incompatible with deferred
---  shading; including both `DeferredPrepass` and [`OcclusionCulling`] results
---  in unspecified behavior.
--- 
---  The algorithm that Bevy uses is known as [*two-phase occlusion culling*].
---  When you enable occlusion culling, Bevy splits the depth prepass into two:
---  an *early* depth prepass and a *late* depth prepass. The early depth prepass
---  renders all the meshes that were visible last frame to produce a
---  conservative approximation of the depth buffer. Then, after producing an
---  acceleration structure known as a hierarchical Z-buffer or depth pyramid,
---  Bevy tests the bounding boxes of all meshes against that depth buffer. Those
---  that can be quickly proven to be behind the geometry rendered during the
---  early depth prepass are skipped entirely. The other potentially-visible
---  meshes are rendered during the late prepass, and finally all the visible
---  meshes are rendered as usual during the opaque, transparent, etc. passes.
--- 
---  Unlike other occlusion culling systems you may be familiar with, Bevy's
---  occlusion culling is fully dynamic and requires no baking step. The CPU
---  overhead is minimal. Large skinned meshes and other dynamic objects can
---  occlude other objects.
--- 
---  [*two-phase occlusion culling*]:
---  https://medium.com/@mil_kru/two-pass-occlusion-culling-4100edcad501
OcclusionCulling = {}


---@class GlobalsUniform : ReflectReference
---  Contains global values useful when writing shaders.
---  Currently only contains values related to time.
---@field  time ? number
---@field  delta_time ? number
---@field  frame_count ? integer
GlobalsUniform = {}


---@class Mesh2d : ReflectReference
---  A component for 2D meshes. Requires a [`MeshMaterial2d`] to be rendered, commonly using a [`ColorMaterial`].
--- 
---  [`MeshMaterial2d`]: <https://docs.rs/bevy/latest/bevy/sprite/struct.MeshMaterial2d.html>
---  [`ColorMaterial`]: <https://docs.rs/bevy/latest/bevy/sprite/struct.ColorMaterial.html>
--- 
---  # Example
--- 
---  ```ignore
---  # use bevy_sprite::{ColorMaterial, Mesh2d, MeshMaterial2d};
---  # use bevy_ecs::prelude::*;
---  # use bevy_render::mesh::Mesh;
---  # use bevy_color::palettes::basic::RED;
---  # use bevy_asset::Assets;
---  # use bevy_math::primitives::Circle;
---  #
---  // Spawn an entity with a mesh using `ColorMaterial`.
---  fn setup(
---      mut commands: Commands,
---      mut meshes: ResMut<Assets<Mesh>>,
---      mut materials: ResMut<Assets<ColorMaterial>>,
---  ) {
---      commands.spawn((
---          Mesh2d(meshes.add(Circle::new(50.0))),
---          MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
---      ));
---  }
---  ```
---@field  [1] ? Handle
Mesh2d = {}


---@class Mesh3d : ReflectReference
---  A component for 3D meshes. Requires a [`MeshMaterial3d`] to be rendered, commonly using a [`StandardMaterial`].
--- 
---  [`MeshMaterial3d`]: <https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html>
---  [`StandardMaterial`]: <https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html>
--- 
---  # Example
--- 
---  ```ignore
---  # use bevy_pbr::{Material, MeshMaterial3d, StandardMaterial};
---  # use bevy_ecs::prelude::*;
---  # use bevy_render::mesh::{Mesh, Mesh3d};
---  # use bevy_color::palettes::basic::RED;
---  # use bevy_asset::Assets;
---  # use bevy_math::primitives::Capsule3d;
---  #
---  // Spawn an entity with a mesh using `StandardMaterial`.
---  fn setup(
---      mut commands: Commands,
---      mut meshes: ResMut<Assets<Mesh>>,
---      mut materials: ResMut<Assets<StandardMaterial>>,
---  ) {
---      commands.spawn((
---          Mesh3d(meshes.add(Capsule3d::default())),
---          MeshMaterial3d(materials.add(StandardMaterial {
---              base_color: RED.into(),
---              ..Default::default()
---          })),
---      ));
---  }
---  ```
---@field  [1] ? Handle
Mesh3d = {}


---@class Aabb : ReflectReference
---  An axis-aligned bounding box, defined by:
---  - a center,
---  - the distances from the center to each faces along the axis,
---    the faces are orthogonal to the axis.
--- 
---  It is typically used as a component on an entity to represent the local space
---  occupied by this entity, with faces orthogonal to its local axis.
--- 
---  This component is notably used during "frustum culling", a process to determine
---  if an entity should be rendered by a [`Camera`] if its bounding box intersects
---  with the camera's [`Frustum`].
--- 
---  It will be added automatically by the systems in [`CalculateBounds`] to entities that:
---  - could be subject to frustum culling, for example with a [`Mesh3d`]
---    or `Sprite` component,
---  - don't have the [`NoFrustumCulling`] component.
--- 
---  It won't be updated automatically if the space occupied by the entity changes,
---  for example if the vertex positions of a [`Mesh3d`] are updated.
--- 
---  [`Camera`]: crate::camera::Camera
---  [`NoFrustumCulling`]: crate::view::visibility::NoFrustumCulling
---  [`CalculateBounds`]: crate::view::visibility::VisibilitySystems::CalculateBounds
---  [`Mesh3d`]: crate::mesh::Mesh
---@field  center ? Vec3A
---@field  half_extents ? Vec3A
Aabb = {}


---@class CascadesFrusta : ReflectReference
CascadesFrusta = {}


---@class CubemapFrusta : ReflectReference
CubemapFrusta = {}


---@class Frustum : ReflectReference
---  A region of 3D space defined by the intersection of 6 [`HalfSpace`]s.
--- 
---  Frustums are typically an apex-truncated square pyramid (a pyramid without the top) or a cuboid.
--- 
---  Half spaces are ordered left, right, top, bottom, near, far. The normal vectors
---  of the half-spaces point towards the interior of the frustum.
--- 
---  A frustum component is used on an entity with a [`Camera`] component to
---  determine which entities will be considered for rendering by this camera.
---  All entities with an [`Aabb`] component that are not contained by (or crossing
---  the boundary of) the frustum will not be rendered, and not be used in rendering computations.
--- 
---  This process is called frustum culling, and entities can opt out of it using
---  the [`NoFrustumCulling`] component.
--- 
---  The frustum component is typically added automatically for cameras, either `Camera2d` or `Camera3d`.
---  It is usually updated automatically by [`update_frusta`] from the
---  [`CameraProjection`] component and [`GlobalTransform`] of the camera entity.
--- 
---  [`Camera`]: crate::camera::Camera
---  [`NoFrustumCulling`]: crate::view::visibility::NoFrustumCulling
---  [`update_frusta`]: crate::view::visibility::update_frusta
---  [`CameraProjection`]: crate::camera::CameraProjection
---  [`GlobalTransform`]: bevy_transform::components::GlobalTransform
Frustum = {}


---@class ShaderStorageBuffer : ReflectReference
---  A storage buffer that is prepared as a [`RenderAsset`] and uploaded to the GPU.
ShaderStorageBuffer = {}


---@class SyncToRenderWorld : ReflectReference
---  Marker component that indicates that its entity needs to be synchronized to the render world.
--- 
---  This component is automatically added as a required component by [`ExtractComponentPlugin`] and [`SyncComponentPlugin`].
---  For more information see [`SyncWorldPlugin`].
--- 
---  NOTE: This component should persist throughout the entity's entire lifecycle.
---  If this component is removed from its entity, the entity will be despawned.
--- 
---  [`ExtractComponentPlugin`]: crate::extract_component::ExtractComponentPlugin
---  [`SyncComponentPlugin`]: crate::sync_component::SyncComponentPlugin
SyncToRenderWorld = {}


---@class ColorGrading : ReflectReference
---  Configures filmic color grading parameters to adjust the image appearance.
--- 
---  Color grading is applied just before tonemapping for a given
---  [`Camera`](crate::camera::Camera) entity, with the sole exception of the
---  `post_saturation` value in [`ColorGradingGlobal`], which is applied after
---  tonemapping.
---@field  global ? ColorGradingGlobal
---@field  shadows ? ColorGradingSection
---@field  midtones ? ColorGradingSection
---@field  highlights ? ColorGradingSection
ColorGrading = {}


---@class ColorGradingGlobal : ReflectReference
---  Filmic color grading values applied to the image as a whole (as opposed to
---  individual sections, like shadows and highlights).
---@field  exposure ? number
---@field  temperature ? number
---@field  tint ? number
---@field  hue ? number
---@field  post_saturation ? number
---@field  midtones_range ? Range
ColorGradingGlobal = {}


---@class ColorGradingSection : ReflectReference
---  A section of color grading values that can be selectively applied to
---  shadows, midtones, and highlights.
---@field  saturation ? number
---@field  contrast ? number
---@field  gamma ? number
---@field  gain ? number
---@field  lift ? number
ColorGradingSection = {}


---@class Msaa : ReflectReference
---  Component for configuring the number of samples for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing)
---  for a [`Camera`](crate::camera::Camera).
--- 
---  Defaults to 4 samples. A higher number of samples results in smoother edges.
--- 
---  Some advanced rendering features may require that MSAA is disabled.
--- 
---  Note that the web currently only supports 1 or 4 samples.
Msaa = {}


---@class InheritedVisibility : ReflectReference
---  Whether or not an entity is visible in the hierarchy.
---  This will not be accurate until [`VisibilityPropagate`] runs in the [`PostUpdate`] schedule.
--- 
---  If this is false, then [`ViewVisibility`] should also be false.
--- 
---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate
---@field  [1] ? boolean
InheritedVisibility = {}


---@class NoFrustumCulling : ReflectReference
---  Use this component to opt-out of built-in frustum culling for entities, see
---  [`Frustum`].
--- 
---  It can be used for example:
---  - when a [`Mesh`] is updated but its [`Aabb`] is not, which might happen with animations,
---  - when using some light effects, like wanting a [`Mesh`] out of the [`Frustum`]
---    to appear in the reflection of a [`Mesh`] within.
NoFrustumCulling = {}


---@class ViewVisibility : ReflectReference
---  Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
--- 
---  Each frame, this will be reset to `false` during [`VisibilityPropagate`] systems in [`PostUpdate`].
---  Later in the frame, systems in [`CheckVisibility`] will mark any visible entities using [`ViewVisibility::set`].
---  Because of this, values of this type will be marked as changed every frame, even when they do not change.
--- 
---  If you wish to add custom visibility system that sets this value, make sure you add it to the [`CheckVisibility`] set.
--- 
---  [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate
---  [`CheckVisibility`]: VisibilitySystems::CheckVisibility
---@field  [1] ? boolean
ViewVisibility = {}


---@class Visibility : ReflectReference
---  User indication of whether an entity is visible. Propagates down the entity hierarchy.
--- 
---  If an entity is hidden in this way, all [`Children`] (and all of their children and so on) who
---  are set to [`Inherited`](Self::Inherited) will also be hidden.
--- 
---  This is done by the `visibility_propagate_system` which uses the entity hierarchy and
---  `Visibility` to set the values of each entity's [`InheritedVisibility`] component.
Visibility = {}


---@class VisibilityClass : ReflectReference
---  A bucket into which we group entities for the purposes of visibility.
--- 
---  Bevy's various rendering subsystems (3D, 2D, UI, etc.) want to be able to
---  quickly winnow the set of entities to only those that the subsystem is
---  tasked with rendering, to avoid spending time examining irrelevant entities.
---  At the same time, Bevy wants the [`check_visibility`] system to determine
---  all entities' visibilities at the same time, regardless of what rendering
---  subsystem is responsible for drawing them. Additionally, your application
---  may want to add more types of renderable objects that Bevy determines
---  visibility for just as it does for Bevy's built-in objects.
--- 
---  The solution to this problem is *visibility classes*. A visibility class is
---  a type, typically the type of a component, that represents the subsystem
---  that renders it: for example, `Mesh3d`, `Mesh2d`, and `Sprite`. The
---  [`VisibilityClass`] component stores the visibility class or classes that
---  the entity belongs to. (Generally, an object will belong to only one
---  visibility class, but in rare cases it may belong to multiple.)
--- 
---  When adding a new renderable component, you'll typically want to write an
---  add-component hook that adds the type ID of that component to the
---  [`VisibilityClass`] array. See `custom_phase_item` for an example.
---@field  [1] ? SmallVec
VisibilityClass = {}


---@class VisibleEntities : ReflectReference
---  Collection of entities visible from the current view.
--- 
---  This component contains all entities which are visible from the currently
---  rendered view. The collection is updated automatically by the [`VisibilitySystems::CheckVisibility`]
---  system set. Renderers can use the equivalent [`RenderVisibleEntities`] to optimize rendering of
---  a particular view, to prevent drawing items not visible from that view.
--- 
---  This component is intended to be attached to the same entity as the [`Camera`] and
---  the [`Frustum`] defining the view.
VisibleEntities = {}


---@class VisibilityRange : ReflectReference
---  Specifies the range of distances that this entity must be from the camera in
---  order to be rendered.
--- 
---  This is also known as *hierarchical level of detail* or *HLOD*.
--- 
---  Use this component when you want to render a high-polygon mesh when the
---  camera is close and a lower-polygon mesh when the camera is far away. This
---  is a common technique for improving performance, because fine details are
---  hard to see in a mesh at a distance. To avoid an artifact known as *popping*
---  between levels, each level has a *margin*, within which the object
---  transitions gradually from invisible to visible using a dithering effect.
--- 
---  You can also use this feature to replace multiple meshes with a single mesh
---  when the camera is distant. This is the reason for the term "*hierarchical*
---  level of detail". Reducing the number of meshes can be useful for reducing
---  drawcall count. Note that you must place the [`VisibilityRange`] component
---  on each entity you want to be part of a LOD group, as [`VisibilityRange`]
---  isn't automatically propagated down to children.
--- 
---  A typical use of this feature might look like this:
--- 
---  | Entity                  | `start_margin` | `end_margin` |
---  |-------------------------|----------------|--------------|
---  | Root                    | N/A            | N/A          |
---  | ├─ High-poly mesh       | [0, 0)         | [20, 25)     |
---  | ├─ Low-poly mesh        | [20, 25)       | [70, 75)     |
---  | └─ Billboard *imposter* | [70, 75)       | [150, 160)   |
--- 
---  With this setup, the user will see a high-poly mesh when the camera is
---  closer than 20 units. As the camera zooms out, between 20 units to 25 units,
---  the high-poly mesh will gradually fade to a low-poly mesh. When the camera
---  is 70 to 75 units away, the low-poly mesh will fade to a single textured
---  quad. And between 150 and 160 units, the object fades away entirely. Note
---  that the `end_margin` of a higher LOD is always identical to the
---  `start_margin` of the next lower LOD; this is important for the crossfade
---  effect to function properly.
---@field  start_margin ? Range
---@field  end_margin ? Range
---@field  use_aabb ? boolean
VisibilityRange = {}


---@class RenderLayers : ReflectReference
---  Describes which rendering layers an entity belongs to.
--- 
---  Cameras with this component will only render entities with intersecting
---  layers.
--- 
---  Entities may belong to one or more layers, or no layer at all.
--- 
---  The [`Default`] instance of `RenderLayers` contains layer `0`, the first layer.
--- 
---  An entity with this component without any layers is invisible.
--- 
---  Entities without this component belong to layer `0`.
---@field  [1] ? SmallVec
RenderLayers = {}


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
---@field  [1] ? RenderTarget
Screenshot = {}


---@class ScreenshotCaptured : ReflectReference
---@field  [1] ? Image
ScreenshotCaptured = {}


---@class ColorMaterial : ReflectReference
---  A [2d material](Material2d) that renders [2d meshes](crate::Mesh2d) with a texture tinted by a uniform color
---@field  color ? Color
---@field  alpha_mode ? AlphaMode2d
---@field  uv_transform ? Affine2
---@field  texture ? Option
ColorMaterial = {}


---@class AlphaMode2d : ReflectReference
---  Sets how a 2d material's base color alpha channel is used for transparency.
---  Currently, this only works with [`Mesh2d`]. Sprites are always transparent.
--- 
---  This is very similar to [`AlphaMode`](bevy_render::alpha::AlphaMode) but this only applies to 2d meshes.
---  We use a separate type because 2d doesn't support all the transparency modes that 3d does.
AlphaMode2d = {}


---@class Anchor : ReflectReference
---  How a sprite is positioned relative to its [`Transform`].
---  It defaults to `Anchor::Center`.
Anchor = {}


---@class Sprite : ReflectReference
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
Sprite = {}


---@class SpriteImageMode : ReflectReference
---  Controls how the image is altered when scaled.
SpriteImageMode = {}


---@class BorderRect : ReflectReference
---  Defines the extents of the border of a rectangle.
--- 
---  This struct is used to represent thickness or offsets from the edges
---  of a rectangle (left, right, top, and bottom), with values increasing inwards.
---@field  left ? number
---@field  right ? number
---@field  top ? number
---@field  bottom ? number
BorderRect = {}


---@class SliceScaleMode : ReflectReference
---  Defines how a texture slice scales when resized
SliceScaleMode = {}


---@class TextureSlicer : ReflectReference
---  Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes
---  without needing to prepare multiple assets. The associated texture will be split into nine portions,
---  so that on resize the different portions scale or tile in different ways to keep the texture in proportion.
--- 
---  For example, when resizing a 9-sliced texture the corners will remain unscaled while the other
---  sections will be scaled or tiled.
--- 
---  See [9-sliced](https://en.wikipedia.org/wiki/9-slice_scaling) textures.
---@field  border ? BorderRect
---@field  center_scale_mode ? SliceScaleMode
---@field  sides_scale_mode ? SliceScaleMode
---@field  max_corner_scale ? number
TextureSlicer = {}


---@class ReflectableScheduleLabel : ReflectReference
ReflectableScheduleLabel = {}


---@class TextBounds : ReflectReference
---  The maximum width and height of text. The text will wrap according to the specified size.
--- 
---  Characters out of the bounds after wrapping will be truncated. Text is aligned according to the
---  specified [`JustifyText`](crate::text::JustifyText).
--- 
---  Note: only characters that are completely out of the bounds will be truncated, so this is not a
---  reliable limit if it is necessary to contain the text strictly in the bounds. Currently this
---  component is mainly useful for text wrapping only.
---@field  width ? Option
---@field  height ? Option
TextBounds = {}


---@class GlyphAtlasInfo : ReflectReference
---  Information about a glyph in an atlas.
--- 
---  Rasterized glyphs are stored as rectangles
---  in one or more [`FontAtlas`](crate::FontAtlas)es.
--- 
---  Used in [`PositionedGlyph`] and [`FontAtlasSet`](crate::FontAtlasSet).
---@field  texture ? Handle
---@field  texture_atlas ? Handle
---@field  location ? GlyphAtlasLocation
GlyphAtlasInfo = {}


---@class GlyphAtlasLocation : ReflectReference
---  The location of a glyph in an atlas,
---  and how it should be positioned when placed.
--- 
---  Used in [`GlyphAtlasInfo`] and [`FontAtlas`](crate::FontAtlas).
---@field  glyph_index ? integer
---@field  offset ? IVec2
GlyphAtlasLocation = {}


---@class PositionedGlyph : ReflectReference
---  A glyph of a font, typically representing a single character, positioned in screen space.
--- 
---  Contains information about how and where to render a glyph.
--- 
---  Used in [`TextPipeline::queue_text`](crate::TextPipeline::queue_text) and [`crate::TextLayoutInfo`] for rendering glyphs.
---@field  position ? Vec2
---@field  size ? Vec2
---@field  atlas_info ? GlyphAtlasInfo
---@field  span_index ? integer
---@field  line_index ? integer
---@field  byte_index ? integer
---@field  byte_length ? integer
PositionedGlyph = {}


---@class TextLayoutInfo : ReflectReference
---  Render information for a corresponding text block.
--- 
---  Contains scaled glyphs and their size. Generated via [`TextPipeline::queue_text`] when an entity has
---  [`TextLayout`] and [`ComputedTextBlock`] components.
---@field  glyphs ? Vec
---@field  size ? Vec2
TextLayoutInfo = {}


---@class Text2d : ReflectReference
---  The top-level 2D text component.
--- 
---  Adding `Text2d` to an entity will pull in required components for setting up 2d text.
---  [Example usage.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/text2d.rs)
--- 
---  The string in this component is the first 'text span' in a hierarchy of text spans that are collected into
---  a [`ComputedTextBlock`]. See [`TextSpan`](crate::TextSpan) for the component used by children of entities with [`Text2d`].
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
---  # use bevy_text::{Font, JustifyText, Text2d, TextLayout, TextFont, TextColor, TextSpan};
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
---          font_size: 60.0,
---          ..Default::default()
---      },
---      TextColor(BLUE.into()),
---  ));
--- 
---  // With text justification.
---  world.spawn((
---      Text2d::new("hello world\nand bevy!"),
---      TextLayout::new_with_justify(JustifyText::Center)
---  ));
--- 
---  // With spans
---  world.spawn(Text2d::new("hello ")).with_children(|parent| {
---      parent.spawn(TextSpan::new("world"));
---      parent.spawn((TextSpan::new("!"), TextColor(BLUE.into())));
---  });
---  ```
---@field  [1] ? string
Text2d = {}


---@class ComputedTextBlock : ReflectReference
---  Computed information for a text block.
--- 
---  See [`TextLayout`].
--- 
---  Automatically updated by 2d and UI text systems.
---@field  entities ? SmallVec
---@field  needs_rerender ? boolean
ComputedTextBlock = {}


---@class FontSmoothing : ReflectReference
---  Determines which antialiasing method to use when rendering text. By default, text is
---  rendered with grayscale antialiasing, but this can be changed to achieve a pixelated look.
--- 
---  **Note:** Subpixel antialiasing is not currently supported.
FontSmoothing = {}


---@class JustifyText : ReflectReference
---  Describes the horizontal alignment of multiple lines of text relative to each other.
--- 
---  This only affects the internal positioning of the lines of text within a text entity and
---  does not affect the text entity's position.
--- 
---  _Has no affect on a single line text entity_, unless used together with a
---  [`TextBounds`](super::bounds::TextBounds) component with an explicit `width` value.
JustifyText = {}


---@class LineBreak : ReflectReference
---  Determines how lines will be broken when preventing text from running out of bounds.
LineBreak = {}


---@class LineHeight : ReflectReference
---  Specifies the height of each line of text for `Text` and `Text2d`
--- 
---  Default is 1.2x the font size
LineHeight = {}


---@class TextColor : ReflectReference
---  The color of the text for this section.
---@field  [1] ? Color
TextColor = {}


---@class TextEntity : ReflectReference
---  A sub-entity of a [`ComputedTextBlock`].
--- 
---  Returned by [`ComputedTextBlock::entities`].
---@field  entity ? Entity
---@field  depth ? integer
TextEntity = {}


---@class TextFont : ReflectReference
---  `TextFont` determines the style of a text span within a [`ComputedTextBlock`], specifically
---  the font face, the font size, and the color.
---@field  font ? bevy_asset::handle::Handle<bevy_text::font::Font>
---@field  font_size ? number
---@field  line_height ? LineHeight
---@field  font_smoothing ? FontSmoothing
TextFont = {}


---@class TextLayout : ReflectReference
---  Component with text format settings for a block of text.
--- 
---  A block of text is composed of text spans, which each have a separate string value and [`TextFont`]. Text
---  spans associated with a text block are collected into [`ComputedTextBlock`] for layout, and then inserted
---  to [`TextLayoutInfo`] for rendering.
--- 
---  See [`Text2d`](crate::Text2d) for the core component of 2d text, and `Text` in `bevy_ui` for UI text.
---@field  justify ? JustifyText
---@field  linebreak ? LineBreak
TextLayout = {}


---@class TextSpan : ReflectReference
---  A span of text in a tree of spans.
--- 
---  `TextSpan` is only valid as a child of an entity with [`TextLayout`], which is provided by `Text`
---  for text in `bevy_ui` or `Text2d` for text in 2d world-space.
--- 
---  Spans are collected in hierarchy traversal order into a [`ComputedTextBlock`] for layout.
--- 
---  ```
---  # use bevy_asset::Handle;
---  # use bevy_color::Color;
---  # use bevy_color::palettes::basic::{RED, BLUE};
---  # use bevy_ecs::world::World;
---  # use bevy_text::{Font, TextLayout, TextFont, TextSpan, TextColor};
--- 
---  # let font_handle: Handle<Font> = Default::default();
---  # let mut world = World::default();
---  #
---  world.spawn((
---      // `Text` or `Text2d` are needed, and will provide default instances
---      // of the following components.
---      TextLayout::default(),
---      TextFont {
---          font: font_handle.clone().into(),
---          font_size: 60.0,
---          ..Default::default()
---      },
---      TextColor(BLUE.into()),
---  ))
---  .with_child((
---      // Children must be `TextSpan`, not `Text` or `Text2d`.
---      TextSpan::new("Hello!"),
---      TextFont {
---          font: font_handle.into(),
---          font_size: 60.0,
---          ..Default::default()
---      },
---      TextColor(RED.into()),
---  ));
---  ```
---@field  [1] ? string
TextSpan = {}


---@class UiScale : ReflectReference
---  The current scale of the UI.
--- 
---  A multiplier to fixed-sized ui values.
---  **Note:** This will only affect fixed ui values like [`Val::Px`]
---@field  [1] ? number
UiScale = {}


---@class FocusPolicy : ReflectReference
---  Describes whether the node should block interactions with lower nodes
FocusPolicy = {}


---@class Interaction : ReflectReference
---  Describes what type of input interaction has occurred for a UI node.
--- 
---  This is commonly queried with a `Changed<Interaction>` filter.
--- 
---  Updated in [`ui_focus_system`].
--- 
---  If a UI node has both [`Interaction`] and [`InheritedVisibility`] components,
---  [`Interaction`] will always be [`Interaction::None`]
---  when [`InheritedVisibility::get()`] is false.
---  This ensures that hidden UI nodes are not interactable,
---  and do not end up stuck in an active state if hidden at the wrong time.
--- 
---  Note that you can also control the visibility of a node using the [`Display`](crate::ui_node::Display) property,
---  which fully collapses it during layout calculations.
--- 
---  # See also
--- 
---  - [`Button`](crate::widget::Button) which requires this component
---  - [`RelativeCursorPosition`] to obtain the position of the cursor relative to current node
Interaction = {}


---@class RelativeCursorPosition : ReflectReference
---  A component storing the position of the mouse relative to the node, (0., 0.) being the top-left corner and (1., 1.) being the bottom-right
---  If the mouse is not over the node, the value will go beyond the range of (0., 0.) to (1., 1.)
--- 
---  It can be used alongside [`Interaction`] to get the position of the press.
--- 
---  The component is updated when it is in the same entity with [`Node`](crate::Node).
---@field  normalized_visible_node_rect ? Rect
---@field  normalized ? Option
RelativeCursorPosition = {}


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
---@field  left ? Val
---@field  right ? Val
---@field  top ? Val
---@field  bottom ? Val
UiRect = {}


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
Val = {}


---@class ContentSize : ReflectReference
---  A node with a `ContentSize` component is a node where its size
---  is based on its content.
ContentSize = {}


---@class AlignContent : ReflectReference
---  Used to control how items are distributed.
---  - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
---  - For CSS Grid containers, controls alignment of grid rows.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
AlignContent = {}


---@class AlignItems : ReflectReference
---  Used to control how each individual item is aligned by default within the space they're given.
---  - For Flexbox containers, sets default cross axis alignment of the child items.
---  - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
AlignItems = {}


---@class AlignSelf : ReflectReference
---  Used to control how the specified item is aligned within the space it's given.
---  - For Flexbox items, controls cross axis alignment of the item.
---  - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
AlignSelf = {}


---@class BackgroundColor : ReflectReference
---  The background color of the node
--- 
---  This serves as the "fill" color.
---@field  [1] ? Color
BackgroundColor = {}


---@class BorderColor : ReflectReference
---  The border color of the UI node.
---@field  [1] ? Color
BorderColor = {}


---@class BorderRadius : ReflectReference
---  Used to add rounded corners to a UI node. You can set a UI node to have uniformly
---  rounded corners or specify different radii for each corner. If a given radius exceeds half
---  the length of the smallest dimension between the node's height or width, the radius will
---  calculated as half the smallest dimension.
--- 
---  Elliptical nodes are not supported yet. Percentage values are based on the node's smallest
---  dimension, either width or height.
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
---              ..Default::default()
---          },
---          BackgroundColor(BLUE.into()),
---          BorderRadius::new(
---              // top left
---              Val::Px(10.),
---              // top right
---              Val::Px(20.),
---              // bottom right
---              Val::Px(30.),
---              // bottom left
---              Val::Px(40.),
---          ),
---      ));
---  }
---  ```
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
---@field  top_left ? Val
---@field  top_right ? Val
---@field  bottom_left ? Val
---@field  bottom_right ? Val
BorderRadius = {}


---@class BoxShadow : ReflectReference
---  List of shadows to draw for a [`Node`].
--- 
---  Draw order is determined implicitly from the vector of [`ShadowStyle`]s, back-to-front.
---@field  [1] ? Vec
BoxShadow = {}


---@class BoxShadowSamples : ReflectReference
---  Number of shadow samples.
---  A larger value will result in higher quality shadows.
---  Default is 4, values higher than ~10 offer diminishing returns.
--- 
---  ```
---  use bevy_core_pipeline::prelude::*;
---  use bevy_ecs::prelude::*;
---  use bevy_ui::prelude::*;
--- 
---  fn spawn_camera(mut commands: Commands) {
---      commands.spawn((
---          Camera2d,
---          BoxShadowSamples(6),
---      ));
---  }
---  ```
---@field  [1] ? integer
BoxShadowSamples = {}


---@class BoxSizing : ReflectReference
---  Which part of a Node's box length styles like width and height control
--- 
---  See: <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
BoxSizing = {}


---@class CalculatedClip : ReflectReference
---  The calculated clip of the node
---@field  clip ? Rect
CalculatedClip = {}


---@class ComputedNode : ReflectReference
---  Provides the computed size and layout properties of the node.
--- 
---  Fields in this struct are public but should not be modified under most circumstances.
---  For example, in a scrollbar you may want to derive the handle's size from the proportion of
---  scrollable content in-view. You can directly modify `ComputedNode` after layout to set the
---  handle size without any delays.
---@field  stack_index ? integer
---@field  size ? Vec2
---@field  content_size ? Vec2
---@field  outline_width ? number
---@field  outline_offset ? number
---@field  unrounded_size ? Vec2
---@field  border ? BorderRect
---@field  border_radius ? ResolvedBorderRadius
---@field  padding ? BorderRect
---@field  inverse_scale_factor ? number
ComputedNode = {}


---@class ComputedNodeTarget : ReflectReference
---  Derived information about the camera target for this UI node.
---@field  camera ? Entity
---@field  scale_factor ? number
---@field  physical_size ? UVec2
ComputedNodeTarget = {}


---@class Display : ReflectReference
---  Defines the layout model used by this node.
--- 
---  Part of the [`Node`] component.
Display = {}


---@class FlexDirection : ReflectReference
---  Defines how flexbox items are ordered within a flexbox
FlexDirection = {}


---@class FlexWrap : ReflectReference
---  Defines if flexbox items appear on a single line or on multiple lines
FlexWrap = {}


---@class GridAutoFlow : ReflectReference
---  Controls whether grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.
--- 
---  The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later.
---  This may cause items to appear out-of-order when doing so would fill in holes left by larger items.
--- 
---  Defaults to [`GridAutoFlow::Row`].
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
GridAutoFlow = {}


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
---@field  start ? Option
---@field  span ? Option
---@field  end ? Option
GridPlacement = {}


---@class GridTrack : ReflectReference
---  A [`GridTrack`] is a Row or Column of a CSS Grid. This struct specifies what size the track should be.
---  See below for the different "track sizing functions" you can specify.
---@field  min_sizing_function ? MinTrackSizingFunction
---@field  max_sizing_function ? MaxTrackSizingFunction
GridTrack = {}


---@class GridTrackRepetition : ReflectReference
---  How many times to repeat a repeated grid track
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/repeat>
GridTrackRepetition = {}


---@class JustifyContent : ReflectReference
---  Used to control how items are distributed.
---  - For Flexbox containers, controls alignment of items in the main axis.
---  - For CSS Grid containers, controls alignment of grid columns.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
JustifyContent = {}


---@class JustifyItems : ReflectReference
---  Used to control how each individual item is aligned by default within the space they're given.
---  - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
---  - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
JustifyItems = {}


---@class JustifySelf : ReflectReference
---  Used to control how the specified item is aligned within the space it's given.
---  - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
---  - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
--- 
---  <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
JustifySelf = {}


---@class MaxTrackSizingFunction : ReflectReference
MaxTrackSizingFunction = {}


---@class MinTrackSizingFunction : ReflectReference
MinTrackSizingFunction = {}


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
---  - [`Interaction`](crate::Interaction) to obtain the interaction state of this node
---@field  display ? Display
---@field  box_sizing ? BoxSizing
---@field  position_type ? PositionType
---@field  overflow ? Overflow
---@field  overflow_clip_margin ? OverflowClipMargin
---@field  left ? Val
---@field  right ? Val
---@field  top ? Val
---@field  bottom ? Val
---@field  width ? Val
---@field  height ? Val
---@field  min_width ? Val
---@field  min_height ? Val
---@field  max_width ? Val
---@field  max_height ? Val
---@field  aspect_ratio ? Option
---@field  align_items ? AlignItems
---@field  justify_items ? JustifyItems
---@field  align_self ? AlignSelf
---@field  justify_self ? JustifySelf
---@field  align_content ? AlignContent
---@field  justify_content ? JustifyContent
---@field  margin ? UiRect
---@field  padding ? UiRect
---@field  border ? UiRect
---@field  flex_direction ? FlexDirection
---@field  flex_wrap ? FlexWrap
---@field  flex_grow ? number
---@field  flex_shrink ? number
---@field  flex_basis ? Val
---@field  row_gap ? Val
---@field  column_gap ? Val
---@field  grid_auto_flow ? GridAutoFlow
---@field  grid_template_rows ? Vec
---@field  grid_template_columns ? Vec
---@field  grid_auto_rows ? Vec
---@field  grid_auto_columns ? Vec
---@field  grid_row ? GridPlacement
---@field  grid_column ? GridPlacement
Node = {}


---@class Outline : ReflectReference
---  The [`Outline`] component adds an outline outside the edge of a UI node.
---  Outlines do not take up space in the layout.
--- 
---  To add an [`Outline`] to a ui node you can spawn a `(Node, Outline)` tuple bundle:
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
---  # use bevy_color::Color;
---  fn outline_hovered_button_system(
---      mut commands: Commands,
---      mut node_query: Query<(Entity, &Interaction, Option<&mut Outline>), Changed<Interaction>>,
---  ) {
---      for (entity, interaction, mut maybe_outline) in node_query.iter_mut() {
---          let outline_color =
---              if matches!(*interaction, Interaction::Hovered) {
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
---@field  width ? Val
---@field  offset ? Val
---@field  color ? Color
Outline = {}


---@class Overflow : ReflectReference
---  Whether to show or hide overflowing items
---@field  x ? OverflowAxis
---@field  y ? OverflowAxis
Overflow = {}


---@class OverflowAxis : ReflectReference
---  Whether to show or hide overflowing items
OverflowAxis = {}


---@class OverflowClipBox : ReflectReference
---  Used to determine the bounds of the visible area when a UI node is clipped.
OverflowClipBox = {}


---@class OverflowClipMargin : ReflectReference
---  The bounds of the visible area when a UI node is clipped.
---@field  visual_box ? OverflowClipBox
---@field  margin ? number
OverflowClipMargin = {}


---@class PositionType : ReflectReference
---  The strategy used to position this node
PositionType = {}


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
---@field  repetition ? GridTrackRepetition
---@field  tracks ? SmallVec
RepeatedGridTrack = {}


---@class ResolvedBorderRadius : ReflectReference
---  Represents the resolved border radius values for a UI node.
--- 
---  The values are in physical pixels.
---@field  top_left ? number
---@field  top_right ? number
---@field  bottom_left ? number
---@field  bottom_right ? number
ResolvedBorderRadius = {}


---@class ScrollPosition : ReflectReference
---  The scroll position of the node.
--- 
---  Updating the values of `ScrollPosition` will reposition the children of the node by the offset amount.
---  `ScrollPosition` may be updated by the layout system when a layout change makes a previously valid `ScrollPosition` invalid.
---  Changing this does nothing on a `Node` without setting at least one `OverflowAxis` to `OverflowAxis::Scroll`.
---@field  offset_x ? number
---@field  offset_y ? number
ScrollPosition = {}


---@class ShadowStyle : ReflectReference
---@field  color ? Color
---@field  x_offset ? Val
---@field  y_offset ? Val
---@field  spread_radius ? Val
---@field  blur_radius ? Val
ShadowStyle = {}


---@class TextShadow : ReflectReference
---  Adds a shadow behind text
---@field  offset ? Vec2
---@field  color ? Color
TextShadow = {}


---@class UiAntiAlias : ReflectReference
---  Marker for controlling whether Ui is rendered with or without anti-aliasing
---  in a camera. By default, Ui is always anti-aliased.
--- 
---  **Note:** This does not affect text anti-aliasing. For that, use the `font_smoothing` property of the [`TextFont`](bevy_text::TextFont) component.
--- 
---  ```
---  use bevy_core_pipeline::prelude::*;
---  use bevy_ecs::prelude::*;
---  use bevy_ui::prelude::*;
--- 
---  fn spawn_camera(mut commands: Commands) {
---      commands.spawn((
---          Camera2d,
---          // This will cause all Ui in this camera to be rendered without
---          // anti-aliasing
---          UiAntiAlias::Off,
---      ));
---  }
---  ```
UiAntiAlias = {}


---@class UiTargetCamera : ReflectReference
---  Indicates that this root [`Node`] entity should be rendered to a specific camera.
--- 
---  UI then will be laid out respecting the camera's viewport and scale factor, and
---  rendered to this camera's [`bevy_render::camera::RenderTarget`].
--- 
---  Setting this component on a non-root node will have no effect. It will be overridden
---  by the root node's component.
--- 
---  Root node's without an explicit [`UiTargetCamera`] will be rendered to the default UI camera,
---  which is either a single camera with the [`IsDefaultUiCamera`] marker component or the highest
---  order camera targeting the primary window.
---@field  [1] ? Entity
UiTargetCamera = {}


---@class ZIndex : ReflectReference
---  Indicates that this [`Node`] entity's front-to-back ordering is not controlled solely
---  by its location in the UI hierarchy. A node with a higher z-index will appear on top
---  of sibling nodes with a lower z-index.
--- 
---  UI nodes that have the same z-index will appear according to the order in which they
---  appear in the UI hierarchy. In such a case, the last node to be added to its parent
---  will appear in front of its siblings.
--- 
---  Nodes without this component will be treated as if they had a value of [`ZIndex(0)`].
--- 
---  Use [`GlobalZIndex`] if you need to order separate UI hierarchies or nodes that are
---  not siblings in a given UI hierarchy.
---@field  [1] ? integer
ZIndex = {}


---@class Button : ReflectReference
---  Marker struct for buttons
Button = {}


---@class ImageNode : ReflectReference
---  A UI Node that renders an image.
---@field  color ? Color
---@field  image ? Handle
---@field  texture_atlas ? Option
---@field  flip_x ? boolean
---@field  flip_y ? boolean
---@field  rect ? Option
---@field  image_mode ? NodeImageMode
ImageNode = {}


---@class ImageNodeSize : ReflectReference
---  The size of the image's texture
--- 
---  This component is updated automatically by [`update_image_content_size_system`]
---@field  size ? UVec2
ImageNodeSize = {}


---@class NodeImageMode : ReflectReference
---  Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image
NodeImageMode = {}


---@class Label : ReflectReference
---  Marker struct for labels
Label = {}


---@class Text : ReflectReference
---  The top-level UI text component.
--- 
---  Adding [`Text`] to an entity will pull in required components for setting up a UI text node.
--- 
---  The string in this component is the first 'text span' in a hierarchy of text spans that are collected into
---  a [`ComputedTextBlock`]. See [`TextSpan`](bevy_text::TextSpan) for the component used by children of entities with [`Text`].
--- 
---  Note that [`Transform`](bevy_transform::components::Transform) on this entity is managed automatically by the UI layout system.
--- 
--- 
---  ```
---  # use bevy_asset::Handle;
---  # use bevy_color::Color;
---  # use bevy_color::palettes::basic::BLUE;
---  # use bevy_ecs::world::World;
---  # use bevy_text::{Font, JustifyText, TextLayout, TextFont, TextColor, TextSpan};
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
---          font_size: 60.0,
---          ..Default::default()
---      },
---      TextColor(BLUE.into()),
---  ));
--- 
---  // With text justification.
---  world.spawn((
---      Text::new("hello world\nand bevy!"),
---      TextLayout::new_with_justify(JustifyText::Center)
---  ));
--- 
---  // With spans
---  world.spawn(Text::new("hello ")).with_children(|parent| {
---      parent.spawn(TextSpan::new("world"));
---      parent.spawn((TextSpan::new("!"), TextColor(BLUE.into())));
---  });
---  ```
---@field  [1] ? string
Text = {}


---@class TextNodeFlags : ReflectReference
---  UI text system flags.
--- 
---  Used internally by [`measure_text_system`] and [`text_system`] to schedule text for processing.
---@field  needs_measure_fn ? boolean
---@field  needs_recompute ? boolean
TextNodeFlags = {}


---@class AppLifecycle : ReflectReference
---  Application lifetime events
AppLifecycle = {}


---@class CursorEntered : ReflectReference
---  An event that is sent whenever the user's cursor enters a window.
---@field  window ? Entity
CursorEntered = {}


---@class CursorLeft : ReflectReference
---  An event that is sent whenever the user's cursor leaves a window.
---@field  window ? Entity
CursorLeft = {}


---@class CursorMoved : ReflectReference
---  An event reporting that the mouse cursor has moved inside a window.
--- 
---  The event is sent only if the cursor is over one of the application's windows.
---  It is the translated version of [`WindowEvent::CursorMoved`] from the `winit` crate with the addition of `delta`.
--- 
---  Not to be confused with the `MouseMotion` event from `bevy_input`.
--- 
---  Because the range of data is limited by the window area and it may have been transformed by the OS to implement certain effects like acceleration,
---  you should not use it for non-cursor-like behavior such as 3D camera control. Please see `MouseMotion` instead.
--- 
---  [`WindowEvent::CursorMoved`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
---@field  window ? Entity
---@field  position ? Vec2
---@field  delta ? Option
CursorMoved = {}


---@class FileDragAndDrop : ReflectReference
---  Events related to files being dragged and dropped on a window.
FileDragAndDrop = {}


---@class Ime : ReflectReference
---  An Input Method Editor event.
--- 
---  This event is the translated version of the `WindowEvent::Ime` from the `winit` crate.
--- 
---  It is only sent if IME was enabled on the window with [`Window::ime_enabled`](crate::window::Window::ime_enabled).
Ime = {}


---@class RequestRedraw : ReflectReference
---  An event that indicates all of the application's windows should be redrawn,
---  even if their control flow is set to `Wait` and there have been no window events.
RequestRedraw = {}


---@class WindowBackendScaleFactorChanged : ReflectReference
---  An event that indicates a window's OS-reported scale factor has changed.
---@field  window ? Entity
---@field  scale_factor ? number
WindowBackendScaleFactorChanged = {}


---@class WindowCloseRequested : ReflectReference
---  An event that is sent whenever the operating systems requests that a window
---  be closed. This will be sent when the close button of the window is pressed.
--- 
---  If the default [`WindowPlugin`] is used, these events are handled
---  by closing the corresponding [`Window`].
---  To disable this behavior, set `close_when_requested` on the [`WindowPlugin`]
---  to `false`.
--- 
---  [`WindowPlugin`]: crate::WindowPlugin
---  [`Window`]: crate::Window
---@field  window ? Entity
WindowCloseRequested = {}


---@class WindowClosed : ReflectReference
---  An event that is sent whenever a window is closed. This will be sent when
---  the window entity loses its [`Window`](crate::window::Window) component or is despawned.
---@field  window ? Entity
WindowClosed = {}


---@class WindowClosing : ReflectReference
---  An event that is sent whenever a window is closing. This will be sent when
---  after a [`WindowCloseRequested`] event is received and the window is in the process of closing.
---@field  window ? Entity
WindowClosing = {}


---@class WindowCreated : ReflectReference
---  An event that is sent whenever a new window is created.
--- 
---  To create a new window, spawn an entity with a [`crate::Window`] on it.
---@field  window ? Entity
WindowCreated = {}


---@class WindowDestroyed : ReflectReference
---  An event that is sent whenever a window is destroyed by the underlying window system.
--- 
---  Note that if your application only has a single window, this event may be your last chance to
---  persist state before the application terminates.
---@field  window ? Entity
WindowDestroyed = {}


---@class WindowEvent : ReflectReference
---  Wraps all `bevy_window` and `bevy_input` events in a common enum.
--- 
---  Read these events with `EventReader<WindowEvent>` if you need to
---  access window events in the order they were received from the
---  operating system. Otherwise, the event types are individually
---  readable with `EventReader<E>` (e.g. `EventReader<KeyboardInput>`).
WindowEvent = {}


---@class WindowFocused : ReflectReference
---  An event that indicates a window has received or lost focus.
---@field  window ? Entity
---@field  focused ? boolean
WindowFocused = {}


---@class WindowMoved : ReflectReference
---  An event that is sent when a window is repositioned in physical pixels.
---@field  window ? Entity
---@field  position ? IVec2
WindowMoved = {}


---@class WindowOccluded : ReflectReference
---  The window has been occluded (completely hidden from view).
--- 
---  This is different to window visibility as it depends on
---  whether the window is closed, minimized, set invisible,
---  or fully occluded by another window.
--- 
---  It is the translated version of [`WindowEvent::Occluded`] from the `winit` crate.
--- 
---  [`WindowEvent::Occluded`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.Occluded
---@field  window ? Entity
---@field  occluded ? boolean
WindowOccluded = {}


---@class WindowResized : ReflectReference
---  A window event that is sent whenever a window's logical size has changed.
---@field  window ? Entity
---@field  width ? number
---@field  height ? number
WindowResized = {}


---@class WindowScaleFactorChanged : ReflectReference
---  An event that indicates a window's scale factor has changed.
---@field  window ? Entity
---@field  scale_factor ? number
WindowScaleFactorChanged = {}


---@class WindowThemeChanged : ReflectReference
---  An event sent when the system theme changes for a window.
--- 
---  This event is only sent when the window is relying on the system theme to control its appearance.
---  i.e. It is only sent when [`Window::window_theme`](crate::window::Window::window_theme) is `None` and the system theme changes.
---@field  window ? Entity
---@field  theme ? WindowTheme
WindowThemeChanged = {}


---@class Monitor : ReflectReference
---  Represents an available monitor as reported by the user's operating system, which can be used
---  to query information about the display, such as its size, position, and video modes.
--- 
---  Each monitor corresponds to an entity and can be used to position a monitor using
---  [`crate::window::MonitorSelection::Entity`].
--- 
---  # Warning
--- 
---  This component is synchronized with `winit` through `bevy_winit`, but is effectively
---  read-only as `winit` does not support changing monitor properties.
---@field  name ? Option
---@field  physical_height ? integer
---@field  physical_width ? integer
---@field  physical_position ? IVec2
---@field  refresh_rate_millihertz ? Option
---@field  scale_factor ? number
---@field  video_modes ? Vec
Monitor = {}


---@class VideoMode : ReflectReference
---  Represents a video mode that a monitor supports
---@field  physical_size ? UVec2
---@field  bit_depth ? integer
---@field  refresh_rate_millihertz ? integer
VideoMode = {}


---@class SystemCursorIcon : ReflectReference
---  The icon to display for a window.
--- 
---  Examples of all of these cursors can be found [here](https://www.w3schools.com/cssref/playit.php?filename=playcss_cursor&preval=crosshair).
---  This `enum` is simply a copy of a similar `enum` found in [`winit`](https://docs.rs/winit/latest/winit/window/enum.CursorIcon.html).
---  `winit`, in turn, is based upon the [CSS3 UI spec](https://www.w3.org/TR/css-ui-3/#cursor).
--- 
---  See the [`window_settings`] example for usage.
--- 
---  [`window_settings`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
SystemCursorIcon = {}


---@class CompositeAlphaMode : ReflectReference
---  Specifies how the alpha channel of the textures should be handled during compositing, for a [`Window`].
CompositeAlphaMode = {}


---@class CursorGrabMode : ReflectReference
---  Defines if and how the cursor is grabbed by a [`Window`].
--- 
---  ## Platform-specific
--- 
---  - **`Windows`** doesn't support [`CursorGrabMode::Locked`]
---  - **`macOS`** doesn't support [`CursorGrabMode::Confined`]
---  - **`iOS/Android`** don't have cursors.
--- 
---  Since `Windows` and `macOS` have different [`CursorGrabMode`] support, we first try to set the grab mode that was asked for. If it doesn't work then use the alternate grab mode.
CursorGrabMode = {}


---@class CursorOptions : ReflectReference
---  Cursor data for a [`Window`].
---@field  visible ? boolean
---@field  grab_mode ? CursorGrabMode
---@field  hit_test ? boolean
CursorOptions = {}


---@class EnabledButtons : ReflectReference
---  Specifies which [`Window`] control buttons should be enabled.
--- 
---  ## Platform-specific
--- 
---  **`iOS`**, **`Android`**, and the **`Web`** do not have window control buttons.
--- 
---  On some **`Linux`** environments these values have no effect.
---@field  minimize ? boolean
---@field  maximize ? boolean
---@field  close ? boolean
EnabledButtons = {}


---@class InternalWindowState : ReflectReference
---  Stores internal [`Window`] state that isn't directly accessible.
---@field  minimize_request ? Option
---@field  maximize_request ? Option
---@field  drag_move_request ? boolean
---@field  drag_resize_request ? Option
---@field  physical_cursor_position ? Option
InternalWindowState = {}


---@class MonitorSelection : ReflectReference
---  References a screen monitor.
--- 
---  Used when centering a [`Window`] on a monitor.
MonitorSelection = {}


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
PresentMode = {}


---@class PrimaryWindow : ReflectReference
---  Marker [`Component`] for the window considered the primary window.
--- 
---  Currently this is assumed to only exist on 1 entity at a time.
--- 
---  [`WindowPlugin`](crate::WindowPlugin) will spawn a [`Window`] entity
---  with this component if [`primary_window`](crate::WindowPlugin::primary_window)
---  is `Some`.
PrimaryWindow = {}


---@class VideoModeSelection : ReflectReference
---  References an exclusive fullscreen video mode.
--- 
---  Used when setting [`WindowMode::Fullscreen`] on a window.
VideoModeSelection = {}


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
Window = {}


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
WindowLevel = {}


---@class WindowMode : ReflectReference
---  Defines the way a [`Window`] is displayed.
WindowMode = {}


---@class WindowPosition : ReflectReference
---  Defines where a [`Window`] should be placed on the screen.
WindowPosition = {}


---@class WindowRef : ReflectReference
---  Reference to a [`Window`], whether it be a direct link to a specific entity or
---  a more vague defaulting choice.
WindowRef = {}


---@class WindowResizeConstraints : ReflectReference
---  The size limits on a [`Window`].
--- 
---  These values are measured in logical pixels (see [`WindowResolution`]), so the user's
---  scale factor does affect the size limits on the window.
--- 
---  Please note that if the window is resizable, then when the window is
---  maximized it may have a size outside of these limits. The functionality
---  required to disable maximizing is not yet exposed by winit.
---@field  min_width ? number
---@field  min_height ? number
---@field  max_width ? number
---@field  max_height ? number
WindowResizeConstraints = {}


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
---@field  physical_width ? integer
---@field  physical_height ? integer
---@field  scale_factor_override ? Option
---@field  scale_factor ? number
WindowResolution = {}


---@class WindowTheme : ReflectReference
---  The [`Window`] theme variant to use.
WindowTheme = {}


---@class CursorIcon : ReflectReference
---  Insert into a window entity to set the cursor for that window.
CursorIcon = {}


---@class bool : ReflectReference
--- A boolean value
bool = {}


---@class char : ReflectReference
--- An 8-bit character
char = {}


---@class NonZeroI16 : ReflectReference
NonZeroI16 = {}


---@class NonZeroU16 : ReflectReference
NonZeroU16 = {}


---@class NonZeroU32 : ReflectReference
NonZeroU32 = {}


---@class f32 : ReflectReference
--- A 32-bit floating point number
f32 = {}


---@class f64 : ReflectReference
--- A 64-bit floating point number
f64 = {}


---@class i128 : ReflectReference
--- A signed 128-bit integer
i128 = {}


---@class i16 : ReflectReference
--- A signed 16-bit integer
i16 = {}


---@class i32 : ReflectReference
--- A signed 32-bit integer
i32 = {}


---@class i64 : ReflectReference
--- A signed 64-bit integer
i64 = {}


---@class i8 : ReflectReference
--- A signed 8-bit integer
i8 = {}


---@class isize : ReflectReference
--- A signed pointer-sized integer
isize = {}


---@class u128 : ReflectReference
--- An unsigned 128-bit integer
u128 = {}


---@class u16 : ReflectReference
--- An unsigned 16-bit integer
u16 = {}


---@class u32 : ReflectReference
--- An unsigned 32-bit integer
u32 = {}


---@class u64 : ReflectReference
--- An unsigned 64-bit integer
u64 = {}


---@class u8 : ReflectReference
--- An unsigned 8-bit integer
u8 = {}


---@class usize : ReflectReference
--- An unsigned pointer-sized integer
usize = {}


---@class Cow : ReflectReference
Cow = {}


---@class Arc : ReflectReference
Arc = {}


---@class Range : ReflectReference
Range = {}


---@class RangeInclusive : ReflectReference
RangeInclusive = {}




---@type Annulus
--- A static class allowing calls through the "." operator only. 
Annulus = {}

---@type AlignItems
--- A static class allowing calls through the "." operator only. 
AlignItems = {}

---@type FunctionInfo
--- A static class allowing calls through the "." operator only. 
FunctionInfo = {}

---@type WindowBackendScaleFactorChanged
--- A static class allowing calls through the "." operator only. 
WindowBackendScaleFactorChanged = {}

---@type TextColor
--- A static class allowing calls through the "." operator only. 
TextColor = {}

---@type BackgroundColor
--- A static class allowing calls through the "." operator only. 
BackgroundColor = {}

---@type AtomicU16
--- A static class allowing calls through the "." operator only. 
AtomicU16 = {}

---@type GamepadRumbleRequest
--- A static class allowing calls through the "." operator only. 
GamepadRumbleRequest = {}

---@type ConeMeshBuilder
--- A static class allowing calls through the "." operator only. 
ConeMeshBuilder = {}

---@type KeyboardFocusLost
--- A static class allowing calls through the "." operator only. 
KeyboardFocusLost = {}

---@type ComputedNode
--- A static class allowing calls through the "." operator only. 
ComputedNode = {}

---@type SmolStr
--- A static class allowing calls through the "." operator only. 
SmolStr = {}

---@type RegularPolygonMeshBuilder
--- A static class allowing calls through the "." operator only. 
RegularPolygonMeshBuilder = {}

---@type ChromaticAberration
--- A static class allowing calls through the "." operator only. 
ChromaticAberration = {}

---@type OnInsert
--- A static class allowing calls through the "." operator only. 
OnInsert = {}

---@type U8Vec3
--- A static class allowing calls through the "." operator only. 
U8Vec3 = {}

---@type DQuat
--- A static class allowing calls through the "." operator only. 
DQuat = {}

---@type DynamicComponent
--- A static class allowing calls through the "." operator only. 
DynamicComponent = {}

---@type U16Vec4
--- A static class allowing calls through the "." operator only. 
U16Vec4 = {}

---@type CursorLeft
--- A static class allowing calls through the "." operator only. 
CursorLeft = {}

---@type Arc2d
--- A static class allowing calls through the "." operator only. 
Arc2d = {}

---@type BloomCompositeMode
--- A static class allowing calls through the "." operator only. 
BloomCompositeMode = {}

---@type CameraRenderGraph
--- A static class allowing calls through the "." operator only. 
CameraRenderGraph = {}

---@type NonZeroI16
--- A static class allowing calls through the "." operator only. 
NonZeroI16 = {}

---@type FileDragAndDrop
--- A static class allowing calls through the "." operator only. 
FileDragAndDrop = {}

---@type Mat3
--- A static class allowing calls through the "." operator only. 
Mat3 = {}

---@type Frustum
--- A static class allowing calls through the "." operator only. 
Frustum = {}

---@type ColorGradingSection
--- A static class allowing calls through the "." operator only. 
ColorGradingSection = {}

---@type bool
--- A static class allowing calls through the "." operator only. 
bool = {}

---@type ComputedNodeTarget
--- A static class allowing calls through the "." operator only. 
ComputedNodeTarget = {}

---@type BorderColor
--- A static class allowing calls through the "." operator only. 
BorderColor = {}

---@type SystemIdMarker
--- A static class allowing calls through the "." operator only. 
SystemIdMarker = {}

---@type ImageNode
--- A static class allowing calls through the "." operator only. 
ImageNode = {}

---@type AccumulatedMouseMotion
--- A static class allowing calls through the "." operator only. 
AccumulatedMouseMotion = {}

---@type WindowPosition
--- A static class allowing calls through the "." operator only. 
WindowPosition = {}

---@type Label
--- A static class allowing calls through the "." operator only. 
Label = {}

---@type WindowCreated
--- A static class allowing calls through the "." operator only. 
WindowCreated = {}

---@type FontSmoothing
--- A static class allowing calls through the "." operator only. 
FontSmoothing = {}

---@type SmaaPreset
--- A static class allowing calls through the "." operator only. 
SmaaPreset = {}

---@type i16
--- A static class allowing calls through the "." operator only. 
i16 = {}

---@type MinTrackSizingFunction
--- A static class allowing calls through the "." operator only. 
MinTrackSizingFunction = {}

---@type PathBuf
--- A static class allowing calls through the "." operator only. 
PathBuf = {}

---@type ZIndex
--- A static class allowing calls through the "." operator only. 
ZIndex = {}

---@type WindowClosing
--- A static class allowing calls through the "." operator only. 
WindowClosing = {}

---@type Entity
--- A static class allowing calls through the "." operator only. 
Entity = {}

---@type Sensitivity
--- A static class allowing calls through the "." operator only. 
Sensitivity = {}

---@type Isometry2d
--- A static class allowing calls through the "." operator only. 
Isometry2d = {}

---@type RemovedComponentEntity
--- A static class allowing calls through the "." operator only. 
RemovedComponentEntity = {}

---@type f64
--- A static class allowing calls through the "." operator only. 
f64 = {}

---@type Namespace
--- A static class allowing calls through the "." operator only. 
Namespace = {}

---@type BloomPrefilter
--- A static class allowing calls through the "." operator only. 
BloomPrefilter = {}

---@type Text2d
--- A static class allowing calls through the "." operator only. 
Text2d = {}

---@type TouchPhase
--- A static class allowing calls through the "." operator only. 
TouchPhase = {}

---@type TorusMeshBuilder
--- A static class allowing calls through the "." operator only. 
TorusMeshBuilder = {}

---@type OrderIndependentTransparencySettings
--- A static class allowing calls through the "." operator only. 
OrderIndependentTransparencySettings = {}

---@type EulerRot
--- A static class allowing calls through the "." operator only. 
EulerRot = {}

---@type Camera3dDepthTextureUsage
--- A static class allowing calls through the "." operator only. 
Camera3dDepthTextureUsage = {}

---@type U64Vec2
--- A static class allowing calls through the "." operator only. 
U64Vec2 = {}

---@type I8Vec2
--- A static class allowing calls through the "." operator only. 
I8Vec2 = {}

---@type ContentSize
--- A static class allowing calls through the "." operator only. 
ContentSize = {}

---@type u8
--- A static class allowing calls through the "." operator only. 
u8 = {}

---@type Tetrahedron
--- A static class allowing calls through the "." operator only. 
Tetrahedron = {}

---@type Dir3A
--- A static class allowing calls through the "." operator only. 
Dir3A = {}

---@type ImageRenderTarget
--- A static class allowing calls through the "." operator only. 
ImageRenderTarget = {}

---@type Quat
--- A static class allowing calls through the "." operator only. 
Quat = {}

---@type Plane2d
--- A static class allowing calls through the "." operator only. 
Plane2d = {}

---@type I16Vec4
--- A static class allowing calls through the "." operator only. 
I16Vec4 = {}

---@type GamepadButtonChangedEvent
--- A static class allowing calls through the "." operator only. 
GamepadButtonChangedEvent = {}

---@type CompassOctant
--- A static class allowing calls through the "." operator only. 
CompassOctant = {}

---@type Ime
--- A static class allowing calls through the "." operator only. 
Ime = {}

---@type TextEntity
--- A static class allowing calls through the "." operator only. 
TextEntity = {}

---@type Line2d
--- A static class allowing calls through the "." operator only. 
Line2d = {}

---@type Sprite
--- A static class allowing calls through the "." operator only. 
Sprite = {}

---@type DeferredPrepass
--- A static class allowing calls through the "." operator only. 
DeferredPrepass = {}

---@type KeyCode
--- A static class allowing calls through the "." operator only. 
KeyCode = {}

---@type TemporalJitter
--- A static class allowing calls through the "." operator only. 
TemporalJitter = {}

---@type ViewVisibility
--- A static class allowing calls through the "." operator only. 
ViewVisibility = {}

---@type Ellipse
--- A static class allowing calls through the "." operator only. 
Ellipse = {}

---@type AspectRatio
--- A static class allowing calls through the "." operator only. 
AspectRatio = {}

---@type I64Vec4
--- A static class allowing calls through the "." operator only. 
I64Vec4 = {}

---@type Color
--- A static class allowing calls through the "." operator only. 
Color = {}

---@type WindowRef
--- A static class allowing calls through the "." operator only. 
WindowRef = {}

---@type GlobalTransform
--- A static class allowing calls through the "." operator only. 
GlobalTransform = {}

---@type Rectangle
--- A static class allowing calls through the "." operator only. 
Rectangle = {}

---@type WindowLevel
--- A static class allowing calls through the "." operator only. 
WindowLevel = {}

---@type ButtonSettings
--- A static class allowing calls through the "." operator only. 
ButtonSettings = {}

---@type IVec4
--- A static class allowing calls through the "." operator only. 
IVec4 = {}

---@type RequestRedraw
--- A static class allowing calls through the "." operator only. 
RequestRedraw = {}

---@type CalculatedClip
--- A static class allowing calls through the "." operator only. 
CalculatedClip = {}

---@type Line3d
--- A static class allowing calls through the "." operator only. 
Line3d = {}

---@type PrimaryWindow
--- A static class allowing calls through the "." operator only. 
PrimaryWindow = {}

---@type Triangle2dMeshBuilder
--- A static class allowing calls through the "." operator only. 
Triangle2dMeshBuilder = {}

---@type U8Vec2
--- A static class allowing calls through the "." operator only. 
U8Vec2 = {}

---@type BoxShadow
--- A static class allowing calls through the "." operator only. 
BoxShadow = {}

---@type U64Vec4
--- A static class allowing calls through the "." operator only. 
U64Vec4 = {}

---@type SubCameraView
--- A static class allowing calls through the "." operator only. 
SubCameraView = {}

---@type UVec3
--- A static class allowing calls through the "." operator only. 
UVec3 = {}

---@type DebandDither
--- A static class allowing calls through the "." operator only. 
DebandDither = {}

---@type Screenshot
--- A static class allowing calls through the "." operator only. 
Screenshot = {}

---@type OverflowClipMargin
--- A static class allowing calls through the "." operator only. 
OverflowClipMargin = {}

---@type GlyphAtlasLocation
--- A static class allowing calls through the "." operator only. 
GlyphAtlasLocation = {}

---@type Image
--- A static class allowing calls through the "." operator only. 
Image = {}

---@type FunctionReturnInfo
--- A static class allowing calls through the "." operator only. 
FunctionReturnInfo = {}

---@type I8Vec3
--- A static class allowing calls through the "." operator only. 
I8Vec3 = {}

---@type Fixed
--- A static class allowing calls through the "." operator only. 
Fixed = {}

---@type Triangle3d
--- A static class allowing calls through the "." operator only. 
Triangle3d = {}

---@type UiScale
--- A static class allowing calls through the "." operator only. 
UiScale = {}

---@type TextBounds
--- A static class allowing calls through the "." operator only. 
TextBounds = {}

---@type Vec2
--- A static class allowing calls through the "." operator only. 
Vec2 = {}

---@type Camera
--- A static class allowing calls through the "." operator only. 
Camera = {}

---@type BoundingCircle
--- A static class allowing calls through the "." operator only. 
BoundingCircle = {}

---@type Laba
--- A static class allowing calls through the "." operator only. 
Laba = {}

---@type Capsule3dMeshBuilder
--- A static class allowing calls through the "." operator only. 
Capsule3dMeshBuilder = {}

---@type DepthOfField
--- A static class allowing calls through the "." operator only. 
DepthOfField = {}

---@type f32
--- A static class allowing calls through the "." operator only. 
f32 = {}

---@type BorderRect
--- A static class allowing calls through the "." operator only. 
BorderRect = {}

---@type DAffine3
--- A static class allowing calls through the "." operator only. 
DAffine3 = {}

---@type i8
--- A static class allowing calls through the "." operator only. 
i8 = {}

---@type OverflowAxis
--- A static class allowing calls through the "." operator only. 
OverflowAxis = {}

---@type Range
--- A static class allowing calls through the "." operator only. 
Range = {}

---@type NonZeroU32
--- A static class allowing calls through the "." operator only. 
NonZeroU32 = {}

---@type BoundingCircleCast
--- A static class allowing calls through the "." operator only. 
BoundingCircleCast = {}

---@type Ray3d
--- A static class allowing calls through the "." operator only. 
Ray3d = {}

---@type Indices
--- A static class allowing calls through the "." operator only. 
Indices = {}

---@type AlignSelf
--- A static class allowing calls through the "." operator only. 
AlignSelf = {}

---@type SliceScaleMode
--- A static class allowing calls through the "." operator only. 
SliceScaleMode = {}

---@type u64
--- A static class allowing calls through the "." operator only. 
u64 = {}

---@type VideoMode
--- A static class allowing calls through the "." operator only. 
VideoMode = {}

---@type AtomicUsize
--- A static class allowing calls through the "." operator only. 
AtomicUsize = {}

---@type BVec3A
--- A static class allowing calls through the "." operator only. 
BVec3A = {}

---@type RenderAssetUsages
--- A static class allowing calls through the "." operator only. 
RenderAssetUsages = {}

---@type MouseButton
--- A static class allowing calls through the "." operator only. 
MouseButton = {}

---@type Vec3A
--- A static class allowing calls through the "." operator only. 
Vec3A = {}

---@type RotationGesture
--- A static class allowing calls through the "." operator only. 
RotationGesture = {}

---@type VisibilityClass
--- A static class allowing calls through the "." operator only. 
VisibilityClass = {}

---@type GamepadConnection
--- A static class allowing calls through the "." operator only. 
GamepadConnection = {}

---@type AutoExposure
--- A static class allowing calls through the "." operator only. 
AutoExposure = {}

---@type ScriptAsset
--- A static class allowing calls through the "." operator only. 
ScriptAsset = {}

---@type Xyza
--- A static class allowing calls through the "." operator only. 
Xyza = {}

---@type EntityHashSet
--- A static class allowing calls through the "." operator only. 
EntityHashSet = {}

---@type FlexDirection
--- A static class allowing calls through the "." operator only. 
FlexDirection = {}

---@type CircularMeshUvMode
--- A static class allowing calls through the "." operator only. 
CircularMeshUvMode = {}

---@type OnAdd
--- A static class allowing calls through the "." operator only. 
OnAdd = {}

---@type Text
--- A static class allowing calls through the "." operator only. 
Text = {}

---@type CapsuleUvProfile
--- A static class allowing calls through the "." operator only. 
CapsuleUvProfile = {}

---@type ReflectReference
--- A static class allowing calls through the "." operator only. 
ReflectReference = {}

---@type CustomProjection
--- A static class allowing calls through the "." operator only. 
CustomProjection = {}

---@type ReflectSystem
--- A static class allowing calls through the "." operator only. 
ReflectSystem = {}

---@type Tonemapping
--- A static class allowing calls through the "." operator only. 
Tonemapping = {}

---@type Mat4
--- A static class allowing calls through the "." operator only. 
Mat4 = {}

---@type OnDespawn
--- A static class allowing calls through the "." operator only. 
OnDespawn = {}

---@type RawGamepadEvent
--- A static class allowing calls through the "." operator only. 
RawGamepadEvent = {}

---@type Virtual
--- A static class allowing calls through the "." operator only. 
Virtual = {}

---@type CursorIcon
--- A static class allowing calls through the "." operator only. 
CursorIcon = {}

---@type InfinitePlane3d
--- A static class allowing calls through the "." operator only. 
InfinitePlane3d = {}

---@type Affine2
--- A static class allowing calls through the "." operator only. 
Affine2 = {}

---@type AtomicU32
--- A static class allowing calls through the "." operator only. 
AtomicU32 = {}

---@type JumpAt
--- A static class allowing calls through the "." operator only. 
JumpAt = {}

---@type ComponentId
--- A static class allowing calls through the "." operator only. 
ComponentId = {}

---@type AtomicBool
--- A static class allowing calls through the "." operator only. 
AtomicBool = {}

---@type Bloom
--- A static class allowing calls through the "." operator only. 
Bloom = {}

---@type ManualTextureViewHandle
--- A static class allowing calls through the "." operator only. 
ManualTextureViewHandle = {}

---@type Segment3d
--- A static class allowing calls through the "." operator only. 
Segment3d = {}

---@type WindowClosed
--- A static class allowing calls through the "." operator only. 
WindowClosed = {}

---@type Cow
--- A static class allowing calls through the "." operator only. 
Cow = {}

---@type EnabledButtons
--- A static class allowing calls through the "." operator only. 
EnabledButtons = {}

---@type GamepadConnectionEvent
--- A static class allowing calls through the "." operator only. 
GamepadConnectionEvent = {}

---@type RangeInclusive
--- A static class allowing calls through the "." operator only. 
RangeInclusive = {}

---@type InternalWindowState
--- A static class allowing calls through the "." operator only. 
InternalWindowState = {}

---@type Outline
--- A static class allowing calls through the "." operator only. 
Outline = {}

---@type VideoModeSelection
--- A static class allowing calls through the "." operator only. 
VideoModeSelection = {}

---@type u128
--- A static class allowing calls through the "." operator only. 
u128 = {}

---@type ChildOf
--- A static class allowing calls through the "." operator only. 
ChildOf = {}

---@type CylinderAnchor
--- A static class allowing calls through the "." operator only. 
CylinderAnchor = {}

---@type U16Vec2
--- A static class allowing calls through the "." operator only. 
U16Vec2 = {}

---@type GamepadRumbleIntensity
--- A static class allowing calls through the "." operator only. 
GamepadRumbleIntensity = {}

---@type table<string, ScriptTypeRegistration | ScriptComponentRegistration | ScriptResourceRegistration>
--- An global instance of this type
types = {}

---@type Skybox
--- A static class allowing calls through the "." operator only. 
Skybox = {}

---@type CylinderMeshBuilder
--- A static class allowing calls through the "." operator only. 
CylinderMeshBuilder = {}

---@type Camera3dDepthLoadOp
--- A static class allowing calls through the "." operator only. 
Camera3dDepthLoadOp = {}

---@type JustifySelf
--- A static class allowing calls through the "." operator only. 
JustifySelf = {}

---@type ScreenSpaceTransmissionQuality
--- A static class allowing calls through the "." operator only. 
ScreenSpaceTransmissionQuality = {}

---@type Dir2
--- A static class allowing calls through the "." operator only. 
Dir2 = {}

---@type CircularSegment
--- A static class allowing calls through the "." operator only. 
CircularSegment = {}

---@type BoxShadowSamples
--- A static class allowing calls through the "." operator only. 
BoxShadowSamples = {}

---@type LinearRgba
--- A static class allowing calls through the "." operator only. 
LinearRgba = {}

---@type TextLayoutInfo
--- A static class allowing calls through the "." operator only. 
TextLayoutInfo = {}

---@type ImageNodeSize
--- A static class allowing calls through the "." operator only. 
ImageNodeSize = {}

---@type TypeId
--- A static class allowing calls through the "." operator only. 
TypeId = {}

---@type I16Vec2
--- A static class allowing calls through the "." operator only. 
I16Vec2 = {}

---@type RectangleMeshBuilder
--- A static class allowing calls through the "." operator only. 
RectangleMeshBuilder = {}

---@type URect
--- A static class allowing calls through the "." operator only. 
URect = {}

---@type Capsule2dMeshBuilder
--- A static class allowing calls through the "." operator only. 
Capsule2dMeshBuilder = {}

---@type SphereMeshBuilder
--- A static class allowing calls through the "." operator only. 
SphereMeshBuilder = {}

---@type I64Vec2
--- A static class allowing calls through the "." operator only. 
I64Vec2 = {}

---@type Children
--- A static class allowing calls through the "." operator only. 
Children = {}

---@type i32
--- A static class allowing calls through the "." operator only. 
i32 = {}

---@type CursorOptions
--- A static class allowing calls through the "." operator only. 
CursorOptions = {}

---@type AtomicI16
--- A static class allowing calls through the "." operator only. 
AtomicI16 = {}

---@type MeshMorphWeights
--- A static class allowing calls through the "." operator only. 
MeshMorphWeights = {}

---@type Projection
--- A static class allowing calls through the "." operator only. 
Projection = {}

---@type DMat4
--- A static class allowing calls through the "." operator only. 
DMat4 = {}

---@type Gamepad
--- A static class allowing calls through the "." operator only. 
Gamepad = {}

---@type Affine3
--- A static class allowing calls through the "." operator only. 
Affine3 = {}

---@type TimerMode
--- A static class allowing calls through the "." operator only. 
TimerMode = {}

---@type Display
--- A static class allowing calls through the "." operator only. 
Display = {}

---@type VisibilityRange
--- A static class allowing calls through the "." operator only. 
VisibilityRange = {}

---@type RenderLayers
--- A static class allowing calls through the "." operator only. 
RenderLayers = {}

---@type U8Vec4
--- A static class allowing calls through the "." operator only. 
U8Vec4 = {}

---@type SyncToRenderWorld
--- A static class allowing calls through the "." operator only. 
SyncToRenderWorld = {}

---@type TextShadow
--- A static class allowing calls through the "." operator only. 
TextShadow = {}

---@type AabbCast3d
--- A static class allowing calls through the "." operator only. 
AabbCast3d = {}

---@type I8Vec4
--- A static class allowing calls through the "." operator only. 
I8Vec4 = {}

---@type ConicalFrustumMeshBuilder
--- A static class allowing calls through the "." operator only. 
ConicalFrustumMeshBuilder = {}

---@type LineBreak
--- A static class allowing calls through the "." operator only. 
LineBreak = {}

---@type Real
--- A static class allowing calls through the "." operator only. 
Real = {}

---@type WindowCloseRequested
--- A static class allowing calls through the "." operator only. 
WindowCloseRequested = {}

---@type ScrollPosition
--- A static class allowing calls through the "." operator only. 
ScrollPosition = {}

---@type RayCast3d
--- A static class allowing calls through the "." operator only. 
RayCast3d = {}

---@type OverflowClipBox
--- A static class allowing calls through the "." operator only. 
OverflowClipBox = {}

---@type Aabb3d
--- A static class allowing calls through the "." operator only. 
Aabb3d = {}

---@type Window
--- A static class allowing calls through the "." operator only. 
Window = {}

---@type InheritedVisibility
--- A static class allowing calls through the "." operator only. 
InheritedVisibility = {}

---@type TextureAtlasLayout
--- A static class allowing calls through the "." operator only. 
TextureAtlasLayout = {}

---@type Triangle3dMeshBuilder
--- A static class allowing calls through the "." operator only. 
Triangle3dMeshBuilder = {}

---@type I16Vec3
--- A static class allowing calls through the "." operator only. 
I16Vec3 = {}

---@type Smaa
--- A static class allowing calls through the "." operator only. 
Smaa = {}

---@type Oklcha
--- A static class allowing calls through the "." operator only. 
Oklcha = {}

---@type ShadowStyle
--- A static class allowing calls through the "." operator only. 
ShadowStyle = {}

---@type GridTrackRepetition
--- A static class allowing calls through the "." operator only. 
GridTrackRepetition = {}

---@type CompositeAlphaMode
--- A static class allowing calls through the "." operator only. 
CompositeAlphaMode = {}

---@type Aabb2d
--- A static class allowing calls through the "." operator only. 
Aabb2d = {}

---@type GamepadSettings
--- A static class allowing calls through the "." operator only. 
GamepadSettings = {}

---@type Camera3d
--- A static class allowing calls through the "." operator only. 
Camera3d = {}

---@type Arc
--- A static class allowing calls through the "." operator only. 
Arc = {}

---@type ClearColorConfig
--- A static class allowing calls through the "." operator only. 
ClearColorConfig = {}

---@type DVec3
--- A static class allowing calls through the "." operator only. 
DVec3 = {}

---@type FocusPolicy
--- A static class allowing calls through the "." operator only. 
FocusPolicy = {}

---@type Segment2d
--- A static class allowing calls through the "." operator only. 
Segment2d = {}

---@type Hwba
--- A static class allowing calls through the "." operator only. 
Hwba = {}

---@type DMat3
--- A static class allowing calls through the "." operator only. 
DMat3 = {}

---@type Identifier
--- A static class allowing calls through the "." operator only. 
Identifier = {}

---@type RepeatedGridTrack
--- A static class allowing calls through the "." operator only. 
RepeatedGridTrack = {}

---@type MotionVectorPrepass
--- A static class allowing calls through the "." operator only. 
MotionVectorPrepass = {}

---@type DynamicFunctionMut
--- A static class allowing calls through the "." operator only. 
DynamicScriptFunctionMut = {}

---@type ButtonAxisSettings
--- A static class allowing calls through the "." operator only. 
ButtonAxisSettings = {}

---@type OrthographicProjection
--- A static class allowing calls through the "." operator only. 
OrthographicProjection = {}

---@type SystemCursorIcon
--- A static class allowing calls through the "." operator only. 
SystemCursorIcon = {}

---@type U64Vec3
--- A static class allowing calls through the "." operator only. 
U64Vec3 = {}

---@type BVec4
--- A static class allowing calls through the "." operator only. 
BVec4 = {}

---@type Uuid
--- A static class allowing calls through the "." operator only. 
Uuid = {}

---@type Mesh2d
--- A static class allowing calls through the "." operator only. 
Mesh2d = {}

---@type ShaderStorageBuffer
--- A static class allowing calls through the "." operator only. 
ShaderStorageBuffer = {}

---@type GamepadButtonStateChangedEvent
--- A static class allowing calls through the "." operator only. 
GamepadButtonStateChangedEvent = {}

---@type AabbCast2d
--- A static class allowing calls through the "." operator only. 
AabbCast2d = {}

---@type ReflectableScheduleLabel
--- A static class allowing calls through the "." operator only. 
ReflectableScheduleLabel = {}

---@type AutoExposureCompensationCurve
--- A static class allowing calls through the "." operator only. 
AutoExposureCompensationCurve = {}

---@type GridTrack
--- A static class allowing calls through the "." operator only. 
GridTrack = {}

---@type ClearColor
--- A static class allowing calls through the "." operator only. 
ClearColor = {}

---@type Plane3d
--- A static class allowing calls through the "." operator only. 
Plane3d = {}

---@type Button
--- A static class allowing calls through the "." operator only. 
Button = {}

---@type WindowDestroyed
--- A static class allowing calls through the "." operator only. 
WindowDestroyed = {}

---@type Capsule3d
--- A static class allowing calls through the "." operator only. 
Capsule3d = {}

---@type CircularSectorMeshBuilder
--- A static class allowing calls through the "." operator only. 
CircularSectorMeshBuilder = {}

---@type MouseWheel
--- A static class allowing calls through the "." operator only. 
MouseWheel = {}

---@type JustifyText
--- A static class allowing calls through the "." operator only. 
JustifyText = {}

---@type BoundingSphereCast
--- A static class allowing calls through the "." operator only. 
BoundingSphereCast = {}

---@type NativeKeyCode
--- A static class allowing calls through the "." operator only. 
NativeKeyCode = {}

---@type DMat2
--- A static class allowing calls through the "." operator only. 
DMat2 = {}

---@type PositionType
--- A static class allowing calls through the "." operator only. 
PositionType = {}

---@type IVec2
--- A static class allowing calls through the "." operator only. 
IVec2 = {}

---@type ReflectSchedule
--- A static class allowing calls through the "." operator only. 
ReflectSchedule = {}

---@type UiAntiAlias
--- A static class allowing calls through the "." operator only. 
UiAntiAlias = {}

---@type DefaultQueryFilters
--- A static class allowing calls through the "." operator only. 
DefaultQueryFilters = {}

---@type FloatOrd
--- A static class allowing calls through the "." operator only. 
FloatOrd = {}

---@type Capsule2d
--- A static class allowing calls through the "." operator only. 
Capsule2d = {}

---@type Anchor
--- A static class allowing calls through the "." operator only. 
Anchor = {}

---@type Hsla
--- A static class allowing calls through the "." operator only. 
Hsla = {}

---@type AtomicU64
--- A static class allowing calls through the "." operator only. 
AtomicU64 = {}

---@type ComponentTicks
--- A static class allowing calls through the "." operator only. 
ComponentTicks = {}

---@type Torus
--- A static class allowing calls through the "." operator only. 
Torus = {}

---@type ScriptSystemBuilder
--- A static class allowing calls through the "." operator only. 
ScriptSystemBuilder = {}

---@type Interval
--- A static class allowing calls through the "." operator only. 
Interval = {}

---@type NormalPrepass
--- A static class allowing calls through the "." operator only. 
NormalPrepass = {}

---@type UiRect
--- A static class allowing calls through the "." operator only. 
UiRect = {}

---@type AtomicU8
--- A static class allowing calls through the "." operator only. 
AtomicU8 = {}

---@type CompassQuadrant
--- A static class allowing calls through the "." operator only. 
CompassQuadrant = {}

---@type DynamicFunction
--- A static class allowing calls through the "." operator only. 
DynamicScriptFunction = {}

---@type NativeKey
--- A static class allowing calls through the "." operator only. 
NativeKey = {}

---@type AlphaMode2d
--- A static class allowing calls through the "." operator only. 
AlphaMode2d = {}

---@type SocketAddr
--- A static class allowing calls through the "." operator only. 
SocketAddr = {}

---@type IVec3
--- A static class allowing calls through the "." operator only. 
IVec3 = {}

---@type TextLayout
--- A static class allowing calls through the "." operator only. 
TextLayout = {}

---@type DepthPrepass
--- A static class allowing calls through the "." operator only. 
DepthPrepass = {}

---@type DVec2
--- A static class allowing calls through the "." operator only. 
DVec2 = {}

---@type I64Vec3
--- A static class allowing calls through the "." operator only. 
I64Vec3 = {}

---@type Viewport
--- A static class allowing calls through the "." operator only. 
Viewport = {}

---@type Oklaba
--- A static class allowing calls through the "." operator only. 
Oklaba = {}

---@type WindowFocused
--- A static class allowing calls through the "." operator only. 
WindowFocused = {}

---@type MouseMotion
--- A static class allowing calls through the "." operator only. 
MouseMotion = {}

---@type Cuboid
--- A static class allowing calls through the "." operator only. 
Cuboid = {}

---@type Vec4
--- A static class allowing calls through the "." operator only. 
Vec4 = {}

---@type ScriptAttachment
--- A static class allowing calls through the "." operator only. 
ScriptAttachment = {}

---@type BVec3
--- A static class allowing calls through the "." operator only. 
BVec3 = {}

---@type TextFont
--- A static class allowing calls through the "." operator only. 
TextFont = {}

---@type WindowOccluded
--- A static class allowing calls through the "." operator only. 
WindowOccluded = {}

---@type DenoiseCas
--- A static class allowing calls through the "." operator only. 
DenoiseCas = {}

---@type ScreenshotCaptured
--- A static class allowing calls through the "." operator only. 
ScreenshotCaptured = {}

---@type RawGamepadAxisChangedEvent
--- A static class allowing calls through the "." operator only. 
RawGamepadAxisChangedEvent = {}

---@type TextNodeFlags
--- A static class allowing calls through the "." operator only. 
TextNodeFlags = {}

---@type Key
--- A static class allowing calls through the "." operator only. 
Key = {}

---@type PinchGesture
--- A static class allowing calls through the "." operator only. 
PinchGesture = {}

---@type Monitor
--- A static class allowing calls through the "." operator only. 
Monitor = {}

---@type CircularSegmentMeshBuilder
--- A static class allowing calls through the "." operator only. 
CircularSegmentMeshBuilder = {}

---@type Cone
--- A static class allowing calls through the "." operator only. 
Cone = {}

---@type OnRemove
--- A static class allowing calls through the "." operator only. 
OnRemove = {}

---@type GamepadButton
--- A static class allowing calls through the "." operator only. 
GamepadButton = {}

---@type AtomicI8
--- A static class allowing calls through the "." operator only. 
AtomicI8 = {}

---@type Msaa
--- A static class allowing calls through the "." operator only. 
Msaa = {}

---@type TetrahedronMeshBuilder
--- A static class allowing calls through the "." operator only. 
TetrahedronMeshBuilder = {}

---@type WindowResized
--- A static class allowing calls through the "." operator only. 
WindowResized = {}

---@type BVec2
--- A static class allowing calls through the "." operator only. 
BVec2 = {}

---@type TextureAtlas
--- A static class allowing calls through the "." operator only. 
TextureAtlas = {}

---@type AtomicI32
--- A static class allowing calls through the "." operator only. 
AtomicI32 = {}

---@type WindowMode
--- A static class allowing calls through the "." operator only. 
WindowMode = {}

---@type Dir3
--- A static class allowing calls through the "." operator only. 
Dir3 = {}

---@type Mesh3d
--- A static class allowing calls through the "." operator only. 
Mesh3d = {}

---@type BoxSizing
--- A static class allowing calls through the "." operator only. 
BoxSizing = {}

---@type FlexWrap
--- A static class allowing calls through the "." operator only. 
FlexWrap = {}

---@type ResolvedBorderRadius
--- A static class allowing calls through the "." operator only. 
ResolvedBorderRadius = {}

---@type Mat2
--- A static class allowing calls through the "." operator only. 
Mat2 = {}

---@type WindowThemeChanged
--- A static class allowing calls through the "." operator only. 
WindowThemeChanged = {}

---@type PerspectiveProjection
--- A static class allowing calls through the "." operator only. 
PerspectiveProjection = {}

---@type IRect
--- A static class allowing calls through the "." operator only. 
IRect = {}

---@type ContrastAdaptiveSharpening
--- A static class allowing calls through the "." operator only. 
ContrastAdaptiveSharpening = {}

---@type CameraMainTextureUsages
--- A static class allowing calls through the "." operator only. 
CameraMainTextureUsages = {}

---@type SphereKind
--- A static class allowing calls through the "." operator only. 
SphereKind = {}

---@type AlphaMode
--- A static class allowing calls through the "." operator only. 
AlphaMode = {}

---@type CubemapFrusta
--- A static class allowing calls through the "." operator only. 
CubemapFrusta = {}

---@type ScalingMode
--- A static class allowing calls through the "." operator only. 
ScalingMode = {}

---@type Exposure
--- A static class allowing calls through the "." operator only. 
Exposure = {}

---@type MotionBlur
--- A static class allowing calls through the "." operator only. 
MotionBlur = {}

---@type Transform
--- A static class allowing calls through the "." operator only. 
Transform = {}

---@type TemporalAntiAliasing
--- A static class allowing calls through the "." operator only. 
TemporalAntiAliasing = {}

---@type i64
--- A static class allowing calls through the "." operator only. 
i64 = {}

---@type ScriptQueryBuilder
--- A static class allowing calls through the "." operator only. 
ScriptQueryBuilder = {}

---@type UVec4
--- A static class allowing calls through the "." operator only. 
UVec4 = {}

---@type ButtonState
--- A static class allowing calls through the "." operator only. 
ButtonState = {}

---@type RegularPolygon
--- A static class allowing calls through the "." operator only. 
RegularPolygon = {}

---@type Hsva
--- A static class allowing calls through the "." operator only. 
Hsva = {}

---@type WindowMoved
--- A static class allowing calls through the "." operator only. 
WindowMoved = {}

---@type ScriptValue
--- A static class allowing calls through the "." operator only. 
ScriptValue = {}

---@type CircleMeshBuilder
--- A static class allowing calls through the "." operator only. 
CircleMeshBuilder = {}

---@type Duration
--- A static class allowing calls through the "." operator only. 
Duration = {}

---@type CursorGrabMode
--- A static class allowing calls through the "." operator only. 
CursorGrabMode = {}

---@type Lcha
--- A static class allowing calls through the "." operator only. 
Lcha = {}

---@type JustifyItems
--- A static class allowing calls through the "." operator only. 
JustifyItems = {}

---@type AssetPath
--- A static class allowing calls through the "." operator only. 
AssetPath = {}

---@type EllipseMeshBuilder
--- A static class allowing calls through the "." operator only. 
EllipseMeshBuilder = {}

---@type GamepadAxisChangedEvent
--- A static class allowing calls through the "." operator only. 
GamepadAxisChangedEvent = {}

---@type VisibleEntities
--- A static class allowing calls through the "." operator only. 
VisibleEntities = {}

---@type MipBias
--- A static class allowing calls through the "." operator only. 
MipBias = {}

---@type CascadesFrusta
--- A static class allowing calls through the "." operator only. 
CascadesFrusta = {}

---@type RawGamepadButtonChangedEvent
--- A static class allowing calls through the "." operator only. 
RawGamepadButtonChangedEvent = {}

---@type MorphWeights
--- A static class allowing calls through the "." operator only. 
MorphWeights = {}

---@type Rect
--- A static class allowing calls through the "." operator only. 
Rect = {}

---@type ScriptResourceRegistration
--- A static class allowing calls through the "." operator only. 
ScriptResourceRegistration = {}

---@type Node
--- A static class allowing calls through the "." operator only. 
Node = {}

---@type AxisSettings
--- A static class allowing calls through the "." operator only. 
AxisSettings = {}

---@type Vec3
--- A static class allowing calls through the "." operator only. 
Vec3 = {}

---@type Overflow
--- A static class allowing calls through the "." operator only. 
Overflow = {}

---@type AssetIndex
--- A static class allowing calls through the "." operator only. 
AssetIndex = {}

---@type Fxaa
--- A static class allowing calls through the "." operator only. 
Fxaa = {}

---@type Ray2d
--- A static class allowing calls through the "." operator only. 
Ray2d = {}

---@type GlyphAtlasInfo
--- A static class allowing calls through the "." operator only. 
GlyphAtlasInfo = {}

---@type ComputedTextBlock
--- A static class allowing calls through the "." operator only. 
ComputedTextBlock = {}

---@type AtomicIsize
--- A static class allowing calls through the "." operator only. 
AtomicIsize = {}

---@type TransformTreeChanged
--- A static class allowing calls through the "." operator only. 
TransformTreeChanged = {}

---@type ScriptComponentRegistration
--- A static class allowing calls through the "." operator only. 
ScriptComponentRegistration = {}

---@type AtomicI64
--- A static class allowing calls through the "." operator only. 
AtomicI64 = {}

---@type UiTargetCamera
--- A static class allowing calls through the "." operator only. 
UiTargetCamera = {}

---@type CircularSector
--- A static class allowing calls through the "." operator only. 
CircularSector = {}

---@type Rot2
--- A static class allowing calls through the "." operator only. 
Rot2 = {}

---@type RayCast2d
--- A static class allowing calls through the "." operator only. 
RayCast2d = {}

---@type TextureSlicer
--- A static class allowing calls through the "." operator only. 
TextureSlicer = {}

---@type Srgba
--- A static class allowing calls through the "." operator only. 
Srgba = {}

---@type DVec4
--- A static class allowing calls through the "." operator only. 
DVec4 = {}

---@type Visibility
--- A static class allowing calls through the "." operator only. 
Visibility = {}

---@type Camera2d
--- A static class allowing calls through the "." operator only. 
Camera2d = {}

---@type Instant
--- A static class allowing calls through the "." operator only. 
Instant = {}

---@type CuboidMeshBuilder
--- A static class allowing calls through the "." operator only. 
CuboidMeshBuilder = {}

---@type NonZeroU16
--- A static class allowing calls through the "." operator only. 
NonZeroU16 = {}

---@type BoundingSphere
--- A static class allowing calls through the "." operator only. 
BoundingSphere = {}

---@type GamepadAxis
--- A static class allowing calls through the "." operator only. 
GamepadAxis = {}

---@type Mesh
--- A static class allowing calls through the "." operator only. 
Mesh = {}

---@type ColorGradingGlobal
--- A static class allowing calls through the "." operator only. 
ColorGradingGlobal = {}

---@type i128
--- A static class allowing calls through the "." operator only. 
i128 = {}

---@type isize
--- A static class allowing calls through the "." operator only. 
isize = {}

---@type MonitorSelection
--- A static class allowing calls through the "." operator only. 
MonitorSelection = {}

---@type Isometry3d
--- A static class allowing calls through the "." operator only. 
Isometry3d = {}

---@type FunctionCallContext
--- A static class allowing calls through the "." operator only. 
FunctionCallContext = {}

---@type GridPlacement
--- A static class allowing calls through the "." operator only. 
GridPlacement = {}

---@type Circle
--- A static class allowing calls through the "." operator only. 
Circle = {}

---@type FunctionArgInfo
--- A static class allowing calls through the "." operator only. 
FunctionArgInfo = {}

---@type SpriteImageMode
--- A static class allowing calls through the "." operator only. 
SpriteImageMode = {}

---@type RangeFull
--- A static class allowing calls through the "." operator only. 
RangeFull = {}

---@type OcclusionCulling
--- A static class allowing calls through the "." operator only. 
OcclusionCulling = {}

---@type GridAutoFlow
--- A static class allowing calls through the "." operator only. 
GridAutoFlow = {}

---@type ScriptTypeRegistration
--- A static class allowing calls through the "." operator only. 
ScriptTypeRegistration = {}

---@type RelativeCursorPosition
--- A static class allowing calls through the "." operator only. 
RelativeCursorPosition = {}

---@type Disabled
--- A static class allowing calls through the "." operator only. 
Disabled = {}

---@type U16Vec3
--- A static class allowing calls through the "." operator only. 
U16Vec3 = {}

---@type ScriptQueryResult
--- A static class allowing calls through the "." operator only. 
ScriptQueryResult = {}

---@type u32
--- A static class allowing calls through the "." operator only. 
u32 = {}

---@type ColorGrading
--- A static class allowing calls through the "." operator only. 
ColorGrading = {}

---@type Timer
--- A static class allowing calls through the "." operator only. 
Timer = {}

---@type usize
--- A static class allowing calls through the "." operator only. 
usize = {}

---@type RenderTarget
--- A static class allowing calls through the "." operator only. 
RenderTarget = {}

---@type WindowScaleFactorChanged
--- A static class allowing calls through the "." operator only. 
WindowScaleFactorChanged = {}

---@type DAffine2
--- A static class allowing calls through the "." operator only. 
DAffine2 = {}

---@type AnnulusMeshBuilder
--- A static class allowing calls through the "." operator only. 
AnnulusMeshBuilder = {}

---@type Interaction
--- A static class allowing calls through the "." operator only. 
Interaction = {}

---@type ForceTouch
--- A static class allowing calls through the "." operator only. 
ForceTouch = {}

---@type InteropError
--- A static class allowing calls through the "." operator only. 
InteropError = {}

---@type WindowResolution
--- A static class allowing calls through the "." operator only. 
WindowResolution = {}

---@type Val
--- A static class allowing calls through the "." operator only. 
Val = {}

---@type AccumulatedMouseScroll
--- A static class allowing calls through the "." operator only. 
AccumulatedMouseScroll = {}

---@type WindowEvent
--- A static class allowing calls through the "." operator only. 
WindowEvent = {}

---@type LineHeight
--- A static class allowing calls through the "." operator only. 
LineHeight = {}

---@type Sphere
--- A static class allowing calls through the "." operator only. 
Sphere = {}

---@type ConicalFrustum
--- A static class allowing calls through the "." operator only. 
ConicalFrustum = {}

---@type NoFrustumCulling
--- A static class allowing calls through the "." operator only. 
NoFrustumCulling = {}

---@type DepthOfFieldMode
--- A static class allowing calls through the "." operator only. 
DepthOfFieldMode = {}

---@type KeyboardInput
--- A static class allowing calls through the "." operator only. 
KeyboardInput = {}

---@type Name
--- A static class allowing calls through the "." operator only. 
Name = {}

---@type AppLifecycle
--- A static class allowing calls through the "." operator only. 
AppLifecycle = {}

---@type MouseScrollUnit
--- A static class allowing calls through the "." operator only. 
MouseScrollUnit = {}

---@type PositionedGlyph
--- A static class allowing calls through the "." operator only. 
PositionedGlyph = {}

---@type char
--- A static class allowing calls through the "." operator only. 
char = {}

---@type TouchInput
--- A static class allowing calls through the "." operator only. 
TouchInput = {}

---@type EaseFunction
--- A static class allowing calls through the "." operator only. 
EaseFunction = {}

---@type GamepadEvent
--- A static class allowing calls through the "." operator only. 
GamepadEvent = {}

---@type Triangle2d
--- A static class allowing calls through the "." operator only. 
Triangle2d = {}

---@type u16
--- A static class allowing calls through the "." operator only. 
u16 = {}

---@type String
--- A static class allowing calls through the "." operator only. 
String = {}

---@type ConeAnchor
--- A static class allowing calls through the "." operator only. 
ConeAnchor = {}

---@type MouseButtonInput
--- A static class allowing calls through the "." operator only. 
MouseButtonInput = {}

---@type TextSpan
--- A static class allowing calls through the "." operator only. 
TextSpan = {}

---@type MaxTrackSizingFunction
--- A static class allowing calls through the "." operator only. 
MaxTrackSizingFunction = {}

---@type GamepadInput
--- A static class allowing calls through the "." operator only. 
GamepadInput = {}

---@type CursorMoved
--- A static class allowing calls through the "." operator only. 
CursorMoved = {}

---@type BorderRadius
--- A static class allowing calls through the "." operator only. 
BorderRadius = {}

---@type ColorMaterial
--- A static class allowing calls through the "." operator only. 
ColorMaterial = {}

---@type GlobalsUniform
--- A static class allowing calls through the "." operator only. 
GlobalsUniform = {}

---@type Tick
--- A static class allowing calls through the "." operator only. 
Tick = {}

---@type BVec4A
--- A static class allowing calls through the "." operator only. 
BVec4A = {}

---@type OnReplace
--- A static class allowing calls through the "." operator only. 
OnReplace = {}

---@type RhombusMeshBuilder
--- A static class allowing calls through the "." operator only. 
RhombusMeshBuilder = {}

---@type Affine3A
--- A static class allowing calls through the "." operator only. 
Affine3A = {}

---@type DoubleTapGesture
--- A static class allowing calls through the "." operator only. 
DoubleTapGesture = {}

---@type Stopwatch
--- A static class allowing calls through the "." operator only. 
Stopwatch = {}

---@type AlignContent
--- A static class allowing calls through the "." operator only. 
AlignContent = {}

---@type WindowTheme
--- A static class allowing calls through the "." operator only. 
WindowTheme = {}

---@type PlaneMeshBuilder
--- A static class allowing calls through the "." operator only. 
PlaneMeshBuilder = {}

---@type Mat3A
--- A static class allowing calls through the "." operator only. 
Mat3A = {}

---@type SkinnedMesh
--- A static class allowing calls through the "." operator only. 
SkinnedMesh = {}

---@type PresentMode
--- A static class allowing calls through the "." operator only. 
PresentMode = {}

---@type EntityHash
--- A static class allowing calls through the "." operator only. 
EntityHash = {}

---@type UVec2
--- A static class allowing calls through the "." operator only. 
UVec2 = {}

---@type JustifyContent
--- A static class allowing calls through the "." operator only. 
JustifyContent = {}

---@type CursorEntered
--- A static class allowing calls through the "." operator only. 
CursorEntered = {}

---@type Aabb
--- A static class allowing calls through the "." operator only. 
Aabb = {}

---@type PanGesture
--- A static class allowing calls through the "." operator only. 
PanGesture = {}

---@type Rhombus
--- A static class allowing calls through the "." operator only. 
Rhombus = {}

---@type NodeImageMode
--- A static class allowing calls through the "." operator only. 
NodeImageMode = {}

---@type WindowResizeConstraints
--- A static class allowing calls through the "." operator only. 
WindowResizeConstraints = {}

---@type Cylinder
--- A static class allowing calls through the "." operator only. 
Cylinder = {}

---@type Handle
--- An global instance of this type
script_asset = {}

---@type World
--- An global instance of this type
world = {}

---@type Entity
--- An global instance of this type
entity = {}

