#include <array>
#include <cassert>
#include <cstddef>
#include <iomanip>
#include <ostream>
#include <string>
#include <string_view>
#include <type_traits>
#include <variant>

#ifndef TOKEN_H_
#define TOKEN_H_

class Literal {
public:
  enum class Type {
    Identifer,
    String,
    Bool,
    Nil,
    Number,
  };

  Literal() : value_(nullptr), type_(Type::Nil) {}
  Literal(double d) : value_(d), type_(Type::Number) {}
  Literal(bool b) : value_(b), type_(Type::Bool) {}
  Literal(std::string str, Type ty) : value_(std::move(str)), type_(ty) {
    assert(ty == Type::String || ty == Type::Identifer);
  }

  std::string to_string() const {
    return std::visit(
        [](auto &&arg) -> std::string {
          using T = std::decay_t<decltype(arg)>;
          if constexpr (std::is_same_v<T, double>)
            return std::to_string(arg);
          else if constexpr (std::is_same_v<T, bool>)
            return arg ? "true" : "false";
          else if constexpr (std::is_same_v<T, std::nullptr_t>)
            return "nil";
          else if constexpr (std::is_same_v<T, std::string>)
            return "\"" + arg + "\"";
        },
        value_);
  }

  friend std::ostream &operator<<(std::ostream &os, Literal literal) {
    os << literal.to_string();
    return os;
  }

private:
  std::variant<double, bool, std::string, std::nullptr_t> value_;
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

std::string_view to_string(TokenType tk_type);
std::ostream &operator<<(std::ostream &os, TokenType tk_type);

TokenType match_keyword(const std::string&);

class Token {
public:
  Token(TokenType type, std::string lexeme, Literal literal, int line)
      : type(type), lexeme(lexeme), literal(literal), line(line) {}

  friend std::ostream &operator<<(std::ostream &os, const Token &token) {
    os << token.type << ' ' << token.lexeme << ' ' << token.literal;
    return os;
  }

  TokenType get_tokentype() { return type; }

private:
  TokenType type;
  std::string lexeme;
  Literal literal;
  int line;
};

#endif // TOKEN_H_
