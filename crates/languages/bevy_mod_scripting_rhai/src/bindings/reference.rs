use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

use crate::IntoRhaiError;
use bevy_mod_scripting_bindings::{
    ReflectReference, ScriptValue, ThreadWorldContainer, error::InteropError,
    function::script_function::DynamicScriptFunctionMut,
};
use bevy_mod_scripting_display::OrFakeId;
use rhai::{CustomType, Dynamic, EvalAltResult};
use strum::VariantNames;

use super::script_value::{FromDynamic, FunctionWithReceiver, IntoDynamic, RHAI_CALLER_CONTEXT};

#[derive(Debug, strum::EnumString, strum::VariantNames, Clone)]
/// A list of reserved keywords in Rhai
#[allow(missing_docs)]
pub enum ReservedKeyword {
    // Reserved under certain flags
    #[strum(serialize = "?.")]
    QuestionDot,
    #[strum(serialize = "?[")]
    QuestionBracket,
    #[strum(serialize = "fn")]
    Fn,
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "import")]
    Import,
    #[strum(serialize = "export")]
    Export,
    #[strum(serialize = "as")]
    As,
    // Reserved symbols
    #[strum(serialize = "===")]
    TripleEquals,
    #[strum(serialize = "!==")]
    NotEquals,
    #[strum(serialize = "->")]
    ArrowRight,
    #[strum(serialize = "<-")]
    ArrowLeft,
    #[strum(serialize = "?")]
    Question,
    #[strum(serialize = ":=")]
    ColonEquals,
    #[strum(serialize = ":;")]
    ColonSemicolon,
    #[strum(serialize = "~")]
    Tilde,
    #[strum(serialize = "!.")]
    ExclamationDot,
    #[strum(serialize = "::<")]
    DoubleColonLess,
    #[strum(serialize = "(*")]
    ParenStar,
    #[strum(serialize = "*)")]
    StarParen,
    #[strum(serialize = "#")]
    Hash,
    #[strum(serialize = "#!")]
    HashBang,
    #[strum(serialize = "@")]
    At,
    #[strum(serialize = "$")]
    Dollar,
    #[strum(serialize = "++")]
    PlusPlus,
    #[strum(serialize = "--")]
    MinusMinus,
    #[strum(serialize = "...")]
    Ellipsis,
    #[strum(serialize = "<|")]
    LessPipe,
    #[strum(serialize = "|>")]
    PipeGreater,
    // Reserved keywords
    #[strum(serialize = "public")]
    Public,
    #[strum(serialize = "protected")]
    Protected,
    #[strum(serialize = "super")]
    Super,
    #[strum(serialize = "new")]
    New,
    #[strum(serialize = "use")]
    Use,
    #[strum(serialize = "module")]
    Module,
    #[strum(serialize = "package")]
    Package,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "shared")]
    Shared,
    #[strum(serialize = "with")]
    With,
    #[strum(serialize = "is")]
    Is,
    #[strum(serialize = "goto")]
    Goto,
    #[strum(serialize = "exit")]
    Exit,
    #[strum(serialize = "match")]
    Match,
    #[strum(serialize = "case")]
    Case,
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "void")]
    Void,
    #[strum(serialize = "null")]
    Null,
    #[strum(serialize = "nil")]
    Nil,
    #[strum(serialize = "spawn")]
    Spawn,
    #[strum(serialize = "thread")]
    Thread,
    #[strum(serialize = "go")]
    Go,
    #[strum(serialize = "sync")]
    Sync,
    #[strum(serialize = "async")]
    Async,
    #[strum(serialize = "await")]
    Await,
    #[strum(serialize = "yield")]
    Yield,
    // Keyword functions
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "debug")]
    Debug,
    #[strum(serialize = "type_of")]
    TypeOf,
    #[strum(serialize = "eval")]
    Eval,
    #[strum(serialize = "Fn")]
    FnKeyword,
    #[strum(serialize = "call")]
    Call,
    #[strum(serialize = "curry")]
    Curry,
    #[strum(serialize = "this")]
    This,
    #[strum(serialize = "is_def_var")]
    IsDefVar,
    #[strum(serialize = "is_def_fn")]
    IsDefFn,
    #[strum(serialize = "is_shared")]
    IsShared,
}

