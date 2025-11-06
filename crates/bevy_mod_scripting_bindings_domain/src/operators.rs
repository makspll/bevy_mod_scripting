//! Describes the operators supported by BMS.
//!
//! These operators are implemented by each language plugin,
//! by searching for appropriate functions in the registry.

/// Main operator enum
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
