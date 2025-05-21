use crate::{
    expression::{Expr, Literal, Visitor},
    token::LiteralType,
};

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push_str(&format!("({}", name));
        for expr in exprs.iter() {
            result.push(' ');
            let s = expr.accept(self);
            result.push_str(&s);
        }
        result.push(')');
        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&self, expr: &crate::expression::Literal) -> String {
        expr.value.to_string()
    }

    fn visit_grouping(&self, expr: &crate::expression::Grouping) -> String {
        self.parenthesize("group", &[&expr.expr])
    }

    fn visit_unary(&self, expr: &crate::expression::Unary) -> String {
        self.parenthesize(expr.operator.get_lexeme(), &[&expr.right])
    }

    fn visit_binary(&self, expr: &crate::expression::Binary) -> String {
        self.parenthesize(expr.operator.get_lexeme(), &[&expr.left, &expr.right])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expression::*;
    use crate::token::*;

    fn ast_printer_example() -> String {
        // -123.456 * (987.654)
        let example = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralType::Number(123.456),
                })),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping(Grouping {
                expr: Box::new(Expr::Literal(Literal {
                    value: LiteralType::Number(987.654),
                })),
            })),
        });
        AstPrinter.print(&example)
    }

    #[test]
    fn test_ast_printer() {
        let s = ast_printer_example();
        assert_eq!(s, "(* (- 123.456) (group 987.654))");
    }
}
