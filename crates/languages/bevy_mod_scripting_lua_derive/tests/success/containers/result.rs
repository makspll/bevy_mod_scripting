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

#[derive(ScriptProxy, Reflect, Clone)]
#[proxy(languages("lua"), derive(Clone))]
#[functions[
    #[lua(Function)]
    fn fn_returning_usize_result() -> Result<usize, MyError> {
        Ok(2)
    }

    #[lua(Function)]
    fn fn_returning_usize_result_err() -> Result<usize, MyError> {
        Err(MyError)
    }


    #[lua(Function, output(proxy))]
    fn fn_returning_ok_proxy() -> Result<Self, MyError> {
        Ok(MyStruct)
    }

    #[lua(Function, output(proxy))]
    fn fn_returning_err_proxy() -> Result<Self, MyError> {
        Err(MyError)
    }
]]
pub struct MyStruct;

pub fn main() {}
