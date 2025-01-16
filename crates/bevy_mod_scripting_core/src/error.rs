use crate::bindings::{
    access_map::DisplayCodeLocation, function::namespace::Namespace,
    pretty_print::DisplayWithWorld, script_value::ScriptValue, ReflectBaseType, ReflectReference,
};
use bevy::{
    ecs::component::ComponentId,
    prelude::Entity,
    reflect::{func::FunctionError, PartialReflect, Reflect},
};
use std::{
    any::TypeId,
    fmt::{Debug, Display},
    ops::Deref,
    str::Utf8Error,
    sync::Arc,
};

pub type ScriptResult<T> = Result<T, ScriptError>;

/// An error with an optional script Context
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(opaque)]
pub struct ScriptError(pub Arc<ScriptErrorInner>);

impl std::error::Error for ScriptError {}

impl Deref for ScriptError {
    type Target = ScriptErrorInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The innards are separated to reduce the size of this error
#[derive(Debug, Clone)]
pub struct ScriptErrorInner {
    pub script: Option<String>,
    pub context: String,
    pub reason: Arc<ErrorKind>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Display(Box<dyn std::error::Error + Send + Sync>),
    WithWorld(Box<dyn DisplayWithWorld + Send + Sync>),
}

impl DisplayWithWorld for ErrorKind {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        match self {
            ErrorKind::Display(e) => e.to_string(),
            ErrorKind::WithWorld(e) => e.display_with_world(world),
        }
    }

    fn display_without_world(&self) -> String {
        match self {
            ErrorKind::Display(e) => e.to_string(),
            ErrorKind::WithWorld(e) => e.display_without_world(),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_without_world())
    }
}

impl PartialEq for ScriptErrorInner {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context
    }
}

impl ScriptError {
    #[cfg(feature = "mlua_impls")]
    /// Destructures mlua error into a script error, taking care to preserve as much information as possible
    pub fn from_mlua_error(error: mlua::Error) -> Self {
        match error {
            mlua::Error::CallbackError { traceback, cause }
                if matches!(cause.as_ref(), mlua::Error::ExternalError(_)) =>
            {
                let inner = cause.deref().clone();
                Self::from_mlua_error(inner).with_context(traceback)
            }
            e => {
                if let Some(inner) = e.downcast_ref::<InteropError>() {
                    Self::new(inner.clone())
                } else if let Some(inner) = e.downcast_ref::<ScriptError>() {
                    inner.clone()
                } else {
                    Self::new_external(e)
                }
            }
        }
    }

    // #[cfg(feature = "rhai_impls")]
    // pub fn from_rhai_error(error: rhai::EvalAltResult) -> Self {
    //     match error {
    //         rhai::EvalAltResult::ErrorSystem(message, error) => {
    //             if let Some(inner) = error.downcast_ref::<InteropError>() {
    //                 Self::new(inner.clone())
    //             } else if let Some(inner) = error.downcast_ref::<ScriptError>() {
    //                 inner.clone()
    //             } else {
    //                 Self::new_external_boxed(error).with_context(message)
    //             }
    //         }
    //         _ => Self::new_external(error),
    //     }
    // }

    pub fn new_external(reason: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new_external_boxed(Box::new(reason))
    }

    pub fn new_external_boxed(reason: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Arc::new(ErrorKind::Display(reason)),
            context: Default::default(),
        }))
    }

    pub fn new(reason: impl DisplayWithWorld + Send + Sync + 'static) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Arc::new(ErrorKind::WithWorld(Box::new(reason))),
            context: Default::default(),
        }))
    }

    pub fn with_script<S: ToString>(self, script: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: Some(script.to_string()),
            context: self.0.context.clone(),
            reason: self.0.reason.clone(),
        }))
    }

    pub fn with_context<S: ToString>(self, context: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: self.0.script.clone(),
            context: format!("{}\n{}", self.0.context, context.to_string()),
            reason: self.0.reason.clone(),
        }))
    }
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_without_world())
    }
}

