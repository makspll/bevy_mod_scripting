use std::ops::{Deref, DerefMut};

use bevy::prelude::Entity;
use bevy_mod_scripting_core::bindings::{
    ReflectAllocator, ReflectReference, ScriptQueryBuilder, ScriptTypeRegistration,
};
use tealr::{
    mlu::{
        mlua::{IntoLua, Value},
        TealData, TypedFunction,
    },
    ToTypename,
};

use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata, util::Variadic};

use super::{
    // proxy::{LuaProxied, LuaValProxy},
    reference::LuaReflectReference,
    world::GetWorld,
};

#[derive(Default, Clone)]
pub struct LuaQueryBuilder(pub(crate) ScriptQueryBuilder);

impl Deref for LuaQueryBuilder {
    type Target = ScriptQueryBuilder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaQueryBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LuaQueryBuilder {}

impl LuaProxied for ScriptQueryBuilder {
    type Proxy = LuaQueryBuilder;
}

impl_userdata_from_lua!(LuaQueryBuilder);
impl_userdata_with_tealdata!(LuaQueryBuilder);

impl TealData for LuaQueryBuilder {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "with",
            |_, (mut this, mut query): (Self, Variadic<LuaValProxy<ScriptTypeRegistration>>)| {
                let registrations: Vec<_> = query
                    .iter_mut()
                    .map(|v| v.unproxy())
                    .collect::<Result<_, _>>()
                    .map_err(tealr::mlu::mlua::Error::external)?;
                this.0.with(registrations);
                Ok(this)
            },
        );

        methods.add_function(
            "without",
            |_, (mut this, mut query): (Self, Variadic<LuaValProxy<ScriptTypeRegistration>>)| {
                let registrations: Vec<_> = query
                    .iter_mut()
                    .map(|v| v.unproxy())
                    .collect::<Result<_, _>>()
                    .map_err(tealr::mlu::mlua::Error::external)?;
                this.0.without(registrations);
                Ok(this)
            },
        );

        methods.add_function("iter", |l, this: LuaQueryBuilder| {
            let world = l.get_world();
            let mut result = world
                .query(this.0)
                .map_err(tealr::mlu::mlua::Error::external)?;
            let mut len = result.len();
            let iterator = TypedFunction::from_rust_mut(
                move |lua, ()| {
                    if len > 0 {
                        let result = result
                            .pop_front()
                            .expect("invariant: len of array = len && len > 0");
                        let entity =
                            world.with_resource::<ReflectAllocator, _, _>(|_, mut allocator| {
                                ReflectReference::new_allocated(result.0, &mut allocator)
                            });
                        let proxy_entity =
                            <Entity as LuaProxied>::Proxy::from(entity).into_lua(lua)?;
                        let component_refs: Vec<_> = result
                            .1
                            .into_iter()
                            .map(|r| LuaReflectReference(r).to_lua_proxy(lua))
                            .collect::<Result<_, _>>()?;

                        len -= 1;

                        Ok(Variadic::new(
                            std::iter::once(proxy_entity).chain(component_refs.into_iter()),
                        ))
                    } else {
                        Ok(Variadic::new(vec![Value::Nil, Value::Nil]))
                    }
                },
                l,
            )?;
            Ok(iterator)
        });
    }
}

impl ToTypename for LuaQueryBuilder {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("QueryBuilder", tealr::KindOfType::External)
    }
}
