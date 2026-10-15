//! [`AnimationGraph`](bevy_animation::graph::AnimationGraph): the tree of clips and blends that an animation player plays.

// `script_bindings` expects these in scope
use bevy_animation::{
    AnimationClip,
    graph::{AnimationGraph, AnimationNodeIndex},
};
use bevy_asset::Handle;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "animation_graph_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "animation",
    core
)]
impl AnimationGraph {
    /// A new graph that plays `handle` on its own at full weight.
    fn from_clip(handle: R<Handle<AnimationClip>>) -> V<AnimationGraph> {
        V(AnimationGraph::from_clip((*handle).clone()).0)
    }

    /// The index of the root node of the graph, which all of its animations are blended into.
    fn root_node(_self: R<AnimationGraph>) -> V<AnimationNodeIndex> {
        V(_self.root)
    }

    /// Adds the clip `handle` under the node `parent`, with `weight` relative to its siblings. Returns the new node.
    fn add_clip(
        mut _self: M<AnimationGraph>,
        handle: R<Handle<AnimationClip>>,
        weight: f32,
        parent: R<AnimationNodeIndex>,
    ) -> V<AnimationNodeIndex> {
        V(_self.add_clip((*handle).clone(), weight, *parent))
    }
}

/// Registers the [`AnimationGraph`] functions.
pub(crate) fn register(world: &mut World) {
    register_animation_graph_functions(world);
}
