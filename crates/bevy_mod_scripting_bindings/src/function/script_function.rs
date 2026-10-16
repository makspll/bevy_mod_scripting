//! Implementations of the [`ScriptFunction`] and [`ScriptFunctionMut`] traits for functions with up to 13 arguments.

use super::MagicFunctions;
use super::{from::FromScript, into::IntoScript, namespace::FunctionScope};
use crate::docgen::info::{FunctionInfo, GetFunctionInfo};
use crate::function::arg_meta::ArgMeta;
use crate::{ScriptValue, error::InteropError};
use crate::{StaticTypeRef, VariadicTuple};
use bevy_ecs::prelude::Resource;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::DisplayWithTypeInfo;
use bevy_mod_scripting_world::{ThreadWorldContainer, WorldGuard};
use bevy_platform::collections::HashMap;
use bevy_reflect::{Reflect, TypeInfo};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[diagnostic::on_unimplemented(
    message = "This function does not fulfil the requirements to be a script callable function. All arguments must implement the ScriptArgument trait and all return values must implement the ScriptReturn trait"
)]
/// A trait implemented by functions which can act as dynamic script functions, which can then be registered against a [`ScriptFunctionRegistry`].
pub trait ScriptFunction<'env, Marker> {
    /// Convert this function into a [`DynamicScriptFunction`]
    fn into_dynamic_script_function(self) -> DynamicScriptFunction;
}

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. Registering functions also requires they implement GetTypeDependencies",
    note = "If you're trying to use a non-primitive type, you might need to use V<T> R<T> or M<T> wrappers"
)]
/// A trait implemented by functions which can act as mutable dynamic script functions.
pub trait ScriptFunctionMut<'env, Marker> {
    /// Convert this function into a [`DynamicScriptFunctionMut`]
    fn into_dynamic_script_function_mut(self) -> DynamicScriptFunctionMut;
}

/// The caller context when calling a script function.
/// Functions can choose to react to caller preferences such as converting 1-indexed numbers to 0-indexed numbers
#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
#[reflect(opaque)]
pub struct FunctionCallContext {
    language: Language,
    location_context: Option<LocationContext>,
}

