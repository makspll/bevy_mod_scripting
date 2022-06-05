use rlua::{UserData,MetaMethod,Function,prelude::*};
use bevy::prelude::Quat;

#[derive(Debug,Clone)]
pub struct MyUserData {
    quat: Quat
}

impl UserData for MyUserData {
    fn add_methods<'lua, T: LuaUserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_,s,()| {
            Ok(format!("{:?}",s))
        });

        methods.add_meta_method_mut(MetaMethod::NewIndex, |_,s,(_,val) : (String,MyUserData)|{
            s.quat = val.quat.clone();
            Ok(())
        })
    }
}


fn main() {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let g = lua_ctx.globals();
        let f = lua_ctx.create_function(|_,(x,y,z,w):(f32,f32,f32,f32)| 
                Ok(MyUserData{quat:Quat::from_xyzw(x, y, z, w)
            })).unwrap();

        g.set("quat",f).unwrap();

        lua_ctx
            .load("
            function on_update(my_user_data)
                print(my_user_data)
                my_user_data.quat = quat(4,3,2,1)
                print(my_user_data)
            end
            ")
            .exec()
    }).unwrap();

    lua.context(|lua_ctx| {
        let g = lua_ctx.globals();
        let f : Function = g.get("on_update").unwrap();

        f.call::<MyUserData,()>(MyUserData{
            quat : Quat::from_xyzw(1.0,2.0,3.0,4.0),
        }).unwrap();
    })

}
