use ::std::borrow::Cow;

use bevy::reflect::FromReflect;
use bevy::reflect::Reflect;

use bevy::reflect::TypePath;
use bevy_mod_scripting_lua::tealr;

use tealr::mlu::mlua::MetaMethod;
use tealr::mlu::TypedFunction;
use tealr::mlu::{
    mlua::{self, FromLua, Lua, IntoLua, UserData, Value},
    TealData, TealDataMethods,
};
use tealr::TypeBody;
use tealr::TypeName;

use paste::paste;

use crate::common::std::ScriptVec;
use crate::{
    error::ReflectionError,
    script_ref::{ScriptRef, ValueIndex},
    sub_reflect::ReflectPathElem,
};

use super::util::to_lua_idx;
use super::util::LuaIndex;
use super::ApplyLua;
use super::FromLuaProxy;
use super::LuaProxyable;
use super::ToLuaProxy;

/// Implements custom user data for simple copy types which implement to and from lua
macro_rules! impl_proxyable_by_copy(
    ( $($num_ty:ty),*) => {
        paste! {
            $(
                impl $crate::lua::LuaProxyable for $num_ty {
                    fn ref_to_lua(self_: $crate::script_ref::ScriptRef,lua: & tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value< '_> >  {
                        self_.get_typed(|self_ : &Self| self_.into_lua(lua))?
                    }

                    fn apply_lua< 'lua>(self_: &mut $crate::script_ref::ScriptRef,lua: & 'lua tealr::mlu::mlua::Lua,new_val:tealr::mlu::mlua::Value< 'lua>) -> tealr::mlu::mlua::Result<()>  {
                        self_.set_val(Self::from_lua(new_val,lua)?)?;
                        Ok(())
                    }
                }

                impl <'lua>$crate::lua::FromLuaProxy<'lua> for $num_ty {
                    #[inline(always)]
                    fn from_lua_proxy(new_value: Value<'lua>, lua: &'lua Lua) -> tealr::mlu::mlua::Result<Self> {
                        Self::from_lua(new_value,lua)
                    }
                }

                impl <'lua>$crate::lua::ToLuaProxy<'lua> for $num_ty {
                    #[inline(always)]
                    fn to_lua_proxy(self, lua: &'lua Lua) -> tealr::mlu::mlua::Result<Value<'lua>> {
                        self.into_lua(lua)
                    }
                }
            )*
        }
    }
);

impl_proxyable_by_copy!(bool);
impl_proxyable_by_copy!(f32, f64);
impl_proxyable_by_copy!(i8, i16, i32, i64, i128, isize);
impl_proxyable_by_copy!(u8, u16, u32, u64, u128, usize);

impl LuaProxyable for String {
    fn ref_to_lua(self_: ScriptRef, lua: &Lua) -> mlua::Result<Value> {
        self_.get_typed(|self_: &String| self_.as_str().into_lua(lua))?
    }

    fn apply_lua<'lua>(
        self_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()> {
        self_.get_mut_typed(|self_| {
            *self_ = Self::from_lua(new_val, lua)?;
            Ok(())
        })?
    }
}

impl<'lua> FromLuaProxy<'lua> for String {
    fn from_lua_proxy(new_val: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        Self::from_lua(new_val, lua)
    }
}

impl<'lua> ToLuaProxy<'lua> for String {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.into_lua(lua)
    }
}

