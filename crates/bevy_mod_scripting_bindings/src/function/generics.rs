//! Generic parameters, which reflection can only see through the types they are instantiated with.

use bevy_app::App;
use bevy_asset::Asset;
use bevy_ecs::resource::Resource;
use bevy_reflect::{Reflect, TypeInfo, Typed};

/// Stands in for the generic parameter of a generic type, so the type's definition can be reflected.
///
/// Reflecting `Time<A>` gives `context: A` wherever the definition says `context: T`, which instantiations
/// like `Time<Virtual>` can't tell apart from a field that happens to be of the argument's type.
/// It implements the traits generic parameters are commonly bounded by, so it can instantiate most generic types.
#[derive(Reflect, Asset, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct A;

/// A generic type declared for documentation, see [`GenericTypesAppExt::register_generic_type`].
#[derive(Clone, Debug)]
pub struct GenericTypeDeclaration {
    /// The generic type instantiated with markers in place of its parameters, i.e. `Handle<A>`
    pub type_info: &'static TypeInfo,
    /// The namespace of the generic type, i.e. `asset` for `Handle`
    pub namespace: &'static str,
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
    /// `G` is the generic type instantiated with [`A`] in place of its parameter, i.e. `Time<A>`, `namespace` is the one
    /// of its instantiations, i.e. `time`, and `params` names the parameters in order, i.e. `["T"]`.
    /// Also registers [`A`], so reflection can describe the uses of the parameter which only expose type ids, i.e. `HashSet<A>`.
    fn register_generic_type<G: Typed>(
        &mut self,
        namespace: &'static str,
        params: &[&'static str],
        documentation: &'static str,
    ) -> &mut Self;
}

impl GenericTypesAppExt for App {
    fn register_generic_type<G: Typed>(
        &mut self,
        namespace: &'static str,
        params: &[&'static str],
        documentation: &'static str,
    ) -> &mut Self {
        self.register_type::<A>();
        self.world_mut()
            .get_resource_or_init::<AppGenericTypes>()
            .0
            .push(GenericTypeDeclaration {
                type_info: G::type_info(),
                namespace,
                params: params.to_vec(),
                documentation,
            });
        self
    }
}
