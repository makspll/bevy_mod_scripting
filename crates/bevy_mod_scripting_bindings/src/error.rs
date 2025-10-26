//! Error types for the bindings
use crate::{
    FunctionCallContext, Namespace, ReflectBaseType, ReflectReference, access_map::ReflectAccessId,
    script_value::ScriptValue,
};
use bevy_ecs::entity::Entity;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{
    DebugWithTypeInfo, DisplayWithTypeInfo, GetTypeInfo, OrFakeId, PrintReflectAsDebug,
    WithTypeInfo,
};
use bevy_reflect::{ApplyError, PartialReflect, Reflect};
use std::{any::TypeId, borrow::Cow, error::Error, fmt::Display, panic::Location, sync::Arc};

/// A wrapper around a reflect value to implement various traits useful for error reporting.
#[derive(Clone)]
pub struct ReflectWrapper(Arc<dyn PartialReflect>);

impl bevy_mod_scripting_display::DebugWithTypeInfo for ReflectWrapper {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        PrintReflectAsDebug::new_with_opt_info(&*self.0, type_info_provider)
            .to_string_with_type_info(f, type_info_provider)
    }
}

impl DisplayWithTypeInfo for ReflectWrapper {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        // TODO: different display?
        PrintReflectAsDebug::new_with_opt_info(&*self.0, type_info_provider)
            .to_string_with_type_info(f, type_info_provider)
    }
}

/// An error that occurred in an external library.
#[derive(Clone)]
pub struct ExternalError(pub Arc<dyn Error + Send + Sync>);

impl bevy_mod_scripting_display::DebugWithTypeInfo for ExternalError {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        write!(f, "External error: {}", self.0)
    }
}

impl DisplayWithTypeInfo for ExternalError {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        write!(f, "External error: {}", self.0)
    }
}

/// An error occurring when converting between rust and a script context.
#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
#[reflect(opaque)]
pub enum InteropError {
    /// The given feature is not implemented
    NotImplemented,
    /// A script value was of the wrong type
    ValueMismatch {
        /// The value with the wrong type
        value: Box<ScriptValue>,
        /// The expected type
        expected: Box<TypeId>,
    },
    /// The length of a list like structure was wrong
    LengthMismatch {
        /// The expected length
        expected: usize,
        /// The actual length
        got: usize,
    },
    /// Something failed when converting from a reflect
    FailedFromReflect {
        /// The type id of the reflect
        type_id: Box<Option<TypeId>>,
        /// The reason for the failure
        reason: Box<String>,
    },
    /// A type mismatch occurred
    TypeMismatch {
        /// The expected type
        expected: Box<TypeId>,
        /// The actual type
        got: Box<Option<TypeId>>,
    },
    /// A type mismatch occurred, but with a string representation of the expected type
    StringTypeMismatch {
        /// The expected type
        expected: Box<String>,
        /// The actual type
        got: Box<Option<TypeId>>,
    },
    /// Could not claim access to a value
    CannotClaimAccess {
        /// The id of the access
        base: Box<ReflectAccessId>,
        /// The location of the access
        location: Box<Option<Location<'static>>>,
        /// The context of the access
        context: Box<Cow<'static, str>>,
    },
    /// An invariant was broken
    Invariant(Box<String>),
    /// An unregistered component or resource type was used
    UnregisteredComponentOrResourceType {
        /// The name of the type
        type_name: Box<Cow<'static, str>>,
    },
    /// An unsupported operation was performed
    UnsupportedOperation {
        /// The base type of the operation
        base: Box<Option<TypeId>>,
        /// The value of the operation
        value: Box<Option<ReflectWrapper>>,
        /// The operation
        operation: Box<String>,
    },
    /// Some type data was missing
    MissingTypeData {
        /// The type id of the missing data
        type_id: Box<TypeId>,
        /// The missing type data
        type_data: Box<String>,
    },
    /// An entity was missing
    MissingEntity(Entity),

