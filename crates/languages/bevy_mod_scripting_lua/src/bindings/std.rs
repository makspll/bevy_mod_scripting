use tealr::{
    mlu::{mlua::IntoLua, TealData},
    ToTypename,
};

pub struct LuaResult<T, E>(Result<T, E>);

impl<T, E> TealData for LuaResult<T, E>
where
    T: ToTypename + for<'l> IntoLua<'l>,
    E: ToTypename + for<'l> IntoLua<'l>,
{
    fn add_methods<'lua, M: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_ok", |_, this, _: ()| Ok(this.0.is_ok()));
        methods.add_method("is_err", |_, this, _: ()| Ok(this.0.is_err()));
        methods.add_function("unwrap", |_, this: LuaResult<T, E>| match this.0 {
            Ok(value) => Ok(value),
            Err(_) => Err(tealr::mlu::mlua::Error::RuntimeError(
                "called `LuaResult::unwrap()` on an `Err` value".to_string(),
            )),
        });
        methods.add_method("unwrap_err", |_, this, _: ()| match &this.0 {
            Ok(_) => Err(tealr::mlu::mlua::Error::RuntimeError(
                "called `LuaResult::unwrap_err()` on an `Ok` value".to_string(),
            )),
            Err(value) => Ok(value),
        });
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

impl<T: ToTypename, E: ToTypename> ToTypename for LuaResult<T, E> {
    fn to_typename() -> tealr::Type {
        let t = std::any::type_name::<T>();
        let e = std::any::type_name::<E>();
        tealr::Type::new_single(format!("LuaResult<{t},{e}>"), tealr::KindOfType::External)
    }
}
