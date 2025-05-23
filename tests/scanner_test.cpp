#include <cstddef>
#include <gtest/gtest.h>
#include <string>
#include <vector>

#include "scanner.h"

#define SUITE ScannerTest

class ScannerTester : public testing::Test {
protected:
  void initialize(const std::string &content) { scanner = Scanner(content); }

  void tokens_tester(const std::vector<Token> &expected) {
    std::vector<Token> tokens = scanner.scan_tokens();
    ASSERT_EQ(expected.size(), tokens.size());
    for (size_t i = 0; i < expected.size(); i++) {
      const Token &tk_e = expected[i];
      const Token &tk_g = tokens[i];
      EXPECT_EQ(tk_e.get_line(), tk_g.get_line());
      EXPECT_EQ(tk_e.get_tokentype(), tk_g.get_tokentype());
      EXPECT_EQ(tk_e.get_lexeme(), tk_g.get_lexeme());
      EXPECT_EQ(tk_e.get_literal(), tk_g.get_literal());
    }
  }

protected:
  Scanner scanner;
};

TEST_F(ScannerTester, scan_operator) {
  std::string content = "(*!) != <= == =;";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::LeftParen, "(", Literal(), 1);
  expected.emplace_back(TokenType::Star, "*", Literal(), 1);
  expected.emplace_back(TokenType::Bang, "!", Literal(), 1);
  expected.emplace_back(TokenType::RightParen, ")", Literal(), 1);
  expected.emplace_back(TokenType::BangEqual, "!=", Literal(), 1);
  expected.emplace_back(TokenType::LessEqual, "<=", Literal(), 1);
  expected.emplace_back(TokenType::EqualEqual, "==", Literal(), 1);
  expected.emplace_back(TokenType::Equal, "=", Literal(), 1);
  expected.emplace_back(TokenType::SemiColon, ";", Literal(), 1);
  expected.emplace_back(TokenType::Eof, "", Literal(), 1);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_special_ascii) {
  std::string content = "a\r\t\nb  \"happy\"//nothing\nc";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Identifier, "a",
                        Literal("a", Literal::Type::Identifer), 1);
  expected.emplace_back(TokenType::Identifier, "b",
                        Literal("b", Literal::Type::Identifer), 2);
  expected.emplace_back(TokenType::String, "\"happy\"",
                        Literal("happy", Literal::Type::String), 2);
  expected.emplace_back(TokenType::Identifier, "c",
                        Literal("c", Literal::Type::Identifer), 3);
  expected.emplace_back(TokenType::Eof, "", Literal(), 3);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_number) {
  std::string content = "123456\r\n 123.456";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Number, "123456",
                        Literal(static_cast<double>(123456)), 1);
  expected.emplace_back(TokenType::Number, "123.456",
                        Literal(static_cast<double>(123.456)), 2);
  expected.emplace_back(TokenType::Eof, "", Literal(), 2);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_keyword) {
  std::string content = "fun if funny \n false classifier class \rreturn";
  // std::string content = "fun if \n false class \rreturn";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Fun, "fun", Literal(), 1);
  expected.emplace_back(TokenType::If, "if", Literal(), 1);
  expected.emplace_back(TokenType::Identifier, "funny",
                        Literal("funny", Literal::Type::Identifer), 1);
  expected.emplace_back(TokenType::False, "false", Literal(false), 2);
  expected.emplace_back(TokenType::Identifier, "classifier",
                        Literal("classifier", Literal::Type::Identifer), 2);
  expected.emplace_back(TokenType::Class_, "class", Literal(), 2);
  expected.emplace_back(TokenType::Return, "return", Literal(), 2);
  expected.emplace_back(TokenType::Eof, "", Literal(), 2);
  tokens_tester(expected);
}