impl ReservedKeyword {
    /// Returns whether the given string is a reserved keyword in Rhai
    pub fn is_reserved_keyword(s: impl AsRef<str>) -> bool {
        ReservedKeyword::VARIANTS.iter().any(|v| v == &s.as_ref())
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A wrapper around a [`ReflectReference`] that implements [`CustomType`] for Rhai
pub struct RhaiReflectReference(pub ReflectReference);

impl AsRef<ReflectReference> for RhaiReflectReference {
    fn as_ref(&self) -> &ReflectReference {
        &self.0
    }
}

impl From<ReflectReference> for RhaiReflectReference {
    fn from(value: ReflectReference) -> Self {
        RhaiReflectReference(value)
    }
}

impl From<RhaiReflectReference> for ReflectReference {
    fn from(value: RhaiReflectReference) -> Self {
        value.0
    }
}

impl Deref for RhaiReflectReference {
    type Target = ReflectReference;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RhaiReflectReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A rhai operator enum
#[allow(missing_docs)]
pub enum RhaiOperator {
    Sub,
    Add,
    Mul,
    Div,
    Mod,
    Unm,
    Pow,
    Eq,
    Ne,
    Lt,
}

impl RhaiOperator {
    /// Returns the function name for the operator
    pub fn function_name(self) -> &'static str {
        match self {
            RhaiOperator::Sub => "-",
            RhaiOperator::Add => "+",
            RhaiOperator::Mul => "*",
            RhaiOperator::Div => "/",
            RhaiOperator::Mod => "%",
            RhaiOperator::Unm => "-",
            RhaiOperator::Pow => "**",
            RhaiOperator::Eq => "==",
            RhaiOperator::Lt => "<",
            RhaiOperator::Ne => "!=",
        }
    }
}

/// An iterator over a [`ReflectReference`] that implements [`IntoIterator`] for Rhai.
pub struct RhaiReflectRefIter {
    next_func: DynamicScriptFunctionMut,
}

impl Iterator for RhaiReflectRefIter {
    type Item = Result<Dynamic, Box<EvalAltResult>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_func.call(vec![], RHAI_CALLER_CONTEXT) {
            Ok(ScriptValue::Unit) => None,
            Ok(v) => Some(v.into_dynamic()),
            Err(error) => Some(Err(error.into_rhai_error())),
        }
    }
}

impl IntoIterator for RhaiReflectReference {
    type Item = Result<Dynamic, Box<EvalAltResult>>;

    type IntoIter = RhaiReflectRefIter;

