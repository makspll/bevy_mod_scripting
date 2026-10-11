//! Contains various `Reflect` type data we use in BMS.

use bevy_reflect::CreateTypeData;

/// A marker type to indicate that a type is generated.
///
/// To mark a type as generated use:
/// ```rust,ignore
/// registry.register_type_data::<T, MarkAsGenerated>();
/// ```
#[derive(Clone, Copy)]
pub struct MarkAsGenerated;

impl<T> CreateTypeData<T> for MarkAsGenerated {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}

/// A marker type to indicate that a type is significant.
///
/// Significant types appear "before" core types in documentation
#[derive(Clone, Copy)]
pub struct MarkAsSignificant;

impl<T> CreateTypeData<T> for MarkAsSignificant {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}

/// A marker type to indicate that a type is core to BMS.
///
/// core types appear before insignificant types in documentation.
#[derive(Clone, Copy)]
pub struct MarkAsCore;

impl<T> CreateTypeData<T> for MarkAsCore {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}
