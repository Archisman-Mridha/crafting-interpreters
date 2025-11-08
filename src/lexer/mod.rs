use {
  crate::lexer::{
    source::{Position, Source},
    token::{Keyword, Token, TokenType}
  },
  itertools::Itertools
};

pub mod source;
pub mod token;

// The lexer takes in raw source code as a series of characters and groups it into a series of
// chunks we call tokens. These are the meaningful “words” and “punctuation” that make up the
// language’s grammar.
pub struct Lexer<'lexer> {
  source: Source<'lexer>
}

impl<'lexer> Lexer<'lexer> {
  pub fn new(source: &'lexer str) -> Self {
    Self {
      source: Source::new(source)
    }
  }

  pub fn lex(&mut self) -> Result<Vec<Token<'lexer>>, Vec<Error>> {
    // Even if an error occurs, we keep scanning. There may be other errors later in the program.
    // It gives our users a better experience if we detect as many of those as possible in one go.
    let (tokens, errors): (Vec<_>, Vec<_>) = self.by_ref().partition_result();

    if !errors.is_empty() {
      return Err(errors);
    }

    Ok(tokens)
  }
}

impl<'lexer> Iterator for Lexer<'lexer> {
  type Item = Result<Token<'lexer>, Error>;

  fn next(&mut self) -> Option<Self::Item> {
    // Ignore any leading whitespaces.
    self.consume_whitespaces();

    /*
      We go through the list of characters and group them together into the smallest sequence that
      still represent something. This blob of characters is called a lexeme.

      The rules that determine how a particular language groups characters into lexemes, is called
      the lexical grammar. In Lox, as in most programming languages, the rules of that grammar are
      simple enough to be classified a regular language. That’s the same “regular” as in regular
      expressions.

      You very precisely can recognize all of the different lexemes for Lox using regexes if you
      want to, and there’s a pile of interesting theory underlying why that is and what it means.
      Tools like Lex or Flex are designed expressly to let you do this—throw a handful of regexes
      at them, and they give you a complete lexer back.

      The lexemes are only the raw substrings of the source code. However, in the process of
      grouping character sequences into lexemes, we also stumble upon some other useful
      information. When we take the lexeme and bundle it together with that other data, the result
      is a token.
    */

    let character = self.source.peek()?;

    match character {
      '"' => self.lex_string(),
      _ if character.is_numeric() => self.lex_number(),
      _ if character.is_alphabetic() => self.lex_keyword_or_identifier(),

      _ => self.lex_symbol()
    }
  }
}

impl<'lexer> Lexer<'lexer> {
  fn lex_string(&mut self) -> Option<Result<Token<'lexer>, Error>> {
    // Consume the opening double quote.
    let (start, _) = self.source.next_if_character('"')?;

    while self.source.consume_if_not_character('"') {}

    // Determine the literal value.
    let value = &(self.source.source())[(*start.index() + 1)..*self.source.position().index()];

