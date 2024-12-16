use std::{
    any::TypeId,
    borrow::Cow,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy::{
    ecs::component::ComponentId,
    prelude::Entity,
    reflect::{
        func::{args::ArgInfo, FunctionError, FunctionInfo},
        ApplyError, PartialReflect, Reflect, ReflectPathError,
    },
};
use thiserror::Error;

use crate::{
    bindings::{
        pretty_print::{DisplayWithWorld, DisplayWithWorldAndDummy},
        ReflectAllocationId, ReflectBase, ReflectBaseType, ReflectReference,
    },
    impl_dummy_display,
    prelude::ScriptValue,
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
#[derive(Debug)]
pub struct ScriptErrorInner {
    pub script: Option<String>,
    pub context: String,
    pub reason: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Display(Arc<dyn std::error::Error + Send + Sync>),
    WithWorld(Arc<dyn DisplayWithWorldAndDummy + Send + Sync>),
}

impl DisplayWithWorld for ErrorKind {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        match self {
            ErrorKind::Display(e) => e.to_string(),
            ErrorKind::WithWorld(e) => e.display_with_world(world),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Display(e) => write!(f, "{}", e),
            ErrorKind::WithWorld(e) => write!(f, "{}", e),
        }
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

    pub fn new_external(reason: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: ErrorKind::Display(Arc::new(reason)),
            context: Default::default(),
        }))
    }

    pub fn new(reason: impl DisplayWithWorldAndDummy + Send + Sync + 'static) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: ErrorKind::WithWorld(Arc::new(reason)),
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
        if let Some(script) = &self.0.script {
            write!(
                f,
                "error in script `{}`: {}.\nContext:{}",
                script, self.0.reason, self.0.context
            )
        } else {
            write!(f, "error: {}.\nContext:{}", self.0.reason, self.0.context)
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct InteropError(Arc<InteropErrorInner>);

impl std::error::Error for InteropError {}

impl DisplayWithWorld for InteropError {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        self.0.display_with_world(world)
    }
}

impl_dummy_display!(InteropError);

impl From<InteropError> for ScriptError {
    fn from(val: InteropError) -> Self {
        ScriptError::new(val)
    }
}

pub trait FlattenError<O,E> {
    fn flatten_interop_error(self) -> Result<O, E>;
}

impl <O>FlattenError<O, InteropError> for Result<Result<O,InteropError>, InteropError> {
    fn flatten_interop_error(self) -> Result<O, InteropError> {
        match self {
            Ok(Ok(o)) => Ok(o),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(e)
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
    pub fn cannot_claim_access(base: ReflectBaseType) -> Self {
        Self(Arc::new(InteropErrorInner::CannotClaimAccess { base }))
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
        Self(Arc::new(InteropErrorInner::BetterConversionExists{
            context: std::any::type_name::<T>().to_string()
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
    pub fn function_interop_error(
        function_info: &FunctionInfo,
        argument_info: Option<&ArgInfo>,
        error: InteropError,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionInteropError {
            function_name: function_info
                .name()
                .map(|s| s.to_string())
                .unwrap_or("<unnamed function>".to_owned()),
            argument: argument_info
                .map(|a| {
                    format!(
                        "{}({}) {}",
                        a.index(),
                        a.ownership(),
                        a.name().unwrap_or("<no_name>")
                    )
                })
                .unwrap_or("None".to_owned()),
            error,
        }))
    }

    /// Thrown when the error happens after a function call, and an error is thrown by bevy.
    ///
    /// I.e. mismatch in args, or invalid number of arguments
    pub fn function_call_error(inner: FunctionError) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionCallError { inner }))
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

impl_dummy_display!(InteropErrorInner);

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
    },
    ImpossibleConversion {
        into: TypeId,
    },
    BetterConversionExists {
        context: String
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
    FunctionInteropError {
        function_name: String,
        argument: String,
        error: InteropError,
    },
}

impl PartialEq for InteropErrorInner {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl DisplayWithWorld for InteropErrorInner {
    fn display_with_world(&self, world: crate::bindings::WorldGuard) -> String {
        match self {
            
            InteropErrorInner::UnregisteredBase { base } => {
                format!("Unregistered base type: {}", base.display_with_world(world))
            }
            InteropErrorInner::CannotClaimAccess { base } => {
                format!(
                    "Cannot claim access to base type: {}",
                    base.display_with_world(world)
                )
            }
            InteropErrorInner::ImpossibleConversion { into } => {
                format!("Cannot convert to type: {}", into.display_with_world(world))
            }
            InteropErrorInner::TypeMismatch { expected, got } => {
                format!(
                    "Type mismatch, expected: {}, got: {}",
                    expected.display_with_world(world.clone()),
                    got.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::StringTypeMismatch { expected, got } => {
                format!(
                    "Type mismatch, expected: {}, got: {}",
                    expected,
                    got.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned())
                )
            }
            InteropErrorInner::CouldNotDowncast { from, to } => {
                format!(
                    "Could not downcast from: {} to: {}",
                    from.display_with_world(world.clone()),
                    to.display_with_world(world)
                )
            }
            InteropErrorInner::GarbageCollectedAllocation { reference } => {
                format!(
                    "Allocation was garbage collected. Could not access reference: {} as a result.",
                    reference.display_with_world(world),
                )
            }
            InteropErrorInner::ReflectionPathError { error, reference } => {
                format!(
                    "Error while reflecting path: {} on reference: {}",
                    error,
                    reference
                        .as_ref()
                        .map(|r| r.display_with_world(world))
                        .unwrap_or("None".to_owned()),
                )
            }
            InteropErrorInner::MissingTypeData { type_id, type_data } => {
                format!(
                    "Missing type data {} for type: {}. Did you register the type correctly?",
                    type_data,
                    type_id.display_with_world(world),
                )
            }
            InteropErrorInner::FailedFromReflect { type_id, reason } => {
                format!(
                    "Failed to convert from reflect for type: {} with reason: {}",
                    type_id
                        .map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned()),
                    reason
                )
            }
            InteropErrorInner::ValueMismatch { expected, got } => {
                format!(
                    "Value mismatch, expected: {}, got: {}",
                    expected.display_with_world(world.clone()),
                    got.display_with_world(world)
                )
            }
            InteropErrorInner::UnsupportedOperation {
                base,
                value,
                operation,
            } => {
                format!(
                    "Unsupported operation: {} on base: {} with value: {:?}",
                    operation,
                    base.map(|t| t.display_with_world(world))
                        .unwrap_or("None".to_owned()),
                    value
                )
            }
            InteropErrorInner::InvalidIndex { value, reason } => {
                format!(
                    "Invalid index for value: {}: {}",
                    value.display_with_world(world),
                    reason
                )
            }
            InteropErrorInner::MissingEntity { entity } => {
                format!("Missing or invalid entity: {}", entity)
            }
            InteropErrorInner::InvalidComponent { component_id } => {
                format!("Invalid component: {:?}", component_id)
            }
            InteropErrorInner::StaleWorldAccess => {
                "Stale world access. The world has been dropped and a script tried to access it. Do not try to store or copy the world."
                    .to_owned()
            }
            InteropErrorInner::MissingWorld => {
                "Missing world. The world was not initialized in the script context.".to_owned()
            },
            InteropErrorInner::FunctionInteropError { function_name, argument, error } => {
                format!(
                    "Error in function: {} argument: {} error: {}",
                    function_name,
                    argument,
                    error.display_with_world(world)
                )
            },
            InteropErrorInner::FunctionCallError { inner } => {
                inner.to_string()
            },
            InteropErrorInner::BetterConversionExists{ context } => {
                format!("Unfinished conversion in context of: {}. A better conversion exists but caller didn't handle the case.", context)
            },
        }
    }
}
