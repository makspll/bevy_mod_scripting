pub mod assets;
pub mod docs;
pub mod util;
use bevy::{
    app::{App, Plugin},
    ecs::{entity::Entity, world::World},
    reflect::{FromType, GetTypeRegistration, PartialReflect, Reflect, TypePath},
};
use bevy_mod_scripting_core::{
    bindings::{ReflectAllocator, ReflectReference, WorldCallbackAccess},
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    handler::Args,
    script::ScriptId,
    AddContextPreHandlingInitializer, ScriptingPlugin,
};
use bindings::{
    providers::bevy_ecs::LuaEntity,
    proxy::LuaProxied,
    world::{GetWorld, LuaWorld},
};
pub use tealr;
pub mod bindings;
use tealr::mlu::mlua::{FromLua, Function, IntoLua, IntoLuaMulti, Lua, Value};

pub mod prelude {
    pub use crate::tealr::{
        self,
        mlu::{
            mlua::{self, prelude::*, Value},
            TealData,
        },
    };
}

pub trait LuaEventArg: Args + for<'l> IntoLuaMulti<'l> {}
impl<T: Args + for<'l> IntoLuaMulti<'l>> LuaEventArg for T {}

pub struct LuaScriptingPlugin<A: Args + for<'l> IntoLuaMulti<'l>> {
    pub scripting_plugin: ScriptingPlugin<A, Lua, ()>,
}

impl<A: LuaEventArg> Default for LuaScriptingPlugin<A> {
    fn default() -> Self {
        LuaScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                context_assigner: None,
                runtime_builder: Default::default,
                runtime_settings: None,
                callback_handler: Some(lua_handler::<A>),
                context_builder: Some(ContextBuilder::<Lua, ()> {
                    load: lua_context_load,
                    reload: lua_context_reload,
                }),
            },
        }
    }
}

pub fn register_lua_values(app: &mut bevy::prelude::App) {
    app.register_lua_value::<usize>();
    app.register_lua_value::<isize>();
    app.register_lua_value::<f32>();
    app.register_lua_value::<f64>();
    app.register_lua_value::<u128>();
    app.register_lua_value::<u64>();
    app.register_lua_value::<u32>();
    app.register_lua_value::<u16>();
    app.register_lua_value::<u8>();
    app.register_lua_value::<i128>();
    app.register_lua_value::<i64>();
    app.register_lua_value::<i32>();
    app.register_lua_value::<i16>();
    app.register_lua_value::<i8>();
    app.register_lua_value::<String>();
    app.register_lua_value::<bool>();
}

impl<A: LuaEventArg> Plugin for LuaScriptingPlugin<A> {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
        register_lua_values(app);
        app.add_context_pre_handling_initializer::<()>(|script_id, entity, context: &mut Lua| {
            let world = context.get_world().unwrap();
            let lua_entity = world.with_resource::<ReflectAllocator, _, _>(|_, mut allocator| {
                let reflect_reference = ReflectReference::new_allocated(entity, &mut allocator);
                <Entity as LuaProxied>::Proxy::from(reflect_reference)
            });

            context.globals().set("script_id", script_id.to_owned())?;
            context.globals().set("entity", lua_entity)?;
            Ok(())
        });
    }
}

pub fn lua_context_load(
    script_id: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<Lua>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    world: &mut World,
    _: &mut (),
) -> Result<Lua, ScriptError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let context = unsafe { Lua::unsafe_new() };
    #[cfg(not(feature = "unsafe_lua_modules"))]
    let mut context = Lua::new();

    with_world(world, &mut context, |context| {
        initializers
            .iter()
            .try_for_each(|init| init(script_id, context))?;

        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script_id, Entity::from_raw(0), context))?;

        context.load(content).exec()?;
        Ok(())
    })?;

    Ok(context)
}

pub fn lua_context_reload(
    script: &ScriptId,
    content: &[u8],
    old_ctxt: &mut Lua,
    initializers: &[ContextInitializer<Lua>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    world: &mut World,
    _: &mut (),
) -> Result<(), ScriptError> {
    *old_ctxt = lua_context_load(
        script,
        content,
        initializers,
        pre_handling_initializers,
        world,
        &mut (),
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn lua_handler<A: Args + for<'l> IntoLuaMulti<'l>>(
    args: A,
    entity: bevy::ecs::entity::Entity,
    script_id: &ScriptId,
    callback_label: &CallbackLabel,
    context: &mut Lua,
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    _: &mut (),
    world: &mut bevy::ecs::world::World,
) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
    with_world(world, context, |context| {
        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script_id, entity, context))?;

        let handler: Function = match context.globals().raw_get(callback_label.as_ref()) {
            Ok(handler) => handler,
            // not subscribed to this event type
            Err(_) => return Ok(()),
        };

        handler.call::<_, ()>(args)?;
        Ok(())
    })
}

