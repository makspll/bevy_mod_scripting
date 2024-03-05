use std::{
    any::type_name,
    fmt::{Debug, Display},
    iter::Map,
};

use bevy::reflect::{FromReflect, Reflect, TypePath};
#[allow(deprecated)]
use bevy_mod_scripting_rhai::rhai::{CustomType, Dynamic, Engine, EvalAltResult, Position};

use crate::{
    common::std::ScriptVec, error::ReflectionError, ReflectPathElem, ScriptRef, ValueIndex,
};

use super::{ApplyRhai, FromRhaiProxy, RhaiProxyable, ToDynamic, ToRhaiProxy};

impl<T: Clone + RhaiCopy + Reflect> RhaiProxyable for T {
    fn ref_to_rhai(self_: crate::ScriptRef) -> Result<Dynamic, Box<EvalAltResult>> {
        self_.get_typed(|self_: &T| Ok(Dynamic::from(self_.clone())))?
    }

    fn apply_rhai(
        self_: &mut crate::ScriptRef,
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
            ) -> Result<Dynamic, Box<EvalAltResult>> {
                self_.get_typed(|$self_to_rhai: &$type| Ok($($proxy_expr_to_rhai)*))?
            }

            fn apply_rhai(
                self_: &mut crate::ScriptRef,
                new_val: Dynamic,
            ) -> Result<(), Box<EvalAltResult>> {
                self_.set_val(Self::from_rhai_proxy(new_val)?)?;
                Ok(())
            }
        }

        impl FromRhaiProxy for $type {
            #[inline(always)]
            fn from_rhai_proxy(self_: Dynamic) -> Result<Self, Box<EvalAltResult>> {
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
            fn to_rhai_proxy($self) -> Result<Dynamic, Box<EvalAltResult>> {
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

impl<T: RhaiProxyable + Reflect + FromReflect + TypePath + Clone + FromRhaiProxy> RhaiProxyable
    for Option<T>
{
    fn ref_to_rhai(self_: crate::ScriptRef) -> Result<Dynamic, Box<EvalAltResult>> {
        self_.get_typed(|s: &Option<T>| match s {
            Some(_) => T::ref_to_rhai(self_.sub_ref(ReflectPathElem::SubReflection {
                label: "as_ref",
                get: |ref_| {
                    ref_.downcast_ref::<Option<T>>()
                        .ok_or_else(|| ReflectionError::CannotDowncast {
                            from: ref_.get_represented_type_info().unwrap().type_path().into(),
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
                        // i tried having from: ref_.get_represented_type_info().unwrap().type_path().into() instead of "Reflect"
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
            })),
            None => Ok(Dynamic::UNIT),
        })?
    }

    fn apply_rhai(
        self_: &mut crate::ScriptRef,
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
                    *s = Some(T::from_rhai_proxy(new_val)?);
                    Ok::<_, Box<EvalAltResult>>(())
                })?;
            }

            T::apply_rhai(
                &mut self_.sub_ref(ReflectPathElem::SubReflection {
                    label: "",
                    get: |ref_| {
                        ref_.downcast_ref::<Option<T>>()
                            .ok_or_else(|| ReflectionError::CannotDowncast {
                                from: ref_.get_represented_type_info().unwrap().type_path().into(),
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
                                from: ref_.get_represented_type_info().unwrap().type_path().into(),
                                to: stringify!(Option<T>).into(),
                            })
                        }
                    },
                }),
                new_val,
            )
        }
    }
}

impl<T: FromRhaiProxy> FromRhaiProxy for Option<T> {
    fn from_rhai_proxy(self_: Dynamic) -> Result<Self, Box<EvalAltResult>> {
        if self_.is::<()>() {
            Ok(None)
        } else {
            T::from_rhai_proxy(self_).map(Option::Some)
        }
    }
}

impl<T: ToRhaiProxy> ToRhaiProxy for Option<T> {
    fn to_rhai_proxy(self) -> Result<Dynamic, Box<EvalAltResult>> {
        match self {
            Some(v) => v.to_rhai_proxy(),
            None => Ok(Dynamic::UNIT),
        }
    }
}

/// Composite trait composing the various traits required for a type `T` to be used as part of a RhaiVec<T>
pub trait RhaiVecElem: FromReflect + TypePath + RhaiProxyable + FromRhaiProxy + Clone {}
impl<T: FromReflect + TypePath + RhaiProxyable + FromRhaiProxy + Clone> RhaiVecElem for T {}

/// A ScriptVec wrapper which implements a custom iterator ontop of ScriptVec's
pub struct RhaiVec<T: RhaiVecElem>(pub ScriptVec<T>);

impl<T: RhaiVecElem> Clone for RhaiVec<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: RhaiVecElem> Debug for RhaiVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: Display + RhaiVecElem> Display for RhaiVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: RhaiVecElem> RhaiVec<T> {
    pub fn new_ref(self_: crate::ScriptRef) -> Self {
        Self(ScriptVec::<T>::new_ref(self_))
    }
}

impl<T: RhaiVecElem> std::ops::Deref for RhaiVec<T> {
    type Target = ScriptVec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: RhaiVecElem> std::ops::DerefMut for RhaiVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: RhaiVecElem> IntoIterator for RhaiVec<T> {
    type Item = Result<Dynamic, Box<EvalAltResult>>;

    type IntoIter = Map<<ScriptVec<T> as IntoIterator>::IntoIter, fn(ScriptRef) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|v| v.to_dynamic())
    }
}

