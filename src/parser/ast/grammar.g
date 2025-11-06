expression -> literal
            | unary-expression
            | binary-expression
            | grouped-expression

literal -> NUMBER | STRING | BOOLEAN | "nil";

grouped-expression -> "(" expression ")";

unary-expression -> unary-operator expression;

unary-operator -> "-" | "!";

binary-expression -> expression binary-operator expression;

binary-operator -> "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-"  | "*" | "/" ;