impl<T: LuaProxyable + Reflect + FromReflect + TypePath + for<'a> FromLuaProxy<'a> + Clone>
    LuaProxyable for Option<T>
{
    fn ref_to_lua(self_: ScriptRef, lua: &Lua) -> mlua::Result<Value> {
        self_.get_typed(|s: &Option<T>| match s {
            Some(_) => T::ref_to_lua(
                self_.sub_ref(ReflectPathElem::SubReflection {
                    label: "as_ref",
                    get: |ref_| {
                        ref_.downcast_ref::<Option<T>>()
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_ref()
                            .map(|t| t as &dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                    get_mut: |ref_| {
                        ref_.downcast_mut::<Option<T>>()
                            // TODO: there is some weird borrow checker fuckery going on here
                            // i tried having from: ref_.type_name().to_owned().into() instead of "Reflect"
                            // and lying this out in an if let expression, but nothing will satisfy the borrow checker here, so leaving this for now
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: "Reflect".into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_mut()
                            .map(|t| t as &mut dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                }),
                lua,
            ),
            None => Ok(Value::Nil),
        })?
    }

    fn apply_lua<'lua>(
        self_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()> {
        if let Value::Nil = new_val {
            self_.get_mut_typed(|s: &mut Option<T>| {
                *s = None;
                Ok(())
            })?
        } else {
            // we need to do this in two passes, first
            // ensure that the target type is the 'some' variant to allow a sub reference
            let is_none = self_.get_typed(|s: &Option<T>| s.is_none())?;

            if is_none {
                return self_.get_mut_typed(|s: &mut Option<T>| {
                    *s = Some(T::from_lua_proxy(new_val, lua)?);
                    Ok::<_, mlua::Error>(())
                })?;
            }

            T::apply_lua(
                &mut self_.sub_ref(ReflectPathElem::SubReflection {
                    label: "",
                    get: |ref_| {
                        ref_.downcast_ref::<Option<T>>()
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_ref()
                            .map(|t| t as &dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                    get_mut: |ref_| {
                        if ref_.is::<Option<T>>() {
                            ref_.downcast_mut::<Option<T>>()
                                .unwrap()
                                .as_mut()
                                .map(|t| t as &mut dyn Reflect)
                                .ok_or_else(|| {
                                    ReflectionError::Other(
                                        "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                    )
                                })
                        } else {
                            Err(ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })
                        }
                    },
                }),
                lua,
                new_val,
            )
        }
    }
}

impl<'lua, T: for<'a> FromLuaProxy<'a>> FromLuaProxy<'lua> for Option<T> {
    fn from_lua_proxy(new_val: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Nil = new_val {
            Ok(None)
        } else {
            T::from_lua_proxy(new_val, lua).map(Option::Some)
        }
    }
}

impl<'lua, T: for<'a> ToLuaProxy<'a>> ToLuaProxy<'lua> for Option<T> {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        match self {
            Some(v) => v.to_lua_proxy(lua),
            None => Ok(Value::Nil),
        }
    }
}

/// A reference to a rust vec (vec reference proxy), does not need an owned variant since
/// lua can natively represent lists of things
pub type LuaVec<T> = ScriptVec<T>;

impl<
        T: TypeName
            + FromReflect
            + TypePath
            + LuaProxyable
            + for<'a> FromLuaProxy<'a>
            + for<'a> ToLuaProxy<'a>
            + std::fmt::Debug,
    > UserData for LuaVec<T>
{
    fn add_methods<'lua, M: tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        let mut x = tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
        <Self as tealr::mlu::TealData>::add_methods(&mut x);
    }
    fn add_fields<'lua, F: tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
        <Self as tealr::mlu::TealData>::add_fields(&mut wrapper)
    }
}

impl<T: TypeName> TypeName for LuaVec<T> {
    fn get_type_parts() -> Cow<'static, [tealr::NamePart]> {
        let mut parts = vec![
            tealr::NamePart::Type(tealr::TealType {
                name: Cow::Borrowed("LuaVec"),
                type_kind: tealr::KindOfType::External,
                generics: None,
            }),
            tealr::NamePart::Symbol("<".into()),
        ];
        parts.extend(T::get_type_parts().iter().cloned());
        parts.push(tealr::NamePart::Symbol(">".into()));
        parts.into()
    }
}

impl<
        T: TypeName
            + FromReflect
            + TypePath
            + LuaProxyable
            + for<'a> FromLuaProxy<'a>
            + for<'a> ToLuaProxy<'a>
            + std::fmt::Debug,
    > TypeBody for LuaVec<T>
{
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}

impl<
        T: TypeName
            + FromReflect
            + TypePath
            + LuaProxyable
            + for<'a> FromLuaProxy<'a>
            + for<'a> ToLuaProxy<'a>,
    > TealData for LuaVec<T>
{
    fn add_methods<'lua, M: TealDataMethods<'lua, Self>>(methods: &mut M) {
        methods.document_type("A reference to the Vec<T> Rust type.");
        methods.document_type("All indexing begins at 1.");

        methods.add_meta_method(MetaMethod::ToString, |_, s, ()| {
            Ok(s.ref_.get(|s| format!("{:?}", s))?)
        });

        methods.add_meta_method(MetaMethod::Index, |_, s, index: LuaIndex| {
            Ok(s.index(*index))
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, s, (index, value): (LuaIndex, Value)| s.index(*index).apply_lua(ctx, value),
        );

        bevy_mod_scripting_lua::__cfg_feature_any_lua52_lua53_lua54_luajit52!(
            methods.add_meta_method(
                MetaMethod::Pairs,
                |ctx, s, _: ()| {
                    let len = s.len()?;
                    let mut curr_idx = 0;
                    let ref_: ScriptRef = s.clone().into();
                    TypedFunction::from_rust_mut(
                        move |ctx, ()| {
                            let o = if curr_idx < len {
                                (
                                    to_lua_idx(curr_idx).into_lua(ctx)?,
                                    ref_.index(curr_idx).into_lua(ctx)?,
                                )
                            } else {
                                (Value::Nil, Value::Nil)
                            };
                            curr_idx += 1;
                            Ok(o)
                        },
                        ctx,
                    )
                },
            );
        );
        methods.add_meta_method(MetaMethod::Len, |_, s, ()| Ok(s.len()?));

        methods.add_method("to_table", |ctx, s, ()| {
            let table = ctx.create_table()?;
            let len = s.len()?;

            for i in 0..len {
                table.raw_set(to_lua_idx(i), s.index(i).into_lua(ctx)?)?;
            }

            Ok(table)
        });

        methods.add_method_mut("push", |ctx, s, v: Value| {
            let new_val = T::from_lua_proxy(v, ctx)?;
            s.push(new_val)?;
            Ok(())
        });

        methods.add_method_mut("pop", |ctx, s, ()| s.pop().map(|v| v.to_lua_proxy(ctx))?);

        methods.add_method_mut("clear", |_, s, ()| {
            s.clear()?;
            Ok(())
        });

        methods.add_method_mut("insert", |ctx, s, (idx, v): (LuaIndex, Value<'lua>)| {
            s.insert(*idx, T::from_lua_proxy(v, ctx)?)?;
            Ok(())
        });

        methods.add_method_mut("remove", |ctx, s, idx: LuaIndex| {
            let removed = s.remove(*idx)?;
            removed.to_lua_proxy(ctx)
        });
    }
}

impl<
        T: TypeName
            + FromReflect
            + TypePath
            + LuaProxyable
            + for<'a> FromLuaProxy<'a>
            + for<'a> ToLuaProxy<'a>
            + std::fmt::Debug,
    > LuaProxyable for Vec<T>
{
    fn ref_to_lua(self_: ScriptRef, lua: &Lua) -> mlua::Result<Value> {
        LuaVec::<T>::new_ref(self_).into_lua(lua)
    }

    fn apply_lua<'lua>(
        self_: &mut ScriptRef,
        lua: &'lua Lua,
        new_val: Value<'lua>,
    ) -> mlua::Result<()> {
        match &new_val {
            Value::UserData(ud) => {
                let lua_vec = ud.borrow::<LuaVec<T>>()?;
                self_.apply(&lua_vec.ref_)?;
            }
            Value::Table(table) => {
                let last_target_idx = self_.get_typed(|s: &Vec<T>| s.len())? - 1;
                // there is also another case to consider, Vec has a lua representation available as well (table)
                // if we receive one of those, we should also apply it
                for entry in table.clone().pairs::<usize, Value>() {
                    let (lua_idx, v) = entry?;
                    let idx = lua_idx - 1;
                    if idx > last_target_idx {
                        // here we don't need to do anything special just use LuaProxyable impl
                        T::apply_lua(&mut self_.index(idx), lua, v)?;
                    } else {
                        // here we don't have anything to apply this to
                        // use FromLua impl
                        self_.get_mut_typed(|s: &mut Vec<T>| {
                            s[idx] = T::from_lua_proxy(v, lua)?;
                            Ok::<_, mlua::Error>(())
                        })??;
                    }
                }
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: new_val.type_name(),
                    to: "userdata or table",
                    message: Some("LuaVec can only be assigned with itself or a table".to_owned()),
                })
            }
        }

        Ok(())
    }
}

