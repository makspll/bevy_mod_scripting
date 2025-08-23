//! Contains various `Reflect` type data we use in BMS.

use bevy_reflect::FromType;

/// A marker type to indicate that a type is generated.
///
/// To mark a type as generated use:
/// ```rust,ignore
/// registry.register_type_data::<T, MarkAsGenerated>();
/// ```
#[derive(Clone, Copy)]
pub struct MarkAsGenerated;

impl<T> FromType<T> for MarkAsGenerated {
    fn from_type() -> Self {
        Self
    }
}

/// A marker type to indicate that a type is significant.
///
/// Significant types appear "before" core types in documentation
#[derive(Clone, Copy)]
pub struct MarkAsSignificant;

impl<T> FromType<T> for MarkAsSignificant {
    fn from_type() -> Self {
        Self
    }
}

/// A marker type to indicate that a type is core to BMS.
///
/// core types appear before insignificant types in documentation.
#[derive(Clone, Copy)]
pub struct MarkAsCore;

impl<T> FromType<T> for MarkAsCore {
    fn from_type() -> Self {
        Self
    }
}