impl<T: RhaiVecElem> RhaiProxyable for Vec<T> {
    fn ref_to_rhai(self_: crate::ScriptRef) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(Dynamic::from(RhaiVec::<T>::new_ref(self_)))
    }

    fn apply_rhai(
        self_: &mut crate::ScriptRef,
        new_val: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        if new_val.is::<Vec<Dynamic>>() {
            let last_target_idx = self_.get_typed(|s: &Vec<T>| s.len())? - 1;
            // there is also another case to consider, Vec has a lua representation available as well (table)
            // if we receive one of those, we should also apply it
            for (idx, entry) in new_val.cast::<Vec<Dynamic>>().into_iter().enumerate() {
                if idx > last_target_idx {
                    // here we don't need to do anything special just use LuaProxyable impl
                    T::apply_rhai(&mut self_.index(idx), entry)?;
                } else {
                    // here we don't have anything to apply this to
                    // use FromLua impl
                    self_.get_mut_typed(|s: &mut Vec<T>| {
                        s[idx] = T::from_rhai_proxy(entry)?;
                        Ok::<_, Box<EvalAltResult>>(())
                    })??;
                }
            }
            Ok(())
        } else if new_val.is::<RhaiVec<T>>() {
            let vec = new_val.cast::<RhaiVec<T>>();
            self_.apply(&vec.ref_)?;
            Ok(())
        } else {
            Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                "Array or Vec".to_owned(),
                new_val.type_name().to_owned(),
                Position::NONE,
            )))
        }
    }
}

impl<T: RhaiVecElem> FromRhaiProxy for Vec<T> {
    fn from_rhai_proxy(self_: Dynamic) -> Result<Self, Box<EvalAltResult>> {
        if self_.is::<RhaiVec<T>>() {
            let vec = self_.cast::<RhaiVec<T>>();
            vec.ref_.get_typed(|s: &Vec<T>| Ok(s.clone()))?
        } else if self_.is::<Vec<Dynamic>>() {
            self_
                .cast::<Vec<Dynamic>>()
                .into_iter()
                .map(|v| T::from_rhai_proxy(v))
                .collect::<Result<Vec<_>, _>>()
        } else {
            Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                "Array or Vec".to_owned(),
                self_.type_name().to_owned(),
                Position::NONE,
            )))
        }
    }
}

impl<T: RhaiVecElem + ToRhaiProxy> ToRhaiProxy for Vec<T> {
    fn to_rhai_proxy(self) -> Result<Dynamic, Box<EvalAltResult>> {
        self.into_iter()
            .map(|v| T::to_rhai_proxy(v))
            .collect::<Result<Vec<_>, _>>()
            .map(Dynamic::from)
    }
}

#[allow(deprecated)]
impl<T: RhaiVecElem> CustomType for RhaiVec<T> {
    fn build(mut builder: bevy_mod_scripting_rhai::rhai::TypeBuilder<Self>) {
        builder
            .with_name(type_name::<Vec<T>>())
            .with_fn("to_debug", |vec: &mut RhaiVec<T>| format!("{:?}", vec))
            .with_fn("to_string", |vec: &mut RhaiVec<T>| {
                vec.ref_
                    .get(|s| format!("{:?}", &s))
                    .map_err::<Box<EvalAltResult>, _>(|e| e.into())
            })
            .with_result_fn("is_empty", |vec: &mut RhaiVec<T>| {
                vec.is_empty().map_err(Into::into)
            })
            .with_result_fn("len", |vec: &mut RhaiVec<T>| {
                vec.len().map(|v| v as INT).map_err(Into::into)
            })
            .with_result_fn("push", |vec: &mut RhaiVec<T>, val: Dynamic| {
                vec.push(T::from_rhai_proxy(val)?).map_err(Into::into)
            })
            .with_result_fn("pop", |vec: &mut RhaiVec<T>| vec.pop().map_err(Into::into))
            .with_result_fn("clear", |vec: &mut RhaiVec<T>| {
                vec.clear().map_err(Into::into)
            })
            .with_result_fn("insert", |vec: &mut RhaiVec<T>, idx: INT, val: Dynamic| {
                vec.insert(idx as usize, T::from_rhai_proxy(val)?)
                    .map_err(Into::into)
            })
            .with_result_fn("remove", |vec: &mut RhaiVec<T>, idx: INT| {
                vec.remove(idx as usize).map_err(Into::into)
            })
            .with_result_fn("index$get$", |vec: &mut RhaiVec<T>, idx: INT| {
                vec.index(idx as usize).to_dynamic()
            })
            .with_result_fn(
                "index$set$",
                |vec: &mut RhaiVec<T>, idx: INT, value: Dynamic| {
                    vec.index(idx as usize).apply_rhai(value)
                },
            );
    }
}

/// A trait for making monomorphization of Vec<T> implementations for any T easier.
///
/// Rhai does not support the idea of generic types, instead every function is a standalone thing, and hence
/// generics must be monomorphized manually (registered for every type you want to use them with).
pub trait RegisterVecType {
    fn register_vec_functions<T: RhaiVecElem>(&mut self) -> &mut Self;
}

impl RegisterVecType for Engine {
    fn register_vec_functions<T: RhaiVecElem>(&mut self) -> &mut Self {
        self.build_type::<RhaiVec<T>>();
        self.register_iterator_result::<RhaiVec<T>, _>();
        self
    }
}
