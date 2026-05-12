use bevy_ecs::world::WorldId;
use bevy_log::info;
use bevy_mod_scripting_bindings::{InteropError, ReflectReference, ScriptValue};
use bevy_mod_scripting_core::{config::GetPluginThreadConfig, event::CallbackLabel};
use bevy_mod_scripting_script::ScriptAttachment;
use wasmtime::{AsContext, AsContextMut, StoreContext, StoreContextMut, component::*};

use crate::{WasmtimeContext, WasmtimeScriptingPlugin, WasmtimeStoreData, to_interop_error};

/// Convert a `ScriptValue` to a wasmtime component `Val`.  
pub fn script_value_to_val(sv: ScriptValue, data: &mut StoreContextMut<WasmtimeStoreData>) -> Val {
    match sv {
        ScriptValue::Unit => Val::Tuple(vec![]),
        ScriptValue::Bool(b) => Val::Bool(b),
        ScriptValue::Integer(i) => Val::S64(i),
        ScriptValue::Float(f) => Val::Float64(f),
        ScriptValue::String(s) => Val::String(s.into_owned().into()),
        ScriptValue::List(list) => Val::List(
            list.into_iter()
                .map(|v| script_value_to_val(v, data))
                .collect(),
        ),
        ScriptValue::Reference(r) => {
            let res = data.data_mut().push_ref(r);
            let as_any = res.try_into_resource_any(data).unwrap();
            Val::Resource(as_any)
        }
        ScriptValue::Error(e) => {
            Val::Result(Err(Some(Box::new(Val::String(format!("{e:?}").into())))))
        }
        ScriptValue::Function(_) | ScriptValue::FunctionMut(_) => Val::Tuple(vec![]),
        ScriptValue::Map(map) => Val::List(
            map.into_iter()
                .map(|(k, v)| Val::Tuple(vec![Val::String(k.into()), script_value_to_val(v, data)]))
                .collect(),
        ),
        _ => todo!(),
    }
}

/// Convert a wasmtime component `Val` to a `ScriptValue`.  
pub fn val_to_script_value(
    val: &Val,
    data: &mut StoreContextMut<WasmtimeStoreData>,
) -> ScriptValue {
    match val {
        Val::Bool(b) => ScriptValue::Bool(*b),
        Val::S64(i) => ScriptValue::Integer(*i),
        Val::U32(i) => ScriptValue::Integer(*i as i64),
        Val::Float64(f) => ScriptValue::Float(*f),
        Val::String(s) => ScriptValue::String(s.to_string().into()),
        Val::List(list) => {
            ScriptValue::List(list.iter().map(|v| val_to_script_value(v, data)).collect())
        }
        Val::Tuple(items) if items.is_empty() => ScriptValue::Unit,
        Val::Tuple(items) => {
            ScriptValue::List(items.iter().map(|v| val_to_script_value(v, data)).collect())
        }
        Val::Resource(res) => {
            // reborrow required for some reason due to implt AsContext consuming reference

            if let Ok(res) = res.try_into_resource::<ReflectReference>(&mut *data)
                && let Some(res) = data.data_mut().get_ref(&res)
            {
                return ScriptValue::Reference(res.clone());
            }
            return ScriptValue::Unit;
        }
        _ => ScriptValue::Unit,
    }
}

/// The wasmtime handler for events.  
pub fn wasmtime_handler(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback_label: &CallbackLabel,
    context: &mut WasmtimeContext,
    world_id: WorldId,
) -> Result<ScriptValue, InteropError> {
    let config = WasmtimeScriptingPlugin::readonly_configuration(world_id);

    config
        .pre_handling_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    // Get the exported function by callback name
    let func = context.instance.get_func(
        &mut context.store,
        &callback_label.as_ref().replace("_", "-"),
    );

    // .and_then(|(item, f)| context.instance.get_func(&mut context.store, f));

    let func = match func {
        Some(f) => f,
        None => {
            info!(
                "Context {} is not subscribed to callback {}",
                context_key,
                callback_label.as_ref()
            );
            return Ok(ScriptValue::Unit);
        }
    };

    let mut context_mut = context.store.as_context_mut();

    // Convert args to Val
    let input_vals: Vec<Val> = args
        .into_iter()
        .map(|v| script_value_to_val(v, &mut context_mut))
        .collect();

    // Call the function
    let mut output_vals = vec![]; // Placeholder for result  
    func.call(&mut context_mut, &input_vals, &mut output_vals)
        .map_err(|e| to_interop_error(e))?;

    // Convert result back to ScriptValue
    let result = output_vals
        .first()
        .map(|v| val_to_script_value(v, &mut context_mut))
        .unwrap_or(ScriptValue::Unit);

    Ok(result)
}
