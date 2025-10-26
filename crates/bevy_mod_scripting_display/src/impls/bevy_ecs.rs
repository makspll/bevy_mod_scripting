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
        let mut builder = f.debug_tuple_with_type_info("ComponentId", type_info_provider);
        match type_info_provider {
            Some(type_info_provider) => match type_info_provider
                .get_component_info(*self)
                .map(|info| info.name().to_string())
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
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match type_info_provider {
            Some(type_info_provider) => match type_info_provider
                .get_component_info(*self)
                .map(|info| info.name().to_string())
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
