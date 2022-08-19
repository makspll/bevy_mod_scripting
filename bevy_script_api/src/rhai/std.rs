use std::any::type_name;

use bevy::reflect::Reflect;
use bevy_mod_scripting_rhai::rhai::{Dynamic, EvalAltResult, NativeCallContext, Position};

use crate::{error::ReflectionError, ReflectPathElem};

use super::{FromRhaiProxy, RhaiProxyable, ToRhaiProxy};

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
macro_rules! impl_rhai_proxy {
    // i.e. impl_rhai_proxy!(String as Into)
    ($type:ty as Into) => {
        impl_rhai_proxy!($type,$type,self: {self.into()}, s: {s.into()});
    };
    // i.e. impl_rhai_proxy!(u32 as i64)
    ($type:ty as $proxy_type:ty) => {
        impl_rhai_proxy!($type, $proxy_type,self:{(self as $proxy_type).into()}, s:{(*s as $proxy_type).into()});
    };
    // i.e. impl_rhai_proxy!(ident, u32, i64, (*ident as i64).into()) expression is used in ref_to_rhai
    ($type:ty, $proxy_type:ty,$self:ident: {$($proxy_expr:tt)*}, $self_to_rhai:ident : {$($proxy_expr_to_rhai:tt)*} ) => {
        impl RhaiProxyable for $type {
            fn ref_to_rhai(
                self_: crate::ScriptRef,
                _: NativeCallContext,
            ) -> Result<Dynamic, Box<EvalAltResult>> {
                self_.get_typed(|$self_to_rhai: &$type| Ok($($proxy_expr_to_rhai)*))?
            }

            fn apply_rhai(
                self_: &mut crate::ScriptRef,
                ctx: NativeCallContext,
                new_val: Dynamic,
            ) -> Result<(), Box<EvalAltResult>> {
                self_.set_val(Self::from_rhai_proxy(new_val,ctx)?)?;
                Ok(())
            }
        }

        impl FromRhaiProxy for $type {
            #[inline(always)]
            fn from_rhai_proxy(self_: Dynamic, _: NativeCallContext) -> Result<Self, Box<EvalAltResult>> {
                if self_.is::<$proxy_type>(){
                    Ok(self_.cast::<$proxy_type>() as $type)
                } else {
                    Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                        stringify!($type).to_owned(),
                        self_.type_name().to_owned(),
                        Position::NONE,
                    )))
                }

            }
        }

        impl ToRhaiProxy for $type {
            #[inline(always)]
            fn to_rhai_proxy($self, _ : NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
                Ok($($proxy_expr)*)
            }
        }
    };
}
use bevy_mod_scripting_rhai::rhai::{FLOAT, INT};

impl_rhai_proxy!(i8 as INT);
impl_rhai_proxy!(i16 as INT);
impl_rhai_proxy!(i32 as INT);
impl_rhai_proxy!(i64 as INT);
impl_rhai_proxy!(i128 as INT);
impl_rhai_proxy!(isize as INT);
impl_rhai_proxy!(u8 as INT);
impl_rhai_proxy!(u16 as INT);
impl_rhai_proxy!(u32 as INT);
impl_rhai_proxy!(u64 as INT);
impl_rhai_proxy!(u128 as INT);
impl_rhai_proxy!(usize as INT);
impl_rhai_proxy!(f32 as FLOAT);
impl_rhai_proxy!(f64 as FLOAT);
impl_rhai_proxy!(bool as bool);
impl_rhai_proxy!(String as Into);

impl<T: RhaiProxyable + Reflect + Clone + FromRhaiProxy> RhaiProxyable for Option<T> {
    fn ref_to_rhai(
        self_: crate::ScriptRef,
        ctx: NativeCallContext,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        self_.get_typed(|s: &Option<T>| match s {
            Some(_) => T::ref_to_rhai(
                self_.sub_ref(ReflectPathElem::SubReflection {
                    label: "as_ref",
                    get: |ref_| {
                        ref_.downcast_ref::<Option<T>>()
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_ref()
                            .map(|t| t as &dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                    get_mut: |ref_| {
                        ref_.downcast_mut::<Option<T>>()
                            // TODO: there is some weird borrow checker fuckery going on here
                            // i tried having from: ref_.type_name().to_owned().into() instead of "Reflect"
                            // and lying this out in an if let expression, but nothing will satisfy the borrow checker here, so leaving this for now
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: "Reflect".into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_mut()
                            .map(|t| t as &mut dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                }),
                ctx,
            ),
            None => Ok(Dynamic::UNIT),
        })?
    }

    fn apply_rhai(
        self_: &mut crate::ScriptRef,
        ctx: NativeCallContext,
        new_val: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        if new_val.is::<()>() {
            self_.get_mut_typed(|s: &mut Option<T>| {
                *s = None;
                Ok(())
            })?
        } else {
            // we need to do this in two passes, first
            // ensure that the target type is the 'some' variant to allow a sub reference

            let is_none = self_.get_typed(|s: &Option<T>| s.is_none())?;

            if is_none {
                return self_.get_mut_typed(|s: &mut Option<T>| {
                    *s = Some(T::from_rhai_proxy(new_val, ctx)?);
                    Ok::<_, Box<EvalAltResult>>(())
                })?;
            }

            T::apply_rhai(
                &mut self_.sub_ref(ReflectPathElem::SubReflection {
                    label: "",
                    get: |ref_| {
                        ref_.downcast_ref::<Option<T>>()
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })?
                            .as_ref()
                            .map(|t| t as &dyn Reflect)
                            .ok_or_else(|| {
                                ReflectionError::Other(
                                    "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                )
                            })
                    },
                    get_mut: |ref_| {
                        if ref_.is::<Option<T>>() {
                            ref_.downcast_mut::<Option<T>>()
                                .unwrap()
                                .as_mut()
                                .map(|t| t as &mut dyn Reflect)
                                .ok_or_else(|| {
                                    ReflectionError::Other(
                                        "Stale reference to Option. Cannot sub reflect.".to_owned(),
                                    )
                                })
                        } else {
                            Err(ReflectionError::CannotDowncast {
                                from: ref_.type_name().to_owned().into(),
                                to: stringify!(Option<T>).into(),
                            })
                        }
                    },
                }),
                ctx,
                new_val,
            )
        }
    }
}

impl<T: FromRhaiProxy> FromRhaiProxy for Option<T> {
    fn from_rhai_proxy(self_: Dynamic, ctx: NativeCallContext) -> Result<Self, Box<EvalAltResult>> {
        if self_.is::<()>() {
            Ok(None)
        } else {
            T::from_rhai_proxy(self_, ctx).map(Option::Some)
        }
    }
}

impl<T: ToRhaiProxy> ToRhaiProxy for Option<T> {
    fn to_rhai_proxy(self, ctx: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
        match self {
            Some(v) => v.to_rhai_proxy(ctx),
            None => Ok(Dynamic::UNIT),
        }
    }
}
