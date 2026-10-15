//! [`Val`] lengths: logical pixels, percentages of the parent, viewport units, or automatic.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::V;
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::Val;

#[script_bindings(
    remote,
    name = "val_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Val {
    /// A length in logical pixels, such as `Val.px(100)` for 100 pixels.
    ///
    /// ```multi-lang-example,lua
    /// local node = Node.new({ width = Val.px(100) })
    /// assert(node.width[1] == 100)
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let node = Node.new_.call(#{ width: Val.px.call(100.0) });
    /// assert(node.width[0] == 100.0);
    /// ```
    fn px(value: f32) -> V<Val> {
        V(Val::Px(value))
    }

    /// A length as a percentage of the parent's size along the axis it applies to.
    fn percent(value: f32) -> V<Val> {
        V(Val::Percent(value))
    }

    /// A length as a percentage of the viewport width.
    fn vw(value: f32) -> V<Val> {
        V(Val::Vw(value))
    }

    /// A length as a percentage of the viewport height.
    fn vh(value: f32) -> V<Val> {
        V(Val::Vh(value))
    }

    /// A length as a percentage of the viewport's smaller dimension.
    fn vmin(value: f32) -> V<Val> {
        V(Val::VMin(value))
    }

    /// A length as a percentage of the viewport's larger dimension.
    fn vmax(value: f32) -> V<Val> {
        V(Val::VMax(value))
    }

    /// A length that the layout works out from the node and its contents.
    fn auto() -> V<Val> {
        V(Val::Auto)
    }
}

/// Registers the [`Val`] functions.
pub(crate) fn register(world: &mut World) {
    register_val_functions(world);
}
