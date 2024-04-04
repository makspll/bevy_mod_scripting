use bevy::prelude::*;
use bevy_mod_scripting::api::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MyError;

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

#[derive(LuaProxy, Reflect, Clone)]
#[proxy(functions = [
    r#"
    #[lua(kind="Function")]
    fn fn_returning_usize_result() -> Result<usize, MyError> {
        Ok(2)
    }
    "#,

    r#"
    #[lua(kind="Function")]
    fn fn_returning_usize_result_err() -> Result<usize, MyError> {
        Err(MyError)
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_ok_proxy() -> Result<Self, MyError> {
        Ok(MyStruct)
    }
    "#,

    r#"
    #[lua(kind="Function", output(proxy))]
    fn fn_returning_err_proxy() -> Result<Self, MyError> {
        Err(MyError)
    }
    "#,
])]
pub struct MyStruct;

pub fn main() {}
