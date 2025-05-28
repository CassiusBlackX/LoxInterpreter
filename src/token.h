#ifndef TOKEN_H_
#define TOKEN_H_

#include <cassert>
#include <cstddef>
#include <ostream>
#include <string>
#include <string_view>

#include "object.h"

enum class TokenType {
  // single character tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  SemiColon,
  Slash,
  Star,
  // one or two character tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  // literals
  Identifier,
  String,
  Number,
  // keywords
  And,
  Class_,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
  Invalid,
};

std::string_view tk_type_to_string(TokenType tk_type);
std::ostream &operator<<(std::ostream &os, TokenType tk_type);

TokenType match_keyword(std::string_view);

class Token {
public:
  Token(TokenType type, std::string_view lexeme, size_t line);

  friend std::ostream &operator<<(std::ostream &os, const Token &token) {
    os << token.type << ' ' << token.lexeme << ' ' << token.literal;
    return os;
  }

  std::string to_string() const {
    std::string_view tk_type_name = tk_type_to_string(type);
    return std::string(tk_type_name) + " " + lexeme + " " + literal.to_string();
  }

  TokenType get_tokentype() const { return type; }
  std::string get_lexeme() const { return lexeme; }
  size_t get_line() const { return line; }
  Object get_literal() const { return literal; }

private:
  TokenType type;
  // lexeme must not be modified or else LiteralType
  // might be dangling!
  const std::string lexeme;
  Object literal;
  size_t line;
};

#endif // TOKEN_H_
