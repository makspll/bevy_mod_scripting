use ::std::borrow::Cow;

use ::bevy::{
    prelude::{App, AppTypeRegistry},
    reflect::{FromType, GetTypeRegistration, Reflect},
};
#[allow(deprecated)]
use bevy_mod_scripting_rhai::rhai::{CustomType, Dynamic, EvalAltResult, INT};

use crate::{ReflectedValue, ScriptRef, ValueIndex};

pub mod bevy;
pub mod std;

/// A trait allowing the registration of the [`RhaiProxyable`] trait with the type registry for foreign types
///
/// If you have access to the type you should prefer to use `#[reflect(RhaiProxyable)]` instead.
/// This is exactly equivalent.
pub trait RegisterForeignRhaiType {
    fn register_foreign_rhai_type<T: RhaiProxyable + Reflect + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self;
}

impl RegisterForeignRhaiType for App {
    fn register_foreign_rhai_type<T: RhaiProxyable + Reflect + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self {
        {
            let registry = self.world.resource_mut::<AppTypeRegistry>();
            let mut registry = registry.write();

            let rhai_data = <ReflectRhaiProxyable as FromType<T>>::from_type();

            if let Some(registration) = registry.get_mut(TypeId::of::<T>()) {
                registration.insert(rhai_data)
            } else {
                let mut registration = T::get_type_registration();
                registration.insert(rhai_data);
                registry.add_registration(registration);
            }
        }

        self
    }
}

pub trait RhaiProxyable {
    fn ref_to_rhai(self_: ScriptRef) -> Result<Dynamic, Box<EvalAltResult>>;
    fn apply_rhai(self_: &mut ScriptRef, new_val: Dynamic) -> Result<(), Box<EvalAltResult>>;
}

pub trait FromRhaiProxy: Sized {
    fn from_rhai_proxy(self_: Dynamic) -> Result<Self, Box<EvalAltResult>>;
}

pub trait ToRhaiProxy {
    fn to_rhai_proxy(self) -> Result<Dynamic, Box<EvalAltResult>>;
}

#[derive(Clone)]
pub struct ReflectRhaiProxyable {
    ref_to_rhai: fn(ref_: ScriptRef) -> Result<Dynamic, Box<EvalAltResult>>,
    apply_rhai: fn(ref_: &mut ScriptRef, new_val: Dynamic) -> Result<(), Box<EvalAltResult>>,
}

impl ReflectRhaiProxyable {
    pub fn ref_to_rhai(&self, ref_: ScriptRef) -> Result<Dynamic, Box<EvalAltResult>> {
        (self.ref_to_rhai)(ref_)
    }

    pub fn apply_rhai(
        &self,
        ref_: &mut ScriptRef,
        new_val: Dynamic,
    ) -> Result<(), Box<EvalAltResult>> {
        (self.apply_rhai)(ref_, new_val)
    }
}

impl<T: RhaiProxyable + Reflect> FromType<T> for ReflectRhaiProxyable {
    fn from_type() -> Self {
        Self {
            ref_to_rhai: T::ref_to_rhai,
            apply_rhai: T::apply_rhai,
        }
    }
}

pub trait ToDynamic {
    fn to_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>>;
}

impl ToDynamic for ReflectedValue {
    fn to_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(Dynamic::from(self))
    }
}

impl ToDynamic for ScriptRef {
    fn to_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>> {
        // clone since it's cheap and we don't want to clone self later
        let world = self.world_ptr.clone();
        let world = world.read();

        let type_data = world.resource::<AppTypeRegistry>();
        let g = type_data.read();

        let type_id = self.get(|s| s.type_id())?;

        if let Some(v) = g.get_type_data::<ReflectRhaiProxyable>(type_id) {
            v.ref_to_rhai(self)
        } else {
            ReflectedValue { ref_: self }.to_dynamic()
        }
    }
}

pub trait ApplyRhai {
    fn apply_rhai(&mut self, value: Dynamic) -> Result<(), Box<EvalAltResult>>;
}

impl ApplyRhai for ScriptRef {
    fn apply_rhai(&mut self, value: Dynamic) -> Result<(), Box<EvalAltResult>> {
        let world_ptr = self.world_ptr.clone();

        // remove typedata from the world to be able to manipulate world
        let proxyable = {
            let world = world_ptr.read();
            let type_registry = world.resource::<AppTypeRegistry>().read();
            type_registry
                .get_type_data::<ReflectRhaiProxyable>(self.get(|s| s.type_id())?)
                .cloned()
        };

        if let Some(ud) = proxyable {
            return ud.apply_rhai(self, value);
        } else if value.is::<ReflectedValue>() {
            let b = value.cast::<ReflectedValue>();
            self.apply(&b.into())?;
            return Ok(());
        }

        Err(Box::new(EvalAltResult::ErrorRuntime(self.get(|s|
            format!("Attempted to assign `{}` = {value:?}. Did you forget to call `app.register_foreign_rhai_type::<{}>`?",
                self.path,
                s.get_represented_type_info().unwrap().type_path()
            ))?.into(),Position::NONE)
        ))
    }
}

impl ValueIndex<Dynamic> for ScriptRef {
    type Output = Result<Self, Box<EvalAltResult>>;

    fn index(&self, index: Dynamic) -> Self::Output {
        if index.is::<INT>() {
            return Ok(self.index(index.as_int().unwrap() as usize));
        } else if index.is::<String>() {
            return Ok(self.index(Cow::Owned(index.into_string().unwrap())));
        };

        Err(Box::new(EvalAltResult::ErrorMismatchDataType(
            index.type_name().to_owned(),
            "integer or string".to_owned(),
            Position::NONE,
        )))
    }
}

use bevy_mod_scripting_rhai::rhai::plugin::*;

#[allow(deprecated)]
impl CustomType for ReflectedValue {
    fn build(mut builder: bevy_mod_scripting_rhai::rhai::TypeBuilder<Self>) {
        builder
            .with_indexer_get_result(|obj: &mut ReflectedValue, index: Dynamic| {
                obj.ref_.index(index)?.to_dynamic()
            })
            .with_indexer_set_result(|obj: &mut ReflectedValue, index: Dynamic, value: Dynamic| {
                obj.ref_.index(index)?.apply_rhai(value)
            })
            .with_fn("to_debug", |self_: &mut ReflectedValue| {
                format!("{:?}", self_)
            })
            .with_fn("to_string", |self_: &mut ReflectedValue| {
                self_
                    .ref_
                    .get(|s| format!("{:?}", &s))
                    .map_err::<Box<EvalAltResult>, _>(|e| e.into())
            });
    }
}
