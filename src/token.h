#include <cassert>
#include <cstddef>
#include <ostream>
#include <string>
#include <string_view>
#include <variant>

#ifndef TOKEN_H_
#define TOKEN_H_

// LiteralType is copiable(shallo copy is complete)
class LiteralType {
public:
  enum class Type {
    Identifer,
    String,
    Bool,
    Nil,
    Number,
  };

  LiteralType() : value_(nullptr), type_(Type::Nil) {}
  LiteralType(double d) : value_(d), type_(Type::Number) {}
  LiteralType(bool b) : value_(b), type_(Type::Bool) {}
  LiteralType(std::string_view str, Type ty) : value_(str), type_(ty) {
    assert(ty == Type::String || ty == Type::Identifer);
  }

  bool operator==(const LiteralType &other) const;
  std::string to_string() const;

  friend std::ostream &operator<<(std::ostream &os, LiteralType literal) {
    os << literal.to_string();
    return os;
  }

private:
  std::variant<double, bool, std::string_view, std::nullptr_t> value_;
  Type type_;
};

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

constexpr std::string_view to_string(TokenType tk_type);
std::ostream &operator<<(std::ostream &os, TokenType tk_type);

TokenType match_keyword(std::string_view);

class Token {
public:
  Token(TokenType type, std::string_view lexeme, size_t line);

  friend std::ostream &operator<<(std::ostream &os, const Token &token) {
    os << token.type << ' ' << token.lexeme << ' ' << token.literal;
    return os;
  }

  TokenType get_tokentype() const { return type; }
  std::string_view get_lexeme() const { return lexeme; }
  size_t get_line() const { return line; }
  LiteralType get_literal() const { return literal; }

private:
  TokenType type;
  // lexeme must not be modified or else LiteralType
  // might be dangling!
  const std::string lexeme;
  LiteralType literal;
  size_t line;
};

#endif // TOKEN_H_