    /// An error occurred in a function
    FunctionInteropError {
        /// The name of the function
        function_name: Box<String>,
        /// The namespace of the function
        on: Box<Namespace>,
        /// The error that occurred
        error: Box<InteropError>,
        /// The context at the time of the call
        context: Box<Option<FunctionCallContext>>,
    },
    /// An error occurred when converting a function argument
    FunctionArgConversionError {
        /// The argument that failed to convert
        argument: Box<String>,
        /// The error that occurred
        error: Box<InteropError>,
    },
    /// The number of arguments in a function call was wrong
    ArgumentCountMismatch {
        /// The expected number of arguments
        expected: usize,
        /// The actual number of arguments
        got: usize,
    },
    /// A garbage collected allocation was used
    GarbageCollectedAllocation {
        /// The reference to the allocation
        reference: Box<ReflectReference>,
    },
    /// An unregistered reflect base was used
    UnregisteredReflectBase {
        /// The base that was unregistered
        base: Box<ReflectBaseType>,
    },
    /// An error occurred when using a reflection path
    ReflectionPathError {
        /// The error that occurred
        error: Box<String>,
        /// The reflected value
        reflected: Option<Box<ReflectReference>>,
    },
    /// Could not downcast a reference
    CouldNotDowncastReference {
        /// The type to downcast to
        to: Box<TypeId>,
        /// The reference to downcast
        reference: Box<ReflectReference>,
    },
    /// A schedule was missing
    MissingSchedule {
        /// The name of the schedule
        schedule_name: &'static str,
    },
    /// An invalid index was used
    InvalidIndex {
        /// The index that was invalid
        index: Box<ScriptValue>,
        /// The reason the index was invalid
        reason: Box<String>,
    },
    /// The world was missing
    MissingWorld,
    /// An external error occurred
    External(ExternalError),
    /// an error enriched with some contextual information
    WithContext(
        /// The context to add
        Box<Cow<'static, str>>,
        /// The error to add context to
        Box<InteropError>,
    ),
}

impl InteropError {
    /// Strips outer context layers from the error, returning all contexts and the base error
    pub fn unwrap_context(self) -> (Vec<Cow<'static, str>>, InteropError) {
        let mut contexts = Vec::new();
        let mut current = self;
        while let InteropError::WithContext(context, err) = current {
            contexts.push(*context);
            current = *err;
        }
        (contexts, current)
    }

    /// Adds context to an existing error
    pub fn with_context(self, context: impl Into<Cow<'static, str>>) -> Self {
        Self::WithContext(Box::new(context.into()), Box::new(self))
    }

    /// Creates a new external error.
    pub fn external(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::External(ExternalError(Arc::new(error)))
    }

    /// Creates a new external error from a boxed error.
    pub fn external_boxed(error: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self::External(ExternalError(Arc::from(error)))
    }

    /// Creates a new external error from a static string.
    pub fn str(reason: &'static str) -> Self {
        Self::External(ExternalError(Arc::from(Box::<
            dyn std::error::Error + Send + Sync + 'static,
        >::from(reason))))
    }

    /// Creates a new external error from a string.
    pub fn string(reason: String) -> Self {
        Self::External(ExternalError(Arc::from(Box::<
            dyn std::error::Error + Send + Sync + 'static,
        >::from(reason))))
    }

    /// Creates a new value mismatch error.
    pub fn value_mismatch(expected: TypeId, value: ScriptValue) -> Self {
        Self::ValueMismatch {
            value: Box::new(value),
            expected: Box::new(expected),
        }
    }

    /// Creates a new length mismatch error.
    pub fn length_mismatch(expected: usize, got: usize) -> Self {
        Self::LengthMismatch { expected, got }
    }

    /// Creates a new failed from reflect error.
    pub fn failed_from_reflect(type_id: Option<TypeId>, reason: impl Display) -> Self {
        Self::FailedFromReflect {
            type_id: Box::new(type_id),
            reason: Box::new(reason.to_string()),
        }
    }

