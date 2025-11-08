use {
  crate::{
    ast::parser::Parser,
    lexer::token::{Token, TokenType}
  },
  getset::Getters,
  paste::paste
};

#[derive(Debug, Getters)]
pub struct Operator<'operator> {
  #[getset(get = "pub")]
  precedance: Precedance,

  token: Token<'operator>
}

#[derive(Debug, strum_macros::Display)]
pub enum Precedance {
  #[strum(to_string = "{0}")]
  Unary(Unary),

  #[strum(to_string = "{0}")]
  Multiplicative(Multiplicative),

  #[strum(to_string = "{0}")]
  Additive(Additive),

  #[strum(to_string = "{0}")]
  Comparison(Comparison),

  #[strum(to_string = "{0}")]
  Equality(Equality)
}

macro_rules! create_precedance {
  ($name:ident { $($variant:ident),+ }) => {
    paste!{

      #[derive(Debug, strum_macros::Display)]
      pub enum $name {
        $($variant),+
      }

      impl<'a> $name {
        pub fn try_from(token_type: &TokenType<'a>) -> Option<Self> {
          Some(match token_type {
            $(
              TokenType::$variant => Self::$variant,
            )+

            _ => return None
          })
        }
      }

      // Corresponding to each precedance level, we define a utility method for Parser.
      // For e.g., corresponding to the equality precedance level, the next_if_comparison_operator
      // method will be defined. As you can infer from the name, it'll progress the tokens
      // iterator if the next token is of type comparison operator.
      impl<'parser> Parser<'parser> {
        pub(crate) fn [<next_if_ $name:lower _operator>](&mut self) -> Option<Operator<'parser>> {
          let token = self.tokens.peek()?;

          let variant = $name::try_from(token.r#type())?;

          Some(Operator {
            precedance: Precedance::$name(variant),
            token: self.tokens.next()?,
          })
        }
      }

    }
  };
}

create_precedance!(Unary { Minus, Not });

create_precedance!(Multiplicative { Multiply, Divide });

create_precedance!(Additive { Plus, Minus });

create_precedance!(Comparison {
  GreaterThan,
  GreaterThanOrEquals,
  LessThan,
  LessThanOrEquals
});

create_precedance!(Equality { Equals, NotEquals });
