//! Generic parameters, which reflection can only see through the types they are instantiated with.

use std::{marker::PhantomData, ops::Deref};

use bevy_app::App;
use bevy_asset::Asset;
use bevy_ecs::resource::Resource;
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{Reflect, TypeInfo, TypeRegistry, Typed};

use crate::{
    ReflectReference, ScriptValue,
    docgen::{TypedThrough, typed_through::ThroughTypeInfo},
    error::InteropError,
    function::{
        arg_meta::ArgMeta, from::FromScript, into::IntoScript,
        type_dependencies::GetTypeDependencies,
    },
};

/// Stands in for the generic parameter of a generic type, so the type's definition can be reflected.
///
/// Reflecting `Time<A>` gives `context: A` wherever the definition says `context: T`, which instantiations
/// like `Time<Virtual>` can't tell apart from a field that happens to be of the argument's type.
/// It implements the traits generic parameters are commonly bounded by, so it can instantiate most generic types.
///
/// In the signature of a script function, it is a parameter of the function, see [`TypeOf`] and [`ValueOf`].
#[derive(Reflect, Asset, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct A;

/// A type passed to a generic script function, converted as `R`, i.e. the component type `get_component` takes:
///
/// ```rust,ignore
/// fn get_component(entity: V<Entity>, registration: TypeOf<V<ScriptComponentRegistration>, A>)
///     -> Result<Option<ValueOf<A>>, InteropError>
/// ```
///
/// `M` is the marker standing for the function's parameter ([`A`]), [`ValueOf`] values of the same marker are of that type.
/// It only changes how the function is documented.
pub struct TypeOf<R, M>(pub R, PhantomData<fn() -> M>);

impl<R, M> TypeOf<R, M> {
    /// Unwrap the value the type is passed as.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R, M> Deref for TypeOf<R, M> {
    type Target = R;

    fn deref(&self) -> &R {
        &self.0
    }
}

impl<R: FromScript, M> FromScript for TypeOf<R, M> {
    type This<'w> = TypeOf<R::This<'w>, M>;

    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        R::from_script(value, world).map(|inner| TypeOf(inner, PhantomData))
    }
}

impl<R: ArgMeta, M> ArgMeta for TypeOf<R, M> {
    fn default_value() -> Option<ScriptValue> {
        R::default_value()
    }
}

impl<R: GetTypeDependencies, M> GetTypeDependencies for TypeOf<R, M> {
    type Underlying = R::Underlying;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        R::register_type_dependencies(registry)
    }
}

impl<R: TypedThrough, M: Typed> TypedThrough for TypeOf<R, M> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypeOf {
            of: M::type_info(),
            erased: Box::new(R::through_type_info()),
        }
    }
}

/// A value of the type a [`TypeOf`] with the same marker was given, i.e. the component `get_component` returns.
///
/// It is a reference at runtime, and only changes how the function is documented.
pub struct ValueOf<M>(pub ReflectReference, PhantomData<fn() -> M>);

impl<M> ValueOf<M> {
    /// Wrap a reference to a value of the type.
    pub fn new(reference: ReflectReference) -> Self {
        ValueOf(reference, PhantomData)
    }

    /// Unwrap the reference.
    pub fn into_inner(self) -> ReflectReference {
        self.0
    }
}

impl<M> Deref for ValueOf<M> {
    type Target = ReflectReference;

    fn deref(&self) -> &ReflectReference {
        &self.0
    }
}

impl<M> FromScript for ValueOf<M> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard<'_>) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(ValueOf::new)
    }
}

impl<M> IntoScript for ValueOf<M> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl<M> ArgMeta for ValueOf<M> {}

impl<M> GetTypeDependencies for ValueOf<M> {
    type Underlying = ReflectReference;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        ReflectReference::register_type_dependencies(registry)
    }
}

impl<M: Typed> TypedThrough for ValueOf<M> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Val(M::type_info())
    }
}

/// A generic type declared for documentation, see [`GenericTypesAppExt::register_generic_type`].
#[derive(Clone, Debug)]
pub struct GenericTypeDeclaration {
    /// The generic type instantiated with markers in place of its parameters, i.e. `Handle<A>`
    pub type_info: &'static TypeInfo,
    /// The names of the parameters, in order
    pub params: Vec<&'static str>,
    /// The documentation of the generic type
    pub documentation: &'static str,
}

/// The generic types declared for documentation, which documentation generators describe in terms of their parameters.
#[derive(Resource, Default, Clone, Debug)]
pub struct AppGenericTypes(pub Vec<GenericTypeDeclaration>);

/// Declares generic types for documentation, see [`AppGenericTypes`].
pub trait GenericTypesAppExt {
    /// Declares a generic type, so its instantiations are documented as such instead of as unrelated types.
    ///
    /// `G` is the generic type instantiated with [`A`] in place of its parameter, i.e. `Time<A>`, and `params` names
    /// the parameters in order, i.e. `["T"]`.
    fn register_generic_type<G: Typed>(
        &mut self,
        params: &[&'static str],
        documentation: &'static str,
    ) -> &mut Self;
}

impl GenericTypesAppExt for App {
    fn register_generic_type<G: Typed>(
        &mut self,
        params: &[&'static str],
        documentation: &'static str,
    ) -> &mut Self {
        self.world_mut()
            .get_resource_or_init::<AppGenericTypes>()
            .0
            .push(GenericTypeDeclaration {
                type_info: G::type_info(),
                params: params.to_vec(),
                documentation,
            });
        self
    }
}