    /// Creates a new type mismatch error.
    pub fn type_mismatch(expected: TypeId, got: Option<TypeId>) -> Self {
        Self::TypeMismatch {
            expected: Box::new(expected),
            got: Box::new(got),
        }
    }

    /// Creates a new string type mismatch error.
    pub fn string_type_mismatch(expected: String, got: Option<TypeId>) -> Self {
        Self::StringTypeMismatch {
            expected: Box::new(expected),
            got: Box::new(got),
        }
    }

    /// Creates a new cannot claim access error.
    pub fn cannot_claim_access(
        base: ReflectAccessId,
        location: Option<Location<'static>>,
        context: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::CannotClaimAccess {
            base: Box::new(base),
            location: Box::new(location),
            context: Box::new(context.into()),
        }
    }

    /// Creates a new invariant error.
    pub fn invariant(reason: impl Into<String>) -> Self {
        Self::Invariant(Box::new(reason.into()))
    }

    /// Creates a new unregistered component or resource type error.
    pub fn unregistered_component_or_resource_type(
        type_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::UnregisteredComponentOrResourceType {
            type_name: Box::new(type_name.into()),
        }
    }

    /// Creates a new unsupported operation error.
    pub fn unsupported_operation(
        base: Option<TypeId>,
        value: Option<Box<dyn PartialReflect>>,
        op: impl Display,
    ) -> Self {
        Self::UnsupportedOperation {
            base: Box::new(base),
            value: Box::new(value.map(|v| ReflectWrapper(Arc::from(v)))),
            operation: Box::new(op.to_string()),
        }
    }

    /// Creates a new missing type data error.
    pub fn missing_type_data(type_id: TypeId, type_data: String) -> Self {
        Self::MissingTypeData {
            type_id: Box::new(type_id),
            type_data: Box::new(type_data),
        }
    }

    /// Creates a new missing entity error.
    pub fn missing_entity(entity: Entity) -> Self {
        Self::MissingEntity(entity)
    }

    /// Creates a new reflect apply error.
    pub fn reflect_apply_error(e: ApplyError) -> Self {
        Self::External(ExternalError(Arc::new(e)))
    }

    /// Creates a new function interop error.
    pub fn function_interop_error(
        function_name: impl Display,
        on: Namespace,
        error: InteropError,
        context: Option<FunctionCallContext>,
    ) -> Self {
        Self::FunctionInteropError {
            function_name: Box::new(function_name.to_string()),
            on: Box::new(on),
            error: Box::new(error),
            context: Box::new(context),
        }
    }

    /// Thrown when an error happens during argument conversion in a function call
    pub fn function_arg_conversion_error(argument: String, error: InteropError) -> Self {
        Self::FunctionArgConversionError {
            argument: Box::new(argument),
            error: Box::new(error),
        }
    }

    /// Thrown when the number of arguments in a function call does not match.
    pub fn argument_count_mismatch(expected: usize, got: usize) -> Self {
        Self::ArgumentCountMismatch { expected, got }
    }

    /// Creates a new garbage collected allocation error.
    pub fn garbage_collected_allocation(reference: ReflectReference) -> Self {
        Self::GarbageCollectedAllocation {
            reference: Box::new(reference),
        }
    }

    /// Creates a new unregistered base error.
    pub fn unregistered_base(base: ReflectBaseType) -> Self {
        Self::UnregisteredReflectBase {
            base: Box::new(base),
        }
    }

    /// Creates a new reflection path error.
    pub fn reflection_path_error(error: String, reflected: Option<ReflectReference>) -> Self {
        Self::ReflectionPathError {
            error: Box::new(error),
            reflected: reflected.map(Box::new),
        }
    }

    /// Creates a new could not downcast error.
    pub fn could_not_downcast(reference: ReflectReference, to: TypeId) -> Self {
        Self::CouldNotDowncastReference {
            to: Box::new(to),
            reference: Box::new(reference),
        }
    }

