//! Errors that can occur when interacting with the scripting system

use std::{
    any::TypeId,
    borrow::Cow,
    fmt::{Debug, Display},
    ops::Deref,
    str::Utf8Error,
    sync::Arc,
};

use bevy_ecs::entity::Entity;

use ::{
    bevy_asset::{AssetPath, Handle},
    bevy_ecs::{
        component::ComponentId,
        schedule::{ScheduleBuildError, ScheduleNotInitialized},
    },
    bevy_reflect::{PartialReflect, Reflect},
};

use crate::{
    ScriptAsset,
    bindings::{
        ReflectBaseType, ReflectReference,
        access_map::{DisplayCodeLocation, ReflectAccessId},
        function::namespace::Namespace,
        pretty_print::DisplayWithWorld,
        script_value::ScriptValue,
    },
    script::{ContextKey, DisplayProxy},
};

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
    /// The script that caused the error
    pub script: Option<String>,
    /// The context in which the error occurred
    pub context: String,
    /// The error that occurred
    pub reason: Arc<ErrorKind>,
}

#[derive(Debug)]
/// The kind of error that occurred
pub enum ErrorKind {
    /// An error that can be displayed
    Display(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// An error that can be displayed with a world
    WithWorld(Box<dyn DisplayWithWorld + Send + Sync + 'static>),
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
    /// Tried to downcast a script error to an interop error
    pub fn downcast_interop_inner(&self) -> Option<&InteropErrorInner> {
        match self.reason.as_ref() {
            ErrorKind::WithWorld(display_with_world) => {
                let any: &dyn DisplayWithWorld = display_with_world.as_ref();
                if let Some(interop_error) = any.downcast_ref::<InteropError>() {
                    Some(interop_error.inner())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

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

    #[cfg(feature = "rhai_impls")]
    /// destructures a rhai error into a script error, taking care to preserve as much information as possible
    pub fn from_rhai_error(error: rhai::EvalAltResult) -> Self {
        match error {
            rhai::EvalAltResult::ErrorSystem(message, error) => {
                if let Some(inner) = error.downcast_ref::<InteropError>() {
                    Self::new(inner.clone())
                } else if let Some(inner) = error.downcast_ref::<ScriptError>() {
                    inner.clone()
                } else {
                    Self::new_external_boxed(error).with_context(message)
                }
            }
            _ => Self::new_external(error),
        }
    }

    /// Creates a new script error with an external error
    pub fn new_external(reason: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new_external_boxed(Box::new(reason))
    }

    /// Creates a new script error with an external error
    pub fn new_external_boxed(reason: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Arc::new(ErrorKind::Display(reason)),
            context: Default::default(),
        }))
    }

    /// Creates a new script error with a reason
    pub fn new(reason: impl DisplayWithWorld + Send + Sync + 'static) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Arc::new(ErrorKind::WithWorld(Box::new(reason))),
            context: Default::default(),
        }))
    }

    /// Creates a new script error with a reason
    pub fn with_script<S: ToString>(self, script: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: Some(script.to_string()),
            context: self.0.context.clone(),
            reason: self.0.reason.clone(),
        }))
    }

    /// Adds context to the error
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

impl From<ScheduleBuildError> for InteropError {
    fn from(value: ScheduleBuildError) -> Self {
        InteropError::external_error(Box::new(value))
    }
}

