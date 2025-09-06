use bevy_reflect::ParsedPath;

use crate::*;

impl_debug_with_type_info_via_debug!(ParsedPath);
impl_debug_with_type_info_via_debug!(TypeInfo);
impl_debug_with_type_info_via_debug!(&'static TypeInfo);

impl_display_with_type_info_via_display!(ParsedPath);
