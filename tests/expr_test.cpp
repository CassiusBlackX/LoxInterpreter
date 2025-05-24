#include <gtest/gtest.h>

#include "expr.h"

TEST(AstPrinter, simple_expression) {
  Expr* expr = new Binary(
    new Unary(
      Token(TokenType::Minus, "-", 1),
      new Literal(LiteralType(123.456))
    ),
    Token(TokenType::Star, "*", 1),
    new Grouping(
      new Literal(LiteralType(987.654))
    )
  );
  std::string got = expr->print();
  std::string expected = "(* (- 123.456) (group 987.654))";
  EXPECT_EQ(got, expected);
}
