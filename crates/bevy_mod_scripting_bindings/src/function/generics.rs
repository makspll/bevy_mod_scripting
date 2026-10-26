//! Generic parameters, which reflection can only see through the types they are instantiated with.

use bevy_asset::Asset;
use bevy_reflect::Reflect;

/// Stands in for the generic parameter of a generic type, so the type's definition can be reflected.
///
/// Reflecting `Time<A>` gives `context: A` wherever the definition says `context: T`, which instantiations
/// like `Time<Virtual>` can't tell apart from a field that happens to be of the argument's type.
/// It implements the traits generic parameters are commonly bounded by, so it can instantiate most generic types.
#[derive(Reflect, Asset, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct A;
