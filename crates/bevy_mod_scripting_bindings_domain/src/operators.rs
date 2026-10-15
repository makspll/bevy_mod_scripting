//! Describes the operators supported by BMS.
//!
//! These operators are implemented by each language plugin,
//! by searching for appropriate functions in the registry.

/// Main operator enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptOperatorNames {
    /// a + b
    Addition,
    /// a - b
    Subtraction,
    /// a * b
    Multiplication,
    /// a / b
    Division,
    /// a % b
    Remainder,
    /// -a
    Negation,
    /// a ^ 2 or a ** b
    Exponentiation,
    /// a == b
    Equality,
    /// a < b
    LessThanComparison,
    /// len(a)
    Length,
    /// for a in b.iter()
    Iteration,
    /// print(a)
    DisplayPrint,
    /// debug(a)
    DebugPrint,
}

impl ScriptOperatorNames {
    /// Returns the function names to dispatch these operators to
    pub const fn script_function_name(self) -> &'static str {
        match self {
            ScriptOperatorNames::Addition => "add",
            ScriptOperatorNames::Subtraction => "sub",
            ScriptOperatorNames::Multiplication => "mul",
            ScriptOperatorNames::Division => "div",
            ScriptOperatorNames::Remainder => "rem",
            ScriptOperatorNames::Negation => "neg",
            ScriptOperatorNames::Exponentiation => "pow",
            ScriptOperatorNames::Equality => "eq",
            ScriptOperatorNames::LessThanComparison => "lt",
            ScriptOperatorNames::Length => "len",
            ScriptOperatorNames::Iteration => "iter",
            ScriptOperatorNames::DisplayPrint => "display",
            ScriptOperatorNames::DebugPrint => "debug",
        }
    }

    /// Returns the function name to dispatch a binary operator to when only its right operand is a value, i.e. `2 * v`.
    ///
    /// The function is called on the right operand, with the left one as its argument.
    /// For `+` and `*` this is the same function as for `v * 2`, the others use reflected names, i.e. `2 - v` calls `rsub(v, 2)`.
    /// Returns `None` for operators which aren't arithmetic.
    pub const fn reflected_script_function_name(self) -> Option<&'static str> {
        match self {
            ScriptOperatorNames::Addition => Some("add"),
            ScriptOperatorNames::Multiplication => Some("mul"),
            ScriptOperatorNames::Subtraction => Some("rsub"),
            ScriptOperatorNames::Division => Some("rdiv"),
            ScriptOperatorNames::Remainder => Some("rrem"),
            ScriptOperatorNames::Exponentiation => Some("rpow"),
            _ => None,
        }
    }

    /// Parse an operator function into this enum
    pub fn parse(name: impl AsRef<str>) -> Option<Self> {
        match name.as_ref() {
            "add" => Some(ScriptOperatorNames::Addition),
            "sub" => Some(ScriptOperatorNames::Subtraction),
            "mul" => Some(ScriptOperatorNames::Multiplication),
            "div" => Some(ScriptOperatorNames::Division),
            "rem" => Some(ScriptOperatorNames::Remainder),
            "neg" => Some(ScriptOperatorNames::Negation),
            "pow" => Some(ScriptOperatorNames::Exponentiation),
            "eq" => Some(ScriptOperatorNames::Equality),
            "lt" => Some(ScriptOperatorNames::LessThanComparison),
            "len" => Some(ScriptOperatorNames::Length),
            "iter" => Some(ScriptOperatorNames::Iteration),
            "display" => Some(ScriptOperatorNames::DisplayPrint),
            "debug" => Some(ScriptOperatorNames::DebugPrint),
            _ => None,
        }
    }
}
