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
      token::{Keyword, Token, TokenType}
    }
  },
  std::{iter::Peekable, slice::Iter},
  strum::Display
};

pub struct Parser<'parser> {
  tokens:   Peekable<Iter<'parser, Token<'parser>>>,
  position: &'parser Position
}

impl<'parser> Parser<'parser> {
  pub fn new(tokens: &'parser [Token<'parser>]) -> Option<Self> {
    if tokens.is_empty() {
      return None;
    }

    Some(Self {
      tokens:   tokens.iter().peekable(),
      position: tokens[0].position()
    })
  }

  pub fn parse(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    self.parse_expression()
  }

  fn parse_expression(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    self.parse_equality()
  }

  pub fn parse_equality(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    let mut left_operand = self.parse_comparison()?;

    while let Some(operator) = self.tokens.next_if(|token| token.is_equality_operator()) {
      let right_operand = self.parse_comparison()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  pub fn parse_comparison(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    let mut left_operand = self.parse_additive_expression()?;

    while let Some(operator) = self.tokens.next_if(|token| token.is_comparison_operator()) {
      let right_operand = self.parse_additive_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  pub fn parse_additive_expression(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    let mut left_operand = self.parse_multiplicative_expression()?;

    while let Some(operator) = self.tokens.next_if(|token| token.is_additive_operator()) {
      let right_operand = self.parse_multiplicative_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  pub fn parse_multiplicative_expression(
    &mut self
  ) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    let mut left_operand = self.parse_unary_expression()?;

    while let Some(operator) = self
      .tokens
      .next_if(|token| token.is_multiplicative_operator())
    {
      let right_operand = self.parse_unary_expression()?;

      left_operand = Box::new(Expression::BinaryExpression(BinaryExpression {
        left_operand,
        operator,
        right_operand
      }))
    }

    Ok(left_operand)
  }

  pub fn parse_unary_expression(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
    match self.tokens.next_if(|token| token.is_unary_operator()) {
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

  pub fn parse_paranthesized(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
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
            position: open_paranthesis.position(),
            r#type:   ErrorType::ExpectedCloseParanthesis
          });
        }

        Ok(inner)
      }

      _ => self.parse_literal()
    }
  }

  pub fn parse_literal(&mut self) -> Result<Box<Expression<'parser>>, Error<'parser>> {
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
pub struct Error<'error> {
  position: &'error Position,
  r#type:   ErrorType
}

#[derive(Debug, Display)]
pub enum ErrorType {
  #[strum(to_string = "expected a close paranthesis")]
  ExpectedCloseParanthesis,

  #[strum(to_string = "expected a literal")]
  ExpectedLiteral
}

impl<'token> Token<'token> {
  fn is_equality_operator(&self) -> bool {
    matches!(self.r#type(), TokenType::Equals | TokenType::NotEquals)
  }

  fn is_comparison_operator(&self) -> bool {
    matches!(
      self.r#type(),
      TokenType::GreaterThan
        | TokenType::GreaterThanOrEquals
        | TokenType::LessThan
        | TokenType::LessThanOrEquals
    )
  }

  fn is_additive_operator(&self) -> bool {
    matches!(self.r#type(), TokenType::Plus | TokenType::Minus)
  }

  fn is_multiplicative_operator(&self) -> bool {
    matches!(self.r#type(), TokenType::Multiply | TokenType::Divide)
  }

  fn is_unary_operator(&self) -> bool {
    matches!(self.r#type(), TokenType::Minus | TokenType::Not)
  }

  fn is_literal(&self) -> bool {
    matches!(
      self.r#type(),
      TokenType::String(_)
        | TokenType::Number(_)
        | TokenType::Keyword(Keyword::True | Keyword::False | Keyword::Nil)
    )
  }
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

    let mut parser = Parser::new(&tokens).unwrap();
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
