#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{M, R, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyAssetScriptingPlugin;
pub(crate) fn register_untyped_handle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_asset::UntypedHandle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_asset::UntypedHandle>| {
                let output: V<::bevy_asset::UntypedHandle> = {
                    {
                        let output: V<::bevy_asset::UntypedHandle> = <::bevy_asset::UntypedHandle as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_asset::UntypedHandle>,
                other: R<::bevy_asset::UntypedHandle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_asset::UntypedHandle as ::core::cmp::PartialEq<
                            ::bevy_asset::UntypedHandle,
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
            "id",
            |_self: R<::bevy_asset::UntypedHandle>| {
                let output: V<::bevy_asset::UntypedAssetId> = {
                    {
                        let output: V<::bevy_asset::UntypedAssetId> = ::bevy_asset::UntypedHandle::id(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`UntypedAssetId`] for the referenced asset.",
            &["_self"],
        )
        .register_documented(
            "type_id",
            |_self: R<::bevy_asset::UntypedHandle>| {
                let output: V<::core::any::TypeId> = {
                    {
                        let output: V<::core::any::TypeId> = ::bevy_asset::UntypedHandle::type_id(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`TypeId`] of the referenced [`Asset`].",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_asset::UntypedHandle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_untyped_asset_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_asset::UntypedAssetId,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_asset::UntypedAssetId>| {
            let output: V<::bevy_asset::UntypedAssetId> = {
                {
                    let output: V<::bevy_asset::UntypedAssetId> =
                        <::bevy_asset::UntypedAssetId as ::core::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_asset::UntypedAssetId>, other: R<::bevy_asset::UntypedAssetId>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_asset::UntypedAssetId as ::core::cmp::PartialEq<
                        ::bevy_asset::UntypedAssetId,
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
        "type_id",
        |_self: R<::bevy_asset::UntypedAssetId>| {
            let output: V<::core::any::TypeId> = {
                {
                    let output: V<::core::any::TypeId> =
                        ::bevy_asset::UntypedAssetId::type_id(&_self).into();
                    output
                }
            };
            output
        },
        " Returns the stored [`TypeId`] of the referenced [`Asset`].",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_asset::UntypedAssetId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_asset_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_asset::AssetIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: R<::bevy_asset::AssetIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_asset::AssetIndex as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: R<::bevy_asset::AssetIndex>| {
                let output: V<::bevy_asset::AssetIndex> = {
                    {
                        let output: V<::bevy_asset::AssetIndex> = <::bevy_asset::AssetIndex as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_asset::AssetIndex>, other: R<::bevy_asset::AssetIndex>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_asset::AssetIndex as ::core::cmp::PartialEq<
                            ::bevy_asset::AssetIndex,
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
            "from_bits",
            |bits: u64| {
                let output: V<::bevy_asset::AssetIndex> = {
                    {
                        let output: V<::bevy_asset::AssetIndex> = ::bevy_asset::AssetIndex::from_bits(
                                bits,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert an opaque `u64` acquired from [`AssetIndex::to_bits`] back into an [`AssetIndex`]. This should not be used with any inputs other than those\n derived from [`AssetIndex::to_bits`], as there are no guarantees for what will happen with such inputs.",
            &["bits"],
        )
        .register_documented(
            "to_bits",
            |_self: V<::bevy_asset::AssetIndex>| {
                let output: u64 = {
                    {
                        let output: u64 = ::bevy_asset::AssetIndex::to_bits(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert the [`AssetIndex`] into an opaque blob of bits to transport it in circumstances where carrying a strongly typed index isn't possible.\n The result of this function should not be relied upon for anything except putting it back into [`AssetIndex::from_bits`] to recover the index.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_asset::AssetIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_asset_usages_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_asset::RenderAssetUsages,
    >::new(world)
        .register_documented(
            "all",
            || {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::all()
                            .into();
                        output
                    }
                };
                output
            },
            " Get a flags value with all known bits set.",
            &[],
        )
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: R<::bevy_asset::RenderAssetUsages>| {
                let output: () = {
                    {
                        let output: () = <::bevy_asset::RenderAssetUsages as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            "bits",
            |_self: R<::bevy_asset::RenderAssetUsages>| {
                let output: u8 = {
                    {
                        let output: u8 = ::bevy_asset::RenderAssetUsages::bits(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the underlying bits value.\n The returned value is exactly the bits set in this flags value.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_asset::RenderAssetUsages>| {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = <::bevy_asset::RenderAssetUsages as ::core::clone::Clone>::clone(
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
            "complement",
            |_self: V<::bevy_asset::RenderAssetUsages>| {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::complement(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise negation (`!`) of the bits in a flags value, truncating the result.",
            &["_self"],
        )
        .register_documented(
            "contains",
            |
                _self: R<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_asset::RenderAssetUsages::contains(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Whether all set bits in a source flags value are also set in a target flags value.",
            &["_self", "other"],
        )
        .register_documented(
            "difference",
            |
                _self: V<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::difference(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The intersection of a source flags value with the complement of a target flags\n value (`&!`).\n This method is not equivalent to `self & !other` when `other` has unknown bits set.\n `difference` won't truncate `other`, but the `!` operator will.",
            &["_self", "other"],
        )
        .register_documented(
            "empty",
            || {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::empty()
                            .into();
                        output
                    }
                };
                output
            },
            " Get a flags value with all bits unset.",
            &[],
        )
        .register_documented(
            "eq",
            |
                _self: R<::bevy_asset::RenderAssetUsages>,
                other: R<::bevy_asset::RenderAssetUsages>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_asset::RenderAssetUsages as ::core::cmp::PartialEq<
                            ::bevy_asset::RenderAssetUsages,
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
            "from_bits_retain",
            |bits: u8| {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::from_bits_retain(
                                bits,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert from a bits value exactly.",
            &["bits"],
        )
        .register_documented(
            "from_bits_truncate",
            |bits: u8| {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::from_bits_truncate(
                                bits,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert from a bits value, unsetting any unknown bits.",
            &["bits"],
        )
        .register_documented(
            "insert",
            |
                mut _self: M<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_asset::RenderAssetUsages::insert(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise or (`|`) of the bits in two flags values.",
            &["_self", "other"],
        )
        .register_documented(
            "intersection",
            |
                _self: V<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::intersection(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise and (`&`) of the bits in two flags values.",
            &["_self", "other"],
        )
        .register_documented(
            "intersects",
            |
                _self: R<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_asset::RenderAssetUsages::intersects(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Whether any set bits in a source flags value are also set in a target flags value.",
            &["_self", "other"],
        )
        .register_documented(
            "is_all",
            |_self: R<::bevy_asset::RenderAssetUsages>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_asset::RenderAssetUsages::is_all(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Whether all known bits in this flags value are set.",
            &["_self"],
        )
        .register_documented(
            "is_empty",
            |_self: R<::bevy_asset::RenderAssetUsages>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_asset::RenderAssetUsages::is_empty(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Whether all bits in this flags value are unset.",
            &["_self"],
        )
        .register_documented(
            "remove",
            |
                mut _self: M<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_asset::RenderAssetUsages::remove(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The intersection of a source flags value with the complement of a target flags\n value (`&!`).\n This method is not equivalent to `self & !other` when `other` has unknown bits set.\n `remove` won't truncate `other`, but the `!` operator will.",
            &["_self", "other"],
        )
        .register_documented(
            "set",
            |
                mut _self: M<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>,
                value: bool|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_asset::RenderAssetUsages::set(
                                &_self,
                                &other,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Call `insert` when `value` is `true` or `remove` when `value` is `false`.",
            &["_self", "other", "value"],
        )
        .register_documented(
            "sub",
            |
                _self: V<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = <::bevy_asset::RenderAssetUsages as ::core::ops::Sub<
                            ::bevy_asset::RenderAssetUsages,
                        >>::sub(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            " The intersection of a source flags value with the complement of a target flags value (`&!`).\n This method is not equivalent to `self & !other` when `other` has unknown bits set.\n `difference` won't truncate `other`, but the `!` operator will.",
            &["_self", "other"],
        )
        .register_documented(
            "symmetric_difference",
            |
                _self: V<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::symmetric_difference(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise exclusive-or (`^`) of the bits in two flags values.",
            &["_self", "other"],
        )
        .register_documented(
            "toggle",
            |
                mut _self: M<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_asset::RenderAssetUsages::toggle(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise exclusive-or (`^`) of the bits in two flags values.",
            &["_self", "other"],
        )
        .register_documented(
            "union",
            |
                _self: V<::bevy_asset::RenderAssetUsages>,
                other: V<::bevy_asset::RenderAssetUsages>|
            {
                let output: V<::bevy_asset::RenderAssetUsages> = {
                    {
                        let output: V<::bevy_asset::RenderAssetUsages> = ::bevy_asset::RenderAssetUsages::union(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The bitwise or (`|`) of the bits in two flags values.",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_asset::RenderAssetUsages,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyAssetScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_untyped_handle_functions(&mut world);
        register_untyped_asset_id_functions(&mut world);
        register_asset_index_functions(&mut world);
        register_render_asset_usages_functions(&mut world);
    }
}
