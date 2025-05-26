#include <gtest/gtest.h>

#include "environment.h"
#include "expr.h"

#define SUITE ExprTest

TEST(SUITE, ast_printer) {
  Expr *expr = new Binary(new Unary(Token(TokenType::Minus, "-", 1),
                                    new Literal(LiteralType(123.456))),
                          Token(TokenType::Star, "*", 1),
                          new Grouping(new Literal(LiteralType(987.654))));
  std::string got = expr->print();
  std::string expected = "(* (- 123.456) (group 987.654))";
  EXPECT_EQ(got, expected);
  delete_expr(expr);
}

TEST(SUITE, expr_interpret_double) {
  Expr *expr = new Binary(new Unary(Token(TokenType::Minus, "-", 1),
                                    new Literal(LiteralType(123.456))),
                          Token(TokenType::Star, "*", 1),
                          new Grouping(new Literal(LiteralType(987.654))));
  LiteralType value = expr->evaluate(new Environment());
  EXPECT_EQ(value, LiteralType(-123.456 * 987.654));
  delete_expr(expr);
}

TEST(SUITE, expr_interpret_bool) {
  // true == ( 0 == (1.0 - 1.00))
  Expr *expr = new Binary(
      new Literal(LiteralType(true)), Token(TokenType::EqualEqual, "==", 1),
      new Grouping(new Binary(
          new Literal(LiteralType(0.0)), Token(TokenType::EqualEqual, "==", 1),
          new Grouping(new Binary(new Literal(LiteralType(1.0)),
                                  Token(TokenType::Minus, "-", 1),
                                  new Literal(LiteralType(1.00)))))));

  LiteralType value = expr->evaluate(new Environment());
  EXPECT_EQ(value, LiteralType(true));
  delete_expr(expr);
}

TEST(SUITE, expr_interpret_unequal) {
  // true == ( 1 != (1.5 - 1.00))
  Expr *expr = new Binary(
      new Literal(LiteralType(true)), Token(TokenType::EqualEqual, "==", 1),
      new Grouping(new Binary(
          new Literal(LiteralType(1.0)), Token(TokenType::BangEqual, "!=", 1),
          new Grouping(new Binary(new Literal(LiteralType(1.5)),
                                  Token(TokenType::Minus, "-", 1),
                                  new Literal(LiteralType(1.00)))))));

  LiteralType value = expr->evaluate(new Environment());
  EXPECT_EQ(value, LiteralType(true));
  delete_expr(expr);
}

TEST(SUITE, expr_interpret_string_plus) {
  // "hello" + (" " + "world")
  Expr *expr = new Binary(
      new Literal(LiteralType("hello", LiteralType::Type::String)),
      Token(TokenType::Plus, "+", 1),
      new Grouping(new Binary(
          new Literal(LiteralType(" ", LiteralType::Type::String)),
          Token(TokenType::Plus, "+", 1),
          new Literal(LiteralType("world", LiteralType::Type::String)))));

  LiteralType interpreted_result = expr->evaluate(new Environment());
  LiteralType expected = LiteralType("hello world", LiteralType::Type::String);
  EXPECT_EQ(interpreted_result, expected);
  delete_expr(expr);
}

TEST(SUITE, expr_interpret_nil_bool) {
  // !nil
  Expr *expr =
      new Unary(Token(TokenType::Bang, "!", 1), new Literal(LiteralType()));
  LiteralType value = expr->evaluate(new Environment());
  EXPECT_EQ(value, LiteralType(true));
}
