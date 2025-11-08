/*
  Parsers play that game in reverse. Given a string (a series of tokens), we map those tokens to
  terminals in the grammar to figure out which rules could have generated that string.

  The “could have” part is interesting. It’s entirely possible to create a grammar that is
  ambiguous, where different choices of productions can lead to the same string.

  The way mathematicians have addressed this ambiguity is by defining rules for precedence and
  associativity.

  In the formal grammar, we define a separate rule for each precedence level.

  We'll use the Recursive Descent algorithm to implement our parser. It's considered a top-down
  algorithm because it starts from the top or outermost grammar rule (here expression) and works
  its way down into the nested subexpressions before finally reaching the leaves of the syntax
  tree. This is in contrast with bottom-up parsers like LR that start with primary expressions and
  compose them into larger and larger chunks of syntax.

  A recursive descent parser is a literal translation of the grammar’s rules straight into
  imperative code. Each rule becomes a function.
  The “recursive” part of recursive descent is because when a grammar rule refers to itself (
  directly or indirectly) that translates to a recursive function call.
*/

use {
  crate::{
    ast::{BinaryExpression, Expression, UnaryExpression},
    lexer::{
      source::Position,
      token::{Token, TokenType}
    }
  },
  std::{iter::Peekable, vec::IntoIter}
};

pub struct Parser<'parser> {
  pub(crate) tokens: Peekable<IntoIter<Token<'parser>>>,
  position:          Position
}

impl<'parser> Parser<'parser> {
  pub fn new(tokens: Vec<Token<'parser>>) -> Option<Self> {
    if tokens.is_empty() {
      return None;
    }

    let position = tokens[0].position().clone();

    Some(Self {
      tokens: tokens.into_iter().peekable(),
      position
    })
  }

  pub fn parse(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    self.parse_expression()
  }

  fn parse_expression(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    let mut left_operand = self.parse_comparison()?;

    loop {
      let operator = match self.next_if_equality_operator() {
        Some(operator) => operator,
        None => break
      };

      let right_operand = self.parse_comparison()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  fn parse_comparison(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    let mut left_operand = self.parse_additive_expression()?;

    while let Some(operator) = self.next_if_comparison_operator() {
      let right_operand = self.parse_additive_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  fn parse_additive_expression(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    let mut left_operand = self.parse_multiplicative_expression()?;

    while let Some(operator) = self.next_if_additive_operator() {
      let right_operand = self.parse_multiplicative_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  fn parse_multiplicative_expression(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    let mut left_operand = self.parse_unary_expression()?;

    while let Some(operator) = self.next_if_multiplicative_operator() {
      let right_operand = self.parse_unary_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  fn parse_unary_expression(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    match self.next_if_unary_operator() {
      Some(operator) => {
        let operand = self.parse_unary_expression()?;

        Ok(Box::new(Expression::UnaryExpression(UnaryExpression {
          operator,
          operand
        })))
      }

      _ => self.parse_paranthesized()
    }
  }

  fn parse_paranthesized(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    match self
      .tokens
      .next_if(|token| *(token.r#type()) == TokenType::OpenParanthesis)
    {
      Some(open_paranthesis) => {
        let inner = self.parse_expression()?;

        // Ensure that the closing paranthesis is there.
        if self
          .tokens
          .next_if(|token| *(token.r#type()) == TokenType::CloseParanthesis)
          .is_none()
        {
          return Err(Error {
            position: open_paranthesis.position().clone(),
            r#type:   ErrorType::ExpectedCloseParanthesis
          });
        }

        Ok(inner)
      }

      _ => self.parse_literal()
    }
  }

  fn parse_literal(&mut self) -> Result<Box<Expression<'parser>>, Error> {
    match self.tokens.next_if(|token| token.is_literal()) {
      None => Err(Error {
        position: self.position,
        r#type:   ErrorType::ExpectedLiteral
      }),

      Some(token) => Ok(Box::new(Expression::Literal(token)))
    }
  }
}

#[derive(Debug)]
pub struct Error {
  position: Position,
  r#type:   ErrorType
}

#[derive(Debug, strum::Display)]
pub enum ErrorType {
  #[strum(to_string = "invalid unary operator")]
  InvalidUnaryOperator,

  #[strum(to_string = "invalid binary operator")]
  InvalidBinaryOperator,

  #[strum(to_string = "expected a close paranthesis")]
  ExpectedCloseParanthesis,

  #[strum(to_string = "expected a literal")]
  ExpectedLiteral
}

#[cfg(test)]
mod test {
  use {
    super::*,
    crate::{ast::printer::Printer, lexer::Lexer}
  };

  #[test]
  fn test() {
    let source = "!(-1 == 2 + 3 * 4 + 5)";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex().unwrap();

    let mut parser = Parser::new(tokens).unwrap();
    let expression = parser.parse().unwrap();

    Printer::print(&expression);
  }
}

/*
  In modern IDEs and editors, the parser is constantly reparsing code—often while the user is
  still editing it, in order to syntax highlight and support things like auto-complete. That means
  it will encounter code in incomplete, half-wrong states all the time.

  There are a couple of hard requirements for when the parser runs into a syntax error :

    (1) It must detect and report the error.

    (2) It must not crash or hang.

  Once a single error is found, the parser no longer really knows what’s going on. It tries to get
  itself back on track and keep going, but if it gets confused, it may report a slew of ghost
  errors that don’t indicate other real problems in the code. These ghost errors are called
  cascaded errors. When the first error is fixed, those phantoms disappear, because they reflect
  only the parser’s own confusion. We will try to minimize these cascaded errors as well.
*/
