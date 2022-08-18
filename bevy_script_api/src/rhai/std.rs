use bevy::reflect::Reflect;
use bevy_mod_scripting_rhai::rhai::{Dynamic, EvalAltResult, NativeCallContext, Position};

use super::RhaiProxyable;

impl<T: Clone + RhaiCopy + Reflect> RhaiProxyable for T {
    fn ref_to_rhai(
        self_: crate::ScriptRef,
        _: NativeCallContext,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        self_.get_typed(|self_: &T| Ok(Dynamic::from(self_.clone())))?
    }

    fn apply_rhai(
        self_: &mut crate::ScriptRef,
        _: NativeCallContext,
        new_val: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        let other = if new_val.is::<T>() {
            new_val.cast::<T>()
        } else {
            return Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                stringify!(T).to_owned(),
                new_val.type_name().to_string(),
                Position::NONE,
            )));
        };

        self_.set_val(other)?;
        Ok(())
    }
}

/// A marker trait signifying this type is to receive an automatic proxy implementation via `Dynamic::from`.
/// This means the proxy for this type is the type itself, and is created by cloning the original reference.
pub trait RhaiCopy {}

/// Implements RhaiProxyabel for a numeric type via another proxy type by coercing the type
macro_rules! impl_rhai_proxy_coerced_proxy{
    [$($num_type:ty as $proxy_type:ty),*$(,)?] => {
        $(
            impl RhaiProxyable for $num_type {
                fn ref_to_rhai(
                    self_: crate::ScriptRef,
                    _: NativeCallContext,
                ) -> Result<Dynamic, Box<EvalAltResult>> {
                    self_.get_typed(|s: &$num_type| Ok(Dynamic::from(*s as $proxy_type)))?
                }

                fn apply_rhai(
                    self_: &mut crate::ScriptRef,
                    _: NativeCallContext,
                    new_val: Dynamic,
                ) -> Result<(), Box<EvalAltResult>> {
                    let other = if new_val.is::<$proxy_type>() {
                        new_val.cast::<$proxy_type>()
                    } else {
                        return Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                            stringify!(i64).to_owned(),
                            new_val.type_name().to_string(),
                            Position::NONE,
                        )));
                    };

                    self_.set_val(other as $num_type)?;
                    Ok(())
                }
            }
        )*
    }
}
use bevy_mod_scripting_rhai::rhai::{FLOAT, INT};

impl_rhai_proxy_coerced_proxy![
    i8 as INT,
    i16 as INT,
    i32 as INT,
    i64 as INT,
    i128 as INT,
    isize as INT,
    u8 as INT,
    u16 as INT,
    u32 as INT,
    u64 as INT,
    u128 as INT,
    usize as INT,
    f32 as FLOAT,
    f64 as FLOAT,
    bool as bool,
];
