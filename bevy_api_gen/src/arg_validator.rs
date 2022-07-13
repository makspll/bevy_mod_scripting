use rustdoc_types::Type;

use crate::{Config, WrappedItem};


pub(crate) fn is_valid_parameter(str : &str, config : &Config, wrapper_prefix: &str) -> bool{
    const FROM_PRIMITIVES : [&str;18] = ["bool","StdString","CString","BString","i8","u8","i16","u16","i32","u32","i64","u64","i128","u128","isize","usize","f32","f64"];

    // we also allow references to these, since we can just reference by value
    // but we do not allow mutable versions since that can easilly cause unexpected behaviour
    let base_string = 
        if str.starts_with("&"){
            &str[1..]
        } else if str.starts_with("& mut") {
            &str[4..]
        } else {
            &str[..]
        };
    
    if  base_string == "self" ||
        FROM_PRIMITIVES.contains(&base_string) ||
        (base_string.starts_with("Lua") &&
        config.types.contains_key(&base_string[wrapper_prefix.len()..])){
        return true
    };

    false
}

pub(crate) fn is_valid_return_type(str : &str, config : &Config, wrapper_prefix: &str) -> bool{
    const TO_PRIMITIVES : [&str;21] = ["bool","StdString","Box<str>","CString","&CStr","BString","&BStr","i8","u8","i16","u16","i32","u32","i64","u64","i128","u128","isize","usize","f32","f64"];

    // TODO: support slices of supported types + Cow strings

    // we also allow references to these, since we can just reference by value
    // but we do not allow mutable versions since that can easilly cause unexpected behaviour
    
    if TO_PRIMITIVES.contains(&str) ||
        (str.starts_with("Lua") &&
        config.types.contains_key(&str[wrapper_prefix.len()..])){
        return true
    };

    false
}

/// standardizes simple function arguments identifiers to auto method macro format
pub(crate) fn to_auto_method_argument(base_string : &String, wrapped: &WrappedItem, config : &Config, is_first_arg : bool, wrapper_prefix: &str) -> Result<String,String>{
    let underlying_type = 
        if base_string == "Self"{
            if is_first_arg {
                return Ok("self".to_owned())
            } else {
                wrapped.wrapped_type
            }
        } else {
            base_string
        };

    if config.types.contains_key(underlying_type){
        // wrap things that need wrapped
        Ok(format!("{wrapper_prefix}{underlying_type}"))
    } else if config.primitives.contains(underlying_type) {
        Ok(underlying_type.to_string())
    } else {
        Err(underlying_type.to_owned())
    }
    
}

pub(crate) fn to_op_argument(base_string: &String, self_type : &String, wrapped : &WrappedItem, config : &Config, is_first_arg : bool, is_return_type : bool, wrapper_prefix: &str) -> Result<String,String>{
        // first of all deal with Self arguments
    // return if needs to just be self
    // otherwise get unwrapped type name

    let self_on_lhs = self_type == wrapped.wrapped_type;

    let underlying_type = 
        if base_string == "Self" && self_on_lhs && is_first_arg{
            return Ok("self".to_owned())
        } else if base_string == "Self"{
            &self_type
        } else {
            if !self_on_lhs && !is_return_type{
                return Ok("self".to_owned())
            } else{
                base_string
            }
        };

    if config.types.contains_key(underlying_type){
        // wrap things that need wrapped
        Ok(format!("{wrapper_prefix}{underlying_type}"))
    } else if config.primitives.contains(underlying_type) {
        Ok(underlying_type.to_string())
    } else {
        Err(underlying_type.to_owned())
    }
}


/// Converts an arbitary type to its simple string representation while converting the base type identifier with the given function
pub(crate) fn type_to_string<F : Fn(&String) -> Result<String,String>>(t : &Type, f : &F) -> Result<String,String> {
    match t {
        Type::ResolvedPath { name, .. } | 
        Type::Generic(name) => f(name),
        Type::Primitive(v) => Ok(v.to_string()),
        Type::Tuple(v) => Ok(format!("({})",v.iter().map(|t| type_to_string(t,f.clone())).collect::<Result<Vec<_>,_>>()?.join(","))),
        Type::Slice(v) => Ok(format!("[{}]",type_to_string(v,f)?)),
        Type::Array { type_, len } => Ok(format!("[{};{}]",type_to_string(type_,f)?,len)),
        Type::BorrowedRef { lifetime, mutable, type_ } => {
            
            let base = type_to_string(type_,f)?;
            let inner = format!("&{}{}{}",
                lifetime.as_ref()
                        .map(|v| format!("'{v} "))
                        .unwrap_or_default(),
                mutable.then(|| "mut ")
                    .unwrap_or_default(),
                base
            );
            Ok(inner)
            
        },
        _ => Err(format!("{t:?}"))
    }
}
