use std::fmt;

use bevy::reflect::{Reflect, ReflectRef};



pub struct PrintableReflect<'a>(pub &'a dyn Reflect);

impl fmt::Debug for PrintableReflect<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.print(f)
    }
}

pub trait PrintReflect {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}


/// Implements tealr::TypeName,tealr::TypeBody and mlua::Userdata based on non-generic single token type name implementing TealData
macro_rules! impl_tealr_type {
    ($v:ty) => {
        impl tealr::TypeName for $v {
            fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
                std::borrow::Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
                    name: std::borrow::Cow::Borrowed(stringify!($v)),
                    generics: None,
                    type_kind: tealr::KindOfType::External,
                })]) 
            }
        }

        impl mlua::UserData  for $v{
            fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
                let mut wrapper = ::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
                <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper)
            }
        
            fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
                let mut x = ::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut x);
            }
        } 
        
        impl tealr::TypeBody for $v{
            fn get_type_body() -> tealr::TypeGenerator {
                let mut gen = ::tealr::RecordGenerator::new::<Self>(false);
                gen.is_user_data = true;
                <Self as ::tealr::mlu::TealData>::add_fields(&mut gen);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut gen);
                <_ as ::std::convert::From<_>>::from(gen)    
            }
        }
        
    }
}
pub(crate) use impl_tealr_type;

macro_rules! impl_downcast_print_cases {
    ($v:ident,$fmt:ident,$f:ty,$($x:ty),*) => {
        {

            if let Some(i) = $v.downcast_ref::<$f>(){
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            $(
            else if let Some(i) = $v.downcast_ref::<$x>() {
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            )*
            else {
                write!($fmt,"({:#?}){:?}",$v.type_name(),$v)?;
            }
        }
    };
}

impl<T: Reflect + ?Sized> PrintReflect for &T {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reflect_ref() {
            ReflectRef::Struct(s) => (0..s.field_len())
                .fold(f.debug_struct(s.type_name()), |mut b, i| {
                    b.field(
                        s.name_at(i).unwrap(),
                        &PrintableReflect(s.field_at(i).unwrap()),
                    );
                    b
                })
                .finish(),
            ReflectRef::Map(m) => m
                .iter()
                .fold(f.debug_map(), |mut b, (k, v)| {
                    b.entry(&PrintableReflect(k), &PrintableReflect(v));
                    b
                })
                .finish(),
            ReflectRef::TupleStruct(ts) => ts
                .iter_fields()
                .fold(f.debug_tuple(ts.type_name()), |mut b, i| {
                    b.field(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Tuple(t) => t
                .iter_fields()
                .fold(f.debug_tuple(""), |mut b, i| {
                    b.field(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::List(l) => l
                .iter()
                .fold(f.debug_list(), |mut b, i| {
                    b.entry(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Array(a) => a
                .iter()
                .fold(f.debug_list(), |mut b, i| {
                    b.entry(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Value(v) => {
                impl_downcast_print_cases!(
                    v, f, usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, f32, f64,
                    String
                );
                Ok(())
            }
        }
    }
}