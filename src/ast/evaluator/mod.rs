use crate::{
  ast::{
    Expression,
    evaluator::value::Value,
    operator::{Additive, Comparison, Equality, Multiplicative, Precedance, Unary}
  },
  lexer::{
    source::Position,
    token::{Keyword, TokenType}
  }
};

pub struct Evaluator;

impl<'expression> Evaluator {
  pub fn evaluate(expression: Expression<'expression>) -> Result<Value<'expression>, ()> {
    Ok(match expression {
      Expression::UnaryExpression(expression) => match expression.operator.precedance() {
        Precedance::Unary(variant) => match variant {
          Unary::Minus => todo!(),
          Unary::Not => todo!()
        },

        _ => unreachable!()
      },

      Expression::BinaryExpression(expression) => match expression.operator.precedance() {
        Precedance::Multiplicative(variant) => match variant {
          Multiplicative::Multiply => todo!(),
          Multiplicative::Divide => todo!()
        },

        Precedance::Additive(variant) => match variant {
          Additive::Plus => todo!(),
          Additive::Minus => todo!()
        },

        Precedance::Comparison(variant) => match variant {
          Comparison::GreaterThan => todo!(),
          Comparison::GreaterThanOrEquals => todo!(),
          Comparison::LessThan => todo!(),
          Comparison::LessThanOrEquals => todo!()
        },

        Precedance::Equality(variant) => match variant {
          Equality::Equals => todo!(),
          Equality::NotEquals => todo!()
        },

        _ => unreachable!()
      },

      Expression::Literal(token) => match token.r#type() {
        TokenType::Number(number) => Value::Number(*number),

        TokenType::String(string) => Value::String(string),

        TokenType::Keyword(Keyword::True) => Value::Boolean(true),
        TokenType::Keyword(Keyword::False) => Value::Boolean(false),

        _ => unreachable!()
      }
    })
  }
}

#[derive(Debug)]
pub struct Error {
  position: Position,
  r#type:   ErrorType
}

#[derive(Debug, strum::Display)]
pub enum ErrorType {}

pub mod value;
