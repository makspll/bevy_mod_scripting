//! Contains the [`FromScriptRef`] trait and its implementations.

use crate::{
    FromScript, ScriptValue, WorldGuard, error::InteropError, match_by_type,
    reflection_extensions::TypeInfoExtensions,
};
use bevy_reflect::{
    DynamicEnum, DynamicList, DynamicMap, DynamicSet, DynamicTuple, DynamicVariant, Map,
    PartialReflect, ReflectKind, Set,
};
use std::{any::TypeId, ffi::OsString, path::PathBuf};

/// Converts from a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
///
/// Type Erased version of [`super::from::FromScript`].
pub trait FromScriptRef {
    /// Converts a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
    fn from_script_ref(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized;
}

impl FromScriptRef for Box<dyn PartialReflect> {
    #[profiling::function]
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
                tsv: ScriptValue => return <ScriptValue>::from_script(value, world).map(|a| Box::new(a) as _),
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

        if let Some(inner_option_type) = type_info.option_inner_type() {
            let mut dynamic_enum = match value {
                ScriptValue::Unit => DynamicEnum::new("None", DynamicVariant::Unit),
                _ => {
                    let inner = Self::from_script_ref(inner_option_type, value, world)?;
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
                    let inner = Self::from_script_ref(inner_list_type, item, world.clone())?;
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
                    let key = Self::from_script_ref(
                        key_type,
                        ScriptValue::String(key.into()),
                        world.clone(),
                    )?;
                    let val = Self::from_script_ref(val_type, val, world.clone())?;
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
                    let key = Self::from_script_ref(val_type, val, world.clone())?;
                    dynamic_set.insert_boxed(key);
                }
                dynamic_set.set_represented_type(Some(type_info));
                return Ok(Box::new(dynamic_set));
            }
        }

        match value {
            ScriptValue::Reference(reflect_reference) => reflect_reference.to_owned_value(world),
            value => Err(InteropError::value_mismatch(target, value)),
        }
    }
}
