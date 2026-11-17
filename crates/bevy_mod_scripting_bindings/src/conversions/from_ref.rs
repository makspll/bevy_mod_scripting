//! Contains the [`FromScriptErased`] trait and its implementations.

use crate::{
    FromScript, ScriptValue, error::InteropError, match_by_type,
    reflection_extensions::TypeInfoExtensions,
};
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{
    PartialReflect, ReflectKind,
    enums::{DynamicEnum, DynamicVariant},
    list::DynamicList,
    map::{DynamicMap, Map},
    set::{DynamicSet, Set},
    tuple::DynamicTuple,
};
use std::{any::TypeId, ffi::OsString, path::PathBuf};

/// Converts from a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
///
/// Type Erased version of [`super::from::FromScript`].
pub trait FromScriptErased {
    /// Converts a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
    fn from_script_erased(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized;
}

impl FromScriptErased for Box<dyn PartialReflect> {
    #[profiling::function]
    fn from_script_erased(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized,
    {
        // the primitives don't borrow from the world
        let primitive = world.with_call_scope(|scope| {
            match_by_type! (
            match target {
                ta : usize   => return Ok(<usize>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tb : isize   => return Ok(<isize>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tc : u8      => return Ok(<u8>::from_script(value, scope).map(|a| Box::new(a) as _)),
                td : u16     => return Ok(<u16>::from_script(value, scope).map(|a| Box::new(a) as _)),
                te : u32     => return Ok(<u32>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tf : u64     => return Ok(<u64>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tg : u128    => return Ok(<u128>::from_script(value, scope).map(|a| Box::new(a) as _)),
                th : i8      => return Ok(<i8>::from_script(value, scope).map(|a| Box::new(a) as _)),
                ti : i16     => return Ok(<i16>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tj : i32     => return Ok(<i32>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tk : i64     => return Ok(<i64>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tl : i128    => return Ok(<i128>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tm : f32     => return Ok(<f32>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tn : f64     => return Ok(<f64>::from_script(value, scope).map(|a| Box::new(a) as _)),
                to : bool    => return Ok(<bool>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tp : char    => return Ok(<char>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tq : String  => return Ok(<String>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tr : PathBuf => return Ok(<PathBuf>::from_script(value, scope).map(|a| Box::new(a) as _)),
                ts : OsString=> return Ok(<OsString>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tsv: ScriptValue => return Ok(<ScriptValue>::from_script(value, scope).map(|a| Box::new(a) as _)),
                tn : ()      => return Ok(<()>::from_script(value, scope).map(|a| Box::new(a) as _))
            }
            );
            Err(value)
        });
        let value = match primitive {
            Ok(converted) => return converted,
            Err(value) => value,
        };

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        let type_info = type_registry.get_type_info(target).ok_or_else(|| {
            InteropError::missing_type_data(
                target,
                "Type was not registered, could not determine conversion strategy.".to_owned(),
            )
        })?;

        if let Some(inner_option_type) = type_info.option_inner_type() {
            let mut dynamic_enum = match value {
                ScriptValue::Unit => DynamicEnum::new("None", DynamicVariant::Unit),
                _ => {
                    let inner = Self::from_script_erased(inner_option_type, value, world.clone())?;
                    DynamicEnum::new(
                        "Some",
                        DynamicVariant::Tuple(DynamicTuple::from_iter(vec![inner])),
                    )
                }
            };

            dynamic_enum.set_represented_type(Some(type_info));
            return Ok(Box::new(dynamic_enum));
        }

        // speed up lookups for non-complex things
        if matches!(
            type_info.kind(),
            ReflectKind::Set | ReflectKind::List | ReflectKind::Map
        ) {
            if let Some(inner_list_type) = type_info.list_inner_type()
                && let ScriptValue::List(vec) = value
            {
                let mut dynamic_list = DynamicList::default();
                for item in vec {
                    let inner = Self::from_script_erased(inner_list_type, item, world.clone())?;
                    dynamic_list.push_box(inner);
                }

                dynamic_list.set_represented_type(Some(type_info));
                return Ok(Box::new(dynamic_list));
            }

            if let Some((key_type, val_type)) = type_info.map_inner_types()
                && let ScriptValue::Map(map) = value
            {
                let mut dynamic_map = DynamicMap::default();
                for (key, val) in map {
                    let key = Self::from_script_erased(
                        key_type,
                        ScriptValue::String(key.into()),
                        world.clone(),
                    )?;
                    let val = Self::from_script_erased(val_type, val, world.clone())?;
                    dynamic_map.insert_boxed(key, val);
                }
                dynamic_map.set_represented_type(Some(type_info));
                return Ok(Box::new(dynamic_map));
            }

            if let Some(val_type) = type_info.set_inner_type()
                && let ScriptValue::List(set) = value
            {
                let mut dynamic_set = DynamicSet::default();
                for val in set {
                    let key = Self::from_script_erased(val_type, val, world.clone())?;
                    dynamic_set.insert_boxed(key);
                }
                dynamic_set.set_represented_type(Some(type_info));
                return Ok(Box::new(dynamic_set));
            }
        }

        match value {
            ScriptValue::Reference(reflect_reference) => {
                reflect_reference.to_owned_value(world.clone())
            }
            value => Err(InteropError::value_mismatch(target, value)),
        }
    }
}