impl DisplayWithWorld for ScriptError {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        if let Some(script) = &self.0.script {
            format!(
                "error in script `{}`: {}.\nContext:{}",
                script,
                self.0.reason.display_with_world(world),
                self.0.context
            )
        } else {
            format!(
                "error: {}.\nContext:{}",
                self.0.reason.display_with_world(world),
                self.0.context
            )
        }
    }

    fn display_without_world(&self) -> String {
        if let Some(script) = &self.0.script {
            format!(
                "error in script `{}`: {}.\nContext:{}",
                script, self.0.reason, self.0.context,
            )
        } else {
            format!("error: {}.\nContext:{}", self.0.reason, self.0.context)
        }
    }
}

#[cfg(feature = "mlua_impls")]
impl From<ScriptError> for mlua::Error {
    fn from(value: ScriptError) -> Self {
        mlua::Error::external(value)
    }
}

#[cfg(feature = "mlua_impls")]
impl From<InteropError> for mlua::Error {
    fn from(value: InteropError) -> Self {
        mlua::Error::external(value)
    }
}

#[cfg(feature = "mlua_impls")]
impl From<mlua::Error> for ScriptError {
    fn from(value: mlua::Error) -> Self {
        ScriptError::from_mlua_error(value)
    }
}

// #[cfg(feature = "rhai_impls")]
// impl From<rhai::ParseError> for ScriptError {
//     fn from(value: rhai::ParseError) -> Self {
//         ScriptError::new_external(value)
//     }
// }

// #[cfg(feature = "rhai_impls")]
// impl From<Box<rhai::EvalAltResult>> for ScriptError {
//     fn from(value: Box<rhai::EvalAltResult>) -> Self {
//         ScriptError::from_rhai_error(*value)
//     }
// }

// #[cfg(feature = "rhai_impls")]
// impl From<ScriptError> for Box<rhai::EvalAltResult> {
//     fn from(value: ScriptError) -> Self {
//         Box::new(rhai::EvalAltResult::ErrorSystem(
//             "ScriptError".to_owned(),
//             Box::new(value),
//         ))
//     }
// }

// #[cfg(feature = "rhai_impls")]
// impl From<InteropError> for Box<rhai::EvalAltResult> {
//     fn from(value: InteropError) -> Self {
//         Box::new(rhai::EvalAltResult::ErrorSystem(
//             "InteropError".to_owned(),
//             Box::new(value),
//         ))
//     }
// }

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct InteropError(#[reflect(ignore)] Arc<InteropErrorInner>);

impl std::error::Error for InteropError {}

impl DisplayWithWorld for InteropError {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        self.0.display_with_world(world)
    }

    fn display_without_world(&self) -> String {
        self.0.display_without_world()
    }
}

impl Display for InteropError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_without_world())
    }
}

impl From<InteropError> for ScriptError {
    fn from(val: InteropError) -> Self {
        ScriptError::new(val)
    }
}

impl From<Utf8Error> for ScriptError {
    fn from(val: Utf8Error) -> Self {
        ScriptError::new_external(val)
    }
}

pub trait FlattenError<O, E> {
    fn flatten_interop_error(self) -> Result<O, E>;
}

impl<O> FlattenError<O, InteropError> for Result<Result<O, InteropError>, InteropError> {
    fn flatten_interop_error(self) -> Result<O, InteropError> {
        match self {
            Ok(Ok(o)) => Ok(o),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(e),
        }
    }
}

impl InteropError {
    /// Thrown if a callback requires world access, but is unable to do so due
    /// to the world not being reachable at all via any mechanism.
    pub fn missing_world() -> Self {
        Self(Arc::new(InteropErrorInner::MissingWorld))
    }

    /// Thrown if a callback requires world access, but is unable to do so due
    /// to the world being dropped. I.e. Symptom of a script trying to persist a world reference somewhere.
    pub fn stale_world_access() -> Self {
        Self(Arc::new(InteropErrorInner::StaleWorldAccess))
    }

    /// Thrown if a base type is not registered with the reflection system
    /// and therefore the reference cannot be dereferenced
    pub fn unregistered_base(base: ReflectBaseType) -> Self {
        Self(Arc::new(InteropErrorInner::UnregisteredBase { base }))
    }

