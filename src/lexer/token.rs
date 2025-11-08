use {
  crate::lexer::source::Position, derive_more::Constructor, getset::Getters,
  ordered_float::OrderedFloat, strum::Display, strum_macros::EnumString
};

#[derive(Debug, Constructor, Getters)]
pub struct Token<'token> {
  #[getset(get = "pub")]
  r#type: TokenType<'token>,

  #[getset(get = "pub")]
  position: Position
}

impl<'token> Token<'token> {
  pub fn is_literal(&self) -> bool {
    matches!(
      self.r#type(),
      TokenType::Number(_) | TokenType::String(_) | TokenType::Identifier(_)
    )
  }
}

#[derive(Debug, PartialEq, Eq, Display)]
pub enum TokenType<'token_type> {
  #[strum(to_string = "(")]
  OpenParanthesis,

  #[strum(to_string = ")")]
  CloseParanthesis,

  #[strum(to_string = "{")]
  OpenBrace,

  #[strum(to_string = "}}")]
  CloseBrace,

  #[strum(to_string = ",")]
  Comma,

  #[strum(to_string = ".")]
  Dot,

  #[strum(to_string = ";")]
  Semicolon,

  #[strum(to_string = "+")]
  Plus,

  #[strum(to_string = "-")]
  Minus,

  #[strum(to_string = "*")]
  Multiply,

  #[strum(to_string = "/")]
  Divide,

  #[strum(to_string = "=")]
  Assign,

  #[strum(to_string = "!")]
  Not,

  #[strum(to_string = "!=")]
  NotEquals,

  #[strum(to_string = "==")]
  Equals,

  #[strum(to_string = ">")]
  GreaterThan,

  #[strum(to_string = ">=")]
  GreaterThanOrEquals,

  #[strum(to_string = "<")]
  LessThan,

  #[strum(to_string = "<=")]
  LessThanOrEquals,

  #[strum(to_string = "{0}")]
  String(&'token_type str),

  #[strum(to_string = "{0}")]
  Number(OrderedFloat<f64>),

  #[strum(to_string = "{0}")]
  Identifier(&'token_type str),

  #[strum(to_string = "{0}")]
  Keyword(Keyword)
}

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While
}
