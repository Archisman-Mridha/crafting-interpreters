/*
  Operator precedance is the same as that in C :

    (1) -, !
    (2) /, *
    (3) +, -
    (4) >, >=, <, <=
    (5) ==, !=
*/

expression -> equality;

equality -> comparison (("==" | "!=") comparison)*;

comparison -> additive-expression ((">" | ">=" | "<" | "<=") additive-expression)*;

additive-expression -> multiplicative-expression (("+" | "-") multiplicative-expression)*;

multiplicative-expression -> unary-expression (("*" | "/") unary-expression)*;

unary-expression -> ("-" | "!") unary-expression
                  | paranthesized;

paranthesized -> "(" expression ")"
               | literal;

literal -> NUMBER | STRING | ("true" | "false") | "nil";
