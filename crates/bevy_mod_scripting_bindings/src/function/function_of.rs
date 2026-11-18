//! Functions passed to and from scripts, documented with their signature, see [`FunctionOf`].

use std::{marker::PhantomData, ops::Deref};

use bevy_mod_scripting_world::{CallScope, WorldGuard};
use bevy_reflect::TypeRegistry;

use crate::{
    ScriptValue,
    conversions::{from::FromScript, into::IntoScript},
    docgen::{
        info::GetFunctionInfo,
        typed_through::{ThroughTypeInfo, TypedThrough},
    },
    error::InteropError,
    function::{
        arg_meta::ArgMeta,
        namespace::FunctionScope,
        script_function::DynamicScriptFunction,
        type_dependencies::{GetFunctionTypeDependencies, GetTypeDependencies},
    },
};

/// A function, documented with the types of its arguments and of what it returns, i.e. `FunctionOf<fn(V<Entity>) -> bool>`
/// is `fun(arg1: Entity): boolean` in Lua. It converts like a [`DynamicScriptFunction`]:
///
/// ```rust,ignore
/// fn on_hit(callback: FunctionOf<fn(V<Entity>, f32) -> ()>) { .. }
/// ```
///
/// The signature only documents the function, calls aren't checked against it.
pub struct FunctionOf<F>(pub DynamicScriptFunction, PhantomData<fn() -> F>);

impl<F> FunctionOf<F> {
    /// Wrap a function.
    pub fn new(function: DynamicScriptFunction) -> Self {
        FunctionOf(function, PhantomData)
    }

    /// Unwrap the function.
    pub fn into_inner(self) -> DynamicScriptFunction {
        self.0
    }
}

impl<F> Clone for FunctionOf<F> {
    fn clone(&self) -> Self {
        FunctionOf::new(self.0.clone())
    }
}

impl<F> Deref for FunctionOf<F> {
    type Target = DynamicScriptFunction;

    fn deref(&self) -> &DynamicScriptFunction {
        &self.0
    }
}

impl<F> FromScript for FunctionOf<F> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        DynamicScriptFunction::from_script(value, scope).map(FunctionOf::new)
    }

    fn could_convert(value: &ScriptValue) -> bool {
        DynamicScriptFunction::could_convert(value)
    }
}

impl<F> IntoScript for FunctionOf<F> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl<F> ArgMeta for FunctionOf<F> {}

// a signature `fn(A) -> R` is itself a function, described by the machinery of registered functions
impl<F: GetFunctionInfo<F>> TypedThrough for FunctionOf<F> {
    fn through_type_info() -> ThroughTypeInfo {
        let info = F::function_info("".into(), FunctionScope::Builtin);
        let described = |type_info: Option<ThroughTypeInfo>| {
            type_info.unwrap_or(ThroughTypeInfo::Primitive(
                bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind::ScriptValue,
            ))
        };
        ThroughTypeInfo::Function {
            params: info
                .arg_info
                .into_iter()
                .map(|arg| described(arg.type_info))
                .collect(),
            returns: Box::new(described(info.return_info.type_info)),
        }
    }
}

impl<F: GetFunctionTypeDependencies<F>> GetTypeDependencies for FunctionOf<F> {
    type Underlying = DynamicScriptFunction;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        F::register_type_dependencies(registry)
    }
}

/// A [`FunctionOf<F>`], or a list of a function and values it is called with before its own arguments:
/// `{ f, a, b }` is `f` called as `f(a, b, ..)`.
///
/// It's for languages whose closures can't capture what the function needs (Rune closures handed to BMS only
/// capture constants). It is documented as `F` only, so typed languages are offered the plain function.
pub struct WithDeps<F>(pub FunctionOf<F>);

impl<F> WithDeps<F> {
    /// Unwrap the function, called with its dependencies first.
    pub fn into_inner(self) -> DynamicScriptFunction {
        self.0.0
    }
}

impl<F> Clone for WithDeps<F> {
    fn clone(&self) -> Self {
        WithDeps(self.0.clone())
    }
}

impl<F> FromScript for WithDeps<F> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        let ScriptValue::List(mut list) = value else {
            return FunctionOf::from_script(value, scope).map(WithDeps);
        };
        let function = match list.pop_front() {
            Some(ScriptValue::Function(function)) => function,
            other => {
                return Err(InteropError::string(format!(
                    "expected a function, or a list of a function and its dependencies, got a list starting with: {other:?}"
                )));
            }
        };
        if list.is_empty() {
            return Ok(WithDeps(FunctionOf::new(function)));
        }
        let deps = list;
        let curried = DynamicScriptFunction::from(
            move |context, mut args: std::collections::VecDeque<ScriptValue>| {
                for dep in deps.iter().rev() {
                    args.push_front(dep.clone());
                }
                function
                    .call(args, context)
                    .unwrap_or_else(ScriptValue::Error)
            },
        );
        Ok(WithDeps(FunctionOf::new(curried)))
    }

    fn could_convert(value: &ScriptValue) -> bool {
        match value {
            ScriptValue::List(list) => matches!(list.front(), Some(ScriptValue::Function(_))),
            value => FunctionOf::<F>::could_convert(value),
        }
    }
}

impl<F> IntoScript for WithDeps<F> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl<F> ArgMeta for WithDeps<F> {}

impl<F: GetFunctionInfo<F>> TypedThrough for WithDeps<F> {
    fn through_type_info() -> ThroughTypeInfo {
        FunctionOf::<F>::through_type_info()
    }
}

impl<F: GetFunctionTypeDependencies<F>> GetTypeDependencies for WithDeps<F> {
    type Underlying = DynamicScriptFunction;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        F::register_type_dependencies(registry)
    }
}
