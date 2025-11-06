use {
  getset::Getters,
  std::{fmt::Display, iter::Peekable, rc::Rc, str::Chars}
};

#[derive(Getters)]
pub struct Source<'source> {
  #[getset(get = "pub")]
  source: &'source str,

  characters: Peekable<Chars<'source>>,

  #[getset(get = "pub")]
  position: Position
}

impl<'source> Source<'source> {
  pub fn new(source: &'source str) -> Self {
    Self {
      source,
      characters: source.chars().peekable(),
      position: Position::default()
    }
  }

  pub fn peek(&mut self) -> Option<&char> {
    self.characters.peek()
  }

  pub fn next_if(&mut self, predicate: impl FnOnce(&char) -> bool) -> Option<(Position, char)> {
    let next_character = self.characters.peek()?;

    let preicate_satisfied = predicate(next_character);

    if !preicate_satisfied {
      return None;
    }

    // The next character satisfies the predicate.
    // So, let's consume the next item from this Source iterator.
    self.next()
  }

  #[inline]
  pub fn consume_if(&mut self, predicate: impl FnOnce(&char) -> bool) -> bool {
    self.next_if(predicate).is_some()
  }

  #[inline]
  pub fn next_if_character(&mut self, expected: char) -> Option<(Position, char)> {
    self.next_if(|character| *character == expected)
  }

  #[inline]
  pub fn consume_if_character(&mut self, expected: char) -> bool {
    self.next_if_character(expected).is_some()
  }

  #[inline]
  pub fn next_if_not_character(&mut self, expected: char) -> Option<(Position, char)> {
    self.next_if(|character| *character != expected)
  }

  #[inline]
  pub fn consume_if_not_character(&mut self, expected: char) -> bool {
    self.next_if_not_character(expected).is_some()
  }
}

impl<'source> Iterator for Source<'source> {
  type Item = (Position, char);

  fn next(&mut self) -> Option<Self::Item> {
    let position = self.position.clone();
    let character = self.characters.next()?;

    // Update the position tracker.
    match character {
      '\n' => self.position.move_to_next_line(),
      _ => self.position.move_to_next_column()
    }

    Some((position, character))
  }
}

#[derive(Debug, Default, Clone, Copy, Getters)]
pub struct Position {
  line:   usize,
  column: usize,

  #[getset(get = "pub")]
  index: usize
}

impl Position {
  fn move_to_next_column(&mut self) {
    self.column += 1;

    // Also, increment the index.
    self.index += 1;
  }

  fn move_to_next_line(&mut self) {
    self.line += 1;
    self.column = 0;

    // Also, increment the index.
    self.index += 1;
  }
}

impl Display for Position {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "line {}, column {}", self.line, self.column)
  }
}