    /// Thrown if a base type is not registered with the reflection system
    /// with the specific type data.
    pub fn missing_type_data(type_id: TypeId, type_data: String) -> Self {
        Self(Arc::new(InteropErrorInner::MissingTypeData {
            type_id,
            type_data,
        }))
    }

    /// Thrown if a type cannot be converted from reflect, this can happen if the type was unable to
    /// re-construct itself from a dynamic value.
    pub fn failed_from_reflect(type_id: Option<TypeId>, reason: String) -> Self {
        Self(Arc::new(InteropErrorInner::FailedFromReflect {
            type_id,
            reason,
        }))
    }

    /// Thrown if access to the given reflection base is required but cannot be claimed.
    /// This is likely due to some other script already claiming access to the base.
    pub fn cannot_claim_access(
        base: ReflectBaseType,
        location: Option<std::panic::Location<'static>>,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::CannotClaimAccess {
            base,
            location,
        }))
    }

    /// Thrown if a conversion into the given type is impossible.
    /// Should be thrown with context on the other type if possible.
    pub fn impossible_conversion(into: TypeId) -> Self {
        Self(Arc::new(InteropErrorInner::ImpossibleConversion { into }))
    }

    /// Thrown if a conversion was not fully completed, as a better conversion exists.
    /// If a function might throw this error it should be handled by the caller.
    ///
    /// A user seeing this error is evidence of unfinished logic.
    pub fn better_conversion_exists<T>() -> Self {
        Self(Arc::new(InteropErrorInner::BetterConversionExists {
            context: std::any::type_name::<T>().to_string(),
        }))
    }

    /// Thrown if a value was expected to be of one type but was of another
    pub fn type_mismatch(expected: TypeId, got: Option<TypeId>) -> Self {
        Self(Arc::new(InteropErrorInner::TypeMismatch { expected, got }))
    }

    /// Identical to [`InteropError::type_mismatch`] but for more abstract types
    pub fn string_type_mismatch(expected: String, got: Option<TypeId>) -> Self {
        Self(Arc::new(InteropErrorInner::StringTypeMismatch {
            expected,
            got,
        }))
    }

    /// Thrown if a [`ScriptValue`] could not be converted to the expected type
    pub fn value_mismatch(expected: TypeId, got: ScriptValue) -> Self {
        Self(Arc::new(InteropErrorInner::ValueMismatch { expected, got }))
    }

    /// Thrown if a downcast from a reflect reference to a specific type failed
    pub fn could_not_downcast(from: ReflectReference, to: TypeId) -> Self {
        Self(Arc::new(InteropErrorInner::CouldNotDowncast { from, to }))
    }

    /// Thrown if a garbage collected allocation was attempted to be accessed
    pub fn garbage_collected_allocation(reference: ReflectReference) -> Self {
        Self(Arc::new(InteropErrorInner::GarbageCollectedAllocation {
            reference,
        }))
    }

    /// Thrown if a reflection path is invalid
    pub fn reflection_path_error(error: String, reference: Option<ReflectReference>) -> Self {
        Self(Arc::new(InteropErrorInner::ReflectionPathError {
            error,
            reference,
        }))
    }

    /// Thrown if an operation is not supported on the given base type, optionally with a value argument that was used to carry it out
    pub fn unsupported_operation(
        base: Option<TypeId>,
        value: Option<Box<dyn PartialReflect>>,
        operation: String,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::UnsupportedOperation {
            base,
            value,
            operation,
        }))
    }

    /// Thrown if an invalid index operation was attempted on a value
    pub fn invalid_index(value: ScriptValue, reason: String) -> Self {
        Self(Arc::new(InteropErrorInner::InvalidIndex { value, reason }))
    }

    /// Thrown if an entity was missing or invalid
    pub fn missing_entity(entity: Entity) -> Self {
        Self(Arc::new(InteropErrorInner::MissingEntity { entity }))
    }

    /// Thrown if a component was invalid
    pub fn invalid_component(component_id: ComponentId) -> Self {
        Self(Arc::new(InteropErrorInner::InvalidComponent {
            component_id,
        }))
    }

    /// Thrown when an error happens in a function call. The inner error provides details on the error.
    pub fn function_interop_error(function_name: &str, on: Namespace, error: InteropError) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionInteropError {
            function_name: function_name.to_string(),
            on,
            error,
        }))
    }

    /// Thrown when the error happens after a function call, and an error is thrown by bevy.
    ///
    /// I.e. mismatch in args, or invalid number of arguments
    pub fn function_call_error(inner: FunctionError) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionCallError { inner }))
    }

    pub fn function_arg_conversion_error(argument: String, error: InteropError) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionArgConversionError {
            argument,
            error,
        }))
    }
    pub fn length_mismatch(expected: usize, got: usize) -> Self {
        Self(Arc::new(InteropErrorInner::LengthMismatch {
            expected,
            got,
        }))
    }

    pub fn external_error(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self(Arc::new(InteropErrorInner::OtherError { error }))
    }

    pub fn missing_function(on: TypeId, function_name: String) -> Self {
        Self(Arc::new(InteropErrorInner::MissingFunctionError {
            on,
            function_name,
        }))
    }

    pub fn invalid_access_count(count: usize, expected: usize, context: String) -> Self {
        Self(Arc::new(InteropErrorInner::InvalidAccessCount {
            count,
            expected,
            context,
        }))
    }

    pub fn inner(&self) -> &InteropErrorInner {
        &self.0
    }

    /// Unwraps the inner error
    ///
    /// # Panics
    /// - if there are multiple references to the inner error
    pub fn into_inner(self) -> InteropErrorInner {
        Arc::try_unwrap(self.0).unwrap_or_else(|a| {
            Arc::try_unwrap(a).expect("tried to unwrap interop error while a copy exists")
        })
    }
}

