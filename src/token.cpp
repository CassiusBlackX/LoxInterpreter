#include "token.h"
#include <cstdint>
#include <string_view>
#include <type_traits>

std::string Literal::to_string() const {
  return std::visit(
      [](auto &&arg) -> std::string {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, double>)
          return std::to_string(arg);
        else if constexpr (std::is_same_v<T, bool>)
          return arg ? "true" : "false";
        else if constexpr (std::is_same_v<T, std::nullptr_t>)
          return "nil";
        else if constexpr (std::is_same_v<T, std::string_view>)
          return std::string("\"") + std::string(arg) + "\"";
        else
          return "";
      },
      value_);
}

bool Literal::operator==(const Literal &other) const {
  if (type_ != other.type_)
    return false;

  switch (type_) {
  case Type::Nil:
    return true;
  case Type::Number:
    return std::get<double>(value_) == std::get<double>(other.value_);
  case Type::Bool:
    return std::get<bool>(value_) == std::get<bool>(other.value_);
  case Type::String:
  case Type::Identifer:
    return std::get<std::string_view>(value_) ==
           std::get<std::string_view>(other.value_);
  default:
    return false;
  }
}

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

constexpr std::string_view to_string(TokenType tk_type) {
  return TOKEN_TYPE_NAMES.at(static_cast<std::size_t>(tk_type));
}

std::ostream &operator<<(std::ostream &os, TokenType tk_type) {
  os << to_string(tk_type);
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
