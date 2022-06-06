use bevy_mod_scripting::impl_lua_newtype;



pub struct Test {

}

impl_lua_newtype!{
    Test : Vector + Matrix

    impl {
        "hello" => |_,()| {Ok(())};
        mut "hello_mut" => |_,()| {Ok(())};
    }

}


