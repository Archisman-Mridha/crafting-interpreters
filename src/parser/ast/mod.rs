/*
  Consider the following operation :

                                    1 + 2 * 3 - 4

  Because you understand the order of operations, you know that the multiplication is evaluated
  before the addition or subtraction. One way to visualize the evaluation precedence is using a
  tree. Leaf nodes are numbers, and interior nodes are operators with branches for each of their
  operands.

  In order to evaluate an arithmetic node, you need to know the numeric values of its subtrees, so
  you have to evaluate those first. That means working your way from the leaves up to the root :
  a post-order traversal.

  Regular languages aren’t powerful enough to handle such expressions which can nest arbitrarily
  deeply. We need a context-free grammar (CFG) : a type of formal grammar.

  A formal grammar takes a set of atomic pieces it calls its “alphabet”. Then it defines a
  (usually infinite) set of “strings” that are “in” the grammar. Each string is a sequence of
  “letters” in the alphabet.

  Here, an alphabet = a  Token.
        a  string   = an Expression.

  We create a finite set of rules. Starting with the rules, you can use them to generate strings
  that are in the grammar. Strings created this way are called derivations because each is
  “derived” from the rules of the grammar.  Rules are called productions because they produce
  strings in the grammar.

  Each production in a context-free grammar has a head, its name, and a body which describes what
  it generates. The body is simply a list of symbols. Symbols come in two delectable flavors :

    (1) A terminal is a letter from the grammar’s alphabet. You can think of it like a literal
      value.

      These are called “terminals”, in the sense of an “end point” because they don’t lead to any
      further “moves” in the game. You simply produce that one symbol.

    (2) A nonterminal is a named reference to another rule in the grammar. It means “play that rule
      and insert whatever it produces here”. In this way, the grammar composes.

      You may have multiple rules with the same name. When you reach a nonterminal with that name,
      you are allowed to pick any of the rules for it, whichever floats your boat.

  The formal grammar for Lox interpreter is defined at ./grammar.g.
*/

use crate::lexer::token::Token;

pub enum Expression<'expression> {
  Literal(Token<'expression>),
  UnaryExpression(UnaryExpression<'expression>),
  BinaryExpression(BinaryExpression<'expression>)
}

pub struct UnaryExpression<'unary_expression> {
  operator: Token<'unary_expression>,
  operand:  Box<Expression<'unary_expression>>
}

pub struct BinaryExpression<'binary_expression> {
  left_operand:  Box<Expression<'binary_expression>>,
  operator:      Token<'binary_expression>,
  right_operand: Box<Expression<'binary_expression>>
}

pub mod printer;
