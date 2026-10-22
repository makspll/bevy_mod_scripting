//! Contains the [`IntoScriptErased`] trait and its implementations.

use std::{borrow::Cow, ffi::OsString, path::PathBuf};

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_display::OrFakeId;
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::PartialReflect;

use crate::{
    ReferencePart, ReflectReference, ScriptValue, error::InteropError, function::into::IntoScript,
    reflection_extensions::PartialReflectExt,
};

/// Converts a value represented by a reference into a [`crate::ScriptValue`].
/// Instead of a direct conversion, the trait tries to peek into the value behind the reference and find out the most suitable representation.
///
/// Type Erased version of [`super::from::FromScript`].
///
/// - Primitives are converted to simple values
/// - Container types are converted to references (so the references persist after accesses inside them)
pub trait IntoScriptErased {
    /// Converts a value represented by a reference into a [`crate::ScriptValue`].
    fn into_script_erased(
        self_: ReflectReference,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError>;
}

/// a utility for matching types by their [`std::any::TypeId`]
#[macro_export]
macro_rules! match_by_type {
    (match $on:ident {$($id:ident : $ty:ty => $conv:expr),*}) => {
        match $on {
        $(
            $id if $id == std::any::TypeId::of::<$ty>() => {$conv},
        )*
            _ => {},
        }
    };
}

pub use match_by_type;

#[macro_export]
/// Downcasts a reference into a value of a given type or returns an error if the downcast fails.
macro_rules! downcast_into_value {
    ($r:ident, $ty:ty) => {
        *$r.try_downcast_ref::<$ty>().ok_or_else(|| {
            InteropError::type_mismatch(
                std::any::TypeId::of::<$ty>(),
                $r.get_represented_type_info().map(|i| i.type_id()),
            )
        })?
    };
}

#[profiling::all_functions]
impl IntoScriptErased for ReflectReference {
    #[profiling::function]
    fn into_script_erased(
        self_: ReflectReference,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        let erased = self_.with_reflect(world.clone(), |r| into_script_erased(r, world))??;
        Ok(match erased {
            Erased::Value(value) => value,
            Erased::Reference { somes } => {
                let mut self_ = self_;
                for _ in 0..somes {
                    // don't want correction to mess with this
                    self_.push_path(ReferencePart::IntegerAccess(0, false));
                }
                ScriptValue::Reference(self_)
            }
        })
    }
}

fn into_script_erased(r: &dyn PartialReflect, world: WorldGuard) -> Result<Erased, InteropError> {
    let type_id = r
        .get_represented_type_info()
        .map(|i| i.type_id())
        .or_fake_id();

    let value = |v: Result<ScriptValue, InteropError>| v.map(Erased::Value);
    match_by_type! (
        match type_id {
            ta : usize   => return value(downcast_into_value!(r, usize).into_script(world)),
            tb : isize   => return value(downcast_into_value!(r, isize).into_script(world)),
            tc : u8      => return value(downcast_into_value!(r, u8).into_script(world)),
            td : u16     => return value(downcast_into_value!(r, u16).into_script(world)),
            te : u32     => return value(downcast_into_value!(r, u32).into_script(world)),
            tf : u64     => return value(downcast_into_value!(r, u64).into_script(world)),
            tg : u128    => return value(downcast_into_value!(r, u128).into_script(world)),
            th : i8      => return value(downcast_into_value!(r, i8).into_script(world)),
            ti : i16     => return value(downcast_into_value!(r, i16).into_script(world)),
            tj : i32     => return value(downcast_into_value!(r, i32).into_script(world)),
            tk : i64     => return value(downcast_into_value!(r, i64).into_script(world)),
            tl : i128    => return value(downcast_into_value!(r, i128).into_script(world)),
            tm : f32     => return value(downcast_into_value!(r, f32).into_script(world)),
            tn : f64     => return value(downcast_into_value!(r, f64).into_script(world)),
            to : bool    => return value(downcast_into_value!(r, bool).into_script(world)),
            tp : char    => return value(downcast_into_value!(r, char).into_script(world)),
            tq : String  => return value(downcast_into_value!(r, String).clone().into_script(world)),
            tcs: Cow<'static, str> => match r.try_downcast_ref::<Cow<'static, str>>() {
                Some(cow) => return Ok(Erased::Value(ScriptValue::String(cow.clone()))),
                None => return Err(InteropError::type_mismatch(
                    std::any::TypeId::of::<Cow<str>>(),
                    r.get_represented_type_info().map(|i| i.type_id()),
                )),
            },
            tr : PathBuf => return value(downcast_into_value!(r, PathBuf).clone().into_script(world)),
            ts : OsString=> return value(downcast_into_value!(r, OsString).clone().into_script(world)),
            tsv: ScriptValue=> return Ok(Erased::Value(downcast_into_value!(r, ScriptValue).clone())),
            tent: Entity => return Ok(Erased::Value(ScriptValue::Entity(downcast_into_value!(r, Entity)))),
            tn : ()      => return Ok(Erased::Value(ScriptValue::Unit))
        }
    );

    // either return nil or ref into
    if let Ok(as_option) = r.as_option() {
        return match as_option {
            Some(s) => into_script_erased(s, world).map(Erased::inside_some),
            None => Ok(Erased::Value(ScriptValue::Unit)),
        };
    }

    Ok(Erased::Reference { somes: 0 })
}

/// What a reference converts to, worked out while borrowing the value, so the reference is only moved into the result when needed.
enum Erased {
    /// A value converted on its own, i.e. a primitive
    Value(ScriptValue),
    /// The reference itself, through `somes` levels of `Some` (i.e. `Option<Option<T>>`)
    Reference { somes: usize },
}

impl Erased {
    fn inside_some(self) -> Self {
        match self {
            Erased::Reference { somes } => Erased::Reference { somes: somes + 1 },
            value => value,
        }
    }
}
