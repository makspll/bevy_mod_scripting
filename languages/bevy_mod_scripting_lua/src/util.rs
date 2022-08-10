/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide ".tl" paths
/// ```rust
/// use bevy_mod_scripting::lua_path;
/// assert_eq!("scripts/my_script.tl",lua_path!("my_script"))
/// ```
#[cfg(all(feature = "teal", debug_assertions))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/", $v, ".tl")
    };
}

/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide ".lua" paths
/// ```rust
/// use bevy_mod_scripting::lua_path;
/// assert_eq!("scripts/build/my_script.lua",lua_path!("my_script"))
/// ```
#[cfg(all(feature = "teal", not(debug_assertions)))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/build/", $v, ".lua")
    };
}


