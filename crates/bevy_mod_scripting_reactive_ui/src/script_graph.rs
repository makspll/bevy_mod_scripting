//! Support for running the reactive graph inside the script (`lua/runtime.lua`) instead of in Rust.
//!
//! The script graph binds properties to slots (`slot`), sends their values in one `commit` per flush,
//! and asks for a flush (`request_flush`) when its signals change. Rust calls its `on_ui_flush` callback
//! with the ids of the watched components which changed (`subscribe`).

use std::sync::Arc;

use bevy_ecs::world::World;
use bevy_mod_scripting_bindings::{
    FunctionCallContext, InteropError, NamespaceMembers, ScriptValue,
    function::{from::V, namespace::NamespaceBuilder},
};

use crate::{
    Signal, current_attachment,
    element::apply_prop,
    expr::{Expr, UiValue},
    runtime::runtime,
};

/// The callback the script graph defines, run with the ids of the subscribed watches which changed.
pub(crate) const FLUSH_CALLBACK: &str = "on_ui_flush";

/// Registers the functions the script graph uses in the `reactive_ui` namespace.
pub(crate) fn register(world: &mut World) {
    NamespaceBuilder::<NamespaceMembers>::in_namespace(world, "reactive_ui")
        .register_documented(
            "lua_runtime",
            || include_str!("lua/runtime.lua"),
            "The source of the Lua reactive graph, load it with `local rx = load(ui.lua_runtime())()`.",
            &[],
        )
        .register_documented(
            "slot",
            |id: u32| V(UiValue(Arc::new(Expr::Slot(id)))),
            "A property value which the script graph sets with `commit`, `id` is chosen by the script and unique within it.",
            &["id"],
        )
        .register_documented(
            "commit",
            |ctx: FunctionCallContext, writes: Vec<ScriptValue>| {
                let attachment = current_attachment(&ctx)?;
                let runtime = runtime(&ctx)?;
                let targets = {
                    let rt = runtime.lock();
                    writes
                        .chunks(2)
                        .filter_map(|pair| match pair {
                            [ScriptValue::Integer(slot), value] => rt
                                .slots
                                .get(&(attachment.clone(), *slot as u32))
                                .map(|target| (*target, value.clone())),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                };
                ctx.world()?.with_world_mut_access(|w| {
                    for ((entity, prop), value) in &targets {
                        if let Ok(mut e) = w.get_entity_mut(*entity) {
                            apply_prop(&mut e, *prop, value)?;
                        }
                    }
                    Ok::<_, InteropError>(())
                })?
            },
            "Sets the properties bound to slots, given as a flat list `{ slot, value, slot, value, ... }`.",
            &["writes"],
        )
        .register_documented(
            "request_flush",
            |ctx: FunctionCallContext| {
                let attachment = current_attachment(&ctx)?;
                runtime(&ctx)?
                    .lock()
                    .script_flushes
                    .entry(attachment)
                    .or_default();
                Ok::<_, InteropError>(())
            },
            "Asks for `on_ui_flush` to run in this script during the next flush.",
            &[],
        )
        .register_documented(
            "subscribe",
            |ctx: FunctionCallContext, watch: V<Signal>| {
                let attachment = current_attachment(&ctx)?;
                let id = watch.into_inner().0;
                runtime(&ctx)?.lock().subscribed.insert(id, attachment);
                Ok::<_, InteropError>(id as i64)
            },
            "Runs `on_ui_flush` with the returned id whenever the watched component changes.",
            &["watch"],
        );
}
