use bevy_ecs::{component::ComponentId, entity::Entity};
use bevy_utils::prelude::DebugName;

use crate::*;

impl_debug_with_type_info_via_debug!(Entity);
impl_display_with_type_info_via_display!(Entity);

impl DebugWithTypeInfo for ComponentId {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        let mut builder = f.debug_tuple_with_type_info("ComponentId", type_info_provider);
        match type_info_provider {
            Some(type_info_provider) => match type_info_provider
                .as_unsafe_world_cell()
                .ok()
                .and_then(|i| i.components().get_name(*self))
            {
                Some(type_info) => builder.field(&type_info),
                None => builder.field(&format!("Unregistered ComponentId - {self:?}")),
            },
            None => builder.field(&format!("Unregistered ComponentId - {self:?}")),
        };
        builder.finish()
    }
}

impl DisplayWithTypeInfo for ComponentId {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        match type_info_provider {
            Some(type_info_provider) => match type_info_provider
                .as_unsafe_world_cell()
                .ok()
                .and_then(|i| i.components().get_name(*self))
            {
                Some(type_info) => f.write_str(&type_info),
                None => {
                    f.write_str("Unregistered ComponentId - ")?;
                    std::fmt::Debug::fmt(self, f)
                }
            },
            None => {
                f.write_str("component: ")?;
                std::fmt::Debug::fmt(self, f)
            }
        }
    }
}

impl DebugWithTypeInfo for DebugName {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
