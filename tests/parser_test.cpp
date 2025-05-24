#include <gtest/gtest.h>
#include <vector>

#include "parser.h"

// class ParserTester : public testing::Test {
// protected:
//   void initialize(const std::vector<Token> &tokens) {
//     parser = Parser(tokens);
//     got = parser.parse();
//   }
//
//   void parser_tester(Expr *expected) {
//     if (expected == nullptr)
//       return;
//     ASSERT_EQ(typeid(*expected), typeid(*got));
//
//     if (auto binary = dynamic_cast<Binary *>(expected)) {
//       Expr *current = got;
//       got = dynamic_cast<Binary *>(current)->left;
//       parser_tester(binary->left);
//       got = dynamic_cast<Binary *>(current)->right;
//     } else if (auto unary = dynamic_cast<Unary *>(expected)) {
//       Expr *current = got;
//       EXPECT_EQ(dynamic_cast<Unary *>(expected)->op,
//                 dynamic_cast<Unary *>(current)->op);
//       got = dynamic_cast<Unary *>(current)->right;
//       parser_tester(unary->right);
//     } else if (auto grouping = dynamic_cast<Grouping *>(expected)) {
//       Expr *current = got;
//       got = dynamic_cast<Grouping *>(current)->expr;
//       parser_tester(grouping->expr);
//     } else if (auto literal = dynamic_cast<Literal *>(expected)) {
//       EXPECT_EQ(literal->value, dynamic_cast<Literal *>(got)->value);
//     }
//   }
//
// protected:
//   Parser parser;
//   Expr *got;
// };

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
