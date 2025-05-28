#include "token.h"
#include "error.h"

#include <cmath>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string_view>

constexpr std::array<std::string_view,
                     static_cast<std::size_t>(TokenType::Invalid)>
    TOKEN_TYPE_NAMES = {
        "LeftParen", "RightParen",   "LeftBrace", "RightBrace", "Comma",
        "Dot",       "Minus",        "Plus",      "SemiColon",  "Slash",
        "Star",      "Bang",         "BangEqual", "Equal",      "EqualEqual",
        "Greater",   "GreaterEqual", "Less",      "LessEqual",  "Identifier",
        "String",    "Number",       "And",       "Class_",     "Else",
        "False",     "Fun",          "For",       "If",         "Nil",
        "Or",        "Print",        "Return",    "Super",      "This",
        "True",      "Var",          "While",     "Eof",
};

std::string_view tk_type_to_string(TokenType tk_type) {
  return TOKEN_TYPE_NAMES.at(static_cast<std::size_t>(tk_type));
}

std::ostream &operator<<(std::ostream &os, TokenType tk_type) {
  os << tk_type_to_string(tk_type);
  return os;
}

constexpr uint64_t my_hash(std::string_view str) {
  uint64_t result = 0;
  for (auto c : str) {
    result = result * 26 + static_cast<uint32_t>(c);
  }
  return result;
}

TokenType match_keyword(std::string_view str) {
  switch (my_hash(str)) {
  case my_hash("and"):
    return TokenType::And;
  case my_hash("class"):
    return TokenType::Class_;
  case my_hash("else"):
    return TokenType::Else;
  case my_hash("false"):
    return TokenType::False;
  case my_hash("for"):
    return TokenType::For;
  case my_hash("fun"):
    return TokenType::Fun;
  case my_hash("if"):
    return TokenType::If;
  case my_hash("nil"):
    return TokenType::Nil;
  case my_hash("or"):
    return TokenType::Or;
  case my_hash("print"):
    return TokenType::Print;
  case my_hash("return"):
    return TokenType::Return;
  case my_hash("super"):
    return TokenType::Super;
  case my_hash("this"):
    return TokenType::This;
  case my_hash("true"):
    return TokenType::True;
  case my_hash("var"):
    return TokenType::Var;
  case my_hash("while"):
    return TokenType::While;
  default:
    return TokenType::Identifier;
  }
}

static double sv_to_double(std::string_view sv) {
  double result = NAN;
  auto [ptr, ec] = std::from_chars(sv.data(), sv.data() + sv.size(), result);
  if (ec != std::errc()) {
    throw std::invalid_argument("Invalid double conversion!");
  }
  return result;
}

Token::Token(TokenType type, std::string_view lexeme_, size_t line)
    : type(type), lexeme(lexeme_), line(line) {
  switch (type) {
  case TokenType::String:
    literal =
        Object(std::string(this->lexeme.data() + 1, this->lexeme.size() - 2),
               Object::Type::String);
    break;
  case TokenType::Identifier:
    literal = Object(std::string(this->lexeme), Object::Type::Identifer);
    break;
  case TokenType::Number: {
    double value = NAN;
    try {
      value = sv_to_double(lexeme_);
    } catch (std::invalid_argument) {
      error(line, "failed to tokenize double when parsing!");
    }
    literal = Object(value);
    break;
  }
  case TokenType::True:
    literal = Object(true);
    break;
  case TokenType::False:
    literal = Object(false);
    break;
  default:
    literal = Object();
    break;
  }
}
