use crate::ast::Expression;

pub struct Printer;

impl<'expression> Printer {
  pub fn print(expression: &Expression<'expression>) {
    println!("root");
    Self::inner(expression, "", true);
  }

  fn inner(expression: &Expression<'expression>, prefix: &str, is_last_child: bool) {
    // Determine the indentation that visually connects this node with the parent node.
    let connector = if !is_last_child { "├── " } else { "└── " };

    match expression {
      Expression::Literal(token) => {
        println!("{prefix}{connector}{}", token.r#type());
      }

      Expression::UnaryExpression(unary_expression) => {
        // Print the unary operator.
        let unary_operator_type = unary_expression.operator.precedance();
        println!("{prefix}{connector}{unary_operator_type}");

        // Print the operand as a child node.

        let child_prefix = format!("{prefix}{}", if is_last_child { "    " } else { "│   " });

        Self::inner(&unary_expression.operand, &child_prefix, true);
      }

      Expression::BinaryExpression(binary_expression) => {
        // Print the binary operator.
        let binary_operator_type = binary_expression.operator.precedance();
        println!("{prefix}{connector}{binary_operator_type}");

        // Print the operands as child nodes.

        let child_prefix = format!("{prefix}{}", if is_last_child { "    " } else { "│   " });

        Self::inner(&binary_expression.left_operand, &child_prefix, false);
        Self::inner(&binary_expression.right_operand, &child_prefix, true);
      }
    }
  }
}