impl From<ScheduleNotInitialized> for InteropError {
    fn from(value: ScheduleNotInitialized) -> Self {
        InteropError::external_error(Box::new(value))
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

#[cfg(feature = "rhai_impls")]
impl From<rhai::ParseError> for ScriptError {
    fn from(value: rhai::ParseError) -> Self {
        ScriptError::new_external(value)
    }
}

#[cfg(feature = "rhai_impls")]
impl From<Box<rhai::EvalAltResult>> for ScriptError {
    fn from(value: Box<rhai::EvalAltResult>) -> Self {
        ScriptError::from_rhai_error(*value)
    }
}

#[cfg(feature = "rhai_impls")]
impl From<ScriptError> for Box<rhai::EvalAltResult> {
    fn from(value: ScriptError) -> Self {
        Box::new(rhai::EvalAltResult::ErrorSystem(
            "ScriptError".to_owned(),
            Box::new(value),
        ))
    }
}

#[cfg(feature = "rhai_impls")]
impl From<InteropError> for Box<rhai::EvalAltResult> {
    fn from(value: InteropError) -> Self {
        Box::new(rhai::EvalAltResult::ErrorSystem(
            "InteropError".to_owned(),
            Box::new(value),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
/// An error thrown when a resource is missing
pub struct MissingResourceError(&'static str);

impl MissingResourceError {
    /// Creates a new missing resource error
    pub fn new<R>() -> Self {
        Self(std::any::type_name::<R>())
    }
}

impl Display for MissingResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Missing resource: {}. Was the plugin initialized correctly?",
            self.0
        )
    }
}

impl std::error::Error for MissingResourceError {}

#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(opaque)]
/// An error thrown when interoperating with scripting languages.
pub struct InteropError(Arc<InteropErrorInner>);

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

/// Utility trait for flattening errors
pub trait FlattenError<O, E> {
    /// Flattens the error into a single error type
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
    /// Creates a new invariant error. Thrown when an invariant is violated.
    pub fn invariant(message: impl Display) -> Self {
        Self(Arc::new(InteropErrorInner::Invariant {
            message: message.to_string(),
        }))
    }

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
    pub fn failed_from_reflect(type_id: Option<TypeId>, reason: impl Into<String>) -> Self {
        Self(Arc::new(InteropErrorInner::FailedFromReflect {
            type_id,
            reason: reason.into(),
        }))
    }

    /// Thrown if access to the given reflection base is required but cannot be claimed.
    /// This is likely due to some other script already claiming access to the base.
    pub fn cannot_claim_access(
        base: ReflectAccessId,
        location: Option<std::panic::Location<'static>>,
        context: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::CannotClaimAccess {
            base,
            location,
            context: context.into(),
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
        operation: impl Display,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::UnsupportedOperation {
            base,
            value,
            operation: operation.to_string(),
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

    /// Thrown when an error happens during argument conversion in a function call
    pub fn function_arg_conversion_error(argument: String, error: InteropError) -> Self {
        Self(Arc::new(InteropErrorInner::FunctionArgConversionError {
            argument,
            error,
        }))
    }

    /// Thrown when a length mismatch occurs
    pub fn length_mismatch(expected: usize, got: usize) -> Self {
        Self(Arc::new(InteropErrorInner::LengthMismatch {
            expected,
            got,
        }))
    }

    /// Thrown when an error happens that is not covered by the other variants
    pub fn external_error(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self(Arc::new(InteropErrorInner::OtherError { error }))
    }

    /// Thrown when a function is missing from the function registry
    pub fn missing_function(on: TypeId, function_name: impl Display) -> Self {
        Self(Arc::new(InteropErrorInner::MissingFunctionError {
            on,
            function_name: function_name.to_string(),
        }))
    }

    /// Thrown when an invalid access count is detected
    pub fn invalid_access_count(count: usize, expected: usize, context: impl Display) -> Self {
        Self(Arc::new(InteropErrorInner::InvalidAccessCount {
            count,
            expected,
            context: context.to_string(),
        }))
    }

    /// Thrown when a component or resource type is not registered
    pub fn unregistered_component_or_resource_type(
        type_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self(Arc::new(
            InteropErrorInner::UnregisteredComponentOrResourceType {
                type_name: type_name.into(),
            },
        ))
    }

    /// Thrown when constructing types and we find missing data needed to construct the type
    pub fn missing_data_in_constructor(
        type_id: TypeId,
        missing_data_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self(Arc::new(InteropErrorInner::MissingDataInConstructor {
            type_id,
            missing_data_name: missing_data_name.into(),
        }))
    }

    /// Thrown if an enum variant is invalid.
    pub fn invalid_enum_variant(type_id: TypeId, variant_name: impl ToString) -> Self {
        Self(Arc::new(InteropErrorInner::InvalidEnumVariant {
            type_id,
            variant_name: variant_name.to_string(),
        }))
    }

    /// Thrown when the number of arguments in a function call does not match.
    pub fn argument_count_mismatch(expected: usize, got: usize) -> Self {
        Self(Arc::new(InteropErrorInner::ArgumentCountMismatch {
            expected,
            got,
        }))
    }

    /// Thrown if a script could not be found when trying to call a synchronous callback or otherwise
    pub fn missing_script(script_id: impl Into<Handle<ScriptAsset>>) -> Self {
        Self(Arc::new(InteropErrorInner::MissingScript {
            script_id: Some(script_id.into()),
            script_path: None,
        }))
    }

    /// Thrown if a script could not be found when trying to call a synchronous callback or otherwise
    pub fn missing_script_by_path<'a>(script_id: impl Into<AssetPath<'a>>) -> Self {
        Self(Arc::new(InteropErrorInner::MissingScript {
            script_path: Some(script_id.into().to_string()),
            script_id: None,
        }))
    }

    /// Thrown if the required context for an operation is missing.
    pub fn missing_context(context_key: impl Into<ContextKey>) -> Self {
        Self(Arc::new(InteropErrorInner::MissingContext {
            context_key: context_key.into(),
        }))
    }

    /// Thrown when a schedule is missing from the registry.
    pub fn missing_schedule(schedule_name: impl Into<Cow<'static, str>>) -> Self {
        Self(Arc::new(InteropErrorInner::MissingSchedule {
            schedule_name: schedule_name.into(),
        }))
    }

    /// Returns the inner error
    pub fn inner(&self) -> &InteropErrorInner {
        &self.0
    }
}

/// For errors to do with reflection, type conversions or other interop issues
#[derive(Debug)]
pub enum InteropErrorInner {
    /// Thrown if a callback requires world access, but is unable to do so due
    StaleWorldAccess,
    /// Thrown if a callback requires world access, but is unable to do so due
    MissingWorld,
    /// Thrown if a script could not be found when trying to call a synchronous callback.
    /// The path or id is used depending on which stage the script was in when the error occurred.
    MissingScript {
        /// The script path that was not found.
        script_path: Option<String>,
        /// The script id that was not found.
        script_id: Option<Handle<ScriptAsset>>,
    },
    /// Thrown if a base type is not registered with the reflection system
    UnregisteredBase {
        /// The base type that was not registered
        base: ReflectBaseType,
    },
    /// Thrown if a base type is not registered with the reflection system
    MissingTypeData {
        /// The type that was missing data
        type_id: TypeId,
        /// The type data that was missing
        type_data: String,
    },
    /// Thrown if a type cannot be converted from reflect
    FailedFromReflect {
        /// The type that failed to convert
        type_id: Option<TypeId>,
        /// The reason for the failure
        reason: String,
    },
    /// Thrown if access to the given reflection base is required but cannot be claimed
    CannotClaimAccess {
        /// The base that could not be claimed
        base: ReflectAccessId,
        /// The context in which the error occurred
        context: Cow<'static, str>,
        /// The location in the code where the blocking access is being held
        location: Option<std::panic::Location<'static>>,
    },
    /// thrown when the access count is invalid
    InvalidAccessCount {
        /// The count of accesses
        count: usize,
        /// The expected count
        expected: usize,
        /// The context in which the error occurred
        context: String,
    },
    /// Thrown if a conversion into the given type is impossible
    ImpossibleConversion {
        /// The type that the conversion was attempted into
        into: TypeId,
    },
    /// Thrown if a conversion was not fully completed, as a better conversion exists
    BetterConversionExists {
        /// The context in which the error occurred
        context: String,
    },
    /// Thrown if a value was expected to be of one type but was of another
    TypeMismatch {
        /// The type that was expected
        expected: TypeId,
        /// The type that was received
        got: Option<TypeId>,
    },
    /// Thrown if a value was expected to be of one type but was of another
    StringTypeMismatch {
        /// The type that was expected
        expected: String,
        /// The type that was received
        got: Option<TypeId>,
    },
    /// Thrown if a [`ScriptValue`] could not be converted to the expected type
    ValueMismatch {
        /// The type that was expected
        expected: TypeId,
        /// The value that was received
        got: ScriptValue,
    },
    /// Thrown if a length mismatch occurs
    LengthMismatch {
        /// The length that was expected
        expected: usize,
        /// The length that was received
        got: usize,
    },
    /// Thrown if a downcast from a reflect reference to a specific type failed
    CouldNotDowncast {
        /// The reference that was attempted to be downcast
        from: ReflectReference,
        /// The type that the downcast was attempted to
        to: TypeId,
    },
    /// Thrown if a garbage collected allocation was attempted to be accessed
    GarbageCollectedAllocation {
        /// The reference that was attempted to be accessed
        reference: ReflectReference,
    },
    /// Thrown if a reflection path is invalid
    ReflectionPathError {
        /// The error that occurred
        error: String,
        /// The reference that was attempted to be accessed
        reference: Option<ReflectReference>,
    },
    /// Thrown if an operation is not supported on the given base type
    UnsupportedOperation {
        /// The base that the operation was attempted on
        base: Option<TypeId>,
        /// The value that was used in the operation
        value: Option<Box<dyn PartialReflect>>,
        /// The operation that was attempted
        operation: String,
    },
    /// Thrown if an invalid index operation was attempted on a value
    InvalidIndex {
        /// The value that was attempted to be indexed
        value: ScriptValue,
        /// The reason for the invalid index
        reason: String,
    },
    /// Thrown if an entity was missing or invalid
    MissingEntity {
        /// The entity that was missing
        entity: Entity,
    },
    /// Thrown if a component was invalid
    InvalidComponent {
        /// The component that was invalid
        component_id: ComponentId,
    },
    /// Thrown when an error happens during argument conversion in a function call
    MissingFunctionError {
        /// The type that the function was attempted to be called on
        on: TypeId,
        /// The function that was attempted to be called
        function_name: String,
    },
    /// Thrown when an error happens in the context of a function call
    FunctionInteropError {
        /// The function that the error occurred in
        function_name: String,
        /// The namespace that the function was called on
        on: Namespace,
        /// The error that occurred
        error: InteropError,
    },
    /// Thrown when an error happens that is not covered by the other variants
    FunctionArgConversionError {
        /// The argument that was attempted to be converted
        argument: String,
        /// The error that occurred
        error: InteropError,
    },
    /// Thrown when an error happens that is not covered by the other variants
    OtherError {
        /// The error that occurred
        error: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Thrown when a component or resource type is not registered
    UnregisteredComponentOrResourceType {
        /// The type that was not registered
        type_name: Cow<'static, str>,
    },
    /// Thrown when constructing types and we find missing data
    MissingDataInConstructor {
        /// The type id of the type we're constructing
        type_id: TypeId,
        /// the name of the missing data
        missing_data_name: Cow<'static, str>,
    },
    /// Thrown when an invariant is violated
    Invariant {
        /// The message that describes the invariant violation
        message: String,
    },
    /// New variant for invalid enum variant errors.
    InvalidEnumVariant {
        /// the enum type id
        type_id: TypeId,
        /// the variant
        variant_name: String,
    },
    /// Thrown when the number of arguments in a function call does not match.
    ArgumentCountMismatch {
        /// The number of arguments that were expected
        expected: usize,
        /// The number of arguments that were received
        got: usize,
    },
    /// Thrown if the required context for an operation is missing.
    MissingContext {
        /// The script that was attempting to access the context
        context_key: ContextKey,
    },
    /// Thrown when a schedule is missing from the registry.
    MissingSchedule {
        /// The name of the schedule that was missing
        schedule_name: Cow<'static, str>,
    },
}

/// For test purposes
impl PartialEq for InteropErrorInner {
    fn eq(&self, _other: &Self) -> bool {
        match (self, _other) {
            (
                InteropErrorInner::MissingScript {
                    script_id: a,
                    script_path: b,
                },
                InteropErrorInner::MissingScript {
                    script_id: c,
                    script_path: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::InvalidAccessCount {
                    count: a,
                    expected: b,
                    context: c,
                },
                InteropErrorInner::InvalidAccessCount {
                    count: d,
                    expected: e,
                    context: f,
                },
            ) => a == d && b == e && c == f,
            (InteropErrorInner::StaleWorldAccess, InteropErrorInner::StaleWorldAccess) => true,
            (InteropErrorInner::MissingWorld, InteropErrorInner::MissingWorld) => true,
            (
                InteropErrorInner::UnregisteredBase { base: a },
                InteropErrorInner::UnregisteredBase { base: b },
            ) => a == b,
            (
                InteropErrorInner::MissingTypeData {
                    type_id: a,
                    type_data: b,
                },
                InteropErrorInner::MissingTypeData {
                    type_id: c,
                    type_data: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::FailedFromReflect {
                    type_id: a,
                    reason: b,
                },
                InteropErrorInner::FailedFromReflect {
                    type_id: c,
                    reason: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::CannotClaimAccess {
                    base: a,
                    context: b,
                    location: c,
                },
                InteropErrorInner::CannotClaimAccess {
                    base: d,
                    context: e,
                    location: f,
                },
            ) => a == d && b == e && c == f,
            (
                InteropErrorInner::ImpossibleConversion { into: a },
                InteropErrorInner::ImpossibleConversion { into: b },
            ) => a == b,
            (
                InteropErrorInner::BetterConversionExists { context: a },
                InteropErrorInner::BetterConversionExists { context: b },
            ) => a == b,
            (
                InteropErrorInner::TypeMismatch {
                    expected: a,
                    got: b,
                },
                InteropErrorInner::TypeMismatch {
                    expected: c,
                    got: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::StringTypeMismatch {
                    expected: a,
                    got: b,
                },
                InteropErrorInner::StringTypeMismatch {
                    expected: c,
                    got: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::ValueMismatch {
                    expected: a,
                    got: b,
                },
                InteropErrorInner::ValueMismatch {
                    expected: c,
                    got: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::LengthMismatch {
                    expected: a,
                    got: b,
                },
                InteropErrorInner::LengthMismatch {
                    expected: c,
                    got: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::CouldNotDowncast { from: a, to: b },
                InteropErrorInner::CouldNotDowncast { from: c, to: d },
            ) => a == c && b == d,
            (
                InteropErrorInner::GarbageCollectedAllocation { reference: a },
                InteropErrorInner::GarbageCollectedAllocation { reference: b },
            ) => a == b,
            (
                InteropErrorInner::ReflectionPathError {
                    error: a,
                    reference: b,
                },
                InteropErrorInner::ReflectionPathError {
                    error: c,
                    reference: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::UnsupportedOperation {
                    base: a,
                    value: _b,
                    operation: c,
                },
                InteropErrorInner::UnsupportedOperation {
                    base: d,
                    value: _e,
                    operation: f,
                },
            ) => a == d && c == f,
            (
                InteropErrorInner::InvalidIndex {
                    value: a,
                    reason: b,
                },
                InteropErrorInner::InvalidIndex {
                    value: c,
                    reason: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::MissingEntity { entity: a },
                InteropErrorInner::MissingEntity { entity: b },
            ) => a == b,
            (
                InteropErrorInner::InvalidComponent { component_id: a },
                InteropErrorInner::InvalidComponent { component_id: b },
            ) => a == b,
            (
                InteropErrorInner::MissingFunctionError {
                    on: a,
                    function_name: _b,
                },
                InteropErrorInner::MissingFunctionError {
                    on: c,
                    function_name: _d,
                },
            ) => a == c,
            (
                InteropErrorInner::FunctionInteropError {
                    function_name: a,
                    on: b,
                    error: c,
                },
                InteropErrorInner::FunctionInteropError {
                    function_name: d,
                    on: e,
                    error: f,
                },
            ) => a == d && b == e && c == f,
            (
                InteropErrorInner::FunctionArgConversionError {
                    argument: a,
                    error: b,
                },
                InteropErrorInner::FunctionArgConversionError {
                    argument: c,
                    error: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::OtherError { error: a },
                InteropErrorInner::OtherError { error: b },
            ) => a.to_string() == b.to_string(),
            (
                InteropErrorInner::UnregisteredComponentOrResourceType { type_name: a },
                InteropErrorInner::UnregisteredComponentOrResourceType { type_name: b },
            ) => a == b,
            (
                InteropErrorInner::Invariant { message: a },
                InteropErrorInner::Invariant { message: b },
            ) => a == b,
            (
                InteropErrorInner::MissingDataInConstructor {
                    type_id: a,
                    missing_data_name: b,
                },
                InteropErrorInner::MissingDataInConstructor {
                    type_id: c,
                    missing_data_name: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::InvalidEnumVariant {
                    type_id: a,
                    variant_name: b,
                },
                InteropErrorInner::InvalidEnumVariant {
                    type_id: c,
                    variant_name: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::ArgumentCountMismatch {
                    expected: a,
                    got: b,
                },
                InteropErrorInner::ArgumentCountMismatch {
                    expected: c,
                    got: d,
                },
            ) => a == c && b == d,
            (
                InteropErrorInner::MissingContext { context_key: b },
                InteropErrorInner::MissingContext { context_key: d },
            ) => b == d,
            (
                InteropErrorInner::MissingSchedule { schedule_name: a },
                InteropErrorInner::MissingSchedule { schedule_name: b },
            ) => a == b,
            _ => false,
        }
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
    ($base:expr, $location:expr, $ctxt:expr) => {
        format!(
            "Cannot claim access to base type: {}. The base is already claimed by something else in a way which prevents safe access. Location: {}. Context: {}",
            $base, $location, $ctxt
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
        {
            if ($entity.index() == 0) {
                format!("Invalid entity: {}. Are you trying to use an entity in a callback in which it's unavailable?", $entity)
            } else {
                format!("Missing or invalid entity: {}", $entity)
            }
        }
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

macro_rules! missing_data_in_constructor {
    ($type_id:expr, $missing_data_name:expr) => {
        format!(
            "Missing data in constructor for type: {}. Missing data: {}",
            $type_id, $missing_data_name
        )
    };
}

macro_rules! invariant {
    ($message:expr) => {
        format!(
            "An invariant has been broken. This is a bug in BMS, please report me! : {}",
            $message
        )
    };
}

macro_rules! unregistered_component_or_resource_type {
    ($type_name:expr) => {
        format!(
            "Expected registered component/resource but got unregistered type: {}",
            $type_name
        )
    };
}

macro_rules! missing_script_for_callback {
    ($script_id:expr, $script_path:expr) => {
        format!(
            "Could not find script {}. Is the script loaded?",
            $script_id.map_or_else(
                || $script_path.unwrap_or_default(),
                |id| id.display().to_string()
            )
        )
    };
}

// Define a single macro for the invalid enum variant error.
macro_rules! invalid_enum_variant_msg {
    ($variant:expr, $enum_display:expr) => {
        format!(
            "Invalid enum variant: {} for enum: {}",
            $variant, $enum_display
        )
    };
}

// Define a single macro for the argument count mismatch error.
macro_rules! argument_count_mismatch_msg {
    ($expected:expr, $got:expr) => {
        format!(
            "Argument count mismatch, expected: {}, got: {}",
            $expected, $got
        )
    };
}

macro_rules! missing_context_for_callback {
    ($context_key:expr) => {
        format!(
            "Missing context for {}. Was the script loaded?.",
            $context_key
        )
    };
}

macro_rules! missing_schedule_error {
    ($schedule:expr) => {
        format!("Missing schedule: '{}'. This can happen if you try to access a schedule from within itself. Have all schedules been registered?", $schedule)
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
            InteropErrorInner::CannotClaimAccess { base, location, context } => {
                cannot_claim_access!(base.display_with_world(world), location.display_location(), context)
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
                "Missing world. The world was either not initialized, or invalidated.".to_owned()
            },
            InteropErrorInner::FunctionInteropError { function_name, on, error } => {
                let opt_on = match on {
                    Namespace::Global => "".to_owned(),
                    Namespace::OnType(type_id) => format!("on type: {}", type_id.display_with_world(world.clone())),
                };
                let display_name = if function_name.starts_with("TypeId") {
                    function_name.split("::").last().unwrap_or_default()
                } else {
                    function_name.as_str()
                };
                function_interop_error!(display_name, opt_on, error.display_with_world(world))
            },
            InteropErrorInner::FunctionArgConversionError { argument, error } => {
                function_arg_conversion_error!(argument, error.display_with_world(world))
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
            InteropErrorInner::Invariant { message } => {
                invariant!(message)
            },
            InteropErrorInner::UnregisteredComponentOrResourceType { type_name } => {
                unregistered_component_or_resource_type!(type_name)
            },
            InteropErrorInner::MissingDataInConstructor { type_id, missing_data_name } => {
                missing_data_in_constructor!(type_id.display_with_world(world), missing_data_name)
            },
            InteropErrorInner::InvalidEnumVariant { type_id, variant_name } => {
                invalid_enum_variant_msg!(variant_name, type_id.display_with_world(world))
            },
            InteropErrorInner::ArgumentCountMismatch { expected, got } => {
                argument_count_mismatch_msg!(expected, got)
            },
            InteropErrorInner::MissingScript { script_id, script_path } => {
                missing_script_for_callback!(script_id.clone(), script_path.clone())
            },
            InteropErrorInner::MissingContext { context_key } => {
                missing_context_for_callback!(
                    context_key
                )
            },
            InteropErrorInner::MissingSchedule { schedule_name } => {
                missing_schedule_error!(schedule_name)
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
            InteropErrorInner::CannotClaimAccess { base, location, context } => {
                cannot_claim_access!(base.display_without_world(), location.display_location(), context)
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
                "Missing world. The world was either not initialized, or invalidated.".to_owned()
            },
            InteropErrorInner::FunctionInteropError { function_name, on, error } => {
                let opt_on = match on {
                    Namespace::Global => "".to_owned(),
                    Namespace::OnType(type_id) => format!("on type: {}", type_id.display_without_world()),
                };
                let display_name = if function_name.starts_with("TypeId") {
                    function_name.split("::").last().unwrap_or_default()
                } else {
                    function_name.as_str()
                };
                function_interop_error!(display_name, opt_on, error.display_without_world())
            },
            InteropErrorInner::FunctionArgConversionError { argument, error } => {
                function_arg_conversion_error!(argument, error.display_without_world())
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
            InteropErrorInner::Invariant { message } => {
                invariant!(message)
            },
            InteropErrorInner::UnregisteredComponentOrResourceType { type_name } => {
                unregistered_component_or_resource_type!(type_name)
            },
            InteropErrorInner::MissingDataInConstructor { type_id, missing_data_name } => {
                missing_data_in_constructor!(type_id.display_without_world(), missing_data_name)
            },
            InteropErrorInner::InvalidEnumVariant { type_id, variant_name } => {
                invalid_enum_variant_msg!(variant_name, type_id.display_without_world())
            },
            InteropErrorInner::ArgumentCountMismatch { expected, got } => {
                argument_count_mismatch_msg!(expected, got)
            },
            InteropErrorInner::MissingScript { script_id, script_path } => {
                missing_script_for_callback!(script_id.clone(), script_path.clone())
            },
            InteropErrorInner::MissingContext { context_key } => {
                missing_context_for_callback!(
                    context_key
                )
            },
            InteropErrorInner::MissingSchedule { schedule_name } => {
                missing_schedule_error!(schedule_name)
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

    use bevy_ecs::{reflect::AppTypeRegistry, world::World};

    use super::*;
    use crate::bindings::{
        AppReflectAllocator, WorldGuard, function::script_function::AppScriptFunctionRegistry,
    };

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

        let world_guard = WorldGuard::new_exclusive(&mut world);
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
