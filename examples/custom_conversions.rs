// The standard way of interacting with reflect types is via R<T> V<T> and M<T> wrappers.
// These do the following under the hood:
// - Claim the necessary access from underlying references
// - downcast, construct or clone the reflect value as needed to generate a reference
// You may want your types to convert from more than just references, i.e. primitives or perhaps from a variety other types.

// you can do this by implementing FromScript and IntoScript as needed

use std::any::TypeId;

use bevy::reflect::Typed;
use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_bindings::{FromScript, InteropError, ReflectReference};

// `ScriptType` implements everything else a type needs to be used in function signatures, `from = custom` leaves `FromScript` to us
#[derive(Clone, ScriptType, Reflect)]
#[script_type(from = custom)]
pub enum MyFunkyArgumentType {
    Nil,
    Number(usize),
    HandleToSprite(Handle<Image>),
}

impl FromScript for MyFunkyArgumentType {
    // generally this is always Self, unless you are implementing complex wrapper types
    type This<'w> = Self;

    fn from_script(
        value: ScriptValue,
        world: bevy_mod_scripting_bindings::WorldGuard<'_>,
    ) -> std::result::Result<Self::This<'_>, bevy_mod_scripting_bindings::InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Unit => Ok(Self::Nil),
            ScriptValue::Bool(_) => todo!(),
            ScriptValue::Integer(i) => Ok(Self::Number(i as usize)),
            ScriptValue::Float(f) => Ok(Self::Number(f as usize)),
            ScriptValue::Reference(reflect_reference) => {
                // references point into the world, or to values allocated for scripts
                // you can use with_reflect if you don't want to clone the value, otherwise use downcast
                reflect_reference.with_reflect(world, |my_val| {
                    // and here we have a normal reflect reference
                    let me = my_val
                        .try_downcast_ref::<MyFunkyArgumentType>()
                        .ok_or_else(|| InteropError::str("expected myfinkyargumenttype"))?;

                    Ok(me.clone())
                })?
            }
            // you can probably make this error better, this will just show "v != MyFunkyType" but this is the quickest way to get this working nicely.
            _ => Err(InteropError::value_mismatch(TypeId::of::<Self>(), value)),
        }
    }
}

// you can do the same thing for IntoScript

// `ScriptType` implements everything else a type needs to be used in function signatures, `into = custom` leaves `IntoScript` to us
#[derive(Clone, ScriptType, Reflect)]
#[script_type(into = custom)]
pub enum MyFunkyReturnType {
    Nil,
    Number(usize),
    HandleToSprite(Handle<Image>),
}

impl bevy_mod_scripting::bindings::IntoScript for MyFunkyReturnType {
    fn into_script(
        self,
        _world: bevy_mod_scripting_bindings::WorldGuard,
    ) -> std::result::Result<ScriptValue, InteropError> {
        match self {
            // note if we return primitives, the type is no longer callable!
            // i.e. these get converted to primitive lua types
            MyFunkyReturnType::Nil => Ok(ScriptValue::Unit),
            MyFunkyReturnType::Number(n) => Ok(ScriptValue::Integer(n as i64)),
            // references stay callabke though, in this case we change the type
            MyFunkyReturnType::HandleToSprite(handle) => {
                // here in order to create a reference to this handle, we must allocate it first
                Ok(ReflectReference::new_allocated(handle).into())
            }
        }
    }
}

// these can both participate in bindings now

#[script_bindings(remote, unregistered)]
impl World {
    pub fn my_funky_function(a: MyFunkyArgumentType) -> MyFunkyReturnType {
        match a {
            MyFunkyArgumentType::Nil => MyFunkyReturnType::Nil,
            MyFunkyArgumentType::Number(n) => MyFunkyReturnType::Number(n),
            MyFunkyArgumentType::HandleToSprite(handle) => {
                MyFunkyReturnType::HandleToSprite(handle)
            }
        }
    }
}
callback_labels!(OnExample => "on_example");
pub fn main() {
    let mut app = App::new();
    // required for bindings to actually be visibly
    register_functions(app.world_mut());

    app.add_plugins(DefaultPlugins)
        .add_plugins(BMSPlugin)
        .add_systems(
            Startup,
            move |mut commands: Commands,
                  mut script_assets: ResMut<Assets<ScriptAsset>>,
                  mut callbacks: MessageWriter<ScriptCallbackMessage>| {
                let content = r#"
                    function on_example(my_funky_argument_payload)
                        print("input:", my_funky_argument_payload)
                        local output = world.my_funky_function(my_funky_argument_payload)
                        print("output:", output)
                        return output
                    end
                "#;
                let script_asset = ScriptAsset::new(content).with_language(Language::Lua);
                let handle = script_assets.add(script_asset);
                let attachment = ScriptAttachment::StaticScript(handle.clone());

                let reference_payload =
                    ReflectReference::new_allocated(MyFunkyArgumentType::Number(2));

                commands.queue(AttachScript::<LuaScriptingPlugin>::new(attachment.clone()));
                callbacks.write(
                    ScriptCallbackMessage::new_for_static_script(
                        OnExample,
                        vec![reference_payload.into()],
                        handle,
                    )
                    .with_response(),
                );
            },
        )
        .add_systems(Update, event_handler::<OnExample, LuaScriptingPlugin>);

    app.run();
}
