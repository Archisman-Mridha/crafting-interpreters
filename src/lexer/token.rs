use {
  crate::lexer::source::Position, derive_more::Constructor, getset::Getters,
  ordered_float::OrderedFloat, strum_macros::EnumString
};

#[derive(Debug, Constructor, Getters)]
pub struct Token<'token> {
  #[getset(get = "pub")]
  r#type: TokenType<'token>,

  #[getset(get = "pub")]
  position: Position
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'token_type> {
  // Punctuations.
  OpenParanthesis,
  CloseParanthesis,
  OpenBrace,
  CloseBrace,
  Comma,
  Dot,
  Semicolon,

  // Mathematical Operators.
  Plus,
  Minus,
  Multiply,
  Divide,

  // Comparison Operators.
  Not,
  NotEquals,
  Equals,
  GreaterThan,
  GreaterThanOrEquals,
  LessThan,
  LessThanOrEquals,

  // Literals.
  String(&'token_type str),
  Number(OrderedFloat<f64>),
  Identifier(&'token_type str),

  Keyword(Keyword),

  Assign
}

#[derive(Debug, PartialEq, Eq, EnumString)]
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