impl std::fmt::Display for FunctionCallContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("in language: ")?;
        self.language.fmt(f)?;
        if let Some(context) = &self.location_context {
            if let Some(script_name) = &context.script_name {
                f.write_str(", in script: ")?;
                script_name.fmt(f)?;
            }

            f.write_str(", at line: ")?;
            context.line.fmt(f)?;

            if let Some(col) = &context.col {
                f.write_str(", at column: ")?;
                col.fmt(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// Describes a location within a script
pub struct LocationContext {
    /// The name of the script the function call originates from
    pub script_name: Option<String>,
    /// The line number
    pub line: u32,
    /// The column number
    pub col: Option<u32>,
}

impl FunctionCallContext {
    /// Create a new FunctionCallContext with the given 1-indexing conversion preference
    pub const fn new(language: Language) -> Self {
        Self {
            language,
            location_context: None,
        }
    }

    /// Creates a new function call context with location information
    pub const fn new_with_location(
        language: Language,
        location_context: Option<LocationContext>,
    ) -> Self {
        Self {
            language,
            location_context,
        }
    }

    /// Tries to access the world, returning an error if the world is not available
    #[profiling::function]
    pub fn world<'l>(&self) -> Result<WorldGuard<'l>, InteropError> {
        Ok(ThreadWorldContainer.try_get_context().map(|c| c.world)?)
    }
    /// Whether the caller uses 1-indexing on all indexes and expects 0-indexing conversions to be performed.
    #[profiling::function]
    pub fn convert_to_0_indexed(&self) -> bool {
        self.language.one_indexed()
    }

    /// Gets the scripting language of the caller
    #[profiling::function]
    pub fn language(&self) -> Language {
        self.language.clone()
    }

    /// Returns call location inside the script if available
    pub fn location(&self) -> Option<&LocationContext> {
        self.location_context.as_ref()
    }
}

#[derive(Reflect, Clone, DebugWithTypeInfo)]
#[reflect(opaque)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// A dynamic script function.
pub struct DynamicScriptFunction {
    /// The meta information about the function
    pub info: Arc<FunctionInfo>,
    // TODO: info about the function, this is hard right now because of non 'static lifetimes in wrappers, we can't use TypePath etc
    #[debug_with_type_info(skip)]
    func: Arc<
        dyn Fn(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
    >,
}

#[derive(Reflect, Clone, DebugWithTypeInfo)]
#[reflect(opaque)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// A dynamic mutable script function.
pub struct DynamicScriptFunctionMut {
    /// The meta information about the function
    pub info: Arc<FunctionInfo>,
    #[debug_with_type_info(skip)]
    func: Arc<
        RwLock<
            // I'd rather consume an option or something instead of having the RWLock but I just wanna get this release out
            dyn FnMut(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue
                + Send
                + Sync
                + 'static,
        >,
    >,
}

impl DisplayWithTypeInfo for DynamicScriptFunction {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        f.write_str("fn ")?;
        let name = &self.info.name;
        f.write_str(name)?;
        f.write_str("(")?;
        for arg in &self.info.arg_info {
            arg.display_with_type_info(f, type_info_provider)?;
            f.write_str(", ")?;
        }
        f.write_str(") -> ")?;
        self.info
            .return_info
            .display_with_type_info(f, type_info_provider)
    }
}

impl DisplayWithTypeInfo for DynamicScriptFunctionMut {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        f.write_str("fn mut ")?;
        let name = &self.info.name;
        f.write_str(name)?;
        f.write_str("(")?;
        for arg in &self.info.arg_info {
            arg.display_with_type_info(f, type_info_provider)?;
            f.write_str(", ")?;
        }
        f.write_str(") -> ")?;
        self.info
            .return_info
            .display_with_type_info(f, type_info_provider)
    }
}

#[profiling::all_functions]
impl DynamicScriptFunction {
    /// Call the function with the given arguments and caller context.
    ///
    /// In the case of errors wraps the error in a [`InteropError::function_interop_error`] to provide more context.
    pub fn call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        profiling::scope!("Dynamic Call ", self.name().deref());
        let args = args.into_iter().collect::<VecDeque<_>>();
        // should we be inlining call errors into the return value?
        let return_val = (self.func)(context.clone(), args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace,
                e,
                Some(context),
            )),
            v => Ok(v),
        }
    }

    /// Get the name of the function
    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    /// Binds a function shared between all types, i.e. one registered on [`StaticTypeRef`], to the given type.
    ///
    /// The returned function is called without the type argument, which is filled in on each call.
    /// Its signature is that of [`FunctionInfo::bind_to_type`].
    pub fn bind_to_type(&self, type_info: &'static TypeInfo) -> Self {
        let mut bound = self.curry(ScriptValue::Type(type_info.type_id()));
        bound.info = Arc::new(self.info.bind_to_type(type_info));
        bound
    }

    /// Fixes the first argument of the function, i.e. a receiver or a type.
    ///
    /// The returned function is called with the remaining arguments. Its info is unchanged.
    pub fn curry(&self, first: ScriptValue) -> Self {
        let inner = self.func.clone();
        DynamicScriptFunction {
            info: self.info.clone(),
            func: Arc::new(move |context, mut args| {
                args.push_front(first.clone());
                inner(context, args)
            }),
        }
    }

    /// Set the meta information about the function
    pub fn with_info(mut self, info: FunctionInfo) -> Self {
        self.info = Arc::new(info);
        self
    }
}

#[profiling::all_functions]
impl DynamicScriptFunctionMut {
    /// Call the function with the given arguments and caller context.
    ///
    /// In the case of errors wraps the error in a [`InteropError::function_interop_error`] to provide more context.
    pub fn call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        profiling::scope!("Dynamic Call Mut", self.name().deref());
        let args = args.into_iter().collect::<VecDeque<_>>();
        // should we be inlining call errors into the return value?
        let mut write = self.func.write();
        let return_val = (write)(context.clone(), args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace,
                e,
                Some(context),
            )),
            v => Ok(v),
        }
    }

    /// Get the name of the function
    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    /// Set the meta information about the function
    pub fn with_info(mut self, info: FunctionInfo) -> Self {
        self.info = Arc::new(info);
        self
    }
}

