use bevy_reflect::{ParsedPath, PartialReflect};

use crate::*;

impl_debug_with_type_info_via_debug!(ParsedPath);
impl_debug_with_type_info_via_debug!(TypeInfo);
impl_debug_with_type_info_via_debug!(&'static TypeInfo);

impl_display_with_type_info_via_display!(ParsedPath);

impl DebugWithTypeInfo for Box<dyn PartialReflect> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        ReflectPrinter::new(f, type_info_provider).debug(self.as_ref())
    }
}
