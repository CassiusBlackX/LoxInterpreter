#include "token.h"
#include <array>
#include <cstddef>
#include <gtest/gtest.h>
#include <string>

#define SUITE TokenTest

TEST(SUITE, keyword_match) {
  std::array<TokenType, 6> keywords = {
      TokenType::And,   TokenType::Class_, TokenType::Else,
      TokenType::Var, TokenType::While,    TokenType::True,
  };
  std::array<std::string, 6> str_literal = {
    "and", "class", "else", "var", "while", "true",
  };
  std::array<const char*, 6> char_literal = {
    "and", "class", "else", "var", "while", "true",
  };
  for (size_t i = 0; i < 6; i++) {
    EXPECT_EQ(keywords[i], match_keyword(str_literal[i]));
  }
  for (size_t i = 0; i < 6; i++) {
    EXPECT_EQ(keywords[i], match_keyword(char_literal[i]));
  }
}