impl PartialEq for DynamicScriptFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self as *const Self, other as *const Self)
    }
}

impl PartialEq for DynamicScriptFunctionMut {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self as *const Self, other as *const Self)
    }
}

impl<F> From<F> for DynamicScriptFunction
where
    F: Fn(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
{
    fn from(fn_: F) -> Self {
        DynamicScriptFunction {
            info: FunctionInfo::default()
                .with_name(std::any::type_name::<F>())
                .into(),
            func: Arc::new(fn_),
        }
    }
}

impl<F> From<F> for DynamicScriptFunctionMut
where
    F: FnMut(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
{
    fn from(fn_: F) -> Self {
        DynamicScriptFunctionMut {
            info: FunctionInfo::default()
                .with_name(std::any::type_name::<F>())
                .into(),
            func: Arc::new(RwLock::new(fn_)),
        }
    }
}

/// Equivalent to [`AppScriptFunctionRegistry`] but stores functions with a more convenient signature for scripting to avoid boxing every argument.
#[derive(Clone, Default, Resource, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct AppScriptFunctionRegistry(pub ScriptFunctionRegistryArc);

impl Deref for AppScriptFunctionRegistry {
    type Target = ScriptFunctionRegistryArc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppScriptFunctionRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Default, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// A thread-safe reference counted wrapper around a [`ScriptFunctionRegistry`]
pub struct ScriptFunctionRegistryArc(pub Arc<RwLock<ScriptFunctionRegistry>>);

#[profiling::all_functions]
impl ScriptFunctionRegistryArc {
    /// claim a read lock on the registry
    pub fn read(&self) -> RwLockReadGuard<'_, ScriptFunctionRegistry> {
        self.0.read()
    }

    /// claim a write lock on the registry
    pub fn write(&mut self) -> RwLockWriteGuard<'_, ScriptFunctionRegistry> {
        self.0.write()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// A key used to identify a function in the registry
pub struct FunctionKey {
    /// The name of the function
    pub name: Cow<'static, str>,
    /// The type the function is on
    pub type_id: TypeId,
}

#[derive(Default, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// A registry of the functions on types.
///
/// Functions which don't belong to a type are members of a namespace instead, see [`crate::NamespaceMembers`].
pub struct ScriptFunctionRegistry {
    functions: HashMap<FunctionKey, DynamicScriptFunction>,
    /// Functions on [`StaticTypeRef`] bound to a type, see [`ScriptFunctionRegistry::get_bound_to_type`].
    /// Cleared whenever the functions change.
    #[debug_with_type_info(skip)]
    bound_to_type: Mutex<HashMap<(TypeId, Cow<'static, str>), DynamicScriptFunction>>,
    /// A registry of magic functions
    pub magic_functions: MagicFunctions,
}

/// Turns a function into a [`DynamicScriptFunction`] with its signature, docs and argument names.
pub(crate) fn documented_function<'env, F, M>(
    scope: FunctionScope,
    name: Cow<'static, str>,
    func: F,
    docs: Option<impl Into<Cow<'static, str>>>,
    arg_names: Option<&'static [&'static str]>,
) -> DynamicScriptFunction
where
    F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
{
    let info = func.get_function_info(name, scope);
    let info = match docs {
        Some(docs) => info.with_docs(docs.into()),
        None => info,
    };
    let info = match arg_names {
        Some(arg_names) => info.with_arg_names(arg_names),
        None => info,
    };
    func.into_dynamic_script_function().with_info(info)
}

#[profiling::all_functions]
impl ScriptFunctionRegistry {
    /// Register a script function on the given type, replacing any function already registered with that name.
    ///
    /// There is one function per name and type: for arguments of different types, accept a [`Union`](crate::Union) or a [`ScriptValue`].
    pub fn register<'env, F, M>(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        func: F,
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        self.insert_function(type_id, name, func, None::<&'static str>, None);
    }

    /// Equivalent to [`ScriptFunctionRegistry::register`] but with the ability to provide documentation for the function.
    pub fn register_documented<'env, F, M>(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        func: F,
        docs: &'static str,
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        self.insert_function(type_id, name, func, Some(docs), None);
    }

    /// Equivalent to [`ScriptFunctionRegistry::register`] but with the ability to provide argument names for the function as well as documentation.
    pub fn register_with_arg_names<'env, F, M>(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        func: F,
        docs: &'static str,
        arg_names: &'static [&'static str],
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        self.insert_function(type_id, name, func, Some(docs), Some(arg_names));
    }

    /// Remove a function from the registry if it exists. Returns the removed function if it was found.
    pub fn remove(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
    ) -> Option<DynamicScriptFunction> {
        let name = name.into();
        self.functions_mut().remove(&FunctionKey { name, type_id })
    }

    /// Registers a script function with the given name, replacing any function with the same name.
    fn insert_function<'env, F, M>(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        func: F,
        docs: Option<impl Into<Cow<'static, str>>>,
        arg_names: Option<&'static [&'static str]>,
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        let name: Cow<'static, str> = name.into();
        let func = documented_function(
            FunctionScope::OnType(type_id),
            name.clone(),
            func,
            docs,
            arg_names,
        );
        self.functions_mut()
            .insert(FunctionKey { name, type_id }, func);
    }

    /// Check if a function with the given name exists on the type
    pub fn contains(&self, type_id: TypeId, name: impl Into<Cow<'static, str>>) -> bool {
        self.functions.contains_key(&FunctionKey {
            name: name.into(),
            type_id,
        })
    }

    /// Get the function with the given name on the type
    pub fn get_function(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<&DynamicScriptFunction, Cow<'static, str>> {
        let key = FunctionKey {
            name: name.into(),
            type_id,
        };
        match self.functions.get(&key) {
            Some(func) => Ok(func),
            None => Err(key.name),
        }
    }

    /// Gets a function registered on [`StaticTypeRef`], bound to the given type with [`DynamicScriptFunction::bind_to_type`].
    ///
    /// Bound functions are cached until the registry changes.
    pub fn get_bound_to_type(
        &self,
        type_info: &'static TypeInfo,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'static, str>> {
        let key = (type_info.type_id(), name.into());
        if let Some(func) = self.bound_to_type.lock().get(&key) {
            return Ok(func.clone());
        }
        let func = self
            .get_function(TypeId::of::<StaticTypeRef>(), key.1.clone())?
            .bind_to_type(type_info);
        self.bound_to_type.lock().insert(key, func.clone());
        Ok(func)
    }

    /// The functions, for changing them, which invalidates the bound functions.
    fn functions_mut(&mut self) -> &mut HashMap<FunctionKey, DynamicScriptFunction> {
        self.bound_to_type.get_mut().clear();
        &mut self.functions
    }

    /// Iterates over all functions
    pub fn iter_all(&self) -> impl Iterator<Item = (&FunctionKey, &DynamicScriptFunction)> {
        self.functions.iter()
    }

    /// Iterates over the functions on the given type
    pub fn iter_type(
        &self,
        type_id: TypeId,
    ) -> impl Iterator<Item = (&FunctionKey, &DynamicScriptFunction)> {
        self.functions
            .iter()
            .filter(move |(key, _)| key.type_id == type_id)
    }

    /// Insert a function into the registry with the given key, keeping its function info as is.
    /// Do not use unless you really need to.
    pub fn raw_insert(
        &mut self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        func: DynamicScriptFunction,
    ) {
        self.functions_mut().insert(
            FunctionKey {
                name: name.into(),
                type_id,
            },
            func,
        );
    }
}

