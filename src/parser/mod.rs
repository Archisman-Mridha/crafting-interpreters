pub mod ast;

/*
  Parsers play that game in reverse. Given a string (a series of tokens), we map those tokens to
  terminals in the grammar to figure out which rules could have generated that string.

  The “could have” part is interesting. It’s entirely possible to create a grammar that is
  ambiguous, where different choices of productions can lead to the same string.

  The way mathematicians have addressed this ambiguity is by defining rules for precedence and
  associativity.
*/

pub struct Parser;
