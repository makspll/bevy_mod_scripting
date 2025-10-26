use parking_lot::RwLock;

use crate::*;

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for RwLock<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        if let Some(read) = self.try_read() {
            f.debug_tuple_with_type_info("RwLock", type_info_provider)
                .field(&*read as &dyn DebugWithTypeInfo)
                .finish()
        } else {
            f.debug_tuple_with_type_info("RwLock", type_info_provider)
                .field(&"<locked>")
                .finish()
        }
    }
}

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for parking_lot::Mutex<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        if let Some(guard) = self.try_lock() {
            f.debug_tuple_with_type_info("Mutex", type_info_provider)
                .field(&*guard as &dyn DebugWithTypeInfo)
                .finish()
        } else {
            f.debug_tuple_with_type_info("Mutex", type_info_provider)
                .field(&"<locked>")
                .finish()
        }
    }
}
