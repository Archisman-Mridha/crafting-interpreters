/*
  Operator precedance is the same as that in C :

    (1) -, !
    (2) /, *
    (3) +, -
    (4) >, >=, <, <=
    (5) ==, !=

  We define a separate rule for each precedence level.
*/

expression -> equality;

equality -> comparison (( "==" | "!=" ) comparison)*;

comparison -> additive (( ">" | ">=" | "<" | "<=" ) additive)*;

additive -> multiplicative (( "+" | "-" ) multiplicative)*;

multiplicative -> unary (( "*" | "/") unary)*;

unary -> ( "-" | "!" ) unary
       | primary;

primary -> "(" expression ")"
         | literal;

literal -> NUMBER | STRING | BOOLEAN | "nil";
