//! Contains the [`IntoScriptRef`] trait and its implementations.

use std::{borrow::Cow, ffi::OsString, path::PathBuf};

use bevy_mod_scripting_display::OrFakeId;
use bevy_reflect::PartialReflect;

use crate::{
    ReferencePart, ReflectReference, ScriptValue, WorldGuard, error::InteropError,
    function::into::IntoScript, reflection_extensions::PartialReflectExt,
};

/// Converts a value represented by a reference into a [`crate::ScriptValue`].
/// Instead of a direct conversion, the trait tries to peek into the value behind the reference and find out the most suitable representation.
///
/// Type Erased version of [`super::from::FromScript`].
///
/// - Primitives are converted to simple values
/// - Container types are converted to references (so the references persist after accesses inside them)
pub trait IntoScriptRef {
    /// Converts a value represented by a reference into a [`crate::ScriptValue`].
    fn into_script_ref(
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
impl IntoScriptRef for ReflectReference {
    #[profiling::function]
    fn into_script_ref(
        self_: ReflectReference,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        self_.with_reflect(world.clone(), |r| into_script_ref(self_.clone(), r, world))?
    }
}

#[profiling::function]
fn into_script_ref(
    mut self_: ReflectReference,
    r: &dyn PartialReflect,
    world: WorldGuard,
) -> Result<ScriptValue, InteropError> {
    let type_id = r
        .get_represented_type_info()
        .map(|i| i.type_id())
        .or_fake_id();

    match_by_type! (
        match type_id {
            ta : usize   => return downcast_into_value!(r, usize).into_script(world),
            tb : isize   => return downcast_into_value!(r, isize).into_script(world),
            tc : u8      => return downcast_into_value!(r, u8).into_script(world),
            td : u16     => return downcast_into_value!(r, u16).into_script(world),
            te : u32     => return downcast_into_value!(r, u32).into_script(world),
            tf : u64     => return downcast_into_value!(r, u64).into_script(world),
            tg : u128    => return downcast_into_value!(r, u128).into_script(world),
            th : i8      => return downcast_into_value!(r, i8).into_script(world),
            ti : i16     => return downcast_into_value!(r, i16).into_script(world),
            tj : i32     => return downcast_into_value!(r, i32).into_script(world),
            tk : i64     => return downcast_into_value!(r, i64).into_script(world),
            tl : i128    => return downcast_into_value!(r, i128).into_script(world),
            tm : f32     => return downcast_into_value!(r, f32).into_script(world),
            tn : f64     => return downcast_into_value!(r, f64).into_script(world),
            to : bool    => return downcast_into_value!(r, bool).into_script(world),
            tp : char    => return downcast_into_value!(r, char).into_script(world),
            tq : String  => return downcast_into_value!(r, String).clone().into_script(world),
            tcs: Cow<'static, str> => match r.try_downcast_ref::<Cow<'static, str>>() {
                Some(cow) => return Ok(ScriptValue::String(cow.clone())),
                None => return Err(InteropError::type_mismatch(
                    std::any::TypeId::of::<Cow<str>>(),
                    r.get_represented_type_info().map(|i| i.type_id()),
                )),
            },
            tr : PathBuf => return downcast_into_value!(r, PathBuf).clone().into_script(world),
            ts : OsString=> return downcast_into_value!(r, OsString).clone().into_script(world),
            tsv: ScriptValue=> return Ok(downcast_into_value!(r, ScriptValue).clone()),
            tn : ()      => return Ok(ScriptValue::Unit)
        }
    );

    // either return nil or ref into
    if let Ok(as_option) = r.as_option() {
        return if let Some(s) = as_option {
            // don't want correction to mess with this
            self_.push_path(ReferencePart::IntegerAccess(0, false));
            into_script_ref(self_, s, world)
        } else {
            Ok(ScriptValue::Unit)
        };
    }

    Ok(ScriptValue::Reference(self_))
}
