#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyAnimationScriptingPlugin;
pub(crate) fn register_animation_node_type_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::AnimationNodeType,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_animation::graph::AnimationNodeType>| {
            let output: Val<::bevy_animation::graph::AnimationNodeType> = {
                {
                    let output: Val<::bevy_animation::graph::AnimationNodeType> =
                        <::bevy_animation::graph::AnimationNodeType as ::std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::AnimationNodeType,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_graph_handle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::AnimationGraphHandle,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_animation::graph::AnimationGraphHandle>| {
                let output: () = {
                    {
                        let output: () = <::bevy_animation::graph::AnimationGraphHandle as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::graph::AnimationGraphHandle>| {
                let output: Val<::bevy_animation::graph::AnimationGraphHandle> = {
                    {
                        let output: Val<::bevy_animation::graph::AnimationGraphHandle> = <::bevy_animation::graph::AnimationGraphHandle as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_animation::graph::AnimationGraphHandle>,
                other: Ref<::bevy_animation::graph::AnimationGraphHandle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_animation::graph::AnimationGraphHandle as ::std::cmp::PartialEq<
                            ::bevy_animation::graph::AnimationGraphHandle,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::AnimationGraphHandle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_threaded_animation_graphs_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::ThreadedAnimationGraphs,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::ThreadedAnimationGraphs,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_clip_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::prelude::AnimationClip,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_animation::prelude::AnimationClip>| {
            let output: Val<::bevy_animation::prelude::AnimationClip> = {
                {
                    let output: Val<::bevy_animation::prelude::AnimationClip> =
                        <::bevy_animation::prelude::AnimationClip as ::std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    )
    .register_documented(
        "duration",
        |_self: Ref<::bevy_animation::prelude::AnimationClip>| {
            let output: f32 = {
                {
                    let output: f32 =
                        ::bevy_animation::prelude::AnimationClip::duration(&_self).into();
                    output
                }
            };
            output
        },
        " Duration of the clip, represented in seconds.",
        &["_self"],
    )
    .register_documented(
        "set_duration",
        |mut _self: Mut<::bevy_animation::prelude::AnimationClip>, duration_sec: f32| {
            let output: () = {
                {
                    let output: () = ::bevy_animation::prelude::AnimationClip::set_duration(
                        &mut _self,
                        duration_sec,
                    )
                    .into();
                    output
                }
            };
            output
        },
        " Set the duration of the clip in seconds.",
        &["_self", "duration_sec"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::prelude::AnimationClip,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_player_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::prelude::AnimationPlayer,
    >::new(world)
        .register_documented(
            "all_finished",
            |_self: Ref<::bevy_animation::prelude::AnimationPlayer>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_animation::prelude::AnimationPlayer::all_finished(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if all playing animations have finished, according to the repetition behavior.",
            &["_self"],
        )
        .register_documented(
            "all_paused",
            |_self: Ref<::bevy_animation::prelude::AnimationPlayer>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_animation::prelude::AnimationPlayer::all_paused(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if all playing animations are paused.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::prelude::AnimationPlayer>| {
                let output: Val<::bevy_animation::prelude::AnimationPlayer> = {
                    {
                        let output: Val<::bevy_animation::prelude::AnimationPlayer> = <::bevy_animation::prelude::AnimationPlayer as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone_from",
            |
                mut _self: Mut<::bevy_animation::prelude::AnimationPlayer>,
                source: Ref<::bevy_animation::prelude::AnimationPlayer>|
            {
                let output: () = {
                    {
                        let output: () = <::bevy_animation::prelude::AnimationPlayer as ::std::clone::Clone>::clone_from(
                                &mut _self,
                                &source,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "source"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::prelude::AnimationPlayer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_graph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::AnimationGraph,
    >::new(world)
        .register_documented(
            "add_target_to_mask_group",
            |
                mut _self: Mut<::bevy_animation::graph::AnimationGraph>,
                target: Val<::bevy_animation::AnimationTargetId>,
                mask_group: u32|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_animation::graph::AnimationGraph::add_target_to_mask_group(
                                &mut _self,
                                target.into_inner(),
                                mask_group,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Adds an animation target (bone) to the mask group with the given ID.\n Calling this method multiple times with the same animation target but\n different mask groups will result in that target being added to all of\n the specified groups.",
            &["_self", "target", "mask_group"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::graph::AnimationGraph>| {
                let output: Val<::bevy_animation::graph::AnimationGraph> = {
                    {
                        let output: Val<::bevy_animation::graph::AnimationGraph> = <::bevy_animation::graph::AnimationGraph as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            || {
                let output: Val<::bevy_animation::graph::AnimationGraph> = {
                    {
                        let output: Val<::bevy_animation::graph::AnimationGraph> = ::bevy_animation::graph::AnimationGraph::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new animation graph with a root node and no other nodes.",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::AnimationGraph,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_transitions_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::transition::AnimationTransitions,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::transition::AnimationTransitions>| {
                let output: Val<::bevy_animation::transition::AnimationTransitions> = {
                    {
                        let output: Val<
                            ::bevy_animation::transition::AnimationTransitions,
                        > = <::bevy_animation::transition::AnimationTransitions as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone_from",
            |
                mut _self: Mut<::bevy_animation::transition::AnimationTransitions>,
                source: Ref<::bevy_animation::transition::AnimationTransitions>|
            {
                let output: () = {
                    {
                        let output: () = <::bevy_animation::transition::AnimationTransitions as ::std::clone::Clone>::clone_from(
                                &mut _self,
                                &source,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "source"],
        )
        .register_documented(
            "new",
            || {
                let output: Val<::bevy_animation::transition::AnimationTransitions> = {
                    {
                        let output: Val<
                            ::bevy_animation::transition::AnimationTransitions,
                        > = ::bevy_animation::transition::AnimationTransitions::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`AnimationTransitions`] component, ready to be added to\n an entity with an [`AnimationPlayer`].",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::transition::AnimationTransitions,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_target_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::AnimationTargetId,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_animation::AnimationTargetId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_animation::AnimationTargetId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::AnimationTargetId>| {
                let output: Val<::bevy_animation::AnimationTargetId> = {
                    {
                        let output: Val<::bevy_animation::AnimationTargetId> = <::bevy_animation::AnimationTargetId as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_animation::AnimationTargetId>,
                other: Ref<::bevy_animation::AnimationTargetId>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_animation::AnimationTargetId as ::std::cmp::PartialEq<
                            ::bevy_animation::AnimationTargetId,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "from_name",
            |name: Ref<::bevy_ecs::name::Name>| {
                let output: Val<::bevy_animation::AnimationTargetId> = {
                    {
                        let output: Val<::bevy_animation::AnimationTargetId> = ::bevy_animation::AnimationTargetId::from_name(
                                &name,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`AnimationTargetId`] by hashing a single name.",
            &["name"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::AnimationTargetId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::AnimationTarget,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_animation::AnimationTarget>| {
            let output: Val<::bevy_animation::AnimationTarget> = {
                {
                    let output: Val<::bevy_animation::AnimationTarget> =
                        <::bevy_animation::AnimationTarget as ::std::clone::Clone>::clone(&_self)
                            .into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::AnimationTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_repeat_animation_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::RepeatAnimation,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_animation::RepeatAnimation>| {
                let output: () = {
                    {
                        let output: () = <::bevy_animation::RepeatAnimation as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::RepeatAnimation>| {
                let output: Val<::bevy_animation::RepeatAnimation> = {
                    {
                        let output: Val<::bevy_animation::RepeatAnimation> = <::bevy_animation::RepeatAnimation as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_animation::RepeatAnimation>,
                other: Ref<::bevy_animation::RepeatAnimation>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_animation::RepeatAnimation as ::std::cmp::PartialEq<
                            ::bevy_animation::RepeatAnimation,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::RepeatAnimation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_active_animation_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::ActiveAnimation,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: Val<::bevy_animation::ActiveAnimation> = {
                    {
                        let output: Val<::bevy_animation::ActiveAnimation> = <::bevy_animation::ActiveAnimation as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "completions",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_animation::ActiveAnimation::completions(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the number of times this animation has completed.",
            &["_self"],
        )
        .register_documented(
            "elapsed",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_animation::ActiveAnimation::elapsed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the amount of time the animation has been playing.",
            &["_self"],
        )
        .register_documented(
            "is_finished",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_animation::ActiveAnimation::is_finished(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the animation has finished, based on its repetition behavior and the number of times it has repeated.\n Note: An animation with `RepeatAnimation::Forever` will never finish.",
            &["_self"],
        )
        .register_documented(
            "is_paused",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_animation::ActiveAnimation::is_paused(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if this animation is currently paused.\n Note that paused animations are still [`ActiveAnimation`]s.",
            &["_self"],
        )
        .register_documented(
            "is_playback_reversed",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_animation::ActiveAnimation::is_playback_reversed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the animation is playing in reverse.",
            &["_self"],
        )
        .register_documented(
            "repeat_mode",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: Val<::bevy_animation::RepeatAnimation> = {
                    {
                        let output: Val<::bevy_animation::RepeatAnimation> = ::bevy_animation::ActiveAnimation::repeat_mode(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the repeat mode assigned to this active animation.",
            &["_self"],
        )
        .register_documented(
            "replay",
            |mut _self: Mut<::bevy_animation::ActiveAnimation>| {
                let output: () = {
                    {
                        let output: () = ::bevy_animation::ActiveAnimation::replay(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reset back to the initial state as if no time has elapsed.",
            &["_self"],
        )
        .register_documented(
            "seek_time",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_animation::ActiveAnimation::seek_time(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the seek time of the animation.\n This is nonnegative and no more than the clip duration.",
            &["_self"],
        )
        .register_documented(
            "speed",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_animation::ActiveAnimation::speed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the speed of the animation playback.",
            &["_self"],
        )
        .register_documented(
            "weight",
            |_self: Ref<::bevy_animation::ActiveAnimation>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_animation::ActiveAnimation::weight(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the current weight of this animation.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::ActiveAnimation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_weights_curve_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::gltf_curves::WeightsCurve,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::gltf_curves::WeightsCurve>| {
                let output: Val<::bevy_animation::gltf_curves::WeightsCurve> = {
                    {
                        let output: Val<::bevy_animation::gltf_curves::WeightsCurve> = <::bevy_animation::gltf_curves::WeightsCurve as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::gltf_curves::WeightsCurve,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubic_rotation_curve_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::gltf_curves::CubicRotationCurve,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::gltf_curves::CubicRotationCurve>| {
                let output: Val<::bevy_animation::gltf_curves::CubicRotationCurve> = {
                    {
                        let output: Val<
                            ::bevy_animation::gltf_curves::CubicRotationCurve,
                        > = <::bevy_animation::gltf_curves::CubicRotationCurve as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::gltf_curves::CubicRotationCurve,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_graph_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::AnimationGraphNode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::graph::AnimationGraphNode>| {
                let output: Val<::bevy_animation::graph::AnimationGraphNode> = {
                    {
                        let output: Val<::bevy_animation::graph::AnimationGraphNode> = <::bevy_animation::graph::AnimationGraphNode as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::AnimationGraphNode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_threaded_animation_graph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::graph::ThreadedAnimationGraph,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::graph::ThreadedAnimationGraph,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_animation_transition_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_animation::transition::AnimationTransition,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_animation::transition::AnimationTransition>| {
                let output: Val<::bevy_animation::transition::AnimationTransition> = {
                    {
                        let output: Val<
                            ::bevy_animation::transition::AnimationTransition,
                        > = <::bevy_animation::transition::AnimationTransition as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_animation::transition::AnimationTransition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyAnimationScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_animation_node_type_functions(&mut world);
        register_animation_graph_handle_functions(&mut world);
        register_threaded_animation_graphs_functions(&mut world);
        register_animation_clip_functions(&mut world);
        register_animation_player_functions(&mut world);
        register_animation_graph_functions(&mut world);
        register_animation_transitions_functions(&mut world);
        register_animation_target_id_functions(&mut world);
        register_animation_target_functions(&mut world);
        register_repeat_animation_functions(&mut world);
        register_active_animation_functions(&mut world);
        register_weights_curve_functions(&mut world);
        register_cubic_rotation_curve_functions(&mut world);
        register_animation_graph_node_functions(&mut world);
        register_threaded_animation_graph_functions(&mut world);
        register_animation_transition_functions(&mut world);
    }
}