/// For errors to do with reflection, type conversions or other interop issues
#[derive(Debug)]
pub enum InteropErrorInner {
    StaleWorldAccess,
    MissingWorld,
    UnregisteredBase {
        base: ReflectBaseType,
    },
    MissingTypeData {
        type_id: TypeId,
        type_data: String,
    },
    FailedFromReflect {
        type_id: Option<TypeId>,
        reason: String,
    },
    CannotClaimAccess {
        base: ReflectBaseType,
        location: Option<std::panic::Location<'static>>,
    },
    InvalidAccessCount {
        count: usize,
        expected: usize,
        context: String,
    },
    ImpossibleConversion {
        into: TypeId,
    },
    BetterConversionExists {
        context: String,
    },
    TypeMismatch {
        expected: TypeId,
        got: Option<TypeId>,
    },
    StringTypeMismatch {
        expected: String,
        got: Option<TypeId>,
    },
    ValueMismatch {
        expected: TypeId,
        got: ScriptValue,
    },
    LengthMismatch {
        expected: usize,
        got: usize,
    },
    CouldNotDowncast {
        from: ReflectReference,
        to: TypeId,
    },
    GarbageCollectedAllocation {
        reference: ReflectReference,
    },
    ReflectionPathError {
        error: String,
        reference: Option<ReflectReference>,
    },
    UnsupportedOperation {
        base: Option<TypeId>,
        value: Option<Box<dyn PartialReflect>>,
        operation: String,
    },
    InvalidIndex {
        value: ScriptValue,
        reason: String,
    },
    MissingEntity {
        entity: Entity,
    },
    InvalidComponent {
        component_id: ComponentId,
    },
    FunctionCallError {
        inner: FunctionError,
    },
    MissingFunctionError {
        on: TypeId,
        function_name: String,
    },
    FunctionInteropError {
        function_name: String,
        on: Namespace,
        error: InteropError,
    },
    FunctionArgConversionError {
        argument: String,
        error: InteropError,
    },
    OtherError {
        error: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl PartialEq for InteropErrorInner {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

macro_rules! missing_function_error {
    ($function_name:expr, $on:expr) => {
        format!(
            "Could not find function: {} for type: {}",
            $function_name, $on
        )
    };
}

macro_rules! unregistered_base {
    ($base:expr) => {
        format!("Unregistered base type: {}", $base)
    };
}

macro_rules! cannot_claim_access {
    ($base:expr, $location:expr) => {
        format!(
            "Cannot claim access to base type: {}. The base is already claimed by something else in a way which prevents safe access. Location: {}",
            $base, $location
        )
    };
}

macro_rules! impossible_conversion {
    ($into:expr) => {
        format!("Cannot convert to type: {}", $into)
    };
}

macro_rules! type_mismatch {
    ($expected:expr, $got:expr) => {
        format!("Type mismatch, expected: {}, got: {}", $expected, $got)
    };
}

macro_rules! string_type_mismatch {
    ($expected:expr, $got:expr) => {
        format!("Type mismatch, expected: {}, got: {}", $expected, $got)
    };
}

macro_rules! could_not_downcast {
    ($from:expr, $to:expr) => {
        format!("Could not downcast from: {} to: {}", $from, $to)
    };
}

macro_rules! garbage_collected_allocation {
    ($reference:expr) => {
        format!(
            "Allocation was garbage collected. Could not access reference: {} as a result.",
            $reference
        )
    };
}

macro_rules! reflection_path_error {
    ($error:expr, $reference:expr) => {
        format!(
            "Error while reflecting path: {} on reference: {}",
            $error, $reference
        )
    };
}

macro_rules! missing_type_data {
    ($type_data:expr, $type_id:expr) => {
        format!(
            "Missing type data {} for type: {}. Did you register the type correctly?",
            $type_data, $type_id
        )
    };
}

macro_rules! failed_from_reflect {
    ($type_id:expr, $reason:expr) => {
        format!(
            "Failed to convert from reflect for type: {} with reason: {}",
            $type_id, $reason
        )
    };
}

macro_rules! value_mismatch {
    ($expected:expr, $got:expr) => {
        format!("Value mismatch, expected: {}, got: {}", $expected, $got)
    };
}

macro_rules! unsupported_operation {
    ($operation:expr, $base:expr, $value:expr) => {
        format!(
            "Unsupported operation: {} on base: {} with value: {:?}",
            $operation, $base, $value
        )
    };
}

macro_rules! invalid_index {
    ($value:expr, $reason:expr) => {
        format!("Invalid index for value: {}: {}", $value, $reason)
    };
}

macro_rules! missing_entity {
    ($entity:expr) => {
        format!("Missing or invalid entity: {}", $entity)
    };
}

macro_rules! invalid_component {
    ($component_id:expr) => {
        format!("Invalid component: {:?}", $component_id)
    };
}

macro_rules! function_interop_error {
    ($display_name:expr, $opt_on:expr, $error:expr) => {
        format!(
            "Error in function {} {}: {}",
            $display_name, $opt_on, $error
        )
    };
}

macro_rules! function_arg_conversion_error {
    ($argument:expr, $error:expr) => {
        format!("Error converting argument {}: {}", $argument, $error)
    };
}

macro_rules! function_call_error {
    ($inner:expr) => {
        format!("Error in function call: {}", $inner)
    };
}

macro_rules! better_conversion_exists {
    ($context:expr) => {
        format!("Unfinished conversion in context of: {}. A better conversion exists but caller didn't handle the case.", $context)
    };
}

macro_rules! length_mismatch {
    ($expected:expr, $got:expr) => {
        format!(
            "Array/List Length mismatch, expected: {}, got: {}",
            $expected, $got
        )
    };
}

macro_rules! invalid_access_count {
    ($expected:expr, $count:expr, $context:expr) => {
        format!(
            "Invalid access count, expected: {}, got: {}. {}",
            $expected, $count, $context
        )
    };
}

impl DisplayWithWorld for InteropErrorInner {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        match self {
            InteropErrorInner::MissingFunctionError { on, function_name } => {
                missing_function_error!(function_name, on.display_with_world(world))
            },
            InteropErrorInner::UnregisteredBase { base } => {
                unregistered_base!(base.display_with_world(world))
            }
            InteropErrorInner::CannotClaimAccess { base, location } => {
                cannot_claim_access!(base.display_with_world(world), location.display_location())
            }
            InteropErrorInner::ImpossibleConversion { into } => {
                impossible_conversion!(into.display_with_world(world))
            }
            InteropErrorInner::TypeMismatch { expected, got } => {
                type_mismatch!(
                    expected.display_with_world(world.clone()),
                    got.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::StringTypeMismatch { expected, got } => {
                string_type_mismatch!(
                    expected,
                    got.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::CouldNotDowncast { from, to } => {
                could_not_downcast!(
                    from.display_with_world(world.clone()),
                    to.display_with_world(world)
                )
            }
            InteropErrorInner::GarbageCollectedAllocation { reference } => {
                garbage_collected_allocation!(reference.display_with_world(world))
            }
            InteropErrorInner::ReflectionPathError { error, reference } => {
                reflection_path_error!(
                    error,
                    reference
                        .as_ref()
                        .map(|r| r.display_with_world(world))
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::MissingTypeData { type_id, type_data } => {
                missing_type_data!(type_data, type_id.display_with_world(world))
            }
            InteropErrorInner::FailedFromReflect { type_id, reason } => {
                failed_from_reflect!(
                    type_id
                        .map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned()),
                    reason
                )
            }
            InteropErrorInner::ValueMismatch { expected, got } => {
                value_mismatch!(
                    expected.display_with_world(world.clone()),
                    got.display_with_world(world)
                )
            }
            InteropErrorInner::UnsupportedOperation {
                base,
                value,
                operation,
            } => {
                unsupported_operation!(
                    operation,
                    base.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned()),
                    value
                )
            }
            InteropErrorInner::InvalidIndex { value, reason } => {
                invalid_index!(value.display_with_world(world), reason)
            }
            InteropErrorInner::MissingEntity { entity } => {
                missing_entity!(entity)
            }
            InteropErrorInner::InvalidComponent { component_id } => {
                invalid_component!(component_id)
            }
            InteropErrorInner::StaleWorldAccess => {
                "Stale world access. The world has been dropped and a script tried to access it. Do not try to store or copy the world."
                    .to_owned()
            }
            InteropErrorInner::MissingWorld => {
                "Missing world. The world was not initialized in the script context.".to_owned()
            },
            InteropErrorInner::FunctionInteropError { function_name, on, error } => {
                let opt_on = match on {
                    Namespace::Global => "".to_owned(),
                    Namespace::OnType(type_id) => format!("on type: {}", type_id.display_with_world(world.clone())),
                };
                let display_name = if function_name.starts_with("TypeId") {
                    function_name.split("::").last().unwrap()
                } else {
                    function_name.as_str()
                };
                function_interop_error!(display_name, opt_on, error.display_with_world(world))
            },
            InteropErrorInner::FunctionArgConversionError { argument, error } => {
                function_arg_conversion_error!(argument, error.display_with_world(world))
            },
            InteropErrorInner::FunctionCallError { inner } => {
                function_call_error!(inner)
            },
            InteropErrorInner::BetterConversionExists{ context } => {
                better_conversion_exists!(context)
            },
            InteropErrorInner::OtherError { error } => error.to_string(),
            InteropErrorInner::LengthMismatch { expected, got } => {
                length_mismatch!(expected, got)
            },
            InteropErrorInner::InvalidAccessCount { count, expected, context } => {
                invalid_access_count!(expected, count, context)
            },
        }
    }

    // todo macro this, or use format strings to reduce duplication
    fn display_without_world(&self) -> String {
        match self {
            InteropErrorInner::MissingFunctionError { on, function_name } => {
                missing_function_error!(function_name, on.display_without_world())
            },
            InteropErrorInner::UnregisteredBase { base } => {
                unregistered_base!(base.display_without_world())
            }
            InteropErrorInner::CannotClaimAccess { base, location } => {
                cannot_claim_access!(base.display_without_world(), location.display_location())
            }
            InteropErrorInner::ImpossibleConversion { into } => {
                impossible_conversion!(into.display_without_world())
            }
            InteropErrorInner::TypeMismatch { expected, got } => {
                type_mismatch!(
                    expected.display_without_world(),
                    got.map(|t| t.display_without_world())
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::StringTypeMismatch { expected, got } => {
                string_type_mismatch!(
                    expected,
                    got.map(|t| t.display_without_world())
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::CouldNotDowncast { from, to } => {
                could_not_downcast!(
                    from.display_without_world(),
                    to.display_without_world()
                )
            }
            InteropErrorInner::GarbageCollectedAllocation { reference } => {
                garbage_collected_allocation!(reference.display_without_world())
            }
            InteropErrorInner::ReflectionPathError { error, reference } => {
                reflection_path_error!(
                    error,
                    reference
                        .as_ref()
                        .map(|r| r.display_without_world())
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::MissingTypeData { type_id, type_data } => {
                missing_type_data!(type_data, type_id.display_without_world())
            }
            InteropErrorInner::FailedFromReflect { type_id, reason } => {
                failed_from_reflect!(
                    type_id
                        .map(|t| t.display_without_world())
                        .unwrap_or("None".to_owned()),
                    reason
                )
            }
            InteropErrorInner::ValueMismatch { expected, got } => {
                value_mismatch!(
                    expected.display_without_world(),
                    got.display_without_world()
                )
            }
            InteropErrorInner::UnsupportedOperation {
                base,
                value,
                operation,
            } => {
                unsupported_operation!(
                    operation,
                    base.map(|t| t.display_without_world())
                        .unwrap_or("None".to_owned()),
                    value
                )
            }
            InteropErrorInner::InvalidIndex { value, reason } => {
                invalid_index!(value.display_without_world(), reason)
            }
            InteropErrorInner::MissingEntity { entity } => {
                missing_entity!(entity)
            }
            InteropErrorInner::InvalidComponent { component_id } => {
                invalid_component!(component_id)
            }
            InteropErrorInner::StaleWorldAccess => {
                "Stale world access. The world has been dropped and a script tried to access it. Do not try to store or copy the world."
                    .to_owned()
            }
            InteropErrorInner::MissingWorld => {
                "Missing world. The world was not initialized in the script context.".to_owned()
            },
            InteropErrorInner::FunctionInteropError { function_name, on, error } => {
                let opt_on = match on {
                    Namespace::Global => "".to_owned(),
                    Namespace::OnType(type_id) => format!("on type: {}", type_id.display_without_world()),
                };
                let display_name = if function_name.starts_with("TypeId") {
                    function_name.split("::").last().unwrap()
                } else {
                    function_name.as_str()
                };
                function_interop_error!(display_name, opt_on, error.display_without_world())
            },
            InteropErrorInner::FunctionArgConversionError { argument, error } => {
                function_arg_conversion_error!(argument, error.display_without_world())
            },
            InteropErrorInner::FunctionCallError { inner } => {
                function_call_error!(inner)
            },
            InteropErrorInner::BetterConversionExists{ context } => {
                better_conversion_exists!(context)
            },
            InteropErrorInner::OtherError { error } => error.to_string(),
            InteropErrorInner::LengthMismatch { expected, got } => {
                length_mismatch!(expected, got)
            },
            InteropErrorInner::InvalidAccessCount { count, expected, context } => {
                invalid_access_count!(expected, count, context)
            },
        }
    }
}

/// Purely for purposes of the automatic [`GetTypeRegistration`] impl.
impl Default for InteropErrorInner {
    fn default() -> Self {
        InteropErrorInner::StaleWorldAccess
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::{AppTypeRegistry, World};

    use crate::bindings::{
        function::script_function::AppScriptFunctionRegistry, AppReflectAllocator,
        WorldAccessGuard, WorldGuard,
    };

    use super::*;

    #[test]
    fn test_error_display() {
        let error =
            InteropError::failed_from_reflect(Some(TypeId::of::<String>()), "reason".to_owned());
        let mut world = World::default();
        let type_registry = AppTypeRegistry::default();
        world.insert_resource(type_registry);

        let script_allocator = AppReflectAllocator::default();
        world.insert_resource(script_allocator);

        let script_function_registry = AppScriptFunctionRegistry::default();
        world.insert_resource(script_function_registry);

        let world_guard = WorldGuard::new(WorldAccessGuard::new(&mut world));
        assert_eq!(
            error.display_with_world(world_guard),
            format!(
                "Failed to convert from reflect for type: {} with reason: reason",
                std::any::type_name::<String>()
            )
        );

        assert_eq!(
            error.display_without_world(),
            format!(
                "Failed to convert from reflect for type: {:?} with reason: reason",
                TypeId::of::<String>()
            )
        );
    }
}
