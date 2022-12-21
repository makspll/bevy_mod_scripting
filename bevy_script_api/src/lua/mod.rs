use ::std::any::TypeId;
use ::std::borrow::Cow;

use crate::impl_tealr_type;
use ::bevy::prelude::{App, AppTypeRegistry};

use ::bevy::reflect::{FromType, GetTypeRegistration, Reflect};

use bevy_mod_scripting_lua::tealr;

use tealr::mlu::mlua::MetaMethod;
use tealr::mlu::{
    mlua::{self, FromLua, Lua, ToLua, UserData, Value},
    TealData, TealDataMethods,
};
use tealr::TypeName;

use crate::script_ref::{ReflectedValue, ScriptRef, ValueIndex};

use self::bevy::LuaWorld;

pub mod bevy;
pub mod std;
pub mod util;

/// A trait allowing to register the [`LuaProxyable`] trait with the type registry for foreign types
///
/// If you have access to the type you should prefer to use `#[reflect(LuaProxyable)]` instead.
/// This is exactly equivalent.
pub trait RegisterForeignLuaType {
    /// Register an instance of `ReflecLuaProxyable` type data on this type's registration,
    /// if a registration does not yet exist, creates one.
    fn register_foreign_lua_type<T: LuaProxyable + Reflect + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self;
}

impl RegisterForeignLuaType for App {
    fn register_foreign_lua_type<T: LuaProxyable + Reflect + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self {
        {
            let registry = self.world.resource_mut::<AppTypeRegistry>();
            let mut registry = registry.write();

            let user_data = <ReflectLuaProxyable as FromType<T>>::from_type();

            if let Some(registration) = registry.get_mut(TypeId::of::<T>()) {
                registration.insert(user_data)
            } else {
                let mut registration = T::get_type_registration();
                registration.insert(user_data);
                registry.add_registration(registration);
            }
        }

        self
    }
}

impl ValueIndex<Value<'_>> for ScriptRef {
    type Output = Result<Self, mlua::Error>;

    fn index(&self, index: Value<'_>) -> Self::Output {
        match index {
            Value::Integer(idx) => Ok(self.index(idx as usize)),
            Value::String(field) => {
                let str_ = field.to_str()?.to_string();
                // TODO: hopefully possible to use a &'_ str here
                // but this requires Reflect implementation for &str
                Ok(<Self as ValueIndex<Cow<'static, str>>>::index(
                    self,
                    str_.into(),
                ))
            }
            _ => Err(mlua::Error::RuntimeError(format!(
                "Cannot index a rust object with {:?}",
                index
            ))),
        }
    }
}

/// For internal use only.
///
/// Mainly necessary for separation of concerns on the [`ScriptRef`] type, but might have other uses potentially.
///
/// This is not the same as [`LuaProxyable`], internally this in fact will use [`LuaProxyable`] so treating it like so will cause inifnite loops.
pub(crate) trait ApplyLua {
    /// set the proxied object with the given lua value
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> mlua::Result<()>;
}
impl ApplyLua for ScriptRef {
    /// Applies the given lua value to the proxied reflect type. Semantically equivalent to `Reflect::apply`
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> Result<(), mlua::Error> {
        let luaworld = ctx.globals().get::<_, LuaWorld>("world").unwrap();

        // remove typedata from the world to be able to manipulate world
        let proxyable = {
            let world = luaworld.read();
            let type_registry = world.resource::<AppTypeRegistry>().read();
            type_registry
                .get_type_data::<ReflectLuaProxyable>(self.get(|s| s.type_id())?)
                .cloned()
        };

        if let Some(ud) = proxyable {
            return ud.apply_lua(self, ctx, v);
        } else if let Value::UserData(v) = &v {
            if v.is::<ReflectedValue>() {
                let b = v.take::<ReflectedValue>().unwrap();
                self.apply(&b.into())?;
                return Ok(());
            }
        }

        Err(mlua::Error::RuntimeError(self.get(|s|
            format!("Attempted to assign `{}` = {v:?}. Did you forget to call `app.register_foreign_lua_type::<{}>`?",
                self.path,
                s.type_name()
            ))?)
        )
    }
}

impl<'lua> ToLua<'lua> for ScriptRef {
    /// Converts the LuaRef to the most convenient representation
    /// checking conversions in this order:
    /// - A primitive or bevy type which has a reflect interface is converted to a custom UserData exposing its API to lua conveniently
    /// - A type implementing CustomUserData is converted with its `ref_to_lua` method
    /// - Finally the method is represented as a `ReflectedValue` which exposes the Reflect interface
    fn to_lua(self, ctx: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let luaworld = ctx.globals().get::<_, LuaWorld>("world").unwrap();

        let world = luaworld.read();

        let typedata = &world.resource::<AppTypeRegistry>();
        let g = typedata.read();

        let type_id = self.get(|s| s.type_id())?;
        if let Some(v) = g.get_type_data::<ReflectLuaProxyable>(type_id) {
            Ok(v.ref_to_lua(self, ctx)?)
        } else {
            ReflectedValue { ref_: self }.to_lua(ctx)
        }
    }
}

