#include <cstddef>
#include <gtest/gtest.h>
#include <initializer_list>
#include <string>
#include <string_view>
#include <vector>

#include "scanner.h"

#define SUITE ScannerTest

// Theoratically, using initializer_list is a more elegant way
// but it is unnecessary to change the written code
// so I keep them the way they were
static void compare(const std::vector<Token> tokens,
                    std::initializer_list<Token> expected) {
  ASSERT_EQ(tokens.size(), expected.size());
  size_t i = 0;
  for (auto tk_e : expected) {
    const Token &tk_g = tokens[i++];
    EXPECT_EQ(tk_e.get_line(), tk_g.get_line());
    EXPECT_EQ(tk_e.get_tokentype(), tk_g.get_tokentype());
    EXPECT_EQ(tk_e.get_lexeme(), tk_g.get_lexeme());
    EXPECT_EQ(tk_e.get_literal(), tk_g.get_literal());
  }
}

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
  expected.emplace_back(TokenType::LeftParen, "(", 1);
  expected.emplace_back(TokenType::Star, "*", 1);
  expected.emplace_back(TokenType::Bang, "!", 1);
  expected.emplace_back(TokenType::RightParen, ")", 1);
  expected.emplace_back(TokenType::BangEqual, "!=", 1);
  expected.emplace_back(TokenType::LessEqual, "<=", 1);
  expected.emplace_back(TokenType::EqualEqual, "==", 1);
  expected.emplace_back(TokenType::Equal, "=", 1);
  expected.emplace_back(TokenType::SemiColon, ";", 1);
  expected.emplace_back(TokenType::Eof, "", 1);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_special_ascii) {
  std::string content = "a\r\t\nb  \"happy\"//nothing\nc";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Identifier, "a", 1);
  expected.emplace_back(TokenType::Identifier, "b", 2);
  expected.emplace_back(TokenType::String, "\"happy\"", 2);
  expected.emplace_back(TokenType::Identifier, "c", 3);
  expected.emplace_back(TokenType::Eof, "", 3);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_number) {
  std::string content = "123456\r\n 123.456";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Number, "123456", 1);
  expected.emplace_back(TokenType::Number, "123.456", 2);
  expected.emplace_back(TokenType::Eof, "", 2);
  tokens_tester(expected);
}

TEST_F(ScannerTester, scan_keyword) {
  std::string content = "fun if funny \n false classifier class \rreturn";
  // std::string content = "fun if \n false class \rreturn";
  initialize(content);
  std::vector<Token> expected;
  expected.emplace_back(TokenType::Fun, "fun", 1);
  expected.emplace_back(TokenType::If, "if", 1);
  expected.emplace_back(TokenType::Identifier, "funny", 1);
  expected.emplace_back(TokenType::False, "false", 2);
  expected.emplace_back(TokenType::Identifier, "classifier", 2);
  expected.emplace_back(TokenType::Class_, "class", 2);
  expected.emplace_back(TokenType::Return, "return", 2);
  expected.emplace_back(TokenType::Eof, "", 2);
  tokens_tester(expected);
}

TEST(SUITE, scanner_lifetime) {
  std::string_view original = "fun if funny (100) else 98799.99 !=; <= + ,\n "
                              "\"hello world\" * // this is comment\nreturn";
  std::vector<Token> result;
  {
    std::string content(original);
    Scanner scanner(content);
    result = scanner.scan_tokens();
    // scanner is destoyed, so is content
  }
  compare(result, {
                      Token(TokenType::Fun, "fun", 1),
                      Token(TokenType::If, "if", 1),
                      Token(TokenType::Identifier, "funny", 1),
                      Token(TokenType::LeftParen, "(", 1),
                      Token(TokenType::Number, "100", 1),
                      Token(TokenType::RightParen, ")", 1),
                      Token(TokenType::Else, "else", 1),
                      Token(TokenType::Number, "98799.99", 1),
                      Token(TokenType::BangEqual, "!=", 1),
                      Token(TokenType::SemiColon, ";", 1),
                      Token(TokenType::LessEqual, "<=", 1),
                      Token(TokenType::Plus, "+", 1),
                      Token(TokenType::Comma, ",", 1),
                      Token(TokenType::String, "\"hello world\"", 2),
                      Token(TokenType::Star, "*", 2),
                      Token(TokenType::Return, "return", 3),
                      Token(TokenType::Eof, "", 3),
                  });
}
