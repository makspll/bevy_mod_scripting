use std::str::FromStr;

#[derive(strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum ForbiddenKeywords {
    End,
    And,
    Break,
    Do,
    Else,
    Elseif,
    False,
    For,
    Function,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
}

impl ForbiddenKeywords {
    /// Checks if a keyword is forbidden
    pub fn is_forbidden_err(word: &str) -> anyhow::Result<&str> {
        if ForbiddenKeywords::from_str(word).is_ok() {
            Err(anyhow::anyhow!("{word} is a reserved keyword in lua"))
        } else {
            Ok(word)
        }
    }
}
