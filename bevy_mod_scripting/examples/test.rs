use rlua::{Value,UserData,MetaMethod,Function,prelude::*};

#[derive(Debug)]
pub struct MyUserData {
    v : [u32;3]
}

impl UserData for MyUserData {
    fn add_methods<'lua, T: LuaUserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_,s,()| {
            Ok(format!("{:?}",s))
        });

        methods.add_meta_method_mut(MetaMethod::NewIndex, |_,s,(idx,val) : (Value,u32)|{
            let idx = match idx {
                Value::Integer(i) => i as usize,
                Value::String(s) => match s.to_str().unwrap() {
                    "x" => 0,
                    "y" => 1,
                    "z" => 2,
                    _ => panic!()
                },
                _ => panic!(),
            };

            Ok(s.v[idx] = val)
        })
    }
}


fn main() {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        lua_ctx
            .load("
            function on_update(my_user_data)
                print(my_user_data)
                my_user_data[0] = 69
                my_user_data.y = 42
                print(my_user_data)
            end
            ")
            .exec()
    }).unwrap();

    lua.context(|lua_ctx| {
        let g = lua_ctx.globals();
        let f : Function = g.get("on_update").unwrap();

        f.call::<MyUserData,()>(MyUserData{
            v: [0,1,2],
        }).unwrap();
    })

}