    // Try consuming the closing double quote.
    match self.source.next_if_character('"') {
      // Closing double quote not present.
      // So, we've encountered an unterminated string.
      None => Some(Err(Error {
        position: start,
        r#type:   ErrorType::UnterminatedString
      })),

      Some(_) => {
        let token = Token::new(TokenType::String(value), start);
        Some(Ok(token))
      }
    }
  }

  fn lex_number(&mut self) -> Option<Result<Token<'lexer>, Error>> {
    // Consume the integral part.

    let (start, _) = self.source.next_if(|character| character.is_numeric())?;

    while self.source.consume_if(|character| character.is_numeric()) {}

    // Try consuming the decimal.
    // Note that, we don’t allow a leading or trailing decimal point.
    if self.source.consume_if_character('.') {
      // Consume the fractional part.

      if self
        .source
        .next_if(|character| character.is_numeric())
        .is_none()
      {
        // No numeric character present.
        // Which means the number has no fractional part.
        return Some(Err(Error {
          position: start,
          r#type:   ErrorType::NumberHasNoFractionalPart
        }));
      };

      while self.source.consume_if(|character| character.is_numeric()) {}
    }

    // Determine the literal value.

    let value = &(self.source.source())[*start.index()..*self.source.position().index()];
    match value.parse() {
      Err(_) => Some(Err(Error {
        position: start,
        r#type:   ErrorType::FailedParsingNumber
      })),

      Ok(value) => {
        let token = Token::new(TokenType::Number(value), start);
        Some(Ok(token))
      }
    }
  }

  fn lex_keyword_or_identifier(&mut self) -> Option<Result<Token<'lexer>, Error>> {
    // The first character must be an alphabet.
    let (start, _) = self.source.next_if(|character| character.is_alphabetic())?;

    while self
      .source
      .consume_if(|character| character.is_alphanumeric() || (*character == '_'))
    {}

    let value = &(self.source.source())[(*start.index())..(*self.source.position().index())];

    let token = match Keyword::try_from(value) {
      Ok(keyword) => Token::new(TokenType::Keyword(keyword), start),

      _ => Token::new(TokenType::Identifier(value), start)
    };

    Some(Ok(token))
  }

  fn lex_symbol(&mut self) -> Option<Result<Token<'lexer>, Error>> {
    let (mut position, mut character) = self.source.next()?;

    // Ignore any comments.
    while (character == '/') && self.source.consume_if_character('/') {
      self.consume_comment();

      (position, character) = self.source.next()?;
    }

    macro_rules! make_token {
      ($token_type: expr) => {
        Token::new($token_type, position)
      };
    }

    let token = match character {
      '(' => make_token!(TokenType::OpenParanthesis),
      ')' => make_token!(TokenType::CloseParanthesis),
      '{' => make_token!(TokenType::OpenBrace),
      '}' => make_token!(TokenType::OpenBrace),
      ',' => make_token!(TokenType::Comma),
      '.' => make_token!(TokenType::Dot),
      ';' => make_token!(TokenType::Semicolon),

      '+' => make_token!(TokenType::Plus),
      '-' => make_token!(TokenType::Minus),
      '*' => make_token!(TokenType::Multiply),
      '/' => make_token!(TokenType::Divide),

      '!' if self.source.consume_if_character('=') => make_token!(TokenType::NotEquals),
      '!' => make_token!(TokenType::Not),
      '>' if self.source.consume_if_character('=') => make_token!(TokenType::GreaterThanOrEquals),
      '>' => make_token!(TokenType::GreaterThan),
      '<' if self.source.consume_if_character('=') => make_token!(TokenType::LessThanOrEquals),
      '<' => make_token!(TokenType::LessThan),
      '=' if self.source.consume_if_character('=') => make_token!(TokenType::Equals),

      '=' => make_token!(TokenType::Assign),

      // We have encountered an unrecognized character.
      _ =>
        return Some(Err(Error {
          r#type: ErrorType::InvalidCharacter,
          position
        })),
    };

    Some(Ok(token))
  }

  #[inline]
  fn consume_whitespaces(&mut self) {
    while self
      .source
      .consume_if(|character| character.is_whitespace())
    {}
  }

  #[inline]
  fn consume_comment(&mut self) {
    while self.source.consume_if_not_character('\n') {}
  }
}

#[derive(Debug)]
pub struct Error {
  position: Position,
  r#type:   ErrorType
}

#[derive(Debug, PartialEq, Eq, strum_macros::Display)]
pub enum ErrorType {
  #[strum(to_string = "invalid character")]
  InvalidCharacter,

  #[strum(to_string = "unterminated string")]
  UnterminatedString,

  #[strum(to_string = "number has no fractional part")]
  NumberHasNoFractionalPart,

  #[strum(to_string = "failed parsing number")]
  FailedParsingNumber
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_source() {
    let source = "";

    let mut lexer = Lexer::new(source);

    let tokens = lexer.lex().unwrap();
    assert!(tokens.is_empty());
  }

  #[test]
  fn unrecognized_character() {
    let source = "^";

    let mut lexer = Lexer::new(source);

    let errors = lexer.lex().unwrap_err();

    let error = &errors[0];
    assert_eq!(error.r#type, ErrorType::InvalidCharacter);
  }

  #[test]
  fn empty_string() {
    let source = "\"\"";

    let mut lexer = Lexer::new(source);

    let tokens = lexer.lex().unwrap();

    let token = &tokens[0];
    assert_eq!(*token.r#type(), TokenType::String(""));
  }

  #[test]
  fn hello_world() {
    let source = "
      fun say_hello_world( ) {
        var message = \"HELLO WORLD\";
        print message;
      }

      say_hello_world( )
    ";

    let mut lexer = Lexer::new(source);

    assert!(lexer.lex().is_ok());
  }
}
