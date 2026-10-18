//! [`AnimationNodeIndex`](bevy_animation::graph::AnimationNodeIndex): identifies a node of an animation graph, as used to play animations.

// `script_bindings` expects these in scope
use bevy_animation::graph::AnimationNodeIndex;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "animation_node_index_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "animation",
    core
)]
impl AnimationNodeIndex {
    /// A node index from its number.
    ///
    /// Scripts see this type as `NodeIndex`.
    ///
    /// ```multi-lang-example,lua
    /// local NodeIndex = import_namespace("animation").NodeIndex
    /// local node = NodeIndex.new(3)
    /// assert(node:index() == 3)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let NodeIndex = import_namespace("animation").NodeIndex;
    /// let node = NodeIndex.new(3);
    /// assert!(node.index() == 3);
    /// ```
    fn new(index: u32) -> V<AnimationNodeIndex> {
        V(AnimationNodeIndex::new(index as usize))
    }

    /// The number of this node index.
    fn index(_self: R<AnimationNodeIndex>) -> u32 {
        _self.index() as u32
    }
}

/// Registers the [`AnimationNodeIndex`] functions.
pub(crate) fn register(world: &mut World) {
    register_animation_node_index_functions(world);
}