impl TypeName for ScriptRef {
    /// ReflectedValue represents the "lowest common denominator" across the possible returned types
    /// people can always use 'as' to cast to the right type
    /// but the static analysis will be conservative, i.e. the compiler will assume the smallest set of functionality
    /// by default
    fn get_type_parts() -> Cow<'static, [tealr::NamePart]> {
        Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
            name: Cow::Borrowed("ReflectedValue"),
            generics: None,
            type_kind: tealr::KindOfType::Builtin,
        })])
    }
}

impl_tealr_type!(ReflectedValue);
impl TealData for ReflectedValue {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("This type represents a generic reflected value.");
        methods.document_type("If you know the reflected value converts to a LuaType (via LuaProxyable), use the `as` operator to convert to said type.");

        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.ref_.get(|s| Ok(format!("{:#?}", &s)))?
        });

        methods.add_meta_method_mut(MetaMethod::Index, |_, val, field: Value| {
            let r = val.ref_.index(field).unwrap();
            Ok(r)
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.ref_
                    .index(field)
                    .unwrap()
                    .apply_lua(ctx, new_val)
                    .unwrap();
                Ok(())
            },
        );
    }
}
/// A higher level trait for allowing types to be interpreted as custom lua proxy types (or just normal types, this interface is flexible).
/// Types implementing this trait can have [`ReflectLuaProxyable`] type data registrations inserted into the reflection API.
///
/// Types registered via the reflection API this way can be accessed from Lua via [`ScriptRef`] objects (via field access).
pub trait LuaProxyable {
    /// a version of [`mlua::ToLua::to_lua`] which does not consume the object.
    ///
    /// Note: The self reference is sourced from the given ScriptRef, attempting to get another mutable reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn ref_to_lua(self_: ScriptRef, lua: &Lua) -> mlua::Result<Value>;

    /// similar to [`Reflect::apply`]
    ///
    /// Note:
    /// The self reference is sourced from the given ScriptRef, attempting to get another reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn apply_lua<'lua>(
        self_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()>;
}

/// Exactly alike to [`mlua::ToLua`]
pub trait FromLuaProxy<'lua>: Sized {
    fn from_lua_proxy(new_val: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self>;
}

/// Exactly alike to [`mlua::FromLua`]
pub trait ToLuaProxy<'lua> {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;
}

/// A struct providing type data for the `LuaProxyable` trait.
///
/// This allows casting static methods from the `LuaProxyable trait`.
#[derive(Clone)]
pub struct ReflectLuaProxyable {
    ref_to_lua: for<'lua> fn(ref_: ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>,
    apply_lua: for<'lua> fn(
        ref_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()>,
}

impl ReflectLuaProxyable {
    pub fn ref_to_lua<'lua>(&self, ref_: ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        (self.ref_to_lua)(ref_, lua)
    }

    pub fn apply_lua<'lua>(
        &self,
        ref_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()> {
        (self.apply_lua)(ref_, lua, new_val)
    }
}

impl<T: LuaProxyable + ::bevy::reflect::Reflect> ::bevy::reflect::FromType<T>
    for ReflectLuaProxyable
{
    fn from_type() -> Self {
        Self {
            ref_to_lua: T::ref_to_lua,
            apply_lua: T::apply_lua,
        }
    }
}

/// A dummy trait used to combat rust's orphan rules
///
/// In the future when trait specialization is a thing, this might be a companion trait
/// to `RefLuaType` which allows non Clone types to be used
pub trait ValueLuaType {}

impl<T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> LuaProxyable for T {
    fn ref_to_lua(self_: ScriptRef, lua: &Lua) -> mlua::Result<Value> {
        self_.get_typed(|s: &Self| s.clone().to_lua(lua))?
    }

    fn apply_lua<'lua>(
        self_: &mut ScriptRef,
        _: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()> {
        if let Value::UserData(v) = new_val {
            let o = v.borrow::<T>()?;

            self_.get_mut_typed(|s| *s = o.clone())?;

            Ok(())
        } else {
            Err(mlua::Error::RuntimeError(
                "Error in assigning to custom user data".to_owned(),
            ))
        }
    }
}

impl<'lua, T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> FromLuaProxy<'lua> for T {
    fn from_lua_proxy(new_val: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        T::from_lua(new_val, lua)
    }
}

impl<'lua, T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> ToLuaProxy<'lua> for T {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.to_lua(lua)
    }
}
