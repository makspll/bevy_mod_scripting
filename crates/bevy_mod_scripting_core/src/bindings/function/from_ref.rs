use std::{any::TypeId, ffi::OsString, path::PathBuf};

use bevy::reflect::{DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, PartialReflect};

use crate::{
    bindings::{function::from::FromScript, WorldGuard},
    error::InteropError,
    match_by_type,
    prelude::ScriptValue,
    reflection_extensions::TypeInfoExtensions,
};

/// Converts from a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
///
/// Type Erased version of [`super::from::FromScript`].
pub trait FromScriptRef {
    fn from_script_ref(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized;
}

impl FromScriptRef for Box<dyn PartialReflect> {
    fn from_script_ref(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized,
    {
        match_by_type! (
            match target {
                ta : usize   => return <usize>::from_script(value, world).map(|a| Box::new(a) as _),
                tb : isize   => return <isize>::from_script(value, world).map(|a| Box::new(a) as _),
                tc : u8      => return <u8>::from_script(value, world).map(|a| Box::new(a) as _),
                td : u16     => return <u16>::from_script(value, world).map(|a| Box::new(a) as _),
                te : u32     => return <u32>::from_script(value, world).map(|a| Box::new(a) as _),
                tf : u64     => return <u64>::from_script(value, world).map(|a| Box::new(a) as _),
                tg : u128    => return <u128>::from_script(value, world).map(|a| Box::new(a) as _),
                th : i8      => return <i8>::from_script(value, world).map(|a| Box::new(a) as _),
                ti : i16     => return <i16>::from_script(value, world).map(|a| Box::new(a) as _),
                tj : i32     => return <i32>::from_script(value, world).map(|a| Box::new(a) as _),
                tk : i64     => return <i64>::from_script(value, world).map(|a| Box::new(a) as _),
                tl : i128    => return <i128>::from_script(value, world).map(|a| Box::new(a) as _),
                tm : f32     => return <f32>::from_script(value, world).map(|a| Box::new(a) as _),
                tn : f64     => return <f64>::from_script(value, world).map(|a| Box::new(a) as _),
                to : bool    => return <bool>::from_script(value, world).map(|a| Box::new(a) as _),
                tp : char    => return <char>::from_script(value, world).map(|a| Box::new(a) as _),
                tq : String  => return <String>::from_script(value, world).map(|a| Box::new(a) as _),
                tr : PathBuf => return <PathBuf>::from_script(value, world).map(|a| Box::new(a) as _),
                ts : OsString=> return <OsString>::from_script(value, world).map(|a| Box::new(a) as _),
                tn : ()      => return <()>::from_script(value, world).map(|a| Box::new(a) as _)
            }
        );

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        let type_info = type_registry.get_type_info(target).ok_or_else(|| {
            InteropError::missing_type_data(
                target,
                "Type was not registered, could not determine conversion strategy.".to_owned(),
            )
        })?;

        if type_info.is_option() {
            let inner_type = type_info.option_inner_type().expect("invariant");
            let mut dynamic_enum = match value {
                ScriptValue::Unit => DynamicEnum::new("None", DynamicVariant::Unit),
                _ => {
                    let inner = Self::from_script_ref(inner_type, value, world)?;
                    DynamicEnum::new(
                        "Some",
                        DynamicVariant::Tuple(DynamicTuple::from_iter(vec![inner])),
                    )
                }
            };

            dynamic_enum.set_represented_type(Some(type_info));
            return Ok(Box::new(dynamic_enum));
        }

        if type_info.is_list() {
            let inner_type = type_info.list_inner_type().expect("invariant");

            if let ScriptValue::List(vec) = value {
                let mut dynamic_list = DynamicList::default();
                for item in vec {
                    let inner = Self::from_script_ref(inner_type, item, world.clone())?;
                    dynamic_list.push_box(inner);
                }

                dynamic_list.set_represented_type(Some(type_info));
                return Ok(Box::new(dynamic_list));
            }
        }

        match value {
            ScriptValue::Reference(reflect_reference) => reflect_reference.to_owned_value(world),
            value => Err(InteropError::value_mismatch(target, value)),
        }
    }
}
