//! Lua specific traits and types relevant to binding rust types to Lua types

// /// Trait for types which can be converted into a value representible in a scripting language from a reference,
// pub trait RefIntoLua: Reflect {
//     fn ref_into_script_value(&self, ctxt: ScriptContext) -> ScriptValueType;
// }

// /// Type data corresponding to the [`RefIntoScriptValue`] trait
// pub struct ReflectRefIntoScriptValue<ScriptValueType: Sized, ScriptContext> {
//     pub ref_into_script_value: fn(&dyn Reflect, ctxt: ScriptContext) -> ScriptValueType,
// }

// impl<T, C> Clone for ReflectRefIntoScriptValue<T, C> {
//     fn clone(&self) -> Self {
//         Self {
//             ref_into_script_value: self.ref_into_script_value,
//         }
//     }
// }

// /// Trait for types which be converted from a value representible in a scripting language to a reference to the original type.
// pub trait ScriptValueToRef<ScriptValueType: Sized, ScriptContext: Sized>: Reflect {
//     fn script_value_to_ref(script_value: &ScriptValueType, ctxt: ScriptContext) -> &Self;
// }

// /// Type data corresponding to the [`ScriptValueToRef`] trait
// pub struct ReflectScriptValueToRef<ScriptValueType: Sized, ScriptContext> {
//     pub script_value_to_ref:
//         fn(script_value: &ScriptValueType, ctxt: ScriptContext) -> &dyn Reflect,
// }

// impl<T, C> Clone for ReflectScriptValueToRef<T, C> {
//     fn clone(&self) -> Self {
//         Self {
//             script_value_to_ref: self.script_value_to_ref,
//         }
//     }
// }