    /// Creates a new missing schedule error.
    pub fn missing_schedule(schedule_name: &'static str) -> Self {
        Self::MissingSchedule { schedule_name }
    }

    /// Creates a new invalid index error.
    pub fn invalid_index(index: ScriptValue, reason: impl Into<String>) -> Self {
        Self::InvalidIndex {
            index: Box::new(index),
            reason: Box::new(reason.into()),
        }
    }

    /// Creates a new missing world error.
    pub fn missing_world() -> Self {
        Self::MissingWorld
    }

    /// Creates a new missing function error.
    pub fn missing_function(
        function_name: impl Display,
        on: Namespace,
        context: Option<FunctionCallContext>,
    ) -> Self {
        Self::FunctionInteropError {
            function_name: Box::new(function_name.to_string()),
            on: Box::new(on),
            error: Box::new(InteropError::str("Function not found")),
            context: Box::new(context),
        }
    }
}

impl std::fmt::Display for InteropError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&WithTypeInfo::new(self), f)
    }
}

impl std::error::Error for InteropError {}

impl DisplayWithTypeInfo for InteropError {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            InteropError::NotImplemented => {
                write!(f, "This feature is not implemented")
            }
            InteropError::ValueMismatch { value, expected } => {
                write!(
                    f,
                    "Value type mismatch: expected {}, got {}",
                    WithTypeInfo::new_with_opt_info(expected, type_info_provider),
                    WithTypeInfo::new_with_opt_info(value, type_info_provider)
                )
            }
            InteropError::LengthMismatch { expected, got } => {
                write!(f, "Length mismatch: expected {expected}, got {got}")
            }
            InteropError::FailedFromReflect { type_id, reason } => {
                write!(
                    f,
                    "Failed to convert from reflect {}: {}",
                    WithTypeInfo::new_with_opt_info(
                        &type_id.as_ref().or_fake_id(),
                        type_info_provider
                    ),
                    reason
                )
            }
            InteropError::TypeMismatch { expected, got } => {
                write!(
                    f,
                    "Type mismatch: expected {}, got {}",
                    WithTypeInfo::new_with_opt_info(expected, type_info_provider),
                    WithTypeInfo::new_with_opt_info(&got.as_ref().or_fake_id(), type_info_provider)
                )
            }
            InteropError::StringTypeMismatch { expected, got } => {
                write!(
                    f,
                    "Type mismatch: expected {}, got {}",
                    expected,
                    WithTypeInfo::new_with_opt_info(&got.as_ref().or_fake_id(), type_info_provider)
                )
            }
            InteropError::CannotClaimAccess {
                base,
                location,
                context,
            } => {
                if let Some(location) = location.as_ref() {
                    write!(
                        f,
                        "Cannot claim access to {} at {}:{}:{}: {}",
                        WithTypeInfo::new_with_opt_info(base, type_info_provider),
                        location.file(),
                        location.line(),
                        location.column(),
                        context
                    )
                } else {
                    write!(
                        f,
                        "Cannot claim access to {}: {}",
                        WithTypeInfo::new_with_opt_info(base, type_info_provider),
                        context
                    )
                }
            }
            InteropError::Invariant(i) => {
                write!(f, "Invariant broken: {i}")
            }
            InteropError::UnregisteredComponentOrResourceType { type_name } => {
                write!(f, "Unregistered component or resource type: {type_name}")
            }
            InteropError::UnsupportedOperation {
                base,
                value,
                operation,
            } => {
                write!(
                    f,
                    "Unsupported operation on {}{}: {}",
                    WithTypeInfo::new_with_opt_info(
                        &base.as_ref().or_fake_id(),
                        type_info_provider
                    ),
                    if let Some(value) = value.as_ref() {
                        format!(
                            " with value {}",
                            WithTypeInfo::new_with_opt_info(value, type_info_provider)
                        )
                    } else {
                        "".to_string()
                    },
                    operation
                )
            }
            InteropError::MissingTypeData { type_id, type_data } => {
                write!(
                    f,
                    "Missing type data {} for type: {}",
                    type_data,
                    WithTypeInfo::new_with_opt_info(type_id, type_info_provider)
                )
            }
            InteropError::MissingEntity(entity) => {
                if *entity == Entity::PLACEHOLDER || entity.index() == 0 {
                    write!(
                        f,
                        "Invalid entity: {entity}. Are you trying to use an entity in a callback in which it's unavailable?"
                    )
                } else {
                    write!(f, "Missing or invalid entity: {entity}")
                }
            }
            InteropError::FunctionInteropError {
                function_name,
                on,
                error,
                context,
            } => {
                write!(
                    f,
                    "Error {}\n in function {} on {}:\n {}",
                    context
                        .clone()
                        .unwrap_or(FunctionCallContext::new(Language::Unknown)),
                    function_name,
                    WithTypeInfo::new_with_opt_info(on, type_info_provider),
                    error
                )
            }
            InteropError::FunctionArgConversionError { argument, error } => {
                write!(
                    f,
                    "Error converting argument {}: {}",
                    argument,
                    WithTypeInfo::new_with_opt_info(error, type_info_provider)
                )
            }
            InteropError::ArgumentCountMismatch { expected, got } => {
                write!(f, "Argument count mismatch: expected {expected}, got {got}")
            }
            InteropError::GarbageCollectedAllocation { reference } => {
                write!(
                    f,
                    "Garbage collected allocation used: {}",
                    WithTypeInfo::new_with_opt_info(reference, type_info_provider)
                )
            }
            InteropError::UnregisteredReflectBase { base } => {
                write!(
                    f,
                    "Unregistered reflect base: {}",
                    WithTypeInfo::new_with_opt_info(base, type_info_provider)
                )
            }
            InteropError::ReflectionPathError { error, reflected } => {
                write!(
                    f,
                    "Reflection path error: {}{}",
                    error,
                    if let Some(reflected) = reflected.as_ref() {
                        format!(
                            "\nOn value: {}",
                            WithTypeInfo::new_with_opt_info(reflected, type_info_provider)
                        )
                    } else {
                        "".to_string()
                    }
                )
            }
            InteropError::CouldNotDowncastReference { to, reference } => {
                write!(
                    f,
                    "Could not downcast reference {} to type {}",
                    WithTypeInfo::new_with_opt_info(reference, type_info_provider),
                    WithTypeInfo::new_with_opt_info(to, type_info_provider)
                )
            }
            InteropError::MissingSchedule { schedule_name } => {
                write!(f, "Missing schedule: {schedule_name}")
            }
            InteropError::InvalidIndex { index, reason } => {
                write!(
                    f,
                    "Invalid index {}: {}",
                    WithTypeInfo::new_with_opt_info(index, type_info_provider),
                    reason
                )
            }
            InteropError::MissingWorld => {
                write!(f, "Missing world")
            }
            InteropError::External(external_error) => {
                write!(
                    f,
                    "External error: {}",
                    WithTypeInfo::new_with_opt_info(external_error, type_info_provider)
                )
            }
            InteropError::WithContext(cow, interop_error) => {
                write!(
                    f,
                    "{}: {}",
                    cow,
                    WithTypeInfo::new_with_opt_info(interop_error, type_info_provider)
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bevy_reflect::TypeRegistry;

    use super::*;
    #[test]
    fn test_script_value_prints_using_type_data() {
        // check script values print fine
        let mut registry = TypeRegistry::empty();
        registry.register::<ScriptValue>();
        pretty_assertions::assert_str_eq!(
            format!(
                "{:?}",
                PrintReflectAsDebug::new_with_opt_info(&ScriptValue::Integer(1), Some(&registry))
            ),
            "1",
        );
    }
}