impl<
        'lua,
        T: TypeName
            + for<'a> FromLuaProxy<'a>
            + for<'a> ToLuaProxy<'a>
            + Clone
            + FromReflect
            + TypePath
            + LuaProxyable
            + std::fmt::Debug,
    > FromLuaProxy<'lua> for Vec<T>
{
    fn from_lua_proxy(new_val: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        match new_val {
            Value::UserData(ud) => {
                let lua_vec = ud.borrow::<LuaVec<T>>()?;
                lua_vec.ref_.get_typed(|s: &Vec<T>| Ok(s.clone()))?
            }
            Value::Table(table) => {
                // there is also another case to consider, Vec has a lua representation available as well (table)
                // if we receive one of those, we should clone it one by one
                table
                    .pairs::<usize, Value>()
                    .map(|v| v.and_then(|(_, v)| T::from_lua_proxy(v, lua)))
                    .collect::<Result<Vec<_>, _>>()
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: new_val.type_name(),
                    to: "userdata or table",
                    message: Some("LuaVec can only be assigned with itself or a table".to_owned()),
                })
            }
        }
    }
}

impl<'lua, T: for<'a> ToLuaProxy<'a> + Clone + FromReflect + LuaProxyable> ToLuaProxy<'lua>
    for Vec<T>
{
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let proxies = lua.create_table()?;
        for (idx, elem) in self.into_iter().enumerate() {
            proxies.raw_set(idx, elem.to_lua_proxy(lua)?)?;
        }

        proxies.into_lua(lua)
    }
}
