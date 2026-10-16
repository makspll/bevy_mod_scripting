use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

use crate::IntoRhaiError;
use bevy_mod_scripting_bindings::{
    ReflectReference, ScriptValue, WorldGuard,
    dispatch::{self, Member},
    error::InteropError,
    function::script_function::DynamicScriptFunctionMut,
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use rhai::{CustomType, Dynamic, EvalAltResult};
use strum::VariantNames;

use super::script_value::{FromDynamic, IntoDynamic, RHAI_CALLER_CONTEXT};

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

/// Runs a [`dispatch`] function with the world of the running script.
fn with_world<O>(
    f: impl FnOnce(WorldGuard<'static>) -> Result<O, InteropError>,
) -> Result<O, Box<EvalAltResult>> {
    let world = ThreadWorldContainer
        .try_get_context()
        .map_err(IntoRhaiError::into_rhai_error)?
        .world;
    f(world).map_err(IntoRhaiError::into_rhai_error)
}

/// Applies a binary operator, see [`dispatch::binary`].
fn binary(
    operator: ScriptOperatorNames,
    lhs: ScriptValue,
    rhs: ScriptValue,
) -> Result<Dynamic, Box<EvalAltResult>> {
    with_world(|world| dispatch::binary(world, operator, lhs, rhs, RHAI_CALLER_CONTEXT))?
        .into_dynamic()
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
        let result = ThreadWorldContainer
            .try_get_context()
            .map_err(InteropError::from)
            .and_then(|ctxt| dispatch::iter(ctxt.world, self.0, RHAI_CALLER_CONTEXT));

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
    }
}

/// The binary operators of references, with the [`ScriptOperatorNames`] they dispatch to.
const BINARY_OPERATORS: [(RhaiOperator, ScriptOperatorNames); 8] = [
    (RhaiOperator::Sub, ScriptOperatorNames::Subtraction),
    (RhaiOperator::Add, ScriptOperatorNames::Addition),
    (RhaiOperator::Mul, ScriptOperatorNames::Multiplication),
    (RhaiOperator::Div, ScriptOperatorNames::Division),
    (RhaiOperator::Mod, ScriptOperatorNames::Remainder),
    (RhaiOperator::Pow, ScriptOperatorNames::Exponentiation),
    (RhaiOperator::Eq, ScriptOperatorNames::Equality),
    (RhaiOperator::Lt, ScriptOperatorNames::LessThanComparison),
];

impl CustomType for RhaiReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<RhaiReflectReference>())
            .with_indexer_get(|self_: &mut Self, index: Dynamic| {
                let receiver = self_.0.clone();
                let key = ScriptValue::from_dynamic(index)?;
                // rhai has no method call syntax for function values, so methods get their receiver here
                match with_world(|world| {
                    dispatch::index(world, receiver.clone(), key, RHAI_CALLER_CONTEXT)
                })? {
                    Member::Method(func) => {
                        ScriptValue::Function(func.curry(ScriptValue::Reference(receiver)))
                            .into_dynamic()
                    }
                    Member::Static(value) => value.into_dynamic(),
                }
            })
            .with_indexer_set(|self_: &mut Self, index: Dynamic, value: Dynamic| {
                let key = ScriptValue::from_dynamic(index)?;
                let value = ScriptValue::from_dynamic(value)?;
                with_world(|world| {
                    dispatch::set_index(world, self_.0.clone(), key, value, RHAI_CALLER_CONTEXT)
                })
            })
            .with_fn(RhaiOperator::Unm.function_name(), |self_: Self| {
                with_world(|world| {
                    dispatch::unary(
                        world,
                        ScriptOperatorNames::Negation,
                        self_.0,
                        RHAI_CALLER_CONTEXT,
                    )
                })?
                .into_dynamic()
            })
            .with_fn(
                RhaiOperator::Ne.function_name(),
                |self_: Self, other: Dynamic| {
                    let other = ScriptValue::from_dynamic(other)?;
                    let eq = with_world(|world| {
                        dispatch::binary(
                            world,
                            ScriptOperatorNames::Equality,
                            ScriptValue::Reference(self_.0),
                            other,
                            RHAI_CALLER_CONTEXT,
                        )
                    })?;
                    match eq {
                        ScriptValue::Bool(b) => ScriptValue::Bool(!b).into_dynamic(),
                        _ => {
                            Err(InteropError::invariant("eq did not return a bool")
                                .into_rhai_error())
                        }
                    }
                },
            )
            .on_debug(|self_| {
                with_world(|world| dispatch::debug(world, self_.0.clone(), RHAI_CALLER_CONTEXT))
                    .unwrap_or_else(|error| error.to_string())
            })
            .on_print(|self_| {
                with_world(|world| dispatch::display(world, self_.0.clone(), RHAI_CALLER_CONTEXT))
                    .unwrap_or_else(|error| error.to_string())
            });

        for (rhai_operator, operator) in BINARY_OPERATORS {
            let name = rhai_operator.function_name();
            builder.with_fn(name, move |self_: Self, other: Dynamic| {
                binary(
                    operator,
                    ScriptValue::Reference(self_.0),
                    ScriptValue::from_dynamic(other)?,
                )
            });
            // a value on the right of a number, i.e. `2 * v`, dispatches to the operator's reflected function
            if operator.reflected_script_function_name().is_some() {
                builder
                    .with_fn(name, move |lhs: rhai::INT, self_: Self| {
                        binary(
                            operator,
                            ScriptValue::Integer(lhs),
                            ScriptValue::Reference(self_.0),
                        )
                    })
                    .with_fn(name, move |lhs: rhai::FLOAT, self_: Self| {
                        binary(
                            operator,
                            ScriptValue::Float(lhs),
                            ScriptValue::Reference(self_.0),
                        )
                    });
            }
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
/// A type itself, i.e. what [`ScriptValue::Type`] is in Rhai. Indexing it reaches the type's static members.
pub struct RhaiTypeRef(pub TypeId);

impl CustomType for RhaiTypeRef {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<RhaiTypeRef>())
            .with_indexer_get(|self_: &mut Self, index: Dynamic| {
                let key = ScriptValue::from_dynamic(index)?;
                with_world(|world| {
                    dispatch::static_index(world, self_.0, key, RHAI_CALLER_CONTEXT)
                })?
                .into_dynamic()
            });
    }
}