macro_rules! count {
        () => (0usize);
        ( $x:tt $($xs:tt)* ) => (1usize + $crate::function::script_function::count!($($xs)*));
}

/// Pops the stack of args depending on how many are requested by the current argument.
/// If failed to find argument, returns None
fn pop_args_stack_for_arg<A: ArgMeta>(args: &mut VecDeque<ScriptValue>) -> Option<ScriptValue> {
    if A::variadic() {
        // just absorb the rest, tuplify it
        if !args.is_empty() {
            return Some(ScriptValue::Tuple(VariadicTuple(std::mem::take(args))));
        }
    }

    args.pop_front().or_else(A::default_value)
}

pub(crate) use count;

macro_rules! impl_script_function {

    ($( $param:ident ),* ) => {
        // all of this is pretty heavy on the compile time.
        // ideally we'd do less, but for now this will suffice

        // Fn(T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : -> O => O );
        // FnMut(T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : -> O => O );

        // Fn(CallerContext, T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: FunctionCallContext) -> O => O);
        // FnMut(FunctionCallContext, T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: FunctionCallContext) -> O => O);

        // Fn(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : -> O => Result<O, InteropError> where s);
        // FnMut(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : -> O => Result<O, InteropError> where s);

        // Fn(FunctionCallContext, WorldGuard<'w>, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: FunctionCallContext)-> O => Result<O, InteropError> where s);
        // FnMut(FunctionCallContext, WorldGuard<'w>, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: FunctionCallContext) -> O => Result<O, InteropError> where s);


    };

    (@ $trait_type:ident $fn_type:ident $dynamic_type:ident $trait_fn_name:ident $( $param:ident ),* :  $(($context:ident: $contextty:ty))? -> O => $res:ty $(where $out:ident)?) => {
        #[allow(non_snake_case)]
        impl<
            'env,
            $( $param: FromScript + ArgMeta,)*
            O,
            F
        > $trait_type<'env,
            fn( $($contextty,)? $($param ),* ) -> $res
        > for F
        where
            O: IntoScript,
            F: $fn_type(  $($contextty,)? $($param ),* ) -> $res + Send + Sync + 'static,
            $( $param::This<'env>: Into<$param>,)*
        {
            #[allow(unused_mut,unused_variables)]
            #[profiling::function]
            fn $trait_fn_name(mut self) -> $dynamic_type {

                let func = (move |caller_context: FunctionCallContext, mut args: VecDeque<ScriptValue> | {
                    let res: Result<ScriptValue, InteropError> = (|| {
                        profiling::scope!("script function call mechanism");
                        let received_args_len = args.len();
                        let expected_arg_count = count!($($param )*);

                        $( let $context = caller_context.clone(); )?
                        let world = caller_context.world()?;
                        // Safety: we're not holding any references to the world, the arguments which might have aliased will always be dropped
                        let ret: Result<ScriptValue, InteropError> = unsafe {
                            world.with_access_scope(||{
                                let mut current_arg = 0;

                                $(let $param = {
                                        profiling::scope!("argument conversion", &format!("argument #{}", current_arg));
                                        current_arg += 1;
                                        let $param = match pop_args_stack_for_arg::<$param>(&mut args) {
                                            Some($param) => $param,
                                            None => {
                                                return Err(InteropError::argument_count_mismatch(expected_arg_count,received_args_len));
                                            }
                                        };
                                        let $param = <$param>::from_script($param, world.clone())
                                            .map_err(|e| InteropError::function_arg_conversion_error(current_arg.to_string(), e))?;
                                        $param
                                    };
                                )*

                                let ret = {
                                    let out = {
                                        profiling::scope!("function call");
                                        self( $( $context,)?  $( $param.into(), )* )
                                    };

                                    $(
                                        let $out = out?;
                                        let out = $out;
                                    )?
                                    profiling::scope!("return type conversion");
                                    out.into_script(world.clone()).map_err(|e| InteropError::function_arg_conversion_error("return value".to_owned(), e))
                                };
                                ret
                            })?
                        };
                        ret
                    })();
                    let script_value: ScriptValue = res.into();
                    script_value
                });

                func.into()
            }
        }
    };
}

variadics_please::all_tuples!(impl_script_function, 0, 13, T);

#[cfg(test)]
mod test {
    use crate::{CurrentScriptAttachment, WorldExtensions};

    use super::*;
    use bevy_ecs::{prelude::Component, world::World};
    use bevy_mod_scripting_world::{ThreadScriptContext, WorldAccessGuard};

    fn with_local_world<F: Fn()>(f: F) {
        let mut world = World::default();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldGuard::with_static_guard(&mut world, cache, |world| {
            ThreadWorldContainer.set_context(ThreadScriptContext { world });
            f()
        });
    }

    #[test]
    fn test_register_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;

        let namespace = TypeId::of::<()>();
        registry.register(namespace, "test", fn_);
        let function = registry
            .get_function(namespace, "test")
            .expect("Failed to get function");

        assert_eq!(function.info.name, "test");
        assert_eq!(function.info.namespace, FunctionScope::OnType(namespace));
    }

    #[test]
    fn test_optional_argument_not_required() {
        let fn_ = |a: usize, b: Option<usize>| a + b.unwrap_or(0);
        let script_function = fn_.into_dynamic_script_function();

        with_local_world(|| {
            let out = script_function
                .call(
                    vec![ScriptValue::from(1)],
                    FunctionCallContext::new(Language::Lua),
                )
                .unwrap();

            assert_eq!(out, ScriptValue::from(1));
        });
    }

    #[test]
    fn test_invalid_amount_of_args_errors_nicely() {
        let fn_ = |a: usize, b: usize| a + b;
        let script_function = fn_.into_dynamic_script_function();

        with_local_world(|| {
            let out = script_function.call(
                vec![ScriptValue::from(1)],
                FunctionCallContext::new(Language::Lua),
            );

            assert!(out.is_err());

            let gotten = out.unwrap_err();
            let expected_function_name = "<bevy_mod_scripting_bindings::function::script_function::test::test_invalid_amount_of_args_errors_nicely::{{closure}} as bevy_mod_scripting_bindings::function::script_function::ScriptFunction<'_, fn(usize, usize) -> usize>>::into_dynamic_script_function::{{closure}}";
            let expected_namespace = FunctionScope::Member;

            if let InteropError::FunctionInteropError {
                function_name,
                on,
                error,
                ..
            } = gotten
            {
                assert_eq!(*function_name, expected_function_name);
                assert_eq!(*on, expected_namespace);
                if let InteropError::ArgumentCountMismatch { expected, got } = error.as_ref() {
                    assert_eq!(*expected, 2);
                    assert_eq!(*got, 1);
                } else {
                    panic!("Expected ArgumentCountMismatch, got {error:?}");
                }
            } else {
                panic!("Expected FunctionInteropError, got {gotten:?}");
            }
        });
    }

    #[test]
    fn test_interrupted_call_releases_access_scope() {
        #[derive(Component, Reflect)]
        struct Comp;

        let fn_ = |_a: crate::function::from::M<Comp>| 0usize;
        let script_function = fn_.into_dynamic_script_function();

        with_local_world(|| {
            let out = script_function.call(
                vec![ScriptValue::from(1)],
                FunctionCallContext::new(Language::Lua),
            );

            assert!(out.is_err());
            let world = FunctionCallContext::new(Language::Lua).world().unwrap();
            // assert no access is held
            assert!(world.list_accesses().is_empty());
        });
    }

    #[test]
    fn test_registering_a_name_again_replaces_the_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = TypeId::of::<()>();
        registry.register(namespace, "test", fn_);
        let fn_2 = |a: usize, b: i32| a + (b as usize);
        registry.register(namespace, "test", fn_2);

        let function = registry
            .get_function(namespace, "test")
            .expect("Failed to get function");

        assert_eq!(function.info.name, "test");
        assert_eq!(function.info.arg_info[1].type_id, TypeId::of::<i32>());
        assert_eq!(registry.iter_all().count(), 1);
    }

    #[test]
    fn test_remove_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = TypeId::of::<()>();
        registry.register(namespace, "test", fn_);
        let removed = registry.remove(namespace, "test");
        assert!(removed.is_some());
        let removed = registry.remove(namespace, "test");
        assert!(removed.is_none());
    }
}