/// Safely scopes world access for a lua context to the given closure's scope
pub fn with_world<F: FnOnce(&mut Lua) -> Result<(), ScriptError>>(
    world: &mut World,
    context: &mut Lua,
    f: F,
) -> Result<(), ScriptError> {
    WorldCallbackAccess::with_callback_access(world, |guard| {
        context.globals().set("world", LuaWorld(guard.clone()))?;
        f(context)
    })
}

/// Registers a lua proxy object via the reflection system
pub trait RegisterLua {
    fn register_lua_proxy<T: LuaProxied + Reflect + TypePath + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self
    where
        T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T::Proxy: From<ReflectReference> + AsRef<ReflectReference>;

    fn register_lua_value<T>(&mut self) -> &mut Self
    where
        T: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T: Reflect + Clone + TypePath + GetTypeRegistration;
}

impl RegisterLua for App {
    fn register_lua_proxy<T: LuaProxied + Reflect + TypePath + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self
    where
        T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T::Proxy: From<ReflectReference> + AsRef<ReflectReference>,
    {
        self.register_type::<T>();
        self.register_type_data::<T, ReflectLuaProxied>()
    }

    fn register_lua_value<T>(&mut self) -> &mut Self
    where
        T: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T: Reflect + Clone + TypePath + GetTypeRegistration,
    {
        self.register_type::<T>();
        self.register_type_data::<T, ReflectLuaValue>()
    }
}

/// Stores the procedure used to convert a lua value to a reflect value and vice versa, Used for types which are represented in lua via proxies which store
/// a reference to the actual value.
/// This is used for types which are represented in lua with pass by reference semantics
#[derive(Clone)]
pub struct ReflectLuaProxied {
    pub into_proxy:
        for<'l> fn(ReflectReference, &'l Lua) -> Result<Value<'l>, tealr::mlu::mlua::Error>,
    pub from_proxy:
        for<'l> fn(Value<'l>, &'l Lua) -> Result<ReflectReference, tealr::mlu::mlua::Error>,
}

impl<T: LuaProxied + Reflect> FromType<T> for ReflectLuaProxied
where
    T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
    T::Proxy: From<ReflectReference> + AsRef<ReflectReference>,
{
    fn from_type() -> Self {
        Self {
            into_proxy: |p, l| T::Proxy::from(p).into_lua(l),
            from_proxy: |v, l| T::Proxy::from_lua(v, l).map(|p| p.as_ref().clone()),
        }
    }
}

/// Stores the procedure used to convert a lua value to a reflect value and vice versa, Used for types which are represented directly in lua with
/// pass by value semantics, These need to implement [`Clone`]
#[derive(Clone)]
pub struct ReflectLuaValue {
    pub into_value:
        for<'l> fn(&dyn PartialReflect, &'l Lua) -> Result<Value<'l>, tealr::mlu::mlua::Error>,
    pub set_value: for<'l> fn(
        &mut dyn PartialReflect,
        Value<'l>,
        &'l Lua,
    ) -> Result<(), tealr::mlu::mlua::Error>,
    pub from_value:
        for<'l> fn(Value<'l>, &'l Lua) -> Result<Box<dyn PartialReflect>, tealr::mlu::mlua::Error>,
}

impl<T: Reflect + Clone + for<'l> IntoLua<'l> + for<'l> FromLua<'l>> FromType<T>
    for ReflectLuaValue
{
    fn from_type() -> Self {
        Self {
            into_value: |v, l| v.try_downcast_ref::<T>().unwrap().clone().into_lua(l),
            set_value: |t, v, l| {
                let t = t.try_downcast_mut::<T>().unwrap();
                *t = T::from_lua(v, l)?;
                Ok(())
            },
            from_value: |v, l| T::from_lua(v, l).map(|v| Box::new(v) as Box<dyn PartialReflect>),
        }
    }
}
