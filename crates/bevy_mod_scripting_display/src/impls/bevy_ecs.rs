use bevy_ecs::entity::Entity;

use crate::*;

impl_debug_with_type_info_via_debug!(Entity);
impl_display_with_type_info_via_display!(Entity);

impl DebugWithTypeInfo for ComponentId {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        let name = type_info_provider
            .and_then(|type_info_provider| {
                type_info_provider
                    .get_component_info(*self)
                    .map(|info| info.name().to_string())
            })
            .unwrap_or_else(|| format!("Unregistered ComponentId - {self:?}"));

        f.debug_tuple("ComponentId").field(&name).finish()
    }
}

impl DisplayWithTypeInfo for ComponentId {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        let name = type_info_provider
            .and_then(|type_info_provider| {
                type_info_provider
                    .get_component_info(*self)
                    .map(|info| info.name().to_string())
            })
            .unwrap_or_else(|| format!("Unregistered ComponentId - {self:?}"));

        f.write_str(&name)
    }
}