    fn into_iter(self) -> Self::IntoIter {
        let result = (|| {
            let world = ThreadWorldContainer.try_get_context()?.world;

            let iter_func = world
                .lookup_function([TypeId::of::<ReflectReference>()], "iter")
                .map_err(|f| {
                    InteropError::missing_function(
                        f,
                        TypeId::of::<ReflectReference>().into(),
                        Some(RHAI_CALLER_CONTEXT),
                    )
                })?;

            iter_func.call(
                vec![ScriptValue::Reference(self.0.clone())],
                RHAI_CALLER_CONTEXT,
            )
        })();

        match result {
            Ok(ScriptValue::FunctionMut(f)) => RhaiReflectRefIter { next_func: f },
            Ok(_) => RhaiReflectRefIter {
                next_func: (|_, _| {
                    ScriptValue::Error(InteropError::invariant(
                        "iter function did not return a function",
                    ))
                })
                .into(),
            },
            Err(error) => RhaiReflectRefIter {
                next_func: (move |_, _| ScriptValue::Error(error.clone())).into(),
            },
        }

        // manually call
    }
}

impl CustomType for RhaiReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<RhaiReflectReference>())
            .with_indexer_get(|self_: &mut Self, index: Dynamic| {
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .world;
                let self_ = &self_.0;
                let type_id = self_
                    .tail_type_id(world.clone())
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .or_fake_id();

                let key: ScriptValue = ScriptValue::from_dynamic(index)?;
                let key = match key.as_string() {
                    Ok(string) => {
                        match world
                            .lookup_function([type_id, TypeId::of::<ReflectReference>()], string)
                        {
                            Ok(func) => {
                                return FunctionWithReceiver::curry(func, self_.clone().into())
                                    .into_dynamic();
                            }
                            Err(string) => ScriptValue::String(string),
                        }
                    }
                    Err(key) => key,
                };

                // call the default magic getter
                let registry = world.script_function_registry();
                let registry = registry.read();

                let out = registry
                    .magic_functions
                    .get(RHAI_CALLER_CONTEXT, self_.clone(), key)
                    .map_err(IntoRhaiError::into_rhai_error)?;

                out.into_dynamic()
            })
            .with_indexer_set(|self_: &mut Self, index: Dynamic, value: Dynamic| {
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .world;
                let self_ = self_.0.clone();
                let key = ScriptValue::from_dynamic(index)?;
                let value = ScriptValue::from_dynamic(value)?;

                let registry = world.script_function_registry();
                let registry = registry.read();

                registry
                    .magic_functions
                    .set(RHAI_CALLER_CONTEXT, self_, key, value)
                    .map_err(IntoRhaiError::into_rhai_error)?;
                Ok(())
            })
            .with_fn(
                RhaiOperator::Sub.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "sub", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Add.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "add", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Mul.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "mul", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Div.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "div", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Mod.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "rem", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(RhaiOperator::Unm.function_name(), |self_: Self| {
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .world;
                let self_: ReflectReference = self_.0.clone();
                let target_type_id = self_
                    .tail_type_id(world.clone())
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .or_fake_id();
                let args = vec![ScriptValue::Reference(self_)];
                let out = world
                    .try_call_overloads(target_type_id, "neg", args, RHAI_CALLER_CONTEXT)
                    .map_err(IntoRhaiError::into_rhai_error)?;
                out.into_dynamic()
            })
            .with_fn(
                RhaiOperator::Pow.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "pow", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Eq.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "eq", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .with_fn(
                RhaiOperator::Ne.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "eq", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    match out {
                        ScriptValue::Bool(b) => ScriptValue::Bool(!b).into_dynamic(),
                        _ => {
                            Err(InteropError::invariant("eq did not return a bool")
                                .into_rhai_error())
                        }
                    }
                },
            )
            .with_fn(
                RhaiOperator::Lt.function_name(),
                |self_: Self, other: Dynamic| {
                    let world = ThreadWorldContainer
                        .try_get_context()
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .world;
                    let self_: ReflectReference = self_.0.clone();
                    let other: ScriptValue = ScriptValue::from_dynamic(other)?;
                    let target_type_id = self_
                        .tail_type_id(world.clone())
                        .map_err(IntoRhaiError::into_rhai_error)?
                        .or_fake_id();
                    let args = vec![ScriptValue::Reference(self_), other];
                    let out = world
                        .try_call_overloads(target_type_id, "lt", args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;
                    out.into_dynamic()
                },
            )
            .on_debug(|self_| {
                let result: Result<_, InteropError> = (|| {
                    let world = ThreadWorldContainer.try_get_context()?.world;
                    let reflect_reference = self_.0.clone();

                    let func = world
                        .lookup_function([TypeId::of::<ReflectReference>()], "debug")
                        .map_err(|f| {
                            InteropError::missing_function(
                                f,
                                TypeId::of::<ReflectReference>().into(),
                                Some(RHAI_CALLER_CONTEXT),
                            )
                        })?;

                    let out = func.call(
                        vec![ScriptValue::Reference(reflect_reference)],
                        RHAI_CALLER_CONTEXT,
                    )?;

                    match out {
                        ScriptValue::String(s) => Ok(s),
                        _ => Err(InteropError::invariant("debug failed to return a string")),
                    }
                })();

                match result {
                    Ok(str_) => str_.into(),
                    Err(error) => error.to_string(),
                }
            })
            .on_print(|self_| {
                let result: Result<_, InteropError> = (|| {
                    let world = ThreadWorldContainer.try_get_context()?.world;
                    let reflect_reference = self_.0.clone();

                    let func = world
                        .lookup_function([TypeId::of::<ReflectReference>()], "display")
                        .map_err(|f| {
                            InteropError::missing_function(
                                f,
                                TypeId::of::<ReflectReference>().into(),
                                Some(RHAI_CALLER_CONTEXT),
                            )
                        })?;

                    let out = func.call(
                        vec![ScriptValue::Reference(reflect_reference)],
                        RHAI_CALLER_CONTEXT,
                    )?;

                    match out {
                        ScriptValue::String(s) => Ok(s),
                        _ => Err(InteropError::invariant("display failed to return a string")),
                    }
                })();

                match result {
                    Ok(str_) => str_.into(),
                    Err(error) => error.to_string(),
                }
            });
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
/// A wrapper around a [`TypeId`] that implements [`CustomType`] for Rhai
pub struct RhaiStaticReflectReference(pub TypeId);

impl CustomType for RhaiStaticReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<RhaiStaticReflectReference>())
            .with_indexer_get(|self_: &mut Self, index: Dynamic| {
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .world;
                let type_id = self_.0;
                let key: ScriptValue = ScriptValue::from_dynamic(index)?;

                let key = match key.as_string() {
                    Ok(name) => match world.lookup_function([type_id], name) {
                        Ok(func) => return ScriptValue::Function(func).into_dynamic(),
                        Err(key) => ScriptValue::String(key),
                    },
                    Err(key) => key,
                };

                Err::<_, Box<EvalAltResult>>(
                    InteropError::missing_function(
                        format!("{key:#?}"),
                        type_id.into(),
                        Some(RHAI_CALLER_CONTEXT),
                    )
                    .into_rhai_error(),
                )
            });
    }
}
