impl crate::DebugWithTypeInfo for bevy_asset::UntypedHandle {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&dyn crate::GetTypeInfo>,
    ) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
