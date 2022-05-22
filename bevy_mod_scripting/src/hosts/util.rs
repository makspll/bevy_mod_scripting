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
                write!($fmt,"({:#?})",$v.type_name())?;
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