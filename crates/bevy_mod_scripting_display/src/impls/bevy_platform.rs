use crate::*;

impl<K: DebugWithTypeInfo, V: DebugWithTypeInfo, S> DebugWithTypeInfo
    for bevy_platform::collections::HashMap<K, V, S>
{
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_map_with_type_info(type_info_provider)
            .entries(
                self.iter()
                    .map(|(k, v)| (k as &dyn DebugWithTypeInfo, v as &dyn DebugWithTypeInfo)),
            )
            .finish()
    }
}

impl<K: DebugWithTypeInfo, S> DebugWithTypeInfo for bevy_platform::collections::HashSet<K, S> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_set_with_type_info(type_info_provider)
            .entries(self.iter().map(|v| v as &dyn DebugWithTypeInfo))
            .finish()
    }
}

impl<K: DebugWithTypeInfo> DebugWithTypeInfo for bevy_platform::collections::HashTable<K> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_set_with_type_info(type_info_provider)
            .entries(self.iter().map(|v| v as &dyn DebugWithTypeInfo))
            .finish()
    }
}
